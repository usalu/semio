use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
fn main() {
    for line in std::io::stdin().lines() {
        let s = line.unwrap();
        let mut h = DefaultHasher::new();
        s.hash(&mut h);
        println!("{:016x}", h.finish());
    }
}
