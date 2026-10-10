use crate::standards::v1::subsets::any::io::export::ifc::testkit::house;
use crate::standards::v1::subsets::any::io::export::ifc::{codec, export_ifc2x3, export_ifc4, model_to_part21, Schema};
use crate::standards::v1::subsets::any::io::import::ifc::{import_ifc2x3, import_ifc4};
use crate::ModelSnapshot;
use semio_s_artifact_stdio_ifc::part21::Part21Value;

fn without_row(model: &ModelSnapshot, row: &str) -> (ModelSnapshot, Vec<String>) {
    let mut document = model_to_part21(Schema::Ifc2x3, model).expect("the model exports").0;
    for instance in &mut document.instances {
        for (name, args) in &mut instance.entities {
            if name == "IFCPROPERTYSINGLEVALUE" && args[0].as_str() == Some(row) {
                args[0] = Part21Value::Str("Gone".into());
            }
        }
    }
    import_ifc2x3(&codec::encode_document(document).expect("the document encodes")).expect("the file imports")
}

#[test]
fn every_roof_of_the_house_survives_the_round_trip_exactly_in_both_schemas() {
    let model = house();
    assert!(!model.roofs.is_empty());
    let (back, notes) = import_ifc2x3(&export_ifc2x3(&model).expect("2x3 export").0).expect("2x3 import");
    assert_eq!(back.roofs, model.roofs);
    assert!(notes.iter().all(|note| !note.starts_with("IFCROOF")), "{notes:?}");
    let (back, notes) = import_ifc4(&export_ifc4(&model).expect("4 export").0).expect("4 import");
    assert_eq!(back.roofs, model.roofs);
    assert!(notes.iter().all(|note| !note.starts_with("IFCROOF")), "{notes:?}");
}

#[test]
fn the_slab_layers_of_a_roof_are_its_parts_and_are_not_imported_as_slabs() {
    let model = house();
    let (back, _) = import_ifc4(&export_ifc4(&model).expect("4 export").0).expect("4 import");
    assert_eq!(back.slabs.keys().collect::<Vec<_>>(), model.slabs.keys().collect::<Vec<_>>());
}

#[test]
fn a_pitched_roof_without_its_record_is_reported_not_guessed() {
    let model = house();
    let (back, notes) = without_row(&model, "Roof");
    assert!(back.roofs.is_empty());
    assert!(notes.iter().any(|note| note.starts_with("IFCROOF")), "{notes:?}");
}
