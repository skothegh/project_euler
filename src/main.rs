fn main() {
    let j = p003();
    println!("{j}");
}


fn p004() {
    
}


fn p003() -> isize {
    // What is the largest prime factor of n?
    let mut n: isize = 600851475143;

    loop {
        let p = smallest_prime(n);

        if p < n {
            n = n / p as isize;
        }
        else {
            return n
        }
    }

}

fn smallest_prime(n: isize) -> isize {
    if n == 1 {
        return 1;
    }
    
    for a in 2..n {
        if n % a == 0 {
            return a;
        }
    }
    n
    
}

fn p002() -> isize {
    // Add all even Fibonacci numbers smaller than 4m
    let mut fib0: isize = 0;
    let mut fib1 = 1;
    let mut fibn = 0;
    let mut temp: isize = 0;
    

    loop {

        fibn = fib0 + fib1;
        fib0 = fib1;
        fib1 = fibn;

        if fibn > 4000000 {
            break;
        }

        if fibn % 2 == 0 {
            temp = temp + fibn as isize;
        }
        
    }
    temp
    
}



fn p001() -> i32 {
    // Add all numbers divisible by 3 or 5 below 1000.
    let mut temp: i32 = 0;
    for i in 1..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            temp = temp + i;
        }
    }
    
    temp
}
