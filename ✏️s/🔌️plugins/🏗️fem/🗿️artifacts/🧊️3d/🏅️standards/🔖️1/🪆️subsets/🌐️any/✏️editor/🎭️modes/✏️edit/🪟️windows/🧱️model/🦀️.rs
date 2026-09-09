//! 🧱️ FEM 3D app — the `edit` mode's Model window: renders the undeformed structure (nodes, bar/frame
//! members, meshed solids) as a `World3d` scene. fem3d's manifest declares this window with the scalar
//! `.window_kind(..)` builder call directly (see `crate::editor::fem3d::create_fem3d_app`) — no
//! `WindowKindDefinition`/`window_kind_def` object is built anywhere in the pre-migration
//! `create_fem3d_app`, so this node exports just its id/body-key constants and `render()`.

#[cfg(test)]
use crate::Fem3dSnapshot;
use crate::FemCamera;

/// 🪟️ The manifest's Model window kind id.
pub const FEM3D_WINDOW_MODEL: &str = "fem3d-model";
/// 📄️ The Model window's sole render body key.
pub const FEM3D_BODY_MODEL: &str = "fem3d.play.model";

/// 🧱️ Renders the undeformed structure: the same node/member/solid instances every results view
/// deforms, at deformation scale `doc.analysis.deformation_scale` with no displacement offset applied
/// (`None` displacements) and no stress coloring.
#[cfg(test)]
pub fn render(doc: &Fem3dSnapshot, camera: &FemCamera) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    use crate::editor::fem3d::{fem3d_camera_json, fem3d_scene_parts};

    let (meshes_json, instances_json) = fem3d_scene_parts(doc, None, doc.analysis.deformation_scale, None);
    crate::app_surface::world_3d_surface(
        FEM3D_BODY_MODEL,
        &semio_framework_plugin::world3d_scene(fem3d_camera_json(camera), meshes_json, instances_json, semio_framework_plugin::world3d_selection_json("rectangle", &[], None), &semio_framework_plugin::WorldSunConfig::default()),
    )
}

/// 👁️ Borrows a generation-qualified immutable renderer packet without scene rebuilding on the UI thread.
///
/// 🕳️ `meshes_json`/`instances_json` stay the literal `"[]"` by design: geometry travels on
/// `scene.snapshot`, the `live_visual` page lease. A `None` lease therefore paints an EMPTY world — the
/// `[DEBUG]` line below is the discriminator between "the reconcile job has not published yet / the
/// identity did not line up" and "the app is genuinely rendering geometry".
pub fn render_with_progress(camera: &FemCamera, visual: Option<&crate::live_visual::Fem3dPageVisualLease>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let mut scene =
        semio_framework_plugin::world3d_scene(crate::editor::fem3d::fem3d_camera_json(camera), "[]".into(), "[]".into(), semio_framework_plugin::world3d_selection_json("rectangle", &[], None), &semio_framework_plugin::WorldSunConfig::default());
    scene.snapshot = visual.map(crate::live_visual::Fem3dPageVisualLease::snapshot);
    eprintln!("[DEBUG] fem3d model window render: liveVisualLease={} sceneSnapshot={}", visual.is_some(), scene.snapshot.is_some());
    crate::app_surface::world_3d_surface(FEM3D_BODY_MODEL, &scene)
}

// #region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🧪️Tests
