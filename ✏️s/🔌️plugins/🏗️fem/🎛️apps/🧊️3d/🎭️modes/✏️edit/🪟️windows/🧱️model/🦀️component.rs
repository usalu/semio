//! 🧱️ FEM 3D app — the `edit` mode's Model window: renders the undeformed structure (nodes, bar/frame
//! members, meshed solids) as a `World3d` scene. fem3d's manifest declares this window with the scalar
//! `.window_kind(..)` builder call directly (see `crate::apps::fem3d::create_fem3d_app`) — no
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
pub fn render(doc: &Fem3dSnapshot, camera: &FemCamera) -> semio_framework_plugin::UiNode {
    use crate::apps::fem3d::{fem3d_camera_json, fem3d_scene_parts};

    let (meshes_json, instances_json) = fem3d_scene_parts(doc, None, doc.analysis.deformation_scale, None);
    semio_framework_plugin::build_world_3d_scene(
        FEM3D_BODY_MODEL,
        crate::apps::fem3d::FEM3D_APP_ID,
        semio_framework_plugin::world3d_scene(fem3d_camera_json(camera), meshes_json, instances_json, semio_framework_plugin::world3d_default_selection_json(), &semio_framework_plugin::WorldSunConfig::default()),
    )
}

// #region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::fem3d::testkit::{fem3d_app, render as render_body};

    #[test]
    fn renders_fem3d_model_scene() {
        let mut app = fem3d_app();
        let json = render_body(&mut app, FEM3D_BODY_MODEL);
        assert!(json.contains("world-3d"));
    }

    #[test]
    fn model_scene_renders_solid_mesh_and_oriented_member_instances_3d() {
        let mut app = fem3d_app();
        crate::apps::fem3d::testkit::dispatch(&mut app, crate::apps::fem3d::Fem3dCommand::SetActiveExample(crate::apps::fem3d::commands::set_active_example::SetActiveExample { example_id: "default".into() }));
        let json = render_body(&mut app, FEM3D_BODY_MODEL);
        assert!(json.contains("solid-sol1"), "expected a solid- mesh/instance id for the example fixture's solid: {json}");
        assert!(json.contains("el-e1"), "expected a single oriented box instance per member (no -{{i}} sphere chain): {json}");
        assert!(!json.contains("\\\"sphere\\\""), "sphere markers should be gone: {json}");
        let _ = app.snapshot().expect("snapshot");
    }
}
// #endregion 🧪️Tests
