#[allow(unused_imports)]
use crate::{gis_terrain_mesh_child_handle, gis_terrain_mesh_content_key, GisTerrainSnapshot};
#[allow(unused_imports)]
use ::semio_framework_schema::ArtifactSchema;
#[allow(unused_imports)]
use semio_framework_surface::terrain::tiles;
#[allow(unused_imports)]
use semio_framework_value_derive::{FromValue, ToValue};
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_mesh::schema::snapshot::SemioMeshSnapshot;
use crate::standards::v1::subsets::any::io::text::snapshot::{build_terrain_camera_json, build_terrain_scene_json, fixture_descriptor, parse_descriptor};
use crate::standards::v1::subsets::any::io::text::snapshot::default_terrain_document;
use crate::standards::v1::subsets::any::io::text::snapshot::empty_gis_terrain_snapshot;
use crate::standards::v1::subsets::any::schema::*;

#[semio_framework_async_macros::async_test]
async fn build_terrain_scene_json_roundtrips_descriptor_fields() {
    let document = default_terrain_document();
    let descriptor = fixture_descriptor(&document);
    let json = build_terrain_scene_json(&document, &descriptor).expect("bundled elevation");
    let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
    assert_eq!(value["projectOriginLon"], descriptor.project_origin.lon);
    assert_eq!(value["exaggeration"], document.exaggeration);
    assert!(value.get("tileUrlTemplate").is_none());
    assert_eq!(value["elevationGrid"]["rows"].as_array().expect("rows").len(), 33);
}

#[semio_framework_async_macros::async_test]
async fn terrain_descriptor_json_defaults_exaggeration_and_overlays_when_absent() {
    let json = r#"{"schema":"gis.terrain","projectOrigin":{"lon":1.0,"lat":2.0}}"#;
    let descriptor: TerrainDescriptorJson = semio_framework_pack_json::from_json_str(json, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("valid descriptor json");
    assert_eq!(descriptor.exaggeration, 1.0);
    assert!(descriptor.overlays.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn terrain_position_data_omits_none_fields_when_serialized() {
    let position = TerrainPositionData { id: "p2".into(), name: "Site".into(), coordinates: [1.0, 2.0], elevation: None };
    let json = semio_framework_pack_json::to_json_string(&position);
    assert!(!json.contains("elevation"));
    assert!(json.contains("\"name\":\"Site\""));
}

#[test]
fn map_overlay_projection_matches_the_wire_fixture() {
    use crate::schema::imported_map::ImportedMap;
    use serde_json::{Value, json};

    let fixture: Value = serde_json::from_str(include_str!("🔣️overlays.json")).expect("overlay fixture");
    let station = &fixture["imported"]["positions"][0];
    let summit = &fixture["imported"]["positions"][1];
    let walk = &fixture["imported"]["routes"][0];
    let reserve = &fixture["imported"]["regions"][0];
    let third_party_wire = json!({
        "positions": [
            { "id": station["id"], "name": station["label"], "coordinates": [station["lon"], station["lat"]], "elevation": station["alt"] },
            { "id": summit["id"], "name": summit["name"], "coordinates": [summit["lon"], summit["lat"]] }
        ],
        "routes": [
            { "id": walk["id"], "name": walk["id"], "coordinates": [[walk["points"][0][0], walk["points"][0][1]], walk["points"][1]] }
        ],
        "regions": [
            { "id": reserve["id"], "name": reserve["label"], "rings": [reserve["ring"].as_array().unwrap()[..4], reserve["holes"][0]] }
        ]
    });
    assert_eq!(third_party_wire, fixture["overlays"]);
    let scenery = ImportedMap::from_json(&fixture["scenery"].to_string()).expect("scenery map");
    let imported = ImportedMap::from_json(&fixture["imported"].to_string()).expect("imported map");
    let snapshot = GisTerrainSnapshot { exaggeration: 1.5, imported_map: Some(imported), ..Default::default() };
    let descriptor = parse_descriptor(&snapshot, &scenery);
    let bundled = default_terrain_document();
    let grid = crate::schema::imported_map::gis_terrain_elevation_grid(&bundled).expect("bundled elevation");
    let scene: Value = serde_json::from_str(&crate::schema::terrain_descriptor::scene::terrain_scene_json(&descriptor, &grid, true)).expect("terrain scene JSON");
    assert_eq!(scene["overlays"], fixture["overlays"]);
    assert_eq!(scene["attribution"]["label"], "Mapzen Terrain Tiles");
    let camera: Value = serde_json::from_str(&build_terrain_camera_json(&descriptor)).expect("terrain camera JSON");
    for (index, field) in ["projectOriginLon", "projectOriginLat"].into_iter().enumerate() {
        let expected = fixture["origin"][index].as_f64().unwrap();
        assert!((scene[field].as_f64().unwrap() - expected).abs() < 1e-9);
        assert!((camera["terrainFrame"][index].as_f64().unwrap() - expected).abs() < 1e-9);
    }
    assert!(camera["position"][2].as_f64().unwrap() > 2_500.0);
}

/// 🧭️ Relocated from the artifact's `⚙️engine` tests alongside `default_terrain_document`/
/// `empty_gis_terrain_snapshot` (`DocumentHelpers` above).
#[semio_framework_async_macros::async_test]
async fn default_terrain_document_seeds_the_fixture_exaggeration() {
    assert_eq!(default_terrain_document().exaggeration, 1.5);
    assert_eq!(empty_gis_terrain_snapshot().exaggeration, 1.0);
}
