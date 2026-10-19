//! members, meshed solids, supports, loads) as a `World3d` scene bound to the framework-owned
//! 🧱️ FEM 3D app — the `edit` mode's Model window: renders the undeformed structure (nodes, bar/frame
//! `fem3d` interaction domain, so the host's own raycast picks, marquee and transform gumball act
//! on document entities. fem3d's manifest declares this window with the scalar `.window_kind(..)`
//! builder call directly (see `semio_s_artifact_fem_3d_editor::editor::fem3d::create_fem3d_app`).

#[path = "🎚️config/🦀️.rs"]
pub mod config;

use semio_s_artifact_fem_3d_editor_interaction::editor::fem3d::interaction::gumball::fem3d_selection_json;
use semio_s_artifact_fem_3d_editor_interaction::editor::fem3d::interaction::Fem3dInteractionSnapshot;
use semio_s_artifact_fem_3d_editor_interaction::editor::fem3d::interaction::FEM3D_GRANULARITY_NODE;
use semio_s_artifact_fem_3d_editor_interaction::editor::fem3d::interaction::FEM3D_INTERACTION_DOMAIN;
use semio_s_artifact_fem_3d_scene::standards::v1::subsets::any::scene::fem3d_scene_parts;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;
use self::config::Fem3dModelWindowConfig;

/// 🪟️ The manifest's Model window kind id.
pub const FEM3D_WINDOW_MODEL: &str = "fem3d-model";
/// 📄️ The Model window's sole render body key.
pub const FEM3D_BODY_MODEL: &str = "fem3d.play.model";

/// 🎬️ The bound world scene: camera, meshes, instances, the selection record (with the gumball when
/// the transform utility is armed) and the interaction domain the host dispatches picks into.
pub fn model_scene(doc: &Fem3dSnapshot, window: &Fem3dModelWindowConfig, interaction: &Fem3dInteractionSnapshot, transform_armed: bool) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::World3dScene> {
    let (meshes_json, instances_json) = fem3d_scene_parts(doc, None, doc.analysis.deformation_scale, None).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("fem3d.scene-identity", "FEM3d scene target identity refused"))?;
    let mut scene = semio_framework_plugin::world3d_scene(semio_s_artifact_fem_3d_scene::viewport::scene_camera_json(&window.camera), meshes_json, instances_json, fem3d_selection_json(doc, interaction, transform_armed, &window.gumball), &semio_framework_plugin::WorldSunConfig::default());
    scene.domain_id = Some(FEM3D_INTERACTION_DOMAIN.into());
    scene.domain_granularity_id = Some(FEM3D_GRANULARITY_NODE.into());
    Ok(scene)
}

/// 🧱️ Renders the undeformed structure with no interaction — the fixture render.
#[cfg(any(test, feature = "testkit"))]
pub fn render(doc: &Fem3dSnapshot, camera: &semio_framework_os_kernel::Viewport3dOrbit) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let window = Fem3dModelWindowConfig { camera: *camera, ..Fem3dModelWindowConfig::default() };
    semio_s_fem_app_surface::world_3d_surface(FEM3D_BODY_MODEL, &model_scene(doc, &window, &Fem3dInteractionSnapshot::default(), false)?)
}

/// 👁️ Renders the undeformed structure (the same node/member/solid instances `render` draws, no
/// displacement, no stress) and carries the generation-qualified `live_visual` page lease on
/// `scene.snapshot` for a host that pages it — a host that only draws `meshes_json`/`instances_json`
/// (the React world today) still shows the model as it is edited.
pub fn render_with_progress(doc: &Fem3dSnapshot, window: &Fem3dModelWindowConfig, interaction: &Fem3dInteractionSnapshot, transform_armed: bool, visual: Option<&crate::live_visual::Fem3dPageVisualLease>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let mut scene = model_scene(doc, window, interaction, transform_armed)?;
    scene.snapshot = visual.map(crate::live_visual::Fem3dPageVisualLease::snapshot);
    semio_s_fem_app_surface::world_3d_surface(FEM3D_BODY_MODEL, &scene)
}

// #region 🧪️Tests
// #endregion 🧪️Tests
