//! 📐️ CAD viewer — the Shape window: a read-only world-3d render of the `spatial.shape` pane, built
//! from the same subset `🧬️schema/💡️inferences` pure snapshot→view-model helpers the editor's own
//! Shape window (`✏️editor/🎭️modes/✏️edit/🪟️windows/📐️shape`) uses — this file itself imports
//! nothing from the sibling editor surface (`policyViewerPurityBreaches` forbids it outright). No
//! selection, no engagement, no gumball/dislocate: a viewer has no utilities that edit and emits no
//! mutations by construction (`ViewEmit`).

use crate::standards::v1::subsets::any::schema::inferences::cad_camera_projection_config;
use crate::{CadCamera, CadPaneId, CadSnapshot};
use semio_framework_plugin::app::WindowKit;
use semio_framework_plugin::{mesh_from_kind, world3d_camera_projection_json, world3d_selection_json, BuiltNode, LocalizedLabel, MeshView, MeshWindowKit, UiAssemblyResult, WindowKindDefinition, WindowOptions};
use ui_wgpu::wgpu::SurfaceKind;

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "cad-view-shape";
pub const BODY_KEY: &str = "cad.view.shape";
pub const SURFACE_ID: &str = "cad.view.scene3d/shape";
pub const PANE: CadPaneId = CadPaneId::Shape;
/// 👁️ Matches the editor's `CAD_FALLBACK_MESH_KIND` literal ("box") — duplicated on purpose rather
/// than imported through the sibling `✏️editor` module, which `policyViewerPurityBreaches` forbids outright.
const CAD_VIEW_FALLBACK_MESH_KIND: &str = "box";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::cad::create_cad_viewer`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: WINDOW_KIND_ID.into(),
        label: LocalizedLabel::native("Shape", "Form"),
        body_key: BODY_KEY.into(),
        surface_kind: SurfaceKind::World3d,
        icon_id: "cad-shape".into(),
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
/// 👁️ Pure `CadSnapshot -> UiNode` read: default camera/sun (a viewer has no persisted per-session
/// camera — `Config = NoConfig`), no selection/gumball/engagement overlay, real references read
/// straight off the document. Objects render the same fallback-box placeholder the editor's own
/// `world_meshes_json` falls back to while composed-child object resolution is unimplemented (ticket
/// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 3 gap, pre-existing, not introduced here).
/// 🌉️ `MeshData` (`semio_framework_plugin`) carries its own first-party `From<MeshData> for
/// pack::json::Value` — reached here through the `dsl`/`protocol` alias's `os_pack` re-export of the
/// same `pack` crate, never through `serde_json`.
fn mesh_data_to_dsl(data: &semio_framework_plugin::MeshData) -> protocol::DslValue {
    protocol::os_pack::json::to_dsl_value(&protocol::os_pack::json::Value::from(data.clone()))
}

pub fn render(_document: &CadSnapshot) -> UiAssemblyResult<BuiltNode> {
    let camera = CadCamera::default();
    let camera_json = world3d_camera_projection_json(camera.position, camera.target, None, camera.zoom, &cad_camera_projection_config(&camera));
    let fallback_mesh = vec![protocol::DslValue::object([("id".to_string(), protocol::DslValue::String(CAD_VIEW_FALLBACK_MESH_KIND.to_string())), ("data".to_string(), mesh_data_to_dsl(&mesh_from_kind(CAD_VIEW_FALLBACK_MESH_KIND)))])];
    let meshes_json = protocol::json::to_json_string(&fallback_mesh);
    let instances_json = "[]".to_string();
    let selection_json = world3d_selection_json("rectangle", &[], None);
    MeshWindowKit::render(&MeshView { camera_json, meshes_json, instances_json, selection_json })
}

//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
