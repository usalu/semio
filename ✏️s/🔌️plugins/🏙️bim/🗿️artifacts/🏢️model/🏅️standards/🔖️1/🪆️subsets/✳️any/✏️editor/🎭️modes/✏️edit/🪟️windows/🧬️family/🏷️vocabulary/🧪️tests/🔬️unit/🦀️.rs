use super::*;
use crate::editor::bim::modes::edit::windows::family::edit::{AXES, CATEGORIES, KINDS, SHAPES};

#[test]
fn every_category_kind_and_shape_has_a_distinct_name_in_both_languages() {
    for labels in [&BimLabels::NATIVE_EN, &BimLabels::NATIVE_DE] {
        let categories: std::collections::BTreeSet<String> = CATEGORIES.iter().map(|category| category_label(labels, *category)).collect();
        let kinds: std::collections::BTreeSet<String> = KINDS.iter().map(|kind| kind_label(labels, *kind)).collect();
        let shapes: std::collections::BTreeSet<String> = SHAPES.iter().map(|shape| shape_label(labels, shape)).collect();
        assert_eq!((categories.len(), kinds.len(), shapes.len()), (CATEGORIES.len(), KINDS.len(), SHAPES.len()));
    }
    assert_eq!(category_label(&BimLabels::NATIVE_DE, FamilyCategory::Furniture), "Möbel");
    assert_eq!(kind_label(&BimLabels::NATIVE_EN, ParameterKind::Length), "Length");
    assert_eq!(shape_label(&BimLabels::NATIVE_DE, "Revolution"), "Rotationskörper");
    assert_eq!(AXES.map(axis_label), ["X", "Y", "Z"]);
}

#[test]
fn values_read_in_millimetres_degrees_numbers_truth_values_and_texts() {
    let en = &BimLabels::NATIVE_EN;
    assert_eq!(value_text(en, &ParameterValue::Length { value: 1.04 }), "1040 mm");
    assert_eq!(value_text(en, &ParameterValue::Length { value: 0.0065 }), "6.5 mm");
    assert_eq!(value_text(en, &ParameterValue::Angle { value: std::f64::consts::FRAC_PI_2 }), "90°");
    assert_eq!(value_text(en, &ParameterValue::Number { value: 4.0 }), "4");
    assert_eq!(value_text(en, &ParameterValue::Number { value: 2.08 }), "2.08");
    assert_eq!(value_text(en, &ParameterValue::Boolean { value: true }), "yes");
    assert_eq!(value_text(&BimLabels::NATIVE_DE, &ParameterValue::Boolean { value: false }), "nein");
    assert_eq!(value_text(en, &ParameterValue::Text { value: "oak".into() }), "oak");
}
