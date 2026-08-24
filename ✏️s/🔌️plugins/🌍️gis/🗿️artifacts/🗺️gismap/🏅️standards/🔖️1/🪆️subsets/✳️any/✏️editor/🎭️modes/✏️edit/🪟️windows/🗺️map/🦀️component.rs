//! 🗺️ GIS 2D play app — the map window (edit mode): the tiled-map canvas and its chrome measures.

use crate::artifacts::gismap::schema::gis_map_descriptor_json;
use crate::artifacts::gismap::GisMapSnapshot;
use crate::editor::gis2d::config::Gis2dConfig;
use crate::editor::gis2d::terminology::Gis2dPlayLabels;
use crate::editor::gis2d::{GIS2D_PLAY_APP_ID, GIS_MAP_LAYER_IDS};
use framework_surface::tiled_map::clamp_map_layer_weight;
use semio_framework_plugin::{build_tiled_map_scene, LocalizedLabel, SurfaceKind, TiledMapScene, UiNode, WindowKindDefinition, WindowMeasure, WindowOptions};
use std::collections::HashMap;

//#region 🔖️Constants
pub const GIS2D_PLAY_WINDOW_MAIN: &str = "gis2d-main";
pub const GIS2D_PLAY_BODY_COMPOSITE: &str = "gis2d.play.composite";
const GIS2D_PLAY_SURFACE: &str = "gis2d.play.composite";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: GIS2D_PLAY_WINDOW_MAIN.into(),
        label: LocalizedLabel::native("Map", "Karte"),
        body_key: GIS2D_PLAY_BODY_COMPOSITE.into(),
        surface_kind: SurfaceKind::TiledMap,
        icon_id: "globe".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}

/// 🎚️ Collects this window's chrome from its own `🎚️options/*` nodes rather than re-listing them —
/// measures are config-derived per frame by `ArtifactEditor::window_measures`, never frozen into the
/// manifest.
pub fn window_measures(cfg: &Gis2dConfig, labels: &Gis2dPlayLabels) -> Vec<WindowMeasure> {
    use crate::editor::gis2d::modes::edit::windows::map::options;
    vec![options::render_mode::measure(cfg, labels), options::vector_style::measure(cfg, labels), options::lod_mode::measure(cfg, labels), options::layers::measure(cfg, labels), options::layer_weights::measure(cfg, labels)]
}
//#endregion 🔖️Definition

//#region 🔖️Render
async fn default_layer_visibility() -> HashMap<String, bool> {
    GIS_MAP_LAYER_IDS.iter().map(|(id, _, _)| ((*id).into(), true)).collect()
}

async fn layer_visibility_json(cfg: &Gis2dConfig) -> String {
    let mut map = default_layer_visibility();
    for (id, visible) in &cfg.layer_visibility {
        map.insert(id.clone(), *visible);
    }
    serde_json::to_string(&map).unwrap_or_else(|_| "{}".into())
}

async fn layer_stroke_scale_json(cfg: &Gis2dConfig) -> String {
    let mut map: HashMap<String, f64> = GIS_MAP_LAYER_IDS.iter().map(|(id, _, _)| ((*id).into(), 1.0)).collect();
    for (id, weight) in &cfg.layer_stroke_scale {
        map.insert(id.clone(), clamp_map_layer_weight(*weight));
    }
    serde_json::to_string(&map).unwrap_or_else(|_| "{}".into())
}

/// 🌐️ Rewrites the tile templates to absolute URLs when the host publishes an asset base (the
/// `/osm` + `/vt` tile-proxy routes this plugin declares in its Cargo metadata).
async fn apply_gis_map_tile_base_url(scene: &mut TiledMapScene) {
    let Ok(base) = std::env::var("SEMIO_ASSET_BASE_URL") else {
        return;
    };
    let base = base.trim_end_matches('/');
    scene.tile_url_template = format!("{base}/osm/{{z}}/{{x}}/{{y}}.png");
    scene.vector_tile_url_template = format!("{base}/vt/{{z}}/{{x}}/{{y}}.pbf");
}

pub fn render(document: &GisMapSnapshot, cfg: &Gis2dConfig) -> UiNode {
    let mut scene = TiledMapScene::base(gis_map_descriptor_json(document), cfg.camera_json.clone());
    scene.render_mode = cfg.render_mode.clone();
    scene.vector_style = cfg.vector_style.clone();
    scene.lod_mode = cfg.lod_mode.clone();
    scene.layer_visibility_json = layer_visibility_json(cfg);
    scene.layer_stroke_scale_json = layer_stroke_scale_json(cfg);
    // 🕹️ Feature selection/hover/method/mode now live in the framework-owned "features" interaction
    // domain (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM). `ArtifactEditor::render`
    // carries no `InteractionView` (a known SDK gap — see `w3c-summary.md`'s flagged `EngineCanvas`/
    // `MapHost::sync_interaction` follow-up), so `TiledMapScene::base`'s own empty-selection defaults
    // are left as-is here rather than sourced from this deleted config state.
    apply_gis_map_tile_base_url(&mut scene);
    build_tiled_map_scene(GIS2D_PLAY_SURFACE, GIS2D_PLAY_APP_ID, scene)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::gis2d::terminology::gis2d_labels;
    use crate::editor::gis2d::testkit::{app, main_window_measures, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn renders_gis_map_scene() {
        let mut app = app();
        assert!(render_body(&mut app, GIS2D_PLAY_BODY_COMPOSITE).contains("tiled-map"));
    }

    #[semio_framework_async_macros::async_test]
    async fn render_canvas_uses_absolute_tile_urls_when_env_set() {
        unsafe { std::env::set_var("SEMIO_ASSET_BASE_URL", "http://127.0.0.1:6141") };
        let mut app = app();
        let json = render_body(&mut app, GIS2D_PLAY_BODY_COMPOSITE);
        assert!(json.contains("http://127.0.0.1:6141/osm/{z}/{x}/{y}.png"));
        assert!(json.contains("http://127.0.0.1:6141/vt/{z}/{x}/{y}.pbf"));
        unsafe { std::env::remove_var("SEMIO_ASSET_BASE_URL") };
    }

    #[semio_framework_async_macros::async_test]
    async fn the_window_collects_every_option_node_exactly_once() {
        let config = Gis2dConfig::default();
        let measures = window_measures(&config, gis2d_labels(&config));
        assert_eq!(measures.len(), 5, "3 selects + the layers and layer-weights groups");
        let mut app = app();
        assert_eq!(main_window_measures(&mut app).len(), measures.len(), "the app routes the same set under the window id");
    }

    #[semio_framework_async_macros::async_test]
    async fn the_definition_binds_the_tiled_map_surface_to_the_composite_body() {
        let definition = definition();
        assert_eq!(definition.id, GIS2D_PLAY_WINDOW_MAIN);
        assert_eq!(definition.body_key, GIS2D_PLAY_BODY_COMPOSITE);
        assert!(matches!(definition.surface_kind, SurfaceKind::TiledMap));
        assert!(definition.options.measures.is_empty(), "measures are config-derived per frame, never frozen into the manifest");
    }
}
//#endregion 🧪️Tests
