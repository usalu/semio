
use super::*;

#[semio_framework_async_macros::async_test]
async fn bundle_contributes_energy_computers() {
    let manifest = bundle().manifest;
    let topic_contribution = &manifest.topic_contributions[0];
    assert_eq!(topic_contribution.topic, "cad.computer");
    let computers_json = topic_contribution.payload["computersJson"].as_str().expect("computersJson");
    let parsed = json::parse(computers_json).expect("parse");
    assert_eq!(parsed.get("statComputers"), Some(&json::array([JsonValue::from("energy.demand")])));
    assert_eq!(parsed.get("propertyComputers"), Some(&json::array([JsonValue::from("energy.heatedvolume")])));
}
