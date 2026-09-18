//! 👁️ `s.wfc.grid3d` viewer — the read-only preview window: the same instanced `World3d` projection
//! the editor's preview publishes, over the same pure `scene_internals` functions, so the two
//! surfaces can never drift. The viewer imports nothing from the sibling editor
//! (`policyViewerPurityBreaches`).
//!
//! ⏱️ A viewer has no app transient to cache a solve in, so it runs the inference INLINE on render.
//! That is bounded by the same admission ceilings the cold route uses and is honest for a read-only
//! surface; a document too large to solve inline paints the unsolved cage instead of failing.

use crate::schema::inferences::solve;
use crate::schema::scene_internals;
use crate::Grid3dSnapshot;
use semio_framework_plugin::plugin_app_close_prelude::SurfaceKind;
use semio_framework_plugin::{world3d_camera_json, world3d_scene, world3d_selection_json, BuiltNode, LocalizedLabel, UiAssemblyResult, WindowKindDefinition, WindowOptions, WorldSunConfig};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "wfc-grid3d-view";
pub const BODY_KEY: &str = "wfc.grid3d.view";
pub const SURFACE_ID: &str = "wfc.grid3d.view";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: WINDOW_KIND_ID.into(),
        label: LocalizedLabel::native("Preview", "Vorschau"),
        body_key: BODY_KEY.into(),
        surface_kind: semio_framework_plugin::SurfaceKind::World3d,
        icon_id: "preview".into(),
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
/// 📷️ The pose a viewer opens on — framed on what this document actually spans, since a viewer has
/// no per-window config to restore a user pose from.
pub fn framed_camera(document: &Grid3dSnapshot) -> ([f64; 3], [f64; 3]) {
    let extent = scene_internals::grid_extent(document);
    let target = [extent[0] * 0.5, extent[1] * 0.5, extent[2] * 0.5];
    let span = extent[0].max(extent[1]).max(extent[2]).max(1.0);
    let distance = (span * 1.9).max(2.5);
    ([target[0] + distance, target[1] - distance, target[2] + distance * 0.75], target)
}

pub fn render(document: &Grid3dSnapshot) -> UiAssemblyResult<BuiltNode> {
    let assignments = solve(document).unwrap_or_default().assignments;
    let (position, target) = framed_camera(document);
    let scene = world3d_scene(
        world3d_camera_json(position, target, 45.0),
        scene_internals::preview_meshes_json(document),
        scene_internals::preview_instances_json(document, &assignments),
        world3d_selection_json("interactionSelect", &[], None),
        &WorldSunConfig::default(),
    );
    semio_framework_plugin::scene_surface(SURFACE_ID, SurfaceKind::World3d, &scene)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
