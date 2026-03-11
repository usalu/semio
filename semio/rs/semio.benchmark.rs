// #region 🔖Header
// [👤semio📚rs🥼semiobenchmark](semiorepo://p/u/semio/b/l/rs/f/semio.benchmark.rs)

// 2026 Ueli Saluz <ueli@semio-tech.de>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🔖Header

use std::time::Instant;
use std::fs;
use std::path::Path;
use semio::*;

const ASSETS_DIR: &str = "../assets/semio";
const ITERATIONS: u32 = 3;

fn load_kit(filename: &str) -> Kit {
    let path = Path::new(ASSETS_DIR).join(filename);
    let data = fs::read_to_string(&path).expect(&format!("Failed to read {}", path.display()));
    serde_json::from_str(&data).expect("Failed to deserialize kit")
}

fn load_kit_diff(filename: &str) -> KitDiff {
    let path = Path::new(ASSETS_DIR).join(filename);
    let data = fs::read_to_string(&path).expect(&format!("Failed to read {}", path.display()));
    serde_json::from_str(&data).expect("Failed to deserialize kit diff")
}

fn bench<F: Fn()>(name: &str, f: F) {
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        f();
    }
    let duration = start.elapsed().as_secs_f64() / ITERATIONS as f64;
    println!("{},{:.6}", name, duration);
}

fn find_design<'a>(kit: &'a Kit, name: &str, parent_name: Option<&str>) -> &'a Design {
    let parent_guid = if let Some(pn) = parent_name {
        kit.designs.iter().flatten().find(|d| d.name == pn).map(|d| d.guid.clone())
    } else {
        None
    };
    
    if parent_name.is_some() && parent_guid.is_none() {
        panic!("Parent {} not found", parent_name.unwrap());
    }
    
    kit.designs.iter().flatten().find(|d| {
        if d.name != name { return false; }
        match &d.parent {
            Some(p) => match &parent_guid {
                Some(pg) => p.guid == *pg,
                None => false,
            },
            None => parent_guid.is_none(),
        }
    }).expect(&format!("Design {} not found", name))
}

fn main() {
    let kit_metabolism = load_kit("metabolism.kit.semio.json");
    let kit_invalid = load_kit("metabolism.kit.semio.json");
    let metabolism_zip_path = Path::new(ASSETS_DIR).join("metabolism.semio.zip");
    let metabolism_zip_str = metabolism_zip_path.to_str().unwrap();
    // We assume running from rs/semio
    
    bench("Roundtrip/Metabolism", || {
        let import_result = semio::zip_roundtrip::import_kit_from_zip(metabolism_zip_str).unwrap();
        
        let temp_zip = "temp_benchmark_metabolism.zip";
        // Need schema? Rust ExportKitToZip calls KitToSqlite which executes schema.
        // Wait, Rust implementation of export_kit_to_zip in semio.rs...
        // Let's check signature.

        semio::zip_roundtrip::export_kit_to_zip(&import_result.kit, &import_result.files, temp_zip).unwrap();
        if Path::new(temp_zip).exists() {
            std::fs::remove_file(temp_zip).unwrap();
        }
    });

    let d1 = find_design(&kit_metabolism, "Nakagin Capsule Tower", None);
    let d1_guid = d1.guid.clone();
    bench("Flatten Design/Nakagin Capsule Tower", || {
        let _ = flatten_design(&kit_metabolism, &d1_guid);
    });

    let d2 = find_design(&kit_metabolism, "Slanted", Some("Nakagin Capsule Tower"));
    let d2_guid = d2.guid.clone();
    bench("Flatten Design/Nakagin Capsule Tower/Slanted", || {
        let _ = flatten_design(&kit_metabolism, &d2_guid);
    });

    let d3 = find_design(&kit_metabolism, "Twisted", Some("Nakagin Capsule Tower"));
    let d3_guid = d3.guid.clone();
    bench("Flatten Design/Nakagin Capsule Tower/Twisted", || {
        let _ = flatten_design(&kit_metabolism, &d3_guid);
    });

    let d4 = find_design(&kit_metabolism, "Dancing", Some("Nakagin Capsule Tower"));
    let d4_guid = d4.guid.clone();
    bench("Flatten Design/Nakagin Capsule Tower/Dancing", || {
        let _ = flatten_design(&kit_metabolism, &d4_guid);
    });

    let d5 = find_design(&kit_metabolism, "Capsule Dream", None);
    let d5_guid = d5.guid.clone();
    bench("Flatten Design/Capsule Dream", || {
        let _ = flatten_design(&kit_metabolism, &d5_guid);
    });

    bench("Validation/Invalid Kit", || {
        let _ = validate_kit(&kit_invalid);
    });

    bench("Validation/Metabolism", || {
        let _ = validate_kit(&kit_metabolism);
    });
}
