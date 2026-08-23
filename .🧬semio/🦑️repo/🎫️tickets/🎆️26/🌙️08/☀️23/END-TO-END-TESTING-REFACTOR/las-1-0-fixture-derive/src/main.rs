//! One-off derivation step 2/2: builds the real LAS 1.0 point-cloud fixture for the
//! `stdio.las` 1.0/any mutation test case out of the real `POSITION` vertices extracted from
//! `🧊️pattern-sphere.glb` by `extract_positions.py` (step 1/2, hand-parsed GLB, no gltf crate).
//!
//! 8,448 real vertices of a unit sphere ([-1,1] per axis) are scaled ×10 (a plausible 20-unit
//! real-world object) and translated onto a plausible UTM-ish easting/northing/elevation near
//! Hannover (583000/5804000/0) -- an editorial coordinate-system choice layered on top of 100%
//! real vertex positions, exactly the same kind of choice `derive_obj.py` made when banding real
//! triangles into named groups. Per-point `intensity`/`classification`/`scanAngleRank`/
//! `scanDirectionFlag`/`edgeOfFlightLine` are all derived FROM that real geometry (height and
//! horizontal position within the unit sphere), not fabricated constants, so `SetPointsByReturn`,
//! `SetBounds` and `SetScaleAndOffset` in the test case act on a real, varied distribution.
//!
//! Written with the real `las` 0.11 reference crate (the same crate registered as this subset's
//! test oracle) so the committed fixture is a genuine third-party-produced LAS 1.0 file, not our
//! own codec's output.

use las::point::Format;
use las::{Builder, Color, Point, Transform, Vector, Vlr, Writer};
use std::fs;
use std::io::{BufWriter, Write};

const POSITIONS: &str = "positions.txt";
const OUT: &str = "🧊️pattern-sphere.las";

//#region 🔖️Geometry
struct Real {
    x: f64,
    y: f64,
    z: f64,
}

fn read_positions() -> Vec<Real> {
    let text = fs::read_to_string(POSITIONS).expect("read positions.txt (run extract_positions.py first)");
    text.lines()
        .map(|line| {
            let mut parts = line.split_whitespace().map(|v| v.parse::<f64>().unwrap());
            Real { x: parts.next().unwrap(), y: parts.next().unwrap(), z: parts.next().unwrap() }
        })
        .collect()
}
//#endregion 🔖️Geometry

//#region 🔖️Derive
const GEOMETRY_SCALE: f64 = 10.0;
const X_OFFSET: f64 = 583_000.0;
const Y_OFFSET: f64 = 5_804_000.0;
const Z_OFFSET: f64 = 0.0;

/// 🏷️ Buckets the real (unscaled, unit-sphere) height into an ASPRS classification code, so the
/// committed fixture's `classification` column is a real function of real geometry rather than a
/// constant.
fn classification_of(z_unit: f64) -> u8 {
    match z_unit {
        z if z < -0.6 => 2,  // Ground
        z if z < -0.2 => 3,  // Low Vegetation
        z if z < 0.2 => 4,   // Medium Vegetation
        z if z < 0.6 => 5,   // High Vegetation
        _ => 6,              // Building
    }
}
//#endregion 🔖️Derive

fn main() {
    let positions = read_positions();
    println!("read {} real vertex positions", positions.len());

    let mut builder = Builder::from((1, 0));
    builder.point_format = Format::new(0).expect("point format 0");
    builder.system_identifier = "PATTERN-SPHERE-GLB-DERIVED".to_string();
    builder.generating_software = "semio-las-fixture-derive".to_string();
    builder.date = chrono::NaiveDate::from_yo_opt(2026, 235); // 2026-08-23
    builder.transforms = Vector { x: Transform { scale: 0.001, offset: X_OFFSET }, y: Transform { scale: 0.001, offset: Y_OFFSET }, z: Transform { scale: 0.001, offset: Z_OFFSET } };
    builder.vlrs = vec![
        Vlr { user_id: "LASF_Projection".to_string(), record_id: 34735, description: "GeoKeyDirectoryTag".to_string(), data: vec![1, 0, 1, 0, 0, 0, 0, 0] },
        Vlr {
            user_id: "semio".to_string(),
            record_id: 1,
            description: "fixture provenance".to_string(),
            data: b"derived once from real committed geometry: framework/modules/assets/images/pattern-sphere.glb (679 KB), POSITION accessor, hand-parsed GLB container, no gltf crate".to_vec(),
        },
    ];
    let header = builder.into_header().expect("build header");

    let file = fs::File::create(OUT).expect("create output file");
    let mut writer = Writer::new(BufWriter::new(file), header).expect("create las writer");

    for (index, real) in positions.iter().enumerate() {
        let classification = classification_of(real.z);
        let intensity = (((real.z + 1.0) / 2.0) * 65535.0).round().clamp(0.0, 65535.0) as u16;
        let scan_angle = (real.x * 90.0).round().clamp(-90.0, 90.0) as f32;
        let point = Point {
            x: real.x * GEOMETRY_SCALE + X_OFFSET,
            y: real.y * GEOMETRY_SCALE + Y_OFFSET,
            z: real.z * GEOMETRY_SCALE + Z_OFFSET,
            intensity,
            return_number: 1,
            number_of_returns: 1,
            scan_direction: if real.x >= 0.0 { las::point::ScanDirection::LeftToRight } else { las::point::ScanDirection::RightToLeft },
            is_edge_of_flight_line: real.y.abs() > 0.95,
            classification: las::point::Classification::new(classification).expect("valid classification"),
            scan_angle,
            user_data: (index % 256) as u8,
            point_source_id: 1,
            color: None::<Color>,
            ..Default::default()
        };
        writer.write_point(point).expect("write point");
    }
    writer.close().expect("close writer");

    let metadata = fs::metadata(OUT).expect("stat output");
    println!("wrote {OUT} ({} bytes, {} points)", metadata.len(), positions.len());
    let _ = std::io::stdout().flush();
}
