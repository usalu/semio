//! 🧱️ FEM 3D app — the `edit` mode's Model window: renders the undeformed structure (nodes, bar/frame
//! members, meshed solids) as a `World3d` scene. fem3d's manifest declares this window with the scalar
//! `.window_kind(..)` builder call directly (see `crate::editor::fem3d::create_fem3d_app`) — no
//! `WindowKindDefinition`/`window_kind_def` object is built anywhere in the pre-migration
//! `create_fem3d_app`, so this node exports just its id/body-key constants and `render()`.

use crate::artifacts::fem3d::{Fem3dSnapshot, FemCamera};

/// 🪟️ The manifest's Model window kind id.
pub const FEM3D_WINDOW_MODEL: &str = "fem3d-model";
/// 📄️ The Model window's sole render body key.
pub const FEM3D_BODY_MODEL: &str = "fem3d.play.model";

/// 🧱️ Renders the undeformed structure: the same node/member/solid instances every results view
/// deforms, at deformation scale `doc.analysis.deformation_scale` with no displacement offset applied
/// (`None` displacements) and no stress coloring.
pub fn render(doc: &Fem3dSnapshot, camera: &FemCamera) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    use crate::editor::fem3d::{fem3d_camera_json, fem3d_scene_parts};

    let (meshes_json, instances_json) = fem3d_scene_parts(doc, None, doc.analysis.deformation_scale, None);
    crate::app_surface::world_3d_surface(
        FEM3D_BODY_MODEL,
        semio_framework_plugin::world3d_scene(fem3d_camera_json(camera), meshes_json, instances_json, semio_framework_plugin::world3d_selection_json("rectangle", &[], None), &semio_framework_plugin::WorldSunConfig::default()),
    )
}

// #region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::fem3d::testkit::{fem3d_app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn renders_fem3d_model_scene() {
        let mut app = fem3d_app();
        let json = render_body(&mut app, FEM3D_BODY_MODEL);
        assert!(json.contains("world-3d"));
    }

    #[semio_framework_async_macros::async_test]
    async fn model_scene_renders_solid_mesh_and_oriented_member_instances_3d() {
        let mut app = fem3d_app();
        crate::editor::fem3d::testkit::dispatch(&mut app, crate::editor::fem3d::Fem3dCommand::SetActiveExample(crate::editor::fem3d::commands::set_active_example::SetActiveExample { example_id: "default".into() })).await;
        let snapshot = semio_framework_plugin::resolve_ready(app.snapshot()).expect("snapshot");
        let node = render(&snapshot, &FemCamera::default());
        let semio_framework_ui_contract::Component::Surface(props) = &node.component else { panic!("expected world surface") };
        let scene: semio_framework_ui_scene::World3dScene = semio_framework_ui_scene::decode(props).expect("decode world scene");
        assert!(scene.meshes_json.contains("solid-sol1"), "expected a solid mesh for the example fixture: {}", scene.meshes_json);
        assert!(scene.instances_json.contains("el-e1"), "expected a single oriented box instance per member: {}", scene.instances_json);
        assert!(!scene.instances_json.contains("\"sphere\""), "sphere markers should be gone: {}", scene.instances_json);
    }
}
// #endregion 🧪️Tests
