//! 🗺️ GIS 2D play app — the shared `MapHost` projection.
//!
//! 🧭️ App level (not the `gismap` artifact engine) on purpose: it needs BOTH the document and the
//! app-only view state (`Gis2dConfig`), and an artifact must never depend on an app. Every
//! `🎮️commands/*` node that has to hit-test, frame or query the live map goes through here.

use crate::apps::gis2d::config::Gis2dConfig;
use crate::artifacts::gismap::engine::gis_map_descriptor_json;
use crate::artifacts::gismap::GisMapDocument;
use framework_surface::tiled_map::MapHost;
use serde_json::Value;

//#region 🔖️MapHost
/// 🗺️ Builds a `MapHost` from the document content (derived descriptor JSON) plus the config's
/// camera/render/style/LOD/selection view state.
pub fn map_host_from(document: &GisMapDocument, cfg: &Gis2dConfig) -> MapHost {
    let mut host = MapHost::new();
    let descriptor = gis_map_descriptor_json(document);
    let _ = host.sync_map_json(&descriptor);
    if let Ok(camera) = serde_json::from_str::<Value>(&cfg.camera_json) {
        let x = camera.get("x").and_then(|value| value.as_f64()).unwrap_or(0.0);
        let y = camera.get("y").and_then(|value| value.as_f64()).unwrap_or(0.0);
        let zoom = camera.get("zoom").and_then(|value| value.as_f64()).unwrap_or(1.0);
        host.set_camera(x, y, zoom);
    }
    host.set_render_mode(&cfg.render_mode);
    host.set_vector_style(&cfg.vector_style);
    host.set_lod_mode(&cfg.lod_mode);
    let _ = host.set_selection_json(&cfg.feature_selection_json);
    host
}
//#endregion 🔖️MapHost

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::gismap::engine::default_document;

    #[test]
    fn the_host_mirrors_the_document_features_and_the_config_camera() {
        let document = default_document();
        let config = Gis2dConfig { camera_json: r#"{"x":10,"y":20,"zoom":4}"#.into(), ..Gis2dConfig::default() };
        let host = map_host_from(&document, &config);
        assert!(!host.positions.is_empty(), "the reuse-map fixture seeds position features");
        let camera: Value = serde_json::from_str(&host.camera_json()).expect("camera json");
        assert_eq!(camera.get("zoom").and_then(Value::as_f64), Some(4.0));
    }

    #[test]
    fn a_malformed_camera_json_leaves_the_host_at_its_own_default() {
        let config = Gis2dConfig { camera_json: "not json".into(), ..Gis2dConfig::default() };
        let host = map_host_from(&GisMapDocument::default(), &config);
        assert!(serde_json::from_str::<Value>(&host.camera_json()).is_ok(), "the host still reports a valid camera");
    }
}
//#endregion 🧪️Tests
