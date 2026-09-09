use super::*;
use crate::MapFeature;

fn dsl_of(value: serde_json::Value) -> dsl::DslValue {
    dsl::DslValue::from(value)
}

#[semio_framework_async_macros::async_test]
async fn empty_snapshot_has_no_bounds() {
    assert!(lon_lat_bounds(&all_lon_lat_pairs(&GisMapSnapshot::default())).is_none());
}

#[semio_framework_async_macros::async_test]
async fn positions_routes_and_regions_all_contribute_points() {
    let snapshot = GisMapSnapshot {
        positions: vec![MapFeature { id: "p1".into(), data: dsl_of(serde_json::json!({ "id": "p1", "lon": -0.1427, "lat": 51.5142 })) }],
        routes: vec![MapFeature { id: "r1".into(), data: dsl_of(serde_json::json!({ "id": "r1", "points": [[1.0, 2.0], [3.0, 4.0]] })) }],
        regions: vec![MapFeature { id: "g1".into(), data: dsl_of(serde_json::json!({ "id": "g1", "ring": [[0.0, 0.0], [1.0, 1.0], [1.0, 0.0]] })) }],
        ..Default::default()
    };
    let bounds = lon_lat_bounds(&all_lon_lat_pairs(&snapshot)).expect("features bound");
    assert_eq!(bounds, GisMapBounds { lon_min: -0.1427, lon_max: 3.0, lat_min: 0.0, lat_max: 51.5142 });
}
