use super::*;

#[test]
fn the_design_fire_load_is_the_active_field_beside_the_characteristic_one() {
    let ColourChannel::Fields { active, fields } = channel(511.0e6, 548.14e6) else { panic!("fields channel") };
    let value = |id: &str| match &fields.iter().find(|f| f.id == id).expect("field").samples {
        FieldSamples::Instance { values } => values[0],
        _ => panic!("instance samples"),
    };
    assert_eq!(active, DESIGN);
    assert!((value(DESIGN) - 548.14e6).abs() < 1.0 && (value(CHARACTERISTIC) - 511.0e6).abs() < 1.0);
    assert!(fields.iter().all(|f| f.unit == QuantityKind::Energy));
}
