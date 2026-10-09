use super::*;
use super::super::tests::kit;
use super::super::family_of;
use crate::{FamilyCategory, ParameterKind};

fn model() -> crate::ModelSnapshot {
    let mut snapshot = kit::model();
    kit::hea(&mut snapshot);
    kit::family(&mut snapshot, "fam-table", FamilyCategory::Furniture);
    kit::parameter(&mut snapshot, "fam-table", "top", ParameterKind::Length, "ghost + 1 m");
    snapshot
}

fn values(snapshot: &crate::ModelSnapshot) -> BTreeMap<String, FamilyValue> {
    snapshot.families.keys().map(|id| (id.clone(), family_of(snapshot, id, &BTreeMap::new()))).collect()
}

#[test]
fn the_metrics_name_values_issues_solids_and_outline_area() {
    let table = values(&model());
    let hea = metrics_of(&table["fam-hea"]);
    assert_eq!(hea.category, "Profile");
    assert!(hea.issues.is_empty());
    assert_eq!(hea.outline_vertices, 12);
    let area = 2.0 * 0.2 * 0.01 + (0.19 - 2.0 * 0.01) * 0.0065;
    assert!((hea.outline_area - area).abs() < 1e-12, "{}", hea.outline_area);
    let solid = &hea.solids["s-hea"];
    assert!((solid.volume - area * 3.0).abs() < 1e-12);
    assert!(solid.visible && solid.triangles > 0);
    assert_eq!(solid.max.len(), 3);
    assert!((solid.max[2] - solid.min[2] - 3.0).abs() < 1e-12);
    let broken = metrics_of(&table["fam-table"]);
    assert_eq!(broken.issues, vec!["unknown|parameter|top|value".to_string()]);
    assert!(broken.parameters["top"].value.is_none());
}

#[test]
fn the_table_is_canonical_json_with_one_row_per_family() {
    let json = table_json(&values(&model()));
    let parsed: serde_json::Value = serde_json::from_str(&json).expect("JSON");
    let rows = parsed.as_object().expect("object");
    assert_eq!(rows.keys().collect::<Vec<_>>(), ["fam-hea", "fam-table"]);
    assert_eq!(parsed["fam-hea"]["parameters"]["h"]["value"]["Length"]["value"], 0.19);
    assert_eq!(parsed["fam-table"]["parameters"]["top"]["value"], serde_json::Value::Null);
    assert_eq!(table_json(&values(&model())), json, "deterministic");
}

#[test]
fn magnitude_reads_numbers_only() {
    assert_eq!(magnitude(&ParameterValue::Length { value: 2.0 }), Some(2.0));
    assert_eq!(magnitude(&ParameterValue::Angle { value: 1.5 }), Some(1.5));
    assert_eq!(magnitude(&ParameterValue::Number { value: 3.0 }), Some(3.0));
    assert_eq!(magnitude(&ParameterValue::Boolean { value: true }), None);
    assert_eq!(magnitude(&ParameterValue::Text { value: "x".into() }), None);
}
