//! 🪚️ Process 3d play app — the workpiece window: the 3D world view of the processed stock, plus the
//! process-timeline engagement (cursor stepper + command-line input).

use crate::apps::process3d::config::Process3dConfig;
use crate::apps::process3d::modes::edit::windows::workpiece::options;
use crate::apps::process3d::process3d_action;
use crate::artifacts::process3d::engine::processed_mesh;
use crate::artifacts::process3d::{Process3dSnapshot, SolidSpec};
use semio_framework_plugin::{
    build_world_3d_scene, mesh_from_kind, world3d_camera_json, world3d_mesh_id_from_url, world3d_scene, world3d_selection_json, LocalizedLabel, SurfaceKind, UiNode, WindowEngagement, WindowEngagementControl, WindowEngagementInput,
    WindowEngagementStatus, WindowKindDefinition, WindowMeasure, WindowOptions, WorldSunConfig,
};
use serde::Serialize;
use serde_json::json;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::{Mutex, OnceLock};

//#region 🔖️Constants
pub const PROCESS_3D_PLAY_WINDOW_MAIN: &str = "process-workpiece";
pub const PROCESS_3D_PLAY_BODY_MAIN: &str = "process.play.main";
const PROCESS_3D_PLAY_SURFACE_MAIN: &str = "process.play";
const PROCESS3D_FALLBACK_MESH_KIND: &str = "box";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::apps::process3d::create_process3d_app`. `options.measures`
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
        params_schema: None,
        document_snapshot_schema: None,
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
/// 🖱️ Extends the base object-selection JSON with face-picking/drag fields: `targets.face` lets the
/// renderer hit-test individual triangles; `engagementSessionActive` gates the ground-click placement
/// path used by the cut/drill/attach utilities; `faceDragActive` gates the push/pull drag gesture, only
/// while the select utility is active (so a click-to-place utility doesn't also start a face drag).
fn process3d_selection_json(cfg: &Process3dConfig, active_utility: &str) -> String {
    let mut value: serde_json::Value = serde_json::from_str(&world3d_selection_json(&cfg.selection_method, &cfg.selected_id.clone().into_iter().collect::<Vec<_>>(), cfg.hovered_id.as_deref())).unwrap_or_else(|_| json!({}));
    if let Some(object) = value.as_object_mut() {
        object.insert("engagementSessionActive".into(), json!(active_utility != "select"));
        object.insert("selectionMode".into(), json!("face"));
        object.insert("targets".into(), json!({ "mesh": true, "face": true, "vertex": false, "edge": false }));
        object.insert("componentIds".into(), json!(cfg.selected_face_id.map(|id| vec![id]).unwrap_or_default()));
        object.insert("faceDragActive".into(), json!(active_utility == "select"));
    }
    value.to_string()
}
//#endregion 🔖️Selection

//#region 🔖️PreviewCache
/// 🖼️ A GLB-imported reference mesh (`SolidSpec::ImportedMesh`) has no kernel-side geometry to
/// tessellate; it renders by pointing the world3d scene straight at `mesh_url`, mirroring `cad`'s
/// `resolve_object_mesh_url` → `world3d_mesh_id_from_url` bridge.
fn evaluated_preview_payload(fixture: &Process3dSnapshot) -> (String, String) {
    if let SolidSpec::ImportedMesh { mesh_url } = &fixture.stock.solid {
        let mesh_id = world3d_mesh_id_from_url(mesh_url);
        let meshes = json!([{ "id": mesh_id, "url": mesh_url }]);
        let instances = json!([{
            "id": "processed",
            "meshId": mesh_id,
            "position": [0.0, 0.0, 0.0],
            "rotation": [0.0, 0.0, 0.0, 1.0],
            "scale": [1.0, 1.0, 1.0],
            "label": fixture.stock.label,
            "selected": false,
            "hovered": false,
        }]);
        return (meshes.to_string(), instances.to_string());
    }
    let mesh = processed_mesh(fixture).unwrap_or_else(|| mesh_from_kind(PROCESS3D_FALLBACK_MESH_KIND));
    let meshes = json!([{ "id": "processed", "data": mesh }]);
    let instances = json!([{
        "id": "processed",
        "meshId": "processed",
        "position": [0.0, 0.0, 0.0],
        "rotation": [0.0, 0.0, 0.0, 1.0],
        "scale": [1.0, 1.0, 1.0],
        "label": fixture.stock.label,
        "selected": false,
        "hovered": false,
    }]);
    (meshes.to_string(), instances.to_string())
}

fn hash_value<T: Serialize>(value: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    if let Ok(json) = serde_json::to_string(value) {
        json.hash(&mut hasher);
    }
    hasher.finish()
}

fn preview_payload_cached(fixture: &Process3dSnapshot) -> (String, String) {
    evaluated_preview_payload(fixture)
}
//#endregion 🔖️PreviewCache

//#region 🔖️Render
pub fn render(fixture: &Process3dSnapshot, config: &Process3dConfig) -> UiNode {
    let (meshes_json, instances_json) = preview_payload_cached(fixture);
    build_world_3d_scene(
        PROCESS_3D_PLAY_SURFACE_MAIN,
        crate::apps::process3d::PROCESS_3D_PLAY_APP_ID,
        world3d_scene(
            world3d_camera_json(config.camera_position, config.camera_target, config.camera_fov),
            meshes_json,
            instances_json,
            process3d_selection_json(config, config.active_utility()),
            &config_sun(config),
        ),
    )
}
//#endregion 🔖️Render

//#region 🔖️Engagement
pub fn engagement(fixture: &Process3dSnapshot, config: &Process3dConfig, labels: &crate::apps::process3d::terminology::Process3dLabels) -> WindowEngagement {
    let active_utility = config.active_utility();
    let len = fixture.steps.len();
    let cursor = fixture.resolved_up_to.unwrap_or(len);
    let volume = crate::artifacts::process3d::engine::processed_volume(fixture).unwrap_or(0.0);
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
            on_change: Some(process3d_action("engagementInput", None)),
            on_submit: Some(process3d_action("engagementSubmit", None)),
            on_repeat_last: None,
            on_abort: Some(process3d_action("engagementAbort", None)),
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
            on_change: Some(process3d_action("setCursor", None)),
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
    use crate::apps::process3d::testkit;

    #[test]
    fn definition_declares_the_world3d_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, PROCESS_3D_PLAY_BODY_MAIN);
        assert!(matches!(definition.surface_kind, SurfaceKind::World3d));
        assert!(definition.options.measures.is_empty(), "measures are config-derived per frame, never frozen into the manifest");
    }

    #[test]
    fn engagement_exposes_no_utility_switch_options() {
        let doc = Process3dSnapshot::default();
        let engagement = engagement(&doc, &Process3dConfig::default(), &crate::apps::process3d::terminology::Process3dLabels::NATIVE_EN);
        assert!(engagement.options.is_none(), "select/cut/drill/attach switching lives only on the framework utility bar; the engagement must not duplicate it as options");
    }

    #[test]
    fn render_world_scene_contains_processed_mesh() {
        let mut app = testkit::app();
        let node = testkit::render(&mut app, PROCESS_3D_PLAY_BODY_MAIN);
        assert!(node.contains("processed"), "expected the processed mesh id in scene json: {node}");
    }
}
//#endregion 🧪️Tests
