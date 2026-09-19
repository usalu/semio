#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

/// 🛰️ What `setActiveExample` does at boot: resolve the catalogue id and frame the document. The
/// asset once carried `positions`/`routes` as flat `{id, lon, lat, …}` objects, which
/// `MapFeature`'s `deny_unknown_fields` rejects (`0.unknown field ‹lon›`) — the app then opened
/// empty with a refused `dispatch-failed` on every boot, so the payload shape is pinned here and
/// the renderer's descriptor projection is required to still find a coordinate on it.
#[cfg(feature = "component-app-assembly")]
#[semio_framework_async_macros::async_test]
async fn default_example_document_carries_addressable_features() {
    use crate::editor::gis2d::commands::example::{example_document, DEFAULT_EXAMPLE_ID};
    let document = example_document(DEFAULT_EXAMPLE_ID).expect("default example resolves");
    assert!(!document.positions.is_empty(), "the demo map has positions");
    assert!(!document.routes.is_empty(), "the demo map has routes");
    let descriptor: serde_json::Value = serde_json::from_str(&crate::schema::gis_map_descriptor_json(&document)).expect("descriptor json");
    let first = descriptor.get("positions").and_then(serde_json::Value::as_array).and_then(|entries| entries.first()).expect("a projected position payload");
    assert!(first.get("lon").and_then(serde_json::Value::as_f64).is_some(), "the opaque payload still carries the renderer's lon");
    assert!(first.get("lat").and_then(serde_json::Value::as_f64).is_some(), "the opaque payload still carries the renderer's lat");
}

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use protocol::Inference;
    let text = include_str!("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    let snapshot = <crate::GisMapSnapshot as store::ArtifactDsl>::parse_dsl(text).expect("demo fixture parses");
    let inference = crate::standards::v1::subsets::any::schema::inferences::GisMapInference::infer(&snapshot);
    assert_eq!(inference, crate::standards::v1::subsets::any::schema::inferences::GisMapInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use protocol::Inference;
    assert_eq!(crate::standards::v1::subsets::any::schema::inferences::GisMapInference::infer(&crate::GisMapSnapshot::default()), crate::standards::v1::subsets::any::schema::inferences::GisMapInference::default(),);
}
//#endregion 🧪️InferenceLaws
