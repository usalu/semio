use crate::standards::v1::subsets::any::io::export::ifc::testkit::house;
use crate::standards::v1::subsets::any::io::export::ifc::{codec, export_ifc2x3, export_ifc4, model_to_part21, Schema};
use crate::standards::v1::subsets::any::io::import::ifc::{import_ifc2x3, import_ifc4};
use crate::{ModelSnapshot, StairFlight};
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
fn every_stair_and_railing_of_the_house_survives_the_round_trip_exactly_in_both_schemas() {
    let model = house();
    assert!(!model.stairs.is_empty() && !model.railings.is_empty());
    let (back, notes) = import_ifc2x3(&export_ifc2x3(&model).expect("2x3 export").0).expect("2x3 import");
    assert_eq!((&back.stairs, &back.railings), (&model.stairs, &model.railings));
    assert!(notes.iter().all(|note| !note.starts_with("IFCSTAIR") && !note.starts_with("IFCRAILING") && !note.starts_with("IFCMEMBER")), "{notes:?}");
    let (back, notes) = import_ifc4(&export_ifc4(&model).expect("4 export").0).expect("4 import");
    assert_eq!((&back.stairs, &back.railings), (&model.stairs, &model.railings));
    assert!(notes.iter().all(|note| !note.starts_with("IFCSTAIR") && !note.starts_with("IFCRAILING") && !note.starts_with("IFCMEMBER")), "{notes:?}");
}

#[test]
fn a_stair_without_its_record_becomes_a_straight_stair_of_its_flights() {
    let model = house();
    let (back, _) = without_row(&model, "Stair");
    assert_eq!(back.stairs.len(), model.stairs.len());
    for (id, stair) in &back.stairs {
        assert!(matches!(stair.flight, StairFlight::Straight), "{id}");
        assert!(stair.width > 0.0 && stair.min_tread > 0.0 && stair.max_riser > 0.0, "{id}");
        assert_eq!(stair.storey, model.stairs[id].storey, "{id}");
    }
}

#[test]
fn a_railing_without_its_record_or_a_polyline_is_reported_not_guessed() {
    let model = house();
    let (back, notes) = without_row(&model, "Railing");
    assert!(back.railings.len() < model.railings.len());
    assert!(notes.iter().any(|note| note.starts_with("IFCRAILING")), "{notes:?}");
}
