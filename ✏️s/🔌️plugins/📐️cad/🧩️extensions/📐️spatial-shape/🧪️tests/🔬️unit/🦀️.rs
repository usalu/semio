use super::*;

#[semio_framework_async_macros::async_test]
async fn bundle_contributes_spatial_shape_for_cad_play() {
    let mut original_caller=semio_framework_plugin::app::OriginalFixtureCaller::admit().expect("original extension fixture caller");let mut cx=original_caller.context();
    let manifest = bundle().into_manifest_cold(&mut cx).unwrap();
    assert_eq!(manifest.extends, "cad");
    assert_eq!(manifest.topic_contributions.len(), 1);
    let topic_contribution = &manifest.topic_contributions[0];
    assert_eq!(topic_contribution.topic, "cad.computer");
    assert_eq!(topic_contribution.payload["appId"].as_str(), Some(HOST_APP_ID));
    assert_eq!(topic_contribution.payload["moduleId"].as_str(), Some(MODULE_ID));
    let computers_json = topic_contribution.payload["computersJson"].as_str().expect("computersJson");
    let parsed = json::parse(computers_json).expect("computers_json");
    assert_eq!(parsed.get("statComputers"), Some(&json::array([json::Value::from("spatial.shape.geometry")])));

    let fuel=cx.fuel_remaining();drop(cx);original_caller.retire(fuel).expect("original extension fixture source close");
}
