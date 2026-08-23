use las::Reader;
use std::collections::HashMap;

fn main() {
    let mut reader = Reader::from_path("🧊️pattern-sphere.las").expect("open");
    let header = reader.header().clone();
    println!("version {:?}", header.version());
    println!("system_identifier {:?}", header.system_identifier());
    println!("generating_software {:?}", header.generating_software());
    println!("bounds {:?}", header.bounds());
    println!("transforms {:?}", header.transforms());
    println!("number_of_points {}", header.number_of_points());
    println!("vlrs {}", header.vlrs().len());
    for v in header.vlrs() {
        println!("  vlr user_id={:?} record_id={} description={:?} data_len={}", v.user_id, v.record_id, v.description, v.data.len());
    }
    let mut class_hist: HashMap<u8, u32> = HashMap::new();
    let mut n = 0;
    let pd = reader.read_all().expect("read_all");
    for wrapped in pd.points() {
        let p = wrapped.expect("point");
        *class_hist.entry(u8::from(p.classification)).or_insert(0) += 1;
        n += 1;
    }
    println!("read {n} points");
    let mut classes: Vec<_> = class_hist.into_iter().collect();
    classes.sort();
    println!("classification histogram: {:?}", classes);
}
