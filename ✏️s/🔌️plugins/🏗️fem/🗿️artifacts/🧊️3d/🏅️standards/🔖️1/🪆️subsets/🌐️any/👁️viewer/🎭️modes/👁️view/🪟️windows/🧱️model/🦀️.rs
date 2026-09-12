//! 🧱️ FEM 3D viewer — the `view` mode's Model window: a read-only World3d render of the undeformed
//! structure (nodes, bar/frame members, meshed solids) — the exact scene the editor's own Model window
//! renders (same `fem3d_scene_parts(doc, None, deformation_scale, None)` call: no displacement offset,
//! no stress coloring), rebuilt from scratch here rather than imported from the sibling editor module,
//! which `policyViewerPurityBreaches` forbids outright. Camera is a hardcoded default
//! (`crate::viewport::INITIAL`) — a viewer has no persisted per-session camera
//! (`Config = NoConfig`). Mirrors fem3d's own editor style: the manifest declares this window with the
//! scalar `.window_kind(..)` builder call directly (see `crate::viewer::fem3d::create_fem3d_viewer`) —
//! no `WindowKindDefinition` object is built anywhere, so this node exports just its id/body-key
//! constants and `render()`.

use semio_framework_plugin::{world3d_scene, world3d_selection_json, WorldSunConfig};

//#region 🔖️Constants
/// 🪟️ The manifest's viewer Model window kind id.
pub const WINDOW_KIND_ID: &str = "fem3d-view-model";
/// 📄️ The viewer Model window's sole render body key.
pub const BODY_KEY: &str = "fem3d.view.model";
//#endregion 🔖️Constants

//#region 🔖️Render
/// 🧱️ Renders the undeformed structure with a hardcoded default camera — no persisted per-session
/// camera (`Config = NoConfig`), no displacement offset, no stress coloring: the exact same scene the
/// editor's own Model window renders for the same document.
pub fn render(visual: Option<&crate::live_visual::Fem3dPageVisualLease>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let camera = crate::viewport::INITIAL;
    let mut scene = world3d_scene(crate::viewport::scene_camera_json(&camera), "[]".into(), "[]".into(), world3d_selection_json("rectangle", &[], None), &WorldSunConfig::default());
    scene.snapshot = visual.map(crate::live_visual::Fem3dPageVisualLease::snapshot);
    eprintln!("[DEBUG] fem3d viewer model window render: liveVisualLease={} sceneSnapshot={}", visual.is_some(), scene.snapshot.is_some());
    crate::app_surface::world_3d_surface(BODY_KEY, &scene)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
