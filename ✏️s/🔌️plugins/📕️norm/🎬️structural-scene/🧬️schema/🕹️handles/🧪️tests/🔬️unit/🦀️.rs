use crate::*;
use value::{FromValue, ToValue};

fn handle() -> Handle {
    Handle {
        id: "member-b1/length/end".into(),
        entity: EntityRef::new(EntityKind::Member, "b1", "members[id=b1]"),
        kind: HandleKind::ScalarAlong { axis: Vec3::new(1.0, 0.0, 0.0) },
        anchor: Vec3::new(4.0, 0.0, 0.0),
        grip: GripStyle::Arrow,
        writes: vec![
            FieldBinding::new("members[id=b1].length", Affine::IDENTITY, "m"),
            FieldBinding { quantise: Some(Quantise::Round { step: 0.5 }), ..FieldBinding::new("members[id=b1].placement.origin.x", Affine { base: 1.0, k: -0.5 }, "m") },
        ],
        snap: SnapRule { grid: None, increment: Some(0.05), targets: vec![], catalogue: vec![4.0], tolerance: 0.02 },
        limits: Some(Interval::new(0.5, 12.0)),
        accessible_name: LocalizedCopy::new("Member B1 length handle", "Längengriff Bauteil B1"),
    }
}

#[test]
fn a_handle_is_valid_and_round_trips() {
    let h = handle();
    assert_eq!(h.validate(), Ok(()));
    assert_eq!(Handle::from_value(h.to_value()).expect("decode"), h);
}

#[test]
fn the_affine_law_and_its_inverse_agree() {
    let map = Affine { base: 2.0, k: -4.0 };
    assert_eq!(map.apply(0.5), 0.0);
    assert!((map.invert(map.apply(1.25)) - 1.25).abs() < 1e-12);
}

#[test]
fn quantisation_rounds_chooses_and_truncates_to_integers() {
    assert!((Quantise::Round { step: 0.05 }.apply(4.26) - 4.25).abs() < 1e-12);
    assert_eq!(Quantise::Choice { values: vec![12.0, 16.0, 20.0, 24.0] }.apply(17.0), 16.0);
    assert_eq!(Quantise::Int.apply(2.5), 3.0);
    assert!((Quantise::Decades { per_decade: 3.0 }.apply(6.34) - 10f64.powf(19.0 / 3.0)).abs() < 1e-6 * 10f64.powf(19.0 / 3.0));
    assert!((Quantise::Decades { per_decade: 1.0 }.apply(5.4) - 1.0e5).abs() < 1e-6);
}

#[test]
fn snapping_prefers_catalogue_values_within_tolerance_then_the_increment() {
    let snap = handle().snap;
    assert_eq!(snap.apply(4.01), 4.0);
    assert!((snap.apply(4.27) - 4.25).abs() < 1e-12);
    assert_eq!(SnapRule::NONE.apply(1.234), 1.234);
}

#[test]
fn drag_writes_are_clamped_snapped_and_bound_per_field() {
    let writes = handle().writes_at(99.0);
    assert_eq!(writes.len(), 2);
    assert_eq!((writes[0].path.as_str(), writes[0].value, writes[0].unit.as_str()), ("members[id=b1].length", 12.0, "m"));
    assert_eq!(writes[1].value, -5.0);
    assert_eq!(handle().writes_at(4.01)[0].value, 4.0);
}

#[test]
fn invalid_handles_report_path_addressed_rules() {
    let mut h = handle();
    h.writes.clear();
    h.limits = Some(Interval::new(2.0, 1.0));
    h.kind = HandleKind::Translate { axis: Vec3::ZERO };
    h.snap.tolerance = -1.0;
    let found: Vec<_> = h.validate().expect_err("invalid").into_iter().map(|v| (v.path, v.rule)).collect();
    for expected in [("$.writes", "non-empty"), ("$.limits", "ordered"), ("$.kind.axis", "non-zero"), ("$.snap.tolerance", "non-negative")] {
        assert!(found.contains(&(expected.0.to_owned(), expected.1.to_owned())), "{expected:?} in {found:?}");
    }
}

#[test]
fn bindings_require_a_path_a_unit_and_an_invertible_map() {
    let binding = FieldBinding { path: String::new(), map: Affine { base: 0.0, k: 0.0 }, unit: String::new(), quantise: Some(Quantise::Choice { values: vec![] }) };
    let found: Vec<_> = binding.validate().expect_err("invalid").into_iter().map(|v| v.rule).collect();
    assert!(found.iter().filter(|r| *r == "non-empty").count() >= 3 && found.contains(&"non-zero".to_owned()));
}

#[test]
fn the_wire_form_tags_handle_kinds() {
    let json = canonical_json(&HandleKind::Angular { axis: Vec3::new(0.0, 0.0, 1.0), centre: Vec3::ZERO });
    assert!(json.contains("\"kind\":\"angular\"") && json.contains("\"centre\""));
    assert_eq!(canonical_json(&HandleKind::DimensionLine), "{\"kind\":\"dimensionLine\"}");
}

#[test]
fn the_angular_zero_direction_is_plus_x_perpendicular_to_the_axis_and_plus_z_for_an_x_axis() {
    let angular = |axis: Vec3| HandleKind::Angular { axis, centre: Vec3::new(1.0, 2.0, 3.0) };
    let about_z = angular(Vec3::new(0.0, 0.0, 2.0));
    assert_eq!(about_z.angular_zero_direction(), Some(Vec3::X));
    let about_y = angular(Vec3::Y);
    assert_eq!(about_y.angular_zero_direction(), Some(Vec3::X));
    let about_x = angular(Vec3::new(-3.0, 0.0, 0.0));
    assert_eq!(about_x.angular_zero_direction(), Some(Vec3::Z));
    assert_eq!(HandleKind::CountStepper.angular_zero_direction(), None);
    assert_eq!(angular(Vec3::ZERO).angular_zero_direction(), None);
}

#[test]
fn the_angular_parameter_is_the_right_handed_angle_and_inverts_exactly() {
    let about_z = HandleKind::Angular { axis: Vec3::Z, centre: Vec3::new(1.0, 2.0, 3.0) };
    let quarter = about_z.angular_s(&Vec3::new(1.0, 5.0, 3.0)).expect("angle");
    assert!((quarter - std::f64::consts::FRAC_PI_2).abs() < 1e-12);
    let negative = about_z.angular_s(&Vec3::new(1.0, -1.0, 3.0)).expect("angle");
    assert!((negative + std::f64::consts::FRAC_PI_2).abs() < 1e-12);
    assert_eq!(about_z.angular_s(&Vec3::new(1.0, 2.0, 9.0)), None);
    let about_y = HandleKind::Angular { axis: Vec3::Y, centre: Vec3::ZERO };
    let toward_minus_z = about_y.angular_s(&Vec3::new(0.0, 0.0, -2.0)).expect("angle");
    assert!((toward_minus_z - std::f64::consts::FRAC_PI_2).abs() < 1e-12, "right-handed about +Y takes +X to -Z");
    for kind in [about_z, about_y, HandleKind::Angular { axis: Vec3::new(1.0, 1.0, 1.0), centre: Vec3::new(0.5, 0.0, -1.0) }] {
        for s in [-3.0, -1.2, 0.0, 0.7, 3.1] {
            let point = kind.angular_point(s, 2.5).expect("point");
            assert!((kind.angular_s(&point).expect("angle") - s).abs() < 1e-9, "{kind:?} s = {s}");
        }
    }
}

fn shapes() -> Vec<ChoiceToken> {
    ["None", "Flat", "Tee"].iter().map(|t| ChoiceToken { token: (*t).into(), label: LocalizedCopy::new(*t, *t) }).collect()
}

#[test]
fn a_token_binding_writes_the_enum_token_at_the_rounded_index() {
    let binding = FieldBinding::token("platedPanels[id=p].stiffenerShape", shapes());
    assert_eq!(binding.validate(), Ok(()));
    assert_eq!(binding.token_at(1.4), Some("Flat"));
    assert_eq!(binding.token_at(1.6), Some("Tee"));
    assert_eq!(binding.token_at(9.0), Some("Tee"));
    assert_eq!(binding.token_at(-3.0), Some("None"));
    assert_eq!(FieldBinding::new("a", Affine::IDENTITY, "m").token_at(1.0), None);
    let handle = Handle { kind: HandleKind::ChoiceStepper, writes: vec![binding], snap: SnapRule { increment: Some(1.0), ..SnapRule::NONE }, limits: Some(Interval::new(0.0, 2.0)), ..handle() };
    assert_eq!(handle.validate(), Ok(()));
    let writes = handle.writes_at(1.9);
    assert_eq!((writes[0].token.as_deref(), writes[0].value), (Some("Tee"), 2.0));
    assert_eq!(Handle::from_value(handle.to_value()).expect("decode"), handle);
}

#[test]
fn a_token_rule_without_or_with_duplicate_tokens_is_invalid() {
    assert!(Quantise::Token { tokens: vec![] }.validate().is_err());
    let mut tokens = shapes();
    tokens[2].token = "None".into();
    assert!(Quantise::Token { tokens }.validate().is_err());
}
