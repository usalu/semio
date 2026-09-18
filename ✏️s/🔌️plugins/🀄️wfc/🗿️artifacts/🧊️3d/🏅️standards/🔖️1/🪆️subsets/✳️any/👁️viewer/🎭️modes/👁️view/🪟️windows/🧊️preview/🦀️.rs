//! 🧊️ WFC 3D viewer — the read-only `wfc-3d-view` window. It solves the document itself (the viewer
//! has no app transient to cache into) and reuses the editor preview's own projection, so both
//! surfaces place the same instances over the same mesh catalogue.

use crate::editor::wfc3d::modes::edit::windows::preview as editor_preview;
use crate::editor::wfc3d::transient::solved_transient;
use crate::Wfc3dSnapshot;
use semio_framework_ui_contract::SurfaceKind as ContractSurfaceKind;
use semio_framework_plugin::{scene_surface, BuiltNode, LocalizedLabel, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowOptions, World3dScene, world3d_selection_json};

//#region 🔖️Constants
pub const WFC_3D_VIEW_WINDOW: &str = "wfc-3d-view";
pub const WFC_3D_VIEW_BODY: &str = "wfc.3d.view";
const WFC_3D_VIEW_SURFACE: &str = "wfc.3d.view";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: WFC_3D_VIEW_WINDOW.into(),
        label: LocalizedLabel::native("Preview", "Vorschau"),
        body_key: WFC_3D_VIEW_BODY.into(),
        surface_kind: SurfaceKind::World3d,
        icon_id: "box".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
        interactions: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(document: &Wfc3dSnapshot) -> UiAssemblyResult<BuiltNode> {
    let transient = solved_transient(document);
    let scene = World3dScene {
        instances_delta_json: Some(editor_preview::instances_delta_json(document, &transient)),
        status_json: Some(editor_preview::status_json(document, &transient)),
        ..World3dScene::base(
            editor_preview::camera_json(document, 1.0),
            editor_preview::meshes_json(document),
            editor_preview::instances_json(document, &transient),
            world3d_selection_json("replace", &[], None),
        )
    };
    scene_surface(WFC_3D_VIEW_SURFACE, ContractSurfaceKind::World3d, &scene)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
