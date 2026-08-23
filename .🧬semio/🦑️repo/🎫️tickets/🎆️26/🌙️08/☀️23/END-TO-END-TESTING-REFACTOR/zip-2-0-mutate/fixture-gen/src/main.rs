//! 🧭️ One-off derivation of the real-world multi-entry ZIP fixture for
//! `mutate-zip-2-0` (ticket 26/08/23/END-TO-END-TESTING-REFACTOR, wave 7). Run once with
//! `cargo run`, output committed at
//! `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🧫️fixtures/🎒️zwischenbericht-projekte.zip`.
//! Not a test step — never invoked by the test platform.

use std::io::Write;
use std::path::{Path, PathBuf};

const NAMES: [&str; 20] = [
    "P01_k118_kopfbau_halle_118.jpg",
    "P02_bedzed.jpg",
    "P03_biopartner_5.jpg",
    "P04_ka13.jpg",
    "P05_recypark_demets.jpg",
    "P06_svanen_kindergarten.jpg",
    "P07_villa_welpeloo.jpg",
    "P08_holbein_gardens.jpg",
    "P09_werkhof_29.jpg",
    "P10_haus_hos.jpg",
    "P11_mehrow_pilot_house.jpg",
    "P12_broethen_twin_house.jpg",
    "P13_crclr_house.jpg",
    "P14_recyclinghaus_hannover.jpg",
    "P15_thoravej_29.jpg",
    "P16_timber_square.jpg",
    "P17_tbc_london.jpg",
    "P18_55_great_suffolk_street.jpg",
    "P19_brent_cross_town_substation.jpg",
    "P20_boulder_fire_station_3.jpg",
];

fn repo_root() -> PathBuf {
    let mut dir = std::env::current_dir().expect("cwd");
    loop {
        if dir.join("nx.json").exists() {
            return dir;
        }
        dir = dir.parent().expect("repo root above cwd").to_path_buf();
    }
}

fn main() {
    let root = repo_root();
    let source_dir = root.join("♻️mit-bestand/📋️bericht/📋️zwischenbericht/asset/projekt");
    let out_path = root.join("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🧫️fixtures/🎒️zwischenbericht-projekte.zip");

    let mut writer = zip::ZipWriter::new(std::io::Cursor::new(Vec::<u8>::new()));
    let options: zip::write::FileOptions<'_, ()> = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    for name in NAMES {
        let source = source_dir.join(name);
        let bytes = std::fs::read(&source).unwrap_or_else(|error| panic!("read {}: {error}", source.display()));
        writer.start_file(format!("projekt/{name}"), options).unwrap_or_else(|error| panic!("start_file {name}: {error}"));
        writer.write_all(&bytes).unwrap_or_else(|error| panic!("write {name}: {error}"));
    }
    writer.set_comment("Zwischenbericht Projektbeispiele – 20 von 67 realen Bestandsarchitektur-Referenzfotos.");
    let cursor = writer.finish().expect("finish zip");
    let bytes = cursor.into_inner();

    if let Some(parent) = Path::new(&out_path).parent() {
        std::fs::create_dir_all(parent).expect("create fixtures dir");
    }
    std::fs::write(&out_path, &bytes).expect("write fixture zip");
    println!("wrote {} bytes to {}", bytes.len(), out_path.display());
}
