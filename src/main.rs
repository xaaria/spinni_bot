use std::env;
use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;

static VERSION: &str = "0.0.1";
 
#[tokio::main]
async fn main() {

    println!("Iltaa ~ Spinni bot v.{}", VERSION);
    println!("Checking Telegram access token...");
    
    let bot: Bot = Bot::from_env();
    // Bot is now ready
    
    // Print some info
    print_commands(&bot).await;
    print_token(&bot);

    print!("Bot is now up and running...")


    let r = teloxide::repl(bot, |bot: Bot, msg: Message| async move {

        let username: String = msg.clone().from.and_then(|f| f.username).unwrap_or(String::from("[no user]") );
        let msg_content = msg.text().unwrap_or("[no message]");
        println!("Received from @{} message/cmd '{}' ", username , msg_content);


        bot.send_dice(msg.chat.id).await?;
        Ok(())
    }).await;

}

// Prints registered commands
async fn print_commands(bot: &Bot) {

    let cmds = bot.get_my_commands().await;
    match cmds {
        Ok(x) => {
            println!("Commands are:");
            for y in x {
                println!("\t/{}", y.command);
            }
        },
        _ => println!("Error fetching commands from telegram API"),
    }

}

fn print_token(bot: &Bot) {
    println!("Token found ...{}", bot.token().split_at( 
        bot.token().chars().count()-5 ).1 
    ); // display only last x chars
}