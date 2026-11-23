fn main() {
    let record = p_mid::record(42);
    println!("{} {} {}", record.id, record.sum, p_side::label(7));
}
