//! 👁️ Procedural3d play app — the 3D preview window (edit mode): the tessellated evaluated geometry.

use crate::artifacts::procedural3d::Procedural3dSnapshot;
use crate::editor::procedural3d::config::Procedural3dConfig;
use crate::editor::procedural3d::PROCEDURAL_3D_PLAY_APP_ID;
use crate::editor::procedural3d::{preview_camera_json, preview_payload, preview_scene_status_json, preview_selection_json, preview_status_json, PreviewInteractionMarks, PROCEDURAL_3D_INTERACTION_DOMAIN, PROCEDURAL_3D_INTERACTION_GRANULARITY};
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
pub fn render(document: &Procedural3dSnapshot, config: &Procedural3dConfig, session: &FlowEvalSession, active_utility: &str, marks: &PreviewInteractionMarks) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let eval_json = config.preview_eval_text.clone().unwrap_or_default();
    let payload = preview_payload(&eval_json, &document.fixture, config, Some(session), marks);
    let selection_json = preview_selection_json(config, active_utility, &payload);
    let (meshes_json, instances_json) = (payload.meshes_json, payload.instances_json);
    let preview_status = preview_status_json(&eval_json, &document.fixture);
    let sun = config.sun();
    let status_json = {
        let base = preview_scene_status_json(session, preview_status);
        let mut debug_object = dsl::json::Object::new();
        debug_object.insert("evalLen", dsl::json::Value::from(eval_json.len()));
        debug_object.insert("meshesLen", dsl::json::Value::from(meshes_json.len()));
        debug_object.insert("instancesLen", dsl::json::Value::from(instances_json.len()));
        debug_object.insert("evalHead", dsl::json::Value::String(eval_json.chars().take(240).collect::<String>()));
        let debug_value = dsl::json::Value::Object(debug_object);
        Some(match base {
            Some(existing) => match dsl::json::parse(&existing) {
                Ok(mut value) => {
                    if let Some(obj) = value.as_object_mut() {
                        obj.insert("debug", debug_value);
                    }
                    dsl::json::to_string(&value)
                }
                _ => dsl::json::to_string(&debug_value),
            },
            None => dsl::json::to_string(&debug_value),
        })
    };
    let _ = PROCEDURAL_3D_PLAY_APP_ID;
    crate::scene_surface(
        PROCEDURAL_3D_PLAY_SURFACE_PREVIEW,
        semio_framework_plugin::plugin_app_close_prelude::SurfaceKind::World3d,
        &ui_wgpu::wgpu::World3dScene {
            status_json,
            domain_id: Some(PROCEDURAL_3D_INTERACTION_DOMAIN.into()),
            domain_granularity_id: Some(PROCEDURAL_3D_INTERACTION_GRANULARITY.into()),
            ..world3d_scene(preview_camera_json(config), meshes_json, instances_json, selection_json, &sun)
        },
    )
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::procedural3d::testkit::{app, render as render_body};

    /// 🔎️ Recursively finds a string-valued JSON field named `key` anywhere in `value` — needed
    /// because `scene_surface` may nest the `World3dScene` fields at an arbitrary depth inside the
    /// rendered `ComponentTree`.
    fn find_json_string_field<'a>(value: &'a serde_json::Value, key: &str) -> Option<&'a str> {
        match value {
            serde_json::Value::Object(map) => {
                if let Some(serde_json::Value::String(found)) = map.get(key) {
                    return Some(found.as_str());
                }
                map.values().find_map(|entry| find_json_string_field(entry, key))
            }
            serde_json::Value::Array(items) => items.iter().find_map(|entry| find_json_string_field(entry, key)),
            _ => None,
        }
    }

    #[test]
    fn renders_world_preview_scene() {
        // 🧵️ Rendering the preview body tessellates BRep geometry through the same process-wide cache
        // `apps::procedural3d`'s own tests serialize on — see that module's `test_support`.
        let _serial = crate::editor::procedural3d::test_support::lock();
        let mut app = app();
        crate::editor::procedural3d::testkit::drain_flow_eval_ticks(&mut app);
        let json = render_body(&mut app, PROCEDURAL_3D_PLAY_BODY_PREVIEW);
        assert!(json.contains("world-3d"));
        // 🐛️ Regression guard for the empty-scene defect: `handle`/`render` used to construct a
        // brand-new `FlowEvalSession` on every call, so `eval_json` was always `""` and
        // `preview_payload` short-circuited to `("[]", "[]")` despite the default
        // `hexagonal-mushroom-column` fixture being non-empty. `json.contains("world-3d")` alone
        // never caught this — both fields still assert non-empty below.
        let value: serde_json::Value = serde_json::from_str(&json).expect("preview render must be valid json");
        let meshes_json = find_json_string_field(&value, "meshesJson").expect("world-3d scene must carry a meshesJson field");
        let instances_json = find_json_string_field(&value, "instancesJson").expect("world-3d scene must carry an instancesJson field");
        assert_ne!(meshes_json, "[]", "hexagonal-mushroom-column must tessellate into non-empty preview meshes");
        assert_ne!(instances_json, "[]", "hexagonal-mushroom-column must produce non-empty preview instances");
    }
}
//#endregion 🧪️Tests
