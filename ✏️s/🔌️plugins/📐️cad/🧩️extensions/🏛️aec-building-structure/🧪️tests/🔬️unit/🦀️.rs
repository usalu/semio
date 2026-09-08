
use super::*;

#[semio_framework_async_macros::async_test]
async fn bundle_contributes_structure_manifest() {
    let manifest = bundle().manifest;
    let topic_contribution = &manifest.topic_contributions[0];
    assert_eq!(topic_contribution.topic, "cad.computer");
    let computers_json = topic_contribution.payload["computersJson"].as_str().expect("computersJson");
    let parsed = json::parse(computers_json).expect("parse");
    assert_eq!(parsed.get("importProfiles").and_then(JsonValue::as_array).map(|rows| rows.len()), Some(5));
    assert_eq!(parsed.get("transformationAppliers"), Some(&json::array([JsonValue::from("aec.building.structure/from_building")])));
}
