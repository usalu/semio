use super::*;
use crate::{document_dsl, MapFeature};
use serde_json::json;

#[semio_framework_async_macros::async_test]
async fn gis_map_document_pack_agrees_with_dsl_for_bundled_reuse_example() {
    let document = document_dsl::parse_dsl(document_dsl::REUSE_MAP_EXAMPLE_TEXT).expect("parse reuse-map example");
    store::os_store::test_support::assert_dsl_pack_equivalence(&document);
    assert_eq!(decode(&encode(&document)).expect("decode"), document);
}

#[semio_framework_async_macros::async_test]
async fn gis_map_document_pack_agrees_with_dsl_for_empty_document() {
    store::os_store::test_support::assert_dsl_pack_equivalence(&GisMapSnapshot::default());
}

/// 🧬️ `MapFeature::data` is `dsl::DslValue` (deliberately untyped — see `crate`'s
/// doc comment) — this bridges a `serde_json::json!` literal into one for test-fixture ergonomics.
#[semio_framework_async_macros::async_test]
async fn gis_map_document_pack_agrees_with_dsl_for_synthetic_value_shapes() {
    let dsl_of = |value: serde_json::Value| ::dsl::DslValue::from(value);
    let document = GisMapSnapshot {
        positions: vec![MapFeature {
            id: "p1".into(),
            data: dsl_of(json!({
                "id": "p1",
                "lon": -0.1427,
                "lat": 51.5142,
                "flag": true,
                "missing": null,
                "tags": ["a", "b"],
                "meta": { "nested": { "depth": 2.0 } },
            })),
        }],
        routes: vec![MapFeature { id: "r1".into(), data: dsl_of(json!({ "id": "r1", "points": [[1.0, 2.0], [3.0, 4.0]] })) }],
        regions: vec![MapFeature { id: "g1".into(), data: dsl_of(json!({ "id": "g1", "ring": [[0.0, 0.0], [1.0, 1.0], [1.0, 0.0]] })) }],
        ..Default::default()
    };
    store::os_store::test_support::assert_dsl_pack_equivalence(&document);
}
