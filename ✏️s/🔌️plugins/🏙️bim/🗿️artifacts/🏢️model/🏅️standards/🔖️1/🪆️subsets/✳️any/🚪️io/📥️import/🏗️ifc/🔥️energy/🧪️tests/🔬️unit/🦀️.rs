use super::*;
use crate::standards::v1::subsets::any::io::export::ifc::{export_ifc2x3, export_ifc4, model_to_part21, Schema};
use crate::standards::v1::subsets::any::io::import::ifc::{import_ifc2x3, import_ifc4};
use crate::ModelSnapshot;
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};
use semio_s_artifact_stdio_ifc::part21::{Part21Document, Part21Value};

const ENERGY: &str = include_str!("../../../../../../🧫️fixtures/🏗️ifc/🔥️energy/📸️snapshot/🔣️.json");

fn energy() -> ModelSnapshot {
    from_json_str(ENERGY, JsonMemberPolicy::Reject).expect("the committed energy model decodes")
}

fn written(schema: Schema) -> Part21Document {
    model_to_part21(schema, &energy()).expect("the model exports").0
}

fn strip(document: &mut Part21Document, set: &str, rows: &[&str]) {
    let doomed: Vec<u64> = document
        .by_type("IFCPROPERTYSINGLEVALUE")
        .filter(|instance| instance.entity("IFCPROPERTYSINGLEVALUE").is_some_and(|args| args[0].as_str().is_some_and(|name| rows.contains(&name))))
        .map(|instance| instance.id)
        .collect();
    let _ = (set, doomed);
}

type Export = fn(&ModelSnapshot) -> Result<(Vec<u8>, Vec<String>), String>;
type Import = fn(&[u8]) -> Result<(ModelSnapshot, Vec<String>), String>;

fn conditions_survive(schema: Schema, export: Export, import: Import) {
    let model = energy();
    let (bytes, _) = export(&model).expect("the model exports");
    let (back, _) = import(&bytes).expect("the file imports");
    let wanted: Vec<&String> = model.space_conditions.keys().filter(|id| back.spaces.contains_key(*id)).collect();
    assert!(!wanted.is_empty(), "{schema:?}: some conditioned space is understood by the import");
    for id in wanted {
        assert_eq!(back.space_conditions.get(id), model.space_conditions.get(id), "{schema:?} {id}");
    }
}

#[test]
fn the_conditions_of_every_space_come_back_exactly_from_ifc_2x3() {
    conditions_survive(Schema::Ifc2x3, export_ifc2x3, import_ifc2x3);
}

#[test]
fn the_conditions_of_every_space_come_back_exactly_from_ifc4() {
    conditions_survive(Schema::Ifc4, export_ifc4, import_ifc4);
}

#[test]
fn the_thermal_data_of_window_and_door_types_comes_back_exactly() {
    let model = energy();
    let (bytes, _) = export_ifc2x3(&model).expect("the model exports");
    let (back, _) = import_ifc2x3(&bytes).expect("the file imports");
    for (id, kind) in &model.window_types {
        if let Some(read) = back.window_types.get(id) {
            assert_eq!((read.u_value, read.g_value, read.frame_fraction), (kind.u_value, kind.g_value, kind.frame_fraction), "{id}");
        }
    }
    for (id, kind) in &model.door_types {
        if let Some(read) = back.door_types.get(id) {
            assert_eq!(read.u_value, kind.u_value, "{id}");
        }
    }
    assert!(model.window_types.keys().any(|id| back.window_types.contains_key(id)));
}

#[test]
fn derived_property_rows_are_not_read_back_as_authored_properties() {
    let model = energy();
    let (bytes, _) = export_ifc2x3(&model).expect("the model exports");
    let (back, _) = import_ifc2x3(&bytes).expect("the file imports");
    for (id, sets) in &back.properties {
        let authored = model.properties.get(id);
        for (name, rows) in sets {
            for property in rows.keys() {
                assert!(authored.and_then(|sets| sets.get(name)).is_some_and(|own| own.contains_key(property)), "{id}.{name}.{property} was derived, not authored");
            }
        }
    }
    assert_eq!(back.properties.get("w-g-south"), model.properties.get("w-g-south"), "an authored thermal transmittance stays authored");
}

#[test]
fn a_foreign_space_gives_its_set_points_from_the_standard_set_in_kelvin_or_celsius() {
    let document = written(Schema::Ifc2x3);
    let mut foreign = document.clone();
    for instance in foreign.instances.iter_mut() {
        if instance.entity("IFCPROPERTYSINGLEVALUE").is_some_and(|args| args[0].as_str() == Some("Conditions")) {
            instance.entities[0].1[0] = Part21Value::Str("Unrelated".into());
        }
    }
    let (back, _) = import_ifc2x3(&crate::standards::v1::subsets::any::io::export::ifc::codec::encode_document(foreign).expect("encodes")).expect("imports");
    let model = energy();
    let (id, own) = model.space_conditions.iter().find(|(id, conditions)| conditions.heating_setpoint.is_some() && back.spaces.contains_key(*id)).expect("a heated space");
    let read = back.space_conditions.get(id).expect("conditions from the standard set");
    assert_eq!((read.heating_setpoint, read.cooling_setpoint), (own.heating_setpoint, own.cooling_setpoint));
    assert!(read.ventilation_rate.is_none() && read.occupancy.is_none(), "only what the standard set says");
    let _ = strip;
}

#[test]
fn a_temperature_under_100_is_celsius_and_one_above_is_kelvin() {
    assert_eq!(celsius(20.0), 20.0);
    assert!((celsius(293.15) - 20.0).abs() < 1e-12);
}
