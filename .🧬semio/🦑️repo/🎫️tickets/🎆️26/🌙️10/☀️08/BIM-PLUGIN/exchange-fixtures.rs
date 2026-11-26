use std::{env, fs, io::{self, Write}, path::PathBuf};
use bim_ifc::standards::v1::subsets::any::io::export::ifc::{export_ifc2x3, export_ifc4};
use bim_svg::standards::v1::subsets::any::io::export::svg::export_svg;
use bim_snapshot::standards::v1::subsets::any::io::text::snapshot::decode_model_snapshot_json;

fn main() {
    let args: Vec<_> = env::args_os().skip(1).collect();
    let input = fs::read_to_string(&args[1]).unwrap();
    let model = decode_model_snapshot_json(&input).unwrap();
    let path = PathBuf::from(&args[2]);
    let before = fs::read(&path).ok();
    let (bytes, notes) = match args[0].to_str().unwrap() {
        "2x3" => export_ifc2x3(&model).unwrap(),
        "4" => export_ifc4(&model).unwrap(),
        "svg" => (export_svg(&model).unwrap().into_bytes(), vec![]),
        _ => panic!("Expected IFC2X3, IFC4 or SVG fixture"),
    };
    if path.as_os_str() == "-" {
        io::stdout().lock().write_all(&bytes).unwrap();
        return;
    }
    if before.as_deref() != Some(bytes.as_slice()) {
        assert_eq!(fs::read(&path).ok(), before, "fixture was changed during export");
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, &bytes).unwrap();
    }
    println!("{} bytes, {} export notes, {}", bytes.len(), notes.len(), path.display());
}
