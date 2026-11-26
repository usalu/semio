//! 🌿️ Produce gbXML with the production WASI exporter.
use std::io::{self, Read, Write};
use bim_gbxml::standards::v1::subsets::any::io::export::gbxml::export_gbxml;
use bim_snapshot::standards::v1::subsets::any::io::text::snapshot::decode_model_snapshot_json;
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("snapshot");
    let model = decode_model_snapshot_json(&input).expect("decode");
    let (document, notes) = export_gbxml(&model).expect("export");
    eprintln!("{} export notes", notes.len());
    io::stdout().write_all(document.as_bytes()).expect("write");
}
