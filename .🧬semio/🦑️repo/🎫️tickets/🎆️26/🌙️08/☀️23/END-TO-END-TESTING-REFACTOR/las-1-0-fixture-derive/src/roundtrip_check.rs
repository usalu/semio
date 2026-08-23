use las::point::Format;
use las::raw::{Header, Point, Vlr};
use std::io::Cursor;

fn main() {
    let path = "🧊️pattern-sphere.las";
    let bytes = std::fs::read(path).expect("read fixture");
    let mut cursor = Cursor::new(&bytes[..]);
    let header = Header::read_from(&mut cursor).expect("read header");
    let mut vlrs = Vec::new();
    for _ in 0..header.number_of_variable_length_records {
        vlrs.push(Vlr::read_from(&mut cursor, false).expect("read vlr"));
    }
    let format = Format::new(header.point_data_record_format).expect("format");
    let pos = cursor.position() as usize;
    println!("computed pos after vlrs = {pos}, header.offset_to_point_data = {}", header.offset_to_point_data);
    let mut cursor2 = Cursor::new(&bytes[header.offset_to_point_data as usize..]);
    let mut points = Vec::new();
    for _ in 0..header.number_of_point_records {
        points.push(Point::read_from(&mut cursor2, &format).expect("read point"));
    }

    let mut out = Vec::new();
    header.write_to(&mut out).expect("write header");
    for v in &vlrs { v.write_to(&mut out).expect("write vlr"); }
    for p in &points { p.write_to(&mut out, &format).expect("write point"); }

    println!("input len = {}, output len = {}", bytes.len(), out.len());
    println!("identical = {}", out == bytes);
    if out != bytes {
        let n = out.len().min(bytes.len());
        for i in 0..n {
            if out[i] != bytes[i] {
                println!("first diff at byte {i}: input={} output={}", bytes[i], out[i]);
                break;
            }
        }
    }
}
