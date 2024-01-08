use std::io;

fn main() {
    // converter();
    // let n = 12;
    // let result = fib(n);
    // println!("The nth fib number is  {result}");
    // let n = 11;
    // let result = fib_rec(n);
    // println!("The nth fib number is  {result}");
    print_song();
    // println("The program ends!");
}


fn converter() {
    println!("Celsius to Fahreinheit converted");
    println!("Please enter celsius value");

    let mut c = String::new();
    io::stdin()
        .read_line(&mut c)
        .expect("Failed to read value");

    let c : f64 = c.trim().parse().expect("Invalid string, please enter a number");
    let f : f64 = (c * 9.0/5.0) + 32.0;

    println!("Given celsius = {c}, converted fahreinheit = {f}");

}

fn fib(n: u32) -> i32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let mut f1 = 0;
    let mut f2 = 1;
    for _i in 2..=n {
        let temp = f2;
        f2 = f2 + f1;
        f1 = temp;
    }
    return f2;
}

fn fib_rec(n: u32) -> i32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    return fib_rec(n-1) + fib_rec(n-2);
}

fn print_song() {
    let song = "Away in a manger, no crib for a bed,
The little Lord Jesus laid down his sweet head.

The stars in the sky looked down where he lay,
The little Lord Jesus asleep in the hay.

The cattle are lowing, the baby awakes,
But little Lord Jesus no crying he makes.

I love Thee, Lord Jesus, look down from the sky
And stay by my cradle 'til morning is nigh.

Be near me, Lord Jesus, I ask Thee to stay
Close by me forever, and love me, I pray.

Bless all the dear children in thy tender care,
And take us to heaven, to live with Thee there.";

    println!("{song}");
}
