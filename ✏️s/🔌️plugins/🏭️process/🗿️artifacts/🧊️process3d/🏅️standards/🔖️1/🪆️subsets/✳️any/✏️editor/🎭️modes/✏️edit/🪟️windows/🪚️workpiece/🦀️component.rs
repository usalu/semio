//! 🪚️ Process 3d play app — the workpiece window: the 3D world view of the processed stock, plus the
//! process-timeline engagement (cursor stepper + command-line input).

use crate::artifacts::process3d::schema::inferences::processed_mesh;
use crate::artifacts::process3d::Process3dSnapshot;
use crate::editor::process3d::config::Process3dConfig;
use crate::editor::process3d::modes::edit::windows::workpiece::options;
use semio_framework_plugin::app::WindowKit;
use semio_framework_plugin::{
    mesh_from_kind, world3d_camera_json, world3d_selection_json, ActionDescriptor, BuiltNode, LocalizedLabel, MeshView, MeshWindowKit, SurfaceKind, UiAssemblyResult, WindowEngagement, WindowEngagementControl, WindowEngagementInput, WindowEngagementStatus,
    WindowKindDefinition, WindowMeasure, WindowOptions, WorldSunConfig,
};
use serde_json::json;

//#region 🔖️Constants
pub const PROCESS_3D_PLAY_WINDOW_MAIN: &str = "process-workpiece";
pub const PROCESS_3D_PLAY_BODY_MAIN: &str = "process.play.main";
const PROCESS_3D_PLAY_SURFACE_MAIN: &str = "process.play";
const PROCESS3D_FALLBACK_MESH_KIND: &str = "box";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::process3d::create_process3d_app`. `options.measures`
/// stays empty here on purpose: this window's measures are config-derived and rebuilt per frame by
/// `window_measures`, never frozen into the manifest.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: PROCESS_3D_PLAY_WINDOW_MAIN.into(),
        label: LocalizedLabel::native("Workpiece", "Werkstück"),
        body_key: PROCESS_3D_PLAY_BODY_MAIN.into(),
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

/// 🎚️ The live chrome measures for this window, collected from its `🎚️options/*` components.
pub fn window_measures(config: &Process3dConfig) -> Vec<WindowMeasure> {
    vec![options::sun::measure(&config_sun(config))]
}
//#endregion 🔖️Definition

//#region 🔖️Sun
/// 🌞️ Reconstructs the shared framework `WorldSunConfig` shape from `Process3dConfig`'s flattened sun
/// fields — `world3d_scene`/`world3d_sun_measures` are shared SDK primitives that still take the
/// nested struct.
pub fn config_sun(cfg: &Process3dConfig) -> WorldSunConfig {
    WorldSunConfig { enabled: cfg.sun_enabled, azimuth: cfg.sun_azimuth, elevation: cfg.sun_elevation, intensity: cfg.sun_intensity, color: cfg.sun_color.clone() }
}
//#endregion 🔖️Sun

//#region 🔖️Selection
/// 🖱️ `engagementSessionActive` gates the ground-click placement path used by the cut/drill/attach
/// utilities; `faceDragActive` gates the push/pull drag gesture, only while the select utility is
/// active (so a click-to-place utility doesn't also start a face drag).
///
/// 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): object/face selection AND hover are the
/// framework-owned `"geometry"` interaction domain now, unreachable at this `render` boundary
/// (`ArtifactEditor::render` carries no `InteractionView` — a known SDK gap, see `w3c-summary.md`) —
/// `selectionMode`/`targets`/`componentIds` are no longer emitted here, matching every other
/// migrated world3d call site's empty-selection `world3d_selection_json` call (e.g. `📐️cad`'s
/// `world_selection_json`).
fn process3d_selection_json(active_utility: &str) -> String {
    let mut value: serde_json::Value = serde_json::from_str(&world3d_selection_json("rectangle", &[], None)).unwrap_or_else(|_| json!({}));
    if let Some(object) = value.as_object_mut() {
        object.insert("engagementSessionActive".into(), json!(active_utility != "select"));
        object.insert("faceDragActive".into(), json!(active_utility == "select"));
    }
    value.to_string()
}

fn process3d_window_action(action: &str, args: Option<serde_json::Value>) -> ActionDescriptor {
    ActionDescriptor { controller_id: crate::editor::process3d::PROCESS_3D_PLAY_APP_ID.into(), action: action.into(), args: semio_framework::optional_json_to_dsl(args) }
}
//#endregion 🔖️Selection

//#region 🔖️PreviewCache
/// 🖼️ Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM wave 4: `fixture.stock_solid` is a
/// composed `s.stdio.semio.brep` CHILD HANDLE now, with no resolvable content without a
/// `LinkResolver` (see `ProcessWorkingScene`'s doc comment) — so the `WorkingSolid::ImportedMesh`
/// mesh-url fast path (which needed to inspect the resolved solid kind) can no longer trigger, and
/// `processed_mesh` degrades to the empty working scene (falls back to
/// `PROCESS3D_FALLBACK_MESH_KIND`), a documented gap.
fn evaluated_preview_payload(fixture: &Process3dSnapshot) -> (String, String) {
    let scene = crate::artifacts::process3d::process_working_scene_from_snapshot(fixture);
    let mesh = processed_mesh(&scene, fixture.resolved_up_to).unwrap_or_else(|| mesh_from_kind(PROCESS3D_FALLBACK_MESH_KIND));
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

fn preview_payload_cached(fixture: &Process3dSnapshot) -> (String, String) {
    evaluated_preview_payload(fixture)
}
//#endregion 🔖️PreviewCache

//#region 🔖️Render
pub fn render(fixture: &Process3dSnapshot, config: &Process3dConfig) -> UiAssemblyResult<BuiltNode> {
    let (meshes_json, instances_json) = preview_payload_cached(fixture);
    MeshWindowKit::render(&MeshView {
        camera_json: world3d_camera_json(config.camera_position, config.camera_target, config.camera_fov),
        meshes_json,
        instances_json,
        selection_json: process3d_selection_json(config.active_utility()),
    })
}
//#endregion 🔖️Render

//#region 🔖️Engagement
pub fn engagement(fixture: &Process3dSnapshot, config: &Process3dConfig, labels: &crate::editor::process3d::terminology::Process3dLabels) -> WindowEngagement {
    let active_utility = config.active_utility();
    // 🌉️ Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM wave 4: `steps` is a composed CHILD
    // HANDLE now (no `.len()` — see `ProcessWorkingScene`'s doc comment), so the stepper's `max`
    // degrades to the empty working scene's length (0), a documented gap.
    let scene = crate::artifacts::process3d::process_working_scene_from_snapshot(fixture);
    let len = scene.steps.len();
    let cursor = fixture.resolved_up_to.unwrap_or(len);
    let volume = crate::artifacts::process3d::schema::inferences::processed_volume(&scene, fixture.resolved_up_to).unwrap_or(0.0);
    WindowEngagement {
        session_active: Some(active_utility != "select"),
        // 🧰️ The select/cut/drill/attach switcher lives in the framework utility bar (declared via
        // `.utility` + `.window_kind_utilities`), so the engagement never duplicates it as options.
        options: None,
        input: Some(WindowEngagementInput {
            id: Some("process3d-engagement".into()),
            value: Some(config.engagement_input.clone()),
            placeholder: Some("cut, drill, attach, back, forward, all".into()),
            disabled: None,
            on_change: Some(process3d_window_action("engagementInput", None)),
            on_submit: Some(process3d_window_action("engagementSubmit", None)),
            on_repeat_last: None,
            on_abort: Some(process3d_window_action("engagementAbort", None)),
        }),
        control: Some(WindowEngagementControl::Stepper {
            id: Some("process3d-cursor".into()),
            label: Some(labels.step_control.into()),
            value: cursor as f64,
            min: Some(0.0),
            max: Some(len as f64),
            step: Some(1.0),
            unit: None,
            disabled: None,
            on_change: Some(process3d_window_action("setCursor", None)),
            on_commit: None,
        }),
        controls: None,
        status: Some(vec![WindowEngagementStatus { id: "process3d-status".into(), text: format!("{cursor}/{len} steps · {volume:.4} m³") }]),
        possible_engagements: None,
    }
}
//#endregion 🔖️Engagement

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::process3d::testkit;

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_the_world3d_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, PROCESS_3D_PLAY_BODY_MAIN);
        assert!(matches!(definition.surface_kind, SurfaceKind::World3d));
        assert!(definition.options.measures.is_empty(), "measures are config-derived per frame, never frozen into the manifest");
    }

    #[semio_framework_async_macros::async_test]
    async fn engagement_exposes_no_utility_switch_options() {
        let doc = Process3dSnapshot::default();
        let engagement = engagement(&doc, &Process3dConfig::default(), &crate::editor::process3d::terminology::Process3dLabels::NATIVE_EN);
        assert!(engagement.options.is_none(), "select/cut/drill/attach switching lives only on the framework utility bar; the engagement must not duplicate it as options");
    }

    #[semio_framework_async_macros::async_test]
    async fn render_world_scene_contains_processed_mesh() {
        let mut app = testkit::app();
        let node = testkit::render(&mut app, PROCESS_3D_PLAY_BODY_MAIN);
        assert!(node.contains("processed"), "expected the processed mesh id in scene json: {node}");
    }
}
//#endregion 🧪️Tests
