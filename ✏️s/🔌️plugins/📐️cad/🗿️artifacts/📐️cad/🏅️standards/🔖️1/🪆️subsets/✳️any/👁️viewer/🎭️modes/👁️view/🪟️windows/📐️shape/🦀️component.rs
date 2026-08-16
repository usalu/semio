//! 📐️ CAD viewer — the Shape window: a read-only world-3d render of the `spatial.shape` pane, built
//! from the same subset `🧬️schema/💡️inferences` pure snapshot→view-model helpers the editor's own
//! Shape window (`✏️editor/🎭️modes/✏️edit/🪟️windows/📐️shape`) uses — never `crate::…::editor::…`
//! (`policyViewerPurityBreaches`). No selection, no engagement, no gumball/dislocate: a viewer has no
//! utilities that edit and emits no mutations by construction (`ViewEmit`).

use crate::artifacts::cad::standards::v1::subsets::any::schema::inferences::cad_camera_projection_config;
use crate::artifacts::cad::{CadCamera, CadPaneId, CadSnapshot};
use semio_framework_plugin::{
    build_world_3d_scene, mesh_from_kind, world3d_camera_projection_json, world3d_chunking_json, world3d_environment_json, world3d_scene_extended, world3d_selection_json, LocalizedLabel, UiNode, WindowKindDefinition, WindowOptions, WorldSunConfig,
};
use ui_wgpu::wgpu::SurfaceKind;

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = "cad-view-shape";
pub const BODY_KEY: &str = "cad.view.shape";
pub const SURFACE_ID: &str = "cad.view.scene3d/shape";
pub const PANE: CadPaneId = CadPaneId::Shape;
/// 👁️ Read-only counterpart of the editor's `CAD_PLAY_APP_ID` controller id — kept distinct so a
/// viewer session's world-3d controller can never be mistaken for an editor session's.
const CAD_VIEW_CONTROLLER_ID: &str = "cad-view";
/// 👁️ Matches the editor's `CAD_FALLBACK_MESH_KIND` literal ("box") — duplicated on purpose rather
/// than imported through `crate::…::editor::…`, which `policyViewerPurityBreaches` forbids outright.
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
pub fn render(document: &CadSnapshot) -> UiNode {
    let camera = CadCamera::default();
    let sun = WorldSunConfig::default();
    let camera_json = world3d_camera_projection_json(camera.position, camera.target, None, camera.zoom, &cad_camera_projection_config(&camera));
    let meshes_json = serde_json::to_string(&[serde_json::json!({ "id": CAD_VIEW_FALLBACK_MESH_KIND, "data": mesh_from_kind(CAD_VIEW_FALLBACK_MESH_KIND) })]).unwrap_or_else(|_| "[]".into());
    let instances_json = "[]".to_string();
    let selection_json = world3d_selection_json("rectangle", &[], None);
    build_world_3d_scene(
        SURFACE_ID,
        CAD_VIEW_CONTROLLER_ID,
        world3d_scene_extended(
            camera_json,
            meshes_json,
            instances_json,
            selection_json,
            None,
            None,
            None,
            world_references_json(document, PANE),
            None,
            None,
            None,
            None,
            Some(world3d_chunking_json(256.0, 8000.0)),
            Some(world3d_environment_json(&sun)),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ),
    )
}

/// 👁️ Read-only twin of the editor's `edit::world_references_json` — background reference overlays
/// are pure document content (`CadSnapshot.references_by_model_definition_id`), safe for a viewer to
/// render directly.
fn world_references_json(document: &CadSnapshot, pane: CadPaneId) -> Option<String> {
    let references = document.references_by_model_definition_id.get(pane.model_definition_id())?;
    if references.is_empty() {
        return None;
    }
    let records: Vec<serde_json::Value> = references
        .iter()
        .filter(|reference| !reference.hidden)
        .map(|reference| {
            serde_json::json!({
                "id": reference.id,
                "url": reference.source_url,
                "origin": reference.origin,
                "widthWorld": if reference.width_world > 0.0 { reference.width_world } else { 1.0 },
                "locked": reference.locked,
                "hidden": reference.hidden,
                "opacity": reference.opacity.unwrap_or(1.0),
            })
        })
        .collect();
    Some(serde_json::to_string(&records).unwrap_or_else(|_| "[]".into()))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn definition_declares_a_world3d_shape_window() {
        let def = definition();
        assert_eq!(def.id, WINDOW_KIND_ID);
        assert_eq!(def.surface_kind, SurfaceKind::World3d);
    }

    #[test]
    fn render_produces_a_scene_node_for_the_default_document() {
        let document = crate::artifacts::cad::standards::v1::subsets::any::schema::inferences::forest_play_scene();
        let _node = render(&document);
    }
}
//#endregion 🧪️Tests
