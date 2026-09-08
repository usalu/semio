//! 🎥️ Shooting play app — the 3D scene window: the editable studio viewport (assets + lighting).

use crate::standards::v1::subsets::any::schema::is_transparent_shooting_background;
use crate::{shooting_asset_scale, ShootingAsset, ShootingShot, ShootingSnapshot};
use crate::editor::shooting::config::ShootingConfig;
use crate::editor::shooting::modes::edit::windows::scene::options;
use crate::editor::shooting::terminology::ShootingLabels;
use semio_framework_plugin::{
    world3d_mesh_id_from_url, world3d_meshes_json_from_kinds_and_urls, world3d_scene, world3d_selection_json, LocalizedLabel, SurfaceKind, WindowEngagement, WindowEngagementInput, WindowEngagementPossible,
    WindowEngagementStatus, WindowKindDefinition, WindowMeasure, WindowOptions, World3dScene, WorldSunConfig,
};
use dsl::json;
use dsl::os_pack::json::{parse, Value};
use std::collections::HashSet;

fn vec3(v: [f64; 3]) -> Value {
    Value::from(v.iter().map(|c| Value::from(*c)).collect::<Vec<Value>>())
}

fn vec4(v: [f64; 4]) -> Value {
    Value::from(v.iter().map(|c| Value::from(*c)).collect::<Vec<Value>>())
}

//#region 🔖️Constants
pub const SHOOTING_PLAY_WINDOW_SCENE: &str = "shooting-scene";
pub const SHOOTING_PLAY_BODY_SCENE: &str = "shooting.play.scene";
const SHOOTING_PLAY_SURFACE_SCENE: &str = "shooting.play.scene";
const SHOOTING_FALLBACK_MESH_KIND: &str = "box";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::shooting::create_shooting_app`. `options.measures`
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
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}

/// 🎚️ The live chrome measures for this window, collected from its `☑️options/*` components.
pub fn window_measures(snapshot: &ShootingSnapshot, labels: &ShootingLabels) -> Vec<WindowMeasure> {
    vec![
        options::center_model::measure(labels),
        options::sun_enabled::measure(snapshot, labels),
        options::sun_azimuth::measure(snapshot, labels),
        options::sun_elevation::measure(snapshot, labels),
        options::sun_intensity::measure(snapshot, labels),
        options::ambient::measure(snapshot, labels),
        options::shadow::measure(snapshot, labels),
        options::roughness::measure(snapshot, labels),
    ]
}

pub fn engagement(snapshot: &ShootingSnapshot, config: &ShootingConfig, labels: &ShootingLabels) -> WindowEngagement {
    WindowEngagement {
        session_active: Some(true),
        options: None,
        input: Some(WindowEngagementInput {
            id: Some("shooting.camera-draft".into()),
            value: Some(config.camera_draft_label.clone()),
            placeholder: Some(labels.camera_label_placeholder.into()),
            disabled: None,
            on_change: Some(crate::editor::shooting::shooting_window_action("setCameraDraftLabel", None)),
            on_submit: Some(crate::editor::shooting::shooting_window_action("saveCamera", None)),
            on_repeat_last: None,
            on_abort: None,
        }),
        control: None,
        controls: None,
        status: Some(vec![WindowEngagementStatus { id: "shooting.status.model".into(), text: format!("{} assets · {} shots", snapshot.assets.len(), snapshot.shots.len()) }]),
        possible_engagements: Some(
            snapshot
                .saved_cameras
                .iter()
                .map(|saved| WindowEngagementPossible {
                    id: format!("shooting.camera.{}", saved.id),
                    label: saved.label.clone(),
                    detail: Some(labels.load_camera.into()),
                    action: Some(crate::editor::shooting::shooting_window_action(
                        "loadSavedCamera",
                        Some(dsl::DslValue::object([("id".into(), dsl::DslValue::String(saved.id.clone()))])),
                    )),
                })
                .collect(),
        ),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn camera_json(camera: &crate::ShootingCamera) -> String {
    let mut value = json!({
        "position": vec3(camera.position),
        "target": vec3(camera.target),
        "fov": camera.fov,
        "zoom": camera.zoom,
        "projection": camera.projection.clone().unwrap_or_else(|| "perspective".into()),
    });
    if let (Some(object), Some(up)) = (value.as_object_mut(), camera.up) {
        object.insert("up", vec3(up));
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

fn collect_mesh_urls(snapshot: &ShootingSnapshot) -> Vec<String> {
    let mut urls = HashSet::new();
    for asset in &snapshot.assets {
        if let Some(url) = resolve_asset_mesh_url(asset) {
            urls.insert(url);
        }
    }
    urls.into_iter().collect()
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: asset selection AND hover are the
/// framework-owned `"assets"` interaction domain now, unreachable at this render boundary
/// (`ArtifactApp::render` has no `InteractionView` parameter, unlike `handle`/`copy_fragment`/
/// `cut_operations`) — `selected` only reflects `active_asset_id` (a real document field) and `hovered`
/// is always `false`. Documented reduced-fidelity gap, matching this wave's other apps (e.g. `cad`'s
/// `instance_is_component_hovered`/`gumball_active`).
fn world_instances_json(snapshot: &ShootingSnapshot) -> String {
    let instances: Vec<Value> = snapshot
        .assets
        .iter()
        .map(|asset| {
            let active = snapshot.active_asset_id == asset.id || (snapshot.active_asset_id.is_empty() && snapshot.assets.first().map(|entry| &entry.id) == Some(&asset.id));
            let selected = active;
            let hovered = false;
            let mesh_id = resolve_asset_mesh_url(asset).map_or_else(|| SHOOTING_FALLBACK_MESH_KIND.into(), |url| world3d_mesh_id_from_url(&url));
            json!({
                "id": asset.id.as_str(),
                "meshId": mesh_id,
                "position": vec3([
                    asset.origin.first().copied().unwrap_or(0.0),
                    asset.origin.get(1).copied().unwrap_or(0.0),
                    asset.origin.get(2).copied().unwrap_or(0.0),
                ]),
                "rotation": vec4(asset.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0])),
                "scale": vec3(shooting_asset_scale(asset)),
                "label": asset.name.as_str(),
                "color": if selected { "#9aa0ab" } else { "#6b7280" },
                "selected": selected,
                "hovered": hovered,
            })
        })
        .collect();
    Value::from(instances).to_string()
}

fn world_meshes_json(snapshot: &ShootingSnapshot) -> String {
    world3d_meshes_json_from_kinds_and_urls(&[SHOOTING_FALLBACK_MESH_KIND.into()], &collect_mesh_urls(snapshot))
}

/// 🕹️ Same render-has-no-`InteractionView` gap as `world_instances_json` above: `gumballActive` is
/// always `false` and no `gumballTarget` is emitted, since the current asset selection (and thus
/// whether the gumball should show, and where) is unreachable here. The world-3d client dispatches
/// `interactionSelect`/`interactionHover` directly against the `"assets"` domain declared on this
/// window kind (client-side hit-testing against the mesh instance ids already in this payload) — it no
/// longer needs `selectionMethod`/`selectionMode`/`targets` from this payload either.
fn world_selection_json(snapshot: &ShootingSnapshot, active_utility: &str) -> String {
    let mut value: Value = parse(&world3d_selection_json("pick", &[], None)).unwrap_or_else(|_| json!({}));
    if let Some(object) = value.as_object_mut() {
        object.insert("transformMode", json!(active_utility));
        object.insert("activeObjectId", json!(snapshot.active_asset_id.as_str()));
        object.insert("gumballActive", json!(false));
    }
    value.to_string()
}

fn shooting_environment_json(snapshot: &ShootingSnapshot) -> String {
    let scene = &snapshot.scene;
    let mut value = json!({
        "ambient": { "intensity": scene.ambient.intensity, "color": scene.ambient.color.as_str() },
        "sun": { "enabled": scene.sun.enabled, "azimuth": scene.sun.azimuth, "elevation": scene.sun.elevation, "intensity": scene.sun.intensity, "color": scene.sun.color.as_str() },
        "shadow": { "enabled": scene.shadow.enabled, "opacity": scene.shadow.opacity, "softness": scene.shadow.softness },
        "material": { "color": scene.material.color.as_str(), "metalness": scene.material.metalness, "roughness": scene.material.roughness, "emissive": scene.material.emissive.as_str(), "emissiveIntensity": scene.material.emissive_intensity },
    });
    if let Some(object) = value.as_object_mut() {
        if !is_transparent_shooting_background(&scene.background) {
            object.insert("background", json!(scene.background.as_str()));
        }
    }
    value.to_string()
}

fn shooting_frame_json(shot: &ShootingShot) -> String {
    json!({ "width": shot.width, "height": shot.height, "shape": shot.shape.as_str(), "badge": true }).to_string()
}

fn shooting_fit_json(cfg: &ShootingConfig) -> String {
    json!({ "enabled": cfg.center_model, "revision": cfg.fit_revision, "padding": 1.25 }).to_string()
}

pub fn render(snapshot: &ShootingSnapshot, cfg: &ShootingConfig, active_utility: &str) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    semio_framework_plugin::scene_surface(
        SHOOTING_PLAY_SURFACE_SCENE,
        semio_framework_ui_contract::SurfaceKind::World3d,
        &World3dScene {
            environment_json: Some(shooting_environment_json(snapshot)),
            frame_json: crate::standards::v1::subsets::any::schema::active_shot(snapshot).map(shooting_frame_json),
            fit_json: Some(shooting_fit_json(cfg)),
            ..world3d_scene(camera_json(&cfg.camera), world_meshes_json(snapshot), world_instances_json(snapshot), world_selection_json(snapshot, active_utility), &WorldSunConfig::default())
        },
    )
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
