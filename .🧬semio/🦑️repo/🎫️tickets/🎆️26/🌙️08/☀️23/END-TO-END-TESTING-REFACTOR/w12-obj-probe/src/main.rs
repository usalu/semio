fn host_number(value: &json::JsonValue) -> f64 {
    value.dump().parse::<f64>().unwrap_or_else(|_| value.as_f64().unwrap_or(f64::NAN))
}

fn main() {
    let texts = ["-1.3283902924697095e-17", "2.7000102824824506", "0.1", "1", "-0.0", "1e300", "4503599627370497", "3.141592653589793"];
    let mut bad_as_f64 = 0;
    let mut bad_dump = 0;
    for text in texts {
        let doc = json::parse(&format!("{{\"v\": {text}}}")).unwrap();
        let native: f64 = text.parse().unwrap();
        let via_as_f64 = doc["v"].as_f64().unwrap();
        let via_dump = host_number(&doc["v"]);
        if via_as_f64.to_bits() != native.to_bits() { bad_as_f64 += 1; }
        if via_dump.to_bits() != native.to_bits() { bad_dump += 1; }
        println!("{text:26}  as_f64 ok={:5}  dump-parse ok={}", via_as_f64.to_bits()==native.to_bits(), via_dump.to_bits()==native.to_bits());
    }
    println!("\nas_f64 lost {bad_as_f64}/{}; dump().parse::<f64>() lost {bad_dump}", texts.len());
}
