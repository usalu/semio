#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

/// 🌍️ Preserves the entire foreign JSON demo through typed text and independent binary projection.
#[semio_framework_async_macros::async_test]
async fn primary_asset_preserves_neutral_document_and_child_identities() {
    use store::{ArtifactDsl, ArtifactPack};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪪️document/🔣️.json")).unwrap();
    let snapshot = crate::GisMapSnapshot::parse_dsl(super::PRIMARY_TEXT).expect("typed demo parses");
    let decoded = crate::GisMapSnapshot::decode_pack(&snapshot.encode_pack()).expect("typed demo binary decodes");
    assert_eq!(decoded, snapshot);
    let descriptor: serde_json::Value = serde_json::from_str(&crate::standards::v1::subsets::any::io::text::snapshot::gis_map_descriptor_json(&decoded)).unwrap();
    for field in ["positions", "routes", "regions"] {
        let expected: Vec<serde_json::Value> = fixture["features"][field].as_array().unwrap().iter().map(|feature| feature["data"].clone()).collect();
        assert_eq!(descriptor[field], serde_json::Value::Array(expected), "{field} foreign JSON meaning");
    }
    for (field, child_id, target) in [("drawing", &decoded.drawing.child_id, &decoded.drawing.target), ("value", &decoded.value.child_id, &decoded.value.target)] {
        assert_eq!(child_id, fixture[field]["child_id"].as_str().unwrap());
        assert_eq!(target.artifact_id, fixture[field]["artifact_id"].as_str().unwrap());
        assert_eq!(target.dialect.artifact_kind, fixture[field]["artifact_kind"].as_str().unwrap());
        assert_eq!(target.dialect.standard, fixture[field]["standard"].as_str().unwrap());
        assert_eq!(target.dialect.subset, fixture[field]["subset"].as_str().unwrap());
    }
    assert!(decoded.image.is_none());
    eprintln!("[DEBUG] GIS demo typed-text/Pack/serde_json positions=152 routes=149 regions=0 durable-children=2");
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
    let descriptor: serde_json::Value = serde_json::from_str(&crate::standards::v1::subsets::any::io::text::snapshot::gis_map_descriptor_json(&document)).expect("descriptor json");
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
    let inference = crate::standards::v1::subsets::any::schema::inferences::GisMapInference::infer(&snapshot).expect("valid materialized inference fixture");
    assert_eq!(inference, crate::standards::v1::subsets::any::schema::inferences::GisMapInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use protocol::Inference;
    assert_eq!(crate::standards::v1::subsets::any::schema::inferences::GisMapInference::infer(&crate::GisMapSnapshot::default()).expect("valid materialized inference fixture"), crate::standards::v1::subsets::any::schema::inferences::GisMapInference::default(),);
}
//#endregion 🧪️InferenceLaws
