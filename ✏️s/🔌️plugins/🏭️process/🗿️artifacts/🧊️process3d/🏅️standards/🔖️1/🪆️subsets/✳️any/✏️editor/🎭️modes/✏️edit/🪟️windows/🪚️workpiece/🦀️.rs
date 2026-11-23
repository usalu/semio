//! 🪚️ Process 3d play app — the workpiece window: the 3D world view of the processed stock, plus the
//! process-timeline engagement (cursor stepper + command-line input).
use semio_framework::kernel::Effect;

use semio_s_artifact_process_process3d_config::Process3dConfig;
use crate::windows::workpiece::options;
use semio_s_artifact_process_process3d_inferences::ProcessPreviewReplay;
use semio_s_artifact_process_process3d_model::{snapshot::Process3dSnapshot, snapshot::ProcessWorkingScene, snapshot::WorkingSolid};
use semio_framework_pack_json as json;
use semio_s_artifact_process_process3d_kit::{PROCESS3D_GRANULARITY_OBJECT, PROCESS3D_INTERACTION_DOMAIN};
use semio_plugin::{scene_surface, world3d_scene};
use semio_plugin::world3d_camera_json;
use semio_plugin::world3d_selection_json;
use semio_plugin::ActionDescriptor;
use semio_plugin::BuiltNode;
use semio_ui_locale::LocalizedLabel;
use semio_plugin::SurfaceKind;
use semio_plugin::UiAssemblyResult;
use semio_plugin::WindowEngagement;
use semio_plugin::WindowEngagementControl;
use semio_plugin::WindowEngagementInput;
use semio_plugin::WindowEngagementStatus;
use semio_plugin::WindowKindDefinition;
use semio_plugin::WindowMeasure;
use semio_plugin::WindowOptions;
use semio_plugin::WorldSunConfig;

#[path = "👆️placement/🦀️.rs"]
pub mod placement;

//#region 🔖️Constants
pub const PROCESS_3D_PLAY_WINDOW_MAIN: &str = "process-workpiece";
pub const PROCESS_3D_PLAY_BODY_MAIN: &str = "process.play.main";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::process3d::create_process3d_app`. `options.measures`
/// stays empty here on purpose: this window's measures are config-derived and rebuilt per frame by
/// `window_measures`, never frozen into the manifest.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        initial_utility_id: None,
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
pub fn window_measures(config: &Process3dConfig, is_de: bool) -> Vec<WindowMeasure> {
    vec![options::sun::measure(&config_sun(config), is_de)]
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
fn process3d_selection_json(active_utility: &str, placement_active: bool, selected: &[String]) -> String {
    let mut value: semio_framework::DslValue = json::from_json_str(&world3d_selection_json("rectangle", selected, None), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|_| semio_framework::DslValue::Object(Vec::new()));
    if let semio_framework::DslValue::Object(entries) = &mut value {
        entries.retain(|(key, _)| key != "engagementSessionActive" && key != "faceDragActive");
        entries.push(("engagementSessionActive".to_string(), semio_framework::DslValue::Bool(placement_active)));
        entries.push(("faceDragActive".to_string(), semio_framework::DslValue::Bool(active_utility == "select")));
    }
    json::to_json_string(&value)
}

fn process3d_window_action(action: &str, args: Option<semio_framework::DslValue>) -> ActionDescriptor {
    ActionDescriptor { controller_id: semio_s_artifact_process_process3d_kit::PROCESS_3D_PLAY_CONTROLLER_ID.into(), action: action.into(), args }
}
//#endregion 🔖️Selection

//#region 🔖️PreviewCache
/// 🖼️ Projects the evaluated workpiece, which may be empty after a subtractive step.
fn evaluated_preview_payload(snapshot: &Process3dSnapshot, mesh: Option<semio_plugin::MeshData>, display_only: bool) -> (String, String) {
    let Some(mesh) = mesh else {
        return ("[]".into(), "[]".into());
    };
    let meshes = json::Value::Array(vec![json::object([("id".to_string(), json::Value::String("processed".to_string())), ("data".to_string(), json::Value::from(mesh))])]);
    let floats = |values: [f64; 3]| json::Value::Array(values.into_iter().map(json::Value::from).collect());
    let position = if display_only { snapshot.stock_pose.position } else { [0.0; 3] };
    let rotation = if display_only { display_rotation(&snapshot.stock_pose) } else { [0.0, 0.0, 0.0, 1.0] };
    let instances = json::Value::Array(vec![json::object([
        ("id".to_string(), json::Value::String(snapshot.stock_id.clone())),
        ("meshId".to_string(), json::Value::String("processed".to_string())),
        ("position".to_string(), floats(position)),
        ("rotation".to_string(), json::Value::Array(rotation.into_iter().map(json::Value::from).collect())),
        ("scale".to_string(), floats([1.0, 1.0, 1.0])),
        ("label".to_string(), json::Value::String(snapshot.stock_label.clone())),
        ("selected".to_string(), json::Value::Bool(false)),
        ("hovered".to_string(), json::Value::Bool(false)),
    ])]);
    (json::to_string(&meshes), json::to_string(&instances))
}

/// 🗃️ Shares one kernel evaluation between the workpiece surface and the engagement readout.
struct Process3dPreviewCache {
    scene: ProcessWorkingScene,
    resolved_up_to: Option<usize>,
    payload: (String, String),
    volume: Option<f64>,
    display_only: bool,
}

fn with_preview_cache<T>(snapshot: &Process3dSnapshot, resolved_up_to: Option<usize>, read: impl FnOnce(&Process3dPreviewCache) -> T) -> T {
    use std::sync::{Arc, Mutex};

    static CACHE: Mutex<Option<Arc<Process3dPreviewCache>>> = Mutex::new(None);
    let cached = CACHE.lock().ok().and_then(|slot| slot.clone());
    if let Some(entry) = cached.filter(|entry| {
        let stock = &entry.scene.stock;
        entry.resolved_up_to == resolved_up_to
            && stock.id == snapshot.stock_id
            && stock.label == snapshot.stock_label
            && stock.pose == snapshot.stock_pose
            && stock.solid == snapshot.stock_payload.solid
            && entry.scene.steps == snapshot.step_payloads
    }) {
        return read(&entry);
    }

    let scene = semio_s_artifact_process_process3d_model::snapshot::process_working_scene_from_snapshot(snapshot);
    let entry = Arc::new(build_preview_cache(snapshot, scene, resolved_up_to));
    let retired = CACHE.lock().ok().and_then(|mut slot| slot.replace(Arc::clone(&entry)));
    drop(retired);
    read(&entry)
}

fn imported_mesh_preview(url: &str) -> Option<semio_plugin::MeshData> {
    use semio_framework::mesh_io::{GlbImporter, MeshImporter};
    let bytes = semio_s_artifact_process_process3d_model::snapshot::decode_process3d_model_data_url(url)?;
    GlbImporter.import(&bytes).ok()
}

fn display_rotation(pose: &semio_s_artifact_process_process3d_model::snapshot::Pose) -> [f64; 4] {
    let [x, y, z] = pose.axis;
    let length = x.hypot(y).hypot(z);
    if length == 0.0 || !length.is_finite() || !pose.angle.is_finite() {
        return [0.0, 0.0, 0.0, 1.0];
    }
    let (sine, cosine) = (pose.angle * 0.5).sin_cos();
    let scale = sine / length;
    [x * scale, y * scale, z * scale, cosine]
}

fn build_preview_cache(snapshot: &Process3dSnapshot, scene: ProcessWorkingScene, resolved_up_to: Option<usize>) -> Process3dPreviewCache {
    static REPLAY: std::sync::Mutex<Option<ProcessPreviewReplay>> = std::sync::Mutex::new(None);
    let (mesh, volume, display_only) = match &scene.stock.solid {
        WorkingSolid::ImportedMesh { mesh_url } => {
            let mesh = imported_mesh_preview(mesh_url);
            let shown = mesh.is_some();
            (mesh, None, shown)
        }
        _ => {
            let mut replay = REPLAY.lock().ok().and_then(|mut slot| slot.take()).unwrap_or_default();
            let preview = replay.evaluate(&scene, resolved_up_to);
            let retired = REPLAY.lock().ok().and_then(|mut slot| slot.replace(replay));
            drop(retired);
            match preview {
                Ok((mesh, volume)) => (mesh, Some(volume), false),
                Err(_) => (None, None, false),
            }
        }
    };
    let payload = evaluated_preview_payload(snapshot, mesh, display_only);
    Process3dPreviewCache { scene, resolved_up_to, payload, volume, display_only }
}

fn preview_payload_cached(snapshot: &Process3dSnapshot, resolved_up_to: Option<usize>) -> (String, String) {
    with_preview_cache(snapshot, resolved_up_to, |entry| entry.payload.clone())
}

/// 📐️ The replayed solid's volume, served from the same memo as the mesh.
fn preview_readout(snapshot: &Process3dSnapshot, resolved_up_to: Option<usize>) -> (Option<f64>, bool) {
    with_preview_cache(snapshot, resolved_up_to, |entry| (entry.volume, entry.display_only))
}
//#endregion 🔖️PreviewCache

//#region 🔖️Render
pub fn render(snapshot: &Process3dSnapshot, config: &Process3dConfig, active_utility: &str, selected_ids: &[String], locale: semio_plugin::Locale) -> UiAssemblyResult<BuiltNode> {
    render_with_placement(snapshot, config, active_utility, selected_ids, None, None, locale)
}

/// 👆️ Renders the same workpiece with an optional local placement wireframe.
pub fn render_with_placement(snapshot: &Process3dSnapshot, config: &Process3dConfig, active_utility: &str, selected_ids: &[String], position: Option<[f64; 3]>, chosen: Option<(&str, &str)>, locale: semio_plugin::Locale) -> UiAssemblyResult<BuiltNode> {
    let placement_active = if active_utility == placement::PLACE_CAPABILITY_UTILITY {
        chosen.is_some()
    } else {
        placement::is_placement_utility(active_utility)
    };
    let (meshes_json, instances_json) = preview_payload_cached(snapshot, config.resolved_up_to);
    let selected = if selected_ids.contains(&snapshot.stock_id) && instances_json != "[]" { std::slice::from_ref(&snapshot.stock_id) } else { &[] };
    let mut scene = world3d_scene(
        world3d_camera_json(config.camera_position, config.camera_target, config.camera_fov),
        meshes_json,
        instances_json,
        process3d_selection_json(active_utility, placement_active, selected),
        &config_sun(config),
    );
    scene.engagement_preview_json = position.and_then(|position| placement::preview_json(snapshot, active_utility, chosen, position));
    scene.domain_id = Some(PROCESS3D_INTERACTION_DOMAIN.into());
    scene.domain_granularity_id = Some(PROCESS3D_GRANULARITY_OBJECT.into());
    scene_surface(PROCESS_3D_PLAY_WINDOW_MAIN, semio_plugin::app::SurfaceKind::World3d, &scene, semio_plugin::surface_name(locale, "Workpiece", "Werkstück"), None)
}
//#endregion 🔖️Render

//#region 🔖️Engagement
pub fn engagement(snapshot: &Process3dSnapshot, config: &Process3dConfig, active_utility: &str, labels: &semio_s_artifact_process_process3d_kit::editor::process3d::terminology::Process3dLabels) -> WindowEngagement {
    let len = snapshot.step_payloads.len();
    let cursor = config.resolved_up_to.unwrap_or(len).min(len);
    let outcome = match preview_readout(snapshot, config.resolved_up_to) {
        (Some(value), _) if value == 0.0 => "0.0000 m³".into(),
        (Some(value), _) if value > 0.0 && value < 0.00005 => format!("{value:.4e} m³"),
        (Some(value), _) => format!("{value:.4} m³"),
        (_, true) if labels.is_de() => "Netz nur zur Anzeige".into(),
        (_, true) => "Display-only mesh".into(),
        (None, false) if labels.is_de() => "Vorschau fehlgeschlagen".into(),
        (None, false) => "Preview failed".into(),
    };
    WindowEngagement {
        session_active: Some(placement::is_placement_utility(active_utility)),
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
        status: Some(vec![WindowEngagementStatus { id: "process3d-status".into(), text: format!("{cursor}/{len} {} · {outcome}", labels.steps.as_str()) }]),
        possible_engagements: None,
    }
}
//#endregion 🔖️Engagement

//#region 🧪️Tests
//#endregion 🧪️Tests

//#region 🔖️Constants
/// 🧰️ Switches the utility of the exact Workpiece addressed by the host.
pub fn set_active_utility_effect(view: Option<&semio_plugin::ViewModel>, utility: &str) -> Option<Effect> {
    let view = view?;
    let id = view.window_id.as_deref()?;
    view.window_instances.iter().any(|window| window.id == id && window.window_kind_id == PROCESS_3D_PLAY_WINDOW_MAIN)
        .then(|| Effect::SetActiveUtility { window_id: id.into(), utility_id: utility.into() })
}
//#endregion 🔖️Constants
