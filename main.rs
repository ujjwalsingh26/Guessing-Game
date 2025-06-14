use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess a number!");

    let secret_no=rand::thread_rng().gen_range(1..=100);

    println!("{secret_no}");

    
    loop{
        println!("Please input your guess:");
        let mut guess: String = String::new();
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess: u32= match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>{
                println!("Please enter a valid input");
                continue;
            },

        };

        println!("You guessed {guess}");

        match guess.cmp(&secret_no){
            Ordering::Equal=>{
                println!("You Won!");
                break;
            },
            Ordering::Greater=>println!("Too high!"),
            Ordering::Less=>println!("Too low!")
        }
    }
}
