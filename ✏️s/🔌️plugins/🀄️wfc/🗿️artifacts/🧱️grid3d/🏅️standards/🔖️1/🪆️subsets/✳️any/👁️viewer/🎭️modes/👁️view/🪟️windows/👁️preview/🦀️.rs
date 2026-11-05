//! 👁️ Read-only grid preview projects current authored pins through the shared scene functions.
//! 🧭️ Completed and live solve results belong to the editor's revision-bound resumable job.

use std::collections::{BTreeMap,BTreeSet};
use crate::schema::scene_internals;
use crate::Grid3dSnapshot;
use semio_plugin::plugin_app_close_prelude::SurfaceKind;
use semio_plugin::world3d_camera_json;
use semio_plugin::world3d_scene;
use semio_plugin::world3d_selection_json;
use semio_plugin::BuiltNode;
use semio_ui_locale::LocalizedLabel;
use semio_plugin::UiAssemblyResult;
use semio_plugin::WindowKindDefinition;
use semio_plugin::WindowOptions;
use semio_plugin::WorldSunConfig;

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "wfc-grid3d-view";
pub const BODY_KEY: &str = "wfc.grid3d.view";
pub const SURFACE_ID: &str = "wfc.grid3d.view";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        initial_utility_id: None,
        id: WINDOW_KIND_ID.into(),
        label: LocalizedLabel::native("Preview", "Vorschau"),
        body_key: BODY_KEY.into(),
        surface_kind: semio_plugin::SurfaceKind::World3d,
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

fn authored_assignments(document:&Grid3dSnapshot)->BTreeMap<(u32,u32,u32),String> {
    let tiles=document.tiles.iter().map(|tile|tile.id.as_str()).collect::<BTreeSet<_>>();
    let masked=document.masked.iter().map(|cell|(cell.x,cell.y,cell.z)).collect::<BTreeSet<_>>();
    document.pinned.iter().filter(|pin|pin.x<document.width&&pin.y<document.height&&pin.z<document.depth&&tiles.contains(pin.tile_id.as_str())&&!masked.contains(&(pin.x,pin.y,pin.z))).map(|pin|((pin.x,pin.y,pin.z),pin.tile_id.clone())).collect()
}

pub fn render(document: &Grid3dSnapshot) -> UiAssemblyResult<BuiltNode> {
    let assignments = authored_assignments(document);
    let (position, target) = framed_camera(document);
    let scene = world3d_scene(
        world3d_camera_json(position, target, 45.0),
        crate::standards::v1::subsets::any::io::text::inferences::scene_projection::preview_meshes_json(document),
        crate::standards::v1::subsets::any::io::text::inferences::scene_projection::preview_instances_json(document, &assignments),
        world3d_selection_json("interactionSelect", &[], None),
        &WorldSunConfig::default(),
    );
    semio_plugin::scene_surface(SURFACE_ID, SurfaceKind::World3d, &scene)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
