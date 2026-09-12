//! 🗺️ GIS 2D play app — the map window (edit mode): the tiled-map canvas and its chrome measures.

use crate::editor::gis2d::modes::edit::windows::map::config::MapWindowConfig;
use crate::editor::gis2d::terminology::Gis2dPlayLabels;
use crate::editor::gis2d::GIS_MAP_LAYER_IDS;
use crate::schema::gis_map_descriptor_json;
use crate::GisMapSnapshot;
use semio_framework_plugin::plugin_app_close_prelude::SurfaceKind as ContractSurfaceKind;
use semio_framework_plugin::{scene_surface, BuiltNode, LocalizedLabel, SurfaceKind, TiledMapScene, UiAssemblyResult, WindowKindDefinition, WindowMeasure, WindowOptions};
use semio_framework_surface::tiled_map::clamp_map_layer_weight;
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
pub fn window_measures(cfg: &MapWindowConfig, labels: &Gis2dPlayLabels) -> Vec<WindowMeasure> {
    use crate::editor::gis2d::modes::edit::windows::map::options;
    vec![options::render_mode::measure(cfg, labels), options::vector_style::measure(cfg, labels), options::lod_mode::measure(cfg, labels), options::layers::measure(cfg, labels), options::layer_weights::measure(cfg, labels)]
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn default_layer_visibility() -> HashMap<String, bool> {
    GIS_MAP_LAYER_IDS.iter().map(|(id, _, _)| ((*id).into(), true)).collect()
}

fn layer_visibility_json(cfg: &MapWindowConfig) -> String {
    let mut map = default_layer_visibility();
    for (id, visible) in &cfg.layer_visibility {
        map.insert(id.clone(), *visible);
    }
    serde_json::to_string(&map).unwrap_or_else(|_| "{}".into())
}

fn layer_stroke_scale_json(cfg: &MapWindowConfig) -> String {
    let mut map: HashMap<String, f64> = GIS_MAP_LAYER_IDS.iter().map(|(id, _, _)| ((*id).into(), 1.0)).collect();
    for (id, weight) in &cfg.layer_stroke_scale {
        map.insert(id.clone(), clamp_map_layer_weight(*weight));
    }
    serde_json::to_string(&map).unwrap_or_else(|_| "{}".into())
}

/// 🌐️ Rewrites the tile templates to absolute URLs when the host publishes an asset base (the
/// `/osm` + `/vt` tile-proxy routes this plugin declares in its Cargo metadata).
fn apply_gis_map_tile_base_url(scene: &mut TiledMapScene) {
    let Ok(base) = std::env::var("SEMIO_ASSET_BASE_URL") else {
        return;
    };
    let base = base.trim_end_matches('/');
    scene.tile_url_template = format!("{base}/osm/{{z}}/{{x}}/{{y}}.png");
    scene.vector_tile_url_template = format!("{base}/vt/{{z}}/{{x}}/{{y}}.pbf");
}

pub fn render(document: &GisMapSnapshot, cfg: &MapWindowConfig) -> UiAssemblyResult<BuiltNode> {
    let mut scene = TiledMapScene::base(gis_map_descriptor_json(document), cfg.camera_json.clone());
    scene.render_mode = cfg.render_mode.clone();
    scene.vector_style = cfg.vector_style.clone();
    scene.lod_mode = cfg.lod_mode.clone();
    scene.layer_visibility_json = layer_visibility_json(cfg);
    scene.layer_stroke_scale_json = layer_stroke_scale_json(cfg);
    // 🕹️ Feature selection/hover/method/mode now live in the framework-owned "features" interaction
    // domain (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM). `ArtifactEditor::render`
    // carries no `InteractionView` (a known SDK gap — see `w3c-summary.md`'s flagged `⚙️EngineCanvas`/
    // `MapHost::sync_interaction` follow-up), so `TiledMapScene::base`'s own empty-selection defaults
    // are left as-is here rather than sourced from this deleted config state.
    apply_gis_map_tile_base_url(&mut scene);
    scene_surface(GIS2D_PLAY_SURFACE, ContractSurfaceKind::TiledMap, &scene)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
