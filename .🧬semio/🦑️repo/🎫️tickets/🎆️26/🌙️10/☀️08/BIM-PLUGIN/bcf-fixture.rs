use std::io::{self, Read, Write};
use bim_bcf::standards::v1::subsets::any::io::bcf::{export_bcf, read_document, table_json};
use bim_snapshot::standards::v1::subsets::any::io::text::snapshot::decode_model_snapshot_json;
fn main() {
    let mut source = String::new();
    io::stdin().read_to_string(&mut source).unwrap();
    let model = decode_model_snapshot_json(&source).unwrap();
    if std::env::args().nth(1).as_deref() == Some("model") {
        print!("{}", pack_json::to_json_string(&model));
    } else if std::env::args().nth(1).as_deref() == Some("table") {
        print!("{}", table_json(&model));
    } else {
        let bytes = export_bcf(&model).unwrap();
        assert_eq!(read_document(&bytes).unwrap().topics.len(), 3);
        io::stdout().lock().write_all(&bytes).unwrap();
    }
}
