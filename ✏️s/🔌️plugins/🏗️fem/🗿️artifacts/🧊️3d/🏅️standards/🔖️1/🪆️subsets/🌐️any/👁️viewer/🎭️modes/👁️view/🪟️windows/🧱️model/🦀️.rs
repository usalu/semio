//! 🧱️ FEM 3D viewer — the `view` mode's Model window: a read-only World3d render of the undeformed
//! structure (nodes, bar/frame members, meshed solids) — the exact scene the editor's own Model window
//! renders (same `fem3d_scene_parts(doc, None, deformation_scale, None)` call: no displacement offset,
//! no stress coloring), rebuilt from scratch here rather than imported from the sibling editor module,
//! which `policyViewerPurityBreaches` forbids outright. Camera is a hardcoded default
//! (`crate::artifacts::fem3d::FemCamera::default()`) — a viewer has no persisted per-session camera
//! (`Config = NoConfig`). Mirrors fem3d's own editor style: the manifest declares this window with the
//! scalar `.window_kind(..)` builder call directly (see `crate::viewer::fem3d::create_fem3d_viewer`) —
//! no `WindowKindDefinition` object is built anywhere, so this node exports just its id/body-key
//! constants and `render()`.

use crate::artifacts::fem3d::FemCamera;
use semio_framework_plugin::{world3d_scene, world3d_selection_json, WorldSunConfig};

//#region 🔖️Constants
/// 🪟️ The manifest's viewer Model window kind id.
pub const WINDOW_KIND_ID: &str = "fem3d-view-model";
/// 📄️ The viewer Model window's sole render body key.
pub const BODY_KEY: &str = "fem3d.view.model";
/// 👁️ Read-only counterpart of the editor's `FEM3D_APP_ID` controller id — kept distinct so a viewer
/// session's world-3d controller can never be mistaken for an editor session's.
const FEM3D_VIEW_CONTROLLER_ID: &str = "fem3d-view";
//#endregion 🔖️Constants

//#region 🔖️PreparedScene
fn fem3d_camera_json(camera: &FemCamera) -> String {
    if camera.json == "{}" {
        semio_framework_plugin::world3d_default_camera()
    } else {
        camera.json.clone()
    }
}
//#endregion 🔖️PreparedScene

//#region 🔖️Render
/// 🧱️ Renders the undeformed structure with a hardcoded default camera — no persisted per-session
/// camera (`Config = NoConfig`), no displacement offset, no stress coloring: the exact same scene the
/// editor's own Model window renders for the same document.
pub fn render(visual: Option<&crate::artifacts::fem3d::live_visual::Fem3dPageVisualLease>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let camera = FemCamera::default();
    let mut scene = world3d_scene(fem3d_camera_json(&camera), "[]".into(), "[]".into(), world3d_selection_json("rectangle", &[], None), &WorldSunConfig::default());
    scene.snapshot = visual.map(crate::artifacts::fem3d::live_visual::Fem3dPageVisualLease::snapshot);
    crate::app_surface::world_3d_surface(BODY_KEY, scene)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_a_prepared_scene_node_without_a_whole_scene_bypass() {
        let node = render(None);
        let json = dsl::json::to_json_string(&node);
        assert!(json.contains("world-3d"));
        let semio_framework_ui_contract::Component::Surface(props) = &node.component else { panic!("expected world surface") };
        let scene: semio_framework_ui_scene::World3dScene = semio_framework_ui_scene::decode(props).expect("decode world scene");
        assert_eq!(scene.meshes_json, "[]");
        assert_eq!(scene.instances_json, "[]");
    }
}
//#endregion 🧪️Tests
