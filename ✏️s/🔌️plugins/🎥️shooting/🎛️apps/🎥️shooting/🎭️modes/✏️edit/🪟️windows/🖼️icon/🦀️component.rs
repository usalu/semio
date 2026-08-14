//! 🖼️ Shooting play app — the icon-render preview window: the active shot's rendered output.

use crate::apps::shooting::config::ShootingConfig;
use crate::apps::shooting::modes::edit::windows::icon::options;
use crate::apps::shooting::terminology::ShootingLabels;
use crate::apps::shooting::SHOOTING_PLAY_APP_ID;
use crate::artifacts::shooting::schema::shooting_icon_render_request_json;
use crate::artifacts::shooting::ShootingSnapshot;
use semio_framework_plugin::{build_icon_render_scene, IconRenderScene, LocalizedLabel, SurfaceKind, UiNode, WindowEngagement, WindowEngagementInput, WindowEngagementStatus, WindowKindDefinition, WindowMeasure, WindowOptions};

//#region 🔖️Constants
pub const SHOOTING_PLAY_WINDOW_ICON: &str = "shooting-icon";
pub const SHOOTING_PLAY_BODY_ICON: &str = "shooting.play.icon";
const SHOOTING_PLAY_SURFACE_ICON: &str = "shooting.play.icon";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::apps::shooting::create_shooting_app`. `options.measures`
/// stays empty here on purpose: shooting's measures are config-derived and rebuilt per frame by
/// [`window_measures`], not frozen into the manifest.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: SHOOTING_PLAY_WINDOW_ICON.into(),
        label: LocalizedLabel::native("Icon", "Symbol"),
        body_key: SHOOTING_PLAY_BODY_ICON.into(),
        surface_kind: SurfaceKind::IconRender,
        icon_id: "image".into(),
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
pub fn window_measures(snapshot: &ShootingSnapshot, labels: &ShootingLabels) -> Vec<WindowMeasure> {
    vec![options::shot::measure(snapshot, labels), options::format::measure(snapshot, labels), options::shape::measure(snapshot, labels)]
}

pub fn engagement(snapshot: &ShootingSnapshot, labels: &ShootingLabels) -> WindowEngagement {
    let shot = crate::artifacts::shooting::schema::active_shot(snapshot);
    WindowEngagement {
        session_active: Some(true),
        options: None,
        input: Some(WindowEngagementInput {
            id: Some("shooting.shot-label".into()),
            value: shot.map(|entry| entry.label.clone()),
            placeholder: Some(labels.shot_label_placeholder.into()),
            disabled: Some(shot.is_none()),
            on_change: Some(crate::apps::shooting::shooting_action("setActiveShotLabel", None)),
            on_submit: None,
            on_repeat_last: None,
            on_abort: None,
        }),
        control: None,
        controls: None,
        status: Some(vec![WindowEngagementStatus { id: "shooting.status.icon".into(), text: shot.map_or_else(|| labels.no_shot.into(), |entry| format!("{}×{} {}", entry.width, entry.height, entry.format.to_uppercase())) }]),
        possible_engagements: None,
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(snapshot: &ShootingSnapshot, cfg: &ShootingConfig) -> UiNode {
    let (request_json, footer) = match (crate::artifacts::shooting::schema::active_shot(snapshot), crate::artifacts::shooting::schema::active_asset(snapshot)) {
        (Some(shot), Some(asset)) => (shooting_icon_render_request_json(snapshot, shot, asset, &cfg.camera), Some(format!("{} · {}×{} · {}", shot.label, shot.width, shot.height, shot.format.to_uppercase()))),
        _ => ("null".into(), None),
    };
    build_icon_render_scene(SHOOTING_PLAY_SURFACE_ICON, SHOOTING_PLAY_APP_ID, IconRenderScene { request_json, footer, frame_json: None })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::shooting::testkit::{icon_window_measures, shooting_app};
    use crate::apps::shooting::SHOOTING_PLAY_BODY_ICON as BODY_ICON;
    use semio_framework_plugin::{PluginApp, ViewModel};
    use serde_json::{json, Value};

    #[test]
    fn renders_icon_render_scene_with_real_request() {
        let mut app = shooting_app();
        let node = app.render(BODY_ICON, None, &ViewModel::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("icon-render"));
        let payload: Value = serde_json::from_str(&json).unwrap();
        let request: Value = serde_json::from_str(payload["iconRender"]["requestJson"].as_str().unwrap()).unwrap();
        assert_eq!(request["assetUrl"], json!("/mesh/🧊️base.glb"));
        assert_eq!(request["format"], json!("svg"));
        assert_eq!(request["shape"], json!("rectangle"));
        assert!(request.get("background").is_none(), "transparent default fixture background is omitted");
        assert_eq!(request["lights"]["sunAzimuth"], json!(45.0));
        assert!(payload["iconRender"]["footer"].as_str().unwrap().contains("256×256"));
    }

    #[test]
    fn window_measures_surface_three_icon_measures() {
        let mut app = shooting_app();
        let measures = icon_window_measures(&mut app);
        assert_eq!(measures.len(), 3);
        assert!(measures.iter().any(|measure| matches!(measure, WindowMeasure::Select { .. })));
    }

    #[test]
    fn definition_declares_the_icon_render_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, SHOOTING_PLAY_BODY_ICON);
        assert!(matches!(definition.surface_kind, SurfaceKind::IconRender));
        assert!(definition.options.measures.is_empty(), "measures are config-derived per frame, never frozen into the manifest");
    }
}
//#endregion 🧪️Tests
