//! 🪚️ Process 3D viewer — the workpiece window: a genuinely independent, read-only 3D world view
//! of the processed stock. Built directly on framework `world3d_*` helpers and the artifact-level
//! `process_working_scene_from_snapshot`/`processed_mesh` pure inference functions — never routes
//! through the sibling `editor` module (policy-checked, see `📋️contract-freeze.md` §7's
//! `policyViewerPurityBreaches`).
//!
//! 🌉️ Camera and sun use hardcoded defaults (this viewer's `Config` is the framework's
//! `NoConfig` — a read-only surface needs no persisted per-session view state for a first pass),
//! documented as an intentional simplification, not a bug — mirrors `📐️cad`'s own viewer window.

use crate::artifacts::process3d::schema::inferences::processed_mesh;
use crate::artifacts::process3d::Process3dSnapshot;
use semio_framework_plugin::{build_world_3d_scene, mesh_from_kind, world3d_camera_json, world3d_scene, world3d_selection_json, LocalizedLabel, SurfaceKind, UiNode, WindowKindDefinition, WindowOptions, WorldSunConfig};
use serde_json::json;

//#region 🔖️Constants
pub const PROCESS3D_VIEW_WINDOW_MAIN: &str = "process-workpiece-view";
pub const PROCESS3D_VIEW_BODY_MAIN: &str = "process.view.main";
const PROCESS3D_VIEW_SURFACE_MAIN: &str = "process.view";
const PROCESS3D_VIEW_CONTROLLER_ID: &str = "process3d-view";
const PROCESS3D_VIEW_FALLBACK_MESH_KIND: &str = "box";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::process3d::create_process3d_viewer`. A
/// read-only twin of the editor's `process-workpiece` window kind — same `SurfaceKind::World3d`
/// body shape, no chrome measures (no sun toggle: the sun is a hardcoded default here).
pub async fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: PROCESS3D_VIEW_WINDOW_MAIN.into(),
        label: LocalizedLabel::native("Workpiece", "Werkstück"),
        body_key: PROCESS3D_VIEW_BODY_MAIN.into(),
        surface_kind: SurfaceKind::World3d,
        icon_id: "hammer".into(),
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
/// 🎥️ Hardcoded default camera/sun — the editor's own `Process3dConfig::default()` values, kept in
/// step manually (the viewer has no config lane these could be read from).
async fn default_camera_json() -> String {
    world3d_camera_json([3.0, -3.0, 2.0], [0.0, 0.0, 0.0], 45.0)
}

async fn default_sun() -> WorldSunConfig {
    WorldSunConfig { enabled: false, azimuth: 45.0, elevation: 35.0, intensity: 0.85, color: "#ffffff".into() }
}

/// 🖼️ Same fallback-box placeholder the editor's own `evaluated_preview_payload` already falls back
/// to while composed-child object resolution is unimplemented (pre-existing
/// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave-4 gap, not introduced here) — real parity
/// with the editor's *current* behavior, not a regression.
async fn view_preview_payload(fixture: &Process3dSnapshot) -> (String, String) {
    let scene = crate::artifacts::process3d::process_working_scene_from_snapshot(fixture);
    let mesh = processed_mesh(&scene, fixture.resolved_up_to).unwrap_or_else(|| mesh_from_kind(PROCESS3D_VIEW_FALLBACK_MESH_KIND));
    let meshes = json!([{ "id": "processed", "data": mesh }]);
    let instances = json!([{
        "id": "processed",
        "meshId": "processed",
        "position": [0.0, 0.0, 0.0],
        "rotation": [0.0, 0.0, 0.0, 1.0],
        "scale": [1.0, 1.0, 1.0],
        "label": fixture.stock_label,
        "selected": false,
        "hovered": false,
    }]);
    (meshes.to_string(), instances.to_string())
}

/// 👁️ The viewer's own pure render function — never calls into the sibling `editor` module.
pub async fn render(fixture: &Process3dSnapshot) -> UiNode {
    let (meshes_json, instances_json) = view_preview_payload(fixture);
    build_world_3d_scene(
        PROCESS3D_VIEW_SURFACE_MAIN,
        PROCESS3D_VIEW_CONTROLLER_ID,
        world3d_scene(default_camera_json(), meshes_json, instances_json, world3d_selection_json("rectangle", &[], None), &default_sun()),
    )
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn definition_declares_the_world3d_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, PROCESS3D_VIEW_BODY_MAIN);
        assert!(matches!(definition.surface_kind, SurfaceKind::World3d));
    }

    #[test]
    async fn render_world_scene_contains_processed_mesh() {
        let fixture = crate::artifacts::process3d::empty_process3d_snapshot();
        let node = serde_json::to_string(&render(&fixture)).expect("render json");
        assert!(node.contains("processed"), "expected the processed mesh id in scene json: {node}");
    }
}
//#endregion 🧪️Tests
