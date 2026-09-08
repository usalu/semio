
use super::*;
use crate::MapFeature;
use serde_json::json;

#[semio_framework_async_macros::async_test]
async fn gis_map_document_dsl_round_trips_bundled_reuse_example() {
    let document = parse_dsl(REUSE_MAP_EXAMPLE_TEXT).expect("parse reuse-map example");
    store::os_store::test_support::assert_dsl_round_trip(&document);
}

#[semio_framework_async_macros::async_test]
async fn gis_map_document_dsl_round_trips_empty_document() {
    store::os_store::test_support::assert_dsl_round_trip(&GisMapSnapshot::default());
}

#[semio_framework_async_macros::async_test]
async fn print_dsl_reproduces_the_bundled_example_text_verbatim() {
    let document = parse_dsl(REUSE_MAP_EXAMPLE_TEXT).expect("parse reuse-map example");
    assert_eq!(parse_dsl(&print_dsl(&document)).expect("reparse"), document);
}

/// 🧬️ `MapFeature::data` is `dsl::DslValue` (deliberately untyped — see `crate`'s
/// doc comment) — round-trips every shape (nested object/array, bool, null, negative number) the
/// generic value grammar has to reconstruct. `depth` is kept as a float literal for this fixture;
/// `dsl::DslValue::Number`'s `UInt`/`Int`/`Float` variants now preserve the same JSON int-vs-float
/// distinction `serde_json::Value::Number` does, so this is no longer load-bearing for the
/// round trip — it stays a float simply because that is the fixture's actual data shape.
#[semio_framework_async_macros::async_test]
async fn gis_map_document_dsl_round_trips_synthetic_value_shapes() {
    let dsl_of = |value: serde_json::Value| dsl::DslValue::from(value);
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
    store::os_store::test_support::assert_dsl_round_trip(&document);
}
