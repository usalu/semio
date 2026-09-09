use super::*;

#[semio_framework_async_macros::async_test]
async fn empty_overlay_has_no_bounds() {
    let snapshot = GisTerrainSnapshot::default();
    assert!(lon_lat_bounds(&imported_lon_lat_positions(&snapshot)).is_none());
}

#[semio_framework_async_macros::async_test]
async fn malformed_overlay_json_contributes_no_positions() {
    let snapshot = GisTerrainSnapshot { exaggeration: 0.0, imported_features_json: "not json".into(), ..Default::default() };
    assert!(imported_lon_lat_positions(&snapshot).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn two_positions_produce_their_enclosing_box() {
    let snapshot = GisTerrainSnapshot {
        exaggeration: 1.0,
        imported_features_json: serde_json::json!({ "positions": [
                { "id": "a", "lon": 5.0, "lat": 50.0 },
                { "id": "b", "lon": 6.0, "lat": 51.0 },
            ] })
        .to_string(),
        ..Default::default()
    };
    let bounds = lon_lat_bounds(&imported_lon_lat_positions(&snapshot)).expect("two positions bound");
    assert_eq!(bounds, GisTerrainBounds { lon_min: 5.0, lon_max: 6.0, lat_min: 50.0, lat_max: 51.0 });
}
