use super::*;

#[semio_framework_async_macros::async_test]
async fn build_terrain_scene_json_roundtrips_descriptor_fields() {
    let descriptor = TerrainDescriptorJson {
        schema: "gis.terrain".to_string(),
        project_origin: TerrainProjectOrigin { lon: 9.7382, lat: 52.3759 },
        positions: vec![TerrainPositionData { id: "p1".to_string(), lon: 9.74, lat: 52.38, label: Some("Site".to_string()), icon: None }],
        exaggeration: 1.5,
    };
    let json = build_terrain_scene_json(&descriptor);
    let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
    assert_eq!(value["projectOriginLon"], 9.7382);
    assert_eq!(value["exaggeration"], 1.5);
    assert_eq!(value["tileUrlTemplate"], GIS_3D_TERRAIN_TILE_URL_TEMPLATE);
}

#[semio_framework_async_macros::async_test]
async fn terrain_descriptor_json_defaults_exaggeration_and_positions_when_absent() {
    let json = r#"{"schema":"gis.terrain","projectOrigin":{"lon":1.0,"lat":2.0}}"#;
    let descriptor: TerrainDescriptorJson = dsl::json::from_json_str(json).expect("valid descriptor json");
    assert_eq!(descriptor.exaggeration, 1.0);
    assert!(descriptor.positions.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn terrain_position_data_omits_none_fields_when_serialized() {
    let position = TerrainPositionData { id: "p2".to_string(), lon: 1.0, lat: 2.0, label: None, icon: Some("pin".to_string()) };
    let json = dsl::json::to_json_string(&position);
    assert!(!json.contains("label"));
    assert!(json.contains("\"icon\":\"pin\""));
}

/// 🧭️ Relocated from the artifact's `⚙️engine` tests alongside `default_terrain_document`/
/// `empty_gis_terrain_snapshot` (`DocumentHelpers` above).
#[semio_framework_async_macros::async_test]
async fn default_terrain_document_seeds_the_fixture_exaggeration() {
    assert_eq!(default_terrain_document().exaggeration, 1.5);
    assert_eq!(empty_gis_terrain_snapshot().exaggeration, 1.0);
}
