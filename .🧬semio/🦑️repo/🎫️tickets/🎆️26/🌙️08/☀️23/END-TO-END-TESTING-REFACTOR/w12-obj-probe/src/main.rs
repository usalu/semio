fn exact(number: f64) -> json::JsonValue {
    if !number.is_finite() {
        return json::JsonValue::Null;
    }
    json::parse(&format!("{number:?}")).unwrap_or(json::JsonValue::from(number))
}

fn main() {
    let cases = [2.7000102824824506_f64, 0.1, 1.0, -8.881784197001252e-16, 1e300, 1.0 / 3.0, 4503599627370497.0, 0.0, -0.0];
    let mut broken_from = 0;
    let mut broken_exact = 0;
    for value in cases {
        let via_from = json::JsonValue::from(value).dump();
        let via_exact = exact(value).dump();
        let back_from: f64 = json::parse(&via_from).unwrap().as_f64().unwrap();
        let back_exact: f64 = json::parse(&via_exact).unwrap().as_f64().unwrap();
        if back_from.to_bits() != value.to_bits() {
            broken_from += 1;
        }
        if back_exact.to_bits() != value.to_bits() {
            broken_exact += 1;
        }
        println!("{value:?}\n   from -> {via_from} (round trips: {})\n   exact-> {via_exact} (round trips: {})", back_from.to_bits() == value.to_bits(), back_exact.to_bits() == value.to_bits());
    }
    println!("\nJsonValue::from(f64) lost {broken_from}/{} values; parse(format!(\"{{:?}}\")) lost {broken_exact}", cases.len());
}
