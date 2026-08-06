//! 🎥️ Shooting play app — the 3D scene window: the editable studio viewport (assets + lighting).

use crate::apps::shooting::config::ShootingConfig;
use crate::apps::shooting::modes::edit::windows::scene::options;
use crate::apps::shooting::terminology::ShootingLabels;
use crate::apps::shooting::SHOOTING_PLAY_APP_ID;
use crate::artifacts::shooting::engine::is_transparent_shooting_background;
use crate::artifacts::shooting::{shooting_asset_scale, ShootingAsset, ShootingFixture, ShootingShot};
use semio_framework_plugin::{
    build_world_3d_scene, world3d_mesh_id_from_url, world3d_meshes_json_from_kinds_and_urls, world3d_scene, world3d_selection_json, LocalizedLabel, SurfaceKind, UiNode, WindowEngagement, WindowEngagementInput, WindowEngagementPossible,
    WindowEngagementStatus, WindowKindDefinition, WindowMeasure, WindowOptions, World3dScene, WorldSunConfig,
};
use serde_json::{json, Value};
use std::collections::HashSet;

//#region 🔖️Constants
pub const SHOOTING_PLAY_WINDOW_SCENE: &str = "shooting-scene";
pub const SHOOTING_PLAY_BODY_SCENE: &str = "shooting.play.scene";
const SHOOTING_PLAY_SURFACE_SCENE: &str = "shooting.play.scene";
const SHOOTING_FALLBACK_MESH_KIND: &str = "box";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::apps::shooting::create_shooting_app`. `options.measures`
/// stays empty here on purpose: shooting's measures are config-derived and rebuilt per frame by
/// [`window_measures`], not frozen into the manifest.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: SHOOTING_PLAY_WINDOW_SCENE.into(),
        label: LocalizedLabel::native("Scene", "Szene"),
        body_key: SHOOTING_PLAY_BODY_SCENE.into(),
        surface_kind: SurfaceKind::World3d,
        icon_id: "shooting-scene".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        params_schema: None,
        document_projection_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}

/// 🎚️ The live chrome measures for this window, collected from its `🎚️options/*` components.
pub fn window_measures(fixture: &ShootingFixture, labels: &ShootingLabels) -> Vec<WindowMeasure> {
    vec![
        options::center_model::measure(labels),
        options::sun_enabled::measure(fixture, labels),
        options::sun_azimuth::measure(fixture, labels),
        options::sun_elevation::measure(fixture, labels),
        options::sun_intensity::measure(fixture, labels),
        options::ambient::measure(fixture, labels),
        options::shadow::measure(fixture, labels),
        options::roughness::measure(fixture, labels),
    ]
}

pub fn engagement(fixture: &ShootingFixture, config: &ShootingConfig, labels: &ShootingLabels) -> WindowEngagement {
    WindowEngagement {
        session_active: Some(true),
        options: None,
        input: Some(WindowEngagementInput {
            id: Some("shooting.camera-draft".into()),
            value: Some(config.camera_draft_label.clone()),
            placeholder: Some(labels.camera_label_placeholder.into()),
            disabled: None,
            on_change: Some(crate::apps::shooting::shooting_action("setCameraDraftLabel", None)),
            on_submit: Some(crate::apps::shooting::shooting_action("saveCamera", None)),
            on_repeat_last: None,
            on_abort: None,
        }),
        control: None,
        controls: None,
        status: Some(vec![WindowEngagementStatus { id: "shooting.status.model".into(), text: format!("{} assets · {} shots", fixture.assets.len(), fixture.shots.len()) }]),
        possible_engagements: Some(
            fixture
                .saved_cameras
                .iter()
                .map(|saved| WindowEngagementPossible {
                    id: format!("shooting.camera.{}", saved.id),
                    label: saved.label.clone(),
                    detail: Some(labels.load_camera.into()),
                    action: Some(crate::apps::shooting::shooting_action("loadSavedCamera", Some(json!({ "id": saved.id })))),
                })
                .collect(),
        ),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn camera_json(camera: &crate::artifacts::shooting::ShootingCamera) -> String {
    let mut value = json!({
        "position": camera.position,
        "target": camera.target,
        "fov": camera.fov,
        "zoom": camera.zoom,
        "projection": camera.projection.clone().unwrap_or_else(|| "perspective".into()),
    });
    if let (Some(object), Some(up)) = (value.as_object_mut(), camera.up) {
        object.insert("up".into(), json!(up));
    }
    value.to_string()
}

fn resolve_asset_mesh_url(asset: &ShootingAsset) -> Option<String> {
    if asset.url.is_empty() {
        None
    } else {
        Some(asset.url.clone())
    }
}

fn collect_mesh_urls(fixture: &ShootingFixture) -> Vec<String> {
    let mut urls = HashSet::new();
    for asset in &fixture.assets {
        if let Some(url) = resolve_asset_mesh_url(asset) {
            urls.insert(url);
        }
    }
    urls.into_iter().collect()
}

fn world_instances_json(fixture: &ShootingFixture, cfg: &ShootingConfig) -> String {
    let instances: Vec<Value> = fixture
        .assets
        .iter()
        .map(|asset| {
            let active = fixture.active_asset_id == asset.id || (fixture.active_asset_id.is_empty() && fixture.assets.first().map(|entry| &entry.id) == Some(&asset.id));
            let selected = cfg.selected_asset_ids.contains(&asset.id) || active;
            let hovered = cfg.hovered_asset_id.as_deref() == Some(asset.id.as_str());
            let mesh_id = resolve_asset_mesh_url(asset).map_or_else(|| SHOOTING_FALLBACK_MESH_KIND.into(), |url| world3d_mesh_id_from_url(&url));
            json!({
                "id": asset.id,
                "meshId": mesh_id,
                "position": [
                    asset.origin.first().copied().unwrap_or(0.0),
                    asset.origin.get(1).copied().unwrap_or(0.0),
                    asset.origin.get(2).copied().unwrap_or(0.0),
                ],
                "rotation": asset.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]),
                "scale": shooting_asset_scale(asset),
                "label": asset.name,
                "color": if selected { "#9aa0ab" } else { "#6b7280" },
                "selected": selected,
                "hovered": hovered,
            })
        })
        .collect();
    serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into())
}

fn world_meshes_json(fixture: &ShootingFixture) -> String {
    world3d_meshes_json_from_kinds_and_urls(&[SHOOTING_FALLBACK_MESH_KIND.into()], &collect_mesh_urls(fixture))
}

fn selection_centroid(fixture: &ShootingFixture, selected_ids: &[String]) -> Option<[f64; 3]> {
    let selected: Vec<&ShootingAsset> = fixture.assets.iter().filter(|asset| selected_ids.contains(&asset.id)).collect();
    if selected.is_empty() {
        return None;
    }
    let count = selected.len() as f64;
    let sum = selected.iter().fold([0.0f64; 3], |acc, asset| [acc[0] + asset.origin[0], acc[1] + asset.origin[1], acc[2] + asset.origin[2]]);
    Some([sum[0] / count, sum[1] / count, sum[2] / count])
}

fn world_selection_json(fixture: &ShootingFixture, cfg: &ShootingConfig) -> String {
    let mut value: Value = serde_json::from_str(&world3d_selection_json(&cfg.selection_method, &cfg.selected_asset_ids, cfg.hovered_asset_id.as_deref())).unwrap_or_else(|_| json!({}));
    if let Some(object) = value.as_object_mut() {
        object.insert("granularity".into(), json!("mesh"));
        object.insert("selectionMode".into(), json!("mesh"));
        object.insert("targets".into(), json!({ "mesh": true, "vertex": false, "edge": false, "face": false }));
        object.insert("transformMode".into(), json!(cfg.active_utility_id));
        object.insert("activeObjectId".into(), json!(fixture.active_asset_id));
        object.insert("gumballActive".into(), json!(!cfg.selected_asset_ids.is_empty()));
        if let Some(target) = selection_centroid(fixture, &cfg.selected_asset_ids) {
            object.insert("gumballTarget".into(), json!(target));
        }
    }
    value.to_string()
}

fn shooting_environment_json(fixture: &ShootingFixture) -> String {
    let scene = &fixture.scene;
    let mut value = json!({
        "ambient": { "intensity": scene.ambient.intensity, "color": scene.ambient.color },
        "sun": { "enabled": scene.sun.enabled, "azimuth": scene.sun.azimuth, "elevation": scene.sun.elevation, "intensity": scene.sun.intensity, "color": scene.sun.color },
        "shadow": { "enabled": scene.shadow.enabled, "opacity": scene.shadow.opacity, "softness": scene.shadow.softness },
        "material": { "color": scene.material.color, "metalness": scene.material.metalness, "roughness": scene.material.roughness, "emissive": scene.material.emissive, "emissiveIntensity": scene.material.emissive_intensity },
    });
    if let Some(object) = value.as_object_mut() {
        if !is_transparent_shooting_background(&scene.background) {
            object.insert("background".into(), json!(scene.background));
        }
    }
    value.to_string()
}

fn shooting_frame_json(shot: &ShootingShot) -> String {
    json!({ "width": shot.width, "height": shot.height, "shape": shot.shape, "badge": true }).to_string()
}

fn shooting_fit_json(cfg: &ShootingConfig) -> String {
    json!({ "enabled": cfg.center_model, "revision": cfg.fit_revision, "padding": 1.25 }).to_string()
}

pub fn render(fixture: &ShootingFixture, cfg: &ShootingConfig) -> UiNode {
    build_world_3d_scene(
        SHOOTING_PLAY_SURFACE_SCENE,
        SHOOTING_PLAY_APP_ID,
        World3dScene {
            environment_json: Some(shooting_environment_json(fixture)),
            frame_json: crate::artifacts::shooting::engine::active_shot(fixture).map(shooting_frame_json),
            fit_json: Some(shooting_fit_json(cfg)),
            ..world3d_scene(camera_json(&cfg.camera), world_meshes_json(fixture), world_instances_json(fixture, cfg), world_selection_json(fixture, cfg), &WorldSunConfig::default())
        },
    )
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::shooting::testkit::{scene_window_measures, shooting_app};
    use crate::apps::shooting::SHOOTING_PLAY_BODY_SCENE as BODY_SCENE;
    use semio_framework_plugin::{PluginApp, ViewModel};

    #[test]
    fn renders_world_model_scene() {
        let mut app = shooting_app();
        let node = app.render(BODY_SCENE, None, &ViewModel::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("world-3d"));
        let payload: Value = serde_json::from_str(&json).unwrap();
        let environment: Value = serde_json::from_str(payload["world3d"]["environmentJson"].as_str().unwrap()).unwrap();
        assert_eq!(environment["sun"]["azimuth"], json!(45.0));
        assert_eq!(environment["material"]["roughness"], json!(1.0));
        let frame: Value = serde_json::from_str(payload["world3d"]["frameJson"].as_str().unwrap()).unwrap();
        assert_eq!(frame["width"], json!(256));
        assert_eq!(frame["shape"], json!("rectangle"));
        let fit: Value = serde_json::from_str(payload["world3d"]["fitJson"].as_str().unwrap()).unwrap();
        assert_eq!(fit["enabled"], json!(true));
        let camera: Value = serde_json::from_str(payload["world3d"]["cameraJson"].as_str().unwrap()).unwrap();
        assert_eq!(camera["zoom"], json!(1.0));
        assert_eq!(camera["projection"], json!("perspective"));
    }

    #[test]
    fn model_scene_uses_asset_mesh_urls() {
        let mut app = shooting_app();
        let node = app.render(BODY_SCENE, None, &ViewModel::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("mesh:🧊️base"));
        assert!(json.contains("/mesh/🧊️base.glb"));
    }

    #[test]
    fn window_measures_surface_eight_scene_measures() {
        let mut app = shooting_app();
        let measures = scene_window_measures(&mut app);
        assert_eq!(measures.len(), 8);
    }

    #[test]
    fn definition_declares_the_world_3d_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, SHOOTING_PLAY_BODY_SCENE);
        assert!(matches!(definition.surface_kind, SurfaceKind::World3d));
        assert!(definition.options.measures.is_empty(), "measures are config-derived per frame, never frozen into the manifest");
    }
}
//#endregion 🧪️Tests
