use semio_s_plugin_stdio::artifacts::dwg::standards::v_ac1024::subsets::any::schema::snapshot::{decode_dwg, encode_dwg, DwgSnapshot};

fn main() {
    for path in std::env::args().skip(1) {
        let bytes = std::fs::read(&path).expect("read");
        print!("{}: len={} head={:?} -> ", path.rsplit('/').next().unwrap(), bytes.len(), String::from_utf8_lossy(&bytes[0..6.min(bytes.len())]));
        match decode_dwg(&bytes) {
            Ok(s) => {
                println!("OK version={} layers={} entities={} classes={}", s.version, s.drawing.layers.len(), s.drawing.entities().len(), s.classes.len());
                match encode_dwg(&s) {
                    Ok(out) => println!("   re-encode: {} bytes, identical={}", out.len(), out == bytes),
                    Err(e) => println!("   re-encode ERR {e}"),
                }
            }
            Err(e) => println!("ERR {e}"),
        }
    }
    let empty = DwgSnapshot { version: "AC1018".into(), maintenance_version: 2, codepage: 30, ..DwgSnapshot::default() };
    match encode_dwg(&empty) {
        Ok(out) => println!("default snapshot encodes to {} bytes", out.len()),
        Err(e) => println!("default snapshot ERR {e}"),
    }
}
