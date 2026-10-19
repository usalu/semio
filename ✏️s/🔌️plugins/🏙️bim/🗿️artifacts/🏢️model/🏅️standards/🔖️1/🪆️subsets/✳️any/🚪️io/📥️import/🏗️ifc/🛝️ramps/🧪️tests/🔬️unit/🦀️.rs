#[allow(unused_imports)]
use semio_s_artifact_bim_model_ifc_2::standards::v1::subsets::any::io::import::ifc::data::label;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::import::ifc::reader::{text, Doc};
#[allow(unused_imports)]
use semio_s_artifact_bim_model_ifc_2::standards::v1::subsets::any::io::import::ifc::spatial::authoring_of;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_ifc_2::standards::v1::subsets::any::io::import::ifc::Import;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::Ramp;
#[allow(unused_imports)]
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};
use semio_s_artifact_bim_model_ifc::standards::v1::subsets::any::io::export::ifc::export_ifc2x3;
use semio_s_artifact_bim_model_ifc::standards::v1::subsets::any::io::export::ifc::testkit::with_ramp;
use semio_s_artifact_bim_model_ifc_2::standards::v1::subsets::any::io::import::ifc::import_ifc2x3;

#[test]
fn a_ramp_survives_the_round_trip_exactly() {
    let model = with_ramp(true);
    let (bytes, _) = export_ifc2x3(&model).expect("the model exports");
    let (back, notes) = import_ifc2x3(&bytes).expect("the file imports");
    assert!(notes.iter().all(|note| !note.contains("IFCRAMP")), "{notes:?}");
    assert_eq!(back.ramps.keys().collect::<Vec<_>>(), ["rp-1"]);
    assert_eq!(back.ramps["rp-1"], model.ramps["rp-1"]);
}

#[test]
fn export_import_export_of_a_ramp_is_byte_stable() {
    let mut model = with_ramp(true);
    model.roofs.clear();
    model.stairs.clear();
    model.railings.clear();
    model.curtain_walls.clear();
    model.slabs.remove("sl-balcony");
    model.spaces.remove("sp-1");
    let (first, _) = export_ifc2x3(&model).expect("first export");
    let (back, _) = import_ifc2x3(&first).expect("the import");
    let (second, _) = export_ifc2x3(&back).expect("second export");
    assert_eq!(first, second);
}

#[test]
fn a_foreign_ramp_without_the_authored_record_is_reported_not_imported() {
    let model = with_ramp(false);
    let (bytes, _) = export_ifc2x3(&model).expect("the model exports");
    let text = String::from_utf8(bytes).expect("a text file").replace("'Ramp'", "'Slope'");
    let (back, notes) = import_ifc2x3(text.as_bytes()).expect("the file imports");
    assert!(back.ramps.is_empty());
    assert!(notes.iter().any(|note| note.contains("IFCRAMP") && note.contains("no authored ramp record")), "{notes:?}");
}

#[test]
fn a_ramp_outside_a_storey_is_skipped() {
    let mut model = with_ramp(false);
    model.ramps.get_mut("rp-1").expect("the ramp").storey = "st-ghost".into();
    let exported = export_ifc2x3(&model).expect("the model exports").0;
    let (back, _) = import_ifc2x3(&exported).expect("the file imports");
    assert!(back.ramps.is_empty());
}
