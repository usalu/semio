use super::*;
use crate::standards::v1::subsets::any::io::export::ifc::{export_ifc2x3, testkit::house};

const HOUSE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🏗️ifc/🏠️house");

fn read(name: &str) -> Vec<u8> {
    std::fs::read(format!("{HOUSE_DIR}/{name}")).unwrap_or_else(|error| panic!("{name}: {error}. Run the test with BIM_BLESS=1 to write the file, then `python 🐍️.py write` of the export case."))
}

#[test]
fn the_report_counts_the_classes_of_the_house_and_lists_each_storey_containment() {
    let report = projection(&house());
    assert_eq!(report.schema, "IFC2X3");
    let count = |class: &str| report.counts[class];
    assert_eq!((count("IfcWallStandardCase"), count("IfcWall"), count("IfcOpeningElement"), count("IfcWindow"), count("IfcDoor")), (2, 5, 6, 3, 2));
    assert_eq!((count("IfcColumn"), count("IfcBeam"), count("IfcStair"), count("IfcStairFlight"), count("IfcSpace"), count("IfcGrid")), (2, 2, 2, 3, 2, 1));
    assert_eq!((count("IfcRoof"), count("IfcCurtainWall"), count("IfcMember"), count("IfcPlate"), count("IfcRailing")), (1, 1, 1, 1, 1));
    assert_eq!(report.containment["st-ground"], ["b-1", "b-2", "c-1", "c-2", "o-door-1", "o-win-1", "o-win-2", "o-win-arc", "s-1", "sl-ground", "w-arc", "w-east", "w-free", "w-north", "w-south", "w-west"]);
    assert_eq!(report.containment["st-roof"], ["r-1"]);
    assert!(report.containment["st-base"].is_empty());
}

#[test]
fn only_exactly_measurable_elements_carry_a_volume() {
    let report = projection(&house());
    assert!(report.volumes.contains_key("w-south") && report.volumes.contains_key("w-free") && report.volumes.contains_key("c-1") && report.volumes.contains_key("b-1") && report.volumes.contains_key("sl-ground"));
    assert!(!report.volumes.contains_key("w-arc"), "the curved wall is tessellated by the kernel");
    assert!(!report.volumes.contains_key("sl-balcony"), "the sloped slab is a brep");
    assert!(!report.volumes.contains_key("c-2"), "the round column is tessellated by the kernel");
    assert!((report.volumes["w-free"] - 3.0 * 0.15 * 2.4).abs() < 1e-9);
    assert!((report.volumes["c-1"] - 0.27).abs() < 1e-9);
    assert!((report.volumes["sl-ground"] - (8.0 * 6.0 - 1.0) * 0.25).abs() < 1e-9);
}

#[test]
fn the_json_form_parses_and_keeps_the_numbers() {
    let report = projection(&house());
    let parsed: serde_json::Value = serde_json::from_str(&report.to_json()).expect("valid JSON");
    assert_eq!(parsed["schema"], "IFC2X3");
    assert_eq!(parsed["counts"]["IfcWall"], 5);
    assert!((parsed["volumes"]["c-1"].as_f64().expect("a number") - 0.27).abs() < 1e-9);
}

#[test]
fn the_committed_house_file_is_the_current_export() {
    let (bytes, notes) = export_ifc2x3(&house()).expect("the house exports");
    assert!(notes.is_empty());
    if std::env::var("BIM_BLESS").is_ok() {
        std::fs::write(format!("{HOUSE_DIR}/🏠️house.ifc"), &bytes).expect("the file is written");
    }
    assert_eq!(read("🏠️house.ifc"), bytes, "the committed export drifted: rewrite it with BIM_BLESS=1");
}

#[test]
fn the_subject_report_equals_the_table_the_ifcopenshell_oracle_measured_from_the_committed_file() {
    let oracle: serde_json::Value = serde_json::from_slice(&read("🔬️measure/🔣️.json")).expect("the oracle table");
    let report = projection(&house());
    assert_eq!(oracle["schema"], "IFC2X3");
    for (class, count) in &report.counts {
        assert_eq!(oracle["counts"][class].as_u64(), Some(*count as u64), "{class}");
    }
    for (storey, tags) in &report.containment {
        let measured: Vec<String> = oracle["containment"][storey].as_array().expect("tags").iter().map(|tag| tag.as_str().expect("a tag").to_string()).collect();
        assert_eq!(&measured, tags, "{storey}");
    }
    assert_eq!(oracle["volumes"].as_object().expect("volumes").len(), report.volumes.len());
    for (tag, volume) in &report.volumes {
        let measured = oracle["volumes"][tag].as_f64().expect("a kernel volume");
        assert!((measured - volume).abs() < 1e-9 * volume.abs().max(1.0), "{tag}: kernel {measured}, written {volume}");
    }
}

const NOTATED_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🏗️ifc/🪧️notated");

fn read_notated(name: &str) -> Vec<u8> {
    std::fs::read(format!("{NOTATED_DIR}/{name}")).unwrap_or_else(|error| panic!("{name}: {error}. Run the test with BIM_BLESS=1 to write the file, then `python 🐍️.py write` of the export case."))
}

#[test]
fn the_report_lists_every_annotation_with_its_kind_printed_text_curves_literals_and_total() {
    use crate::standards::v1::subsets::any::io::export::ifc::testkit::notated;
    let report = projection(&notated());
    assert_eq!(report.counts["IfcAnnotation"], 15);
    assert_eq!(report.counts["IfcTextLiteralWithExtent"], 14);
    let south = &report.annotations["South length"];
    assert_eq!((south.kind.as_str(), south.printed.as_str(), south.literals.as_slice()), ("Dimension", "8.00", ["8.00".to_string()].as_slice()));
    assert!(south.curves >= 3 && south.total.is_some_and(|total| (total - 8.0).abs() < 1e-9), "{south:?}");
    let parallel = &report.annotations["Parallel faces"];
    assert_eq!((parallel.curves, parallel.literals.len(), parallel.total), (0, 0, None), "an unresolved dimension draws nothing and has no value");
    assert_eq!(report.annotations["note-1"].literals, ["Verify on site"]);
    assert_eq!(report.annotations["tag-empty"].literals.len(), 0, "an empty tag prints nothing");
    assert_eq!(report.containment["st-ground"].iter().filter(|name| report.annotations.contains_key(*name)).count(), 15);
    assert!(projection(&house()).annotations.is_empty());
}

#[test]
fn the_committed_notated_file_is_the_current_export() {
    use crate::standards::v1::subsets::any::io::export::ifc::testkit::notated;
    let (bytes, _) = export_ifc2x3(&notated()).expect("the room exports");
    if std::env::var("BIM_BLESS").is_ok() {
        std::fs::create_dir_all(NOTATED_DIR).expect("the fixture directory");
        std::fs::write(format!("{NOTATED_DIR}/🪧️notated.ifc"), &bytes).expect("the file is written");
    }
    assert_eq!(read_notated("🪧️notated.ifc"), bytes, "the committed export drifted: rewrite it with BIM_BLESS=1");
}

#[test]
fn the_notated_subject_report_equals_the_table_the_ifcopenshell_oracle_measured_from_the_committed_file() {
    use crate::standards::v1::subsets::any::io::export::ifc::testkit::notated;
    let oracle: serde_json::Value = serde_json::from_slice(&read_notated("🔬️measure/🔣️.json")).expect("the oracle table");
    let ours: serde_json::Value = serde_json::from_str(&projection(&notated()).to_json()).unwrap();
    assert_eq!(oracle["schema"], ours["schema"]);
    assert_eq!(oracle["counts"], ours["counts"]);
    assert_eq!(oracle["containment"], ours["containment"]);
    assert_eq!(oracle["volumes"], ours["volumes"]);
    assert_eq!(oracle["annotations"].as_object().unwrap().len(), ours["annotations"].as_object().unwrap().len());
    for (name, row) in ours["annotations"].as_object().unwrap() {
        for key in ["kind", "printed", "curves", "literals"] {
            assert_eq!(oracle["annotations"][name][key], row[key], "{name}.{key}");
        }
        match (oracle["annotations"][name]["total"].as_f64(), row["total"].as_f64()) {
            (Some(measured), Some(written)) => assert!((measured - written).abs() < 1e-9, "{name}: {measured} vs {written}"),
            (measured, written) => assert_eq!(measured, written, "{name}: total"),
        }
    }
}

const CEILINGS_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🏗️ifc/🔲️ceilings");

fn read_ceilings(name: &str) -> Vec<u8> {
    std::fs::read(format!("{CEILINGS_DIR}/{name}")).unwrap_or_else(|error| panic!("{name}: {error}. Run the test with BIM_BLESS=1 to write the file, then `python 🐍️.py write` of the export case."))
}

#[test]
fn the_report_counts_the_coverings_and_measures_only_the_flat_ceilings_with_straight_edges() {
    use crate::standards::v1::subsets::any::io::export::ifc::testkit::ceilings;
    let report = projection(&ceilings());
    assert_eq!((report.counts["IfcCovering"], report.counts["IfcCoveringType"]), (4, 3));
    assert_eq!(report.containment["st-ground"], ["c-curved", "c-diagonal-slope", "c-holed", "c-sloped"]);
    assert_eq!(report.volumes.keys().collect::<Vec<_>>(), ["c-holed"], "the sloped ceilings are breps and the curved one is tessellated by the kernel");
    assert!((report.volumes["c-holed"] - (24.0 - 1.0) * 0.0625).abs() < 1e-9, "the net area of the outline less its hole times the layer thickness");
}

#[test]
fn the_committed_ceilings_file_is_the_current_export() {
    use crate::standards::v1::subsets::any::io::export::ifc::testkit::ceilings;
    let (bytes, notes) = export_ifc2x3(&ceilings()).expect("the ceilings export");
    assert!(notes.is_empty(), "{notes:?}");
    if std::env::var("BIM_BLESS").is_ok() {
        std::fs::write(format!("{CEILINGS_DIR}/🔲️ceilings.ifc"), &bytes).expect("the file is written");
    }
    assert_eq!(read_ceilings("🔲️ceilings.ifc"), bytes, "the committed export drifted: rewrite it with BIM_BLESS=1");
}

#[test]
fn the_ceilings_subject_report_equals_the_table_the_ifcopenshell_oracle_measured_from_the_committed_file() {
    use crate::standards::v1::subsets::any::io::export::ifc::testkit::ceilings;
    let oracle: serde_json::Value = serde_json::from_slice(&read_ceilings("🔬️measure/🔣️.json")).expect("the oracle table");
    let ours: serde_json::Value = serde_json::from_str(&projection(&ceilings()).to_json()).expect("the report parses");
    assert_eq!(oracle["schema"], ours["schema"]);
    assert_eq!(oracle["counts"], ours["counts"]);
    assert_eq!(oracle["containment"], ours["containment"]);
    assert_eq!(oracle["volumes"].as_object().expect("volumes").len(), ours["volumes"].as_object().expect("volumes").len());
    for (tag, volume) in ours["volumes"].as_object().expect("volumes") {
        let (measured, written) = (oracle["volumes"][tag].as_f64().expect("a kernel volume"), volume.as_f64().expect("a written volume"));
        assert!((measured - written).abs() < 1e-9 * written.abs().max(1.0), "{tag}: kernel {measured}, written {written}");
    }
}
