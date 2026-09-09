use super::*;
use protocol::Inference;

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = GisTerrainSnapshot { exaggeration: 1.5, imported_features_json: serde_json::json!({ "positions": [{ "id": "p1", "lon": 5.58, "lat": 50.60 }] }).to_string(), ..Default::default() };
    assert_eq!(GisTerrainInference::infer(&snapshot), GisTerrainInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(GisTerrainInference::infer(&GisTerrainSnapshot::default()), GisTerrainInference::default());
}
//#endregion 🧪️InferenceLaws

//#region 🧪️FixtureText
/// 🧭️ Relocated from the artifact's `⚙️engine` tests alongside `parse_descriptor`
/// (`🔖️FixtureText` above).
///
/// 📜️ The `.gisterrain` fixture's `gisterrain exaggeration=...` header is parsed twice for two
/// different purposes (see `parse_descriptor`/`default_terrain_document`'s docs); this proves the
/// scenery-data reader (`terrain_fixture_text`) still recovers the bundled fixture's pins/origin
/// after the document-only conversion — i.e. converting the fixture to the DSL didn't lose data.
#[semio_framework_async_macros::async_test]
async fn terrain_fixture_text_recovers_bundled_scenery_data() {
    let descriptor = parse_descriptor(&GisTerrainSnapshot { exaggeration: 1.5, imported_features_json: String::new(), ..Default::default() });
    assert_eq!(descriptor.project_origin.lon, 5.5818);
    assert_eq!(descriptor.project_origin.lat, 50.603);
    assert_eq!(descriptor.positions.len(), 2);
    assert_eq!(descriptor.positions[0].id, "p_institut_de_botanique_ulg_liege");
}

/// 🔌️ `map:in`'s overlay layer renders as extra pins alongside the fixture's own two.
#[semio_framework_async_macros::async_test]
async fn imported_map_features_render_as_extra_pins() {
    let document = GisTerrainSnapshot { exaggeration: 1.5, imported_features_json: serde_json::json!({ "positions": [{ "id": "imported-1", "lon": 5.58, "lat": 50.60 }] }).to_string(), ..Default::default() };
    let descriptor = parse_descriptor(&document);
    assert_eq!(descriptor.positions.len(), 3, "2 fixture pins + 1 imported pin");
    assert!(descriptor.positions.iter().any(|position| position.id == "imported-1"));
}
//#endregion 🧪️FixtureText
