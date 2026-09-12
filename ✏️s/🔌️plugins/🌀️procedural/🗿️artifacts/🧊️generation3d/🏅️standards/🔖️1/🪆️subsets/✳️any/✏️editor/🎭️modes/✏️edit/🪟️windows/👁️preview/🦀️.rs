//! 👁️ Generation3d play app — the 3D preview window (edit mode): the tessellated evaluated geometry.

use crate::editor::generation3d::config::Generation3dConfig;
use crate::editor::generation3d::GENERATION_3D_PLAY_APP_ID;
use crate::editor::generation3d::{preview_camera_json, preview_payload, preview_selection_json, preview_status_json, preview_window_status_json, PreviewInteractionMarks, PreviewStatusDebug, GENERATION_3D_INTERACTION_DOMAIN, GENERATION_3D_INTERACTION_GRANULARITY};
use crate::Generation3dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{world3d_scene, world3d_sun_measures, ActionDescriptor, BuiltNode, LocalizedLabel, MeasureSelectItem, SurfaceKind, WindowKindDefinition, WindowMeasure, WindowOptions};

#[path = "🫧️transient/🦀️.rs"]
pub mod transient;

//#region 🔖️Constants
pub const GENERATION_3D_PLAY_WINDOW_PREVIEW: &str = "procedural-preview";
pub const GENERATION_3D_PLAY_BODY_PREVIEW: &str = "procedural.play.preview";
const GENERATION_3D_PLAY_SURFACE_PREVIEW: &str = "procedural.play.preview";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: GENERATION_3D_PLAY_WINDOW_PREVIEW.into(),
        label: LocalizedLabel::native("Preview", "Vorschau"),
        body_key: GENERATION_3D_PLAY_BODY_PREVIEW.into(),
        surface_kind: SurfaceKind::World3d,
        icon_id: "preview".into(),
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

/// 👁️ Preview shading mode for the world-3d window.
pub fn show_mode_measure(show_mode: &str, procedural_action: impl Fn(&str, Option<serde_json::Value>) -> ActionDescriptor) -> WindowMeasure {
    let current = if show_mode.is_empty() { "shaded" } else { show_mode };
    WindowMeasure::Select {
        id: "generation3d-measure-show".into(),
        label: Some("Show".into()),
        value: current.into(),
        items: vec![
            MeasureSelectItem { id: "generation3d-measure-show-shaded".into(), value: "shaded".into(), label: "Shaded".into() },
            MeasureSelectItem { id: "generation3d-measure-show-edges".into(), value: "shaded+edges".into(), label: "Shaded + edges".into() },
            MeasureSelectItem { id: "generation3d-measure-show-wireframe".into(), value: "wireframe".into(), label: "Wireframe".into() },
            MeasureSelectItem { id: "generation3d-measure-show-points".into(), value: "points".into(), label: "Points".into() },
        ],
        on_change: procedural_action("setShowMode", None),
    }
}

/// 🎚️ Shared preview-window chrome measures (show-mode toggle + sun group) — reused by both preview
/// windows (edit mode's 3D preview and generate mode's generation preview).
pub fn preview_window_measures(config: &Generation3dConfig, procedural_action: impl Fn(&str, Option<serde_json::Value>) -> ActionDescriptor + Copy) -> Vec<WindowMeasure> {
    let sun = config.sun();
    vec![show_mode_measure(&config.show_mode, procedural_action), world3d_sun_measures("generation3d", &sun, procedural_action)]
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(document: &Generation3dSnapshot, config: &Generation3dConfig, preview_eval_text: Option<&str>, session: &FlowEvalSession, active_utility: &str, marks: &PreviewInteractionMarks) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let eval_json = preview_eval_text.unwrap_or_default().to_string();
    let payload = preview_payload(&eval_json, &document.fixture, config, Some(session), marks);
    let selection_json = preview_selection_json(config, active_utility, &payload);
    let (meshes_json, instances_json) = (payload.meshes_json, payload.instances_json);
    let preview_status = preview_status_json(&eval_json, &document.fixture);
    let sun = config.sun();
    let status_json = preview_window_status_json(Some(session), preview_status, &PreviewStatusDebug { eval_json: &eval_json, meshes_json: &meshes_json, instances_json: &instances_json }, None);
    let _ = GENERATION_3D_PLAY_APP_ID;
    crate::scene_surface(
        GENERATION_3D_PLAY_SURFACE_PREVIEW,
        semio_framework_plugin::plugin_app_close_prelude::SurfaceKind::World3d,
        &semio_framework_ui::wgpu::World3dScene {
            status_json,
            domain_id: Some(GENERATION_3D_INTERACTION_DOMAIN.into()),
            domain_granularity_id: Some(GENERATION_3D_INTERACTION_GRANULARITY.into()),
            ..world3d_scene(preview_camera_json(config), meshes_json, instances_json, selection_json, &sun)
        },
    )
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "./🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
