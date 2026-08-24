//! 👁️ Procedural3d play app — the 3D preview window (edit mode): the tessellated evaluated geometry.

use crate::artifacts::procedural3d::Procedural3dSnapshot;
use crate::editor::procedural3d::config::Procedural3dConfig;
use crate::editor::procedural3d::PROCEDURAL_3D_PLAY_APP_ID;
use crate::editor::procedural3d::{preview_camera_json, preview_payload_from_eval_with_session, preview_scene_status_json, preview_selection_json, preview_status_json};
use flow::FlowEvalSession;
use semio_framework_plugin::{world3d_scene, world3d_sun_measures, ActionDescriptor, BuiltNode, LocalizedLabel, MeasureSelectItem, SurfaceKind, WindowKindDefinition, WindowMeasure, WindowOptions};

//#region 🔖️Constants
pub const PROCEDURAL_3D_PLAY_WINDOW_PREVIEW: &str = "procedural-preview";
pub const PROCEDURAL_3D_PLAY_BODY_PREVIEW: &str = "procedural.play.preview";
const PROCEDURAL_3D_PLAY_SURFACE_PREVIEW: &str = "procedural.play.preview";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: PROCEDURAL_3D_PLAY_WINDOW_PREVIEW.into(),
        label: LocalizedLabel::native("Preview", "Vorschau"),
        body_key: PROCEDURAL_3D_PLAY_BODY_PREVIEW.into(),
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
        id: "procedural3d-measure-show".into(),
        label: Some("Show".into()),
        value: current.into(),
        items: vec![
            MeasureSelectItem { id: "procedural3d-measure-show-shaded".into(), value: "shaded".into(), label: "Shaded".into() },
            MeasureSelectItem { id: "procedural3d-measure-show-edges".into(), value: "shaded+edges".into(), label: "Shaded + edges".into() },
            MeasureSelectItem { id: "procedural3d-measure-show-wireframe".into(), value: "wireframe".into(), label: "Wireframe".into() },
            MeasureSelectItem { id: "procedural3d-measure-show-points".into(), value: "points".into(), label: "Points".into() },
        ],
        on_change: procedural_action("setShowMode", None),
    }
}

/// 🎚️ Shared preview-window chrome measures (show-mode toggle + sun group) — reused by both preview
/// windows (edit mode's 3D preview and generate mode's generation preview).
pub fn preview_window_measures(config: &Procedural3dConfig, procedural_action: impl Fn(&str, Option<serde_json::Value>) -> ActionDescriptor + Copy) -> Vec<WindowMeasure> {
    let sun = config.sun();
    vec![show_mode_measure(&config.show_mode, procedural_action), world3d_sun_measures("procedural3d", &sun, procedural_action)]
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(document: &Procedural3dSnapshot, config: &Procedural3dConfig, session: &FlowEvalSession, active_utility: &str) -> BuiltNode {
    let eval_json = session.eval_json().to_string();
    let (meshes_json, instances_json) = preview_payload_from_eval_with_session(&eval_json, &document.fixture, config, Some(session));
    let preview_status = preview_status_json(&eval_json, &document.fixture);
    let sun = config.sun();
    let status_json = {
        let base = preview_scene_status_json(session, preview_status);
        let debug_value = serde_json::json!({
            "evalLen": eval_json.len(),
            "meshesLen": meshes_json.len(),
            "instancesLen": instances_json.len(),
            "evalHead": eval_json.chars().take(240).collect::<String>()});
        Some(match base {
            Some(existing) => match serde_json::from_str::<serde_json::Value>(&existing) {
                Ok(mut value) => {
                    if let Some(obj) = value.as_object_mut() {
                        obj.insert("debug".into(), debug_value);
                    }
                    value.to_string()
                }
                _ => debug_value.to_string(),
            },
            None => debug_value.to_string(),
        })
    };
    let _ = PROCEDURAL_3D_PLAY_APP_ID;
    crate::scene_surface(
        PROCEDURAL_3D_PLAY_SURFACE_PREVIEW,
        semio_framework_plugin::plugin_app_close_prelude::SurfaceKind::World3d,
        &ui_wgpu::wgpu::World3dScene { status_json, ..world3d_scene(preview_camera_json(config), meshes_json, instances_json, preview_selection_json(config, active_utility), &sun) },
    )
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::procedural3d::testkit::{app, render as render_body};

    #[test]
    fn renders_world_preview_scene() {
        // 🧵️ Rendering the preview body tessellates BRep geometry through the same process-wide cache
        // `apps::procedural3d`'s own tests serialize on — see that module's `test_support`.
        let _serial = crate::editor::procedural3d::test_support::lock();
        let mut app = app();
        crate::editor::procedural3d::testkit::drain_flow_eval_ticks(&mut app);
        let json = render_body(&mut app, PROCEDURAL_3D_PLAY_BODY_PREVIEW);
        assert!(json.contains("world-3d"));
    }
}
//#endregion 🧪️Tests
