//! 🏔️ GIS Terrain editor window using the shared World3d viewport.

use crate::standards::v1::subsets::any::io::text::snapshot::{build_terrain_scene_json, fixture_descriptor};
use crate::GisTerrainSnapshot;
use semio_plugin::plugin_app_close_prelude::SurfaceKind as ContractSurfaceKind;
use semio_plugin::scene_surface;
use semio_plugin::world3d_selection_json;
use semio_plugin::BuiltNode;
use semio_ui_locale::LocalizedLabel;
use semio_plugin::SurfaceKind;
use semio_plugin::UiAssemblyResult;
use semio_plugin::WindowKindDefinition;
use semio_plugin::WindowOptions;
use semio_plugin::World3dScene;

#[path = "🎚️config/🦀️.rs"]
pub mod config;

//#region 🔖️Constants
pub const GIS3D_PLAY_WINDOW_MAIN: &str = "gis3d-main";
pub const GIS3D_PLAY_BODY_COMPOSITE: &str = "gis3d.play.composite";
const GIS3D_PLAY_SURFACE: &str = "gis3d.play.composite";

const MOVE_POSITION_UTILITY: &str = "terrainMovePosition";

pub const UTILITIES: [(&str, &str, &str, &str); 5] = [
    ("select", "Select", "Auswählen", "mouse-pointer"),
    ("terrainRaise", "Raise Terrain", "Gelände anheben", "arrow-up"),
    ("terrainLower", "Lower Terrain", "Gelände absenken", "arrow-down"),
    ("terrainSmooth", "Smooth Terrain", "Gelände glätten", "paintbrush"),
    (MOVE_POSITION_UTILITY, "Move Position", "Position verschieben", "move"),
];
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        initial_utility_id: Some(UTILITIES[0].0.into()),
        id: GIS3D_PLAY_WINDOW_MAIN.into(),
        label: LocalizedLabel::native("Terrain", "Gelände"),
        body_key: GIS3D_PLAY_BODY_COMPOSITE.into(),
        surface_kind: SurfaceKind::World3d,
        icon_id: "terrain-3d".into(),
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
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(document: &GisTerrainSnapshot, cfg: config::GisTerrainWindowConfig, view: &semio_plugin::ViewModel) -> UiAssemblyResult<BuiltNode> {
    let descriptor = fixture_descriptor(document);
    let Some(terrain_json) = build_terrain_scene_json(document, &descriptor) else {
        return crate::schema::terrain_descriptor::scene::terrain_source_required(view);
    };
    let settings = cfg.brush.unwrap_or_default();
    let camera = cfg.scene_camera(&descriptor, document);
    let mut scene = World3dScene::base(
        camera,
        "[]".into(),
        "[]".into(),
        world3d_selection_json("rectangle", &[], None),
    );
    scene.domain_id = Some("features".into());
    scene.domain_granularity_id = Some("pin".into());
    scene.terrain_json = Some(terrain_json);
    let utility = view.active_utility_id.as_deref().filter(|id| UTILITIES.iter().any(|entry| entry.0 == *id)).unwrap_or(UTILITIES[0].0);
    let interaction = match settings.action(utility) {
        Some(brush) => semio_framework_pack_json::json!({
            "activeUtility": utility,
            "terrainBrush": semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&brush))
        }),
        None if utility == MOVE_POSITION_UTILITY && document.elevation_source.as_ref().is_some_and(|source| source.is_valid()) =>
            semio_framework_pack_json::json!({"activeUtility": utility, "terrainMovePosition": {"action": "moveOverlayPosition"}}),
        None => semio_framework_pack_json::json!({"activeUtility": utility}),
    };
    scene.interaction_json = Some(interaction.to_string());
    scene_surface(GIS3D_PLAY_SURFACE, ContractSurfaceKind::World3d, &scene, semio_plugin::surface_name(view.locale, "Terrain view", "Geländeansicht"), None)
}
//#endregion 🔖️Render

//#region 🧪️Tests

//#endregion 🧪️Tests
