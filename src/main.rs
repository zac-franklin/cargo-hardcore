use std::io::{self, Write};
use std::process::Command;
use std::time::SystemTime;
 
fn prompt(name:&str) -> String {
    let mut line = String::new();
    print!("{}", name);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).expect("Error: Could not read a line");
 
    return line.trim().to_string()
}
 
fn should_continue() -> bool {
    loop {
        let input = prompt("> ");
        if input == "y" || input == "Y" {
            return true;
        } 
        else if input == "n" || input == "N" { 
            return false; 
        } else {
            println!("invalid input.");
        };
    }
}

fn main() {
    println!("This may delete your current project. Continue?(y/n)");

    //ask to proceed. 
    if should_continue() {
        println!("hardcore run started!");
        //start timer.
        let now = SystemTime::now();

        //get path to possibly overwrite
        let absolute = std::path::absolute(".").expect("failed to execute process");

        //run build keeping color for a nice and consistent format to cargo build
        let output = Command::new("cargo")
            .args(["check", "--color", "always"])
            .output()
            .expect("failed to execute process");

        //reset project on failure.
        if !output.status.success()  {
            std::fs::remove_dir_all(absolute.join("src")).expect("failed to execute process");
            std::fs::create_dir(absolute.join("src")).expect("failed to execute process");
            std::fs::write(absolute.join("src").join("main.rs"), b"fn main() {\n\tprintln!(\"Hello, world!\");\n}").expect("failed to execute process");
        }

        //display output to terminal
        io::stdout().write_all(&output.stdout).expect("failed to execute process");
        io::stderr().write_all(&output.stderr).expect("failed to execute process");

        //display "game" state
        if !output.status.success()  {
            println!("Game Over.");
            println!("Score: {}", now.elapsed().expect("failed to get elapsed time.").as_millis());
        } else {
            println!("Level passed!");
            println!("Current Score: {}", now.elapsed().expect("failed to get elapsed time.").as_millis());
        }
    } else {
        println!("hardcore run canceled.");
    }
}
