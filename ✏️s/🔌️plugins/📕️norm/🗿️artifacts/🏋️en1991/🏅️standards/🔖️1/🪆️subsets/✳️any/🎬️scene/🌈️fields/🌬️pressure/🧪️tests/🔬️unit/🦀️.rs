use super::*;

#[test]
fn the_active_field_is_the_net_pressure_of_larger_magnitude() {
    let ColourChannel::Fields { active, fields } = channel(1000.0, -1.2, 1.0) else { panic!("fields channel") };
    let value = |id: &str| match &fields.iter().find(|f| f.id == id).expect("field").samples {
        FieldSamples::Instance { values } => values[0],
        _ => panic!("instance samples"),
    };
    let [first, second] = part_1_4::c_pi_cases();
    assert!((value(&net_id(first)) - 1000.0 * (-1.2 - first)).abs() < 1e-9);
    assert!((value(&net_id(second)) - 1000.0 * (-1.2 - second)).abs() < 1e-9);
    assert!((value(EXTERNAL) + 1200.0).abs() < 1e-9);
    assert!(value(&active).abs() >= value(&net_id(first)).abs().min(value(&net_id(second)).abs()));
}
