use super::*;
use semio_s_artifact_stdio_ifc::part21::Part21Decimal;

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
