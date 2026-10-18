use crate::standards::v1::subsets::any::schema::snapshot::{DoorLeaves, LayerFunction, MaterialCategory, PropertyValue, Swing};
use semio_s_artifact_stdio_contract::part21::Part21Value;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::io::import::ifc::reader::{opt_text, refs, text, Doc};
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::io::export::ifc::data::{parent_key, PARENTS_SET};
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::io::export::ifc::Schema;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::io::import::ifc::spatial::{number_of, single_values, string_of};
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::io::import::ifc::Import;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::io::export::ifc::ifc4::{applicable_entity, measure_type};
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::snapshot::{entries_problem, PropertyDef, PropertyKind, PropertyTemplate, TemplateTarget, BeamType, CeilingType, ClassificationItem, ClassificationSystem, ColumnType, DoorType, Layer, Material, Profile, Rgb, RoofType, SlabType, WallType, WindowType};
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::io::export::ifc::wall_sweeps::SWEEP_SET;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::io::export::ifc::walls::{ATTACH_SET, REVEAL_SET};
#[allow(unused_imports)]
use std::collections::BTreeMap;
use crate::standards::v1::subsets::any::io::import::ifc::data::{category, function, door_operation};
use crate::standards::v1::subsets::any::io::import::ifc::data::*;
use semio_s_artifact_stdio_contract::part21::Part21Decimal;

fn typed(name: &str, item: Part21Value) -> Part21Value {
    Part21Value::Typed { name: name.into(), items: vec![item] }
}

fn real(value: f64) -> Part21Value {
    Part21Value::Real(Part21Decimal::from_f64(value))
}

#[test]
fn typed_ifc_values_map_back_to_property_values() {
    let read = |value: Part21Value| property_value(&value);
    assert_eq!(read(typed("IFCLABEL", Part21Value::Str("a".into()))), Some(PropertyValue::Text { value: "a".into() }));
    assert_eq!(read(typed("IFCINTEGER", Part21Value::Int(3))), Some(PropertyValue::Integer { value: 3 }));
    assert_eq!(read(typed("IFCBOOLEAN", Part21Value::Enum("T".into()))), Some(PropertyValue::Boolean { value: true }));
    assert_eq!(read(typed("IFCLENGTHMEASURE", real(1.0))), Some(PropertyValue::Length { value: 1.0 }));
    assert_eq!(read(typed("IFCAREAMEASURE", real(1.0))), Some(PropertyValue::Area { value: 1.0 }));
    assert_eq!(read(typed("IFCVOLUMEMEASURE", real(1.0))), Some(PropertyValue::Volume { value: 1.0 }));
    assert_eq!(read(typed("IFCPLANEANGLEMEASURE", real(1.0))), Some(PropertyValue::Angle { value: 1.0 }));
    assert_eq!(read(typed("IFCTHERMALTRANSMITTANCEMEASURE", real(1.0))), Some(PropertyValue::Real { value: 1.0 }), "any other number is a real");
    assert_eq!(read(Part21Value::Unset), None);
}

#[test]
fn a_boolean_property_keeps_its_flag_and_a_text_property_its_text() {
    assert_eq!(property_value(&typed("IFCBOOLEAN", Part21Value::Enum("F".into()))), Some(PropertyValue::Boolean { value: false }));
    assert_eq!(property_value(&typed("IFCLABEL", Part21Value::Str("EI60".into()))), Some(PropertyValue::Text { value: "EI60".into() }));
}

#[test]
fn material_categories_and_layer_functions_fall_back_to_other_and_structure() {
    assert_eq!(category("Masonry"), MaterialCategory::Masonry);
    assert_eq!(category("Plaster"), MaterialCategory::Other);
    assert_eq!(function("Insulation"), LayerFunction::Insulation);
    assert_eq!(function("unknown"), LayerFunction::Structure);
}

#[test]
fn door_operations_map_to_leaves_and_swing() {
    assert_eq!(door_operation("SINGLE_SWING_RIGHT"), (DoorLeaves::Single, Swing::Right));
    assert_eq!(door_operation("SINGLE_SWING_LEFT"), (DoorLeaves::Single, Swing::Left));
    assert_eq!(door_operation("DOUBLE_DOOR_SINGLE_SWING").0, DoorLeaves::Double);
    assert_eq!(door_operation("NOTDEFINED"), (DoorLeaves::Single, Swing::Left));
}
