//! 🏔️ GIS 3D play app — the terrain window (view mode): the World3d viewport over the DEM patch.
//!
//! ⛰️ Reuses the existing `World3d` viewport/renderer rather than a bespoke one; deliberately
//! read-mostly for this first pass — exaggeration and the `map:in` overlay layer are the only
//! editable/undoable document state (see `crate`).

/// ⚠️ Fixed opportunistically (was a pre-existing, ticket-predating unresolved `crate::modules`
/// import — see `💡️inferences/🦀️.rs`'s identical fix for the full story). Real home:
/// `crate::schema`'s `🔖️TerrainDescriptor` region.
use crate::schema::terrain_descriptor::{TerrainDescriptorJson};
use crate::standards::v1::subsets::any::io::text::snapshot::{build_terrain_scene_json};
use crate::standards::v1::subsets::any::io::text::snapshot::fixture_descriptor;
use crate::GisTerrainSnapshot;
use semio_framework_plugin::plugin_app_close_prelude::SurfaceKind as ContractSurfaceKind;
use semio_framework_plugin::scene_surface;
use semio_framework_plugin::world3d_selection_json;
use semio_framework_plugin::BuiltNode;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::SurfaceKind;
use semio_framework_plugin::UiAssemblyResult;
use semio_framework_plugin::WindowKindDefinition;
use semio_framework_plugin::WindowOptions;
use semio_framework_plugin::World3dScene;
use semio_framework_surface::terrain::projection;
use serde_json::{json, Value};

#[path = "🎚️config/🦀️.rs"]
pub mod config;

//#region 🔖️Constants
pub const GIS3D_PLAY_WINDOW_MAIN: &str = "gis3d-main";
pub const GIS3D_PLAY_BODY_COMPOSITE: &str = "gis3d.play.composite";
const GIS3D_PLAY_SURFACE: &str = "gis3d.play.composite";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        initial_utility_id: None,
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
/// 📍️ GIS pins are emitted as plain `World3d` instances with no matching `meshesJson` entry —
/// `WorldInstancesLayer`'s existing missing-mesh fallback renders a small colored box, so
/// selection/hover/context-menu all work for free without any new scene-schema surface.
fn instances_json(descriptor: &TerrainDescriptorJson) -> String {
    let instances: Vec<Value> = descriptor
        .positions
        .iter()
        .map(|position| {
            let (x, y) = projection::lonlat_to_local_meters(position.lon, position.lat, descriptor.project_origin.lon, descriptor.project_origin.lat);
            json!({
                "id": position.id,
                "meshId": "pin",
                "position": [x, y, 50.0],
                "color": "#ff3355",
                "label": position.label,
            })
        })
        .collect();
    serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into())
}

pub fn render(document: &GisTerrainSnapshot, cfg: config::GisTerrainWindowConfig) -> UiAssemblyResult<BuiltNode> {
    let descriptor = fixture_descriptor(document);
    let mut scene = World3dScene::base(
        cfg.camera_json,
        "[]".into(),
        instances_json(&descriptor),
        world3d_selection_json("rectangle", &[], None),
    );
    scene.domain_id = Some("features".into());
    scene.domain_granularity_id = Some("pin".into());
    scene.terrain_json = Some(build_terrain_scene_json(&descriptor));
    scene_surface(GIS3D_PLAY_SURFACE, ContractSurfaceKind::World3d, &scene)
}
//#endregion 🔖️Render

//#region 🧪️Tests

//#endregion 🧪️Tests
