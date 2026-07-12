//! 📸 Shooting plugin — icon studio with scene + preview windows bundled as a hot-swappable WASM component.

use semio_framework_plugin::{SurfaceKind, PanelGroup,
    build_icon_render_scene, build_world_3d_scene, create_default_layout, merge_world_selection_ids,
    ui_inspector_groups_to_tree, ui_inspector_mixed_number, ui_inspector_readonly_field, ui_stack_vertical,
    ui_text, world3d_mesh_id_from_url, world3d_meshes_json_from_kinds_and_urls, world3d_scene,
    world3d_selection_json, App, ActionDescriptor, IconRenderScene, PluginApp, PluginBundle,
    ToolCategory, ToolNode, UiFieldNode, UiInspectorFieldGroup, UiNode, UiTreeItemNode, UiTreeNode,
    UiTreeSectionNode, ViewState, WindowEngagement, WindowEngagementInput, WindowEngagementOption, WindowMeasure,
    World3dScene, WorldSunConfig,
    FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
    FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use semio_framework_plugin::layout::{MeasureSelectItem, WindowEngagementPossible, WindowEngagementStatus};
use semio_framework_core::DwgDrawing;
use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::atomic::{AtomicU32, Ordering};

//#region 🔖Constants
const SHOOTING_PLAY_APP_ID: &str = "shooting-play";
const SHOOTING_PLAY_CONTROLLER_ID: &str = "shooting-play";
const SHOOTING_PLAY_SURFACE_SCENE: &str = "shooting.play.scene";
const SHOOTING_PLAY_SURFACE_ICON: &str = "shooting.play.icon";
const SHOOTING_PLAY_BODY_SCENE: &str = "shooting.play.scene";
const SHOOTING_PLAY_BODY_ICON: &str = "shooting.play.icon";
const SHOOTING_PLAY_BODY_DOCUMENT: &str = "shooting.play.document";
const SHOOTING_PLAY_BODY_CATALOGUE: &str = "shooting.play.catalogue";
const SHOOTING_PLAY_BODY_INSPECTION: &str = "shooting.play.inspection";
const SHOOTING_PLAY_WINDOW_SCENE: &str = "shooting-scene";
const SHOOTING_PLAY_WINDOW_ICON: &str = "shooting-icon";
const SHOOTING_FIXTURE_SCHEMA: &str = "shooting.fixture";
const SHOOTING_EXAMPLE_DEFAULT_ID: &str = "base-icon";

const SHOOTING_FALLBACK_MESH_KIND: &str = "box";

const DEFAULT_EXAMPLE_JSON: &str = include_str!("../../example/base-icon.shooting.json");

static SHOOTING_ID_COUNTER: AtomicU32 = AtomicU32::new(0);
//#endregion 🔖Constants

//#region 🔖Document
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ShootingCamera {
    #[serde(default = "default_camera_position")]
    position: [f64; 3],
    #[serde(default = "default_camera_target")]
    target: [f64; 3],
    #[serde(default = "one_f64")]
    zoom: f64,
    #[serde(default = "default_fov")]
    fov: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    up: Option<[f64; 3]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    projection: Option<String>,
}

impl Default for ShootingCamera {
    fn default() -> Self {
        Self {
            position: default_camera_position(),
            target: default_camera_target(),
            zoom: 1.0,
            fov: default_fov(),
            up: None,
            projection: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ShootingSavedCamera {
    id: String,
    label: String,
    camera: ShootingCamera,
}

fn default_camera_position() -> [f64; 3] {
    [420.0, -420.0, 320.0]
}

fn default_camera_target() -> [f64; 3] {
    [0.0, 0.0, 40.0]
}

fn default_fov() -> f64 {
    50.0
}

fn one_f64() -> f64 {
    1.0
}

fn default_selection_method() -> String {
    "rectangle".into()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ShootingAsset {
    id: String,
    name: String,
    url: String,
    #[serde(default = "default_glb_format")]
    format: String,
    #[serde(default)]
    origin: [f64; 3],
    #[serde(default)]
    orientation: Option<[f64; 4]>,
    #[serde(default)]
    scale: Option<Value>,
}

fn default_glb_format() -> String {
    "glb".into()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ShootingShot {
    id: String,
    label: String,
    width: u32,
    height: u32,
    format: String,
    shape: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    background: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    camera_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
struct ShootingSun {
    enabled: bool,
    azimuth: f64,
    elevation: f64,
    intensity: f64,
    color: String,
}

impl Default for ShootingSun {
    fn default() -> Self {
        Self { enabled: false, azimuth: 45.0, elevation: 35.0, intensity: 2.4, color: "#ffffff".into() }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
struct ShootingAmbient {
    intensity: f64,
    color: String,
}

impl Default for ShootingAmbient {
    fn default() -> Self {
        Self { intensity: 1.15, color: "#ffffff".into() }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
struct ShootingShadow {
    enabled: bool,
    opacity: f64,
    softness: f64,
}

impl Default for ShootingShadow {
    fn default() -> Self {
        Self { enabled: true, opacity: 0.35, softness: 1.0 }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
struct ShootingMaterial {
    color: String,
    metalness: f64,
    roughness: f64,
    emissive: String,
    emissive_intensity: f64,
}

impl Default for ShootingMaterial {
    fn default() -> Self {
        Self { color: "#9aa0ab".into(), metalness: 0.0, roughness: 1.0, emissive: "#000000".into(), emissive_intensity: 0.0 }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct ShootingSceneLighting {
    #[serde(default)]
    background: String,
    #[serde(default)]
    sun: ShootingSun,
    #[serde(default)]
    ambient: ShootingAmbient,
    #[serde(default)]
    shadow: ShootingShadow,
    #[serde(default)]
    material: ShootingMaterial,
    #[serde(default, rename = "emblemBase64")]
    emblem_base64: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ShootingFixture {
    schema: String,
    #[serde(default)]
    assets: Vec<ShootingAsset>,
    #[serde(default)]
    camera: ShootingCamera,
    #[serde(default)]
    saved_cameras: Vec<ShootingSavedCamera>,
    #[serde(default)]
    scene: ShootingSceneLighting,
    #[serde(default)]
    shots: Vec<ShootingShot>,
    #[serde(default)]
    active_shot_id: String,
    #[serde(default)]
    active_asset_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
struct ShootingPlayRuntime {
    selected_shot_ids: Vec<String>,
    selected_asset_ids: Vec<String>,
    selection_method: String,
    hovered_asset_id: Option<String>,
    center_model: bool,
    fit_revision: u32,
    camera_draft_label: String,
    transform_tool: String,
}

impl Default for ShootingPlayRuntime {
    fn default() -> Self {
        Self {
            selected_shot_ids: Vec::new(),
            selected_asset_ids: Vec::new(),
            selection_method: default_selection_method(),
            hovered_asset_id: None,
            center_model: true,
            fit_revision: 0,
            camera_draft_label: String::new(),
            transform_tool: "move".into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ShootingPlayEnvelope {
    fixture: ShootingFixture,
    #[serde(default)]
    runtime: ShootingPlayRuntime,
}

fn empty_shooting_fixture() -> ShootingFixture {
    ShootingFixture {
        schema: SHOOTING_FIXTURE_SCHEMA.into(),
        assets: Vec::new(),
        camera: ShootingCamera::default(),
        saved_cameras: Vec::new(),
        scene: ShootingSceneLighting::default(),
        shots: Vec::new(),
        active_shot_id: String::new(),
        active_asset_id: String::new(),
    }
}

fn default_envelope() -> ShootingPlayEnvelope {
    serde_json::from_str::<ShootingFixture>(DEFAULT_EXAMPLE_JSON)
        .map(|fixture| ShootingPlayEnvelope {
            fixture,
            runtime: ShootingPlayRuntime::default(),
        })
        .unwrap_or_else(|_| ShootingPlayEnvelope {
            fixture: empty_shooting_fixture(),
            runtime: ShootingPlayRuntime::default(),
        })
}

fn parse_envelope(document_json: &str) -> ShootingPlayEnvelope {
    serde_json::from_str(document_json).unwrap_or_else(|_| default_envelope())
}

fn next_shooting_id(prefix: &str) -> String {
    let next = SHOOTING_ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
    format!("{prefix}-{next}")
}
//#endregion 🔖Document

//#region 🔖FixtureOps
#[derive(Clone, Debug)]
enum ShootingFixtureEditOp {
    SetActiveShot { shot_id: String },
    SetActiveAsset { asset_id: String },
    SetCamera { camera: ShootingCamera },
    AddShot { shot: ShootingShot },
    AddAsset { asset: ShootingAsset },
    RemoveShot { shot_id: String },
    RemoveAsset { asset_id: String },
    PatchShots { shot_ids: Vec<String>, field: String, value: Value },
    PatchAssets { asset_ids: Vec<String>, field: String, value: Value },
}

fn apply_fixture_edit(fixture: &ShootingFixture, op: &ShootingFixtureEditOp) -> ShootingFixture {
    let mut next = fixture.clone();
    match op {
        ShootingFixtureEditOp::SetActiveShot { shot_id } => next.active_shot_id = shot_id.clone(),
        ShootingFixtureEditOp::SetActiveAsset { asset_id } => next.active_asset_id = asset_id.clone(),
        ShootingFixtureEditOp::SetCamera { camera } => next.camera = camera.clone(),
        ShootingFixtureEditOp::AddShot { shot } => next.shots.push(shot.clone()),
        ShootingFixtureEditOp::AddAsset { asset } => next.assets.push(asset.clone()),
        ShootingFixtureEditOp::RemoveShot { shot_id } => {
            next.shots.retain(|shot| shot.id != *shot_id);
            if next.active_shot_id == *shot_id {
                next.active_shot_id = next.shots.first().map(|shot| shot.id.clone()).unwrap_or_default();
            }
        }
        ShootingFixtureEditOp::RemoveAsset { asset_id } => {
            next.assets.retain(|asset| asset.id != *asset_id);
            if next.active_asset_id == *asset_id {
                next.active_asset_id = next.assets.first().map(|asset| asset.id.clone()).unwrap_or_default();
            }
        }
        ShootingFixtureEditOp::PatchShots { shot_ids, field, value } => {
            for shot in &mut next.shots {
                if !shot_ids.contains(&shot.id) {
                    continue;
                }
                match field.as_str() {
                    "label" => {
                        if let Some(label) = value.as_str() {
                            shot.label = label.into();
                        }
                    }
                    "width" => {
                        if let Some(width) = value.as_u64() {
                            shot.width = width as u32;
                        }
                    }
                    "height" => {
                        if let Some(height) = value.as_u64() {
                            shot.height = height as u32;
                        }
                    }
                    "format" => {
                        if let Some(format) = value.as_str() {
                            shot.format = format.into();
                        }
                    }
                    "shape" => {
                        if let Some(shape) = value.as_str() {
                            shot.shape = shape.into();
                        }
                    }
                    _ => {}
                }
            }
        }
        ShootingFixtureEditOp::PatchAssets { asset_ids, field, value } => {
            for asset in &mut next.assets {
                if !asset_ids.contains(&asset.id) {
                    continue;
                }
                match field.as_str() {
                    "name" => {
                        if let Some(name) = value.as_str() {
                            asset.name = name.into();
                        }
                    }
                    "url" => {
                        if let Some(url) = value.as_str() {
                            asset.url = url.into();
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    next
}

fn set_document_op(envelope: &ShootingPlayEnvelope) -> String {
    json!({ "op": "setDocument", "document": envelope }).to_string()
}

fn shooting_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor {
        controller_id: SHOOTING_PLAY_CONTROLLER_ID.into(),
        action: action.into(),
        args,
    }
}

fn active_shot<'a>(fixture: &'a ShootingFixture) -> Option<&'a ShootingShot> {
    fixture
        .shots
        .iter()
        .find(|shot| shot.id == fixture.active_shot_id)
        .or_else(|| fixture.shots.first())
}

fn active_asset<'a>(fixture: &'a ShootingFixture) -> Option<&'a ShootingAsset> {
    fixture
        .assets
        .iter()
        .find(|asset| asset.id == fixture.active_asset_id)
        .or_else(|| fixture.assets.first())
}

fn camera_json(camera: &ShootingCamera) -> String {
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

fn resolve_shot_camera(fixture: &ShootingFixture, shot: &ShootingShot) -> ShootingCamera {
    shot.camera_id
        .as_ref()
        .and_then(|camera_id| fixture.saved_cameras.iter().find(|entry| &entry.id == camera_id))
        .map(|entry| entry.camera.clone())
        .unwrap_or_else(|| fixture.camera.clone())
}

fn apply_camera_for_shot(fixture: &mut ShootingFixture, shot_id: Option<&str>, camera: ShootingCamera) {
    let camera_id = shot_id
        .and_then(|id| fixture.shots.iter().find(|shot| shot.id == id))
        .and_then(|shot| shot.camera_id.clone());
    if let Some(camera_id) = camera_id {
        if let Some(saved) = fixture.saved_cameras.iter_mut().find(|entry| entry.id == camera_id) {
            saved.camera = camera;
            return;
        }
    }
    fixture.camera = camera;
}

fn mesh_selection_ids(args: Option<&Value>, fallback: &[String]) -> Vec<String> {
    args.and_then(|value| value.get("ids"))
        .and_then(|value| serde_json::from_value(value.clone()).ok())
        .filter(|ids: &Vec<String>| !ids.is_empty())
        .unwrap_or_else(|| fallback.to_vec())
}

fn quat_mul(a: [f64; 4], b: [f64; 4]) -> [f64; 4] {
    [
        a[3] * b[0] + a[0] * b[3] + a[1] * b[2] - a[2] * b[1],
        a[3] * b[1] - a[0] * b[2] + a[1] * b[3] + a[2] * b[0],
        a[3] * b[2] + a[0] * b[1] - a[1] * b[0] + a[2] * b[3],
        a[3] * b[3] - a[0] * b[0] - a[1] * b[1] - a[2] * b[2],
    ]
}

fn quat_from_axis_angle(ax: f64, ay: f64, az: f64, angle: f64) -> [f64; 4] {
    let len = (ax * ax + ay * ay + az * az).sqrt();
    if len < 1e-8 {
        return [0.0, 0.0, 0.0, 1.0];
    }
    let half = angle * 0.5;
    let s = half.sin();
    [ax / len * s, ay / len * s, az / len * s, half.cos()]
}

fn scale_vec_mul(scale: [f64; 3], sx: f64, sy: f64, sz: f64) -> [f64; 3] {
    [scale[0] * sx, scale[1] * sy, scale[2] * sz]
}

fn asset_scale_json(asset: &ShootingAsset) -> [f64; 3] {
    match &asset.scale {
        Some(Value::Array(values)) if values.len() >= 3 => [
            values[0].as_f64().unwrap_or(1.0),
            values[1].as_f64().unwrap_or(1.0),
            values[2].as_f64().unwrap_or(1.0),
        ],
        Some(Value::Number(value)) => {
            let scale = value.as_f64().unwrap_or(1.0);
            [scale, scale, scale]
        }
        _ => [1.0, 1.0, 1.0],
    }
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

fn world_instances_json(fixture: &ShootingFixture, runtime: &ShootingPlayRuntime) -> String {
    let instances: Vec<Value> = fixture
        .assets
        .iter()
        .map(|asset| {
            let active = fixture.active_asset_id == asset.id
                || (fixture.active_asset_id.is_empty() && fixture.assets.first().map(|entry| &entry.id) == Some(&asset.id));
            let selected = runtime.selected_asset_ids.contains(&asset.id) || active;
            let hovered = runtime.hovered_asset_id.as_deref() == Some(asset.id.as_str());
            let mesh_id = resolve_asset_mesh_url(asset)
                .map(|url| world3d_mesh_id_from_url(&url))
                .unwrap_or_else(|| SHOOTING_FALLBACK_MESH_KIND.into());
            json!({
                "id": asset.id,
                "meshId": mesh_id,
                "position": [
                    asset.origin.first().copied().unwrap_or(0.0),
                    asset.origin.get(1).copied().unwrap_or(0.0),
                    asset.origin.get(2).copied().unwrap_or(0.0),
                ],
                "rotation": asset.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]),
                "scale": asset_scale_json(asset),
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

fn world_selection_json(fixture: &ShootingFixture, runtime: &ShootingPlayRuntime) -> String {
    let mut value: Value = serde_json::from_str(&world3d_selection_json(
        &runtime.selection_method,
        &runtime.selected_asset_ids,
        runtime.hovered_asset_id.as_deref(),
    ))
    .unwrap_or_else(|_| json!({}));
    if let Some(object) = value.as_object_mut() {
        object.insert("granularity".into(), json!("mesh"));
        object.insert("selectionMode".into(), json!("mesh"));
        object.insert("targets".into(), json!({ "mesh": true, "vertex": false, "edge": false, "face": false }));
        object.insert("transformTool".into(), json!(runtime.transform_tool));
        object.insert("activeObjectId".into(), json!(fixture.active_asset_id));
        object.insert("gumballActive".into(), json!(!runtime.selected_asset_ids.is_empty()));
        if let Some(target) = selection_centroid(fixture, &runtime.selected_asset_ids) {
            object.insert("gumballTarget".into(), json!(target));
        }
    }
    value.to_string()
}

fn selection_centroid(fixture: &ShootingFixture, selected_ids: &[String]) -> Option<[f64; 3]> {
    let selected: Vec<&ShootingAsset> = fixture.assets.iter().filter(|asset| selected_ids.contains(&asset.id)).collect();
    if selected.is_empty() {
        return None;
    }
    let count = selected.len() as f64;
    let sum = selected.iter().fold([0.0f64; 3], |acc, asset| {
        [acc[0] + asset.origin[0], acc[1] + asset.origin[1], acc[2] + asset.origin[2]]
    });
    Some([sum[0] / count, sum[1] / count, sum[2] / count])
}

fn is_transparent_shooting_background(background: &str) -> bool {
    background.is_empty() || background == "transparent"
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

fn shooting_fit_json(runtime: &ShootingPlayRuntime) -> String {
    json!({ "enabled": runtime.center_model, "revision": runtime.fit_revision, "padding": 1.25 }).to_string()
}

fn shooting_icon_render_request_json(fixture: &ShootingFixture, shot: &ShootingShot, asset: &ShootingAsset) -> String {
    let camera = resolve_shot_camera(fixture, shot);
    let scene = &fixture.scene;
    let mut camera_value = json!({
        "position": camera.position,
        "target": camera.target,
        "zoom": camera.zoom,
        "fov": camera.fov,
    });
    if let (Some(object), Some(up)) = (camera_value.as_object_mut(), camera.up) {
        object.insert("up".into(), json!(up));
    }
    let mut value = json!({
        "assetUrl": asset.url,
        "camera": camera_value,
        "lights": {
            "ambientIntensity": scene.ambient.intensity,
            "ambientColor": scene.ambient.color,
            "sunAzimuth": scene.sun.azimuth,
            "sunElevation": scene.sun.elevation,
            "sunIntensity": scene.sun.intensity,
            "sunColor": scene.sun.color,
        },
        "width": shot.width,
        "height": shot.height,
        "format": shot.format,
        "shape": if shot.shape == "ellipse" { "ellipse" } else { "rectangle" },
        "shadowEnabled": scene.shadow.enabled,
        "material": {
            "color": scene.material.color,
            "metalness": scene.material.metalness,
            "roughness": scene.material.roughness,
            "emissive": scene.material.emissive,
            "emissiveIntensity": scene.material.emissive_intensity,
        },
    });
    if let Some(object) = value.as_object_mut() {
        let background = shot.background.clone().unwrap_or_else(|| scene.background.clone());
        if !is_transparent_shooting_background(&background) {
            object.insert("background".into(), json!(background));
        }
    }
    value.to_string()
}
//#endregion 🔖FixtureOps

//#region 🔖Terminology
/// 🗣️ Complete UI label set for the shooting app; one field per label makes every locale combination compile-checked.
struct ShootingLabels {
    shots: &'static str,
    assets: &'static str,
    add_shot: &'static str,
    add_asset: &'static str,
    svg_rectangle: &'static str,
    png_rectangle: &'static str,
    svg_ellipse: &'static str,
    png_ellipse: &'static str,
    glb_asset: &'static str,
    shot: &'static str,
    asset: &'static str,
    open: &'static str,
    import_title: &'static str,
    save: &'static str,
    export_title: &'static str,
    move_tool: &'static str,
    rotate_tool: &'static str,
    scale_tool: &'static str,
    camera_label_placeholder: &'static str,
    load_camera: &'static str,
    shot_label_placeholder: &'static str,
    no_shot: &'static str,
    format_select_label: &'static str,
    shape_select_label: &'static str,
    format_svg: &'static str,
    format_png: &'static str,
    shape_rectangle: &'static str,
    shape_ellipse: &'static str,
    window_scene: &'static str,
    window_icon: &'static str,
}

const SHOOTING_LABELS_NATIVE_EN: ShootingLabels = ShootingLabels {
    shots: "Shots",
    assets: "Assets",
    add_shot: "Add Shot",
    add_asset: "Add Asset",
    svg_rectangle: "SVG Rectangle",
    png_rectangle: "PNG Rectangle",
    svg_ellipse: "SVG Ellipse",
    png_ellipse: "PNG Ellipse",
    glb_asset: "GLB Asset",
    shot: "Shot",
    asset: "Asset",
    open: "Open",
    import_title: "Import",
    save: "Save",
    export_title: "Export",
    move_tool: "Move",
    rotate_tool: "Rotate",
    scale_tool: "Scale",
    camera_label_placeholder: "Camera label",
    load_camera: "Load camera",
    shot_label_placeholder: "Shot label",
    no_shot: "No shot",
    format_select_label: "Format",
    shape_select_label: "Shape",
    format_svg: "SVG",
    format_png: "PNG",
    shape_rectangle: "Rectangle",
    shape_ellipse: "Ellipse",
    window_scene: "Scene",
    window_icon: "Icon",
};

const SHOOTING_LABELS_NATIVE_DE: ShootingLabels = ShootingLabels {
    shots: "Aufnahmen",
    assets: "Objekte",
    add_shot: "Aufnahme hinzufügen",
    add_asset: "Objekt hinzufügen",
    svg_rectangle: "SVG Rechteck",
    png_rectangle: "PNG Rechteck",
    svg_ellipse: "SVG Ellipse",
    png_ellipse: "PNG Ellipse",
    glb_asset: "GLB-Objekt",
    shot: "Aufnahme",
    asset: "Objekt",
    open: "Öffnen",
    import_title: "Importieren",
    save: "Speichern",
    export_title: "Exportieren",
    move_tool: "Verschieben",
    rotate_tool: "Drehen",
    scale_tool: "Skalieren",
    camera_label_placeholder: "Kamera-Bezeichnung",
    load_camera: "Kamera laden",
    shot_label_placeholder: "Aufnahme-Bezeichnung",
    no_shot: "Keine Aufnahme",
    format_select_label: "Format",
    shape_select_label: "Form",
    format_svg: "SVG",
    format_png: "PNG",
    shape_rectangle: "Rechteck",
    shape_ellipse: "Ellipse",
    window_scene: "Szene",
    window_icon: "Symbol",
};

/// 🗣️ Resolves the active label set from the shell-provided locale; no terminology variant exists for this app.
fn shooting_labels(view_state: &ViewState) -> &'static ShootingLabels {
    let is_de = view_state.locale.as_deref().is_some_and(|locale| locale.starts_with("de"));
    if is_de { &SHOOTING_LABELS_NATIVE_DE } else { &SHOOTING_LABELS_NATIVE_EN }
}
//#endregion 🔖Terminology

//#region 🔖Panels
fn tree_item(id: impl Into<String>, label: impl Into<String>) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.into(),
        label: label.into(),
        description: None,
        icon_id: None,
        selected: None,
        default_open: None,
        action: None,
        hover_action: None,
        unhover_action: None,
        actions: None,
        draggable: None,
        drag_data: None,
        items: None,
        control: None,
        is_hidden: None,
    }
}

fn tree_item_with_action(
    id: impl Into<String>,
    label: impl Into<String>,
    icon_id: Option<&str>,
    action: ActionDescriptor,
) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.into(),
        label: label.into(),
        description: None,
        icon_id: icon_id.map(str::to_string),
        selected: None,
        default_open: None,
        action: Some(action),
        hover_action: None,
        unhover_action: None,
        actions: None,
        draggable: None,
        drag_data: None,
        items: None,
        control: None,
        is_hidden: None,
    }
}

fn build_document_tree(envelope: &ShootingPlayEnvelope, labels: &ShootingLabels) -> UiNode {
    let fixture = &envelope.fixture;
    let shot_items: Vec<UiTreeItemNode> = fixture
        .shots
        .iter()
        .map(|shot| {
            tree_item_with_action(
                format!("shooting-shot:{}", shot.id),
                shot.label.clone(),
                Some("camera"),
                shooting_action("setSelection", Some(json!({ "shotIds": [shot.id], "assetIds": [] }))),
            )
        })
        .collect();
    let asset_items: Vec<UiTreeItemNode> = fixture
        .assets
        .iter()
        .map(|asset| {
            tree_item_with_action(
                format!("shooting-asset:{}", asset.id),
                asset.name.clone(),
                Some("box"),
                shooting_action("setSelection", Some(json!({ "shotIds": [], "assetIds": [asset.id] }))),
            )
        })
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![
            UiTreeSectionNode {
                id: "shooting-play-document.shots".into(),
                label: Some(labels.shots.into()),
                default_open: Some(true),
                items: shot_items,
            },
            UiTreeSectionNode {
                id: "shooting-play-document.assets".into(),
                label: Some(labels.assets.into()),
                default_open: Some(true),
                items: asset_items,
            },
        ],
        selected_ids: None,
        highlighted_ids: None,
        selection_change: None,
        drop_action: None,
    })
}

fn build_catalogue_tree(labels: &ShootingLabels) -> UiNode {
    UiNode::Tree(UiTreeNode {
        sections: vec![
            UiTreeSectionNode {
                id: "shooting-play-catalogue.shots".into(),
                label: Some(labels.add_shot.into()),
                default_open: Some(true),
                items: vec![
                    catalog_shot_item("svg-rect", labels.svg_rectangle, "svg", "rectangle"),
                    catalog_shot_item("png-rect", labels.png_rectangle, "png", "rectangle"),
                    catalog_shot_item("svg-ellipse", labels.svg_ellipse, "svg", "ellipse"),
                    catalog_shot_item("png-ellipse", labels.png_ellipse, "png", "ellipse"),
                ],
            },
            UiTreeSectionNode {
                id: "shooting-play-catalogue.assets".into(),
                label: Some(labels.add_asset.into()),
                default_open: Some(true),
                items: vec![tree_item_with_action(
                    "shooting-play-catalogue.asset.glb",
                    labels.glb_asset,
                    Some("box"),
                    shooting_action("addAsset", Some(json!({ "format": "glb" }))),
                )],
            },
        ],
        selected_ids: None,
        highlighted_ids: None,
        selection_change: None,
        drop_action: None,
    })
}

fn catalog_shot_item(id: &str, label: &str, format: &str, shape: &str) -> UiTreeItemNode {
    tree_item_with_action(
        format!("shooting-play-catalogue.{id}"),
        label,
        Some("camera"),
        shooting_action("addShot", Some(json!({ "format": format, "shape": shape }))),
    )
}

fn build_inspector_tree(envelope: &ShootingPlayEnvelope, labels: &ShootingLabels) -> UiNode {
    let fixture = &envelope.fixture;
    if !envelope.runtime.selected_shot_ids.is_empty() {
        let shot_id = &envelope.runtime.selected_shot_ids[0];
        if let Some(shot) = fixture.shots.iter().find(|entry| &entry.id == shot_id) {
            return ui_inspector_groups_to_tree(&[shot_inspector_group(shot, labels)]);
        }
    }
    if !envelope.runtime.selected_asset_ids.is_empty() {
        let asset_id = &envelope.runtime.selected_asset_ids[0];
        if let Some(asset) = fixture.assets.iter().find(|entry| &entry.id == asset_id) {
            return ui_inspector_groups_to_tree(&[asset_inspector_group(asset, labels)]);
        }
    }
    if let Some(shot) = active_shot(fixture) {
        return ui_inspector_groups_to_tree(&[shot_inspector_group(shot, labels)]);
    }
    ui_stack_vertical(vec![
        ui_text(format!("Schema: {SHOOTING_FIXTURE_SCHEMA}")),
        ui_text(format!("Shots: {}", fixture.shots.len())),
        ui_text(format!("Assets: {}", fixture.assets.len())),
    ])
}

fn shot_inspector_group(shot: &ShootingShot, labels: &ShootingLabels) -> UiInspectorFieldGroup {
    let width_mixed = ui_inspector_mixed_number(&[shot.width as f64]);
    let height_mixed = ui_inspector_mixed_number(&[shot.height as f64]);
    UiInspectorFieldGroup {
        id: "shooting-play-inspector.shot".into(),
        label: labels.shot.into(),
        default_open: None,
        fields: vec![
            UiNode::Field(UiFieldNode {
                id: "shooting-play-inspector.shot.label".into(),
                label: "Label".into(),
                child: Box::new(UiNode::Input(semio_framework_plugin::UiInputNode {
                    id: "shooting-play-inspector.shot.label.input".into(),
                    input_kind: "text".into(),
                    value: shot.label.clone(),
                    placeholder: None,
                    commit: None,
                    on_change: shooting_action(
                        "patchShot",
                        Some(json!({ "shotId": shot.id, "field": "label" })),
                    ),
                    min: None,
                    max: None,
                    step: None,
                    accept: None,
                })),
                description: None,
                required: None,
                error: None,
            }),
            ui_inspector_readonly_field("shooting-play-inspector.shot.format", "Format", &shot.format),
            ui_inspector_readonly_field("shooting-play-inspector.shot.shape", "Shape", &shot.shape),
            UiNode::Field(UiFieldNode {
                id: "shooting-play-inspector.shot.width".into(),
                label: "Width".into(),
                child: Box::new(UiNode::Input(semio_framework_plugin::UiInputNode {
                    id: "shooting-play-inspector.shot.width.input".into(),
                    input_kind: "number".into(),
                    value: width_mixed.value.to_string(),
                    placeholder: None,
                    commit: None,
                    on_change: shooting_action(
                        "patchShot",
                        Some(json!({ "shotId": shot.id, "field": "width" })),
                    ),
                    min: None,
                    max: None,
                    step: None,
                    accept: None,
                })),
                description: None,
                required: None,
                error: None,
            }),
            UiNode::Field(UiFieldNode {
                id: "shooting-play-inspector.shot.height".into(),
                label: "Height".into(),
                child: Box::new(UiNode::Input(semio_framework_plugin::UiInputNode {
                    id: "shooting-play-inspector.shot.height.input".into(),
                    input_kind: "number".into(),
                    value: height_mixed.value.to_string(),
                    placeholder: None,
                    commit: None,
                    on_change: shooting_action(
                        "patchShot",
                        Some(json!({ "shotId": shot.id, "field": "height" })),
                    ),
                    min: None,
                    max: None,
                    step: None,
                    accept: None,
                })),
                description: None,
                required: None,
                error: None,
            }),
        ],
    }
}

fn asset_inspector_group(asset: &ShootingAsset, labels: &ShootingLabels) -> UiInspectorFieldGroup {
    UiInspectorFieldGroup {
        id: "shooting-play-inspector.asset".into(),
        label: labels.asset.into(),
        default_open: None,
        fields: vec![
            UiNode::Field(UiFieldNode {
                id: "shooting-play-inspector.asset.name".into(),
                label: "Name".into(),
                child: Box::new(UiNode::Input(semio_framework_plugin::UiInputNode {
                    id: "shooting-play-inspector.asset.name.input".into(),
                    input_kind: "text".into(),
                    value: asset.name.clone(),
                    placeholder: None,
                    commit: None,
                    on_change: shooting_action(
                        "patchAsset",
                        Some(json!({ "assetId": asset.id, "field": "name" })),
                    ),
                    min: None,
                    max: None,
                    step: None,
                    accept: None,
                })),
                description: None,
                required: None,
                error: None,
            }),
            UiNode::Field(UiFieldNode {
                id: "shooting-play-inspector.asset.url".into(),
                label: "URL".into(),
                child: Box::new(UiNode::Input(semio_framework_plugin::UiInputNode {
                    id: "shooting-play-inspector.asset.url.input".into(),
                    input_kind: "text".into(),
                    value: asset.url.clone(),
                    placeholder: None,
                    commit: None,
                    on_change: shooting_action(
                        "patchAsset",
                        Some(json!({ "assetId": asset.id, "field": "url" })),
                    ),
                    min: None,
                    max: None,
                    step: None,
                    accept: None,
                })),
                description: None,
                required: None,
                error: None,
            }),
            ui_inspector_readonly_field("shooting-play-inspector.asset.format", "Format", &asset.format),
        ],
    }
}
//#endregion 🔖Panels

//#region 🔖Render
fn render_model_scene(fixture: &ShootingFixture, runtime: &ShootingPlayRuntime) -> UiNode {
    build_world_3d_scene(
        SHOOTING_PLAY_SURFACE_SCENE,
        SHOOTING_PLAY_APP_ID,
        World3dScene {
            environment_json: Some(shooting_environment_json(fixture)),
            frame_json: active_shot(fixture).map(shooting_frame_json),
            fit_json: Some(shooting_fit_json(runtime)),
            ..world3d_scene(
                camera_json(&fixture.camera),
                world_meshes_json(fixture),
                world_instances_json(fixture, runtime),
                world_selection_json(fixture, runtime),
                &WorldSunConfig::default(),
            )
        },
    )
}

fn escape_svg_text(value: &str) -> String {
    value.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

/// 🖼️ Renders the active shot as an SVG emblem — shot shape as the clip, the emblem override
/// or asset name as the payload — instead of a generic title card.
fn shooting_scene_svg(fixture: &ShootingFixture) -> (String, u32, u32) {
    let shot = active_shot(fixture);
    let asset = active_asset(fixture);
    let (width, height) = shot.map(|entry| (entry.width, entry.height)).unwrap_or((256, 256));
    let shape = shot.map(|entry| entry.shape.as_str()).unwrap_or("rectangle");
    let background = if fixture.scene.background.is_empty() { "#0f172a" } else { fixture.scene.background.as_str() };
    let clip = if shape == "ellipse" {
        format!(
            "<ellipse cx=\"{cx}\" cy=\"{cy}\" rx=\"{rx}\" ry=\"{ry}\" fill=\"{background}\"/>",
            cx = width as f64 / 2.0,
            cy = height as f64 / 2.0,
            rx = width as f64 / 2.0,
            ry = height as f64 / 2.0,
        )
    } else {
        format!("<rect width=\"100%\" height=\"100%\" fill=\"{background}\"/>")
    };
    let emblem = fixture
        .scene
        .emblem_base64
        .as_ref()
        .filter(|data| !data.is_empty())
        .map(|data| {
            format!(
                "<image href=\"data:image/png;base64,{data}\" x=\"0\" y=\"0\" width=\"{width}\" height=\"{height}\" preserveAspectRatio=\"xMidYMid meet\"/>"
            )
        })
        .unwrap_or_default();
    let label = asset.map(|entry| entry.name.as_str()).unwrap_or("Untitled");
    let font_size = (height as f64 * 0.09).max(10.0);
    let text = format!(
        "<text x=\"50%\" y=\"{y}\" font-size=\"{font_size}\" fill=\"white\" text-anchor=\"middle\" font-family=\"sans-serif\">{label}</text>",
        y = height as f64 * 0.92,
        label = escape_svg_text(label),
    );
    semio_framework_os::wrap_svg(width, height, &format!("{clip}{emblem}{text}"))
}

fn render_icon_scene(fixture: &ShootingFixture) -> UiNode {
    let (request_json, footer) = match (active_shot(fixture), active_asset(fixture)) {
        (Some(shot), Some(asset)) => (
            shooting_icon_render_request_json(fixture, shot, asset),
            Some(format!("{} · {}×{} · {}", shot.label, shot.width, shot.height, shot.format.to_uppercase())),
        ),
        _ => ("null".into(), None),
    };
    build_icon_render_scene(
        SHOOTING_PLAY_SURFACE_ICON,
        SHOOTING_PLAY_APP_ID,
        IconRenderScene {
            request_json,
            footer,
            frame_json: None,
        },
    )
}
//#endregion 🔖Render

//#region 🔖Tools
fn shooting_model_measures(envelope: &ShootingPlayEnvelope) -> Vec<WindowMeasure> {
    let scene = &envelope.fixture.scene;
    vec![
        WindowMeasure::Toggle {
            id: "shooting.measure.center-model".into(),
            icon_id: "focus".into(),
            label: Some("Center Model".into()),
            pressed: envelope.runtime.center_model,
            text: None,
            on_change: shooting_action("setCenterModel", None),
        },
        WindowMeasure::Toggle {
            id: "shooting.measure.sun-enabled".into(),
            icon_id: "sun".into(),
            label: Some("Sun".into()),
            pressed: scene.sun.enabled,
            text: None,
            on_change: shooting_action("toggleSun", None),
        },
        WindowMeasure::Slider {
            id: "shooting.measure.sun-azimuth".into(),
            label: Some("Sun Azimuth".into()),
            value: scene.sun.azimuth,
            min: 0.0,
            max: 360.0,
            step: Some(1.0),
            on_change: shooting_action("setSunAzimuth", None),
        },
        WindowMeasure::Slider {
            id: "shooting.measure.sun-elevation".into(),
            label: Some("Sun Elevation".into()),
            value: scene.sun.elevation,
            min: -10.0,
            max: 90.0,
            step: Some(1.0),
            on_change: shooting_action("setSunElevation", None),
        },
        WindowMeasure::Slider {
            id: "shooting.measure.sun-intensity".into(),
            label: Some("Sun Intensity".into()),
            value: scene.sun.intensity,
            min: 0.0,
            max: 5.0,
            step: Some(0.1),
            on_change: shooting_action("setSunIntensity", None),
        },
        WindowMeasure::Slider {
            id: "shooting.measure.ambient".into(),
            label: Some("Ambient".into()),
            value: scene.ambient.intensity,
            min: 0.0,
            max: 3.0,
            step: Some(0.05),
            on_change: shooting_action("setAmbientIntensity", None),
        },
        WindowMeasure::Toggle {
            id: "shooting.measure.shadow".into(),
            icon_id: "sun".into(),
            label: Some("Shadow".into()),
            pressed: scene.shadow.enabled,
            text: None,
            on_change: shooting_action("setShadowEnabled", None),
        },
        WindowMeasure::Slider {
            id: "shooting.measure.roughness".into(),
            label: Some("Roughness".into()),
            value: scene.material.roughness,
            min: 0.0,
            max: 1.0,
            step: Some(0.05),
            on_change: shooting_action("setMaterialRoughness", None),
        },
    ]
}

fn shooting_icon_measures(envelope: &ShootingPlayEnvelope, labels: &ShootingLabels) -> Vec<WindowMeasure> {
    let fixture = &envelope.fixture;
    let shot = active_shot(fixture);
    vec![
        WindowMeasure::Select {
            id: "shooting.measure.shot".into(),
            label: Some(labels.shot.into()),
            value: shot.map(|entry| entry.id.clone()).unwrap_or_default(),
            items: fixture
                .shots
                .iter()
                .map(|entry| MeasureSelectItem {
                    id: format!("shooting.measure.shot.{}", entry.id),
                    value: entry.id.clone(),
                    label: entry.label.clone(),
                })
                .collect(),
            on_change: shooting_action("setActiveShot", None),
        },
        WindowMeasure::Select {
            id: "shooting.measure.format".into(),
            label: Some(labels.format_select_label.into()),
            value: shot.map(|entry| entry.format.clone()).unwrap_or_else(|| "svg".into()),
            items: vec![
                MeasureSelectItem { id: "shooting.measure.format.svg".into(), value: "svg".into(), label: labels.format_svg.into() },
                MeasureSelectItem { id: "shooting.measure.format.png".into(), value: "png".into(), label: labels.format_png.into() },
            ],
            on_change: shooting_action("setActiveShotFormat", None),
        },
        WindowMeasure::Select {
            id: "shooting.measure.shape".into(),
            label: Some(labels.shape_select_label.into()),
            value: shot.map(|entry| entry.shape.clone()).unwrap_or_else(|| "rectangle".into()),
            items: vec![
                MeasureSelectItem { id: "shooting.measure.shape.rectangle".into(), value: "rectangle".into(), label: labels.shape_rectangle.into() },
                MeasureSelectItem { id: "shooting.measure.shape.ellipse".into(), value: "ellipse".into(), label: labels.shape_ellipse.into() },
            ],
            on_change: shooting_action("setActiveShotShape", None),
        },
    ]
}

fn shooting_model_engagement(envelope: &ShootingPlayEnvelope, labels: &ShootingLabels) -> WindowEngagement {
    let transform = envelope.runtime.transform_tool.clone();
    WindowEngagement {
        session_active: Some(true),
        options: Some(vec![
            WindowEngagementOption {
                id: "shooting.opt.move".into(),
                label: Some(labels.move_tool.into()),
                icon_id: Some("move".into()),
                pressed: Some(transform == "move"),
                disabled: None,
                action: Some(shooting_action("setTransformTool", Some(json!({ "tool": "move" })))),
            },
            WindowEngagementOption {
                id: "shooting.opt.rotate".into(),
                label: Some(labels.rotate_tool.into()),
                icon_id: Some("rotate-cw".into()),
                pressed: Some(transform == "rotate"),
                disabled: None,
                action: Some(shooting_action("setTransformTool", Some(json!({ "tool": "rotate" })))),
            },
            WindowEngagementOption {
                id: "shooting.opt.scale".into(),
                label: Some(labels.scale_tool.into()),
                icon_id: Some("maximize-2".into()),
                pressed: Some(transform == "scale"),
                disabled: None,
                action: Some(shooting_action("setTransformTool", Some(json!({ "tool": "scale" })))),
            },
        ]),
        input: Some(WindowEngagementInput {
            id: Some("shooting.camera-draft".into()),
            value: Some(envelope.runtime.camera_draft_label.clone()),
            placeholder: Some(labels.camera_label_placeholder.into()),
            disabled: None,
            on_change: Some(shooting_action("setCameraDraftLabel", None)),
            on_submit: Some(shooting_action("saveCamera", None)),
            on_repeat_last: None,
            on_abort: None,
        }),
        control: None,
        controls: None,
        status: Some(vec![WindowEngagementStatus {
            id: "shooting.status.model".into(),
            text: format!("{} assets · {} shots", envelope.fixture.assets.len(), envelope.fixture.shots.len()),
        }]),
        possible_engagements: Some(
            envelope
                .fixture
                .saved_cameras
                .iter()
                .map(|saved| WindowEngagementPossible {
                    id: format!("shooting.camera.{}", saved.id),
                    label: saved.label.clone(),
                    detail: Some(labels.load_camera.into()),
                    action: Some(shooting_action("loadSavedCamera", Some(json!({ "id": saved.id })))),
                })
                .collect(),
        ),
    }
}

fn shooting_icon_engagement(envelope: &ShootingPlayEnvelope, labels: &ShootingLabels) -> WindowEngagement {
    let shot = active_shot(&envelope.fixture);
    WindowEngagement {
        session_active: Some(true),
        options: None,
        input: Some(WindowEngagementInput {
            id: Some("shooting.shot-label".into()),
            value: shot.map(|entry| entry.label.clone()),
            placeholder: Some(labels.shot_label_placeholder.into()),
            disabled: Some(shot.is_none()),
            on_change: Some(shooting_action("setActiveShotLabel", None)),
            on_submit: None,
            on_repeat_last: None,
            on_abort: None,
        }),
        control: None,
        controls: None,
        status: Some(vec![WindowEngagementStatus {
            id: "shooting.status.icon".into(),
            text: shot
                .map(|entry| format!("{}×{} {}", entry.width, entry.height, entry.format.to_uppercase()))
                .unwrap_or_else(|| labels.no_shot.into()),
        }]),
        possible_engagements: None,
    }
}

fn shooting_tools(envelope: &ShootingPlayEnvelope, labels: &ShootingLabels) -> Vec<ToolNode> {
    let has_shot = active_shot(&envelope.fixture).is_some() && active_asset(&envelope.fixture).is_some();
    vec![
        ToolNode::Collection {
            id: "shooting.tools.open".into(),
            icon_id: "folder-open".into(),
            label: Some(labels.open.into()),
            text: None,
            title: Some(labels.import_title.into()),
            order: Some(1),
            disabled: None,
            category: Some(ToolCategory::Actions),
            children: vec![
                ToolNode::Button {
                    id: "shooting.tools.open.fixture".into(),
                    icon_id: "file-json".into(),
                    label: Some("Import Shooting".into()),
                    text: None,
                    title: None,
                    order: Some(1),
                    disabled: None,
                    category: None,
                    on_press: shooting_action("loadRequest", None),
                },
                ToolNode::Button {
                    id: "shooting.tools.open.glb".into(),
                    icon_id: "box".into(),
                    label: Some("Import Glb".into()),
                    text: None,
                    title: None,
                    order: Some(2),
                    disabled: None,
                    category: None,
                    on_press: shooting_action("importAssetRequest", None),
                },
            ],
        },
        ToolNode::Collection {
            id: "shooting.tools.save".into(),
            icon_id: "save".into(),
            label: Some(labels.save.into()),
            text: None,
            title: Some(labels.export_title.into()),
            order: Some(2),
            disabled: None,
            category: Some(ToolCategory::Actions),
            children: vec![
                ToolNode::Button {
                    id: "shooting.tools.save.fixture".into(),
                    icon_id: "download".into(),
                    label: Some("Download Shooting".into()),
                    text: None,
                    title: None,
                    order: Some(1),
                    disabled: None,
                    category: None,
                    on_press: shooting_action("saveDownload", None),
                },
                ToolNode::Button {
                    id: "shooting.tools.save.shot".into(),
                    icon_id: "image".into(),
                    label: Some("Export Shot".into()),
                    text: None,
                    title: None,
                    order: Some(2),
                    disabled: Some(!has_shot),
                    category: None,
                    on_press: shooting_action("exportActiveShot", None),
                },
                ToolNode::Button {
                    id: "shooting.tools.save.shots".into(),
                    icon_id: "images".into(),
                    label: Some("Export All Shots".into()),
                    text: None,
                    title: None,
                    order: Some(3),
                    disabled: Some(!has_shot),
                    category: None,
                    on_press: shooting_action("exportAllShots", None),
                },
                ToolNode::Button {
                    id: "shooting.tools.save.reset".into(),
                    icon_id: "rotate-ccw".into(),
                    label: Some("Reset".into()),
                    text: None,
                    title: None,
                    order: Some(4),
                    disabled: None,
                    category: None,
                    on_press: shooting_action("resetFixture", None),
                },
            ],
        },
        ToolNode::Button {
            id: "shooting.tools.save-camera".into(),
            icon_id: "camera".into(),
            label: Some("Save Camera".into()),
            text: None,
            title: None,
            order: Some(3),
            disabled: None,
            category: Some(ToolCategory::Actions),
            on_press: shooting_action("saveCamera", None),
        },
    ]
}
//#endregion 🔖Tools

//#region 🔖ShootingPlayApp
struct ShootingPlayApp;

impl PluginApp for ShootingPlayApp {
    fn app_id(&self) -> &str {
        SHOOTING_PLAY_APP_ID
    }

    fn initial_document_json(&self) -> String {
        serde_json::to_string(&default_envelope()).expect("shooting envelope json")
    }

    fn handle_action_patch_ops(
        &mut self,
        action: &str,
        args: Option<&Value>,
        document_json: &str,
        _view_state: &ViewState,
    ) -> Vec<String> {
        let mut envelope = parse_envelope(document_json);
        match action {
            "setDocument" => {
                if let Some(document) = args.and_then(|value| value.get("document")) {
                    if let Ok(parsed) = serde_json::from_value(document.clone()) {
                        return vec![set_document_op(&parsed)];
                    }
                }
            }
            "setFixtureJson" => {
                let json_text = args
                    .and_then(|value| value.get("json").or_else(|| value.get("payload")))
                    .and_then(|value| value.as_str())
                    .map(str::to_string)
                    .or_else(|| {
                        args.and_then(|value| value.get("json").or_else(|| value.get("payload")))
                            .filter(|value| value.is_object())
                            .map(|value| value.to_string())
                    });
                if let Some(json_text) = json_text {
                    if let Ok(fixture) = serde_json::from_str::<ShootingFixture>(&json_text) {
                        envelope.fixture = fixture;
                        envelope.runtime = ShootingPlayRuntime::default();
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "setActiveExample" => {
                let example_id = args
                    .and_then(|value| value.get("fixtureId"))
                    .or_else(|| args.and_then(|value| value.get("exampleId")))
                    .and_then(|value| value.as_str())
                    .unwrap_or("");
                envelope = if example_id.is_empty() || example_id == "empty" {
                    ShootingPlayEnvelope {
                        fixture: empty_shooting_fixture(),
                        runtime: ShootingPlayRuntime::default(),
                    }
                } else if example_id == SHOOTING_EXAMPLE_DEFAULT_ID || example_id == "base" {
                    default_envelope()
                } else {
                    envelope
                };
                return vec![set_document_op(&envelope)];
            }
            "setSelection" => {
                let shot_ids = args
                    .and_then(|value| value.get("shotIds"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_default();
                let asset_ids = args
                    .and_then(|value| value.get("assetIds"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_default();
                envelope.runtime.selected_shot_ids = shot_ids;
                envelope.runtime.selected_asset_ids = asset_ids;
                return vec![set_document_op(&envelope)];
            }
            "setActiveShot" => {
                let shot_id = args
                    .and_then(|value| value.get("value"))
                    .or_else(|| args.and_then(|value| value.get("id")))
                    .and_then(|value| value.as_str())
                    .unwrap_or("");
                if !shot_id.is_empty() {
                    envelope.fixture = apply_fixture_edit(
                        &envelope.fixture,
                        &ShootingFixtureEditOp::SetActiveShot { shot_id: shot_id.into() },
                    );
                    return vec![set_document_op(&envelope)];
                }
            }
            "setActiveAsset" => {
                let asset_id = args
                    .and_then(|value| value.get("value"))
                    .or_else(|| args.and_then(|value| value.get("id")))
                    .and_then(|value| value.as_str())
                    .unwrap_or("");
                if !asset_id.is_empty() {
                    envelope.fixture = apply_fixture_edit(
                        &envelope.fixture,
                        &ShootingFixtureEditOp::SetActiveAsset { asset_id: asset_id.into() },
                    );
                    envelope.runtime.fit_revision += 1;
                    return vec![set_document_op(&envelope)];
                }
            }
            "setCamera" => {
                if let Some(camera_value) = args.and_then(|value| value.get("camera")) {
                    if let Ok(camera) = serde_json::from_value::<ShootingCamera>(camera_value.clone()) {
                        let active_shot_id = active_shot(&envelope.fixture).map(|shot| shot.id.clone());
                        apply_camera_for_shot(&mut envelope.fixture, active_shot_id.as_deref(), camera);
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "setShotCamera" => {
                let shot_id = args.and_then(|value| value.get("shotId")).and_then(|value| value.as_str()).map(str::to_string);
                if let (Some(shot_id), Some(camera_value)) = (shot_id, args.and_then(|value| value.get("camera"))) {
                    if let Ok(camera) = serde_json::from_value::<ShootingCamera>(camera_value.clone()) {
                        apply_camera_for_shot(&mut envelope.fixture, Some(&shot_id), camera);
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "saveCamera" => {
                let draft = envelope.runtime.camera_draft_label.trim().to_string();
                let label = if draft.is_empty() {
                    format!("Camera {}", envelope.fixture.saved_cameras.len() + 1)
                } else {
                    draft
                };
                envelope.fixture.saved_cameras.push(ShootingSavedCamera {
                    id: next_shooting_id("camera"),
                    label,
                    camera: envelope.fixture.camera.clone(),
                });
                envelope.runtime.camera_draft_label.clear();
                return vec![set_document_op(&envelope)];
            }
            "loadSavedCamera" => {
                let camera_id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()).unwrap_or("");
                if let Some(saved) = envelope.fixture.saved_cameras.iter().find(|entry| entry.id == camera_id) {
                    envelope.fixture.camera = saved.camera.clone();
                    return vec![set_document_op(&envelope)];
                }
            }
            "setCameraDraftLabel" => {
                envelope.runtime.camera_draft_label = args
                    .and_then(|value| value.get("value"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("")
                    .to_string();
                return vec![set_document_op(&envelope)];
            }
            "setCenterModel" => {
                let next = args
                    .and_then(|value| value.get("pressed").or_else(|| value.get("value")))
                    .and_then(|value| value.as_bool())
                    .unwrap_or(!envelope.runtime.center_model);
                if next && !envelope.runtime.center_model {
                    envelope.runtime.fit_revision += 1;
                }
                envelope.runtime.center_model = next;
                return vec![set_document_op(&envelope)];
            }
            "setTransformTool" => {
                let tool = args.and_then(|value| value.get("tool")).and_then(|value| value.as_str()).unwrap_or("move");
                envelope.runtime.transform_tool = tool.into();
                return vec![set_document_op(&envelope)];
            }
            "setSunAzimuth" | "setSunElevation" | "setSunIntensity" | "setAmbientIntensity" | "setMaterialRoughness" => {
                if let Some(value) = args.and_then(|args| args.get("value")).and_then(|value| value.as_f64()) {
                    match action {
                        "setSunAzimuth" => envelope.fixture.scene.sun.azimuth = value,
                        "setSunElevation" => envelope.fixture.scene.sun.elevation = value,
                        "setSunIntensity" => envelope.fixture.scene.sun.intensity = value,
                        "setAmbientIntensity" => envelope.fixture.scene.ambient.intensity = value,
                        _ => envelope.fixture.scene.material.roughness = value,
                    }
                    return vec![set_document_op(&envelope)];
                }
            }
            "setShadowEnabled" => {
                let next = args
                    .and_then(|value| value.get("value").or_else(|| value.get("pressed")))
                    .and_then(|value| value.as_bool())
                    .unwrap_or(!envelope.fixture.scene.shadow.enabled);
                envelope.fixture.scene.shadow.enabled = next;
                return vec![set_document_op(&envelope)];
            }
            "toggleSun" => {
                let next = args
                    .and_then(|value| value.get("value").or_else(|| value.get("pressed")))
                    .and_then(|value| value.as_bool())
                    .unwrap_or(!envelope.fixture.scene.sun.enabled);
                envelope.fixture.scene.sun.enabled = next;
                return vec![set_document_op(&envelope)];
            }
            "setActiveShotLabel" => {
                if let Some(label) = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()) {
                    if let Some(shot_id) = active_shot(&envelope.fixture).map(|shot| shot.id.clone()) {
                        envelope.fixture = apply_fixture_edit(
                            &envelope.fixture,
                            &ShootingFixtureEditOp::PatchShots {
                                shot_ids: vec![shot_id],
                                field: "label".into(),
                                value: json!(label),
                            },
                        );
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "resetFixture" => {
                return vec![set_document_op(&default_envelope())];
            }
            "saveDownload" => {
                if let Ok(fixture_json) = serde_json::to_string_pretty(&envelope.fixture) {
                    return vec![json!({
                        "op": "downloadMediaExport",
                        "filename": "shooting.fixture.json",
                        "mimeType": "application/json",
                        "data": fixture_json,
                    })
                    .to_string()];
                }
            }
            "loadRequest" => {
                return vec![json!({
                    "op": "requestFileOpen",
                    "accept": ".json,application/json",
                    "importAction": "setFixtureJson",
                })
                .to_string()];
            }
            "importAssetRequest" => {
                return vec![json!({
                    "op": "requestFileOpen",
                    "accept": ".glb,model/gltf-binary",
                    "readAs": "dataUrl",
                    "importAction": "importAsset",
                })
                .to_string()];
            }
            "importAsset" => {
                if let Some(payload) = args.and_then(|value| value.get("payload")).and_then(|value| value.as_str()) {
                    let id = next_shooting_id("asset");
                    let name = args
                        .and_then(|value| value.get("name"))
                        .and_then(|value| value.as_str())
                        .map(|name| name.trim_end_matches(".glb").to_string())
                        .filter(|name| !name.is_empty())
                        .unwrap_or_else(|| format!("Asset {}", envelope.fixture.assets.len() + 1));
                    let asset = ShootingAsset {
                        id: id.clone(),
                        name,
                        url: payload.into(),
                        format: "glb".into(),
                        origin: [0.0, 0.0, 0.0],
                        orientation: Some([0.0, 0.0, 0.0, 1.0]),
                        scale: None,
                    };
                    envelope.fixture = apply_fixture_edit(&envelope.fixture, &ShootingFixtureEditOp::AddAsset { asset });
                    envelope.fixture = apply_fixture_edit(
                        &envelope.fixture,
                        &ShootingFixtureEditOp::SetActiveAsset { asset_id: id.clone() },
                    );
                    envelope.runtime.selected_asset_ids = vec![id];
                    envelope.runtime.selected_shot_ids.clear();
                    envelope.runtime.fit_revision += 1;
                    return vec![set_document_op(&envelope)];
                }
            }
            "exportActiveShot" | "exportAllShots" => {
                if let Some(asset) = active_asset(&envelope.fixture) {
                    let shots: Vec<&ShootingShot> = if action == "exportActiveShot" {
                        active_shot(&envelope.fixture).into_iter().collect()
                    } else {
                        envelope.fixture.shots.iter().collect()
                    };
                    let items: Vec<Value> = shots
                        .iter()
                        .map(|shot| {
                            json!({
                                "filename": format!("{}.{}", shot.id, if shot.format == "png" { "png" } else { "svg" }),
                                "request": serde_json::from_str::<Value>(&shooting_icon_render_request_json(&envelope.fixture, shot, asset)).unwrap_or(Value::Null),
                            })
                        })
                        .collect();
                    if !items.is_empty() {
                        return vec![json!({ "op": "iconRenderExport", "items": items }).to_string()];
                    }
                }
            }
            "translateSelection" => {
                let ids = mesh_selection_ids(args, &envelope.runtime.selected_asset_ids);
                let dx = args.and_then(|value| value.get("dx")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let dy = args.and_then(|value| value.get("dy")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let dz = args.and_then(|value| value.get("dz")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                for asset in &mut envelope.fixture.assets {
                    if ids.contains(&asset.id) {
                        asset.origin[0] += dx;
                        asset.origin[1] += dy;
                        asset.origin[2] += dz;
                    }
                }
                if !ids.is_empty() {
                    return vec![set_document_op(&envelope)];
                }
            }
            "rotateSelection" => {
                let ids = mesh_selection_ids(args, &envelope.runtime.selected_asset_ids);
                let ax = args.and_then(|value| value.get("ax")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let ay = args.and_then(|value| value.get("ay")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let az = args.and_then(|value| value.get("az")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let angle = args.and_then(|value| value.get("angle")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let delta = quat_from_axis_angle(ax, ay, az, angle);
                for asset in &mut envelope.fixture.assets {
                    if ids.contains(&asset.id) {
                        let current = asset.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
                        asset.orientation = Some(quat_mul(delta, current));
                    }
                }
                if !ids.is_empty() {
                    return vec![set_document_op(&envelope)];
                }
            }
            "scaleSelection" => {
                let ids = mesh_selection_ids(args, &envelope.runtime.selected_asset_ids);
                let sx = args.and_then(|value| value.get("sx")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                let sy = args.and_then(|value| value.get("sy")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                let sz = args.and_then(|value| value.get("sz")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                for asset in &mut envelope.fixture.assets {
                    if ids.contains(&asset.id) {
                        let current = asset_scale_json(asset);
                        asset.scale = Some(json!(scale_vec_mul(current, sx, sy, sz)));
                    }
                }
                if !ids.is_empty() {
                    return vec![set_document_op(&envelope)];
                }
            }
            "patchShot" | "patchShots" => {
                let shot_ids: Vec<String> = if action == "patchShot" {
                    args.and_then(|value| value.get("shotId"))
                        .and_then(|value| value.as_str())
                        .map(|id| vec![id.to_string()])
                        .unwrap_or_default()
                } else {
                    args.and_then(|value| value.get("shotIds"))
                        .and_then(|value| serde_json::from_value(value.clone()).ok())
                        .unwrap_or_default()
                };
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                let value = args.and_then(|value| value.get("value")).cloned().unwrap_or(Value::Null);
                if !shot_ids.is_empty() && !field.is_empty() {
                    envelope.fixture = apply_fixture_edit(
                        &envelope.fixture,
                        &ShootingFixtureEditOp::PatchShots {
                            shot_ids,
                            field: field.into(),
                            value,
                        },
                    );
                    return vec![set_document_op(&envelope)];
                }
            }
            "patchAsset" | "patchAssets" => {
                let asset_ids: Vec<String> = if action == "patchAsset" {
                    args.and_then(|value| value.get("assetId"))
                        .and_then(|value| value.as_str())
                        .map(|id| vec![id.to_string()])
                        .unwrap_or_default()
                } else {
                    args.and_then(|value| value.get("assetIds"))
                        .and_then(|value| serde_json::from_value(value.clone()).ok())
                        .unwrap_or_default()
                };
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                let value = args.and_then(|value| value.get("value")).cloned().unwrap_or(Value::Null);
                if !asset_ids.is_empty() && !field.is_empty() {
                    envelope.fixture = apply_fixture_edit(
                        &envelope.fixture,
                        &ShootingFixtureEditOp::PatchAssets {
                            asset_ids,
                            field: field.into(),
                            value,
                        },
                    );
                    return vec![set_document_op(&envelope)];
                }
            }
            "addShot" => {
                let format = args.and_then(|value| value.get("format")).and_then(|value| value.as_str()).unwrap_or("png");
                let shape = args.and_then(|value| value.get("shape")).and_then(|value| value.as_str()).unwrap_or("rectangle");
                let id = next_shooting_id("shot");
                let shot = ShootingShot {
                    id: id.clone(),
                    label: format!("Shot {}", envelope.fixture.shots.len() + 1),
                    width: 256,
                    height: 256,
                    format: format.into(),
                    shape: shape.into(),
                    background: None,
                    camera_id: None,
                };
                envelope.fixture = apply_fixture_edit(&envelope.fixture, &ShootingFixtureEditOp::AddShot { shot });
                envelope.fixture = apply_fixture_edit(
                    &envelope.fixture,
                    &ShootingFixtureEditOp::SetActiveShot { shot_id: id.clone() },
                );
                envelope.runtime.selected_shot_ids = vec![id];
                envelope.runtime.selected_asset_ids.clear();
                return vec![set_document_op(&envelope)];
            }
            "addAsset" => {
                let format = args.and_then(|value| value.get("format")).and_then(|value| value.as_str()).unwrap_or("glb");
                let id = next_shooting_id("asset");
                let asset = ShootingAsset {
                    id: id.clone(),
                    name: format!("Asset {}", envelope.fixture.assets.len() + 1),
                    url: format!("/mesh/placeholder.{format}"),
                    format: format.into(),
                    origin: [0.0, 0.0, 0.0],
                    orientation: Some([0.0, 0.0, 0.0, 1.0]),
                    scale: None,
                };
                envelope.fixture = apply_fixture_edit(&envelope.fixture, &ShootingFixtureEditOp::AddAsset { asset });
                envelope.fixture = apply_fixture_edit(
                    &envelope.fixture,
                    &ShootingFixtureEditOp::SetActiveAsset { asset_id: id.clone() },
                );
                envelope.runtime.selected_asset_ids = vec![id];
                envelope.runtime.selected_shot_ids.clear();
                return vec![set_document_op(&envelope)];
            }
            "setActiveShotFormat" => {
                if let Some(format) = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()) {
                    if let Some(shot) = active_shot(&envelope.fixture).map(|entry| entry.id.clone()) {
                        envelope.fixture = apply_fixture_edit(
                            &envelope.fixture,
                            &ShootingFixtureEditOp::PatchShots {
                                shot_ids: vec![shot],
                                field: "format".into(),
                                value: json!(format),
                            },
                        );
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "setActiveShotShape" => {
                if let Some(shape) = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()) {
                    if let Some(shot) = active_shot(&envelope.fixture).map(|entry| entry.id.clone()) {
                        envelope.fixture = apply_fixture_edit(
                            &envelope.fixture,
                            &ShootingFixtureEditOp::PatchShots {
                                shot_ids: vec![shot],
                                field: "shape".into(),
                                value: json!(shape),
                            },
                        );
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "worldSelect" => {
                let merge = args.and_then(|value| value.get("merge")).and_then(|value| value.as_str()).unwrap_or("replace");
                let ids: Vec<String> = args
                    .and_then(|value| value.get("ids"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_default();
                envelope.runtime.selected_asset_ids =
                    merge_world_selection_ids(&envelope.runtime.selected_asset_ids, &ids, merge);
                return vec![set_document_op(&envelope)];
            }
            "worldHover" => {
                envelope.runtime.hovered_asset_id = args
                    .and_then(|value| value.get("id"))
                    .and_then(|value| value.as_str())
                    .map(str::to_string);
                return vec![set_document_op(&envelope)];
            }
            "setHover" => {
                envelope.runtime.hovered_asset_id = args
                    .and_then(|value| value.get("objectId"))
                    .and_then(|value| value.as_str())
                    .map(str::to_string);
                return vec![set_document_op(&envelope)];
            }
            "worldPick" => {
                let merge = args.and_then(|value| value.get("merge")).and_then(|value| value.as_str()).unwrap_or("replace");
                let id_value = args.and_then(|value| value.get("id"));
                if id_value.map_or(true, |value| value.is_null()) {
                    if merge == "replace" {
                        envelope.runtime.selected_asset_ids.clear();
                    }
                    return vec![set_document_op(&envelope)];
                }
                let asset_id = id_value
                    .and_then(|value| value.as_u64())
                    .and_then(|index| envelope.fixture.assets.get(index as usize))
                    .map(|asset| asset.id.clone())
                    .or_else(|| id_value.and_then(|value| value.as_str()).map(str::to_string));
                if let Some(asset_id) = asset_id {
                    envelope.runtime.selected_asset_ids =
                        merge_world_selection_ids(&envelope.runtime.selected_asset_ids, &[asset_id.clone()], merge);
                    envelope.fixture.active_asset_id = asset_id;
                    return vec![set_document_op(&envelope)];
                }
            }
            "setSelectionMethod" => {
                let method = args
                    .and_then(|value| value.get("method"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("rectangle");
                envelope.runtime.selection_method = method.into();
                return vec![set_document_op(&envelope)];
            }
            "worldPointerDown" => return Vec::new(),
            _ => {}
        }
        Vec::new()
    }

    fn render(&self, body_key: &str, document_json: &str, view_state: &ViewState) -> UiNode {
        let envelope = parse_envelope(document_json);
        let labels = shooting_labels(view_state);
        match body_key {
            SHOOTING_PLAY_BODY_SCENE => render_model_scene(&envelope.fixture, &envelope.runtime),
            SHOOTING_PLAY_BODY_ICON => render_icon_scene(&envelope.fixture),
            SHOOTING_PLAY_BODY_DOCUMENT => build_document_tree(&envelope, labels),
            SHOOTING_PLAY_BODY_CATALOGUE => build_catalogue_tree(labels),
            SHOOTING_PLAY_BODY_INSPECTION => build_inspector_tree(&envelope, labels),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn tools(&self, document_json: &str, view_state: &ViewState) -> Vec<ToolNode> {
        shooting_tools(&parse_envelope(document_json), shooting_labels(view_state))
    }

    fn window_engagements(&self, document_json: &str, view_state: &ViewState) -> HashMap<String, WindowEngagement> {
        let envelope = parse_envelope(document_json);
        let labels = shooting_labels(view_state);
        HashMap::from([
            (SHOOTING_PLAY_WINDOW_SCENE.into(), shooting_model_engagement(&envelope, labels)),
            (SHOOTING_PLAY_WINDOW_ICON.into(), shooting_icon_engagement(&envelope, labels)),
        ])
    }

    fn window_measures(&self, document_json: &str, view_state: &ViewState) -> HashMap<String, Vec<WindowMeasure>> {
        let envelope = parse_envelope(document_json);
        let labels = shooting_labels(view_state);
        HashMap::from([
            (SHOOTING_PLAY_WINDOW_SCENE.into(), shooting_model_measures(&envelope)),
            (SHOOTING_PLAY_WINDOW_ICON.into(), shooting_icon_measures(&envelope, labels)),
        ])
    }

    fn app_labels(&self, view_state: &ViewState) -> semio_framework_plugin::AppLabelsOverlay {
        let labels = shooting_labels(view_state);
        semio_framework_plugin::AppLabelsOverlay {
            app_label: None,
            window_kind_labels: std::collections::HashMap::from([
                (SHOOTING_PLAY_WINDOW_SCENE.to_string(), labels.window_scene.to_string()),
                (SHOOTING_PLAY_WINDOW_ICON.to_string(), labels.window_icon.to_string()),
            ]),
            panel_tab_labels: std::collections::HashMap::new(),
            mode_labels: std::collections::HashMap::new(),
        }
    }
}
//#endregion 🔖ShootingPlayApp

//#region 🔖Manifest
fn create_shooting_app() -> App {
    App::from_builder(
        App::builder(SHOOTING_PLAY_APP_ID, "Shooting").document(["semio", "shooting"])
            .icon_id("camera")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind(SHOOTING_PLAY_WINDOW_SCENE, "Scene", SHOOTING_PLAY_BODY_SCENE, SurfaceKind::World3d)
            .window_kind(SHOOTING_PLAY_WINDOW_ICON, "Icon", SHOOTING_PLAY_BODY_ICON, SurfaceKind::IconRender)
            .default_layout(create_default_layout(
                &[SHOOTING_PLAY_WINDOW_SCENE.into(), SHOOTING_PLAY_WINDOW_ICON.into()],
                "row",
                Some(&[68.0, 32.0]),
                Some(&["Model".into(), "Icon".into()]),
            ))
            .panel_tab(
                FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
                FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
                PanelGroup::Workbench,
                SHOOTING_PLAY_BODY_DOCUMENT,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                PanelGroup::Workbench,
                SHOOTING_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                PanelGroup::Details,
                SHOOTING_PLAY_BODY_INSPECTION,
            ),
    )
    .example(
        SHOOTING_EXAMPLE_DEFAULT_ID,
        "Default Base Icon",
        DEFAULT_EXAMPLE_JSON,
    )
    .program("shooting", "Shooting", "icon")
}

fn shooting_document_json_to_svg(value: &Value) -> Result<(String, u32, u32), String> {
    let fixture: ShootingFixture = serde_json::from_value(
        value.get("fixture").cloned().unwrap_or_else(|| value.clone()),
    )
    .map_err(|error| error.to_string())?;
    Ok(shooting_scene_svg(&fixture))
}

/// 🎯 Frames a `ShootingCamera` around a DWG extent, reusing the default studio angle but
/// scaling distance to the drawing's bounding box; degenerates gracefully for an empty drawing.
fn shooting_camera_from_dwg_bounds(extmin: [f64; 3], extmax: [f64; 3]) -> ShootingCamera {
    let center = [
        (extmin[0] + extmax[0]) * 0.5,
        (extmin[1] + extmax[1]) * 0.5,
        (extmin[2] + extmax[2]) * 0.5,
    ];
    let span = [(extmax[0] - extmin[0]).abs(), (extmax[1] - extmin[1]).abs(), (extmax[2] - extmin[2]).abs()];
    let radius = span[0].max(span[1]).max(span[2]) * 0.5;
    let distance = if radius > 1e-6 { radius * 2.6 } else { 600.0 };
    let direction = default_camera_position();
    let direction_len = (direction[0] * direction[0] + direction[1] * direction[1] + direction[2] * direction[2]).sqrt().max(1e-6);
    let position = [
        center[0] + direction[0] / direction_len * distance,
        center[1] + direction[1] / direction_len * distance,
        center[2] + direction[2] / direction_len * distance,
    ];
    ShootingCamera { position, target: center, zoom: 1.0, fov: default_fov(), up: None, projection: None }
}

/// 📥 Tier C DWG import for `2d.shooting`: the format has no wall/obstacle concept, so this
/// always returns the default studio fixture with the camera reframed to the drawing extent —
/// never errors, including for a structurally empty `DwgDrawing`.
fn shooting_document_json_from_dwg(drawing: &DwgDrawing) -> Result<Value, String> {
    let mut envelope = default_envelope();
    envelope.fixture.camera = shooting_camera_from_dwg_bounds(drawing.extmin, drawing.extmax);
    serde_json::to_value(&envelope).map_err(|error| error.to_string())
}

fn register_shooting_exports() {
    semio_framework_os::register_2d_export_handlers("2d.shooting", "shooting", shooting_document_json_to_svg);
    semio_framework_os::register_dwg_import_handler("2d.shooting", shooting_document_json_from_dwg);
}

fn bundle() -> PluginBundle {
    register_shooting_exports();
    PluginBundle::new("shooting", "Shooting", "0.1.0").register_app(create_shooting_app(), || Box::new(ShootingPlayApp))
}

semio_framework_plugin::plugin_exports!(bundle);
//#endregion 🔖Manifest

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::PluginApp;

    #[test]
    fn renders_world_model_scene() {
        let app = ShootingPlayApp;
        let document = app.initial_document_json();
        let node = app.render(SHOOTING_PLAY_BODY_SCENE, &document, &ViewState::default());
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
    fn renders_icon_render_scene_with_real_request() {
        let app = ShootingPlayApp;
        let document = app.initial_document_json();
        let node = app.render(SHOOTING_PLAY_BODY_ICON, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("icon-render"));
        let payload: Value = serde_json::from_str(&json).unwrap();
        let request: Value = serde_json::from_str(payload["iconRender"]["requestJson"].as_str().unwrap()).unwrap();
        assert_eq!(request["assetUrl"], json!("/mesh/base.glb"));
        assert_eq!(request["format"], json!("svg"));
        assert_eq!(request["shape"], json!("rectangle"));
        assert!(request.get("background").is_none(), "transparent default fixture background is omitted");
        assert_eq!(request["lights"]["sunAzimuth"], json!(45.0));
        assert!(payload["iconRender"]["footer"].as_str().unwrap().contains("256×256"));
    }

    #[test]
    fn save_and_load_camera_round_trip() {
        let mut app = ShootingPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_action_patch_ops("setCameraDraftLabel", Some(&json!({ "value": "Hero" })), &document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&document), &ops);
        let document = serde_json::to_string(&envelope).unwrap();
        let ops = app.handle_action_patch_ops("saveCamera", None, &document, &ViewState::default());
        let envelope = apply_ops(&envelope, &ops);
        assert_eq!(envelope.fixture.saved_cameras.len(), 1);
        assert_eq!(envelope.fixture.saved_cameras[0].label, "Hero");
        assert!(envelope.runtime.camera_draft_label.is_empty());
        let saved_id = envelope.fixture.saved_cameras[0].id.clone();
        let mut moved = envelope.clone();
        moved.fixture.camera.position = [1.0, 2.0, 3.0];
        let document = serde_json::to_string(&moved).unwrap();
        let ops = app.handle_action_patch_ops("loadSavedCamera", Some(&json!({ "id": saved_id })), &document, &ViewState::default());
        let restored = apply_ops(&moved, &ops);
        assert_eq!(restored.fixture.camera.position, envelope.fixture.saved_cameras[0].camera.position);
        let engagements = app.window_engagements(&serde_json::to_string(&restored).unwrap(), &ViewState::default());
        let possible = engagements[SHOOTING_PLAY_WINDOW_SCENE].possible_engagements.as_ref().unwrap();
        assert!(possible.iter().any(|entry| entry.label == "Hero"));
    }

    #[test]
    fn set_shot_camera_writes_saved_camera_when_shot_references_one() {
        let mut app = ShootingPlayApp;
        let mut envelope = default_envelope();
        envelope.fixture.saved_cameras.push(ShootingSavedCamera {
            id: "camera-a".into(),
            label: "A".into(),
            camera: ShootingCamera::default(),
        });
        envelope.fixture.shots[0].camera_id = Some("camera-a".into());
        let document = serde_json::to_string(&envelope).unwrap();
        let camera = json!({ "position": [9.0, 9.0, 9.0], "target": [0.0, 0.0, 0.0], "zoom": 2.0, "fov": 50.0 });
        let ops = app.handle_action_patch_ops(
            "setShotCamera",
            Some(&json!({ "shotId": envelope.fixture.shots[0].id, "camera": camera })),
            &document,
            &ViewState::default(),
        );
        let next = apply_ops(&envelope, &ops);
        assert_eq!(next.fixture.saved_cameras[0].camera.position, [9.0, 9.0, 9.0]);
        assert_eq!(next.fixture.camera.position, envelope.fixture.camera.position, "fixture camera untouched");
    }

    #[test]
    fn scene_setters_mutate_lighting_and_measures_reflect_them() {
        let mut app = ShootingPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_action_patch_ops("setSunAzimuth", Some(&json!({ "value": 90.0 })), &document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&document), &ops);
        assert_eq!(envelope.fixture.scene.sun.azimuth, 90.0);
        let ops = app.handle_action_patch_ops("setShadowEnabled", Some(&json!({ "pressed": false })), &serde_json::to_string(&envelope).unwrap(), &ViewState::default());
        let envelope = apply_ops(&envelope, &ops);
        assert!(!envelope.fixture.scene.shadow.enabled);
        let measures = app.window_measures(&serde_json::to_string(&envelope).unwrap(), &ViewState::default());
        let model_measures = &measures[SHOOTING_PLAY_WINDOW_SCENE];
        assert!(model_measures.iter().any(|measure| matches!(measure, WindowMeasure::Slider { value, .. } if *value == 90.0)));
        assert!(measures[SHOOTING_PLAY_WINDOW_ICON].iter().any(|measure| matches!(measure, WindowMeasure::Select { .. })));
    }

    #[test]
    fn toggle_sun_round_trips_through_runtime_and_defaults_off() {
        let mut app = ShootingPlayApp;
        let document = app.initial_document_json();
        let envelope = parse_envelope(&document);
        assert!(!envelope.fixture.scene.sun.enabled, "sun must be off by default");
        let ops = app.handle_action_patch_ops("toggleSun", None, &document, &ViewState::default());
        let next = apply_ops(&envelope, &ops);
        assert!(next.fixture.scene.sun.enabled);
        let measures = app.window_measures(&document, &ViewState::default());
        let model_measures = &measures[SHOOTING_PLAY_WINDOW_SCENE];
        assert!(model_measures.iter().any(|measure| matches!(measure, WindowMeasure::Toggle { id, .. } if id == "shooting.measure.sun-enabled")));
    }

    #[test]
    fn center_model_and_asset_activation_bump_fit_revision() {
        let mut app = ShootingPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_action_patch_ops("setCenterModel", Some(&json!({ "pressed": false })), &document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&document), &ops);
        assert!(!envelope.runtime.center_model);
        let ops = app.handle_action_patch_ops("setCenterModel", Some(&json!({ "pressed": true })), &serde_json::to_string(&envelope).unwrap(), &ViewState::default());
        let envelope = apply_ops(&envelope, &ops);
        assert!(envelope.runtime.center_model);
        assert_eq!(envelope.runtime.fit_revision, 1);
        let asset_id = envelope.fixture.assets[0].id.clone();
        let ops = app.handle_action_patch_ops("setActiveAsset", Some(&json!({ "value": asset_id })), &serde_json::to_string(&envelope).unwrap(), &ViewState::default());
        let envelope = apply_ops(&envelope, &ops);
        assert_eq!(envelope.runtime.fit_revision, 2);
    }

    #[test]
    fn world_pick_and_hover_drive_selection_protocol() {
        let mut app = ShootingPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_action_patch_ops("worldPick", Some(&json!({ "granularity": "mesh", "id": 0, "merge": "replace" })), &document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&document), &ops);
        assert_eq!(envelope.runtime.selected_asset_ids, vec!["base".to_string()]);
        assert_eq!(envelope.fixture.active_asset_id, "base");
        let selection: Value = serde_json::from_str(&world_selection_json(&envelope.fixture, &envelope.runtime)).unwrap();
        assert_eq!(selection["gumballActive"], json!(true));
        assert_eq!(selection["activeObjectId"], json!("base"));
        assert!(selection.get("gumballTarget").is_some());
        assert_eq!(selection["transformTool"], json!("move"));
        let ops = app.handle_action_patch_ops("setHover", Some(&json!({ "objectId": "base" })), &serde_json::to_string(&envelope).unwrap(), &ViewState::default());
        let envelope = apply_ops(&envelope, &ops);
        assert_eq!(envelope.runtime.hovered_asset_id.as_deref(), Some("base"));
        let ops = app.handle_action_patch_ops("worldPick", Some(&json!({ "granularity": "mesh", "id": Value::Null, "merge": "replace" })), &serde_json::to_string(&envelope).unwrap(), &ViewState::default());
        let envelope = apply_ops(&envelope, &ops);
        assert!(envelope.runtime.selected_asset_ids.is_empty());
    }

    #[test]
    fn export_import_and_download_ops() {
        let mut app = ShootingPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_action_patch_ops("exportActiveShot", None, &document, &ViewState::default());
        let op: Value = serde_json::from_str(&ops[0]).unwrap();
        assert_eq!(op["op"], json!("iconRenderExport"));
        assert_eq!(op["items"].as_array().unwrap().len(), 1);
        assert_eq!(op["items"][0]["filename"], json!("overview-svg.svg"));
        assert_eq!(op["items"][0]["request"]["assetUrl"], json!("/mesh/base.glb"));
        let ops = app.handle_action_patch_ops("exportAllShots", None, &document, &ViewState::default());
        let op: Value = serde_json::from_str(&ops[0]).unwrap();
        assert_eq!(op["items"].as_array().unwrap().len(), 2);
        let ops = app.handle_action_patch_ops("saveDownload", None, &document, &ViewState::default());
        let op: Value = serde_json::from_str(&ops[0]).unwrap();
        assert_eq!(op["op"], json!("downloadMediaExport"));
        assert_eq!(op["filename"], json!("shooting.fixture.json"));
        let round_trip: ShootingFixture = serde_json::from_str(op["data"].as_str().unwrap()).unwrap();
        assert_eq!(round_trip.schema, SHOOTING_FIXTURE_SCHEMA);
        let ops = app.handle_action_patch_ops("loadRequest", None, &document, &ViewState::default());
        let op: Value = serde_json::from_str(&ops[0]).unwrap();
        assert_eq!(op["op"], json!("requestFileOpen"));
        assert_eq!(op["importAction"], json!("setFixtureJson"));
        let ops = app.handle_action_patch_ops("importAssetRequest", None, &document, &ViewState::default());
        let op: Value = serde_json::from_str(&ops[0]).unwrap();
        assert_eq!(op["readAs"], json!("dataUrl"));
        assert_eq!(op["importAction"], json!("importAsset"));
        let ops = app.handle_action_patch_ops(
            "importAsset",
            Some(&json!({ "payload": "data:model/gltf-binary;base64,AAAA", "name": "chair.glb" })),
            &document,
            &ViewState::default(),
        );
        let envelope = apply_ops(&parse_envelope(&document), &ops);
        let imported = envelope.fixture.assets.last().unwrap();
        assert_eq!(imported.name, "chair");
        assert!(imported.url.starts_with("data:"));
        assert_eq!(envelope.fixture.active_asset_id, imported.id);
    }

    #[test]
    fn tools_and_engagements_expose_toolbar_parity() {
        let app = ShootingPlayApp;
        let document = app.initial_document_json();
        let tools = app.tools(&document, &ViewState::default());
        let json = serde_json::to_string(&tools).unwrap();
        for command in ["loadRequest", "importAssetRequest", "saveDownload", "exportActiveShot", "exportAllShots", "resetFixture", "saveCamera"] {
            assert!(json.contains(command), "toolbar exposes {command}");
        }
        let engagements = app.window_engagements(&document, &ViewState::default());
        let model = &engagements[SHOOTING_PLAY_WINDOW_SCENE];
        assert!(model.options.as_ref().unwrap().iter().any(|option| option.id.ends_with("move")));
        assert!(model.status.as_ref().unwrap()[0].text.contains("assets"));
        let icon = &engagements[SHOOTING_PLAY_WINDOW_ICON];
        assert!(icon.status.as_ref().unwrap()[0].text.contains("256×256"));
    }

    #[test]
    fn shooting_labels_resolve_native_english_by_default() {
        let app = ShootingPlayApp;
        let document = app.initial_document_json();
        let document_tree = app.render(SHOOTING_PLAY_BODY_DOCUMENT, &document, &ViewState::default());
        let document_json = serde_json::to_string(&document_tree).unwrap();
        assert!(document_json.contains("Shots"));
        assert!(document_json.contains("Assets"));
        let catalogue = app.render(SHOOTING_PLAY_BODY_CATALOGUE, &document, &ViewState::default());
        let catalogue_json = serde_json::to_string(&catalogue).unwrap();
        assert!(catalogue_json.contains("Add Shot"));
        assert!(catalogue_json.contains("Add Asset"));
        assert!(catalogue_json.contains("SVG Rectangle"));
        assert!(catalogue_json.contains("GLB Asset"));
        let inspector = app.render(SHOOTING_PLAY_BODY_INSPECTION, &document, &ViewState::default());
        assert!(serde_json::to_string(&inspector).unwrap().contains("Shot"));
        let tools = app.tools(&document, &ViewState::default());
        let tools_json = serde_json::to_string(&tools).unwrap();
        assert!(tools_json.contains("\"label\":\"Open\""));
        assert!(tools_json.contains("\"title\":\"Import\""));
        assert!(tools_json.contains("\"label\":\"Save\""));
        assert!(tools_json.contains("\"title\":\"Export\""));
        let engagements = app.window_engagements(&document, &ViewState::default());
        let model = &engagements[SHOOTING_PLAY_WINDOW_SCENE];
        assert!(model.options.as_ref().unwrap().iter().any(|option| option.label.as_deref() == Some("Move")));
        assert_eq!(model.input.as_ref().unwrap().placeholder.as_deref(), Some("Camera label"));
        let icon = &engagements[SHOOTING_PLAY_WINDOW_ICON];
        assert_eq!(icon.input.as_ref().unwrap().placeholder.as_deref(), Some("Shot label"));
        let measures = app.window_measures(&document, &ViewState::default());
        let icon_measures_json = serde_json::to_string(&measures[SHOOTING_PLAY_WINDOW_ICON]).unwrap();
        assert!(icon_measures_json.contains("Rectangle"));
        assert!(icon_measures_json.contains("SVG"));
    }

    #[test]
    fn shooting_labels_resolve_native_german() {
        let app = ShootingPlayApp;
        let document = app.initial_document_json();
        let view_state = ViewState { locale: Some("de".into()), ..ViewState::default() };
        let document_tree = app.render(SHOOTING_PLAY_BODY_DOCUMENT, &document, &view_state);
        let document_json = serde_json::to_string(&document_tree).unwrap();
        assert!(document_json.contains("Aufnahmen"));
        assert!(document_json.contains("Objekte"));
        let catalogue = app.render(SHOOTING_PLAY_BODY_CATALOGUE, &document, &view_state);
        let catalogue_json = serde_json::to_string(&catalogue).unwrap();
        assert!(catalogue_json.contains("Aufnahme hinzufügen"));
        assert!(catalogue_json.contains("Objekt hinzufügen"));
        assert!(catalogue_json.contains("SVG Rechteck"));
        assert!(catalogue_json.contains("GLB-Objekt"));
        let inspector = app.render(SHOOTING_PLAY_BODY_INSPECTION, &document, &view_state);
        assert!(serde_json::to_string(&inspector).unwrap().contains("Aufnahme"));
        let tools = app.tools(&document, &view_state);
        let tools_json = serde_json::to_string(&tools).unwrap();
        assert!(tools_json.contains("\"label\":\"Öffnen\""));
        assert!(tools_json.contains("\"title\":\"Importieren\""));
        assert!(tools_json.contains("\"label\":\"Speichern\""));
        assert!(tools_json.contains("\"title\":\"Exportieren\""));
        let engagements = app.window_engagements(&document, &view_state);
        let model = &engagements[SHOOTING_PLAY_WINDOW_SCENE];
        assert!(model.options.as_ref().unwrap().iter().any(|option| option.label.as_deref() == Some("Verschieben")));
        assert_eq!(model.input.as_ref().unwrap().placeholder.as_deref(), Some("Kamera-Bezeichnung"));
        let icon = &engagements[SHOOTING_PLAY_WINDOW_ICON];
        assert_eq!(icon.input.as_ref().unwrap().placeholder.as_deref(), Some("Aufnahme-Bezeichnung"));
        let measures = app.window_measures(&document, &view_state);
        let icon_measures_json = serde_json::to_string(&measures[SHOOTING_PLAY_WINDOW_ICON]).unwrap();
        assert!(icon_measures_json.contains("Rechteck"));
    }

    #[test]
    fn set_active_shot_label_patches_active_shot() {
        let mut app = ShootingPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_action_patch_ops("setActiveShotLabel", Some(&json!({ "value": "Hero Shot" })), &document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&document), &ops);
        assert_eq!(active_shot(&envelope.fixture).unwrap().label, "Hero Shot");
    }

    #[test]
    fn reset_fixture_restores_default_envelope() {
        let mut app = ShootingPlayApp;
        let mut envelope = default_envelope();
        envelope.fixture.shots.clear();
        let document = serde_json::to_string(&envelope).unwrap();
        let ops = app.handle_action_patch_ops("resetFixture", None, &document, &ViewState::default());
        let restored = apply_ops(&envelope, &ops);
        assert_eq!(restored.fixture.shots.len(), 2);
    }

    #[test]
    fn scene_svg_embeds_active_asset_name_and_shot_shape() {
        let envelope = default_envelope();
        let (svg, width, height) = shooting_scene_svg(&envelope.fixture);
        let shot = active_shot(&envelope.fixture).expect("default fixture shot");
        let asset = active_asset(&envelope.fixture).expect("default fixture asset");
        assert_eq!((width, height), (shot.width, shot.height));
        assert!(svg.contains(&asset.name), "svg emblem includes active asset name");
        assert!(if shot.shape == "ellipse" { svg.contains("<ellipse") } else { svg.contains("<rect") });
    }

    #[test]
    fn export_svg_uses_scene_render_not_title_card() {
        let envelope = default_envelope();
        let document = json!({ "fixture": envelope.fixture });
        let (svg, _width, _height) = shooting_document_json_to_svg(&document).expect("export svg");
        let asset = active_asset(&envelope.fixture).expect("default fixture asset");
        assert!(svg.contains(&asset.name));
        assert!(!svg.contains("Shooting"), "export renders the real scene, not the generic title card");
    }

    #[test]
    fn dwg_import_frames_camera_to_extent_and_stays_schema_valid() {
        let mut drawing = DwgDrawing::default();
        drawing.extmin = [0.0, 0.0, 0.0];
        drawing.extmax = [100.0, 200.0, 0.0];
        let document = shooting_document_json_from_dwg(&drawing).expect("dwg import never errors");
        let envelope: ShootingPlayEnvelope = serde_json::from_value(document).expect("schema-valid envelope");
        assert_eq!(envelope.fixture.schema, SHOOTING_FIXTURE_SCHEMA);
        assert!(!envelope.fixture.shots.is_empty());
        assert_eq!(envelope.fixture.camera.target, [50.0, 100.0, 0.0]);
        assert_ne!(envelope.fixture.camera.position, ShootingCamera::default().position);
    }

    #[test]
    fn dwg_import_never_errors_on_empty_drawing() {
        let drawing = DwgDrawing::default();
        let document = shooting_document_json_from_dwg(&drawing).expect("dwg import never errors on empty drawing");
        let envelope: ShootingPlayEnvelope = serde_json::from_value(document).expect("schema-valid envelope");
        assert_eq!(envelope.fixture.schema, SHOOTING_FIXTURE_SCHEMA);
        assert_eq!(envelope.fixture.camera.target, [0.0, 0.0, 0.0]);
    }

    #[test]
    fn model_scene_uses_asset_mesh_urls() {
        let app = ShootingPlayApp;
        let document = app.initial_document_json();
        let node = app.render(SHOOTING_PLAY_BODY_SCENE, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("mesh:base"));
        assert!(json.contains("/mesh/base.glb"));
    }

    #[test]
    fn document_lists_shots_and_assets() {
        let app = ShootingPlayApp;
        let document = app.initial_document_json();
        let node = app.render(SHOOTING_PLAY_BODY_DOCUMENT, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Overview Svg"));
        assert!(json.contains("Base"));
    }

    #[test]
    fn add_shot_action_appends_shot() {
        let mut app = ShootingPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_action_patch_ops("addShot", Some(&json!({ "format": "svg", "shape": "ellipse" })), &document, &ViewState::default());
        let envelope: ShootingPlayEnvelope = apply_ops(&parse_envelope(&document), &ops);
        assert!(envelope.fixture.shots.iter().any(|shot| shot.format == "svg" && shot.shape == "ellipse"));
    }

    #[test]
    fn set_active_shot_updates_fixture() {
        let mut app = ShootingPlayApp;
        let document = app.initial_document_json();
        let envelope = parse_envelope(&document);
        let second_id = envelope.fixture.shots.get(1).map(|shot| shot.id.clone()).expect("second shot");
        let ops = app.handle_action_patch_ops("setActiveShot", Some(&json!({ "value": second_id })), &document, &ViewState::default());
        let next: ShootingPlayEnvelope = apply_ops(&envelope, &ops);
        assert_eq!(next.fixture.active_shot_id, second_id);
    }

    #[test]
    fn default_example_fixture_parses() {
        let envelope = default_envelope();
        assert_eq!(envelope.fixture.schema, SHOOTING_FIXTURE_SCHEMA);
        assert!(!envelope.fixture.shots.is_empty());
        assert!(!envelope.fixture.assets.is_empty());
    }

    fn apply_ops(envelope: &ShootingPlayEnvelope, ops: &[String]) -> ShootingPlayEnvelope {
        let mut next = envelope.clone();
        for op_json in ops {
            if let Ok(op) = serde_json::from_str::<Value>(op_json) {
                if let Some(document) = op.get("document") {
                    if let Ok(parsed) = serde_json::from_value(document.clone()) {
                        next = parsed;
                    }
                }
            }
        }
        next
    }
}
//#endregion 🧪Tests
