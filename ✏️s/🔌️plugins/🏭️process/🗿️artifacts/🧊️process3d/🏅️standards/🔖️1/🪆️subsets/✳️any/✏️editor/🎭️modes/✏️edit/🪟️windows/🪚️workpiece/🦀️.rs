//! 🪚️ Process 3d play app — the workpiece window: the 3D world view of the processed stock, plus the
//! process-timeline engagement (cursor stepper + command-line input).

use crate::artifacts::process3d::schema::inferences::processed_mesh;
use crate::artifacts::process3d::{Process3dSnapshot, ProcessWorkingScene};
use crate::editor::process3d::config::Process3dConfig;
use crate::editor::process3d::modes::edit::windows::workpiece::options;
use semio_framework_plugin::app::WindowKit;
use semio_framework_plugin::{
    mesh_from_kind, world3d_camera_json, world3d_selection_json, ActionDescriptor, BuiltNode, LocalizedLabel, MeshView, MeshWindowKit, SurfaceKind, UiAssemblyResult, WindowEngagement, WindowEngagementControl, WindowEngagementInput, WindowEngagementStatus,
    WindowKindDefinition, WindowMeasure, WindowOptions, WorldSunConfig,
};
use semio_framework_os_kernel::json;

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

/// 🎚️ The live chrome measures for this window, collected from its `☑️options/*` components.
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
    let mut value: semio_framework::DslValue = semio_framework_os_kernel::json::from_json_str(&world3d_selection_json("rectangle", &[], None)).unwrap_or_else(|_| semio_framework::DslValue::Object(Vec::new()));
    if let semio_framework::DslValue::Object(entries) = &mut value {
        entries.retain(|(key, _)| key != "engagementSessionActive" && key != "faceDragActive");
        entries.push(("engagementSessionActive".to_string(), semio_framework::DslValue::Bool(active_utility != "select")));
        entries.push(("faceDragActive".to_string(), semio_framework::DslValue::Bool(active_utility == "select")));
    }
    semio_framework_os_kernel::json::to_json_string(&value)
}

fn process3d_window_action(action: &str, args: Option<semio_framework::DslValue>) -> ActionDescriptor {
    ActionDescriptor { controller_id: crate::editor::process3d::PROCESS_3D_PLAY_APP_ID.into(), action: action.into(), args }
}
//#endregion 🔖️Selection

//#region 🔖️PreviewCache
/// 🖼️ Builds the world scene from the snapshot's INLINE authoritative records — `stock_payload` and
/// `step_payloads` (`process_working_scene_from_snapshot`) — so a real stock and a real timeline
/// tessellate into a real mesh; only a document that carries neither falls back to
/// `PROCESS3D_FALLBACK_MESH_KIND`.
fn evaluated_preview_payload(fixture: &Process3dSnapshot, scene: &ProcessWorkingScene) -> (String, String) {
    let mesh = processed_mesh(scene, fixture.resolved_up_to).unwrap_or_else(|| mesh_from_kind(PROCESS3D_FALLBACK_MESH_KIND));
    let meshes = json::Value::Array(vec![json::object([("id".to_string(), json::Value::String("processed".to_string())), ("data".to_string(), json::Value::from(mesh))])]);
    let floats = |values: [f64; 3]| json::Value::Array(values.into_iter().map(json::Value::from).collect());
    let instances = json::Value::Array(vec![json::object([
        ("id".to_string(), json::Value::String("processed".to_string())),
        ("meshId".to_string(), json::Value::String("processed".to_string())),
        ("position".to_string(), floats([0.0, 0.0, 0.0])),
        ("rotation".to_string(), json::Value::Array(vec![json::Value::from(0.0), json::Value::from(0.0), json::Value::from(0.0), json::Value::from(1.0)])),
        ("scale".to_string(), floats([1.0, 1.0, 1.0])),
        ("label".to_string(), json::Value::String(fixture.stock_label.clone())),
        ("selected".to_string(), json::Value::Bool(false)),
        ("hovered".to_string(), json::Value::Bool(false)),
    ])]);
    (json::to_string(&meshes), json::to_string(&instances))
}

/// 🗃️ One memoized preview per distinct scene. `processed_mesh` builds a fresh kernel session,
/// replays every enabled step as a real CSG boolean, tessellates and remaps face groups — and
/// `processed_volume` replays the identical sequence again for the engagement readout, so an
/// uncached turn pays for the whole process TWICE. The host settles a turn by re-driving the plugin
/// until every requested surface publishes (`PLUGIN_UI_CONTINUATION_LIMIT`), which multiplies that
/// cost by every continuation. `ProcessWorkingScene` derives `PartialEq`, so the guard is a cheap
/// structural compare against the last scene rendered.
struct Process3dPreviewCache {
    scene: ProcessWorkingScene,
    resolved_up_to: Option<usize>,
    label: String,
    payload: (String, String),
    volume: f64,
}

fn with_preview_cache<T>(fixture: &Process3dSnapshot, read: impl Fn(&Process3dPreviewCache) -> T) -> T {
    static CACHE: std::sync::OnceLock<std::sync::Mutex<Option<Process3dPreviewCache>>> = std::sync::OnceLock::new();
    let scene = crate::artifacts::process3d::process_working_scene_from_snapshot(fixture);
    let cell = CACHE.get_or_init(|| std::sync::Mutex::new(None));
    let Ok(mut slot) = cell.lock() else {
        let entry = build_preview_cache(fixture, scene);
        return read(&entry);
    };
    let fresh = slot.as_ref().is_some_and(|entry| entry.scene == scene && entry.resolved_up_to == fixture.resolved_up_to && entry.label == fixture.stock_label);
    if !fresh {
        *slot = Some(build_preview_cache(fixture, scene));
    }
    read(slot.as_ref().expect("preview cache populated"))
}

fn build_preview_cache(fixture: &Process3dSnapshot, scene: ProcessWorkingScene) -> Process3dPreviewCache {
    let payload = evaluated_preview_payload(fixture, &scene);
    let volume = crate::artifacts::process3d::schema::inferences::processed_volume(&scene, fixture.resolved_up_to).unwrap_or(0.0);
    Process3dPreviewCache { scene, resolved_up_to: fixture.resolved_up_to, label: fixture.stock_label.clone(), payload, volume }
}

fn preview_payload_cached(fixture: &Process3dSnapshot) -> (String, String) {
    with_preview_cache(fixture, |entry| entry.payload.clone())
}

/// 📐️ The replayed solid's volume, served from the same memo as the mesh.
fn processed_volume_cached(fixture: &Process3dSnapshot) -> f64 {
    with_preview_cache(fixture, |entry| entry.volume)
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
    let len = fixture.step_payloads.len();
    let cursor = fixture.resolved_up_to.unwrap_or(len);
    let volume = processed_volume_cached(fixture);
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
