extern crate rand;
use rand::Rng;
use std::io;
fn main() {
    println!("Guess a number");
    let secret = rand::thread_rng().gen_range(1, 1000);
    loop {
        println!("Enter your guess");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Fail");
        let guess: i32 = guess.trim().parse().expect("Fail");
        if guess == secret {
            println!("Guessed Correctly");
            break;
        } else {
            println!("Try Again!");
            if guess > secret {
                println!("Number is smaller");
            } else {
                println!("Number is bigger");
            }
        }
    }
}
