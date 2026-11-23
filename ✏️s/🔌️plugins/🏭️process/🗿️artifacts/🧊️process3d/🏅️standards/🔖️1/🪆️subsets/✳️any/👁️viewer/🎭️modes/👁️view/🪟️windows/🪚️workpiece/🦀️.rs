//! 🪚️ Process 3D viewer — the workpiece window: a genuinely independent, read-only 3D world view
//! of the processed stock. Built directly on framework `world3d_*` helpers and the artifact-level
//! `process_working_scene_from_snapshot`/`processed_mesh` pure inference functions — never routes
//! through the sibling `editor` module (policy-checked, see `📋️contract-freeze.md` §7's
//! `policyViewerPurityBreaches`).
//!
//! 🌉️ Camera and sun use hardcoded defaults (this viewer's `Config` is the framework's
//! `NoConfig` — a read-only surface needs no persisted per-session view state for a first pass),
//! documented as an intentional simplification, not a bug — mirrors `📐️cad`'s own viewer window.

use semio_s_artifact_process_process3d_inferences::processed_mesh;
use semio_s_artifact_process_process3d_model::snapshot::Process3dSnapshot;
use semio_framework_pack_json as json;
use semio_plugin::app::WindowKit;
use semio_plugin::PluginAssemblyError;
use semio_plugin::world3d_camera_json;
use semio_plugin::world3d_selection_json;
use semio_plugin::BuiltNode;
use semio_ui_locale::LocalizedLabel;
use semio_plugin::MeshView;
use semio_plugin::MeshWindowKit;
use semio_plugin::SurfaceKind;
use semio_plugin::UiAssemblyResult;
use semio_plugin::WindowKindDefinition;
use semio_plugin::WindowOptions;

//#region 🔖️Constants
pub const PROCESS3D_VIEW_WINDOW_MAIN: &str = "process-workpiece-view";
pub const PROCESS3D_VIEW_BODY_MAIN: &str = "process.view.main";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::process3d::create_process3d_viewer`. A
/// read-only twin of the editor's `process-workpiece` window kind — same `SurfaceKind::World3d`
/// body shape, no chrome measures (no sun toggle: the sun is a hardcoded default here).
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        initial_utility_id: None,
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
fn default_camera_json() -> String {
    world3d_camera_json([3.0, -3.0, 2.0], [0.0, 0.0, 0.0], 45.0)
}

/// 🖼️ Projects the final solid or the empty result of a complete subtraction.
fn view_preview_payload(snapshot: &Process3dSnapshot) -> UiAssemblyResult<(String, String)> {
    let scene = semio_s_artifact_process_process3d_model::snapshot::process_working_scene_from_snapshot(snapshot);
    let Some(mesh) = processed_mesh(&scene, None).map_err(|error| PluginAssemblyError::new("process3d.preview.failed", error.to_string()))? else {
        return Ok(("[]".into(), "[]".into()));
    };
    let meshes = json::Value::Array(vec![json::object([("id".to_string(), json::Value::String("processed".to_string())), ("data".to_string(), json::Value::from(mesh))])]);
    let floats = |values: [f64; 3]| json::Value::Array(values.into_iter().map(json::Value::from).collect());
    let instances = json::Value::Array(vec![json::object([
        ("id".to_string(), json::Value::String("processed".to_string())),
        ("meshId".to_string(), json::Value::String("processed".to_string())),
        ("position".to_string(), floats([0.0, 0.0, 0.0])),
        ("rotation".to_string(), json::Value::Array(vec![json::Value::from(0.0), json::Value::from(0.0), json::Value::from(0.0), json::Value::from(1.0)])),
        ("scale".to_string(), floats([1.0, 1.0, 1.0])),
        ("label".to_string(), json::Value::String(snapshot.stock_label.clone())),
        ("selected".to_string(), json::Value::Bool(false)),
        ("hovered".to_string(), json::Value::Bool(false)),
    ])]);
    Ok((json::to_string(&meshes), json::to_string(&instances)))
}

/// 👁️ The viewer's own pure render function — never calls into the sibling `editor` module.
pub fn render(snapshot: &Process3dSnapshot, locale: semio_plugin::Locale) -> UiAssemblyResult<BuiltNode> {
    let (meshes_json, instances_json) = view_preview_payload(snapshot)?;
    MeshWindowKit::render(&MeshView { camera_json: default_camera_json(), meshes_json, instances_json, selection_json: world3d_selection_json("rectangle", &[], None) }, locale)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
