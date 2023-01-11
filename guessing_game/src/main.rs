extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101); // OS가 seed를 정한다.
    //println!("The secret number is: {}", secret_number);


    loop
    {
        println!("Please input your guess.");
        // mut 가변 변수
        let mut guess = String::new();  // string growable utf-8

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue;
        }
        //.expect("Please type a number!");
            

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number)
        {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => 
            {
                println!("You win!");
                break;
            } 
        }
    }
}
