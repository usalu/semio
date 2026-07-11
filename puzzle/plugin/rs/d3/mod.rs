//! 🧊 Puzzle 3D plugin — 3D puzzle assembly play app bundled as a hot-swappable WASM component.

use puzzle_3d::{BrushPlacePayload, Puzzle3dPrecomputeSession};
use semio_framework_plugin::{
    build_world_3d_scene, create_default_layout, layout::{MeasureSelectItem, WindowEngagementToggleGroupOption}, merge_world_selection_ids, mesh_from_kind, ui_inspector_groups_to_tree, ui_inspector_readonly_field,
    ui_stack_vertical, ui_text, world3d_chunking_json, world3d_mesh_id_from_url, world3d_meshes_json_from_kinds_and_urls, world3d_meshes_json_from_urls, world3d_scene_extended, world3d_selection_json, App, CommandDescriptor, PanelGroup, PluginApp,
    SurfaceKind, UiControlNode, UiFieldNode, UiInspectorFieldGroup, UiNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, ViewState, WindowEngagement, WindowEngagementControl, WindowEngagementOption, WindowMeasure, FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
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
const PUZZLE3D_PLAY_WINDOW_MAIN: &str = "puzzle3d-main";
const PUZZLE3D_FIXTURE_SCHEMA: &str = "puzzle.3d.fixture";
const PUZZLE3D_EXAMPLE_CONCRETE_FOREST: &str = "concrete-forest";
const PUZZLE3D_EXAMPLE_NAKAGIN: &str = "nakagin-capsule-tower";
const PUZZLE3D_FALLBACK_MESH_KIND: &str = "box";
const PUZZLE3D_ENGAGEMENT_TOOL_BRUSH: &str = "puzzle3d.tool.brush";
const PUZZLE3D_ENGAGEMENT_TOOL_SELECT: &str = "puzzle3d.tool.select";
const PUZZLE3D_ENGAGEMENT_TOOL_FILL: &str = "puzzle3d.tool.fill";
const PUZZLE3D_FILL_COUNT_MAX: u32 = 1000;

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
        }
    }
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
    json!({
        "position": camera.position,
        "target": camera.target,
        "zoom": camera.zoom,
        "fov": 45.0,
    })
    .to_string()
}

fn mesh_selection_ids(args: Option<&Value>, fallback: &[String]) -> Vec<String> {
    args.and_then(|value| value.get("ids")).and_then(|value| serde_json::from_value(value.clone()).ok()).filter(|ids: &Vec<String>| !ids.is_empty()).unwrap_or_else(|| fallback.to_vec())
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
            json!({
                "id": object.id,
                "meshId": mesh_id,
                "position": [
                    object.origin.first().copied().unwrap_or(0.0),
                    object.origin.get(1).copied().unwrap_or(0.0),
                    object.origin.get(2).copied().unwrap_or(0.0),
                ],
                "rotation": object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]),
                "scale": object_scale_json(object),
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
                    Puzzle3dVortex { id: format!("v{index}"), vortex_kind: template.get("vortexKind").and_then(|value| value.as_str()).map(str::to_string), position, direction, radius }
                })
                .collect()
        })
        .unwrap_or_default()
}
//#endregion 🔖Document

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

fn build_document_tree(envelope: &Puzzle3dEnvelope) -> UiNode {
    let object_items: Vec<UiTreeItemNode> = envelope
        .fixture
        .objects
        .iter()
        .map(|object| {
            tree_item_with_command(
                format!("puzzle3d-object:{}", object.id),
                object.object_kind.clone().unwrap_or_else(|| object.id.clone()),
                Some("box"),
                puzzle3d_cmd("setSelection", Some(json!({ "selection": { "objectIds": [object.id], "vortexIds": [], "attractionIds": [] } }))),
            )
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
            UiTreeSectionNode { id: "puzzle3d-play-document.objects".into(), label: Some("Objects".into()), default_open: Some(true), items: object_items },
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

fn build_kinds_tree(envelope: &Puzzle3dEnvelope) -> UiNode {
    let object_entries = puzzle3d_catalog_entries(&envelope.fixture, "objects");
    let vortex_entries = puzzle3d_catalog_entries(&envelope.fixture, "vortices");
    let cable_entries = puzzle3d_catalog_entries(&envelope.fixture, "cables");
    let attraction_entries = puzzle3d_catalog_entries(&envelope.fixture, "attractions");
    UiNode::Tree(UiTreeNode {
        sections: vec![
            UiTreeSectionNode { id: "puzzle3d-play-kinds.objects".into(), label: Some("Objects".into()), default_open: Some(true), items: object_entries.iter().map(puzzle3d_object_kind_item).collect() },
            UiTreeSectionNode { id: "puzzle3d-play-kinds.vortices".into(), label: Some("Vortices".into()), default_open: Some(false), items: vortex_entries.iter().map(|entry| puzzle3d_catalog_kind_item(entry, "circle-dot")).collect() },
            UiTreeSectionNode { id: "puzzle3d-play-kinds.cables".into(), label: Some("Cables".into()), default_open: Some(false), items: cable_entries.iter().map(|entry| puzzle3d_catalog_kind_item(entry, "plug")).collect() },
            UiTreeSectionNode { id: "puzzle3d-play-kinds.attractions".into(), label: Some("Attractions".into()), default_open: Some(false), items: attraction_entries.iter().map(|entry| puzzle3d_catalog_kind_item(entry, "link")).collect() },
        ],
        selected_ids: None,
        highlighted_ids: None,
        selection_change: None,
        drop_command: None,
    })
}

fn build_inspector_tree(envelope: &Puzzle3dEnvelope) -> UiNode {
    if let Some(object_id) = envelope.runtime.selection.object_ids.first() {
        if let Some(object) = envelope.fixture.objects.iter().find(|entry| &entry.id == object_id) {
            return ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
                id: "puzzle3d-play-inspector.object".into(),
                label: "Object".into(),
                default_open: None,
                fields: vec![
                    ui_inspector_readonly_field("puzzle3d-play-inspector.object.id", "Id", &object.id),
                    ui_inspector_readonly_field("puzzle3d-play-inspector.object.kind", "Kind", object.object_kind.as_deref().unwrap_or("")),
                    UiNode::Field(UiFieldNode {
                        id: "puzzle3d-play-inspector.object.origin".into(),
                        label: "Origin".into(),
                        child: Box::new(UiNode::Input(semio_framework_plugin::UiInputNode {
                            id: "puzzle3d-play-inspector.object.origin.input".into(),
                            input_kind: "text".into(),
                            value: format!("{:.2}, {:.2}, {:.2}", object.origin.first().copied().unwrap_or(0.0), object.origin.get(1).copied().unwrap_or(0.0), object.origin.get(2).copied().unwrap_or(0.0),),
                            placeholder: None,
                            commit: None,
                            on_change: puzzle3d_cmd("setSelection", None),
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
            }]);
        }
    }
    ui_stack_vertical(vec![ui_text(format!("Schema: {}", envelope.fixture.schema)), ui_text(format!("Domain: {}", envelope.fixture.domain)), ui_text(format!("Objects: {}", envelope.fixture.objects.len()))])
}
//#endregion 🔖Panels

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

fn puzzle3d_engagement(envelope: &Puzzle3dEnvelope, precompute: &Puzzle3dPrecomputeSession) -> WindowEngagement {
    let object_count = envelope.fixture.objects.len();
    let attraction_count = envelope.fixture.attractions.len();
    let control = match envelope.runtime.active_tool.as_str() {
        "fill" => Some(puzzle3d_fill_count_control(envelope)),
        "brush" => puzzle3d_brush_placement_control(envelope, precompute),
        _ => None,
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
        input: None,
        control,
        controls: None,
        status: Some(vec![semio_framework_plugin::layout::WindowEngagementStatus { id: "puzzle3d-world-status".into(), text: format!("{object_count} objects · {attraction_count} attractions") }]),
        possible_engagements: None,
    }
}

fn puzzle3d_context_menu_json(envelope: &Puzzle3dEnvelope) -> Option<String> {
    if envelope.runtime.selection.object_ids.is_empty() {
        return None;
    }
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

fn puzzle3d_select_measures_group(runtime: &Puzzle3dRuntime) -> WindowMeasure {
    WindowMeasure::Group {
        id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-select"),
        label: "Select".into(),
        default_open: Some(true),
        children: vec![
            WindowMeasure::Toggle { id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-select-rectangle"), icon_id: "square".into(), label: Some("Rectangle".into()), pressed: runtime.selection_method == "rectangle", text: None, on_change: puzzle3d_cmd("setSelectionMethod", Some(json!({ "method": "rectangle" }))) },
            WindowMeasure::Toggle { id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-select-lasso"), icon_id: "lasso".into(), label: Some("Lasso".into()), pressed: runtime.selection_method == "lasso", text: None, on_change: puzzle3d_cmd("setSelectionMethod", Some(json!({ "method": "lasso" }))) },
            WindowMeasure::Toggle { id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-select-objects"), icon_id: "box".into(), label: Some("Objects".into()), pressed: runtime.selectable_kinds.objects, text: None, on_change: puzzle3d_cmd("setSelectableKind", Some(json!({ "kind": "objects" }))) },
            WindowMeasure::Toggle { id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-select-vortices"), icon_id: "circle-dot".into(), label: Some("Vortices".into()), pressed: runtime.selectable_kinds.vortices, text: None, on_change: puzzle3d_cmd("setSelectableKind", Some(json!({ "kind": "vortices" }))) },
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

fn puzzle3d_brush_measures_group(envelope: &Puzzle3dEnvelope) -> WindowMeasure {
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
                        label: "Objects".into(),
                        default_open: Some(false),
                        children: puzzle3d_kind_weight_measures("object-kind", &object_ids, &envelope.runtime.object_kind_weights, "setObjectKindWeight"),
                    },
                    WindowMeasure::Group {
                        id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-brush-distribution-vortices"),
                        label: "Vortices".into(),
                        default_open: Some(false),
                        children: puzzle3d_kind_weight_measures("vortex-kind", &vortex_ids, &envelope.runtime.vortex_kind_weights, "setVortexKindWeight"),
                    },
                ],
            },
        ],
    }
}

fn puzzle3d_window_measures(envelope: &Puzzle3dEnvelope) -> Vec<WindowMeasure> {
    vec![puzzle3d_lod_measures_group(&envelope.runtime), puzzle3d_select_measures_group(&envelope.runtime), puzzle3d_brush_measures_group(envelope)]
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
                });
                envelope.runtime.selection.object_ids = vec![id];
                return vec![set_document_op(&envelope)];
            }
            "deleteSelection" => {
                let ids: Vec<String> = envelope.runtime.selection.object_ids.clone();
                envelope.fixture.objects.retain(|object| !ids.contains(&object.id));
                envelope.runtime.selection.object_ids.clear();
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
                if let Some(object) = envelope.fixture.objects.get(index) {
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
                if let (Some(object), Some(position)) = (envelope.fixture.objects.iter_mut().find(|object| object.id == object_id), position) {
                    object.origin = position;
                    let proximity_radius = 0.75;
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
                envelope.runtime.fill_count = count;
                if count > 0 {
                    envelope.runtime.active_tool = "fill".into();
                    if let Ok(fixture_json) = self.precompute.apply_fill_count_rust(count) {
                        if let Some(next) = fixture_from_engine_json(&envelope, &fixture_json) {
                            envelope = next;
                        }
                    }
                }
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
                }
                return Vec::new();
            }
            "worldPointerDown" => return Vec::new(),
            _ => {}
        };
        Vec::new()
    }

    fn render(&self, body_key: &str, document_json: &str, _view_state: &ViewState) -> UiNode {
        let envelope = parse_envelope(document_json);
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
                        Some(world3d_chunking_json(256.0, 8000.0)),
                        puzzle3d_context_menu_json(&envelope),
                    ),
                )
            }
            PUZZLE3D_PLAY_BODY_DOCUMENT => build_document_tree(&envelope),
            PUZZLE3D_PLAY_BODY_KINDS => build_kinds_tree(&envelope),
            PUZZLE3D_PLAY_BODY_INSPECTOR => build_inspector_tree(&envelope),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn window_engagements(&self, document_json: &str, _view_state: &ViewState) -> HashMap<String, WindowEngagement> {
        let envelope = parse_envelope(document_json);
        HashMap::from([(PUZZLE3D_PLAY_WINDOW_MAIN.into(), puzzle3d_engagement(&envelope, &self.precompute))])
    }

    fn window_measures(&self, document_json: &str, _view_state: &ViewState) -> HashMap<String, Vec<WindowMeasure>> {
        let envelope = parse_envelope(document_json);
        HashMap::from([(PUZZLE3D_PLAY_WINDOW_MAIN.into(), puzzle3d_window_measures(&envelope))])
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
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind_with_engagement(PUZZLE3D_PLAY_WINDOW_MAIN, "Puzzle 3D", PUZZLE3D_PLAY_BODY_COMPOSITE, SurfaceKind::World3d, puzzle3d_engagement(&envelope, &Puzzle3dPrecomputeSession::new()))
            .default_layout(create_default_layout(&[PUZZLE3D_PLAY_WINDOW_MAIN.into()], "row", Some(&[100.0]), Some(&["Puzzle 3D".into()])))
            .panel_tab(FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, PanelGroup::Workbench, PUZZLE3D_PLAY_BODY_DOCUMENT)
            .panel_tab(FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, PanelGroup::Workbench, PUZZLE3D_PLAY_BODY_KINDS)
            .panel_tab(FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, PanelGroup::Details, PUZZLE3D_PLAY_BODY_INSPECTOR),
    )
    .example("empty", "Empty", &serde_json::to_string(&Puzzle3dEnvelope { fixture: empty_fixture(), runtime: Puzzle3dRuntime::default() }).unwrap())
    .example(PUZZLE3D_EXAMPLE_CONCRETE_FOREST, "Concrete Forest", CONCRETE_FOREST_EXAMPLE_JSON)
    .example(PUZZLE3D_EXAMPLE_NAKAGIN, "Nakagin Capsule Tower", NAKAGIN_EXAMPLE_JSON)
    .program("puzzle3d", "Puzzle 3D", "model")
}

fn puzzle3d_mesh_from_document(_doc: &serde_json::Value) -> Result<semio_framework_plugin::MeshData, String> {
    Ok(mesh_from_kind(PUZZLE3D_FALLBACK_MESH_KIND))
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
