
//#region 🔖️Fixtures
const FIXTURES: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures");

const ASSETS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets");

fn decoded(file: &str) -> crate::ModelSnapshot {
    let text = std::fs::read_to_string(file).unwrap_or_else(|error| panic!("{file}: {error}"));
    semio_framework_pack_json::from_json_str(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|error| panic!("{file} decodes: {error:?}"))
}

fn snapshot_at(path: &str) -> crate::ModelSnapshot {
    decoded(&format!("{FIXTURES}/{path}/🔣️.json"))
}

/// 🗂️ The IFC4 fixtures: the folder below `🏢️ifc4` and the snapshot the file is written from.
fn cases() -> Vec<(&'static str, crate::ModelSnapshot)> {
    vec![
        ("🏠️house/🏠️house.ifc", house()),
        ("🏷️psets/🏷️psets.ifc", psets()),
        ("🔲️ceilings/🔲️ceilings.ifc", snapshot_at("🏗️ifc/🔲️ceilings/📸️snapshot")),
        ("🪧️notated/🪧️notated.ifc", snapshot_at("💡️inferences/🪧️annotation-layout/🏠️room/📸️snapshot")),
        ("🛝️ramps/🛝️ramps.ifc", snapshot_at("💡️inferences/🛝️ramp-runs/🏞️ramps/📸️snapshot")),
        ("🧗️wall-depth/🧗️wall-depth.ifc", snapshot_at("💡️inferences/🧗️wall-depth/🏠️attic/📸️snapshot")),
        ("🏡️example-house/🏡️example-house.ifc", decoded(&format!("{ASSETS}/🏡️house/📸️snapshot.json"))),
        ("🏢️example-office/🏢️example-office.ifc", decoded(&format!("{ASSETS}/🏢️office/📸️snapshot.json"))),
    ]
}

#[test]
fn the_committed_ifc4_files_are_the_current_exports() {
    for (path, model) in cases() {
        let (bytes, notes) = export_ifc4(&model).unwrap_or_else(|error| panic!("{path}: {error}"));
        assert!(notes.iter().all(|note| !note.is_empty()), "{path}: {notes:?}");
        let file = format!("{FIXTURES}/🏢️ifc4/{path}");
        if std::env::var("BIM_BLESS").is_ok() {
            std::fs::create_dir_all(std::path::Path::new(&file).parent().expect("a folder")).expect("the fixture folder");
            std::fs::write(&file, &bytes).expect("the file is written");
        }
        let committed = std::fs::read(&file).unwrap_or_else(|error| panic!("{file}: {error}. Run the test with BIM_BLESS=1 to write the file, then `python 🐍️.py write` of the export-bim-1-ifc4 case."));
        assert!(committed == bytes, "{path}: the committed IFC4 export drifted: rewrite it with BIM_BLESS=1");
    }
}

#[test]
fn the_subject_report_equals_the_table_the_ifcopenshell_oracle_measured_from_each_committed_ifc4_file() {
    use crate::standards::v1::subsets::any::io::export::ifc::projection::projection_in;
    for (path, model) in cases() {
        let measured = format!("{FIXTURES}/🏢️ifc4/{}/🔬️measure/🔣️.json", path.split('/').next().expect("a case folder"));
        let oracle: serde_json::Value = serde_json::from_slice(&std::fs::read(&measured).unwrap_or_else(|error| panic!("{measured}: {error}. Run `python 🐍️.py write` of the export-bim-1-ifc4 case."))).expect("the oracle table");
        let report: serde_json::Value = serde_json::from_str(&projection_in(Schema::Ifc4, &model).to_json()).expect("the report parses");
        assert_eq!(oracle["schema"], "IFC4", "{path}");
        assert_eq!(oracle["counts"], report["counts"], "{path}");
        assert_eq!(oracle["containment"], report["containment"], "{path}");
        assert_eq!(oracle["classifications"], report["classifications"], "{path}");
        assert_eq!(oracle["type_properties"], report["type_properties"], "{path}");
        assert_eq!(oracle["annotations"], report["annotations"], "{path}");
        let (kernel, written) = (oracle["volumes"].as_object().expect("volumes"), report["volumes"].as_object().expect("volumes"));
        assert_eq!(kernel.keys().collect::<Vec<_>>(), written.keys().collect::<Vec<_>>(), "{path}");
        for (tag, volume) in written {
            let (a, b) = (kernel[tag].as_f64().expect("a number"), volume.as_f64().expect("a number"));
            assert!((a - b).abs() < 1e-9 * b.abs().max(1.0), "{path} {tag}: kernel {a}, written {b}");
        }
    }
}
//#endregion 🔖️Fixtures
