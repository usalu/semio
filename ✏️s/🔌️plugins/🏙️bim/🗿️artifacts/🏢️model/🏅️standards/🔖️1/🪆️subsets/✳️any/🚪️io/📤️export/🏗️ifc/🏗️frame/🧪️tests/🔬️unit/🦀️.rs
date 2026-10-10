use crate::standards::v1::subsets::any::io::export::ifc::testkit::{count, document, house, quantity, real, rows, string, tags, target};
use semio_s_artifact_stdio_ifc::part21::{Part21Document, Part21Instance, Part21Value};

fn solid_of<'a>(document: &'a Part21Document, args: &[Part21Value]) -> &'a Vec<Part21Value> {
    let shape = target(document, args, 6).and_then(|instance| instance.entity("IFCPRODUCTDEFINITIONSHAPE")).expect("the shape");
    let representation = document.resolve(&shape[2].as_list().expect("representations")[0]).and_then(|instance| instance.entity("IFCSHAPEREPRESENTATION")).expect("the representation");
    document.resolve(&representation[3].as_list().expect("items")[0]).and_then(|instance| instance.entity("IFCEXTRUDEDAREASOLID")).expect("the solid")
}

#[test]
fn columns_extrude_their_profile_over_the_resolved_height() {
    let document = document(&house());
    assert_eq!(tags(&document, "IFCCOLUMN"), ["c-1", "c-2"]);
    let (_, rectangle) = rows(&document, "IFCCOLUMN").into_iter().find(|(_, args)| string(args, 7).as_deref() == Some("c-1")).expect("c-1");
    let solid = solid_of(&document, rectangle);
    assert!((real(solid, 3) - 3.0).abs() < 1e-9, "the ground storey height");
    let profile = target(&document, solid, 0).and_then(|instance| instance.entity("IFCRECTANGLEPROFILEDEF")).expect("a rectangle profile");
    assert_eq!((real(profile, 3), real(profile, 4)), (0.3, 0.3));
    let (_, round) = rows(&document, "IFCCOLUMN").into_iter().find(|(_, args)| string(args, 7).as_deref() == Some("c-2")).expect("c-2");
    assert!((real(solid_of(&document, round), 3) - 2.5).abs() < 1e-9, "the unconnected height");
    let circle = target(&document, solid_of(&document, round), 0).and_then(|instance| instance.entity("IFCCIRCLEPROFILEDEF")).expect("a circle");
    assert!((real(circle, 3) - 0.2).abs() < 1e-9);
}

#[test]
fn a_rotated_column_turns_its_placement_reference_direction() {
    let document = document(&house());
    let (_, rotated) = rows(&document, "IFCCOLUMN").into_iter().find(|(_, args)| string(args, 7).as_deref() == Some("c-1")).expect("c-1");
    let placement = target(&document, rotated, 5).and_then(|instance| instance.entity("IFCLOCALPLACEMENT")).expect("the placement");
    let axes = document.resolve(&placement[1]).and_then(|instance| instance.entity("IFCAXIS2PLACEMENT3D")).expect("the axes");
    let direction = document.resolve(&axes[2]).and_then(|instance| instance.entity("IFCDIRECTION")).expect("the reference direction");
    let components: Vec<f64> = direction[0].as_list().expect("components").iter().filter_map(Part21Value::as_real).collect();
    assert!((components[0] - 0.3f64.cos()).abs() < 1e-8 && (components[1] - 0.3f64.sin()).abs() < 1e-8);
}

#[test]
fn beams_extrude_along_their_axis_with_the_profile_top_on_the_storey_top() {
    let document = document(&house());
    assert_eq!(tags(&document, "IFCBEAM"), ["b-1", "b-2"]);
    let (_, steel) = rows(&document, "IFCBEAM").into_iter().find(|(_, args)| string(args, 7).as_deref() == Some("b-1")).expect("b-1");
    let solid = solid_of(&document, steel);
    assert!((real(solid, 3) - 2.0).abs() < 1e-9, "the beam length");
    assert_eq!(count(&document, "IFCISHAPEPROFILEDEF"), 1);
    let placement = target(&document, steel, 5).and_then(|instance| instance.entity("IFCLOCALPLACEMENT")).expect("the placement");
    let axes = document.resolve(&placement[1]).and_then(|instance| instance.entity("IFCAXIS2PLACEMENT3D")).expect("the axes");
    let location = document.resolve(&axes[0]).and_then(|instance| instance.entity("IFCCARTESIANPOINT")).expect("the location");
    let z = location[0].as_list().expect("coordinates")[2].as_real().expect("a z");
    assert!((z - (3.0 - 0.15)).abs() < 1e-9, "the 300 mm deep I-shape hangs from the ground storey top: {z}");
}

#[test]
fn frames_carry_area_and_volume_quantities() {
    let document = document(&house());
    let column_volume: Vec<f64> = rows(&document, "IFCELEMENTQUANTITY")
        .iter()
        .filter(|(_, args)| string(args, 2).as_deref() == Some("Qto_ColumnBaseQuantities"))
        .flat_map(|(_, args)| args[5].as_list().expect("quantities").iter().filter_map(|item| document.resolve(item)).filter_map(|row| row.entity("IFCQUANTITYVOLUME")).map(|row| real(row, 3)).collect::<Vec<f64>>())
        .collect();
    assert!(column_volume.iter().any(|volume| (volume - 0.09 * 3.0).abs() < 1e-9));
    assert!(column_volume.iter().any(|volume| (volume - std::f64::consts::PI * 0.04 * 2.5).abs() < 1e-9));
    assert!(count(&document, "IFCRELASSOCIATESMATERIAL") > 0);
}

use crate::standards::v1::subsets::any::io::export::ifc::testkit::{property_sets, text_of};
use crate::{Axis, Beam, Column, ModelSnapshot, Point2, Slope};

/// 🏗️ The house with a leaning column, an arc beam, an inclined beam and a beam joined to two columns, on the storey of its first column.
pub(crate) fn leaning() -> ModelSnapshot {
    let mut model = house();
    let template = model.columns.values().next().cloned().expect("a column");
    let beam_type = model.beams.values().next().map(|beam| beam.beam_type.clone()).expect("a beam");
    let storey = template.storey.clone();
    let at = |x: f64, y: f64| Point2 { x, y };
    model.columns.insert("c-lean".into(), Column { position: at(30.0, 30.0), rotation: 0.0, tilt: Some(Slope { direction: 0.5, angle: 0.2 }), ..template.clone() });
    model.columns.insert("c-west".into(), Column { position: at(40.0, 30.0), tilt: None, ..template.clone() });
    model.columns.insert("c-east".into(), Column { position: at(46.0, 30.0), tilt: None, ..template });
    let beam = |axis: Axis, end_top_offset: Option<f64>| Beam { storey: storey.clone(), beam_type: beam_type.clone(), axis, top_offset: 0.0, end_top_offset, phase: crate::Phase::New, name: String::new() };
    model.beams.insert("b-arc".into(), beam(Axis::Arc { start: at(30.0, 40.0), end: at(36.0, 40.0), bulge: 0.4 }, None));
    model.beams.insert("b-incline".into(), beam(Axis::Line { start: at(30.0, 50.0), end: at(36.0, 50.0) }, Some(-0.8)));
    model.beams.insert("b-joined".into(), beam(Axis::Line { start: at(40.0, 30.0), end: at(46.0, 30.0) }, None));
    model
}

fn shape_items<'a>(document: &'a Part21Document, args: &[Part21Value]) -> Vec<(String, &'a Part21Instance)> {
    let shape = target(document, args, 6).and_then(|instance| instance.entity("IFCPRODUCTDEFINITIONSHAPE")).expect("the shape");
    shape[2]
        .as_list()
        .expect("representations")
        .iter()
        .filter_map(|item| document.resolve(item))
        .filter_map(|instance| instance.entity("IFCSHAPEREPRESENTATION").map(|args| (string(args, 1).unwrap_or_default(), instance)))
        .flat_map(|(identifier, instance)| {
            let items = instance.entity("IFCSHAPEREPRESENTATION").expect("a representation")[3].as_list().expect("items").iter().filter_map(|item| document.resolve(item)).map(|item| (identifier.clone(), item)).collect::<Vec<_>>();
            items
        })
        .collect()
}

#[test]
fn a_leaning_column_extrudes_its_stretched_section_along_the_lean() {
    let document = document(&leaning());
    let (_, args) = rows(&document, "IFCCOLUMN").into_iter().find(|(_, args)| string(args, 7).as_deref() == Some("c-lean")).expect("c-lean");
    let (_, solid) = shape_items(&document, args).into_iter().find(|(_, item)| item.is_type("IFCEXTRUDEDAREASOLID")).expect("a swept body");
    let solid = solid.entity("IFCEXTRUDEDAREASOLID").expect("the solid");
    let direction = target(&document, solid, 2).and_then(|instance| instance.entity("IFCDIRECTION")).expect("the extrusion direction");
    let components: Vec<f64> = direction[0].as_list().expect("components").iter().filter_map(Part21Value::as_real).collect();
    let (sine, cosine) = (0.2f64.sin(), 0.2f64.cos());
    assert!((components[0] - sine * 0.5f64.cos()).abs() < 1e-8 && (components[1] - sine * 0.5f64.sin()).abs() < 1e-8 && (components[2] - cosine).abs() < 1e-8, "{components:?}");
    let height = quantity(&document, "c-lean", "Qto_ColumnBaseQuantities", "Length").expect("a length") * cosine;
    assert!((real(solid, 3) - height / cosine).abs() < 1e-9, "the depth is the length along the lean");
    assert!(target(&document, solid, 0).is_some_and(|profile| profile.is_type("IFCARBITRARYCLOSEDPROFILEDEF")), "the horizontal section is a polygon");
    let rows = property_sets(&document, "c-lean", "Semio_Authoring");
    assert!(rows.iter().any(|row| text_of(row, "Column").is_some_and(|record| record.contains("\"tilt\""))), "the lean travels as the authored record");
}

#[test]
fn an_arc_an_inclined_and_a_joined_beam_are_breps_with_their_axis_curve_and_record() {
    let document = document(&leaning());
    for (id, trimmed_curve) in [("b-arc", true), ("b-incline", false), ("b-joined", false)] {
        let (_, args) = rows(&document, "IFCBEAM").into_iter().find(|(_, args)| string(args, 7).as_deref() == Some(id)).unwrap_or_else(|| panic!("{id}"));
        let items = shape_items(&document, args);
        assert!(items.iter().any(|(identifier, item)| identifier == "Body" && item.is_type("IFCFACETEDBREP")), "{id} is a brep");
        let axis = items.iter().find(|(identifier, _)| identifier == "Axis").map(|(_, item)| item).unwrap_or_else(|| panic!("{id} has an axis curve"));
        assert_eq!(axis.is_type("IFCTRIMMEDCURVE"), trimmed_curve, "{id}");
        assert!(property_sets(&document, id, "Semio_Authoring").iter().any(|row| text_of(row, "Beam").is_some()), "{id} carries its record");
    }
    let net = quantity(&document, "b-joined", "Qto_BeamBaseQuantities", "NetVolume").expect("a net volume");
    let gross = quantity(&document, "b-joined", "Qto_BeamBaseQuantities", "GrossVolume").expect("a gross volume");
    assert!(net < gross - 1e-9, "the columns cut both ends back: {net} against {gross}");
}

#[test]
fn a_plain_beam_stays_a_swept_extrusion_without_a_record() {
    let document = document(&leaning());
    let (_, args) = rows(&document, "IFCBEAM").into_iter().find(|(_, args)| string(args, 7).as_deref() == Some("b-1")).expect("b-1");
    assert!(shape_items(&document, args).iter().all(|(identifier, item)| identifier == "Body" && item.is_type("IFCEXTRUDEDAREASOLID")));
    assert!(property_sets(&document, "b-1", "Semio_Authoring").iter().all(|row| text_of(row, "Beam").is_none()));
}

const FRAME_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🏗️ifc/🏗️frame");

/// 🏗️ The frame model: [`leaning`] with the door, open and window panels of the curtain wall of the curtain wall export tests.
pub(crate) fn frame() -> ModelSnapshot {
    let mut model = leaning();
    model.curtain_panel_overrides = crate::standards::v1::subsets::any::io::export::ifc::curtain::tests::framed().curtain_panel_overrides;
    model
}

fn read_frame(name: &str) -> Vec<u8> {
    std::fs::read(format!("{FRAME_DIR}/{name}")).unwrap_or_else(|error| panic!("{name}: {error}. Run the test with BIM_BLESS=1 to write the file, then `python 🐍️.py write` of the export case."))
}

#[test]
fn the_committed_frame_snapshot_is_the_frame_model() {
    let model = frame();
    if std::env::var("BIM_BLESS").is_ok() {
        let value: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&model)).expect("the snapshot is JSON");
        let path = format!("{FRAME_DIR}/📸️snapshot/🔣️.json");
        std::fs::create_dir_all(format!("{FRAME_DIR}/📸️snapshot")).expect("the folder");
        let _ = std::fs::remove_file(&path);
        std::fs::write(path, serde_json::to_string_pretty(&value).expect("pretty JSON") + "\n").expect("the snapshot is written");
    }
    let committed = String::from_utf8(read_frame("📸️snapshot/🔣️.json")).expect("UTF-8");
    let decoded: ModelSnapshot = semio_framework_pack_json::from_json_str(&committed, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the committed snapshot decodes");
    assert!(decoded == model, "the committed frame snapshot drifted: rewrite it with BIM_BLESS=1");
}

#[test]
fn the_committed_frame_file_is_the_current_export() {
    let (bytes, notes) = crate::standards::v1::subsets::any::io::export::ifc::export_ifc2x3(&frame()).expect("the frame exports");
    assert!(notes.is_empty(), "{notes:?}");
    if std::env::var("BIM_BLESS").is_ok() {
        let path = format!("{FRAME_DIR}/🏗️frame.ifc");
        let _ = std::fs::remove_file(&path);
        std::fs::write(path, &bytes).expect("the file is written");
    }
    assert_eq!(read_frame("🏗️frame.ifc"), bytes, "the committed export drifted: rewrite it with BIM_BLESS=1");
}

#[test]
fn the_frame_subject_report_equals_the_table_the_ifcopenshell_oracle_measured_from_the_committed_file() {
    use crate::standards::v1::subsets::any::io::export::ifc::projection::projection;
    let oracle: serde_json::Value = serde_json::from_slice(&read_frame("🔬️measure/🔣️.json")).expect("the oracle table");
    let ours: serde_json::Value = serde_json::from_str(&projection(&frame()).to_json()).expect("the report parses");
    assert_eq!(oracle["schema"], ours["schema"]);
    assert_eq!(oracle["counts"], ours["counts"]);
    assert_eq!(oracle["containment"], ours["containment"]);
    assert_eq!(oracle["volumes"], ours["volumes"]);
    assert!(ours["volumes"].as_object().is_some_and(|volumes| ["c-lean", "b-arc", "b-incline", "b-joined"].iter().all(|id| volumes.contains_key(*id))), "the kernel measures every frame element");
}
