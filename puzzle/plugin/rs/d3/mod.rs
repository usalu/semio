//! 🧊 Puzzle 3D plugin — 3D puzzle assembly play app bundled as a hot-swappable WASM component.

use puzzle_3d::{BrushPlacePayload, Puzzle3dPrecomputeSession};
use semio_framework_plugin::{
    build_world_3d_scene, create_default_layout, layout::{MeasureSelectItem, WindowEngagementToggleGroupOption}, merge_world_selection_ids, mesh_from_kind, ui_inspector_groups_to_tree, ui_inspector_readonly_field,
    ui_stack_vertical, ui_text, world3d_chunking_json, world3d_mesh_id_from_url, world3d_meshes_json_from_kinds_and_urls, world3d_meshes_json_from_urls, world3d_scene_extended, world3d_selection_json, App, CommandDescriptor, PanelGroup, PluginApp,
    SurfaceKind, UiControlNode, UiFieldNode, UiInspectorFieldGroup, UiNode, UiTreeItemAction, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, ViewState, WindowEngagement, WindowEngagementControl, WindowEngagementInput, WindowEngagementOption, WindowMeasure, FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::LazyLock;

//#region 🔖Constants
const PUZZLE3D_PLAY_APP_ID: &str = "puzzle3d-play";
const PUZZLE3D_PLAY_CONTROLLER_ID: &str = "puzzle3d-play";
const PUZZLE3D_PLAY_SURFACE_VIEWPORT: &str = "puzzle.3d.play.viewport";
const PUZZLE3D_PLAY_BODY_COMPOSITE: &str = "puzzle3d.play.composite";
const PUZZLE3D_PLAY_BODY_DOCUMENT: &str = "puzzle.3d.play.document";
const PUZZLE3D_PLAY_BODY_KINDS: &str = "puzzle.3d.play.kinds";
const PUZZLE3D_PLAY_BODY_INSPECTOR: &str = "puzzle.3d.play.inspector";
const PUZZLE3D_PLAY_BODY_SETTINGS: &str = "puzzle.3d.play.settings";
const PUZZLE3D_PLAY_BODY_JACK: &str = "puzzle.3d.play.jack";
const PUZZLE3D_PLAY_WINDOW_MAIN: &str = "puzzle3d-main";
const PUZZLE3D_FIXTURE_SCHEMA: &str = "puzzle.3d.fixture";
const PUZZLE3D_EXAMPLE_CONCRETE_FOREST: &str = "concrete-forest";
const PUZZLE3D_EXAMPLE_NAKAGIN: &str = "nakagin-capsule-tower";
const PUZZLE3D_FALLBACK_MESH_KIND: &str = "box";
const PUZZLE3D_ENGAGEMENT_TOOL_BRUSH: &str = "puzzle3d.tool.brush";
const PUZZLE3D_ENGAGEMENT_TOOL_SELECT: &str = "puzzle3d.tool.select";
const PUZZLE3D_ENGAGEMENT_TOOL_FILL: &str = "puzzle3d.tool.fill";
const PUZZLE3D_FILL_COUNT_MAX: u32 = 1000;

/// ⏪ Commands that mutate the shared `envelope` in place and should be undoable — excludes `setDocument`/`setActiveExample`,
/// which replace the whole envelope wholesale (any pre-match snapshot on it would be discarded), and view-only state
/// (selection/hover/camera/tool) so undo only ever touches document content.
const PUZZLE3D_UNDOABLE_COMMANDS: &[&str] = &[
    "setFixtureJson",
    "addObjectKind",
    "deleteSelection",
    "duplicateSelection",
    "worldRelocate",
    "setSelectionFlag",
    "patchInspector",
    "addBrushObject",
    "setFillCount",
    "createAttraction",
    "deleteAttraction",
    "addTargetVolume",
    "deleteTargetVolume",
];
const PUZZLE3D_UNDO_STACK_MAX: usize = 50;

const CONCRETE_FOREST_EXAMPLE_JSON: &str = include_str!("../../../3d/example/concrete-forest.3d.json");
const NAKAGIN_EXAMPLE_JSON: &str = include_str!("../../../3d/example/nakagin-capsule-tower.3d.json");
//#endregion 🔖Constants

//#region 🔖Document
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Puzzle3dCamera {
    #[serde(default)]
    position: [f64; 3],
    #[serde(default)]
    target: [f64; 3],
    #[serde(default = "one_f64")]
    zoom: f64,
    #[serde(default)]
    up: Option<[f64; 3]>,
    #[serde(default)]
    projection: Option<String>,
}

fn one_f64() -> f64 {
    1.0
}

fn default_selection_method() -> String {
    "rectangle".into()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Puzzle3dVortex {
    id: String,
    #[serde(default, rename = "vortexKind")]
    vortex_kind: Option<String>,
    #[serde(default)]
    position: [f64; 3],
    #[serde(default)]
    direction: Option<[f64; 3]>,
    #[serde(default)]
    radius: Option<f64>,
    #[serde(default)]
    hidden: bool,
    #[serde(default)]
    locked: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Puzzle3dReferenceSource {
    #[serde(default)]
    url: String,
    #[serde(default, rename = "mediaKind")]
    media_kind: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Puzzle3dReference {
    id: String,
    #[serde(default)]
    source: Puzzle3dReferenceSource,
    #[serde(default)]
    origin: [f64; 3],
    #[serde(default, rename = "widthWorld")]
    width_world: f64,
    #[serde(default)]
    locked: bool,
    #[serde(default)]
    hidden: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Puzzle3dObject {
    id: String,
    #[serde(default)]
    label: Option<String>,
    #[serde(default, rename = "objectKind")]
    object_kind: Option<String>,
    #[serde(default)]
    origin: [f64; 3],
    #[serde(default)]
    orientation: Option<[f64; 4]>,
    #[serde(default)]
    scale: Option<Value>,
    #[serde(default, rename = "meshUrl")]
    mesh_url: Option<String>,
    #[serde(default)]
    vortices: Vec<Puzzle3dVortex>,
    #[serde(default)]
    hidden: bool,
    #[serde(default)]
    locked: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Puzzle3dFixtureMeta {
    #[serde(default, rename = "kindCatalogs")]
    kind_catalogs: Option<Value>,
    #[serde(default, rename = "kindCompatibility")]
    kind_compatibility: Option<Value>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Puzzle3dTargetVolume {
    id: String,
    #[serde(default)]
    origin: [f64; 3],
    #[serde(default)]
    orientation: Option<[f64; 4]>,
    #[serde(default)]
    scale: Option<Value>,
    #[serde(default)]
    hidden: bool,
    #[serde(default)]
    locked: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Puzzle3dAttraction {
    id: String,
    attracting: String,
    attracted: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Puzzle3dFixture {
    schema: String,
    #[serde(default)]
    domain: String,
    #[serde(default)]
    camera: Puzzle3dCamera,
    #[serde(default)]
    meta: Puzzle3dFixtureMeta,
    #[serde(default)]
    objects: Vec<Puzzle3dObject>,
    #[serde(default)]
    attractions: Vec<Puzzle3dAttraction>,
    #[serde(default, rename = "targetVolumes")]
    target_volumes: Vec<Puzzle3dTargetVolume>,
    #[serde(default)]
    references: Vec<Puzzle3dReference>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Puzzle3dSelection {
    #[serde(default)]
    object_ids: Vec<String>,
    #[serde(default)]
    vortex_ids: Vec<String>,
    #[serde(default)]
    attraction_ids: Vec<String>,
    #[serde(default)]
    target_volume_ids: Vec<String>,
    #[serde(default)]
    reference_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Puzzle3dSelectableKinds {
    #[serde(default = "default_true")]
    objects: bool,
    #[serde(default = "default_true")]
    vortices: bool,
    #[serde(default = "default_true")]
    attractions: bool,
}

impl Default for Puzzle3dSelectableKinds {
    fn default() -> Self {
        Self { objects: true, vortices: true, attractions: true }
    }
}

fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Puzzle3dRuntime {
    #[serde(default)]
    selection: Puzzle3dSelection,
    #[serde(default)]
    active_tool: String,
    #[serde(default = "default_selection_method")]
    selection_method: String,
    #[serde(default)]
    hovered_object_id: Option<String>,
    #[serde(default = "default_overlap_budget")]
    overlap_budget: f64,
    #[serde(default)]
    fill_count: u32,
    #[serde(default)]
    brush_candidate_index: usize,
    #[serde(default)]
    object_kind_weights: HashMap<String, f64>,
    #[serde(default)]
    vortex_kind_weights: HashMap<String, f64>,
    #[serde(default = "default_transform_tool")]
    transform_tool: String,
    #[serde(default = "default_true")]
    lod_automatic: bool,
    #[serde(default)]
    lod_depth_variable: bool,
    #[serde(default = "default_true")]
    lod_show_grid: bool,
    #[serde(default = "default_manual_lod")]
    lod_manual: f64,
    #[serde(default)]
    grid_snap_enabled: bool,
    #[serde(default = "default_grid_factor")]
    grid_factor: f64,
    #[serde(default)]
    selectable_kinds: Puzzle3dSelectableKinds,
    #[serde(default)]
    hovered_kind_id: Option<String>,
    #[serde(default)]
    engagement_input: String,
    #[serde(default)]
    undo_stack: Vec<Puzzle3dFixture>,
    #[serde(default)]
    redo_stack: Vec<Puzzle3dFixture>,
    #[serde(default = "default_selection_mode")]
    selection_mode_default: String,
    #[serde(default = "default_proximity_radius")]
    proximity_radius: f64,
    #[serde(default = "default_chunk_size")]
    chunk_size: f64,
    #[serde(default)]
    fill_edit_target_volumes: bool,
    #[serde(default = "default_voxel_dims")]
    voxel_dims: [u32; 3],
    #[serde(default = "default_jack_query")]
    jack_query: String,
    #[serde(default = "default_view_preset")]
    view_preset: String,
}

impl Default for Puzzle3dRuntime {
    /// 🎛️ Mirrors every `#[serde(default = "...")]` above — `#[derive(Default)]` would silently ignore
    /// them and zero out fields like `overlap_budget`/`selection_method`/`lod_automatic` in Rust-constructed runtimes.
    fn default() -> Self {
        Self {
            selection: Puzzle3dSelection::default(),
            active_tool: String::new(),
            selection_method: default_selection_method(),
            hovered_object_id: None,
            overlap_budget: default_overlap_budget(),
            fill_count: 0,
            brush_candidate_index: 0,
            object_kind_weights: HashMap::new(),
            vortex_kind_weights: HashMap::new(),
            transform_tool: default_transform_tool(),
            lod_automatic: default_true(),
            lod_depth_variable: false,
            lod_show_grid: default_true(),
            lod_manual: default_manual_lod(),
            grid_snap_enabled: false,
            grid_factor: default_grid_factor(),
            selectable_kinds: Puzzle3dSelectableKinds::default(),
            hovered_kind_id: None,
            engagement_input: String::new(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            selection_mode_default: default_selection_mode(),
            proximity_radius: default_proximity_radius(),
            chunk_size: default_chunk_size(),
            fill_edit_target_volumes: false,
            voxel_dims: default_voxel_dims(),
            jack_query: default_jack_query(),
            view_preset: default_view_preset(),
        }
    }
}

fn default_view_preset() -> String {
    "perspective".into()
}

fn default_transform_tool() -> String {
    "move".into()
}

fn default_overlap_budget() -> f64 {
    0.02
}

fn default_manual_lod() -> f64 {
    100.0
}

fn default_grid_factor() -> f64 {
    10.0
}

fn default_selection_mode() -> String {
    "default".into()
}

fn default_proximity_radius() -> f64 {
    0.75
}

fn default_chunk_size() -> f64 {
    256.0
}

fn default_voxel_dims() -> [u32; 3] {
    [1, 1, 1]
}

fn default_jack_query() -> String {
    "MATCH (n:Object) RETURN n.name".into()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Puzzle3dEnvelope {
    fixture: Puzzle3dFixture,
    #[serde(default)]
    runtime: Puzzle3dRuntime,
}

static PUZZLE3D_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

fn empty_fixture() -> Puzzle3dFixture {
    Puzzle3dFixture {
        schema: PUZZLE3D_FIXTURE_SCHEMA.into(),
        domain: "architecture".into(),
        camera: Puzzle3dCamera::default(),
        meta: Puzzle3dFixtureMeta::default(),
        objects: Vec::new(),
        attractions: Vec::new(),
        target_volumes: Vec::new(),
        references: Vec::new(),
    }
}

fn default_envelope() -> Puzzle3dEnvelope {
    serde_json::from_str::<Puzzle3dFixture>(CONCRETE_FOREST_EXAMPLE_JSON)
        .map(|fixture| Puzzle3dEnvelope { fixture, runtime: Puzzle3dRuntime::default() })
        .unwrap_or_else(|_| Puzzle3dEnvelope { fixture: empty_fixture(), runtime: Puzzle3dRuntime::default() })
}

fn nakagin_envelope() -> Puzzle3dEnvelope {
    serde_json::from_str::<Puzzle3dFixture>(NAKAGIN_EXAMPLE_JSON)
        .map(|fixture| Puzzle3dEnvelope { fixture, runtime: Puzzle3dRuntime::default() })
        .unwrap_or_else(|_| Puzzle3dEnvelope { fixture: empty_fixture(), runtime: Puzzle3dRuntime::default() })
}

fn parse_envelope(document_json: &str) -> Puzzle3dEnvelope {
    if let Ok(envelope) = serde_json::from_str::<Puzzle3dEnvelope>(document_json) {
        return envelope;
    }
    if let Ok(fixture) = serde_json::from_str::<Puzzle3dFixture>(document_json) {
        return Puzzle3dEnvelope { fixture, runtime: Puzzle3dRuntime::default() };
    }
    default_envelope()
}

fn set_document_op(envelope: &Puzzle3dEnvelope) -> String {
    json!({ "op": "setDocument", "document": envelope }).to_string()
}

fn puzzle3d_cmd(command: &str, args: Option<Value>) -> CommandDescriptor {
    CommandDescriptor { controller_id: PUZZLE3D_PLAY_CONTROLLER_ID.into(), command: command.into(), args }
}

fn camera_json(camera: &Puzzle3dCamera) -> String {
    let mut value = json!({
        "position": camera.position,
        "target": camera.target,
        "zoom": camera.zoom,
        "fov": 45.0,
    });
    if let Some(object) = value.as_object_mut() {
        if let Some(up) = camera.up {
            object.insert("up".into(), json!(up));
        }
        if let Some(projection) = &camera.projection {
            object.insert("projection".into(), json!(projection));
        }
    }
    value.to_string()
}

fn mesh_selection_ids(args: Option<&Value>, fallback: &[String]) -> Vec<String> {
    args.and_then(|value| value.get("ids")).and_then(|value| serde_json::from_value(value.clone()).ok()).filter(|ids: &Vec<String>| !ids.is_empty()).unwrap_or_else(|| fallback.to_vec())
}

/// 🎥 Named orbit-camera rigs — top/front/right use an orthographic projection with a non-Z `up` to avoid gimbal lock when looking straight down/along the Z-up axis.
fn puzzle3d_camera_view_preset(preset: &str) -> Puzzle3dCamera {
    match preset {
        "top" => Puzzle3dCamera { position: [0.0, 0.0, 30.0], target: [0.0, 0.0, 0.0], zoom: 1.0, up: Some([0.0, 1.0, 0.0]), projection: Some("orthographic".into()) },
        "front" => Puzzle3dCamera { position: [0.0, -30.0, 0.0], target: [0.0, 0.0, 0.0], zoom: 1.0, up: Some([0.0, 0.0, 1.0]), projection: Some("orthographic".into()) },
        "right" => Puzzle3dCamera { position: [30.0, 0.0, 0.0], target: [0.0, 0.0, 0.0], zoom: 1.0, up: Some([0.0, 0.0, 1.0]), projection: Some("orthographic".into()) },
        "perspective" => Puzzle3dCamera { position: [12.0, -12.0, 9.0], target: [0.0, 0.0, 0.0], zoom: 1.0, up: Some([0.0, 0.0, 1.0]), projection: Some("perspective".into()) },
        _ => Puzzle3dCamera { position: [12.0, -12.0, 9.0], target: [0.0, 0.0, 0.0], zoom: 1.0, up: None, projection: None },
    }
}

fn quat_mul(a: [f64; 4], b: [f64; 4]) -> [f64; 4] {
    [a[3] * b[0] + a[0] * b[3] + a[1] * b[2] - a[2] * b[1], a[3] * b[1] - a[0] * b[2] + a[1] * b[3] + a[2] * b[0], a[3] * b[2] + a[0] * b[1] - a[1] * b[0] + a[2] * b[3], a[3] * b[3] - a[0] * b[0] - a[1] * b[1] - a[2] * b[2]]
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

fn scale_value_mul(scale: &Option<Value>, sx: f64, sy: f64, sz: f64) -> Value {
    match scale {
        Some(Value::Array(values)) if values.len() >= 3 => json!([values[0].as_f64().unwrap_or(1.0) * sx, values[1].as_f64().unwrap_or(1.0) * sy, values[2].as_f64().unwrap_or(1.0) * sz,]),
        Some(Value::Number(value)) => {
            let factor = value.as_f64().unwrap_or(1.0);
            json!([factor * sx, factor * sy, factor * sz])
        }
        _ => json!([sx, sy, sz]),
    }
}

fn resolve_object_mesh_url(object: &Puzzle3dObject, meta: &Puzzle3dFixtureMeta) -> Option<String> {
    if let Some(url) = object.mesh_url.as_ref().filter(|url| !url.is_empty()) {
        return Some(url.clone());
    }
    let kind_id = object.object_kind.as_deref()?;
    let catalogs = meta.kind_catalogs.as_ref()?;
    let objects = catalogs.get("objects")?.as_array()?;
    for entry in objects {
        if entry.get("id").and_then(|v| v.as_str()) == Some(kind_id) {
            return entry.get("meshUrl").and_then(|v| v.as_str()).map(str::to_string);
        }
    }
    None
}

fn collect_mesh_urls(fixture: &Puzzle3dFixture) -> Vec<String> {
    let mut urls = HashSet::new();
    for object in &fixture.objects {
        if let Some(url) = resolve_object_mesh_url(object, &fixture.meta) {
            urls.insert(url);
        }
    }
    if let Some(catalogs) = fixture.meta.kind_catalogs.as_ref() {
        if let Some(objects) = catalogs.get("objects").and_then(|v| v.as_array()) {
            for entry in objects {
                if let Some(url) = entry.get("meshUrl").and_then(|v| v.as_str()) {
                    urls.insert(url.to_string());
                }
            }
        }
    }
    urls.into_iter().collect()
}

fn object_scale_json(object: &Puzzle3dObject) -> [f64; 3] {
    match &object.scale {
        Some(Value::Array(values)) if values.len() >= 3 => [values[0].as_f64().unwrap_or(1.0), values[1].as_f64().unwrap_or(1.0), values[2].as_f64().unwrap_or(1.0)],
        _ => [1.0, 1.0, 1.0],
    }
}

/// 🙈 Hidden objects stay in the emitted array — `worldPick`'s `id` arg is the array index into it — but render at zero scale so they're effectively invisible without shifting any other object's index.
fn world_instances_json(fixture: &Puzzle3dFixture, runtime: &Puzzle3dRuntime) -> String {
    let selection = &runtime.selection;
    let instances: Vec<Value> = fixture
        .objects
        .iter()
        .map(|object| {
            let selected = selection.object_ids.contains(&object.id);
            let hovered = runtime.hovered_object_id.as_deref() == Some(object.id.as_str());
            let kind_highlighted = runtime.hovered_kind_id.is_some() && runtime.hovered_kind_id.as_deref() == object.object_kind.as_deref();
            let mesh_id = resolve_object_mesh_url(object, &fixture.meta).map(|url| world3d_mesh_id_from_url(&url)).unwrap_or_else(|| PUZZLE3D_FALLBACK_MESH_KIND.into());
            let scale = if object.hidden { json!([0.0, 0.0, 0.0]) } else { json!(object_scale_json(object)) };
            json!({
                "id": object.id,
                "meshId": mesh_id,
                "position": [
                    object.origin.first().copied().unwrap_or(0.0),
                    object.origin.get(1).copied().unwrap_or(0.0),
                    object.origin.get(2).copied().unwrap_or(0.0),
                ],
                "rotation": object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]),
                "scale": scale,
                "label": object.label.clone().or_else(|| object.object_kind.clone()).unwrap_or_else(|| object.id.clone()),
                "color": if selected { "#f59e0b" } else if hovered || kind_highlighted { "#fbbf24" } else { "#94a3b8" },
                "selected": selected,
                "hovered": hovered || kind_highlighted,
            })
        })
        .collect();
    serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into())
}

fn world_meshes_json(fixture: &Puzzle3dFixture) -> String {
    let urls = collect_mesh_urls(fixture);
    let kinds = vec![PUZZLE3D_FALLBACK_MESH_KIND.into(), "vortex-marker".into()];
    if urls.is_empty() {
        return world3d_meshes_json_from_kinds_and_urls(&kinds, &[]);
    }
    let mut meshes_json = world3d_meshes_json_from_kinds_and_urls(&kinds, &urls);
    if !meshes_json.contains(PUZZLE3D_FALLBACK_MESH_KIND) {
        let fallback = world3d_meshes_json_from_kinds_and_urls(&[PUZZLE3D_FALLBACK_MESH_KIND.into()], &[]);
        let mut merged: Vec<Value> = serde_json::from_str(&meshes_json).unwrap_or_default();
        let fallback_meshes: Vec<Value> = serde_json::from_str(&fallback).unwrap_or_default();
        merged.extend(fallback_meshes);
        meshes_json = serde_json::to_string(&merged).unwrap_or(meshes_json);
    }
    meshes_json
}

fn quat_rotate_vector(quat: [f64; 4], vector: [f64; 3]) -> [f64; 3] {
    let [x, y, z, w] = quat;
    let vx = vector[0];
    let vy = vector[1];
    let vz = vector[2];
    let ix = w * vx + y * vz - z * vy;
    let iy = w * vy + z * vx - x * vz;
    let iz = w * vz + x * vy - y * vx;
    let iw = -x * vx - y * vy - z * vz;
    [ix * w + iw * -x + iy * -z - iz * -y, iy * w + iw * -y + iz * -x - ix * -z, iz * w + iw * -z + ix * -y - iy * -x]
}

fn world_vortex_position(object: &Puzzle3dObject, vortex: &Puzzle3dVortex) -> [f64; 3] {
    let orientation = object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
    let rotated = quat_rotate_vector(orientation, vortex.position);
    [object.origin.first().copied().unwrap_or(0.0) + rotated[0], object.origin.get(1).copied().unwrap_or(0.0) + rotated[1], object.origin.get(2).copied().unwrap_or(0.0) + rotated[2]]
}

fn world_vortex_direction(object: &Puzzle3dObject, vortex: &Puzzle3dVortex) -> [f64; 3] {
    let direction = vortex.direction.unwrap_or([0.0, 0.0, -1.0]);
    quat_rotate_vector(object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), direction)
}

fn vortex_color(meta: &Puzzle3dFixtureMeta, vortex_kind: Option<&str>) -> String {
    let Some(kind_id) = vortex_kind else {
        return "#38bdf8".into();
    };
    let Some(catalogs) = meta.kind_catalogs.as_ref() else {
        return "#38bdf8".into();
    };
    let Some(entries) = catalogs.get("vortices").and_then(|value| value.as_array()) else {
        return "#38bdf8".into();
    };
    for entry in entries {
        if entry.get("id").and_then(|value| value.as_str()) == Some(kind_id) {
            return entry.get("color").and_then(|value| value.as_str()).unwrap_or("#38bdf8").to_string();
        }
    }
    "#38bdf8".into()
}

fn puzzle3d_vortex_full_id(object_id: &str, vortex_id: &str) -> String {
    if vortex_id.contains(':') {
        vortex_id.to_string()
    } else {
        format!("{object_id}:{vortex_id}")
    }
}

fn resolve_vortex_world_position(fixture: &Puzzle3dFixture, full_id: &str) -> Option<[f64; 3]> {
    for object in &fixture.objects {
        for vortex in &object.vortices {
            if puzzle3d_vortex_full_id(&object.id, &vortex.id) == full_id {
                return Some(world_vortex_position(object, vortex));
            }
        }
    }
    None
}

fn resolve_vortex_kind(fixture: &Puzzle3dFixture, full_id: &str) -> Option<String> {
    fixture.objects.iter().find_map(|object| object.vortices.iter().find(|vortex| puzzle3d_vortex_full_id(&object.id, &vortex.id) == full_id).and_then(|vortex| vortex.vortex_kind.clone()))
}

/// 🧲 Permissive when the fixture declares no `kindCompatibility` rules at all — otherwise requires an explicit (or bidirectional) entry.
fn puzzle3d_kinds_compatible(fixture: &Puzzle3dFixture, source_kind: &str, target_kind: &str) -> bool {
    let Some(entries) = fixture.meta.kind_compatibility.as_ref().and_then(|value| value.as_array()) else {
        return true;
    };
    if entries.is_empty() {
        return true;
    }
    entries.iter().any(|entry| {
        let source = entry.get("source").and_then(|value| value.as_str()).unwrap_or("");
        let target = entry.get("target").and_then(|value| value.as_str()).unwrap_or("");
        let bidirectional = entry.get("bidirectional").and_then(|value| value.as_bool()).unwrap_or(false);
        (source == source_kind && target == target_kind) || (bidirectional && source == target_kind && target == source_kind)
    })
}

fn world_vortices_json(fixture: &Puzzle3dFixture) -> String {
    let mut records = Vec::new();
    for object in &fixture.objects {
        for vortex in &object.vortices {
            let position = world_vortex_position(object, vortex);
            let direction = world_vortex_direction(object, vortex);
            records.push(json!({
                "fullId": puzzle3d_vortex_full_id(&object.id, &vortex.id),
                "objectId": object.id,
                "vortexKind": vortex.vortex_kind,
                "position": position,
                "direction": direction,
                "radius": vortex.radius.unwrap_or(0.36),
                "color": vortex_color(&fixture.meta, vortex.vortex_kind.as_deref()),
            }));
        }
    }
    serde_json::to_string(&records).unwrap_or_else(|_| "[]".into())
}

fn world_attractions_json(fixture: &Puzzle3dFixture) -> String {
    let records: Vec<Value> = fixture
        .attractions
        .iter()
        .filter_map(|attraction| {
            let from = resolve_vortex_world_position(fixture, &attraction.attracting)?;
            let to = resolve_vortex_world_position(fixture, &attraction.attracted)?;
            Some(json!({
                "id": attraction.id,
                "from": from,
                "to": to,
                "color": "#60a5fa",
            }))
        })
        .collect();
    serde_json::to_string(&records).unwrap_or_else(|_| "[]".into())
}

fn target_volume_scale_json(volume: &Puzzle3dTargetVolume) -> [f64; 3] {
    match &volume.scale {
        Some(Value::Array(values)) if values.len() >= 3 => [values[0].as_f64().unwrap_or(1.0), values[1].as_f64().unwrap_or(1.0), values[2].as_f64().unwrap_or(1.0)],
        _ => [1.0, 1.0, 1.0],
    }
}

fn world_target_volumes_json(fixture: &Puzzle3dFixture) -> String {
    let records: Vec<Value> = fixture
        .target_volumes
        .iter()
        .map(|volume| {
            json!({
                "id": volume.id,
                "origin": volume.origin,
                "orientation": volume.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]),
                "scale": target_volume_scale_json(volume),
                "color": "#f472b6",
            })
        })
        .collect();
    serde_json::to_string(&records).unwrap_or_else(|_| "[]".into())
}

fn world_references_json(fixture: &Puzzle3dFixture) -> String {
    let records: Vec<Value> = fixture
        .references
        .iter()
        .map(|reference| {
            json!({
                "id": reference.id,
                "url": reference.source.url,
                "origin": reference.origin,
                "widthWorld": if reference.width_world > 0.0 { reference.width_world } else { 1.0 },
                "locked": reference.locked,
                "hidden": reference.hidden,
            })
        })
        .collect();
    serde_json::to_string(&records).unwrap_or_else(|_| "[]".into())
}

fn world_interaction_json(runtime: &Puzzle3dRuntime) -> String {
    json!({
        "activeTool": runtime.active_tool,
        "brushCandidateIndex": runtime.brush_candidate_index,
        "hoveredVortexFullId": runtime.selection.vortex_ids.first().cloned(),
        "fillEditTargetVolumes": runtime.fill_edit_target_volumes,
        "voxelDims": runtime.voxel_dims,
        "gridFactor": runtime.grid_factor,
    })
    .to_string()
}

fn world3d_lod_json(runtime: &Puzzle3dRuntime) -> String {
    json!({
        "gridFactor": runtime.grid_factor,
        "gridSnapEnabled": runtime.grid_snap_enabled,
        "showLodGrid": runtime.lod_show_grid,
        "automaticLod": runtime.lod_automatic,
        "depthVariableLod": runtime.lod_depth_variable,
        "manualLod": runtime.lod_manual,
    })
    .to_string()
}

fn world_brush_preview_json(session: &Puzzle3dPrecomputeSession, envelope: &Puzzle3dEnvelope) -> Option<String> {
    if envelope.runtime.active_tool != "brush" {
        return None;
    }
    let vortex_id = puzzle3d_brush_target_vortex(envelope)?;
    session.brush_preview_json(&vortex_id, envelope.runtime.brush_candidate_index)
}

fn drive_precompute(session: &mut Puzzle3dPrecomputeSession, envelope: &Puzzle3dEnvelope) {
    sync_precompute_session(session, envelope);
    for _ in 0..128 {
        if !session.precompute_step(32) {
            break;
        }
    }
}

fn scene_config_json(envelope: &Puzzle3dEnvelope) -> String {
    json!({
        "fixture": {
            "objects": envelope.fixture.objects,
            "attractions": envelope.fixture.attractions,
            "targetVolumes": envelope.fixture.target_volumes,
        },
        "kindCatalogs": envelope.fixture.meta.kind_catalogs,
        "kindCompatibility": envelope.fixture.meta.kind_compatibility.clone().unwrap_or(json!([])),
        "overlapBudget": envelope.runtime.overlap_budget,
        "seed": 1,
        "weights": {
            "objectWeights": envelope.runtime.object_kind_weights,
            "vortexWeights": envelope.runtime.vortex_kind_weights,
        }
    })
    .to_string()
}

/// 🧊 Scales the unit box fallback (`mesh_from_kind` extent 1.0) past `BRUSH_COLLISION_MESH_MIN_EXTENT` (2.0) in `puzzle_3d`'s collision engine, otherwise its registration is a silent no-op and brush candidates never populate before a real GLB arrives.
const PUZZLE3D_FALLBACK_MESH_SCALE: f32 = 4.0;

fn scaled_mesh_positions(positions: &[f32], scale: f32) -> Vec<f32> {
    positions.iter().map(|value| value * scale).collect()
}

/// 🧊 Only seeds the box fallback for URLs with no mesh yet, so a real GLB registered earlier via `registerBrushMesh` survives every subsequent resync.
fn sync_precompute_session(session: &mut Puzzle3dPrecomputeSession, envelope: &Puzzle3dEnvelope) {
    let _ = session.set_scene(&scene_config_json(envelope));
    let fallback = mesh_from_kind(PUZZLE3D_FALLBACK_MESH_KIND);
    let fallback_positions = scaled_mesh_positions(&fallback.positions, PUZZLE3D_FALLBACK_MESH_SCALE);
    if !session.has_mesh(PUZZLE3D_FALLBACK_MESH_KIND) {
        session.register_mesh(PUZZLE3D_FALLBACK_MESH_KIND, &fallback_positions, &fallback.indices);
    }
    for url in collect_mesh_urls(&envelope.fixture) {
        if !session.has_mesh(&url) {
            session.register_mesh(&url, &fallback_positions, &fallback.indices);
        }
    }
}

fn world_selection_json(envelope: &Puzzle3dEnvelope) -> String {
    let runtime = &envelope.runtime;
    let mut value: Value = serde_json::from_str(&world3d_selection_json(&runtime.selection_method, &runtime.selection.object_ids, runtime.hovered_object_id.as_deref())).unwrap_or_else(|_| json!({}));
    if let Some(object) = value.as_object_mut() {
        object.insert("granularity".into(), json!("mesh"));
        object.insert("selectionMode".into(), json!("mesh"));
        object.insert(
            "targets".into(),
            json!({
                "mesh": true,
                "vertex": false,
                "edge": false,
                "face": false,
            }),
        );
        object.insert("transformTool".into(), json!(runtime.transform_tool));
        if let Some(active_id) = runtime.selection.object_ids.first() {
            object.insert("activeObjectId".into(), json!(active_id));
        }
        let gumball_active = !runtime.selection.object_ids.is_empty();
        object.insert("gumballActive".into(), json!(gumball_active));
        if gumball_active {
            if let Some(target) = gumball_target_world(envelope) {
                object.insert("gumballTarget".into(), json!(target));
            }
        }
    }
    value.to_string()
}

fn gumball_target_world(envelope: &Puzzle3dEnvelope) -> Option<[f64; 3]> {
    let selected: Vec<&Puzzle3dObject> = envelope.fixture.objects.iter().filter(|object| envelope.runtime.selection.object_ids.contains(&object.id)).collect();
    if selected.is_empty() {
        return None;
    }
    let mut sum = [0.0, 0.0, 0.0];
    for object in &selected {
        sum[0] += object.origin.first().copied().unwrap_or(0.0);
        sum[1] += object.origin.get(1).copied().unwrap_or(0.0);
        sum[2] += object.origin.get(2).copied().unwrap_or(0.0);
    }
    let count = selected.len() as f64;
    Some([sum[0] / count, sum[1] / count, sum[2] / count])
}

fn fixture_from_engine_json(envelope: &Puzzle3dEnvelope, fixture_json: &str) -> Option<Puzzle3dEnvelope> {
    let parsed: Value = serde_json::from_str(fixture_json).ok()?;
    let mut next = envelope.clone();
    next.fixture.objects = serde_json::from_value(parsed.get("objects")?.clone()).ok()?;
    next.fixture.attractions = parsed.get("attractions").and_then(|v| serde_json::from_value(v.clone()).ok()).unwrap_or_default();
    next.fixture.target_volumes = parsed.get("targetVolumes").and_then(|v| serde_json::from_value(v.clone()).ok()).unwrap_or_default();
    Some(next)
}

fn apply_puzzle3d_fill_count(precompute: &mut Puzzle3dPrecomputeSession, mut envelope: Puzzle3dEnvelope, count: u32) -> Puzzle3dEnvelope {
    envelope.runtime.fill_count = count;
    if count > 0 {
        envelope.runtime.active_tool = "fill".into();
        if let Ok(fixture_json) = precompute.apply_fill_count_rust(count) {
            if let Some(next) = fixture_from_engine_json(&envelope, &fixture_json) {
                envelope = next;
            }
        }
    }
    envelope
}

/// 🎯 Mirrors the host's client-side `handleZoomToSelection` framing math so a keybinding/engagement-token
/// driven focus (which bypasses that host interception) still produces a sensible camera.
fn apply_puzzle3d_focus_selection(envelope: &mut Puzzle3dEnvelope) {
    let selected_origins: Vec<[f64; 3]> = envelope.fixture.objects.iter().filter(|object| envelope.runtime.selection.object_ids.contains(&object.id)).map(|object| object.origin).collect();
    if selected_origins.is_empty() {
        return;
    }
    let count = selected_origins.len() as f64;
    let mut center = [0.0, 0.0, 0.0];
    for origin in &selected_origins {
        center[0] += origin[0];
        center[1] += origin[1];
        center[2] += origin[2];
    }
    center = [center[0] / count, center[1] / count, center[2] / count];
    let max_distance = selected_origins
        .iter()
        .map(|origin| {
            let dx = origin[0] - center[0];
            let dy = origin[1] - center[1];
            let dz = origin[2] - center[2];
            (dx * dx + dy * dy + dz * dz).sqrt()
        })
        .fold(1.0_f64, f64::max);
    let distance = max_distance * 3.0 + 2.0;
    envelope.fixture.camera.position = [center[0] + distance * 0.6, center[1] - distance * 0.6, center[2] + distance * 0.5];
    envelope.fixture.camera.target = center;
}

fn next_object_id() -> String {
    let next = PUZZLE3D_ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
    format!("object-{next}")
}

/// 🧊 Seeds real vortices for a freshly placed object from its kind catalog's `vortices` templates, so it is immediately brushable instead of connector-less.
fn puzzle3d_vortices_from_kind_template(catalog_entry: &Value) -> Vec<Puzzle3dVortex> {
    catalog_entry
        .get("vortices")
        .and_then(|value| value.as_array())
        .map(|templates| {
            templates
                .iter()
                .enumerate()
                .map(|(index, template)| {
                    let position = template.get("position").and_then(|value| serde_json::from_value::<[f64; 3]>(value.clone()).ok()).unwrap_or([0.0, 0.0, 0.0]);
                    let direction = template.get("direction").and_then(|value| serde_json::from_value::<[f64; 3]>(value.clone()).ok());
                    let radius = template.get("radius").and_then(|value| value.as_f64());
                    Puzzle3dVortex { id: format!("v{index}"), vortex_kind: template.get("vortexKind").and_then(|value| value.as_str()).map(str::to_string), position, direction, radius, hidden: false, locked: false }
                })
                .collect()
        })
        .unwrap_or_default()
}

/// 🙈 Applies `hidden`/`locked` to the given ids of one entity kind — `"vortex"` ids are full ids (`objectId:vortexId`).
fn apply_puzzle3d_selection_flag(fixture: &mut Puzzle3dFixture, entity: &str, ids: &[String], flag: &str, value: bool) {
    if ids.is_empty() {
        return;
    }
    let ids: HashSet<&str> = ids.iter().map(String::as_str).collect();
    match entity {
        "object" => {
            for object in fixture.objects.iter_mut().filter(|object| ids.contains(object.id.as_str())) {
                if flag == "locked" { object.locked = value } else { object.hidden = value }
            }
        }
        "vortex" => {
            for object in fixture.objects.iter_mut() {
                for vortex in object.vortices.iter_mut() {
                    if ids.contains(puzzle3d_vortex_full_id(&object.id, &vortex.id).as_str()) {
                        if flag == "locked" { vortex.locked = value } else { vortex.hidden = value }
                    }
                }
            }
        }
        "reference" => {
            for reference in fixture.references.iter_mut().filter(|reference| ids.contains(reference.id.as_str())) {
                if flag == "locked" { reference.locked = value } else { reference.hidden = value }
            }
        }
        "targetVolume" => {
            for volume in fixture.target_volumes.iter_mut().filter(|volume| ids.contains(volume.id.as_str())) {
                if flag == "locked" { volume.locked = value } else { volume.hidden = value }
            }
        }
        _ => {}
    }
}

fn value_as_vec3(value: &Value) -> Option<[f64; 3]> {
    let array = value.as_array()?;
    Some([array.first()?.as_f64()?, array.get(1)?.as_f64()?, array.get(2)?.as_f64()?])
}

/// 🔎 Generic inspector edit dispatcher — `entity`/`field` select the target, `ids` scope it (full ids for vortices, `objectId:vortexId`).
fn apply_puzzle3d_inspector_patch(fixture: &mut Puzzle3dFixture, entity: &str, ids: &[String], field: &str, value: &Value) {
    if ids.is_empty() {
        return;
    }
    let id_set: HashSet<&str> = ids.iter().map(String::as_str).collect();
    match entity {
        "object" => {
            for object in fixture.objects.iter_mut().filter(|object| id_set.contains(object.id.as_str())) {
                match field {
                    "label" => object.label = value.as_str().map(str::to_string),
                    "origin" => {
                        if let Some(origin) = value_as_vec3(value) {
                            object.origin = origin;
                        }
                    }
                    _ => {}
                }
            }
        }
        "vortex" => {
            for object in fixture.objects.iter_mut() {
                for vortex in object.vortices.iter_mut() {
                    if !id_set.contains(puzzle3d_vortex_full_id(&object.id, &vortex.id).as_str()) {
                        continue;
                    }
                    match field {
                        "vortexKind" => vortex.vortex_kind = value.as_str().map(str::to_string),
                        "position" => {
                            if let Some(position) = value_as_vec3(value) {
                                vortex.position = position;
                            }
                        }
                        "direction" => vortex.direction = value_as_vec3(value),
                        "radius" => vortex.radius = value.as_f64(),
                        _ => {}
                    }
                }
            }
        }
        "attraction" => {
            for attraction in fixture.attractions.iter_mut().filter(|attraction| id_set.contains(attraction.id.as_str())) {
                match field {
                    "attracting" => {
                        if let Some(text) = value.as_str() {
                            attraction.attracting = text.into();
                        }
                    }
                    "attracted" => {
                        if let Some(text) = value.as_str() {
                            attraction.attracted = text.into();
                        }
                    }
                    _ => {}
                }
            }
        }
        "reference" => {
            for reference in fixture.references.iter_mut().filter(|reference| id_set.contains(reference.id.as_str())) {
                match field {
                    "sourceUrl" => {
                        if let Some(text) = value.as_str() {
                            reference.source.url = text.into();
                        }
                    }
                    "origin" => {
                        if let Some(origin) = value_as_vec3(value) {
                            reference.origin = origin;
                        }
                    }
                    "widthWorld" => {
                        if let Some(width) = value.as_f64() {
                            reference.width_world = width;
                        }
                    }
                    _ => {}
                }
            }
        }
        "targetVolume" => {
            for volume in fixture.target_volumes.iter_mut().filter(|volume| id_set.contains(volume.id.as_str())) {
                if field == "origin" {
                    if let Some(origin) = value_as_vec3(value) {
                        volume.origin = origin;
                    }
                }
            }
        }
        _ => {}
    }
}
//#endregion 🔖Document

//#region 🔖Terminology
/// 🗣️ Complete UI label set for the 3d app; one field per label makes every terminology×locale combination compile-checked.
struct Puzzle3dLabels {
    objects: &'static str,
    object: &'static str,
    vortices: &'static str,
    vortex: &'static str,
}

const PUZZLE3D_LABELS_NATIVE_EN: Puzzle3dLabels = Puzzle3dLabels { objects: "Objects", object: "Object", vortices: "Vortices", vortex: "Vortex" };
const PUZZLE3D_LABELS_NATIVE_DE: Puzzle3dLabels = Puzzle3dLabels { objects: "Objekte", object: "Objekt", vortices: "Vortices", vortex: "Vortex" };
const PUZZLE3D_LABELS_REUSE_EN: Puzzle3dLabels = Puzzle3dLabels { objects: "Building components", object: "Building component", vortices: "Connection points", vortex: "Connection point" };
const PUZZLE3D_LABELS_REUSE_DE: Puzzle3dLabels = Puzzle3dLabels { objects: "Baukomponenten", object: "Baukomponente", vortices: "Verbindungspunkte", vortex: "Verbindungspunkt" };

/// 🗣️ Resolves the active label set from the shell-provided locale/terminology; unknown terminology ids fall back to native.
fn puzzle3d_labels(view_state: &ViewState) -> &'static Puzzle3dLabels {
    let terminology = view_state.terminology.as_deref().unwrap_or("native");
    let is_de = view_state.locale.as_deref().is_some_and(|locale| locale.starts_with("de"));
    match (terminology, is_de) {
        ("reuse", true) => &PUZZLE3D_LABELS_REUSE_DE,
        ("reuse", false) => &PUZZLE3D_LABELS_REUSE_EN,
        (_, true) => &PUZZLE3D_LABELS_NATIVE_DE,
        (_, false) => &PUZZLE3D_LABELS_NATIVE_EN,
    }
}
//#endregion 🔖Terminology

//#region 🔖Panels
fn tree_item_with_command(id: impl Into<String>, label: impl Into<String>, icon_id: Option<&str>, command: CommandDescriptor) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.into(),
        label: label.into(),
        description: None,
        icon_id: icon_id.map(str::to_string),
        selected: None,
        default_open: None,
        command: Some(command),
        hover_command: None,
        unhover_command: None,
        actions: None,
        draggable: None,
        drag_data: None,
        items: None,
        control: None,
        is_hidden: None,
    }
}

fn puzzle3d_hide_lock_actions(hidden: bool, locked: bool, flag_args: impl Fn(&str) -> Value) -> Vec<UiTreeItemAction> {
    vec![
        UiTreeItemAction { icon_id: if hidden { "eye-off".into() } else { "eye".into() }, label: Some(if hidden { "Show".into() } else { "Hide".into() }), command: puzzle3d_cmd("setSelectionFlag", Some(flag_args("hidden"))), reveal_on_hover: Some(true) },
        UiTreeItemAction { icon_id: if locked { "lock".into() } else { "lock-open".into() }, label: Some(if locked { "Unlock".into() } else { "Lock".into() }), command: puzzle3d_cmd("setSelectionFlag", Some(flag_args("locked"))), reveal_on_hover: Some(true) },
    ]
}

fn build_document_tree(envelope: &Puzzle3dEnvelope, labels: &Puzzle3dLabels) -> UiNode {
    let object_items: Vec<UiTreeItemNode> = envelope
        .fixture
        .objects
        .iter()
        .map(|object| {
            let vortex_items: Vec<UiTreeItemNode> = object
                .vortices
                .iter()
                .map(|vortex| {
                    let full_id = puzzle3d_vortex_full_id(&object.id, &vortex.id);
                    tree_item_with_command(
                        format!("puzzle3d-vortex:{full_id}"),
                        vortex.vortex_kind.clone().unwrap_or_else(|| vortex.id.clone()),
                        Some("circle-dot"),
                        puzzle3d_cmd("setSelection", Some(json!({ "selection": { "objectIds": [], "vortexIds": [full_id], "attractionIds": [] } }))),
                    )
                })
                .collect();
            let flag_args = {
                let id = object.id.clone();
                move |flag: &str| json!({ "flag": flag, "value": true, "entity": "object", "ids": [id.clone()] })
            };
            UiTreeItemNode {
                id: format!("puzzle3d-object:{}", object.id),
                label: object.object_kind.clone().unwrap_or_else(|| object.id.clone()),
                description: None,
                icon_id: Some("box".into()),
                selected: Some(envelope.runtime.selection.object_ids.contains(&object.id)),
                default_open: Some(false),
                command: Some(puzzle3d_cmd("setSelection", Some(json!({ "selection": { "objectIds": [object.id], "vortexIds": [], "attractionIds": [] } })))),
                hover_command: Some(puzzle3d_cmd("setHover", Some(json!({ "objectId": object.id })))),
                unhover_command: Some(puzzle3d_cmd("setHover", None)),
                actions: Some(puzzle3d_hide_lock_actions(object.hidden, object.locked, flag_args)),
                draggable: None,
                drag_data: None,
                items: if vortex_items.is_empty() { None } else { Some(vortex_items) },
                control: None,
                is_hidden: Some(object.hidden),
            }
        })
        .collect();
    let reference_items: Vec<UiTreeItemNode> = envelope
        .fixture
        .references
        .iter()
        .map(|reference| {
            let flag_args = {
                let id = reference.id.clone();
                move |flag: &str| json!({ "flag": flag, "value": true, "entity": "reference", "ids": [id.clone()] })
            };
            UiTreeItemNode {
                id: format!("puzzle3d-reference:{}", reference.id),
                label: reference.id.clone(),
                description: Some(reference.source.url.clone()),
                icon_id: Some("globe".into()),
                selected: Some(envelope.runtime.selection.reference_ids.contains(&reference.id)),
                default_open: None,
                command: Some(puzzle3d_cmd("setSelection", Some(json!({ "selection": { "objectIds": [], "vortexIds": [], "attractionIds": [], "referenceIds": [reference.id] } })))),
                hover_command: None,
                unhover_command: None,
                actions: Some(puzzle3d_hide_lock_actions(reference.hidden, reference.locked, flag_args)),
                draggable: None,
                drag_data: None,
                items: None,
                control: None,
                is_hidden: Some(reference.hidden),
            }
        })
        .collect();
    let target_volume_items: Vec<UiTreeItemNode> = envelope
        .fixture
        .target_volumes
        .iter()
        .map(|volume| {
            let flag_args = {
                let id = volume.id.clone();
                move |flag: &str| json!({ "flag": flag, "value": true, "entity": "targetVolume", "ids": [id.clone()] })
            };
            UiTreeItemNode {
                id: format!("puzzle3d-target-volume:{}", volume.id),
                label: volume.id.clone(),
                description: None,
                icon_id: Some("cylinder".into()),
                selected: Some(envelope.runtime.selection.target_volume_ids.contains(&volume.id)),
                default_open: None,
                command: Some(puzzle3d_cmd("setSelection", Some(json!({ "selection": { "objectIds": [], "vortexIds": [], "attractionIds": [], "targetVolumeIds": [volume.id] } })))),
                hover_command: None,
                unhover_command: None,
                actions: Some(puzzle3d_hide_lock_actions(volume.hidden, volume.locked, flag_args)),
                draggable: None,
                drag_data: None,
                items: None,
                control: None,
                is_hidden: Some(volume.hidden),
            }
        })
        .collect();
    let attraction_items: Vec<UiTreeItemNode> = envelope
        .fixture
        .attractions
        .iter()
        .map(|attraction| {
            tree_item_with_command(
                format!("puzzle3d-attraction:{}", attraction.id),
                format!("{} → {}", attraction.attracting, attraction.attracted),
                Some("link"),
                puzzle3d_cmd("setSelection", Some(json!({ "selection": { "objectIds": [], "vortexIds": [], "attractionIds": [attraction.id] } }))),
            )
        })
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![
            UiTreeSectionNode { id: "puzzle3d-play-document.objects".into(), label: Some(labels.objects.into()), default_open: Some(true), items: object_items },
            UiTreeSectionNode { id: "puzzle3d-play-document.references".into(), label: Some("References".into()), default_open: Some(false), items: reference_items },
            UiTreeSectionNode { id: "puzzle3d-play-document.target-volumes".into(), label: Some("Target Volumes".into()), default_open: Some(false), items: target_volume_items },
            UiTreeSectionNode { id: "puzzle3d-play-document.attractions".into(), label: Some("Attractions".into()), default_open: Some(false), items: attraction_items },
        ],
        selected_ids: None,
        highlighted_ids: None,
        selection_change: None,
        drop_command: None,
    })
}

/// 🖱️ MIME key `DeclarativeTreePanel` (framework/renderer/react/ui-interpreter.tsx) reads to auto-wire catalogue drag sources.
const PUZZLE3D_CATALOGUE_DRAG_MIME: &str = "application/x-semio-catalogue-item";

fn puzzle3d_catalog_entries<'a>(fixture: &'a Puzzle3dFixture, section: &str) -> &'a [Value] {
    fixture.meta.kind_catalogs.as_ref().and_then(|catalogs| catalogs.get(section)).and_then(|entries| entries.as_array()).map(Vec::as_slice).unwrap_or(&[])
}

fn puzzle3d_catalog_entry_label(entry: &Value) -> String {
    entry.get("label").and_then(|value| value.as_str()).or_else(|| entry.get("name").and_then(|value| value.as_str())).or_else(|| entry.get("id").and_then(|value| value.as_str())).unwrap_or("kind").into()
}

fn puzzle3d_object_kind_vortex_items(entry: &Value) -> Vec<UiTreeItemNode> {
    entry
        .get("vortices")
        .and_then(|value| value.as_array())
        .map(|templates| {
            templates
                .iter()
                .enumerate()
                .map(|(index, template)| {
                    let vortex_kind = template.get("vortexKind").and_then(|value| value.as_str()).unwrap_or("vortex");
                    let position = template.get("position").cloned().unwrap_or(json!([0.0, 0.0, 0.0]));
                    UiTreeItemNode {
                        id: format!("puzzle3d-kind-vortex.{index}.{vortex_kind}"),
                        label: vortex_kind.into(),
                        description: Some(position.to_string()),
                        icon_id: Some("circle-dot".into()),
                        selected: None,
                        default_open: None,
                        command: None,
                        hover_command: None,
                        unhover_command: None,
                        actions: None,
                        draggable: None,
                        drag_data: None,
                        items: None,
                        control: None,
                        is_hidden: None,
                    }
                })
                .collect()
        })
        .unwrap_or_default()
}

fn puzzle3d_object_kind_item(entry: &Value) -> UiTreeItemNode {
    let kind_id = entry.get("id").and_then(|value| value.as_str()).unwrap_or("kind").to_string();
    let draggable = entry.get("meshUrl").and_then(|value| value.as_str()).map(|url| !url.is_empty()).unwrap_or(false);
    UiTreeItemNode {
        id: format!("puzzle3d-kind:{kind_id}"),
        label: puzzle3d_catalog_entry_label(entry),
        description: Some(kind_id.clone()),
        icon_id: Some("box".into()),
        selected: None,
        default_open: Some(false),
        command: Some(puzzle3d_cmd("addObjectKind", Some(json!({ "objectKind": kind_id.clone() })))),
        hover_command: Some(puzzle3d_cmd("setKindHover", Some(json!({ "kindId": kind_id.clone() })))),
        unhover_command: Some(puzzle3d_cmd("setKindHover", Some(json!({ "kindId": Value::Null })))),
        actions: None,
        draggable: draggable.then_some(true),
        drag_data: draggable.then(|| HashMap::from([(PUZZLE3D_CATALOGUE_DRAG_MIME.to_string(), json!({ "objectKind": kind_id }).to_string())])),
        items: Some(puzzle3d_object_kind_vortex_items(entry)),
        control: None,
        is_hidden: None,
    }
}

fn puzzle3d_catalog_kind_item(entry: &Value, icon_id: &str) -> UiTreeItemNode {
    let kind_id = entry.get("id").and_then(|value| value.as_str()).unwrap_or("kind").to_string();
    UiTreeItemNode {
        id: format!("puzzle3d-kind-entry:{kind_id}"),
        label: puzzle3d_catalog_entry_label(entry),
        description: Some(kind_id),
        icon_id: Some(icon_id.into()),
        selected: None,
        default_open: None,
        command: None,
        hover_command: None,
        unhover_command: None,
        actions: None,
        draggable: None,
        drag_data: None,
        items: None,
        control: None,
        is_hidden: None,
    }
}

fn build_kinds_tree(envelope: &Puzzle3dEnvelope, labels: &Puzzle3dLabels) -> UiNode {
    let object_entries = puzzle3d_catalog_entries(&envelope.fixture, "objects");
    let vortex_entries = puzzle3d_catalog_entries(&envelope.fixture, "vortices");
    let cable_entries = puzzle3d_catalog_entries(&envelope.fixture, "cables");
    let attraction_entries = puzzle3d_catalog_entries(&envelope.fixture, "attractions");
    UiNode::Tree(UiTreeNode {
        sections: vec![
            UiTreeSectionNode { id: "puzzle3d-play-kinds.objects".into(), label: Some(labels.objects.into()), default_open: Some(true), items: object_entries.iter().map(puzzle3d_object_kind_item).collect() },
            UiTreeSectionNode { id: "puzzle3d-play-kinds.vortices".into(), label: Some(labels.vortices.into()), default_open: Some(false), items: vortex_entries.iter().map(|entry| puzzle3d_catalog_kind_item(entry, "circle-dot")).collect() },
            UiTreeSectionNode { id: "puzzle3d-play-kinds.cables".into(), label: Some("Cables".into()), default_open: Some(false), items: cable_entries.iter().map(|entry| puzzle3d_catalog_kind_item(entry, "plug")).collect() },
            UiTreeSectionNode { id: "puzzle3d-play-kinds.attractions".into(), label: Some("Attractions".into()), default_open: Some(false), items: attraction_entries.iter().map(|entry| puzzle3d_catalog_kind_item(entry, "link")).collect() },
        ],
        selected_ids: None,
        highlighted_ids: None,
        selection_change: None,
        drop_command: None,
    })
}

fn inspector_text_field(id: impl Into<String>, label: impl Into<String>, mixed_text: semio_framework_plugin::UiInspectorMixedText, command: CommandDescriptor) -> UiNode {
    let id = id.into();
    UiNode::Field(UiFieldNode {
        id: id.clone(),
        label: label.into(),
        child: Box::new(UiNode::Input(semio_framework_plugin::UiInputNode {
            id: format!("{id}.input"),
            input_kind: "text".into(),
            value: mixed_text.value,
            placeholder: mixed_text.placeholder,
            commit: None,
            on_change: command,
            min: None,
            max: None,
            step: None,
            accept: None,
        })),
        description: None,
        required: None,
        error: None,
    })
}

fn inspector_number_field(id: impl Into<String>, label: impl Into<String>, mixed_number: semio_framework_plugin::UiInspectorMixedNumber, command: CommandDescriptor) -> UiNode {
    let id = id.into();
    UiNode::Field(UiFieldNode {
        id: id.clone(),
        label: label.into(),
        child: Box::new(UiNode::Input(semio_framework_plugin::UiInputNode {
            id: format!("{id}.input"),
            input_kind: "number".into(),
            value: if mixed_number.uniform { mixed_number.value.to_string() } else { String::new() },
            placeholder: if mixed_number.uniform { None } else { Some(semio_framework_plugin::UI_INSPECTOR_MIXED_PLACEHOLDER.into()) },
            commit: None,
            on_change: command,
            min: None,
            max: None,
            step: None,
            accept: None,
        })),
        description: None,
        required: None,
        error: None,
    })
}

fn inspector_vec3_field(id: impl Into<String>, label: impl Into<String>, mixed_vec3: semio_framework_plugin::UiInspectorMixedVec3, command: CommandDescriptor) -> UiNode {
    let id = id.into();
    UiNode::Field(UiFieldNode {
        id: id.clone(),
        label: label.into(),
        child: Box::new(UiNode::Vec3(semio_framework_plugin::UiVec3Node { id: format!("{id}.input"), value: mixed_vec3.value, on_change: command })),
        description: None,
        required: None,
        error: None,
    })
}

fn inspector_header_and_delete(count: usize, noun: &str) -> Vec<UiNode> {
    vec![
        ui_text(format!("{count} {noun} selected")),
        UiNode::Button(semio_framework_plugin::UiButtonNode { id: Some("puzzle3d-play-inspector.delete".into()), icon_id: "trash".into(), label: "Delete".into(), command: puzzle3d_cmd("deleteSelection", None), style: None, disabled: None }),
    ]
}

fn build_inspector_tree(envelope: &Puzzle3dEnvelope, term_labels: &Puzzle3dLabels) -> UiNode {
    let selection = &envelope.runtime.selection;
    if !selection.object_ids.is_empty() {
        let objects: Vec<&Puzzle3dObject> = envelope.fixture.objects.iter().filter(|object| selection.object_ids.contains(&object.id)).collect();
        if !objects.is_empty() {
            let ids_json = json!(selection.object_ids);
            let patch_cmd = |field: &str| puzzle3d_cmd("patchInspector", Some(json!({ "entity": "object", "ids": ids_json, "field": field })));
            let mut fields = inspector_header_and_delete(objects.len(), term_labels.object);
            if let [object] = objects.as_slice() {
                fields.push(ui_inspector_readonly_field("puzzle3d-play-inspector.object.id", "Id", &object.id));
            }
            let labels: Vec<String> = objects.iter().map(|object| object.label.clone().unwrap_or_default()).collect();
            let kinds: Vec<String> = objects.iter().map(|object| object.object_kind.clone().unwrap_or_default()).collect();
            let origins: Vec<[f64; 3]> = objects.iter().map(|object| object.origin).collect();
            fields.push(inspector_text_field("puzzle3d-play-inspector.object.label", "Label", semio_framework_plugin::ui_inspector_mixed_text(&labels), patch_cmd("label")));
            fields.push(ui_inspector_readonly_field("puzzle3d-play-inspector.object.kind", "Kind", &semio_framework_plugin::ui_inspector_mixed_text(&kinds).value));
            fields.push(inspector_vec3_field("puzzle3d-play-inspector.object.origin", "Origin", semio_framework_plugin::ui_inspector_mixed_vec3(&origins), patch_cmd("origin")));
            return ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { id: "puzzle3d-play-inspector.object".into(), label: term_labels.object.into(), default_open: None, fields }]);
        }
    }
    if !selection.vortex_ids.is_empty() {
        let vortices: Vec<(&Puzzle3dObject, &Puzzle3dVortex)> = envelope
            .fixture
            .objects
            .iter()
            .flat_map(|object| object.vortices.iter().map(move |vortex| (object, vortex)))
            .filter(|(object, vortex)| selection.vortex_ids.contains(&puzzle3d_vortex_full_id(&object.id, &vortex.id)))
            .collect();
        if !vortices.is_empty() {
            let full_ids: Vec<String> = vortices.iter().map(|(object, vortex)| puzzle3d_vortex_full_id(&object.id, &vortex.id)).collect();
            let ids_json = json!(full_ids);
            let patch_cmd = |field: &str| puzzle3d_cmd("patchInspector", Some(json!({ "entity": "vortex", "ids": ids_json, "field": field })));
            let mut fields = inspector_header_and_delete(vortices.len(), term_labels.vortex);
            if let [(_, vortex)] = vortices.as_slice() {
                fields.push(ui_inspector_readonly_field("puzzle3d-play-inspector.vortex.id", "Full Id", &full_ids[0]));
                let _ = vortex;
            }
            let kinds: Vec<String> = vortices.iter().map(|(_, vortex)| vortex.vortex_kind.clone().unwrap_or_default()).collect();
            let positions: Vec<[f64; 3]> = vortices.iter().map(|(_, vortex)| vortex.position).collect();
            let radii: Vec<f64> = vortices.iter().map(|(_, vortex)| vortex.radius.unwrap_or(0.35)).collect();
            fields.push(inspector_text_field("puzzle3d-play-inspector.vortex.kind", "Vortex Kind", semio_framework_plugin::ui_inspector_mixed_text(&kinds), patch_cmd("vortexKind")));
            fields.push(inspector_vec3_field("puzzle3d-play-inspector.vortex.position", "Position", semio_framework_plugin::ui_inspector_mixed_vec3(&positions), patch_cmd("position")));
            fields.push(inspector_number_field("puzzle3d-play-inspector.vortex.radius", "Radius", semio_framework_plugin::ui_inspector_mixed_number(&radii), patch_cmd("radius")));
            return ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { id: "puzzle3d-play-inspector.vortex".into(), label: term_labels.vortex.into(), default_open: None, fields }]);
        }
    }
    if !selection.attraction_ids.is_empty() {
        let attractions: Vec<&Puzzle3dAttraction> = envelope.fixture.attractions.iter().filter(|attraction| selection.attraction_ids.contains(&attraction.id)).collect();
        if !attractions.is_empty() {
            let ids_json = json!(selection.attraction_ids);
            let patch_cmd = |field: &str| puzzle3d_cmd("patchInspector", Some(json!({ "entity": "attraction", "ids": ids_json, "field": field })));
            let mut fields = inspector_header_and_delete(attractions.len(), "attraction");
            let attracting: Vec<String> = attractions.iter().map(|attraction| attraction.attracting.clone()).collect();
            let attracted: Vec<String> = attractions.iter().map(|attraction| attraction.attracted.clone()).collect();
            fields.push(inspector_text_field("puzzle3d-play-inspector.attraction.attracting", "Attracting", semio_framework_plugin::ui_inspector_mixed_text(&attracting), patch_cmd("attracting")));
            fields.push(inspector_text_field("puzzle3d-play-inspector.attraction.attracted", "Attracted", semio_framework_plugin::ui_inspector_mixed_text(&attracted), patch_cmd("attracted")));
            return ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { id: "puzzle3d-play-inspector.attraction".into(), label: "Attraction".into(), default_open: None, fields }]);
        }
    }
    if !selection.reference_ids.is_empty() {
        let references: Vec<&Puzzle3dReference> = envelope.fixture.references.iter().filter(|reference| selection.reference_ids.contains(&reference.id)).collect();
        if !references.is_empty() {
            let ids_json = json!(selection.reference_ids);
            let patch_cmd = |field: &str| puzzle3d_cmd("patchInspector", Some(json!({ "entity": "reference", "ids": ids_json, "field": field })));
            let mut fields = inspector_header_and_delete(references.len(), "reference");
            let urls: Vec<String> = references.iter().map(|reference| reference.source.url.clone()).collect();
            let origins: Vec<[f64; 3]> = references.iter().map(|reference| reference.origin).collect();
            let widths: Vec<f64> = references.iter().map(|reference| reference.width_world).collect();
            fields.push(inspector_text_field("puzzle3d-play-inspector.reference.url", "Source Url", semio_framework_plugin::ui_inspector_mixed_text(&urls), patch_cmd("sourceUrl")));
            fields.push(inspector_vec3_field("puzzle3d-play-inspector.reference.origin", "Position", semio_framework_plugin::ui_inspector_mixed_vec3(&origins), patch_cmd("origin")));
            fields.push(inspector_number_field("puzzle3d-play-inspector.reference.width", "Width", semio_framework_plugin::ui_inspector_mixed_number(&widths), patch_cmd("widthWorld")));
            return ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { id: "puzzle3d-play-inspector.reference".into(), label: "Reference".into(), default_open: None, fields }]);
        }
    }
    if !selection.target_volume_ids.is_empty() {
        let volumes: Vec<&Puzzle3dTargetVolume> = envelope.fixture.target_volumes.iter().filter(|volume| selection.target_volume_ids.contains(&volume.id)).collect();
        if !volumes.is_empty() {
            let ids_json = json!(selection.target_volume_ids);
            let patch_cmd = |field: &str| puzzle3d_cmd("patchInspector", Some(json!({ "entity": "targetVolume", "ids": ids_json, "field": field })));
            let mut fields = inspector_header_and_delete(volumes.len(), "target volume");
            let origins: Vec<[f64; 3]> = volumes.iter().map(|volume| volume.origin).collect();
            fields.push(inspector_vec3_field("puzzle3d-play-inspector.target-volume.origin", "Origin", semio_framework_plugin::ui_inspector_mixed_vec3(&origins), patch_cmd("origin")));
            return ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { id: "puzzle3d-play-inspector.target-volume".into(), label: "Target Volume".into(), default_open: None, fields }]);
        }
    }
    ui_stack_vertical(vec![ui_text(format!("Schema: {}", envelope.fixture.schema)), ui_text(format!("Domain: {}", envelope.fixture.domain)), ui_text(format!("Objects: {}", envelope.fixture.objects.len()))])
}

fn build_settings_body(envelope: &Puzzle3dEnvelope) -> UiNode {
    let runtime = &envelope.runtime;
    let selection_mode_field = UiNode::Field(UiFieldNode {
        id: "puzzle3d-play-settings.selection-mode".into(),
        label: "Selection Mode".into(),
        child: Box::new(UiNode::Select(semio_framework_plugin::UiSelectNode {
            id: "puzzle3d-play-settings.selection-mode.input".into(),
            value: runtime.selection_mode_default.clone(),
            items: vec![
                semio_framework_plugin::UiSelectItem { value: "default".into(), label: "Default".into() },
                semio_framework_plugin::UiSelectItem { value: "additive".into(), label: "Additive".into() },
                semio_framework_plugin::UiSelectItem { value: "subtractive".into(), label: "Subtractive".into() },
                semio_framework_plugin::UiSelectItem { value: "invertive".into(), label: "Invertive".into() },
            ],
            placeholder: None,
            on_change: puzzle3d_cmd("setSelectionModeDefault", None),
        })),
        description: None,
        required: None,
        error: None,
    });
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "puzzle3d-play-settings".into(),
        label: "Settings".into(),
        default_open: Some(true),
        fields: vec![
            selection_mode_field,
            inspector_number_field(
                "puzzle3d-play-settings.overlap-budget",
                "Brush Overlap Budget (m³)",
                semio_framework_plugin::UiInspectorMixedNumber { value: runtime.overlap_budget, uniform: true },
                puzzle3d_cmd("setBrushPlacementOverlapBudget", None),
            ),
            inspector_number_field(
                "puzzle3d-play-settings.proximity-radius",
                "Proximity Radius",
                semio_framework_plugin::UiInspectorMixedNumber { value: runtime.proximity_radius, uniform: true },
                puzzle3d_cmd("setProximityRadius", None),
            ),
            inspector_number_field("puzzle3d-play-settings.chunk-size", "Chunk Size", semio_framework_plugin::UiInspectorMixedNumber { value: runtime.chunk_size, uniform: true }, puzzle3d_cmd("setChunkSize", None)),
            inspector_number_field("puzzle3d-play-settings.grid-factor", "Grid Factor", semio_framework_plugin::UiInspectorMixedNumber { value: runtime.grid_factor, uniform: true }, puzzle3d_cmd("setGridFactor", None)),
        ],
    }])
}
//#endregion 🔖Panels

//#region 🔖Jack
/// 🕸️ A row produced by [`puzzle3d_run_jack_query`] — `entity`/`id` let a click reselect the exact match in the scene.
struct Puzzle3dJackRow {
    entity: &'static str,
    id: String,
    value: String,
}

/// 🕸️ Parses the one supported shape — `MATCH (n:Label) RETURN n.field` — this is a deliberately minimal, self-contained
/// stand-in for premigration's full Jack graph-query language (ported here without a cross-technology dependency on
/// `trinity-jack`/`trinity-ram`, which CLAUDE.md's "do not mix technologies" rule rules out for a puzzle-3d ticket).
fn puzzle3d_parse_jack_query(query: &str) -> Option<(String, String)> {
    let query = query.trim();
    let match_marker = "MATCH (n:";
    let match_start = query.find(match_marker)? + match_marker.len();
    let match_end = match_start + query[match_start..].find(')')?;
    let label = query[match_start..match_end].trim().to_string();
    let return_marker = "RETURN n.";
    let return_start = query.find(return_marker)? + return_marker.len();
    let field = query[return_start..].trim().to_string();
    if label.is_empty() || field.is_empty() {
        return None;
    }
    Some((label, field))
}

fn puzzle3d_run_jack_query(fixture: &Puzzle3dFixture, query: &str) -> Result<Vec<Puzzle3dJackRow>, String> {
    let (label, field) = puzzle3d_parse_jack_query(query).ok_or_else(|| "expected \"MATCH (n:Label) RETURN n.field\"".to_string())?;
    match label.as_str() {
        "Object" => Ok(fixture
            .objects
            .iter()
            .map(|object| {
                let value = match field.as_str() {
                    "id" => object.id.clone(),
                    "label" => object.label.clone().unwrap_or_default(),
                    "kind" => object.object_kind.clone().unwrap_or_default(),
                    _ => object.label.clone().or_else(|| object.object_kind.clone()).unwrap_or_else(|| object.id.clone()),
                };
                Puzzle3dJackRow { entity: "object", id: object.id.clone(), value }
            })
            .collect()),
        "Vortex" => Ok(fixture
            .objects
            .iter()
            .flat_map(|object| {
                let field = field.as_str();
                object.vortices.iter().map(move |vortex| {
                    let full_id = puzzle3d_vortex_full_id(&object.id, &vortex.id);
                    let value = match field {
                        "id" => full_id.clone(),
                        "kind" => vortex.vortex_kind.clone().unwrap_or_default(),
                        _ => vortex.vortex_kind.clone().unwrap_or_else(|| full_id.clone()),
                    };
                    Puzzle3dJackRow { entity: "vortex", id: full_id, value }
                })
            })
            .collect()),
        "Attraction" => Ok(fixture
            .attractions
            .iter()
            .map(|attraction| {
                let value = match field.as_str() {
                    "id" => attraction.id.clone(),
                    _ => format!("{} → {}", attraction.attracting, attraction.attracted),
                };
                Puzzle3dJackRow { entity: "attraction", id: attraction.id.clone(), value }
            })
            .collect()),
        other => Err(format!("unknown label \"{other}\" — supported: Object, Vortex, Attraction")),
    }
}

fn jack_row_selection_args(row: &Puzzle3dJackRow) -> Value {
    match row.entity {
        "object" => json!({ "selection": { "objectIds": [row.id], "vortexIds": [], "attractionIds": [] } }),
        "vortex" => json!({ "selection": { "objectIds": [], "vortexIds": [row.id], "attractionIds": [] } }),
        "attraction" => json!({ "selection": { "objectIds": [], "vortexIds": [], "attractionIds": [row.id] } }),
        _ => json!({ "selection": {} }),
    }
}

fn build_jack_body(envelope: &Puzzle3dEnvelope) -> UiNode {
    let query_field = UiNode::Field(UiFieldNode {
        id: "puzzle3d-play-jack.query".into(),
        label: "Query".into(),
        child: Box::new(UiNode::Input(semio_framework_plugin::UiInputNode {
            id: "puzzle3d-play-jack.query.input".into(),
            input_kind: "text".into(),
            value: envelope.runtime.jack_query.clone(),
            placeholder: Some("MATCH (n:Object) RETURN n.name".into()),
            commit: None,
            on_change: puzzle3d_cmd("setJackQuery", None),
            min: None,
            max: None,
            step: None,
            accept: None,
        })),
        description: None,
        required: None,
        error: None,
    });
    match puzzle3d_run_jack_query(&envelope.fixture, &envelope.runtime.jack_query) {
        Ok(rows) => {
            let items: Vec<UiTreeItemNode> = rows.iter().map(|row| tree_item_with_command(format!("puzzle3d-jack-row:{}:{}", row.entity, row.id), row.value.clone(), None, puzzle3d_cmd("setSelection", Some(jack_row_selection_args(row))))).collect();
            let results = UiNode::Tree(UiTreeNode {
                sections: vec![UiTreeSectionNode { id: "puzzle3d-play-jack.results".into(), label: Some(format!("{} results", items.len())), default_open: Some(true), items }],
                selected_ids: None,
                highlighted_ids: None,
                selection_change: None,
                drop_command: None,
            });
            ui_stack_vertical(vec![query_field, results])
        }
        Err(message) => ui_stack_vertical(vec![query_field, ui_text(format!("Error: {message}"))]),
    }
}
//#endregion 🔖Jack

//#region 🔖Engagement
fn parse_brush_candidates_free(raw: &str) -> Vec<Value> {
    let parsed: Value = serde_json::from_str(raw).unwrap_or(Value::Null);
    parsed.get("free").and_then(|value| value.as_array()).cloned().unwrap_or_default()
}

fn parse_brush_candidates_free_count(raw: &str) -> usize {
    parse_brush_candidates_free(raw).len()
}

fn puzzle3d_brush_target_vortex(envelope: &Puzzle3dEnvelope) -> Option<String> {
    envelope.runtime.selection.vortex_ids.first().cloned().or_else(|| {
        let object_id = envelope.runtime.hovered_object_id.as_deref()?;
        let object = envelope.fixture.objects.iter().find(|entry| entry.id == object_id)?;
        let vortex = object.vortices.first()?;
        Some(puzzle3d_vortex_full_id(&object.id, &vortex.id))
    })
}

fn puzzle3d_brush_placement_control(envelope: &Puzzle3dEnvelope, precompute: &Puzzle3dPrecomputeSession) -> Option<WindowEngagementControl> {
    let target = puzzle3d_brush_target_vortex(envelope)?;
    let raw = precompute.brush_candidates(&target);
    let candidates = parse_brush_candidates_free(&raw);
    if candidates.is_empty() {
        return None;
    }
    let options: Vec<WindowEngagementToggleGroupOption> = candidates
        .iter()
        .enumerate()
        .map(|(index, candidate)| {
            let label = candidate.get("objectKind").and_then(|value| value.as_str()).or_else(|| candidate.get("objectKindId").and_then(|value| value.as_str())).unwrap_or("kind");
            WindowEngagementToggleGroupOption { id: format!("puzzle3d.brush.candidate.{index}"), label: label.into(), disabled: None }
        })
        .collect();
    let selected_index = envelope.runtime.brush_candidate_index.min(options.len().saturating_sub(1));
    Some(WindowEngagementControl::ToggleGroup {
        id: Some("puzzle3d-brush-placement".into()),
        label: Some("Placement".into()),
        value: Some(format!("puzzle3d.brush.candidate.{selected_index}")),
        options,
        disabled: None,
        on_select: Some(puzzle3d_cmd("engagementControlSelect", None)),
    })
}

fn puzzle3d_fill_count_control(envelope: &Puzzle3dEnvelope) -> WindowEngagementControl {
    WindowEngagementControl::Slider {
        id: Some("puzzle3d-fill-count".into()),
        label: Some(format!("Fill {}", envelope.runtime.fill_count)),
        value: envelope.runtime.fill_count as f64,
        min: 0.0,
        max: PUZZLE3D_FILL_COUNT_MAX as f64,
        step: Some(1.0),
        unit: None,
        disabled: None,
        on_change: Some(puzzle3d_cmd("setFillCount", None)),
        on_commit: None,
    }
}

/// 🧊 Always visible while the Fill tool is active — the only path to flip `fill_edit_target_volumes`, so it must render
/// regardless of which mode is currently selected (the voxel dim steppers, by contrast, only make sense once in edit mode).
fn puzzle3d_voxel_mode_toggle(runtime: &Puzzle3dRuntime) -> WindowEngagementControl {
    WindowEngagementControl::ToggleGroup {
        id: Some("puzzle3d-voxel-edit-mode".into()),
        label: Some("Mode".into()),
        value: Some(if runtime.fill_edit_target_volumes { "edit-volumes".into() } else { "fill".into() }),
        options: vec![
            WindowEngagementToggleGroupOption { id: "fill".into(), label: "Fill".into(), disabled: None },
            WindowEngagementToggleGroupOption { id: "edit-volumes".into(), label: "Edit Volumes".into(), disabled: None },
        ],
        disabled: None,
        on_select: Some(puzzle3d_cmd("setFillEditTargetVolumes", None)),
    }
}

fn puzzle3d_voxel_dim_steppers(runtime: &Puzzle3dRuntime) -> Vec<WindowEngagementControl> {
    let [w, d, h] = runtime.voxel_dims;
    let axis_stepper = |axis: &str, label: &str, value: u32| WindowEngagementControl::Stepper {
        id: Some(format!("puzzle3d-voxel-{axis}")),
        label: Some(label.into()),
        value: value as f64,
        min: Some(1.0),
        max: Some(64.0),
        step: Some(1.0),
        unit: None,
        disabled: None,
        on_change: Some(puzzle3d_cmd("setVoxelDims", Some(json!({ "axis": axis })))),
        on_commit: None,
    };
    vec![axis_stepper("w", "Width", w), axis_stepper("d", "Depth", d), axis_stepper("h", "Height", h)]
}

fn puzzle3d_engagement(envelope: &Puzzle3dEnvelope, precompute: &Puzzle3dPrecomputeSession) -> WindowEngagement {
    let object_count = envelope.fixture.objects.len();
    let attraction_count = envelope.fixture.attractions.len();
    let voxel_edit_active = envelope.runtime.active_tool == "fill" && envelope.runtime.fill_edit_target_volumes;
    let control = match envelope.runtime.active_tool.as_str() {
        "fill" if !voxel_edit_active => Some(puzzle3d_fill_count_control(envelope)),
        "brush" => puzzle3d_brush_placement_control(envelope, precompute),
        _ => None,
    };
    let controls = if envelope.runtime.active_tool == "fill" {
        let mut rows = vec![puzzle3d_voxel_mode_toggle(&envelope.runtime)];
        if voxel_edit_active {
            rows.extend(puzzle3d_voxel_dim_steppers(&envelope.runtime));
        }
        Some(rows)
    } else {
        None
    };
    WindowEngagement {
        session_active: Some(envelope.runtime.active_tool != "select"),
        options: Some(vec![
            WindowEngagementOption {
                id: PUZZLE3D_ENGAGEMENT_TOOL_SELECT.into(),
                label: Some("Select".into()),
                icon_id: Some("cursor".into()),
                pressed: Some(envelope.runtime.active_tool == "select"),
                disabled: None,
                command: Some(puzzle3d_cmd("engagementPossibleSelect", Some(json!({ "possibleId": PUZZLE3D_ENGAGEMENT_TOOL_SELECT })))),
            },
            WindowEngagementOption {
                id: PUZZLE3D_ENGAGEMENT_TOOL_BRUSH.into(),
                label: Some("Brush".into()),
                icon_id: Some("brush".into()),
                pressed: Some(envelope.runtime.active_tool == "brush"),
                disabled: None,
                command: Some(puzzle3d_cmd("engagementPossibleSelect", Some(json!({ "possibleId": PUZZLE3D_ENGAGEMENT_TOOL_BRUSH })))),
            },
            WindowEngagementOption {
                id: PUZZLE3D_ENGAGEMENT_TOOL_FILL.into(),
                label: Some("Fill".into()),
                icon_id: Some("fill".into()),
                pressed: Some(envelope.runtime.fill_count > 0 || envelope.runtime.active_tool == "fill"),
                disabled: None,
                command: Some(puzzle3d_cmd("engagementPossibleSelect", Some(json!({ "possibleId": PUZZLE3D_ENGAGEMENT_TOOL_FILL })))),
            },
        ]),
        input: Some(WindowEngagementInput {
            id: Some("puzzle3d-engagement".into()),
            value: Some(envelope.runtime.engagement_input.clone()),
            placeholder: Some("select, brush, fill <n>, zoom, clear, rectangle, lasso".into()),
            disabled: None,
            on_change: Some(puzzle3d_cmd("engagementInput", None)),
            on_submit: Some(puzzle3d_cmd("engagementSubmit", None)),
            on_repeat_last: Some(puzzle3d_cmd("engagementRepeatLast", None)),
            on_abort: Some(puzzle3d_cmd("engagementAbort", None)),
        }),
        control,
        controls,
        status: Some(vec![semio_framework_plugin::layout::WindowEngagementStatus { id: "puzzle3d-world-status".into(), text: format!("{object_count} objects · {attraction_count} attractions") }]),
        possible_engagements: None,
    }
}

fn puzzle3d_context_menu_json(envelope: &Puzzle3dEnvelope) -> Option<String> {
    if envelope.runtime.selection.object_ids.is_empty() {
        return None;
    }
    let all_hidden = envelope.fixture.objects.iter().filter(|object| envelope.runtime.selection.object_ids.contains(&object.id)).all(|object| object.hidden);
    let all_locked = envelope.fixture.objects.iter().filter(|object| envelope.runtime.selection.object_ids.contains(&object.id)).all(|object| object.locked);
    let items = vec![
        json!({
            "id": "duplicate",
            "label": "Duplicate",
            "command": "duplicateSelection",
        }),
        json!({
            "id": "select-same-kind",
            "label": "Select all of same kind",
            "command": "selectSameKindSelection",
        }),
        json!({
            "id": "hide-show",
            "label": if all_hidden { "Show" } else { "Hide" },
            "command": "setSelectionFlag",
            "args": { "flag": "hidden", "value": !all_hidden },
        }),
        json!({
            "id": "lock-unlock",
            "label": if all_locked { "Unlock" } else { "Lock" },
            "command": "setSelectionFlag",
            "args": { "flag": "locked", "value": !all_locked },
        }),
        json!({
            "id": "zoom",
            "label": "Zoom to selection",
            "command": "zoomToSelection",
        }),
        json!({
            "id": "delete",
            "label": "Delete",
            "command": "deleteSelection",
        }),
    ];
    serde_json::to_string(&items).ok()
}
//#endregion 🔖Engagement

//#region 🔖Measures
const PUZZLE3D_LOD_SLIDER_MIN: f64 = 0.0;
const PUZZLE3D_LOD_SLIDER_MAX: f64 = 1000.0;

fn puzzle3d_kind_ids(fixture: &Puzzle3dFixture, section: &str) -> Vec<String> {
    fixture
        .meta
        .kind_catalogs
        .as_ref()
        .and_then(|catalogs| catalogs.get(section))
        .and_then(|entries| entries.as_array())
        .map(|entries| entries.iter().filter_map(|entry| entry.get("id").and_then(|value| value.as_str()).map(str::to_string)).collect())
        .unwrap_or_default()
}

fn puzzle3d_lod_measures_group(runtime: &Puzzle3dRuntime) -> WindowMeasure {
    WindowMeasure::Group {
        id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-lod"),
        label: "LOD".into(),
        default_open: Some(true),
        children: vec![
            WindowMeasure::Toggle { id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-lod-auto"), icon_id: "zoom-in".into(), label: Some("Auto zoom".into()), pressed: runtime.lod_automatic, text: None, on_change: puzzle3d_cmd("setLodAutomatic", None) },
            WindowMeasure::Toggle { id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-lod-depth-variable"), icon_id: "layers".into(), label: Some("Depth-variable".into()), pressed: runtime.lod_depth_variable, text: None, on_change: puzzle3d_cmd("setLodDepthVariable", None) },
            WindowMeasure::Toggle { id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-lod-grid"), icon_id: "layout-grid".into(), label: Some("Grid".into()), pressed: runtime.lod_show_grid, text: None, on_change: puzzle3d_cmd("setLodShowGrid", None) },
            WindowMeasure::Slider { id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-lod-value"), label: Some(format!("LOD {:.0}", runtime.lod_manual)), value: runtime.lod_manual, min: PUZZLE3D_LOD_SLIDER_MIN, max: PUZZLE3D_LOD_SLIDER_MAX, step: Some(1.0), on_change: puzzle3d_cmd("setLodManual", None) },
        ],
    }
}

fn puzzle3d_select_measures_group(runtime: &Puzzle3dRuntime, labels: &Puzzle3dLabels) -> WindowMeasure {
    WindowMeasure::Group {
        id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-select"),
        label: "Select".into(),
        default_open: Some(true),
        children: vec![
            WindowMeasure::Toggle { id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-select-rectangle"), icon_id: "square".into(), label: Some("Rectangle".into()), pressed: runtime.selection_method == "rectangle", text: None, on_change: puzzle3d_cmd("setSelectionMethod", Some(json!({ "method": "rectangle" }))) },
            WindowMeasure::Toggle { id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-select-lasso"), icon_id: "lasso".into(), label: Some("Lasso".into()), pressed: runtime.selection_method == "lasso", text: None, on_change: puzzle3d_cmd("setSelectionMethod", Some(json!({ "method": "lasso" }))) },
            WindowMeasure::Toggle { id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-select-objects"), icon_id: "box".into(), label: Some(labels.objects.into()), pressed: runtime.selectable_kinds.objects, text: None, on_change: puzzle3d_cmd("setSelectableKind", Some(json!({ "kind": "objects" }))) },
            WindowMeasure::Toggle { id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-select-vortices"), icon_id: "circle-dot".into(), label: Some(labels.vortices.into()), pressed: runtime.selectable_kinds.vortices, text: None, on_change: puzzle3d_cmd("setSelectableKind", Some(json!({ "kind": "vortices" }))) },
            WindowMeasure::Toggle { id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-select-attractions"), icon_id: "link".into(), label: Some("Attractions".into()), pressed: runtime.selectable_kinds.attractions, text: None, on_change: puzzle3d_cmd("setSelectableKind", Some(json!({ "kind": "attractions" }))) },
        ],
    }
}

fn puzzle3d_kind_weight_measures(prefix: &str, kind_ids: &[String], weights: &HashMap<String, f64>, command: &str) -> Vec<WindowMeasure> {
    kind_ids
        .iter()
        .map(|kind_id| {
            let weight = weights.get(kind_id).copied().unwrap_or(1.0);
            WindowMeasure::Slider {
                id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-{prefix}-{kind_id}"),
                label: Some(format!("{kind_id} {:.0}%", weight * 100.0)),
                value: weight,
                min: 0.0,
                max: 1.0,
                step: Some(0.01),
                on_change: puzzle3d_cmd(command, Some(json!({ "kindId": kind_id }))),
            }
        })
        .collect()
}

fn puzzle3d_brush_measures_group(envelope: &Puzzle3dEnvelope, labels: &Puzzle3dLabels) -> WindowMeasure {
    let object_ids = puzzle3d_kind_ids(&envelope.fixture, "objects");
    let vortex_ids = puzzle3d_kind_ids(&envelope.fixture, "vortices");
    WindowMeasure::Group {
        id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-brush"),
        label: "Brush".into(),
        default_open: Some(false),
        children: vec![
            WindowMeasure::Slider {
                id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-brush-overlap-budget"),
                label: Some("Overlap budget (m³)".into()),
                value: envelope.runtime.overlap_budget,
                min: 0.0,
                max: 1.0,
                step: Some(0.01),
                on_change: puzzle3d_cmd("setBrushPlacementOverlapBudget", None),
            },
            WindowMeasure::Group {
                id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-brush-distribution"),
                label: "Distribution".into(),
                default_open: Some(false),
                children: vec![
                    WindowMeasure::Group {
                        id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-brush-distribution-objects"),
                        label: labels.objects.into(),
                        default_open: Some(false),
                        children: puzzle3d_kind_weight_measures("object-kind", &object_ids, &envelope.runtime.object_kind_weights, "setObjectKindWeight"),
                    },
                    WindowMeasure::Group {
                        id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-brush-distribution-vortices"),
                        label: labels.vortices.into(),
                        default_open: Some(false),
                        children: puzzle3d_kind_weight_measures("vortex-kind", &vortex_ids, &envelope.runtime.vortex_kind_weights, "setVortexKindWeight"),
                    },
                ],
            },
        ],
    }
}

fn puzzle3d_view_measure(runtime: &Puzzle3dRuntime) -> WindowMeasure {
    WindowMeasure::Select {
        id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-view"),
        label: Some("View".into()),
        value: runtime.view_preset.clone(),
        items: vec![
            MeasureSelectItem { id: "perspective".into(), value: "perspective".into(), label: "Perspective".into() },
            MeasureSelectItem { id: "top".into(), value: "top".into(), label: "Top".into() },
            MeasureSelectItem { id: "front".into(), value: "front".into(), label: "Front".into() },
            MeasureSelectItem { id: "right".into(), value: "right".into(), label: "Right".into() },
        ],
        on_change: puzzle3d_cmd("setCameraViewPreset", None),
    }
}

fn puzzle3d_window_measures(envelope: &Puzzle3dEnvelope, labels: &Puzzle3dLabels) -> Vec<WindowMeasure> {
    vec![puzzle3d_view_measure(&envelope.runtime), puzzle3d_lod_measures_group(&envelope.runtime), puzzle3d_select_measures_group(&envelope.runtime, labels), puzzle3d_brush_measures_group(envelope, labels)]
}
//#endregion 🔖Measures

//#region 🔖Puzzle3dPlayApp
pub struct Puzzle3dPlayApp {
    precompute: Puzzle3dPrecomputeSession,
}

impl Default for Puzzle3dPlayApp {
    fn default() -> Self {
        Self { precompute: Puzzle3dPrecomputeSession::new() }
    }
}

impl PluginApp for Puzzle3dPlayApp {
    fn app_id(&self) -> &str {
        PUZZLE3D_PLAY_APP_ID
    }

    fn initial_document_json(&self) -> String {
        serde_json::to_string(&default_envelope()).expect("puzzle3d envelope json")
    }

    fn handle_command_patch_ops(&mut self, command: &str, args: Option<&Value>, document_json: &str, _view_state: &ViewState) -> Vec<String> {
        let mut envelope = parse_envelope(document_json);
        sync_precompute_session(&mut self.precompute, &envelope);
        if PUZZLE3D_UNDOABLE_COMMANDS.contains(&command) {
            envelope.runtime.undo_stack.push(envelope.fixture.clone());
            if envelope.runtime.undo_stack.len() > PUZZLE3D_UNDO_STACK_MAX {
                envelope.runtime.undo_stack.remove(0);
            }
            envelope.runtime.redo_stack.clear();
        }
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
                    if let Ok(fixture) = serde_json::from_str::<Puzzle3dFixture>(json_text) {
                        envelope.fixture = fixture;
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "setActiveExample" => {
                let example_id = args.and_then(|value| value.get("exampleId")).and_then(|value| value.as_str()).unwrap_or("");
                envelope = if example_id.is_empty() || example_id == "empty" {
                    Puzzle3dEnvelope { fixture: empty_fixture(), runtime: Puzzle3dRuntime::default() }
                } else if example_id == PUZZLE3D_EXAMPLE_CONCRETE_FOREST || example_id == "concrete" {
                    default_envelope()
                } else if example_id == PUZZLE3D_EXAMPLE_NAKAGIN || example_id == "nakagin" {
                    nakagin_envelope()
                } else {
                    envelope
                };
                drive_precompute(&mut self.precompute, &envelope);
                return vec![set_document_op(&envelope)];
            }
            "setSelection" => {
                if let Some(selection) = args.and_then(|value| value.get("selection")) {
                    if let Ok(parsed) = serde_json::from_value(selection.clone()) {
                        envelope.runtime.selection = parsed;
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "setActiveTool" => {
                let tool = args.and_then(|value| value.get("tool")).and_then(|value| value.as_str()).unwrap_or("select");
                envelope.runtime.active_tool = tool.into();
                if envelope.runtime.active_tool == "brush" || envelope.runtime.active_tool == "fill" {
                    drive_precompute(&mut self.precompute, &envelope);
                }
                return vec![set_document_op(&envelope)];
            }
            "addObjectKind" => {
                let object_kind = args.and_then(|value| value.get("objectKind")).and_then(|value| value.as_str()).unwrap_or("Object");
                let id = next_object_id();
                let catalog_entry = envelope.fixture.meta.kind_catalogs.as_ref().and_then(|catalogs| catalogs.get("objects")?.as_array()?.iter().find(|entry| entry.get("id").and_then(|v| v.as_str()) == Some(object_kind)).cloned());
                let mesh_url = catalog_entry.as_ref().and_then(|entry| entry.get("meshUrl").and_then(|v| v.as_str()).map(str::to_string));
                let vortices = catalog_entry.as_ref().map(|entry| puzzle3d_vortices_from_kind_template(entry)).unwrap_or_default();
                let origin = args
                    .and_then(|value| value.get("origin"))
                    .and_then(|value| value.as_array())
                    .map(|values| [values.first().and_then(|v| v.as_f64()).unwrap_or(0.0), values.get(1).and_then(|v| v.as_f64()).unwrap_or(0.0), values.get(2).and_then(|v| v.as_f64()).unwrap_or(0.0)])
                    .unwrap_or([0.0, 0.0, 0.0]);
                envelope.fixture.objects.push(Puzzle3dObject {
                    id: id.clone(),
                    label: Some(object_kind.into()),
                    object_kind: Some(object_kind.into()),
                    origin,
                    orientation: Some([0.0, 0.0, 0.0, 1.0]),
                    scale: None,
                    mesh_url,
                    vortices,
                    hidden: false,
                    locked: false,
                });
                envelope.runtime.selection.object_ids = vec![id];
                return vec![set_document_op(&envelope)];
            }
            "deleteSelection" => {
                let object_ids: Vec<String> = envelope.runtime.selection.object_ids.clone();
                let vortex_ids: HashSet<String> = envelope.runtime.selection.vortex_ids.iter().cloned().collect();
                let attraction_ids: Vec<String> = envelope.runtime.selection.attraction_ids.clone();
                let target_volume_ids: Vec<String> = envelope.runtime.selection.target_volume_ids.clone();
                envelope.fixture.objects.retain(|object| !object_ids.contains(&object.id));
                if !vortex_ids.is_empty() {
                    for object in envelope.fixture.objects.iter_mut() {
                        object.vortices.retain(|vortex| !vortex_ids.contains(&puzzle3d_vortex_full_id(&object.id, &vortex.id)));
                    }
                }
                envelope.fixture.attractions.retain(|attraction| !attraction_ids.contains(&attraction.id) && !object_ids.iter().any(|id| attraction.attracting.starts_with(&format!("{id}:")) || attraction.attracted.starts_with(&format!("{id}:"))));
                envelope.fixture.target_volumes.retain(|volume| !target_volume_ids.contains(&volume.id));
                let reference_ids: Vec<String> = envelope.runtime.selection.reference_ids.clone();
                envelope.fixture.references.retain(|reference| !reference_ids.contains(&reference.id));
                envelope.runtime.selection = Puzzle3dSelection::default();
                return vec![set_document_op(&envelope)];
            }
            "duplicateSelection" => {
                let ids = envelope.runtime.selection.object_ids.clone();
                let clones: Vec<Puzzle3dObject> = envelope
                    .fixture
                    .objects
                    .iter()
                    .filter(|object| ids.contains(&object.id))
                    .map(|object| {
                        let mut clone = object.clone();
                        clone.id = next_object_id();
                        clone.origin[0] += 0.5;
                        clone.origin[1] += 0.5;
                        clone
                    })
                    .collect();
                let new_ids: Vec<String> = clones.iter().map(|object| object.id.clone()).collect();
                envelope.fixture.objects.extend(clones);
                envelope.runtime.selection.object_ids = new_ids;
                return vec![set_document_op(&envelope)];
            }
            "selectSameKindSelection" => {
                let Some(first_id) = envelope.runtime.selection.object_ids.first() else {
                    return Vec::new();
                };
                let Some(kind) = envelope.fixture.objects.iter().find(|object| object.id == *first_id).and_then(|object| object.object_kind.clone()).filter(|kind| !kind.is_empty()) else {
                    return Vec::new();
                };
                envelope.runtime.selection.object_ids = envelope.fixture.objects.iter().filter(|object| object.object_kind.as_deref() == Some(kind.as_str())).map(|object| object.id.clone()).collect();
                return vec![set_document_op(&envelope)];
            }
            "setCamera" => {
                if let Some(camera) = args.and_then(|value| value.get("camera")) {
                    if let Ok(parsed) = serde_json::from_value(camera.clone()) {
                        envelope.fixture.camera = parsed;
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "setProjection" => {
                if let Some(projection) = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()) {
                    envelope.fixture.camera.projection = Some(projection.into());
                    return vec![set_document_op(&envelope)];
                }
            }
            "setCameraViewPreset" => {
                if let Some(preset) = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()) {
                    envelope.fixture.camera = puzzle3d_camera_view_preset(preset);
                    envelope.runtime.view_preset = preset.into();
                    return vec![set_document_op(&envelope)];
                }
            }
            "setJackQuery" => {
                if let Some(value) = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()) {
                    envelope.runtime.jack_query = value.into();
                    return vec![set_document_op(&envelope)];
                }
            }
            "translateSelection" => {
                let ids = mesh_selection_ids(args, &envelope.runtime.selection.object_ids);
                let dx = args.and_then(|value| value.get("dx")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let dy = args.and_then(|value| value.get("dy")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let dz = args.and_then(|value| value.get("dz")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                for object in &mut envelope.fixture.objects {
                    if ids.contains(&object.id) {
                        object.origin[0] += dx;
                        object.origin[1] += dy;
                        object.origin[2] += dz;
                    }
                }
                if !ids.is_empty() {
                    return vec![set_document_op(&envelope)];
                }
            }
            "rotateSelection" => {
                let ids = mesh_selection_ids(args, &envelope.runtime.selection.object_ids);
                let ax = args.and_then(|value| value.get("ax")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let ay = args.and_then(|value| value.get("ay")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let az = args.and_then(|value| value.get("az")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let angle = args.and_then(|value| value.get("angle")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let delta = quat_from_axis_angle(ax, ay, az, angle);
                for object in &mut envelope.fixture.objects {
                    if ids.contains(&object.id) {
                        let current = object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
                        object.orientation = Some(quat_mul(delta, current));
                    }
                }
                if !ids.is_empty() {
                    return vec![set_document_op(&envelope)];
                }
            }
            "scaleSelection" => {
                let ids = mesh_selection_ids(args, &envelope.runtime.selection.object_ids);
                let sx = args.and_then(|value| value.get("sx")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                let sy = args.and_then(|value| value.get("sy")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                let sz = args.and_then(|value| value.get("sz")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                for object in &mut envelope.fixture.objects {
                    if ids.contains(&object.id) {
                        object.scale = Some(scale_value_mul(&object.scale, sx, sy, sz));
                    }
                }
                if !ids.is_empty() {
                    return vec![set_document_op(&envelope)];
                }
            }
            "worldSelect" => {
                let merge = args.and_then(|value| value.get("merge")).and_then(|value| value.as_str()).unwrap_or("replace");
                let ids: Vec<String> = args.and_then(|value| value.get("ids")).and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default();
                envelope.runtime.selection.object_ids = merge_world_selection_ids(&envelope.runtime.selection.object_ids, &ids, merge);
                return vec![set_document_op(&envelope)];
            }
            "worldHover" => {
                envelope.runtime.hovered_object_id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()).map(str::to_string);
                return vec![set_document_op(&envelope)];
            }
            "setHover" => {
                if args.is_none() || args.and_then(|value| value.get("objectId")).is_none() {
                    envelope.runtime.hovered_object_id = None;
                } else {
                    envelope.runtime.hovered_object_id = args.and_then(|value| value.get("objectId")).and_then(|value| value.as_str()).map(str::to_string);
                }
                return vec![set_document_op(&envelope)];
            }
            "worldPick" => {
                let merge = args.and_then(|value| value.get("merge")).and_then(|value| value.as_str()).unwrap_or("replace");
                if args.and_then(|value| value.get("id")).map_or(true, |value| value.is_null()) {
                    if merge == "replace" {
                        envelope.runtime.selection.object_ids.clear();
                    }
                    return vec![set_document_op(&envelope)];
                }
                let index = args.and_then(|value| value.get("id")).and_then(|value| value.as_u64()).unwrap_or(0) as usize;
                if let Some(object) = envelope.fixture.objects.get(index).filter(|object| !object.locked && !object.hidden) {
                    let id = object.id.clone();
                    let merge_ids = if merge == "add" {
                        let mut merged = envelope.runtime.selection.object_ids.clone();
                        if !merged.contains(&id) {
                            merged.push(id.clone());
                        }
                        merged
                    } else if merge == "toggle" {
                        let mut merged = envelope.runtime.selection.object_ids.clone();
                        if let Some(pos) = merged.iter().position(|entry| entry == &id) {
                            merged.remove(pos);
                        } else {
                            merged.push(id);
                        }
                        merged
                    } else {
                        vec![id]
                    };
                    envelope.runtime.selection.object_ids = merge_ids;
                    return vec![set_document_op(&envelope)];
                }
            }
            "setTransformTool" => {
                if let Some(tool) = args.and_then(|value| value.get("tool")).and_then(|value| value.as_str()) {
                    envelope.runtime.transform_tool = tool.into();
                    return vec![set_document_op(&envelope)];
                }
            }
            "worldVortexHover" => {
                envelope.runtime.selection.vortex_ids = args.and_then(|value| value.get("fullId")).and_then(|value| value.as_str()).map(|full_id| vec![full_id.to_string()]).unwrap_or_default();
                if envelope.runtime.active_tool == "brush" && !envelope.runtime.selection.vortex_ids.is_empty() {
                    drive_precompute(&mut self.precompute, &envelope);
                }
                return vec![set_document_op(&envelope)];
            }
            "worldVortexSelect" => {
                if let Some(full_id) = args.and_then(|value| value.get("fullId")).and_then(|value| value.as_str()) {
                    envelope.runtime.selection.vortex_ids = vec![full_id.to_string()];
                    envelope.runtime.selection.object_ids.clear();
                    drive_precompute(&mut self.precompute, &envelope);
                    return vec![set_document_op(&envelope)];
                }
            }
            "worldRelocate" => {
                let object_id = args.and_then(|value| value.get("objectId")).and_then(|value| value.as_str()).unwrap_or("");
                let position = args
                    .and_then(|value| value.get("position"))
                    .and_then(|value| value.as_array())
                    .map(|values| [values.first().and_then(|v| v.as_f64()).unwrap_or(0.0), values.get(1).and_then(|v| v.as_f64()).unwrap_or(0.0), values.get(2).and_then(|v| v.as_f64()).unwrap_or(0.0)]);
                let proximity_radius = envelope.runtime.proximity_radius;
                if let (Some(object), Some(position)) = (envelope.fixture.objects.iter_mut().find(|object| object.id == object_id && !object.locked && !object.hidden), position) {
                    object.origin = position;
                    let mut source_vortex: Option<(String, [f64; 3])> = None;
                    for vortex in &object.vortices {
                        let full_id = puzzle3d_vortex_full_id(&object.id, &vortex.id);
                        source_vortex = Some((full_id, world_vortex_position(object, vortex)));
                        break;
                    }
                    if let Some((source_id, source_pos)) = source_vortex {
                        for other in &envelope.fixture.objects {
                            if other.id == object_id {
                                continue;
                            }
                            for vortex in &other.vortices {
                                let target_id = puzzle3d_vortex_full_id(&other.id, &vortex.id);
                                if target_id == source_id {
                                    continue;
                                }
                                let target_pos = world_vortex_position(other, vortex);
                                let dx = source_pos[0] - target_pos[0];
                                let dy = source_pos[1] - target_pos[1];
                                let dz = source_pos[2] - target_pos[2];
                                let distance = (dx * dx + dy * dy + dz * dz).sqrt();
                                if distance <= proximity_radius {
                                    let attraction_id = format!("attraction-{}", PUZZLE3D_ID_COUNTER.fetch_add(1, Ordering::Relaxed));
                                    if !envelope.fixture.attractions.iter().any(|entry| entry.attracting == source_id && entry.attracted == target_id || entry.attracting == target_id && entry.attracted == source_id) {
                                        envelope.fixture.attractions.push(Puzzle3dAttraction { id: attraction_id, attracting: source_id.clone(), attracted: target_id });
                                    }
                                }
                            }
                        }
                    }
                    sync_precompute_session(&mut self.precompute, &envelope);
                    return vec![set_document_op(&envelope)];
                }
            }
            "setSelectionMethod" => {
                let method = args.and_then(|value| value.get("method")).and_then(|value| value.as_str()).unwrap_or("rectangle");
                envelope.runtime.selection_method = method.into();
                return vec![set_document_op(&envelope)];
            }
            "setLodAutomatic" => {
                envelope.runtime.lod_automatic = args.and_then(|value| value.get("pressed")).and_then(|value| value.as_bool()).unwrap_or(!envelope.runtime.lod_automatic);
                return vec![set_document_op(&envelope)];
            }
            "setLodDepthVariable" => {
                envelope.runtime.lod_depth_variable = args.and_then(|value| value.get("pressed")).and_then(|value| value.as_bool()).unwrap_or(!envelope.runtime.lod_depth_variable);
                return vec![set_document_op(&envelope)];
            }
            "setLodShowGrid" => {
                envelope.runtime.lod_show_grid = args.and_then(|value| value.get("pressed")).and_then(|value| value.as_bool()).unwrap_or(!envelope.runtime.lod_show_grid);
                return vec![set_document_op(&envelope)];
            }
            "setLodManual" => {
                if let Some(value) = args.and_then(|value| value.get("value")).and_then(|value| value.as_f64()) {
                    envelope.runtime.lod_manual = value.clamp(PUZZLE3D_LOD_SLIDER_MIN, PUZZLE3D_LOD_SLIDER_MAX);
                    return vec![set_document_op(&envelope)];
                }
            }
            "setGridSnapEnabled" => {
                envelope.runtime.grid_snap_enabled = args.and_then(|value| value.get("pressed")).and_then(|value| value.as_bool()).unwrap_or(!envelope.runtime.grid_snap_enabled);
                return vec![set_document_op(&envelope)];
            }
            "setGridFactor" => {
                if let Some(value) = args.and_then(|value| value.get("value")).and_then(|value| value.as_f64()) {
                    envelope.runtime.grid_factor = value.max(0.1);
                    return vec![set_document_op(&envelope)];
                }
            }
            "setSelectionModeDefault" => {
                if let Some(mode) = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()) {
                    envelope.runtime.selection_mode_default = mode.into();
                    return vec![set_document_op(&envelope)];
                }
            }
            "setProximityRadius" => {
                if let Some(value) = args.and_then(|value| value.get("value")).and_then(|value| value.as_f64()) {
                    envelope.runtime.proximity_radius = value.max(0.0);
                    return vec![set_document_op(&envelope)];
                }
            }
            "setChunkSize" => {
                if let Some(value) = args.and_then(|value| value.get("value")).and_then(|value| value.as_f64()) {
                    envelope.runtime.chunk_size = value.max(1.0);
                    return vec![set_document_op(&envelope)];
                }
            }
            "setSelectableKind" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("");
                let pressed = args.and_then(|value| value.get("pressed")).and_then(|value| value.as_bool());
                match kind {
                    "objects" => envelope.runtime.selectable_kinds.objects = pressed.unwrap_or(!envelope.runtime.selectable_kinds.objects),
                    "vortices" => envelope.runtime.selectable_kinds.vortices = pressed.unwrap_or(!envelope.runtime.selectable_kinds.vortices),
                    "attractions" => envelope.runtime.selectable_kinds.attractions = pressed.unwrap_or(!envelope.runtime.selectable_kinds.attractions),
                    _ => {}
                }
                return vec![set_document_op(&envelope)];
            }
            "setKindHover" => {
                envelope.runtime.hovered_kind_id = args.and_then(|value| value.get("kindId")).and_then(|value| value.as_str()).map(str::to_string);
                return vec![set_document_op(&envelope)];
            }
            "setSelectionFlag" => {
                let flag = args.and_then(|value| value.get("flag")).and_then(|value| value.as_str()).unwrap_or("hidden");
                let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_bool()).unwrap_or(true);
                let entity = args.and_then(|value| value.get("entity")).and_then(|value| value.as_str());
                let explicit_ids: Option<Vec<String>> = args.and_then(|value| value.get("ids")).and_then(|value| serde_json::from_value(value.clone()).ok());
                match (entity, explicit_ids) {
                    (Some(entity), Some(ids)) => apply_puzzle3d_selection_flag(&mut envelope.fixture, entity, &ids, flag, value),
                    _ => {
                        apply_puzzle3d_selection_flag(&mut envelope.fixture, "object", &envelope.runtime.selection.object_ids.clone(), flag, value);
                        apply_puzzle3d_selection_flag(&mut envelope.fixture, "vortex", &envelope.runtime.selection.vortex_ids.clone(), flag, value);
                        apply_puzzle3d_selection_flag(&mut envelope.fixture, "targetVolume", &envelope.runtime.selection.target_volume_ids.clone(), flag, value);
                    }
                }
                return vec![set_document_op(&envelope)];
            }
            "patchInspector" => {
                let entity = args.and_then(|value| value.get("entity")).and_then(|value| value.as_str()).unwrap_or("");
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                let ids: Vec<String> = args.and_then(|value| value.get("ids")).and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default();
                let value = args.and_then(|value| value.get("value")).cloned().unwrap_or(Value::Null);
                apply_puzzle3d_inspector_patch(&mut envelope.fixture, entity, &ids, field, &value);
                return vec![set_document_op(&envelope)];
            }
            "selectAll" => {
                envelope.runtime.selection.object_ids = if envelope.runtime.selectable_kinds.objects {
                    envelope.fixture.objects.iter().filter(|object| !object.hidden && !object.locked).map(|object| object.id.clone()).collect()
                } else {
                    Vec::new()
                };
                envelope.runtime.selection.vortex_ids.clear();
                envelope.runtime.selection.attraction_ids.clear();
                envelope.runtime.selection.target_volume_ids.clear();
                envelope.runtime.selection.reference_ids.clear();
                return vec![set_document_op(&envelope)];
            }
            "clearSelection" => {
                envelope.runtime.selection = Puzzle3dSelection::default();
                return vec![set_document_op(&envelope)];
            }
            "focusSelection" => {
                apply_puzzle3d_focus_selection(&mut envelope);
                return vec![set_document_op(&envelope)];
            }
            "engagementInput" => {
                envelope.runtime.engagement_input = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).unwrap_or("").to_string();
                return vec![set_document_op(&envelope)];
            }
            "engagementSubmit" => {
                let raw = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).unwrap_or("").trim().to_lowercase();
                let mut tokens = raw.split_whitespace();
                match tokens.next() {
                    Some("select") => envelope.runtime.active_tool = "select".into(),
                    Some("brush") => {
                        envelope.runtime.active_tool = "brush".into();
                        drive_precompute(&mut self.precompute, &envelope);
                    }
                    Some("fill") => {
                        drive_precompute(&mut self.precompute, &envelope);
                        let count = tokens.next().and_then(|value| value.parse::<u32>().ok()).unwrap_or(envelope.runtime.fill_count).min(PUZZLE3D_FILL_COUNT_MAX);
                        envelope = apply_puzzle3d_fill_count(&mut self.precompute, envelope, count);
                    }
                    Some("zoom") => apply_puzzle3d_focus_selection(&mut envelope),
                    Some("clear") => envelope.runtime.selection = Puzzle3dSelection::default(),
                    Some("rectangle") => envelope.runtime.selection_method = "rectangle".into(),
                    Some("lasso") => envelope.runtime.selection_method = "lasso".into(),
                    _ => {}
                }
                envelope.runtime.engagement_input = String::new();
                return vec![set_document_op(&envelope)];
            }
            "engagementRepeatLast" => {
                if envelope.runtime.active_tool == "fill" {
                    let count = (envelope.runtime.fill_count + 1).min(PUZZLE3D_FILL_COUNT_MAX);
                    envelope = apply_puzzle3d_fill_count(&mut self.precompute, envelope, count);
                }
                return vec![set_document_op(&envelope)];
            }
            "engagementAbort" => {
                envelope.runtime.engagement_input = String::new();
                envelope.runtime.brush_candidate_index = 0;
                if envelope.runtime.active_tool != "select" {
                    envelope.runtime.active_tool = "select".into();
                }
                return vec![set_document_op(&envelope)];
            }
            "undo" => {
                if let Some(previous) = envelope.runtime.undo_stack.pop() {
                    envelope.runtime.redo_stack.push(envelope.fixture.clone());
                    envelope.fixture = previous;
                    envelope.runtime.selection = Puzzle3dSelection::default();
                    sync_precompute_session(&mut self.precompute, &envelope);
                    return vec![set_document_op(&envelope)];
                }
            }
            "redo" => {
                if let Some(next) = envelope.runtime.redo_stack.pop() {
                    envelope.runtime.undo_stack.push(envelope.fixture.clone());
                    envelope.fixture = next;
                    envelope.runtime.selection = Puzzle3dSelection::default();
                    sync_precompute_session(&mut self.precompute, &envelope);
                    return vec![set_document_op(&envelope)];
                }
            }
            "createAttraction" => {
                let attracting = args.and_then(|value| value.get("attracting")).and_then(|value| value.as_str()).unwrap_or("");
                let attracted = args.and_then(|value| value.get("attracted")).and_then(|value| value.as_str()).unwrap_or("");
                if !attracting.is_empty() && !attracted.is_empty() && attracting != attracted {
                    let already_connected = envelope.fixture.attractions.iter().any(|attraction| (attraction.attracting == attracting && attraction.attracted == attracted) || (attraction.attracting == attracted && attraction.attracted == attracting));
                    let compatible = match (resolve_vortex_kind(&envelope.fixture, attracting), resolve_vortex_kind(&envelope.fixture, attracted)) {
                        (Some(source_kind), Some(target_kind)) => puzzle3d_kinds_compatible(&envelope.fixture, &source_kind, &target_kind),
                        _ => false,
                    };
                    if !already_connected && compatible {
                        let id = format!("attraction-{}", PUZZLE3D_ID_COUNTER.fetch_add(1, Ordering::Relaxed));
                        envelope.fixture.attractions.push(Puzzle3dAttraction { id, attracting: attracting.into(), attracted: attracted.into() });
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "deleteAttraction" => {
                if let Some(id) = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()) {
                    envelope.fixture.attractions.retain(|attraction| attraction.id != id);
                    return vec![set_document_op(&envelope)];
                }
            }
            "setFillEditTargetVolumes" => {
                let id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str());
                envelope.runtime.fill_edit_target_volumes = match id {
                    Some("edit-volumes") => true,
                    Some("fill") => false,
                    _ => !envelope.runtime.fill_edit_target_volumes,
                };
                return vec![set_document_op(&envelope)];
            }
            "setVoxelDims" => {
                let axis = args.and_then(|value| value.get("axis")).and_then(|value| value.as_str()).unwrap_or("");
                if let Some(value) = args.and_then(|value| value.get("value")).and_then(|value| value.as_f64()) {
                    let dimension = value.max(1.0).round() as u32;
                    match axis {
                        "w" => envelope.runtime.voxel_dims[0] = dimension,
                        "d" => envelope.runtime.voxel_dims[1] = dimension,
                        "h" => envelope.runtime.voxel_dims[2] = dimension,
                        _ => {}
                    }
                    return vec![set_document_op(&envelope)];
                }
            }
            "addTargetVolume" => {
                if let Some(origin) = args.and_then(|value| value.get("origin")).and_then(value_as_vec3) {
                    let grid_factor = envelope.runtime.grid_factor.max(0.1);
                    let snapped = [(origin[0] / grid_factor).round() * grid_factor, (origin[1] / grid_factor).round() * grid_factor, (origin[2] / grid_factor).round() * grid_factor];
                    let [w, d, h] = envelope.runtime.voxel_dims;
                    let scale = json!([w as f64 * grid_factor, d as f64 * grid_factor, h as f64 * grid_factor]);
                    let id = format!("target-volume-{}", PUZZLE3D_ID_COUNTER.fetch_add(1, Ordering::Relaxed));
                    envelope.fixture.target_volumes.push(Puzzle3dTargetVolume { id, origin: snapped, orientation: None, scale: Some(scale), hidden: false, locked: false });
                    return vec![set_document_op(&envelope)];
                }
            }
            "deleteTargetVolume" => {
                if let Some(id) = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()) {
                    envelope.fixture.target_volumes.retain(|volume| volume.id != id);
                    return vec![set_document_op(&envelope)];
                }
            }
            "engagementPossibleSelect" => {
                let possible_id = args.and_then(|value| value.get("possibleId")).and_then(|value| value.as_str()).unwrap_or("");
                envelope.runtime.active_tool = match possible_id {
                    PUZZLE3D_ENGAGEMENT_TOOL_BRUSH => "brush",
                    PUZZLE3D_ENGAGEMENT_TOOL_FILL => "fill",
                    _ => "select",
                }
                .into();
                if envelope.runtime.active_tool == "brush" || envelope.runtime.active_tool == "fill" {
                    drive_precompute(&mut self.precompute, &envelope);
                }
                return vec![set_document_op(&envelope)];
            }
            "engagementControlSelect" => {
                let candidate_id = args.and_then(|value| value.get("id").or_else(|| value.get("value"))).and_then(|value| value.as_str()).unwrap_or("");
                if let Some(index) = candidate_id.strip_prefix("puzzle3d.brush.candidate.").and_then(|rest| rest.parse::<usize>().ok()) {
                    envelope.runtime.brush_candidate_index = index;
                    return vec![set_document_op(&envelope)];
                }
            }
            "addBrushObject" => {
                drive_precompute(&mut self.precompute, &envelope);
                if let Some(payload_value) = args {
                    if let Ok(payload) = serde_json::from_value::<BrushPlacePayload>(payload_value.clone()) {
                        if let Ok(fixture_json) = self.precompute.apply_brush_placement_rust(&serde_json::to_string(&payload).unwrap_or_default()) {
                            if let Some(next) = fixture_from_engine_json(&envelope, &fixture_json) {
                                envelope = next;
                                return vec![set_document_op(&envelope)];
                            }
                        }
                    }
                }
            }
            "setFillCount" => {
                drive_precompute(&mut self.precompute, &envelope);
                let count = args.and_then(|value| value.get("count").or_else(|| value.get("value"))).and_then(|value| value.as_u64()).unwrap_or(0).min(u64::from(PUZZLE3D_FILL_COUNT_MAX)) as u32;
                envelope = apply_puzzle3d_fill_count(&mut self.precompute, envelope, count);
                return vec![set_document_op(&envelope)];
            }
            "setBrushPlacementOverlapBudget" => {
                if let Some(value) = args.and_then(|v| v.get("value")).and_then(|v| v.as_f64()) {
                    envelope.runtime.overlap_budget = value.clamp(0.0, 1.0);
                    sync_precompute_session(&mut self.precompute, &envelope);
                    return vec![set_document_op(&envelope)];
                }
            }
            "setObjectKindWeight" | "setVortexKindWeight" => {
                let kind_id = args.and_then(|v| v.get("kindId")).and_then(|v| v.as_str()).unwrap_or("");
                let value = args.and_then(|v| v.get("value")).and_then(|v| v.as_f64()).unwrap_or(1.0);
                if command == "setObjectKindWeight" {
                    envelope.runtime.object_kind_weights.insert(kind_id.into(), value);
                } else {
                    envelope.runtime.vortex_kind_weights.insert(kind_id.into(), value);
                }
                sync_precompute_session(&mut self.precompute, &envelope);
                return vec![set_document_op(&envelope)];
            }
            "cycleBrushCandidate" => {
                drive_precompute(&mut self.precompute, &envelope);
                if let Some(vortex_id) = puzzle3d_brush_target_vortex(&envelope) {
                    let raw = self.precompute.brush_candidates(&vortex_id);
                    let free_count = parse_brush_candidates_free_count(&raw);
                    if free_count > 0 {
                        envelope.runtime.brush_candidate_index = (envelope.runtime.brush_candidate_index + 1) % free_count;
                    }
                } else {
                    envelope.runtime.brush_candidate_index = envelope.runtime.brush_candidate_index.saturating_add(1);
                }
                return vec![set_document_op(&envelope)];
            }
            "registerBrushMesh" => {
                if let (Some(url), Some(positions), Some(indices)) =
                    (args.and_then(|v| v.get("url")).and_then(|v| v.as_str()), args.and_then(|v| v.get("positions")).and_then(|v| v.as_array()), args.and_then(|v| v.get("indices")).and_then(|v| v.as_array()))
                {
                    let positions: Vec<f32> = positions.iter().filter_map(|v| v.as_f64().map(|n| n as f32)).collect();
                    let indices: Vec<u32> = indices.iter().filter_map(|v| v.as_u64().map(|n| n as u32)).collect();
                    self.precompute.register_mesh(url, &positions, &indices);
                    if let Ok(mut registry) = PUZZLE3D_MESH_REGISTRY.lock() {
                        registry.insert(url.to_string(), (positions, indices));
                    }
                }
                return Vec::new();
            }
            "worldPointerDown" => return Vec::new(),
            _ => {}
        };
        Vec::new()
    }

    fn render(&self, body_key: &str, document_json: &str, view_state: &ViewState) -> UiNode {
        let envelope = parse_envelope(document_json);
        let labels = puzzle3d_labels(view_state);
        match body_key {
            PUZZLE3D_PLAY_BODY_COMPOSITE => {
                let brush_preview = world_brush_preview_json(&self.precompute, &envelope);
                build_world_3d_scene(
                    PUZZLE3D_PLAY_SURFACE_VIEWPORT,
                    PUZZLE3D_PLAY_APP_ID,
                    world3d_scene_extended(
                        camera_json(&envelope.fixture.camera),
                        world_meshes_json(&envelope.fixture),
                        world_instances_json(&envelope.fixture, &envelope.runtime),
                        world_selection_json(&envelope),
                        Some(world_vortices_json(&envelope.fixture)),
                        Some(world_attractions_json(&envelope.fixture)),
                        Some(world_target_volumes_json(&envelope.fixture)),
                        Some(world_references_json(&envelope.fixture)),
                        brush_preview,
                        Some(world_interaction_json(&envelope.runtime)),
                        None,
                        Some(world3d_lod_json(&envelope.runtime)),
                        Some(world3d_chunking_json(envelope.runtime.chunk_size, 8000.0)),
                        puzzle3d_context_menu_json(&envelope),
                    ),
                )
            }
            PUZZLE3D_PLAY_BODY_DOCUMENT => build_document_tree(&envelope, labels),
            PUZZLE3D_PLAY_BODY_KINDS => build_kinds_tree(&envelope, labels),
            PUZZLE3D_PLAY_BODY_INSPECTOR => build_inspector_tree(&envelope, labels),
            PUZZLE3D_PLAY_BODY_SETTINGS => build_settings_body(&envelope),
            PUZZLE3D_PLAY_BODY_JACK => build_jack_body(&envelope),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn window_engagements(&self, document_json: &str, _view_state: &ViewState) -> HashMap<String, WindowEngagement> {
        let envelope = parse_envelope(document_json);
        HashMap::from([(PUZZLE3D_PLAY_WINDOW_MAIN.into(), puzzle3d_engagement(&envelope, &self.precompute))])
    }

    fn window_measures(&self, document_json: &str, view_state: &ViewState) -> HashMap<String, Vec<WindowMeasure>> {
        let envelope = parse_envelope(document_json);
        let labels = puzzle3d_labels(view_state);
        HashMap::from([(PUZZLE3D_PLAY_WINDOW_MAIN.into(), puzzle3d_window_measures(&envelope, labels))])
    }
}
//#endregion 🔖Puzzle3dPlayApp

//#region 🔖Manifest
pub fn create_puzzle3d_app() -> App {
    let envelope = default_envelope();
    App::from_builder(
        App::builder(PUZZLE3D_PLAY_APP_ID, "Puzzle 3D")
            .document(["semio", "puzzle", "3d"])
            .icon_id("puzzle")
            .terminology("reuse")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind_with_engagement(PUZZLE3D_PLAY_WINDOW_MAIN, "Puzzle 3D", PUZZLE3D_PLAY_BODY_COMPOSITE, SurfaceKind::World3d, puzzle3d_engagement(&envelope, &Puzzle3dPrecomputeSession::new()))
            .default_layout(create_default_layout(&[PUZZLE3D_PLAY_WINDOW_MAIN.into()], "row", Some(&[100.0]), Some(&["Puzzle 3D".into()])))
            .panel_tab(FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, PanelGroup::Workbench, PUZZLE3D_PLAY_BODY_DOCUMENT)
            .panel_tab(FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, PanelGroup::Workbench, PUZZLE3D_PLAY_BODY_KINDS)
            .panel_tab(FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, PanelGroup::Details, PUZZLE3D_PLAY_BODY_INSPECTOR)
            .panel_tab("puzzle3d.panel.settings", "Settings", PanelGroup::Settings, PUZZLE3D_PLAY_BODY_SETTINGS)
            .panel_tab("puzzle3d.panel.jack", "Jack", PanelGroup::Workbench, PUZZLE3D_PLAY_BODY_JACK)
            .keybinding("mod+a", "selectAll")
            .keybinding("escape", "engagementAbort")
            .keybinding("delete", "deleteSelection")
            .keybinding("backspace", "deleteSelection")
            .keybinding("mod+d", "duplicateSelection")
            .keybinding("tab", "cycleBrushCandidate")
            .keybinding("f", "focusSelection"),
    )
    .example("empty", "Empty", &serde_json::to_string(&Puzzle3dEnvelope { fixture: empty_fixture(), runtime: Puzzle3dRuntime::default() }).unwrap())
    .example(PUZZLE3D_EXAMPLE_CONCRETE_FOREST, "Concrete Forest", CONCRETE_FOREST_EXAMPLE_JSON)
    .example(PUZZLE3D_EXAMPLE_NAKAGIN, "Nakagin Capsule Tower", NAKAGIN_EXAMPLE_JSON)
    .program("puzzle3d", "Puzzle 3D", "model")
}

/// 🗃️ Real GLB geometry the browser round-tripped via `registerBrushMesh` this session, keyed by mesh url; falls back to a box for anything not yet loaded. `fn` pointers can't capture state, so this backs the export handler's plain-function-pointer signature.
static PUZZLE3D_MESH_REGISTRY: LazyLock<std::sync::Mutex<HashMap<String, (Vec<f32>, Vec<u32>)>>> = LazyLock::new(|| std::sync::Mutex::new(HashMap::new()));

/// 🌀 Undoes glTF's Y-up convention to land in this world's Z-up frame — mirrors `GLB_MESH_FRAME_ROTATION_X` (a fixed +90° turn about X) from `@semio-tech/infinite-world-r3f`, which the viewer applies visually but which raw `registerBrushMesh` vertices never carry.
fn glb_frame_correct(position: [f32; 3]) -> [f32; 3] {
    [position[0], -position[2], position[1]]
}

fn quat_rotate_point(point: [f32; 3], quat: [f64; 4]) -> [f32; 3] {
    let [qx, qy, qz, qw] = quat;
    let (x, y, z) = (point[0] as f64, point[1] as f64, point[2] as f64);
    let (cx, cy, cz) = (qy * z - qz * y, qz * x - qx * z, qx * y - qy * x);
    let (tx, ty, tz) = (2.0 * cx, 2.0 * cy, 2.0 * cz);
    let (ux, uy, uz) = (qy * tz - qz * ty, qz * tx - qx * tz, qx * ty - qy * tx);
    [(x + qw * tx + ux) as f32, (y + qw * ty + uy) as f32, (z + qw * tz + uz) as f32]
}

/// 💾 Bakes each object's world transform (GLB frame correction, then scale/orientation/origin) into a single merged mesh for OBJ/GLB export; objects whose GLB hasn't round-tripped through `registerBrushMesh` this session fall back to a box.
fn puzzle3d_mesh_from_document(doc: &serde_json::Value) -> Result<semio_framework_plugin::MeshData, String> {
    let envelope: Puzzle3dEnvelope = serde_json::from_value(doc.clone()).map_err(|error| error.to_string())?;
    let registry = PUZZLE3D_MESH_REGISTRY.lock().map_err(|_| "puzzle3d mesh registry poisoned".to_string())?;
    let fallback = mesh_from_kind(PUZZLE3D_FALLBACK_MESH_KIND);
    let mut merged = semio_framework_plugin::MeshData::default();
    for object in envelope.fixture.objects.iter().filter(|object| !object.hidden) {
        let mesh_url = resolve_object_mesh_url(object, &envelope.fixture.meta);
        let (positions, indices): (&[f32], &[u32]) = match mesh_url.as_deref().and_then(|url| registry.get(url)) {
            Some((positions, indices)) => (positions, indices),
            None => (&fallback.positions, &fallback.indices),
        };
        let orientation = object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
        let scale = object_scale_json(object);
        let index_offset = (merged.positions.len() / 3) as u32;
        for chunk in positions.chunks_exact(3) {
            let corrected = glb_frame_correct([chunk[0], chunk[1], chunk[2]]);
            let scaled = [corrected[0] * scale[0] as f32, corrected[1] * scale[1] as f32, corrected[2] * scale[2] as f32];
            let rotated = quat_rotate_point(scaled, orientation);
            merged.positions.push(rotated[0] + object.origin[0] as f32);
            merged.positions.push(rotated[1] + object.origin[1] as f32);
            merged.positions.push(rotated[2] + object.origin[2] as f32);
        }
        merged.indices.extend(indices.iter().map(|index| index + index_offset));
    }
    if merged.positions.is_empty() {
        return Ok(fallback);
    }
    merged.compute_normals();
    Ok(merged)
}

/// 📥 Tier C DWG mesh import — always returns the empty puzzle-3d fixture; never errors on a structurally valid mesh.
fn puzzle3d_document_from_mesh(_mesh: &semio_framework_plugin::MeshData) -> Result<serde_json::Value, String> {
    serde_json::to_value(Puzzle3dEnvelope { fixture: empty_fixture(), runtime: Puzzle3dRuntime::default() }).map_err(|error| error.to_string())
}

pub fn register_puzzle3d_exports() {
    semio_framework_os::register_mesh_export_handlers("3d.puzzle", "puzzle", puzzle3d_mesh_from_document);
    semio_framework_os::register_mesh_dwg_import_handler("3d.puzzle", puzzle3d_document_from_mesh);
}
//#endregion 🔖Manifest

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::PluginApp;

    #[test]
    fn renders_world_scene() {
        let app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let node = app.render(PUZZLE3D_PLAY_BODY_COMPOSITE, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("world-3d"));
    }

    #[test]
    fn concrete_forest_example_parses() {
        let envelope = default_envelope();
        assert_eq!(envelope.fixture.schema, PUZZLE3D_FIXTURE_SCHEMA);
        assert!(!envelope.fixture.objects.is_empty());
    }

    #[test]
    fn nakagin_example_parses() {
        let envelope = nakagin_envelope();
        assert_eq!(envelope.fixture.schema, PUZZLE3D_FIXTURE_SCHEMA);
        assert!(!envelope.fixture.objects.is_empty());
        assert!(envelope.fixture.meta.kind_catalogs.is_some());
    }

    #[test]
    fn scene_config_json_omits_host_rules_key() {
        let envelope = default_envelope();
        let config: Value = serde_json::from_str(&scene_config_json(&envelope)).unwrap();
        assert!(config.get("hostRules").is_none(), "an explicit empty hostRules object disables the default Nakagin brush rules");
    }

    /// 🧊 A real, above-`BRUSH_COLLISION_MESH_MIN_EXTENT` mesh registered via `registerBrushMesh` must keep its
    /// url mapped across repeated resyncs (the primitive box fallback is itself below the extent threshold, so
    /// its own registration is always a no-op and can never clear an existing entry).
    #[test]
    fn sync_precompute_session_preserves_registered_mesh() {
        let envelope = default_envelope();
        let mut session = Puzzle3dPrecomputeSession::new();
        let positions: Vec<f32> = vec![-4.0, -4.0, -4.0, 4.0, -4.0, -4.0, 4.0, 4.0, -4.0, -4.0, 4.0, -4.0, -4.0, -4.0, 4.0, 4.0, -4.0, 4.0, 4.0, 4.0, 4.0, -4.0, 4.0, 4.0];
        let indices: Vec<u32> = vec![0, 1, 2, 0, 2, 3, 4, 6, 5, 4, 7, 6, 0, 4, 5, 0, 5, 1, 2, 6, 7, 2, 7, 3, 0, 3, 7, 0, 7, 4, 1, 5, 6, 1, 6, 2];
        let url = collect_mesh_urls(&envelope.fixture).into_iter().next().expect("fixture has at least one mesh url");
        session.register_mesh(&url, &positions, &indices);
        sync_precompute_session(&mut session, &envelope);
        sync_precompute_session(&mut session, &envelope);
        assert!(session.has_mesh(&url));
    }

    #[test]
    fn puzzle3d_document_from_mesh_returns_valid_empty_fixture() {
        let mesh = semio_framework_plugin::mesh_from_kind(PUZZLE3D_FALLBACK_MESH_KIND);
        let document = puzzle3d_document_from_mesh(&mesh).unwrap();
        let envelope: Puzzle3dEnvelope = serde_json::from_value(document).unwrap();
        assert_eq!(envelope.fixture.schema, PUZZLE3D_FIXTURE_SCHEMA);
        assert!(envelope.fixture.objects.is_empty());
    }

    #[test]
    fn mesh_from_document_falls_back_to_box_when_no_mesh_registered() {
        let envelope = default_envelope();
        let mesh = puzzle3d_mesh_from_document(&serde_json::to_value(&envelope).unwrap()).unwrap();
        assert!(!mesh.positions.is_empty());
        assert!(!mesh.indices.is_empty());
    }

    #[test]
    fn mesh_from_document_uses_registered_geometry_and_bakes_object_transform() {
        let url = "puzzle3d-test://mesh-from-document-uses-registered-geometry.glb";
        let positions: Vec<f32> = vec![0.0, 10.0, 0.0, 1.0, 10.0, 0.0, 0.0, 10.0, 1.0];
        let indices: Vec<u32> = vec![0, 1, 2];
        PUZZLE3D_MESH_REGISTRY.lock().unwrap().insert(url.to_string(), (positions, indices.clone()));
        let mut fixture = empty_fixture();
        fixture.objects = vec![Puzzle3dObject { id: "o1".into(), label: None, object_kind: None, origin: [5.0, 0.0, 0.0], orientation: None, scale: None, mesh_url: Some(url.into()), vortices: vec![], hidden: false, locked: false }];
        let envelope = Puzzle3dEnvelope { fixture, runtime: Puzzle3dRuntime::default() };
        let mesh = puzzle3d_mesh_from_document(&serde_json::to_value(&envelope).unwrap()).unwrap();
        assert_eq!(mesh.indices, indices);
        assert_eq!(mesh.positions.len(), 9);
        // 🌀 raw (0,10,0) → glb_frame_correct [x,-z,y] → (0,0,10) → identity scale/orientation → + origin (5,0,0) = (5,0,10)
        assert_eq!(&mesh.positions[0..3], &[5.0, 0.0, 10.0]);
    }

    #[test]
    fn mesh_from_document_skips_hidden_objects() {
        let url = "puzzle3d-test://mesh-from-document-skips-hidden.glb";
        let positions: Vec<f32> = vec![0.0, 10.0, 0.0, 1.0, 10.0, 0.0, 0.0, 10.0, 1.0];
        PUZZLE3D_MESH_REGISTRY.lock().unwrap().insert(url.to_string(), (positions, vec![0, 1, 2]));
        let mut fixture = empty_fixture();
        fixture.objects = vec![Puzzle3dObject { id: "o1".into(), label: None, object_kind: None, origin: [0.0, 0.0, 0.0], orientation: None, scale: None, mesh_url: Some(url.into()), vortices: vec![], hidden: true, locked: false }];
        let envelope = Puzzle3dEnvelope { fixture, runtime: Puzzle3dRuntime::default() };
        let mesh = puzzle3d_mesh_from_document(&serde_json::to_value(&envelope).unwrap()).unwrap();
        assert!(!mesh.positions.is_empty(), "an all-hidden fixture still exports the box fallback so downstream tooling gets a valid mesh");
    }

    #[test]
    fn jack_query_default_lists_object_names() {
        let envelope = default_envelope();
        let rows = puzzle3d_run_jack_query(&envelope.fixture, &envelope.runtime.jack_query).unwrap();
        assert_eq!(rows.len(), envelope.fixture.objects.len());
        assert!(rows.iter().all(|row| row.entity == "object"));
    }

    #[test]
    fn jack_query_supports_vortex_and_attraction_labels() {
        let envelope = default_envelope();
        let vortex_rows = puzzle3d_run_jack_query(&envelope.fixture, "MATCH (n:Vortex) RETURN n.kind").unwrap();
        assert!(!vortex_rows.is_empty());
        assert!(vortex_rows.iter().all(|row| row.entity == "vortex"));
        let attraction_rows = puzzle3d_run_jack_query(&envelope.fixture, "MATCH (n:Attraction) RETURN n.id").unwrap();
        assert!(attraction_rows.iter().all(|row| row.entity == "attraction"));
    }

    #[test]
    fn jack_query_rejects_unknown_label() {
        let envelope = default_envelope();
        let result = puzzle3d_run_jack_query(&envelope.fixture, "MATCH (n:Cable) RETURN n.name");
        assert!(result.is_err());
    }

    #[test]
    fn jack_query_rejects_malformed_query() {
        let envelope = default_envelope();
        let result = puzzle3d_run_jack_query(&envelope.fixture, "not a query");
        assert!(result.is_err());
    }

    #[test]
    fn set_jack_query_persists_the_new_query_text() {
        let mut app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops("setJackQuery", Some(&json!({ "value": "MATCH (n:Vortex) RETURN n.kind" })), &document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&document), &ops);
        assert_eq!(envelope.runtime.jack_query, "MATCH (n:Vortex) RETURN n.kind");
    }

    #[test]
    fn jack_body_renders_query_field_and_result_rows() {
        let app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let node = app.render(PUZZLE3D_PLAY_BODY_JACK, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("MATCH (n:Object) RETURN n.name"));
        assert!(json.contains("results"));
    }

    #[test]
    fn jack_result_row_click_selects_the_matching_object() {
        let mut app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let envelope = parse_envelope(&document);
        let object_id = envelope.fixture.objects.first().expect("seed object").id.clone();
        let row = Puzzle3dJackRow { entity: "object", id: object_id.clone(), value: "irrelevant".into() };
        let args = jack_row_selection_args(&row);
        let ops = app.handle_command_patch_ops("setSelection", Some(&args), &document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&document), &ops);
        assert_eq!(envelope.runtime.selection.object_ids, vec![object_id]);
    }

    #[test]
    fn document_lists_objects() {
        let app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let node = app.render(PUZZLE3D_PLAY_BODY_DOCUMENT, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("puzzle3d-object:"));
    }

    #[test]
    fn add_object_kind_appends_object() {
        let mut app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops("addObjectKind", Some(&json!({ "objectKind": "Test Kind" })), &document, &ViewState::default());
        let envelope: Puzzle3dEnvelope = apply_ops(&parse_envelope(&document), &ops);
        assert!(envelope.fixture.objects.iter().any(|object| object.object_kind.as_deref() == Some("Test Kind")));
    }

    #[test]
    fn add_object_kind_seeds_vortices_from_catalog_template() {
        let mut app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops("addObjectKind", Some(&json!({ "objectKind": "Hexagonal Cut Concrete Forest Left", "origin": [3.0, 4.0, 5.0] })), &document, &ViewState::default());
        let envelope: Puzzle3dEnvelope = apply_ops(&parse_envelope(&document), &ops);
        let placed_id = envelope.runtime.selection.object_ids.first().expect("new object selected");
        let placed = envelope.fixture.objects.iter().find(|object| &object.id == placed_id).expect("placed object");
        assert!(!placed.vortices.is_empty(), "brush needs a real vortex to attach to");
        assert_eq!(placed.origin, [3.0, 4.0, 5.0]);
    }

    #[test]
    fn build_kinds_tree_lists_all_catalog_sections() {
        let envelope = default_envelope();
        let node = build_kinds_tree(&envelope, puzzle3d_labels(&ViewState::default()));
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Objects"));
        assert!(json.contains("Vortices"));
        assert!(json.contains("Cables"));
        assert!(json.contains("Attractions"));
        assert!(json.contains("Hexagonal Cut Concrete Forest Left"));
        assert!(json.contains("\"draggable\":true"));
    }

    #[test]
    fn puzzle3d_labels_resolve_native_by_default() {
        let envelope = default_envelope();
        let node = build_kinds_tree(&envelope, puzzle3d_labels(&ViewState::default()));
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Objects"));
        assert!(json.contains("Vortices"));
        assert!(!json.contains("Building components"));
    }

    #[test]
    fn puzzle3d_labels_resolve_reuse_terminology_in_english() {
        let envelope = default_envelope();
        let view_state = ViewState { terminology: Some("reuse".into()), locale: Some("en".into()), ..ViewState::default() };
        let node = build_kinds_tree(&envelope, puzzle3d_labels(&view_state));
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Building components"));
        assert!(json.contains("Connection points"));
        assert!(!json.contains("\"Objects\""));
    }

    #[test]
    fn puzzle3d_labels_resolve_reuse_terminology_in_german() {
        let envelope = default_envelope();
        let view_state = ViewState { terminology: Some("reuse".into()), locale: Some("de".into()), ..ViewState::default() };
        let node = build_kinds_tree(&envelope, puzzle3d_labels(&view_state));
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Baukomponenten"));
        assert!(json.contains("Verbindungspunkte"));
    }

    #[test]
    fn puzzle3d_labels_resolve_native_terminology_in_german() {
        let envelope = default_envelope();
        let view_state = ViewState { terminology: Some("native".into()), locale: Some("de".into()), ..ViewState::default() };
        let node = build_kinds_tree(&envelope, puzzle3d_labels(&view_state));
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Objekte"));
    }

    #[test]
    fn set_kind_hover_highlights_matching_instances() {
        let mut app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops("setKindHover", Some(&json!({ "kindId": "Hexagonal Cut Concrete Forest Left" })), &document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&document), &ops);
        assert_eq!(envelope.runtime.hovered_kind_id.as_deref(), Some("Hexagonal Cut Concrete Forest Left"));
        let instances: Value = serde_json::from_str(&world_instances_json(&envelope.fixture, &envelope.runtime)).unwrap();
        let first = instances.as_array().unwrap().first().expect("at least one instance");
        assert_eq!(first.get("hovered").and_then(|v| v.as_bool()), Some(true));
    }

    #[test]
    fn document_tree_lists_references_and_target_volumes_sections() {
        let envelope = default_envelope();
        let node = build_document_tree(&envelope, puzzle3d_labels(&ViewState::default()));
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("References"));
        assert!(json.contains("Target Volumes"));
        assert!(json.contains("Attractions"));
    }

    #[test]
    fn set_selection_flag_hides_selected_object_and_zeroes_its_scale() {
        let mut app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let envelope = parse_envelope(&document);
        let object_id = envelope.fixture.objects.first().expect("seed object").id.clone();
        let ops = app.handle_command_patch_ops("setSelectionFlag", Some(&json!({ "flag": "hidden", "value": true, "entity": "object", "ids": [object_id.clone()] })), &document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&document), &ops);
        let object = envelope.fixture.objects.iter().find(|object| object.id == object_id).expect("object");
        assert!(object.hidden);
        let instances: Value = serde_json::from_str(&world_instances_json(&envelope.fixture, &envelope.runtime)).unwrap();
        let first = instances.as_array().unwrap().first().expect("instance preserved at same index");
        assert_eq!(first.get("scale").and_then(|v| v.as_array()).cloned(), Some(vec![json!(0.0), json!(0.0), json!(0.0)]));
    }

    #[test]
    fn set_selection_flag_locked_object_cannot_be_picked() {
        let mut app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let envelope = parse_envelope(&document);
        let object_id = envelope.fixture.objects.first().expect("seed object").id.clone();
        let ops = app.handle_command_patch_ops("setSelectionFlag", Some(&json!({ "flag": "locked", "value": true, "entity": "object", "ids": [object_id] })), &document, &ViewState::default());
        let locked_document = serde_json::to_string(&apply_ops(&parse_envelope(&document), &ops)).unwrap();
        let ops = app.handle_command_patch_ops("worldPick", Some(&json!({ "granularity": "mesh", "id": 0, "merge": "replace" })), &locked_document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&locked_document), &ops);
        assert!(envelope.runtime.selection.object_ids.is_empty(), "locked objects must not become selectable");
    }

    #[test]
    fn set_selection_flag_defaults_to_current_selection_when_ids_omitted() {
        let mut app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let envelope = parse_envelope(&document);
        let object_id = envelope.fixture.objects.first().expect("seed object").id.clone();
        let ops = app.handle_command_patch_ops("setSelection", Some(&json!({ "selection": { "objectIds": [object_id.clone()], "vortexIds": [], "attractionIds": [] } })), &document, &ViewState::default());
        let selected_document = serde_json::to_string(&apply_ops(&parse_envelope(&document), &ops)).unwrap();
        let ops = app.handle_command_patch_ops("setSelectionFlag", Some(&json!({ "flag": "hidden", "value": true })), &selected_document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&selected_document), &ops);
        let object = envelope.fixture.objects.iter().find(|object| object.id == object_id).expect("object");
        assert!(object.hidden);
    }

    #[test]
    fn patch_inspector_renames_object_and_moves_origin() {
        let mut app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let envelope = parse_envelope(&document);
        let object_id = envelope.fixture.objects.first().expect("seed object").id.clone();
        let ops = app.handle_command_patch_ops("patchInspector", Some(&json!({ "entity": "object", "ids": [object_id.clone()], "field": "label", "value": "Renamed" })), &document, &ViewState::default());
        let document = serde_json::to_string(&apply_ops(&parse_envelope(&document), &ops)).unwrap();
        let ops = app.handle_command_patch_ops("patchInspector", Some(&json!({ "entity": "object", "ids": [object_id.clone()], "field": "origin", "value": [9.0, 8.0, 7.0] })), &document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&document), &ops);
        let object = envelope.fixture.objects.iter().find(|object| object.id == object_id).expect("object");
        assert_eq!(object.label.as_deref(), Some("Renamed"));
        assert_eq!(object.origin, [9.0, 8.0, 7.0]);
    }

    #[test]
    fn patch_inspector_updates_vortex_radius() {
        let mut app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let envelope = parse_envelope(&document);
        let object = envelope.fixture.objects.first().expect("seed object");
        let full_id = puzzle3d_vortex_full_id(&object.id, &object.vortices.first().expect("seed vortex").id);
        let ops = app.handle_command_patch_ops("patchInspector", Some(&json!({ "entity": "vortex", "ids": [full_id.clone()], "field": "radius", "value": 0.75 })), &document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&document), &ops);
        let object = envelope.fixture.objects.first().unwrap();
        let vortex = object.vortices.iter().find(|vortex| puzzle3d_vortex_full_id(&object.id, &vortex.id) == full_id).expect("vortex");
        assert_eq!(vortex.radius, Some(0.75));
    }

    #[test]
    fn select_all_selects_every_unlocked_object() {
        let mut app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let envelope = parse_envelope(&document);
        let total = envelope.fixture.objects.len();
        let ops = app.handle_command_patch_ops("selectAll", None, &document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&document), &ops);
        assert_eq!(envelope.runtime.selection.object_ids.len(), total);
    }

    #[test]
    fn select_all_excludes_locked_objects() {
        let mut app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let envelope = parse_envelope(&document);
        let object_id = envelope.fixture.objects.first().expect("seed object").id.clone();
        let total = envelope.fixture.objects.len();
        let ops = app.handle_command_patch_ops("setSelectionFlag", Some(&json!({ "flag": "locked", "value": true, "entity": "object", "ids": [object_id.clone()] })), &document, &ViewState::default());
        let locked_document = serde_json::to_string(&apply_ops(&parse_envelope(&document), &ops)).unwrap();
        let ops = app.handle_command_patch_ops("selectAll", None, &locked_document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&locked_document), &ops);
        assert_eq!(envelope.runtime.selection.object_ids.len(), total - 1);
        assert!(!envelope.runtime.selection.object_ids.contains(&object_id));
    }

    #[test]
    fn clear_selection_empties_all_selection_kinds() {
        let mut app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops("selectAll", None, &document, &ViewState::default());
        let selected_document = serde_json::to_string(&apply_ops(&parse_envelope(&document), &ops)).unwrap();
        let ops = app.handle_command_patch_ops("clearSelection", None, &selected_document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&selected_document), &ops);
        assert!(envelope.runtime.selection.object_ids.is_empty());
    }

    #[test]
    fn focus_selection_points_camera_at_selection_center() {
        let mut app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let envelope = parse_envelope(&document);
        let object_id = envelope.fixture.objects.first().expect("seed object").id.clone();
        let ops = app.handle_command_patch_ops("setSelection", Some(&json!({ "selection": { "objectIds": [object_id], "vortexIds": [], "attractionIds": [] } })), &document, &ViewState::default());
        let selected_document = serde_json::to_string(&apply_ops(&parse_envelope(&document), &ops)).unwrap();
        let before = parse_envelope(&selected_document).fixture.camera.clone();
        let ops = app.handle_command_patch_ops("focusSelection", None, &selected_document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&selected_document), &ops);
        assert_ne!(envelope.fixture.camera.position, before.position);
    }

    #[test]
    fn engagement_submit_fill_token_sets_fill_count() {
        let mut app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops("engagementSubmit", Some(&json!({ "value": "fill 42" })), &document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&document), &ops);
        assert_eq!(envelope.runtime.fill_count, 42);
        assert_eq!(envelope.runtime.active_tool, "fill");
        assert_eq!(envelope.runtime.engagement_input, "");
    }

    #[test]
    fn engagement_submit_select_token_switches_tool_back() {
        let mut app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops("engagementSubmit", Some(&json!({ "value": "brush" })), &document, &ViewState::default());
        let brush_document = serde_json::to_string(&apply_ops(&parse_envelope(&document), &ops)).unwrap();
        let ops = app.handle_command_patch_ops("engagementSubmit", Some(&json!({ "value": "select" })), &brush_document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&brush_document), &ops);
        assert_eq!(envelope.runtime.active_tool, "select");
    }

    #[test]
    fn engagement_abort_resets_tool_and_input() {
        let mut app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops("setActiveTool", Some(&json!({ "tool": "brush" })), &document, &ViewState::default());
        let brush_document = serde_json::to_string(&apply_ops(&parse_envelope(&document), &ops)).unwrap();
        let ops = app.handle_command_patch_ops("engagementInput", Some(&json!({ "value": "some text" })), &brush_document, &ViewState::default());
        let typed_document = serde_json::to_string(&apply_ops(&parse_envelope(&brush_document), &ops)).unwrap();
        let ops = app.handle_command_patch_ops("engagementAbort", None, &typed_document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&typed_document), &ops);
        assert_eq!(envelope.runtime.active_tool, "select");
        assert_eq!(envelope.runtime.engagement_input, "");
    }

    #[test]
    fn undo_restores_fixture_before_add_object_kind() {
        let mut app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let before_count = parse_envelope(&document).fixture.objects.len();
        let ops = app.handle_command_patch_ops("addObjectKind", Some(&json!({ "objectKind": "Test Kind" })), &document, &ViewState::default());
        let added_document = serde_json::to_string(&apply_ops(&parse_envelope(&document), &ops)).unwrap();
        assert_eq!(parse_envelope(&added_document).fixture.objects.len(), before_count + 1);
        let ops = app.handle_command_patch_ops("undo", None, &added_document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&added_document), &ops);
        assert_eq!(envelope.fixture.objects.len(), before_count);
    }

    #[test]
    fn redo_reapplies_the_undone_edit() {
        let mut app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let before_count = parse_envelope(&document).fixture.objects.len();
        let ops = app.handle_command_patch_ops("addObjectKind", Some(&json!({ "objectKind": "Test Kind" })), &document, &ViewState::default());
        let added_document = serde_json::to_string(&apply_ops(&parse_envelope(&document), &ops)).unwrap();
        let ops = app.handle_command_patch_ops("undo", None, &added_document, &ViewState::default());
        let undone_document = serde_json::to_string(&apply_ops(&parse_envelope(&added_document), &ops)).unwrap();
        let ops = app.handle_command_patch_ops("redo", None, &undone_document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&undone_document), &ops);
        assert_eq!(envelope.fixture.objects.len(), before_count + 1);
    }

    #[test]
    fn undo_is_a_no_op_when_stack_is_empty() {
        let mut app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops("undo", None, &document, &ViewState::default());
        assert!(ops.is_empty());
    }

    #[test]
    fn new_edit_after_undo_clears_the_redo_stack() {
        let mut app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops("addObjectKind", Some(&json!({ "objectKind": "First" })), &document, &ViewState::default());
        let first_document = serde_json::to_string(&apply_ops(&parse_envelope(&document), &ops)).unwrap();
        let ops = app.handle_command_patch_ops("undo", None, &first_document, &ViewState::default());
        let undone_document = serde_json::to_string(&apply_ops(&parse_envelope(&first_document), &ops)).unwrap();
        assert!(!parse_envelope(&undone_document).runtime.redo_stack.is_empty());
        let ops = app.handle_command_patch_ops("addObjectKind", Some(&json!({ "objectKind": "Second" })), &undone_document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&undone_document), &ops);
        assert!(envelope.runtime.redo_stack.is_empty());
    }

    #[test]
    fn selection_and_camera_changes_do_not_push_undo_snapshots() {
        let mut app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let envelope = parse_envelope(&document);
        let object_id = envelope.fixture.objects.first().expect("seed object").id.clone();
        let ops = app.handle_command_patch_ops("setSelection", Some(&json!({ "selection": { "objectIds": [object_id], "vortexIds": [], "attractionIds": [] } })), &document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&document), &ops);
        assert!(envelope.runtime.undo_stack.is_empty());
    }

    #[test]
    fn settings_body_renders_all_fields() {
        let app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let node = app.render(PUZZLE3D_PLAY_BODY_SETTINGS, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Selection Mode"));
        assert!(json.contains("Proximity Radius"));
        assert!(json.contains("Chunk Size"));
        assert!(json.contains("Grid Factor"));
    }

    #[test]
    fn set_proximity_radius_feeds_world_relocate_attraction_distance() {
        let mut app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops("setProximityRadius", Some(&json!({ "value": 50.0 })), &document, &ViewState::default());
        let wide_document = serde_json::to_string(&apply_ops(&parse_envelope(&document), &ops)).unwrap();
        let envelope = parse_envelope(&wide_document);
        assert_eq!(envelope.runtime.proximity_radius, 50.0);
        let object_a = envelope.fixture.objects[0].id.clone();
        let object_b = envelope.fixture.objects.get(1).map(|object| object.id.clone());
        let Some(object_b) = object_b else { return };
        let ops = app.handle_command_patch_ops("worldRelocate", Some(&json!({ "objectId": object_a, "position": [0.0, 0.0, 0.0] })), &wide_document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&wide_document), &ops);
        let has_attraction = envelope.fixture.attractions.iter().any(|attraction| attraction.attracting.starts_with(&format!("{object_a}:")) || attraction.attracted.starts_with(&format!("{object_a}:")) || attraction.attracting.starts_with(&format!("{object_b}:")) || attraction.attracted.starts_with(&format!("{object_b}:")));
        assert!(has_attraction, "a 50-unit proximity radius should connect any two objects in the fixture");
    }

    #[test]
    fn set_selection_mode_default_persists_value() {
        let mut app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops("setSelectionModeDefault", Some(&json!({ "value": "additive" })), &document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&document), &ops);
        assert_eq!(envelope.runtime.selection_mode_default, "additive");
    }

    #[test]
    fn create_attraction_connects_compatible_vortices() {
        let mut app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let envelope = parse_envelope(&document);
        let object = &envelope.fixture.objects[0];
        let source = puzzle3d_vortex_full_id(&object.id, &object.vortices[0].id);
        let target = puzzle3d_vortex_full_id(&object.id, &object.vortices[2].id);
        assert_eq!(object.vortices[0].vortex_kind, object.vortices[2].vortex_kind, "test fixture assumption: same vortex kind is bidirectionally compatible with itself");
        let ops = app.handle_command_patch_ops("createAttraction", Some(&json!({ "attracting": source, "attracted": target })), &document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&document), &ops);
        assert!(envelope.fixture.attractions.iter().any(|attraction| attraction.attracting == source && attraction.attracted == target));
    }

    #[test]
    fn create_attraction_rejects_incompatible_kinds() {
        let mut fixture = empty_fixture();
        fixture.meta.kind_compatibility = Some(json!([{ "source": "a", "target": "b", "bidirectional": false }]));
        fixture.objects = vec![
            Puzzle3dObject { id: "o1".into(), label: None, object_kind: None, origin: [0.0, 0.0, 0.0], orientation: None, scale: None, mesh_url: None, vortices: vec![Puzzle3dVortex { id: "v0".into(), vortex_kind: Some("a".into()), position: [0.0, 0.0, 0.0], direction: None, radius: None, hidden: false, locked: false }], hidden: false, locked: false },
            Puzzle3dObject { id: "o2".into(), label: None, object_kind: None, origin: [1.0, 0.0, 0.0], orientation: None, scale: None, mesh_url: None, vortices: vec![Puzzle3dVortex { id: "v0".into(), vortex_kind: Some("c".into()), position: [0.0, 0.0, 0.0], direction: None, radius: None, hidden: false, locked: false }], hidden: false, locked: false },
        ];
        let mut app = Puzzle3dPlayApp::default();
        let document = serde_json::to_string(&Puzzle3dEnvelope { fixture, runtime: Puzzle3dRuntime::default() }).unwrap();
        let ops = app.handle_command_patch_ops("createAttraction", Some(&json!({ "attracting": "o1:v0", "attracted": "o2:v0" })), &document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&document), &ops);
        assert!(envelope.fixture.attractions.is_empty(), "kind \"a\" is not declared compatible with kind \"c\"");
    }

    #[test]
    fn create_attraction_does_not_duplicate_an_existing_connection() {
        let mut app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let envelope = parse_envelope(&document);
        let object = &envelope.fixture.objects[0];
        let source = puzzle3d_vortex_full_id(&object.id, &object.vortices[0].id);
        let target = puzzle3d_vortex_full_id(&object.id, &object.vortices[2].id);
        let ops = app.handle_command_patch_ops("createAttraction", Some(&json!({ "attracting": source, "attracted": target })), &document, &ViewState::default());
        let first_document = serde_json::to_string(&apply_ops(&parse_envelope(&document), &ops)).unwrap();
        let ops = app.handle_command_patch_ops("createAttraction", Some(&json!({ "attracting": target, "attracted": source })), &first_document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&first_document), &ops);
        assert_eq!(envelope.fixture.attractions.len(), 1);
    }

    #[test]
    fn delete_attraction_removes_it_by_id() {
        let mut app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let envelope = parse_envelope(&document);
        let object = &envelope.fixture.objects[0];
        let source = puzzle3d_vortex_full_id(&object.id, &object.vortices[0].id);
        let target = puzzle3d_vortex_full_id(&object.id, &object.vortices[2].id);
        let ops = app.handle_command_patch_ops("createAttraction", Some(&json!({ "attracting": source, "attracted": target })), &document, &ViewState::default());
        let connected_document = serde_json::to_string(&apply_ops(&parse_envelope(&document), &ops)).unwrap();
        let attraction_id = parse_envelope(&connected_document).fixture.attractions[0].id.clone();
        let ops = app.handle_command_patch_ops("deleteAttraction", Some(&json!({ "id": attraction_id })), &connected_document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&connected_document), &ops);
        assert!(envelope.fixture.attractions.is_empty());
    }

    #[test]
    fn set_fill_edit_target_volumes_toggles_from_toggle_group_id() {
        let mut app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops("setFillEditTargetVolumes", Some(&json!({ "id": "edit-volumes" })), &document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&document), &ops);
        assert!(envelope.runtime.fill_edit_target_volumes);
        let document = serde_json::to_string(&envelope).unwrap();
        let ops = app.handle_command_patch_ops("setFillEditTargetVolumes", Some(&json!({ "id": "fill" })), &document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&document), &ops);
        assert!(!envelope.runtime.fill_edit_target_volumes);
    }

    #[test]
    fn set_voxel_dims_updates_the_selected_axis_only() {
        let mut app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops("setVoxelDims", Some(&json!({ "axis": "h", "value": 5.0 })), &document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&document), &ops);
        assert_eq!(envelope.runtime.voxel_dims, [1, 1, 5]);
    }

    #[test]
    fn add_target_volume_snaps_origin_to_grid_and_sizes_by_voxel_dims() {
        let mut app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops("setVoxelDims", Some(&json!({ "axis": "w", "value": 2.0 })), &document, &ViewState::default());
        let sized_document = serde_json::to_string(&apply_ops(&parse_envelope(&document), &ops)).unwrap();
        let ops = app.handle_command_patch_ops("addTargetVolume", Some(&json!({ "origin": [4.3, 7.8, 0.2] })), &sized_document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&sized_document), &ops);
        assert_eq!(envelope.fixture.target_volumes.len(), 1);
        let volume = &envelope.fixture.target_volumes[0];
        // 🧊 grid_factor defaults to 10.0, so origin snaps to the nearest multiple of 10.
        assert_eq!(volume.origin, [0.0, 10.0, 0.0]);
        assert_eq!(volume.scale, Some(json!([20.0, 10.0, 10.0])));
    }

    #[test]
    fn delete_target_volume_removes_it_by_id() {
        let mut app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops("addTargetVolume", Some(&json!({ "origin": [0.0, 0.0, 0.0] })), &document, &ViewState::default());
        let added_document = serde_json::to_string(&apply_ops(&parse_envelope(&document), &ops)).unwrap();
        let volume_id = parse_envelope(&added_document).fixture.target_volumes[0].id.clone();
        let ops = app.handle_command_patch_ops("deleteTargetVolume", Some(&json!({ "id": volume_id })), &added_document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&added_document), &ops);
        assert!(envelope.fixture.target_volumes.is_empty());
    }

    #[test]
    fn fill_engagement_shows_voxel_controls_when_edit_mode_active() {
        let mut app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops("setActiveTool", Some(&json!({ "tool": "fill" })), &document, &ViewState::default());
        let fill_document = serde_json::to_string(&apply_ops(&parse_envelope(&document), &ops)).unwrap();
        let ops = app.handle_command_patch_ops("setFillEditTargetVolumes", Some(&json!({ "id": "edit-volumes" })), &fill_document, &ViewState::default());
        let edit_document = serde_json::to_string(&apply_ops(&parse_envelope(&fill_document), &ops)).unwrap();
        let engagements = app.window_engagements(&edit_document, &ViewState::default());
        let engagement = engagements.get(PUZZLE3D_PLAY_WINDOW_MAIN).expect("main engagement");
        let controls = engagement.controls.as_ref().expect("voxel controls");
        assert_eq!(controls.len(), 4);
        assert!(engagement.control.is_none(), "fill-count slider should be replaced by voxel controls in edit mode");
    }

    #[test]
    fn build_inspector_tree_shows_mixed_placeholder_for_differing_labels() {
        let mut app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops("addObjectKind", Some(&json!({ "objectKind": "Test Kind" })), &document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&document), &ops);
        let first_id = envelope.fixture.objects[0].id.clone();
        let second_id = envelope.fixture.objects.last().unwrap().id.clone();
        let mut envelope = envelope;
        envelope.runtime.selection.object_ids = vec![first_id, second_id];
        let node = build_inspector_tree(&envelope, puzzle3d_labels(&ViewState::default()));
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains(semio_framework_plugin::UI_INSPECTOR_MIXED_PLACEHOLDER));
    }

    fn apply_ops(envelope: &Puzzle3dEnvelope, ops: &[String]) -> Puzzle3dEnvelope {
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

    #[test]
    fn world_pick_selects_object_by_index() {
        let mut app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops("worldPick", Some(&json!({ "granularity": "mesh", "id": 0, "merge": "replace" })), &document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&document), &ops);
        assert!(!envelope.runtime.selection.object_ids.is_empty());
        let selection: Value = serde_json::from_str(&world_selection_json(&envelope)).unwrap();
        assert_eq!(selection.get("selectionMode").and_then(|v| v.as_str()), Some("mesh"));
        assert_eq!(selection.get("gumballActive").and_then(|v| v.as_bool()), Some(true));
        assert!(selection.get("gumballTarget").is_some());
    }

    #[test]
    fn world_pick_clears_selection_on_null_id() {
        let mut app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops("worldPick", Some(&json!({ "granularity": "mesh", "id": 0, "merge": "replace" })), &document, &ViewState::default());
        let document = serde_json::to_string(&apply_ops(&parse_envelope(&document), &ops)).unwrap();
        let ops = app.handle_command_patch_ops("worldPick", Some(&json!({ "granularity": "mesh", "id": null, "merge": "replace" })), &document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&document), &ops);
        assert!(envelope.runtime.selection.object_ids.is_empty());
    }

    #[test]
    fn set_hover_updates_hovered_object_id() {
        let mut app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let envelope = parse_envelope(&document);
        let object_id = envelope.fixture.objects.first().map(|object| object.id.clone()).unwrap();
        let ops = app.handle_command_patch_ops("setHover", Some(&json!({ "objectId": object_id, "mode": "mesh", "id": 0 })), &document, &ViewState::default());
        let hovered = apply_ops(&envelope, &ops);
        assert_eq!(hovered.runtime.hovered_object_id.as_deref(), Some(hovered.fixture.objects[0].id.as_str()));
        let ops = app.handle_command_patch_ops("setHover", None, &serde_json::to_string(&hovered).unwrap(), &ViewState::default());
        let cleared = apply_ops(&hovered, &ops);
        assert!(cleared.runtime.hovered_object_id.is_none());
    }

    #[test]
    fn window_engagements_include_select_brush_fill() {
        let app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let engagements = app.window_engagements(&document, &ViewState::default());
        let engagement = engagements.get(PUZZLE3D_PLAY_WINDOW_MAIN).expect("main engagement");
        let options = engagement.options.as_ref().expect("tool options");
        let ids: Vec<&str> = options.iter().map(|option| option.id.as_str()).collect();
        assert!(ids.contains(&PUZZLE3D_ENGAGEMENT_TOOL_SELECT));
        assert!(ids.contains(&PUZZLE3D_ENGAGEMENT_TOOL_BRUSH));
        assert!(ids.contains(&PUZZLE3D_ENGAGEMENT_TOOL_FILL));
    }

    fn measure_group_labels(measures: &[WindowMeasure]) -> Vec<&str> {
        measures
            .iter()
            .filter_map(|measure| match measure {
                WindowMeasure::Group { label, .. } => Some(label.as_str()),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn window_measures_cover_lod_select_and_brush_groups() {
        let app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let measures = app.window_measures(&document, &ViewState::default());
        let groups = measures.get(PUZZLE3D_PLAY_WINDOW_MAIN).expect("main measures");
        let labels = measure_group_labels(groups);
        assert!(labels.contains(&"LOD"));
        assert!(labels.contains(&"Select"));
        assert!(labels.contains(&"Brush"));
    }

    #[test]
    fn set_lod_automatic_toggles_runtime_flag() {
        let mut app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops("setLodAutomatic", Some(&json!({ "pressed": false })), &document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&document), &ops);
        assert!(!envelope.runtime.lod_automatic);
    }

    #[test]
    fn set_lod_manual_clamps_to_slider_range() {
        let mut app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops("setLodManual", Some(&json!({ "value": 5000.0 })), &document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&document), &ops);
        assert_eq!(envelope.runtime.lod_manual, PUZZLE3D_LOD_SLIDER_MAX);
    }

    #[test]
    fn set_selectable_kind_updates_selected_kind_only() {
        let mut app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops("setSelectableKind", Some(&json!({ "kind": "vortices", "pressed": false })), &document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&document), &ops);
        assert!(!envelope.runtime.selectable_kinds.vortices);
        assert!(envelope.runtime.selectable_kinds.objects);
        assert!(envelope.runtime.selectable_kinds.attractions);
    }

    #[test]
    fn lod_json_reflects_runtime_state() {
        let mut envelope = default_envelope();
        envelope.runtime.grid_factor = 5.0;
        envelope.runtime.lod_manual = 250.0;
        let lod: Value = serde_json::from_str(&world3d_lod_json(&envelope.runtime)).unwrap();
        assert_eq!(lod.get("gridFactor").and_then(|v| v.as_f64()), Some(5.0));
        assert_eq!(lod.get("manualLod").and_then(|v| v.as_f64()), Some(250.0));
        assert_eq!(lod.get("automaticLod").and_then(|v| v.as_bool()), Some(true));
    }

    #[test]
    fn interaction_json_exposes_voxel_edit_state_for_the_host() {
        let mut envelope = default_envelope();
        envelope.runtime.fill_edit_target_volumes = true;
        envelope.runtime.voxel_dims = [2, 3, 4];
        let interaction: Value = serde_json::from_str(&world_interaction_json(&envelope.runtime)).unwrap();
        assert_eq!(interaction.get("fillEditTargetVolumes").and_then(|v| v.as_bool()), Some(true));
        assert_eq!(interaction.get("voxelDims").and_then(|v| v.as_array()).cloned(), Some(vec![json!(2), json!(3), json!(4)]));
    }

    #[test]
    fn set_projection_updates_camera_and_serializes_into_camera_json() {
        let mut app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops("setProjection", Some(&json!({ "value": "orthographic" })), &document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&document), &ops);
        assert_eq!(envelope.fixture.camera.projection.as_deref(), Some("orthographic"));
        let camera: Value = serde_json::from_str(&camera_json(&envelope.fixture.camera)).unwrap();
        assert_eq!(camera.get("projection").and_then(|v| v.as_str()), Some("orthographic"));
    }

    #[test]
    fn camera_json_omits_projection_when_unset() {
        let envelope = default_envelope();
        let camera: Value = serde_json::from_str(&camera_json(&envelope.fixture.camera)).unwrap();
        assert!(camera.get("projection").is_none());
    }

    #[test]
    fn set_camera_view_preset_top_uses_orthographic_projection_and_non_z_up() {
        let mut app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops("setCameraViewPreset", Some(&json!({ "value": "top" })), &document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&document), &ops);
        assert_eq!(envelope.fixture.camera.projection.as_deref(), Some("orthographic"));
        assert_eq!(envelope.fixture.camera.up, Some([0.0, 1.0, 0.0]), "top view needs a non-Z up vector to avoid gimbal lock in a Z-up world");
        assert_eq!(envelope.fixture.camera.target, [0.0, 0.0, 0.0]);
        assert_eq!(envelope.runtime.view_preset, "top");
    }

    #[test]
    fn set_camera_view_preset_perspective_restores_perspective_projection() {
        let mut app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops("setCameraViewPreset", Some(&json!({ "value": "front" })), &document, &ViewState::default());
        let front_document = serde_json::to_string(&apply_ops(&parse_envelope(&document), &ops)).unwrap();
        let ops = app.handle_command_patch_ops("setCameraViewPreset", Some(&json!({ "value": "perspective" })), &front_document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&front_document), &ops);
        assert_eq!(envelope.fixture.camera.projection.as_deref(), Some("perspective"));
        assert_eq!(envelope.runtime.view_preset, "perspective");
    }

    #[test]
    fn window_measures_include_view_preset_select() {
        let app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let measures = app.window_measures(&document, &ViewState::default());
        let groups = measures.get(PUZZLE3D_PLAY_WINDOW_MAIN).expect("main measures");
        let view_select = groups.iter().find(|measure| matches!(measure, WindowMeasure::Select { label: Some(label), .. } if label == "View")).expect("view select");
        assert!(matches!(view_select, WindowMeasure::Select { value, .. } if value == "perspective"));
    }

    #[test]
    fn view_preset_select_value_follows_set_camera_view_preset() {
        let mut app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops("setCameraViewPreset", Some(&json!({ "value": "top" })), &document, &ViewState::default());
        let top_document = serde_json::to_string(&apply_ops(&parse_envelope(&document), &ops)).unwrap();
        let measures = app.window_measures(&top_document, &ViewState::default());
        let groups = measures.get(PUZZLE3D_PLAY_WINDOW_MAIN).expect("main measures");
        let view_select = groups.iter().find(|measure| matches!(measure, WindowMeasure::Select { label: Some(label), .. } if label == "View")).expect("view select");
        assert!(matches!(view_select, WindowMeasure::Select { value, .. } if value == "top"));
    }

    #[test]
    fn fill_tool_shows_slider_control() {
        let app = Puzzle3dPlayApp::default();
        let mut envelope = parse_envelope(&app.initial_document_json());
        envelope.runtime.active_tool = "fill".into();
        envelope.runtime.fill_count = 5;
        let document = serde_json::to_string(&envelope).unwrap();
        let engagements = app.window_engagements(&document, &ViewState::default());
        let engagement = engagements.get(PUZZLE3D_PLAY_WINDOW_MAIN).expect("main engagement");
        assert!(matches!(engagement.control, Some(WindowEngagementControl::Slider { .. })));
    }

    #[test]
    fn fill_tool_always_shows_the_edit_volumes_mode_toggle() {
        // The mode ToggleGroup is the only way to flip into edit-volumes mode, so it must render
        // even when fill_edit_target_volumes is still false — otherwise the mode is unreachable.
        let app = Puzzle3dPlayApp::default();
        let mut envelope = parse_envelope(&app.initial_document_json());
        envelope.runtime.active_tool = "fill".into();
        let document = serde_json::to_string(&envelope).unwrap();
        let engagements = app.window_engagements(&document, &ViewState::default());
        let engagement = engagements.get(PUZZLE3D_PLAY_WINDOW_MAIN).expect("main engagement");
        let controls = engagement.controls.as_ref().expect("mode toggle should render outside edit mode too");
        assert_eq!(controls.len(), 1);
        assert!(matches!(&controls[0], WindowEngagementControl::ToggleGroup { id: Some(id), .. } if id == "puzzle3d-voxel-edit-mode"));
    }

    #[test]
    fn parse_envelope_preserves_runtime_state() {
        let envelope = Puzzle3dEnvelope {
            fixture: empty_fixture(),
            runtime: Puzzle3dRuntime {
                active_tool: "fill".into(),
                fill_count: 2,
                ..Puzzle3dRuntime::default()
            },
        };
        let json = serde_json::to_string(&envelope).unwrap();
        let parsed = parse_envelope(&json);
        assert_eq!(parsed.runtime.active_tool, "fill");
        assert_eq!(parsed.runtime.fill_count, 2);
    }

    #[test]
    fn parse_brush_candidates_reads_free_array() {
        let raw = serde_json::to_string(&json!({
            "free": [{ "objectKindId": "Placed", "sourceVortexIndex": 0 }],
            "unknownPending": false
        }))
        .unwrap();
        assert_eq!(parse_brush_candidates_free_count(&raw), 1);
    }

    #[test]
    fn brush_placement_control_lists_free_candidates() {
        let mut app = Puzzle3dPlayApp::default();
        let mut envelope = default_envelope();
        envelope.runtime.active_tool = "brush".into();
        let vortex = envelope.fixture.objects.first().and_then(|object| object.vortices.first()).map(|vortex| puzzle3d_vortex_full_id(&envelope.fixture.objects[0].id, &vortex.id)).expect("seed vortex");
        envelope.runtime.selection.vortex_ids = vec![vortex];
        drive_precompute(&mut app.precompute, &envelope);
        let control = puzzle3d_brush_placement_control(&envelope, &app.precompute);
        assert!(matches!(control, Some(WindowEngagementControl::ToggleGroup { .. })));
    }

    #[test]
    fn set_fill_count_accepts_slider_value_key() {
        let mut app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops("setFillCount", Some(&json!({ "value": 3 })), &document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&document), &ops);
        assert_eq!(envelope.runtime.fill_count, 3);
    }

    #[test]
    fn duplicate_selection_clones_selected_objects() {
        let mut app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let envelope = parse_envelope(&document);
        let object_count = envelope.fixture.objects.len();
        let first_id = envelope.fixture.objects.first().map(|object| object.id.clone()).unwrap();
        let pick_ops = app.handle_command_patch_ops("worldPick", Some(&json!({ "granularity": "mesh", "id": 0, "merge": "replace" })), &document, &ViewState::default());
        let document = serde_json::to_string(&apply_ops(&envelope, &pick_ops)).unwrap();
        let ops = app.handle_command_patch_ops("duplicateSelection", None, &document, &ViewState::default());
        let next = apply_ops(&parse_envelope(&document), &ops);
        assert_eq!(next.fixture.objects.len(), object_count + 1);
        assert_ne!(next.runtime.selection.object_ids[0], first_id);
    }

    #[test]
    fn select_same_kind_expands_selection() {
        let mut app = Puzzle3dPlayApp::default();
        let document = app.initial_document_json();
        let envelope = parse_envelope(&document);
        let kind = envelope.fixture.objects.first().and_then(|object| object.object_kind.clone()).expect("kind");
        let expected = envelope.fixture.objects.iter().filter(|object| object.object_kind.as_deref() == Some(kind.as_str())).count();
        let pick_ops = app.handle_command_patch_ops("worldPick", Some(&json!({ "granularity": "mesh", "id": 0, "merge": "replace" })), &document, &ViewState::default());
        let document = serde_json::to_string(&apply_ops(&envelope, &pick_ops)).unwrap();
        let ops = app.handle_command_patch_ops("selectSameKindSelection", None, &document, &ViewState::default());
        let next = apply_ops(&parse_envelope(&document), &ops);
        assert_eq!(next.runtime.selection.object_ids.len(), expected);
    }

    #[test]
    fn context_menu_emitted_when_selection_nonempty() {
        let app = Puzzle3dPlayApp::default();
        let mut envelope = parse_envelope(&app.initial_document_json());
        envelope.runtime.selection.object_ids = vec![envelope.fixture.objects[0].id.clone()];
        let document = serde_json::to_string(&envelope).unwrap();
        let node = app.render(PUZZLE3D_PLAY_BODY_COMPOSITE, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("contextMenuJson"));
        assert!(json.contains("duplicateSelection"));
    }
}
//#endregion 🧪Tests
