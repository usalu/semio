//! 🏔️ GIS terrain viewer — the Terrain window: a read-only World3d render of the DEM patch, built
//! from the same `crate::schema::{parse_descriptor, build_terrain_scene_json}`
//! pure snapshot→scene helpers the editor's own Terrain window uses — this file itself imports
//! nothing from the sibling editor surface (`policyViewerPurityBreaches` forbids it outright). No
//! exaggeration control, no selection, no camera persistence: a viewer has no utilities that edit and
//! emits no mutations by construction (`ViewEmit`).

use crate::standards::v1::subsets::any::io::text::snapshot::{
    build_artifact_terrain_camera_json, build_terrain_scene_json, fixture_descriptor,
};
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

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "gis3d-view-terrain";
pub const BODY_KEY: &str = "gis3d.view.terrain";
const SURFACE_ID: &str = "gis3d.view.composite";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::gisterrain::create_gisterrain_viewer`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        initial_utility_id: None,
        id: WINDOW_KIND_ID.into(),
        label: LocalizedLabel::native("Terrain", "Gelände"),
        body_key: BODY_KEY.into(),
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
/// 👁️ Renders the same terrain and GIS overlays as the editor with an overview camera.
pub fn render(document: &GisTerrainSnapshot, view: &semio_plugin::ViewModel) -> UiAssemblyResult<BuiltNode> {
    let descriptor = fixture_descriptor(document);
    let Some(terrain_json) = build_terrain_scene_json(document, &descriptor) else {
        return crate::schema::terrain_descriptor::scene::terrain_source_required(view);
    };
    let mut scene = World3dScene::base(
        build_artifact_terrain_camera_json(document, &descriptor),
        "[]".into(),
        "[]".into(),
        world3d_selection_json("rectangle", &[], None),
    );
    scene.domain_id = Some("features".into());
    scene.domain_granularity_id = Some("pin".into());
    scene.terrain_json = Some(terrain_json);
    scene_surface(SURFACE_ID, ContractSurfaceKind::World3d, &scene, semio_plugin::surface_name(view.locale, "Terrain view", "Geländeansicht"), None)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
