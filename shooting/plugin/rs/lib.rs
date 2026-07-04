//! 📸 Shooting plugin — icon studio with model + preview windows bundled as a hot-swappable WASM component.

use semio_framework_plugin::{
    build_raster_scene, build_world_3d_scene, create_default_layout, ui_inspector_groups_to_tree,
    ui_inspector_mixed_number, ui_inspector_readonly_field, ui_stack_vertical, ui_text, App, CommandDescriptor, PluginApp,
    PluginBundle, RasterScene, UiControlNode, UiFieldNode, UiInspectorFieldGroup, UiNode, UiTreeItemNode,
    UiTreeNode, UiTreeSectionNode, ViewState, World3dScene,
    FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_HIERARCHY_ID,
    FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::LazyLock;

//#region 🔖Constants
const SHOOTING_PLAY_APP_ID: &str = "shooting-play";
const SHOOTING_PLAY_CONTROLLER_ID: &str = "shooting-play";
const SHOOTING_PLAY_SURFACE_MODEL: &str = "shooting.play.model";
const SHOOTING_PLAY_SURFACE_ICON: &str = "shooting.play.icon";
const SHOOTING_PLAY_BODY_MODEL: &str = "shooting.play.model";
const SHOOTING_PLAY_BODY_ICON: &str = "shooting.play.icon";
const SHOOTING_PLAY_BODY_HIERARCHY: &str = "shooting.play.hierarchy";
const SHOOTING_PLAY_BODY_CATALOGUE: &str = "shooting.play.catalogue";
const SHOOTING_PLAY_BODY_INSPECTION: &str = "shooting.play.inspection";
const SHOOTING_PLAY_WINDOW_MODEL: &str = "shooting-model";
const SHOOTING_PLAY_WINDOW_ICON: &str = "shooting-icon";
const SHOOTING_FIXTURE_SCHEMA: &str = "shooting.fixture";
const SHOOTING_EXAMPLE_DEFAULT_ID: &str = "base-icon";

const DEFAULT_EXAMPLE_JSON: &str = include_str!("../../example/base-icon.shooting.json");

static SHOOTING_ID_COUNTER: AtomicU32 = AtomicU32::new(0);
//#endregion 🔖Constants

//#region 🔖Document
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
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
}

fn default_camera_position() -> [f64; 3] {
    [0.0, -5.0, 3.0]
}

fn default_camera_target() -> [f64; 3] {
    [0.0, 0.0, 0.0]
}

fn default_fov() -> f64 {
    50.0
}

fn one_f64() -> f64 {
    1.0
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ShootingAsset {
    id: String,
    name: String,
    url: String,
    #[serde(default = "default_glb_format")]
    format: String,
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
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct ShootingSceneLighting {
    #[serde(default)]
    background: String,
    #[serde(default)]
    sun: Value,
    #[serde(default)]
    ambient: Value,
    #[serde(default)]
    shadow: Value,
    #[serde(default)]
    material: Value,
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
    saved_cameras: Vec<Value>,
    #[serde(default)]
    scene: ShootingSceneLighting,
    #[serde(default)]
    shots: Vec<ShootingShot>,
    #[serde(default)]
    active_shot_id: String,
    #[serde(default)]
    active_asset_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ShootingPlayRuntime {
    #[serde(default)]
    selected_shot_ids: Vec<String>,
    #[serde(default)]
    selected_asset_ids: Vec<String>,
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
        camera: ShootingCamera {
            position: default_camera_position(),
            target: default_camera_target(),
            zoom: 1.0,
            fov: default_fov(),
        },
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

fn shooting_cmd(command: &str, args: Option<Value>) -> CommandDescriptor {
    CommandDescriptor {
        controller_id: SHOOTING_PLAY_CONTROLLER_ID.into(),
        command: command.into(),
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
    json!({
        "x": camera.position[0],
        "y": camera.position[1],
        "z": camera.position[2],
        "fov": camera.fov,
    })
    .to_string()
}

fn world_instances_json(fixture: &ShootingFixture) -> String {
    let instances: Vec<Value> = fixture
        .assets
        .iter()
        .enumerate()
        .map(|(index, asset)| {
            let active = fixture.active_asset_id == asset.id
                || (fixture.active_asset_id.is_empty() && index == 0);
            json!({
                "id": asset.id,
                "x": if active { 0.0 } else { index as f64 * 2.0 },
                "y": 0.0,
                "z": 0.0,
                "scale": if active { 1.2 } else { 0.8 },
                "label": asset.name,
                "color": if active { "#9aa0ab" } else { "#6b7280" },
            })
        })
        .collect();
    serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into())
}
//#endregion 🔖FixtureOps

//#region 🔖Panels
fn tree_item(id: impl Into<String>, label: impl Into<String>) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.into(),
        label: label.into(),
        description: None,
        icon_id: None,
        selected: None,
        default_open: None,
        command: None,
        draggable: None,
        drag_data: None,
        items: None,
        control: None,
        is_hidden: None,
    }
}

fn tree_item_with_command(
    id: impl Into<String>,
    label: impl Into<String>,
    icon_id: Option<&str>,
    command: CommandDescriptor,
) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.into(),
        label: label.into(),
        description: None,
        icon_id: icon_id.map(str::to_string),
        selected: None,
        default_open: None,
        command: Some(command),
        draggable: None,
        drag_data: None,
        items: None,
        control: None,
        is_hidden: None,
    }
}

fn build_hierarchy_tree(envelope: &ShootingPlayEnvelope) -> UiNode {
    let fixture = &envelope.fixture;
    let shot_items: Vec<UiTreeItemNode> = fixture
        .shots
        .iter()
        .map(|shot| {
            tree_item_with_command(
                format!("shooting-shot:{}", shot.id),
                shot.label.clone(),
                Some("camera"),
                shooting_cmd("setSelection", Some(json!({ "shotIds": [shot.id], "assetIds": [] }))),
            )
        })
        .collect();
    let asset_items: Vec<UiTreeItemNode> = fixture
        .assets
        .iter()
        .map(|asset| {
            tree_item_with_command(
                format!("shooting-asset:{}", asset.id),
                asset.name.clone(),
                Some("box"),
                shooting_cmd("setSelection", Some(json!({ "shotIds": [], "assetIds": [asset.id] }))),
            )
        })
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![
            UiTreeSectionNode {
                id: "shooting-play-hierarchy.shots".into(),
                label: Some("Shots".into()),
                default_open: Some(true),
                items: shot_items,
            },
            UiTreeSectionNode {
                id: "shooting-play-hierarchy.assets".into(),
                label: Some("Assets".into()),
                default_open: Some(true),
                items: asset_items,
            },
        ],
        selected_ids: None,
        highlighted_ids: None,
        selection_change: None,
    })
}

fn build_catalogue_tree() -> UiNode {
    UiNode::Tree(UiTreeNode {
        sections: vec![
            UiTreeSectionNode {
                id: "shooting-play-catalogue.shots".into(),
                label: Some("Add Shot".into()),
                default_open: Some(true),
                items: vec![
                    catalog_shot_item("svg-rect", "SVG Rectangle", "svg", "rectangle"),
                    catalog_shot_item("png-rect", "PNG Rectangle", "png", "rectangle"),
                    catalog_shot_item("svg-ellipse", "SVG Ellipse", "svg", "ellipse"),
                    catalog_shot_item("png-ellipse", "PNG Ellipse", "png", "ellipse"),
                ],
            },
            UiTreeSectionNode {
                id: "shooting-play-catalogue.assets".into(),
                label: Some("Add Asset".into()),
                default_open: Some(true),
                items: vec![tree_item_with_command(
                    "shooting-play-catalogue.asset.glb",
                    "GLB Asset",
                    Some("box"),
                    shooting_cmd("addAsset", Some(json!({ "format": "glb" }))),
                )],
            },
        ],
        selected_ids: None,
        highlighted_ids: None,
        selection_change: None,
    })
}

fn catalog_shot_item(id: &str, label: &str, format: &str, shape: &str) -> UiTreeItemNode {
    tree_item_with_command(
        format!("shooting-play-catalogue.{id}"),
        label,
        Some("camera"),
        shooting_cmd("addShot", Some(json!({ "format": format, "shape": shape }))),
    )
}

fn build_inspector_tree(envelope: &ShootingPlayEnvelope) -> UiNode {
    let fixture = &envelope.fixture;
    if !envelope.runtime.selected_shot_ids.is_empty() {
        let shot_id = &envelope.runtime.selected_shot_ids[0];
        if let Some(shot) = fixture.shots.iter().find(|entry| &entry.id == shot_id) {
            return ui_inspector_groups_to_tree(&[shot_inspector_group(shot)]);
        }
    }
    if !envelope.runtime.selected_asset_ids.is_empty() {
        let asset_id = &envelope.runtime.selected_asset_ids[0];
        if let Some(asset) = fixture.assets.iter().find(|entry| &entry.id == asset_id) {
            return ui_inspector_groups_to_tree(&[asset_inspector_group(asset)]);
        }
    }
    if let Some(shot) = active_shot(fixture) {
        return ui_inspector_groups_to_tree(&[shot_inspector_group(shot)]);
    }
    ui_stack_vertical(vec![
        ui_text(format!("Schema: {SHOOTING_FIXTURE_SCHEMA}")),
        ui_text(format!("Shots: {}", fixture.shots.len())),
        ui_text(format!("Assets: {}", fixture.assets.len())),
    ])
}

fn shot_inspector_group(shot: &ShootingShot) -> UiInspectorFieldGroup {
    let width_mixed = ui_inspector_mixed_number(&[shot.width as f64]);
    let height_mixed = ui_inspector_mixed_number(&[shot.height as f64]);
    UiInspectorFieldGroup {
        id: "shooting-play-inspector.shot".into(),
        label: "Shot".into(),
        default_open: None,
        fields: vec![
            UiNode::Field(UiFieldNode {
                id: "shooting-play-inspector.shot.label".into(),
                label: "Label".into(),
                child: UiControlNode::Input(semio_framework_plugin::UiInputNode {
                    id: "shooting-play-inspector.shot.label.input".into(),
                    input_kind: "text".into(),
                    value: shot.label.clone(),
                    placeholder: None,
                    commit: None,
                    on_change: shooting_cmd(
                        "patchShot",
                        Some(json!({ "shotId": shot.id, "field": "label" })),
                    ),
                }),
            }),
            ui_inspector_readonly_field("shooting-play-inspector.shot.format", "Format", &shot.format),
            ui_inspector_readonly_field("shooting-play-inspector.shot.shape", "Shape", &shot.shape),
            UiNode::Field(UiFieldNode {
                id: "shooting-play-inspector.shot.width".into(),
                label: "Width".into(),
                child: UiControlNode::Input(semio_framework_plugin::UiInputNode {
                    id: "shooting-play-inspector.shot.width.input".into(),
                    input_kind: "number".into(),
                    value: width_mixed.value.to_string(),
                    placeholder: None,
                    commit: None,
                    on_change: shooting_cmd(
                        "patchShot",
                        Some(json!({ "shotId": shot.id, "field": "width" })),
                    ),
                }),
            }),
            UiNode::Field(UiFieldNode {
                id: "shooting-play-inspector.shot.height".into(),
                label: "Height".into(),
                child: UiControlNode::Input(semio_framework_plugin::UiInputNode {
                    id: "shooting-play-inspector.shot.height.input".into(),
                    input_kind: "number".into(),
                    value: height_mixed.value.to_string(),
                    placeholder: None,
                    commit: None,
                    on_change: shooting_cmd(
                        "patchShot",
                        Some(json!({ "shotId": shot.id, "field": "height" })),
                    ),
                }),
            }),
        ],
    }
}

fn asset_inspector_group(asset: &ShootingAsset) -> UiInspectorFieldGroup {
    UiInspectorFieldGroup {
        id: "shooting-play-inspector.asset".into(),
        label: "Asset".into(),
        default_open: None,
        fields: vec![
            UiNode::Field(UiFieldNode {
                id: "shooting-play-inspector.asset.name".into(),
                label: "Name".into(),
                child: UiControlNode::Input(semio_framework_plugin::UiInputNode {
                    id: "shooting-play-inspector.asset.name.input".into(),
                    input_kind: "text".into(),
                    value: asset.name.clone(),
                    placeholder: None,
                    commit: None,
                    on_change: shooting_cmd(
                        "patchAsset",
                        Some(json!({ "assetId": asset.id, "field": "name" })),
                    ),
                }),
            }),
            UiNode::Field(UiFieldNode {
                id: "shooting-play-inspector.asset.url".into(),
                label: "URL".into(),
                child: UiControlNode::Input(semio_framework_plugin::UiInputNode {
                    id: "shooting-play-inspector.asset.url.input".into(),
                    input_kind: "text".into(),
                    value: asset.url.clone(),
                    placeholder: None,
                    commit: None,
                    on_change: shooting_cmd(
                        "patchAsset",
                        Some(json!({ "assetId": asset.id, "field": "url" })),
                    ),
                }),
            }),
            ui_inspector_readonly_field("shooting-play-inspector.asset.format", "Format", &asset.format),
        ],
    }
}
//#endregion 🔖Panels

//#region 🔖Render
fn render_model_scene(fixture: &ShootingFixture) -> UiNode {
    build_world_3d_scene(
        SHOOTING_PLAY_SURFACE_MODEL,
        SHOOTING_PLAY_APP_ID,
        World3dScene {
            camera_json: camera_json(&fixture.camera),
            meshes_json: "[]".into(),
            instances_json: world_instances_json(fixture),
            selection_json: "[]".into(),
        },
    )
}

fn render_icon_scene(fixture: &ShootingFixture) -> UiNode {
    let shot = active_shot(fixture);
    let (width, height) = shot.map(|entry| (entry.width, entry.height)).unwrap_or((256, 256));
    build_raster_scene(
        SHOOTING_PLAY_SURFACE_ICON,
        SHOOTING_PLAY_APP_ID,
        RasterScene {
            width,
            height,
            pixels_base64: String::new(),
        },
    )
}
//#endregion 🔖Render

//#region 🔖ShootingPlayApp
struct ShootingPlayApp;

impl PluginApp for ShootingPlayApp {
    fn app_id(&self) -> &str {
        SHOOTING_PLAY_APP_ID
    }

    fn initial_document_json(&self) -> String {
        serde_json::to_string(&default_envelope()).expect("shooting envelope json")
    }

    fn handle_command(
        &mut self,
        command: &str,
        args: Option<&Value>,
        document_json: &str,
        _view_state: &ViewState,
    ) -> Vec<String> {
        let mut envelope = parse_envelope(document_json);
        match command {
            "setDocument" => {
                if let Some(document) = args.and_then(|value| value.get("document")) {
                    if let Ok(parsed) = serde_json::from_value(document.clone()) {
                        return vec![set_document_op(&parsed)];
                    }
                }
            }
            "setFixtureJson" => {
                if let Some(json_text) = args.and_then(|value| value.get("json")).and_then(|value| value.as_str()) {
                    if let Ok(fixture) = serde_json::from_str::<ShootingFixture>(json_text) {
                        envelope.fixture = fixture;
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
                    return vec![set_document_op(&envelope)];
                }
            }
            "setCamera" => {
                if let Some(camera_value) = args.and_then(|value| value.get("camera")) {
                    if let Ok(camera) = serde_json::from_value(camera_value.clone()) {
                        envelope.fixture = apply_fixture_edit(
                            &envelope.fixture,
                            &ShootingFixtureEditOp::SetCamera { camera },
                        );
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "patchShot" | "patchShots" => {
                let shot_ids: Vec<String> = if command == "patchShot" {
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
                let asset_ids: Vec<String> = if command == "patchAsset" {
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
            "worldPointerDown" => return Vec::new(),
            _ => {}
        }
        Vec::new()
    }

    fn render(&self, body_key: &str, document_json: &str, _view_state: &ViewState) -> UiNode {
        let envelope = parse_envelope(document_json);
        match body_key {
            SHOOTING_PLAY_BODY_MODEL => render_model_scene(&envelope.fixture),
            SHOOTING_PLAY_BODY_ICON => render_icon_scene(&envelope.fixture),
            SHOOTING_PLAY_BODY_HIERARCHY => build_hierarchy_tree(&envelope),
            SHOOTING_PLAY_BODY_CATALOGUE => build_catalogue_tree(),
            SHOOTING_PLAY_BODY_INSPECTION => build_inspector_tree(&envelope),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }
}
//#endregion 🔖ShootingPlayApp

//#region 🔖Manifest
fn create_shooting_app() -> App {
    App::from_builder(
        App::builder(SHOOTING_PLAY_APP_ID, "Shooting")
            .icon_id("camera")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind(SHOOTING_PLAY_WINDOW_MODEL, "Model", SHOOTING_PLAY_BODY_MODEL)
            .window_kind(SHOOTING_PLAY_WINDOW_ICON, "Icon", SHOOTING_PLAY_BODY_ICON)
            .default_layout(create_default_layout(
                &[SHOOTING_PLAY_WINDOW_MODEL.into(), SHOOTING_PLAY_WINDOW_ICON.into()],
                "row",
                Some(&[68.0, 32.0]),
                Some(&["Model".into(), "Icon".into()]),
            ))
            .panel_tab(
                FRAMEWORK_PANEL_TAB_HIERARCHY_ID,
                FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
                "workbench",
                SHOOTING_PLAY_BODY_HIERARCHY,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                "workbench",
                SHOOTING_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                "details",
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

fn bundle() -> PluginBundle {
    PluginBundle::new("shooting", "Shooting", "0.1.0").register_app(create_shooting_app(), || Box::new(ShootingPlayApp))
}

static _PLUGIN_INIT: LazyLock<()> = LazyLock::new(|| semio_framework_plugin::install_plugin_bundle(bundle()));

semio_framework_plugin::wasm_plugin_exports!();
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
        let node = app.render(SHOOTING_PLAY_BODY_MODEL, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("world-3d"));
    }

    #[test]
    fn renders_icon_raster_scene() {
        let app = ShootingPlayApp;
        let document = app.initial_document_json();
        let node = app.render(SHOOTING_PLAY_BODY_ICON, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("raster"));
    }

    #[test]
    fn hierarchy_lists_shots_and_assets() {
        let app = ShootingPlayApp;
        let document = app.initial_document_json();
        let node = app.render(SHOOTING_PLAY_BODY_HIERARCHY, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Overview Svg"));
        assert!(json.contains("Base"));
    }

    #[test]
    fn add_shot_command_appends_shot() {
        let mut app = ShootingPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_command("addShot", Some(&json!({ "format": "svg", "shape": "ellipse" })), &document, &ViewState::default());
        let envelope: ShootingPlayEnvelope = apply_ops(&parse_envelope(&document), &ops);
        assert!(envelope.fixture.shots.iter().any(|shot| shot.format == "svg" && shot.shape == "ellipse"));
    }

    #[test]
    fn set_active_shot_updates_fixture() {
        let mut app = ShootingPlayApp;
        let document = app.initial_document_json();
        let envelope = parse_envelope(&document);
        let second_id = envelope.fixture.shots.get(1).map(|shot| shot.id.clone()).expect("second shot");
        let ops = app.handle_command("setActiveShot", Some(&json!({ "value": second_id })), &document, &ViewState::default());
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
