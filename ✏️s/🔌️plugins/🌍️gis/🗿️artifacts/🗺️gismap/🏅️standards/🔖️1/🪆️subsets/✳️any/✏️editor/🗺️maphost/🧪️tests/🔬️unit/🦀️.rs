
use super::*;
use crate::schema::default_document;

#[semio_framework_async_macros::async_test]
async fn the_host_mirrors_the_document_features_and_the_config_camera() {
    let document = default_document();
    let config = Gis2dConfig { camera_json: r#"{"x":10,"y":20,"zoom":4}"#.into(), ..Gis2dConfig::default() };
    let host = map_host_from(&document, &config);
    assert!(!host.features.positions.is_empty(), "the reuse-map fixture seeds position features");
    let camera: Value = serde_json::from_str(&host.camera_json()).expect("camera json");
    assert_eq!(camera.get("zoom").and_then(Value::as_f64), Some(4.0));
}

#[semio_framework_async_macros::async_test]
async fn a_malformed_camera_json_leaves_the_host_at_its_own_default() {
    let config = Gis2dConfig { camera_json: "not json".into(), ..Gis2dConfig::default() };
    let host = map_host_from(&GisMapSnapshot::default(), &config);
    assert!(serde_json::from_str::<Value>(&host.camera_json()).is_ok(), "the host still reports a valid camera");
}
