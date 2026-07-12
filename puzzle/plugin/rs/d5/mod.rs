//! 👯 Puzzle 5D plugin — paired 2D board + 3D world puzzle play app bundled as a hot-swappable WASM component.

use puzzle_5d::{BrushPlacePayload, Puzzle5dPrecomputeSession};
use semio_framework_os::register_mesh_export_handlers;
use semio_framework_plugin::{
    build_puzzle2d_board_scene, build_world_3d_scene, create_default_layout,
    layout::{MeasureSelectItem, WindowEngagementStatus, WindowEngagementToggleGroupOption},
    merge_world_selection_ids, ui_inspector_groups_to_tree, ui_inspector_readonly_field, ui_stack_vertical, ui_text, world3d_chunking_json, world3d_mesh_id_from_url, world3d_meshes_json_from_urls, world3d_scene_extended, world3d_selection_json, App,
    ActionDescriptor, PanelGroup, PluginApp, Puzzle2dBoardScene, SurfaceKind, UiFieldNode, UiInspectorFieldGroup, UiNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, ViewState, WindowEngagement, WindowEngagementControl,
    WindowEngagementInput, WindowEngagementOption, WindowMeasure, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::atomic::{AtomicU32, Ordering};

//#region 🔖Constants
const PUZZLE5D_PLAY_APP_ID: &str = "puzzle5d-play";
const PUZZLE5D_PLAY_CONTROLLER_ID: &str = "puzzle5d-play";
const PUZZLE5D_PLAY_SURFACE_2D: &str = "puzzle.5d.play.2d";
const PUZZLE5D_PLAY_SURFACE_3D: &str = "puzzle.5d.play.3d";
const PUZZLE5D_PLAY_BODY_2D: &str = "puzzle.5d.play.2d";
const PUZZLE5D_PLAY_BODY_3D: &str = "puzzle.5d.play.3d";
const PUZZLE5D_PLAY_BODY_DOCUMENT: &str = "puzzle.5d.play.document";
const PUZZLE5D_PLAY_BODY_KINDS: &str = "puzzle.5d.play.kinds";
const PUZZLE5D_PLAY_BODY_INSPECTOR: &str = "puzzle.5d.play.inspector";
const PUZZLE5D_PLAY_WINDOW_2D: &str = "puzzle5d-2d";
const PUZZLE5D_PLAY_WINDOW_3D: &str = "puzzle5d-3d";
const PUZZLE5D_PLAY_WINDOWS: [&str; 2] = [PUZZLE5D_PLAY_WINDOW_2D, PUZZLE5D_PLAY_WINDOW_3D];
const PUZZLE5D_SCHEMA: &str = "puzzle.5d";
const PUZZLE5D_BOARD_FIXTURE_SCHEMA: &str = "puzzle.2d.fixture";
const PUZZLE5D_EXAMPLE_CONCRETE_FOREST: &str = "concrete-forest";
const PUZZLE5D_EXAMPLE_NAKAGIN: &str = "nakagin-capsule-tower";

const PUZZLE5D_FALLBACK_MESH_KIND: &str = "box";
const PUZZLE5D_ENGAGEMENT_TOOL_BRUSH: &str = "puzzle5d.tool.brush";
const PUZZLE5D_ENGAGEMENT_TOOL_SELECT: &str = "puzzle5d.tool.select";
const PUZZLE5D_ENGAGEMENT_TOOL_FILL: &str = "puzzle5d.tool.fill";
const PUZZLE5D_FILL_COUNT_MAX: u32 = 1000;
const PUZZLE5D_LOD_MODE_AUTOMATIC: &str = "automatic";
const PUZZLE5D_SUGGESTION_OFFSET_MIN: f64 = 0.0;
const PUZZLE5D_SUGGESTION_OFFSET_MAX: f64 = 160.0;
const PUZZLE5D_SUGGESTION_OFFSET_STEP: f64 = 4.0;
const PUZZLE5D_DEFAULT_SUGGESTION_OFFSET: f64 = 80.0;
const PUZZLE5D_DEFAULT_PART_RADIUS: f64 = 20.0;
const PUZZLE5D_BOARD_PLACEMENT_GAP: f64 = 16.0;
const PUZZLE5D_PROXIMITY_RADIUS: f64 = 0.75;

const CONCRETE_FOREST_EXAMPLE_JSON: &str = include_str!("../../../5d/example/concrete-forest.5d.json");
const NAKAGIN_EXAMPLE_JSON: &str = include_str!("../../../5d/example/nakagin-capsule-tower.5d.json");

static PUZZLE5D_ID_COUNTER: AtomicU32 = AtomicU32::new(0);
//#endregion 🔖Constants

//#region 🔖Terminology
/// 🗣️ Complete UI label set for the 5D app; one field per label makes every locale combination compile-checked.
struct Puzzle5dLabels {
    parts: &'static str,
    fasteners: &'static str,
    grips: &'static str,
    ropes: &'static str,
    part: &'static str,
    grip: &'static str,
    select: &'static str,
    brush: &'static str,
    fill: &'static str,
    placement: &'static str,
    duplicate: &'static str,
    select_same_kind: &'static str,
    zoom_to_selection: &'static str,
    delete: &'static str,
    lod: &'static str,
    automatic: &'static str,
    suggestion: &'static str,
    offset: &'static str,
    part_weights: &'static str,
    grip_weights: &'static str,
    overlap: &'static str,
}

const PUZZLE5D_LABELS_NATIVE_EN: Puzzle5dLabels = Puzzle5dLabels {
    parts: "Parts",
    fasteners: "Fasteners",
    grips: "Grips",
    ropes: "Ropes",
    part: "Part",
    grip: "Grip",
    select: "Select",
    brush: "Brush",
    fill: "Fill",
    placement: "Placement",
    duplicate: "Duplicate",
    select_same_kind: "Select all of same kind",
    zoom_to_selection: "Zoom to selection",
    delete: "Delete",
    lod: "LOD",
    automatic: "Automatic",
    suggestion: "Suggestion",
    offset: "Offset",
    part_weights: "Part Weights",
    grip_weights: "Grip Weights",
    overlap: "Overlap",
};

const PUZZLE5D_LABELS_NATIVE_DE: Puzzle5dLabels = Puzzle5dLabels {
    parts: "Teile",
    fasteners: "Verbinder",
    grips: "Griffe",
    ropes: "Seile",
    part: "Teil",
    grip: "Griff",
    select: "Auswählen",
    brush: "Pinsel",
    fill: "Füllen",
    placement: "Platzierung",
    duplicate: "Duplizieren",
    select_same_kind: "Alle gleicher Art auswählen",
    zoom_to_selection: "Auf Auswahl zoomen",
    delete: "Löschen",
    lod: "LOD",
    automatic: "Automatisch",
    suggestion: "Vorschlag",
    offset: "Versatz",
    part_weights: "Teilgewichte",
    grip_weights: "Griffgewichte",
    overlap: "Überlappung",
};

/// 🗣️ Resolves the active label set from the shell-provided locale; puzzle5d has no alternate terminology, only native language switching.
fn puzzle5d_labels(view_state: &ViewState) -> &'static Puzzle5dLabels {
    let is_de = view_state.locale.as_deref().is_some_and(|locale| locale.starts_with("de"));
    if is_de { &PUZZLE5D_LABELS_NATIVE_DE } else { &PUZZLE5D_LABELS_NATIVE_EN }
}
//#endregion 🔖Terminology

//#region 🔖Document
fn one_f64() -> f64 {
    1.0
}

fn default_selection_method() -> String {
    "rectangle".into()
}

fn default_active_tool() -> String {
    "select".into()
}

fn default_transform_tool() -> String {
    "move".into()
}

fn default_overlap_budget() -> f64 {
    0.02
}

fn default_lod_mode() -> String {
    PUZZLE5D_LOD_MODE_AUTOMATIC.into()
}

fn default_suggestion_offset() -> f64 {
    PUZZLE5D_DEFAULT_SUGGESTION_OFFSET
}

fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Puzzle5dCamera2d {
    #[serde(default)]
    x: f64,
    #[serde(default)]
    y: f64,
    #[serde(default = "one_f64")]
    zoom: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Puzzle5dCamera3d {
    #[serde(default)]
    position: [f64; 3],
    #[serde(default)]
    target: [f64; 3],
    #[serde(default = "one_f64")]
    zoom: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Puzzle5dGrip2d {
    #[serde(default)]
    angle: f64,
    #[serde(default, rename = "gripKind")]
    grip_kind: String,
    #[serde(default)]
    radius: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Puzzle5dGrip3d {
    #[serde(default)]
    position: [f64; 3],
    #[serde(default)]
    direction: Option<[f64; 3]>,
    #[serde(default)]
    radius: f64,
    #[serde(default)]
    label: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Puzzle5dGrip {
    id: String,
    #[serde(default, rename = "gripKind")]
    grip_kind: String,
    #[serde(default, rename = "2d")]
    grip_2d: Puzzle5dGrip2d,
    #[serde(default, rename = "3d")]
    grip_3d: Puzzle5dGrip3d,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Puzzle5dFastener {
    id: String,
    source: String,
    target: String,
    #[serde(default, rename = "fastenerKind", skip_serializing_if = "Option::is_none")]
    fastener_kind: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Puzzle5dPart2d {
    #[serde(default)]
    x: f64,
    #[serde(default)]
    y: f64,
    #[serde(default)]
    shape: String,
    #[serde(default)]
    radius: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    width: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    height: Option<f64>,
    #[serde(default)]
    text: String,
    #[serde(default, rename = "iconKind", skip_serializing_if = "Option::is_none")]
    icon_kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    hidden: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    locked: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Puzzle5dPart3d {
    #[serde(default)]
    origin: [f64; 3],
    #[serde(default, rename = "meshUrl")]
    mesh_url: Option<String>,
    #[serde(default)]
    orientation: Option<[f64; 4]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    scale: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    label: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Puzzle5dPart {
    id: String,
    #[serde(rename = "partKind")]
    part_kind: String,
    #[serde(default, rename = "2d")]
    part_2d: Puzzle5dPart2d,
    #[serde(default, rename = "3d")]
    part_3d: Puzzle5dPart3d,
    #[serde(default)]
    grips: Vec<Puzzle5dGrip>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Puzzle5dDocument {
    schema: String,
    #[serde(default)]
    domain: String,
    #[serde(default)]
    camera2d: Puzzle5dCamera2d,
    #[serde(default)]
    camera3d: Puzzle5dCamera3d,
    #[serde(default)]
    parts: Vec<Puzzle5dPart>,
    #[serde(default)]
    fasteners: Vec<Puzzle5dFastener>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    meta: Option<Value>,
    #[serde(default, rename = "kindCatalogs")]
    kind_catalogs: Option<Value>,
    #[serde(default, rename = "kindCompatibility")]
    kind_compatibility: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    label: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Puzzle5dSelection {
    #[serde(default)]
    part_ids: Vec<String>,
    #[serde(default)]
    grip_ids: Vec<String>,
    #[serde(default)]
    fastener_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Puzzle5dRuntime {
    #[serde(default)]
    selection: Puzzle5dSelection,
    #[serde(default = "default_selection_method")]
    selection_method: String,
    #[serde(default)]
    hovered_part_id: Option<String>,
    #[serde(default = "default_active_tool")]
    active_tool: String,
    #[serde(default = "default_transform_tool")]
    transform_tool: String,
    #[serde(default)]
    fill_count: u32,
    #[serde(default)]
    brush_candidate_index: usize,
    #[serde(default = "default_overlap_budget")]
    overlap_budget: f64,
    #[serde(default = "default_lod_mode")]
    lod_mode: String,
    #[serde(default = "default_suggestion_offset")]
    suggestion_offset: f64,
    #[serde(default = "default_true")]
    grid_snap_enabled: bool,
    #[serde(default = "one_f64")]
    grid_factor: f64,
    #[serde(default)]
    engagement_input_by_window: BTreeMap<String, String>,
    #[serde(default)]
    object_kind_weights: HashMap<String, f64>,
    #[serde(default)]
    vortex_kind_weights: HashMap<String, f64>,
}

/// ⚠️ Explicit impl (not `#[derive(Default)]`) so Rust construction matches the serde field defaults above.
impl Default for Puzzle5dRuntime {
    fn default() -> Self {
        Self {
            selection: Puzzle5dSelection::default(),
            selection_method: default_selection_method(),
            hovered_part_id: None,
            active_tool: default_active_tool(),
            transform_tool: default_transform_tool(),
            fill_count: 0,
            brush_candidate_index: 0,
            overlap_budget: default_overlap_budget(),
            lod_mode: default_lod_mode(),
            suggestion_offset: default_suggestion_offset(),
            grid_snap_enabled: true,
            grid_factor: 1.0,
            engagement_input_by_window: BTreeMap::new(),
            object_kind_weights: HashMap::new(),
            vortex_kind_weights: HashMap::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Puzzle5dEnvelope {
    document: Puzzle5dDocument,
    #[serde(default)]
    runtime: Puzzle5dRuntime,
}

fn empty_document() -> Puzzle5dDocument {
    Puzzle5dDocument {
        schema: PUZZLE5D_SCHEMA.into(),
        domain: "architecture".into(),
        camera2d: Puzzle5dCamera2d { x: 0.0, y: 0.0, zoom: 1.0 },
        camera3d: Puzzle5dCamera3d { position: [8.0, -8.0, 8.0], target: [0.0, 0.0, 0.0], zoom: 1.0 },
        parts: Vec::new(),
        fasteners: Vec::new(),
        meta: None,
        kind_catalogs: None,
        kind_compatibility: None,
        label: None,
    }
}

fn envelope_from_document_json(json_text: &str) -> Option<Puzzle5dEnvelope> {
    serde_json::from_str::<Puzzle5dDocument>(json_text).ok().map(|document| Puzzle5dEnvelope { document, runtime: Puzzle5dRuntime::default() })
}

fn default_envelope() -> Puzzle5dEnvelope {
    envelope_from_document_json(CONCRETE_FOREST_EXAMPLE_JSON).unwrap_or_else(|| Puzzle5dEnvelope { document: empty_document(), runtime: Puzzle5dRuntime::default() })
}

fn parse_envelope(document_json: &str) -> Puzzle5dEnvelope {
    if let Ok(envelope) = serde_json::from_str::<Puzzle5dEnvelope>(document_json) {
        return envelope;
    }
    if let Ok(document) = serde_json::from_str::<Puzzle5dDocument>(document_json) {
        return Puzzle5dEnvelope { document, runtime: Puzzle5dRuntime::default() };
    }
    default_envelope()
}

fn set_document_op(envelope: &Puzzle5dEnvelope) -> String {
    json!({ "op": "setDocument", "document": envelope }).to_string()
}

fn puzzle5d_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor { controller_id: PUZZLE5D_PLAY_CONTROLLER_ID.into(), action: action.into(), args }
}

fn puzzle5d_grip_full_id(part_id: &str, grip_id: &str) -> String {
    if grip_id.contains(':') {
        grip_id.to_string()
    } else {
        format!("{part_id}:{grip_id}")
    }
}

fn next_part_id() -> String {
    let next = PUZZLE5D_ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
    format!("part-{next}")
}

fn next_fastener_id() -> String {
    let next = PUZZLE5D_ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
    format!("fastener-{next}")
}

fn parse_vec3(text: &str) -> Option<[f64; 3]> {
    let values: Vec<f64> = text.split(',').filter_map(|part| part.trim().parse::<f64>().ok()).collect();
    if values.len() < 3 {
        return None;
    }
    Some([values[0], values[1], values[2]])
}

fn resolve_part_mesh_url(part: &Puzzle5dPart, kind_catalogs: Option<&Value>) -> Option<String> {
    if let Some(url) = part.part_3d.mesh_url.as_ref().filter(|url| !url.is_empty()) {
        return Some(url.clone());
    }
    resolve_part_kind_mesh_url(&part.part_kind, kind_catalogs)
}

fn resolve_part_kind_mesh_url(part_kind: &str, kind_catalogs: Option<&Value>) -> Option<String> {
    let parts = kind_catalogs?.get("parts")?.as_array()?;
    parts.iter().find(|entry| entry.get("id").and_then(|v| v.as_str()) == Some(part_kind)).and_then(|entry| entry.get("meshUrl").and_then(|v| v.as_str()).map(str::to_string))
}

fn collect_mesh_urls(document: &Puzzle5dDocument) -> Vec<String> {
    let mut urls = HashSet::new();
    for part in &document.parts {
        if let Some(url) = resolve_part_mesh_url(part, document.kind_catalogs.as_ref()) {
            urls.insert(url);
        }
    }
    if let Some(parts) = document.kind_catalogs.as_ref().and_then(|catalogs| catalogs.get("parts")).and_then(|v| v.as_array()) {
        for entry in parts {
            if let Some(url) = entry.get("meshUrl").and_then(|v| v.as_str()) {
                urls.insert(url.to_string());
            }
        }
    }
    urls.into_iter().collect()
}

fn part_kind_grip_templates(document: &Puzzle5dDocument, part_kind: &str) -> Vec<Value> {
    document
        .kind_catalogs
        .as_ref()
        .and_then(|catalogs| catalogs.get("parts"))
        .and_then(|parts| parts.as_array())
        .and_then(|parts| parts.iter().find(|entry| entry.get("id").and_then(|v| v.as_str()) == Some(part_kind)))
        .and_then(|entry| entry.get("grips"))
        .and_then(|grips| grips.as_array())
        .cloned()
        .unwrap_or_default()
}

fn grips_from_templates(document: &Puzzle5dDocument, part_kind: &str) -> Vec<Puzzle5dGrip> {
    part_kind_grip_templates(document, part_kind)
        .iter()
        .enumerate()
        .map(|(index, template)| {
            let grip_kind = template.get("gripKind").and_then(|v| v.as_str()).unwrap_or("grip").to_string();
            let grip_2d: Puzzle5dGrip2d = template.get("2d").and_then(|v| serde_json::from_value(v.clone()).ok()).unwrap_or_default();
            let grip_3d: Puzzle5dGrip3d = template.get("3d").and_then(|v| serde_json::from_value(v.clone()).ok()).unwrap_or_default();
            Puzzle5dGrip { id: format!("v{index}"), grip_kind, grip_2d, grip_3d }
        })
        .collect()
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

fn world_grip_position(part: &Puzzle5dPart, grip: &Puzzle5dGrip) -> [f64; 3] {
    let orientation = part.part_3d.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
    let rotated = quat_rotate_vector(orientation, grip.grip_3d.position);
    [part.part_3d.origin[0] + rotated[0], part.part_3d.origin[1] + rotated[1], part.part_3d.origin[2] + rotated[2]]
}

fn world_grip_direction(part: &Puzzle5dPart, grip: &Puzzle5dGrip) -> [f64; 3] {
    let direction = grip.grip_3d.direction.unwrap_or([0.0, 0.0, -1.0]);
    quat_rotate_vector(part.part_3d.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), direction)
}

fn resolve_grip_world_position(document: &Puzzle5dDocument, full_id: &str) -> Option<[f64; 3]> {
    for part in &document.parts {
        for grip in &part.grips {
            if puzzle5d_grip_full_id(&part.id, &grip.id) == full_id {
                return Some(world_grip_position(part, grip));
            }
        }
    }
    None
}

fn find_part_by_grip_full_id<'a>(document: &'a Puzzle5dDocument, full_id: &str) -> Option<(&'a Puzzle5dPart, &'a Puzzle5dGrip)> {
    for part in &document.parts {
        for grip in &part.grips {
            if puzzle5d_grip_full_id(&part.id, &grip.id) == full_id {
                return Some((part, grip));
            }
        }
    }
    None
}

fn strip_tree_prefix(id: &str) -> &str {
    for prefix in ["puzzle5d-play-document.part.", "puzzle5d-play-document.grip.", "puzzle5d-play-document.fastener."] {
        if let Some(rest) = id.strip_prefix(prefix) {
            return rest;
        }
    }
    id
}

fn classify_selection(document: &Puzzle5dDocument, ids: &[String]) -> Puzzle5dSelection {
    let part_ids: HashSet<&str> = document.parts.iter().map(|part| part.id.as_str()).collect();
    let fastener_ids: HashSet<&str> = document.fasteners.iter().map(|fastener| fastener.id.as_str()).collect();
    let grip_ids: HashSet<String> = document.parts.iter().flat_map(|part| part.grips.iter().map(|grip| puzzle5d_grip_full_id(&part.id, &grip.id))).collect();
    let mut selection = Puzzle5dSelection::default();
    for raw in ids {
        let id = strip_tree_prefix(raw);
        if part_ids.contains(id) {
            selection.part_ids.push(id.to_string());
        } else if fastener_ids.contains(id) {
            selection.fastener_ids.push(id.to_string());
        } else if grip_ids.contains(id) {
            selection.grip_ids.push(id.to_string());
        }
    }
    selection
}

fn selection_flat_ids(selection: &Puzzle5dSelection) -> Vec<String> {
    selection.part_ids.iter().chain(selection.grip_ids.iter()).chain(selection.fastener_ids.iter()).cloned().collect()
}

fn mesh_selection_ids(args: Option<&Value>, fallback: &[String]) -> Vec<String> {
    args.and_then(|value| value.get("ids")).and_then(|value| serde_json::from_value::<Vec<String>>(value.clone()).ok()).filter(|ids| !ids.is_empty()).unwrap_or_else(|| fallback.to_vec())
}

fn remove_parts(document: &mut Puzzle5dDocument, part_ids: &[String]) {
    let removed_grips: Vec<String> = document.parts.iter().filter(|part| part_ids.contains(&part.id)).flat_map(|part| part.grips.iter().map(|grip| puzzle5d_grip_full_id(&part.id, &grip.id))).collect();
    document.parts.retain(|part| !part_ids.contains(&part.id));
    document.fasteners.retain(|fastener| !removed_grips.contains(&fastener.source) && !removed_grips.contains(&fastener.target));
}

fn remove_grips(document: &mut Puzzle5dDocument, grip_full_ids: &[String]) {
    if grip_full_ids.is_empty() {
        return;
    }
    for part in &mut document.parts {
        let part_id = part.id.clone();
        part.grips.retain(|grip| !grip_full_ids.contains(&puzzle5d_grip_full_id(&part_id, &grip.id)));
    }
    document.fasteners.retain(|fastener| !grip_full_ids.contains(&fastener.source) && !grip_full_ids.contains(&fastener.target));
}
//#endregion 🔖Document

//#region 🔖Board
fn board_camera_value(camera: &Puzzle5dCamera2d) -> Value {
    json!({ "x": camera.x, "y": camera.y, "zoom": camera.zoom })
}

fn board_node_value(part: &Puzzle5dPart) -> Value {
    let shape = if part.part_2d.shape.is_empty() { "circle" } else { part.part_2d.shape.as_str() };
    let handles: Vec<Value> = part
        .grips
        .iter()
        .map(|grip| {
            json!({
                "id": puzzle5d_grip_full_id(&part.id, &grip.id),
                "handleKind": if grip.grip_kind.is_empty() { grip.grip_2d.grip_kind.clone() } else { grip.grip_kind.clone() },
                "angle": grip.grip_2d.angle,
                "radius": if grip.grip_2d.radius > 0.0 { grip.grip_2d.radius } else { 3.0 },
            })
        })
        .collect();
    let mut node = json!({
        "id": part.id,
        "nodeKind": part.part_kind,
        "shape": shape,
        "x": part.part_2d.x,
        "y": part.part_2d.y,
        "text": part.part_2d.text,
        "handles": handles,
    });
    if shape == "rectangle" {
        node["width"] = json!(part.part_2d.width.unwrap_or(48.0));
        node["height"] = json!(part.part_2d.height.unwrap_or(48.0));
    } else {
        node["radius"] = json!(if part.part_2d.radius > 0.0 { part.part_2d.radius } else { PUZZLE5D_DEFAULT_PART_RADIUS });
    }
    if let Some(icon) = part.part_2d.icon_kind.as_ref() {
        node["iconKind"] = json!(icon);
    }
    if let Some(hidden) = part.part_2d.hidden {
        node["hidden"] = json!(hidden);
    }
    if let Some(locked) = part.part_2d.locked {
        node["locked"] = json!(locked);
    }
    node
}

/// 🗂️ Projects the unified 5d kind bundle (`parts/grips/fasteners/ropes`) to the board's `nodes/handles/edges/wires` naming.
fn board_kind_catalogs_value(document: &Puzzle5dDocument) -> Value {
    let catalogs = document.kind_catalogs.clone().unwrap_or(json!({}));
    json!({
        "nodes": catalogs.get("parts").cloned().unwrap_or(json!([])),
        "handles": catalogs.get("grips").cloned().unwrap_or(json!([])),
        "edges": catalogs.get("fasteners").cloned().unwrap_or(json!([])),
        "wires": catalogs.get("ropes").cloned().unwrap_or(json!([])),
    })
}

fn board_fixture_value(document: &Puzzle5dDocument) -> Value {
    let nodes: Vec<Value> = document.parts.iter().map(board_node_value).collect();
    let edges: Vec<Value> = document
        .fasteners
        .iter()
        .map(|fastener| {
            json!({
                "id": fastener.id,
                "edgeKind": fastener.fastener_kind.clone().unwrap_or_else(|| "link".into()),
                "source": fastener.source,
                "target": fastener.target,
            })
        })
        .collect();
    json!({
        "schema": PUZZLE5D_BOARD_FIXTURE_SCHEMA,
        "camera": board_camera_value(&document.camera2d),
        "nodes": nodes,
        "edges": edges,
        "wires": [],
        "meta": {
            "kindCatalogs": board_kind_catalogs_value(document),
            "kindCompatibility": document.kind_compatibility.clone().unwrap_or(json!([])),
        },
    })
}

fn board_brush_kind_weights_json(runtime: &Puzzle5dRuntime) -> String {
    json!({ "nodeWeights": runtime.object_kind_weights, "handleWeights": runtime.vortex_kind_weights }).to_string()
}

fn puzzle5d_board_scene(envelope: &Puzzle5dEnvelope) -> Puzzle2dBoardScene {
    Puzzle2dBoardScene {
        fixture_json: board_fixture_value(&envelope.document).to_string(),
        camera_json: board_camera_value(&envelope.document.camera2d).to_string(),
        kind_catalogs_json: board_kind_catalogs_value(&envelope.document).to_string(),
        selection_json: serde_json::to_string(&selection_flat_ids(&envelope.runtime.selection)).unwrap_or_else(|_| "[]".into()),
        interactive: true,
        hovered_id: envelope.runtime.hovered_part_id.clone(),
        active_tool: Some(envelope.runtime.active_tool.clone()),
        selection_method: envelope.runtime.selection_method.clone(),
        grid_snap_enabled: envelope.runtime.grid_snap_enabled,
        grid_factor: envelope.runtime.grid_factor,
        suggestion_offset: envelope.runtime.suggestion_offset,
        brush_kind_weights_json: board_brush_kind_weights_json(&envelope.runtime),
        kind_compatibility_json: envelope.document.kind_compatibility.clone().unwrap_or(json!([])).to_string(),
        lod_mode: envelope.runtime.lod_mode.clone(),
    }
}

fn set_part_2d_position(document: &mut Puzzle5dDocument, part_id: &str, x: Option<f64>, y: Option<f64>) {
    if let Some(part) = document.parts.iter_mut().find(|part| part.id == part_id) {
        if let Some(x) = x {
            part.part_2d.x = x;
        }
        if let Some(y) = y {
            part.part_2d.y = y;
        }
    }
}

/// 🎨 Palette drop: creates a free paired part at the flat drop point, deriving the volume origin from the nearest peer part's offset.
fn add_palette_part(envelope: &mut Puzzle5dEnvelope, part_kind: &str, x: f64, y: f64) {
    let flat_to_world = 1.0 / 48.0;
    let origin = envelope
        .document
        .parts
        .first()
        .map(|peer| [peer.part_3d.origin[0] + (x - peer.part_2d.x) * flat_to_world, peer.part_3d.origin[1] - (y - peer.part_2d.y) * flat_to_world, peer.part_3d.origin[2]])
        .unwrap_or([x * flat_to_world, -y * flat_to_world, 0.0]);
    let id = next_part_id();
    let mesh_url = resolve_part_kind_mesh_url(part_kind, envelope.document.kind_catalogs.as_ref());
    let grips = grips_from_templates(&envelope.document, part_kind);
    envelope.document.parts.push(Puzzle5dPart {
        id: id.clone(),
        part_kind: part_kind.into(),
        part_2d: Puzzle5dPart2d { x, y, shape: "circle".into(), radius: PUZZLE5D_DEFAULT_PART_RADIUS, width: None, height: None, text: part_kind.into(), icon_kind: None, hidden: None, locked: None },
        part_3d: Puzzle5dPart3d { origin, mesh_url, orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None, label: None },
        grips,
    });
    envelope.runtime.selection = Puzzle5dSelection { part_ids: vec![id], grip_ids: Vec::new(), fastener_ids: Vec::new() };
}
//#endregion 🔖Board

//#region 🔖Engine
/// 🧠 Maps the unified 5d kind bundle to the puzzle 3d engine naming (`objects` with `vortices` templates, `vortices`, `cables`).
fn engine_kind_catalogs_value(document: &Puzzle5dDocument) -> Option<Value> {
    let catalogs = document.kind_catalogs.as_ref()?;
    let objects: Vec<Value> = catalogs
        .get("parts")
        .and_then(|parts| parts.as_array())
        .into_iter()
        .flatten()
        .map(|entry| {
            let mut object = entry.clone();
            let vortices: Vec<Value> = entry
                .get("grips")
                .and_then(|grips| grips.as_array())
                .into_iter()
                .flatten()
                .map(|template| {
                    let volume = template.get("3d").cloned().unwrap_or(json!({}));
                    json!({
                        "vortexKind": template.get("gripKind").cloned().unwrap_or(json!("grip")),
                        "position": volume.get("position").cloned().unwrap_or(json!([0.0, 0.0, 0.0])),
                        "direction": volume.get("direction").cloned().unwrap_or(json!([0.0, 0.0, -1.0])),
                        "radius": volume.get("radius").cloned().unwrap_or(json!(0.36)),
                    })
                })
                .collect();
            if let Some(object) = object.as_object_mut() {
                object.remove("grips");
                object.insert("vortices".into(), json!(vortices));
            }
            object
        })
        .collect();
    Some(json!({
        "objects": objects,
        "vortices": catalogs.get("grips").cloned().unwrap_or(json!([])),
        "cables": catalogs.get("ropes").cloned().unwrap_or(json!([])),
    }))
}

fn scene_config_json(envelope: &Puzzle5dEnvelope) -> String {
    let objects: Vec<Value> = envelope
        .document
        .parts
        .iter()
        .map(|part| {
            json!({
                "id": part.id,
                "objectKind": part.part_kind,
                "meshUrl": resolve_part_mesh_url(part, envelope.document.kind_catalogs.as_ref()),
                "origin": part.part_3d.origin,
                "orientation": part.part_3d.orientation,
                "scale": part.part_3d.scale,
                "vortices": part.grips.iter().map(|grip| json!({
                    "id": grip.id,
                    "vortexKind": if grip.grip_kind.is_empty() { grip.grip_2d.grip_kind.clone() } else { grip.grip_kind.clone() },
                    "position": grip.grip_3d.position,
                    "direction": grip.grip_3d.direction,
                })).collect::<Vec<_>>(),
            })
        })
        .collect();
    let attractions: Vec<Value> = envelope.document.fasteners.iter().map(|fastener| json!({ "id": fastener.id, "attracting": fastener.source, "attracted": fastener.target })).collect();
    json!({
        "fixture": {
            "objects": objects,
            "attractions": attractions,
            "targetVolumes": [],
        },
        "kindCatalogs": engine_kind_catalogs_value(&envelope.document),
        "kindCompatibility": envelope.document.kind_compatibility.clone().unwrap_or(json!([])),
        "overlapBudget": envelope.runtime.overlap_budget,
        "seed": 1,
        "hostRules": {},
        "weights": {
            "objectWeights": envelope.runtime.object_kind_weights,
            "vortexWeights": envelope.runtime.vortex_kind_weights,
        },
    })
    .to_string()
}

/// 🔄 Adopts an engine fixture while preserving flat aspects: existing parts keep `2d`, new parts get a synthesized flat aspect.
fn merge_engine_fixture(envelope: &Puzzle5dEnvelope, fixture_json: &str) -> Option<Puzzle5dEnvelope> {
    let parsed: Value = serde_json::from_str(fixture_json).ok()?;
    let objects = parsed.get("objects")?.as_array()?;
    let mut next = envelope.clone();
    let existing: HashMap<String, Puzzle5dPart> = envelope.document.parts.iter().map(|part| (part.id.clone(), part.clone())).collect();
    let mut new_ids: Vec<String> = Vec::new();
    next.document.parts = objects
        .iter()
        .filter_map(|object| {
            let id = object.get("id")?.as_str()?.to_string();
            let part_kind = object.get("objectKind").and_then(|value| value.as_str()).unwrap_or("Part").to_string();
            let origin: [f64; 3] = object.get("origin").and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or([0.0, 0.0, 0.0]);
            let orientation: Option<[f64; 4]> = object.get("orientation").and_then(|value| serde_json::from_value(value.clone()).ok());
            let mesh_url = object.get("meshUrl").and_then(|value| value.as_str()).map(str::to_string);
            let scale = object.get("scale").cloned().filter(|value| !value.is_null());
            if let Some(previous) = existing.get(&id) {
                let mut part = previous.clone();
                part.part_kind = part_kind;
                part.part_3d.origin = origin;
                part.part_3d.orientation = orientation.or(part.part_3d.orientation);
                part.part_3d.mesh_url = mesh_url.or(part.part_3d.mesh_url.clone());
                if scale.is_some() {
                    part.part_3d.scale = scale;
                }
                return Some(part);
            }
            let templates = grips_from_templates(&envelope.document, &part_kind);
            let grips: Vec<Puzzle5dGrip> = object
                .get("vortices")
                .and_then(|value| value.as_array())
                .into_iter()
                .flatten()
                .enumerate()
                .map(|(index, vortex)| {
                    let template = templates.get(index);
                    Puzzle5dGrip {
                        id: vortex.get("id").and_then(|value| value.as_str()).map(str::to_string).unwrap_or_else(|| format!("v{index}")),
                        grip_kind: vortex.get("vortexKind").and_then(|value| value.as_str()).map(str::to_string).or_else(|| template.map(|t| t.grip_kind.clone())).unwrap_or_else(|| "grip".into()),
                        grip_2d: template.map(|t| t.grip_2d.clone()).unwrap_or_default(),
                        grip_3d: Puzzle5dGrip3d {
                            position: vortex.get("position").and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or([0.0, 0.0, 0.0]),
                            direction: vortex.get("direction").and_then(|value| serde_json::from_value(value.clone()).ok()),
                            radius: vortex.get("radius").and_then(|value| value.as_f64()).unwrap_or(0.36),
                            label: vortex.get("label").and_then(|value| value.as_str()).map(str::to_string),
                        },
                    }
                })
                .collect();
            let grips = if grips.is_empty() { templates } else { grips };
            new_ids.push(id.clone());
            Some(Puzzle5dPart {
                id,
                part_kind: part_kind.clone(),
                part_2d: Puzzle5dPart2d { x: 0.0, y: 0.0, shape: "circle".into(), radius: PUZZLE5D_DEFAULT_PART_RADIUS, width: None, height: None, text: part_kind, icon_kind: None, hidden: None, locked: None },
                part_3d: Puzzle5dPart3d { origin, mesh_url, orientation: orientation.or(Some([0.0, 0.0, 0.0, 1.0])), scale, label: None },
                grips,
            })
        })
        .collect();
    let existing_kinds: HashMap<String, Option<String>> = envelope.document.fasteners.iter().map(|fastener| (fastener.id.clone(), fastener.fastener_kind.clone())).collect();
    next.document.fasteners = parsed
        .get("attractions")
        .and_then(|value| value.as_array())
        .into_iter()
        .flatten()
        .filter_map(|attraction| {
            let id = attraction.get("id").and_then(|value| value.as_str()).unwrap_or("fastener").to_string();
            Some(Puzzle5dFastener {
                fastener_kind: existing_kinds.get(&id).cloned().flatten().or_else(|| attraction.get("attractionKind").and_then(|value| value.as_str()).map(str::to_string)),
                source: attraction.get("attracting")?.as_str()?.to_string(),
                target: attraction.get("attracted")?.as_str()?.to_string(),
                id,
            })
        })
        .collect();
    synthesize_flat_for_new_parts(&mut next.document, &new_ids);
    Some(next)
}

/// 🌤️ Places flat centers for freshly-adopted parts next to their fastened neighbor, walking chains until every new part is placed.
fn synthesize_flat_for_new_parts(document: &mut Puzzle5dDocument, new_ids: &[String]) {
    let mut pending: HashSet<String> = new_ids.iter().cloned().collect();
    for _ in 0..=new_ids.len() {
        if pending.is_empty() {
            break;
        }
        let mut placed: Vec<(String, f64, f64)> = Vec::new();
        for fastener in &document.fasteners {
            for (own, other) in [(&fastener.source, &fastener.target), (&fastener.target, &fastener.source)] {
                let Some((own_part, _)) = find_part_by_grip_full_id(document, own) else {
                    continue;
                };
                if !pending.contains(&own_part.id) {
                    continue;
                }
                let Some((other_part, other_grip)) = find_part_by_grip_full_id(document, other) else {
                    continue;
                };
                if pending.contains(&other_part.id) {
                    continue;
                }
                let angle = other_grip.grip_2d.angle;
                let own_radius = if own_part.part_2d.radius > 0.0 { own_part.part_2d.radius } else { PUZZLE5D_DEFAULT_PART_RADIUS };
                let other_radius = if other_part.part_2d.radius > 0.0 { other_part.part_2d.radius } else { PUZZLE5D_DEFAULT_PART_RADIUS };
                let distance = own_radius + other_radius + PUZZLE5D_BOARD_PLACEMENT_GAP;
                placed.push((own_part.id.clone(), other_part.part_2d.x + angle.cos() * distance, other_part.part_2d.y + angle.sin() * distance));
            }
        }
        if placed.is_empty() {
            break;
        }
        for (id, x, y) in placed {
            set_part_2d_position(document, &id, Some(x), Some(y));
            pending.remove(&id);
        }
    }
    let mut column = 0usize;
    for id in pending {
        set_part_2d_position(document, &id, Some(120.0 + column as f64 * 56.0), Some(120.0));
        column += 1;
    }
}
//#endregion 🔖Engine

//#region 🔖World
fn world_instances_json(document: &Puzzle5dDocument, runtime: &Puzzle5dRuntime) -> String {
    let instances: Vec<Value> = document
        .parts
        .iter()
        .map(|part| {
            let selected = runtime.selection.part_ids.contains(&part.id);
            let hovered = runtime.hovered_part_id.as_deref() == Some(part.id.as_str());
            let mesh_id = resolve_part_mesh_url(part, document.kind_catalogs.as_ref()).map(|url| world3d_mesh_id_from_url(&url)).unwrap_or_else(|| PUZZLE5D_FALLBACK_MESH_KIND.into());
            json!({
                "id": part.id,
                "meshId": mesh_id,
                "position": part.part_3d.origin,
                "rotation": part.part_3d.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]),
                "scale": part_scale_json(part),
                "label": part.part_3d.label.clone().unwrap_or_else(|| part.part_kind.clone()),
                "color": if selected { "#f59e0b" } else if hovered { "#fbbf24" } else { "#94a3b8" },
                "selected": selected,
                "hovered": hovered,
            })
        })
        .collect();
    serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into())
}

fn part_scale_json(part: &Puzzle5dPart) -> [f64; 3] {
    match &part.part_3d.scale {
        Some(Value::Array(values)) if values.len() >= 3 => [values[0].as_f64().unwrap_or(1.0), values[1].as_f64().unwrap_or(1.0), values[2].as_f64().unwrap_or(1.0)],
        Some(Value::Number(value)) => {
            let factor = value.as_f64().unwrap_or(1.0);
            [factor, factor, factor]
        }
        _ => [1.0, 1.0, 1.0],
    }
}

fn world_meshes_json(document: &Puzzle5dDocument) -> String {
    world3d_meshes_json_from_urls(&collect_mesh_urls(document))
}

fn grip_color(kind_catalogs: Option<&Value>, grip_kind: &str) -> String {
    kind_catalogs
        .and_then(|catalogs| catalogs.get("grips"))
        .and_then(|value| value.as_array())
        .and_then(|entries| entries.iter().find(|entry| entry.get("id").and_then(|value| value.as_str()) == Some(grip_kind)))
        .and_then(|entry| entry.get("color").and_then(|value| value.as_str()).map(str::to_string))
        .unwrap_or_else(|| "#38bdf8".into())
}

fn world_grips_json(document: &Puzzle5dDocument) -> String {
    let mut records = Vec::new();
    for part in &document.parts {
        for grip in &part.grips {
            records.push(json!({
                "fullId": puzzle5d_grip_full_id(&part.id, &grip.id),
                "objectId": part.id,
                "vortexKind": grip.grip_kind,
                "position": world_grip_position(part, grip),
                "direction": world_grip_direction(part, grip),
                "radius": grip.grip_3d.radius.max(0.36),
                "color": grip_color(document.kind_catalogs.as_ref(), &grip.grip_kind),
            }));
        }
    }
    serde_json::to_string(&records).unwrap_or_else(|_| "[]".into())
}

fn world_fasteners_json(document: &Puzzle5dDocument) -> String {
    let records: Vec<Value> = document
        .fasteners
        .iter()
        .filter_map(|fastener| {
            let from = resolve_grip_world_position(document, &fastener.source)?;
            let to = resolve_grip_world_position(document, &fastener.target)?;
            Some(json!({ "id": fastener.id, "from": from, "to": to, "color": "#60a5fa" }))
        })
        .collect();
    serde_json::to_string(&records).unwrap_or_else(|_| "[]".into())
}

fn gumball_target_world(envelope: &Puzzle5dEnvelope) -> Option<[f64; 3]> {
    let selected: Vec<&Puzzle5dPart> = envelope.document.parts.iter().filter(|part| envelope.runtime.selection.part_ids.contains(&part.id)).collect();
    if selected.is_empty() {
        return None;
    }
    let mut sum = [0.0, 0.0, 0.0];
    for part in &selected {
        sum[0] += part.part_3d.origin[0];
        sum[1] += part.part_3d.origin[1];
        sum[2] += part.part_3d.origin[2];
    }
    let count = selected.len() as f64;
    Some([sum[0] / count, sum[1] / count, sum[2] / count])
}

/// 🎯 Base selection JSON augmented with the mesh granularity, transform tool, and gumball fields the world-3d host reads.
fn world_selection_json_ex(envelope: &Puzzle5dEnvelope) -> String {
    let runtime = &envelope.runtime;
    let mut value: Value = serde_json::from_str(&world3d_selection_json(&runtime.selection_method, &runtime.selection.part_ids, runtime.hovered_part_id.as_deref())).unwrap_or_else(|_| json!({}));
    if let Some(object) = value.as_object_mut() {
        object.insert("granularity".into(), json!("mesh"));
        object.insert("selectionMode".into(), json!("mesh"));
        object.insert("targets".into(), json!({ "mesh": true, "vertex": false, "edge": false, "face": false }));
        object.insert("transformTool".into(), json!(runtime.transform_tool));
        if let Some(active_id) = runtime.selection.part_ids.first() {
            object.insert("activeObjectId".into(), json!(active_id));
        }
        let gumball_active = !runtime.selection.part_ids.is_empty();
        object.insert("gumballActive".into(), json!(gumball_active));
        if gumball_active {
            if let Some(target) = gumball_target_world(envelope) {
                object.insert("gumballTarget".into(), json!(target));
            }
        }
    }
    value.to_string()
}

fn world_interaction_json(runtime: &Puzzle5dRuntime) -> String {
    json!({
        "activeTool": runtime.active_tool,
        "brushCandidateIndex": runtime.brush_candidate_index,
        "fillCount": runtime.fill_count,
        "hoveredVortexFullId": runtime.selection.grip_ids.first().cloned(),
    })
    .to_string()
}

fn puzzle5d_context_menu_json(envelope: &Puzzle5dEnvelope, labels: &Puzzle5dLabels) -> Option<String> {
    if envelope.runtime.selection.part_ids.is_empty() {
        return None;
    }
    let items = vec![
        json!({ "id": "duplicate", "label": labels.duplicate, "action": "duplicateSelection" }),
        json!({ "id": "select-same-kind", "label": labels.select_same_kind, "action": "selectSameKindSelection" }),
        json!({ "id": "zoom", "label": labels.zoom_to_selection, "action": "zoomToSelection" }),
        json!({ "id": "delete", "label": labels.delete, "action": "deleteSelection" }),
    ];
    serde_json::to_string(&items).ok()
}

fn camera3d_json(camera: &Puzzle5dCamera3d) -> String {
    json!({ "position": camera.position, "target": camera.target, "zoom": camera.zoom, "fov": 45.0 }).to_string()
}
//#endregion 🔖World

//#region 🔖Brush
fn puzzle5d_brush_target_grip(envelope: &Puzzle5dEnvelope) -> Option<String> {
    envelope.runtime.selection.grip_ids.first().cloned().or_else(|| {
        let part_id = envelope.runtime.hovered_part_id.as_deref().or_else(|| envelope.runtime.selection.part_ids.first().map(String::as_str))?;
        let part = envelope.document.parts.iter().find(|part| part.id == part_id)?;
        let grip = part.grips.first()?;
        Some(puzzle5d_grip_full_id(&part.id, &grip.id))
    })
}

fn parse_brush_candidates_free(raw: &str) -> Vec<Value> {
    let parsed: Value = serde_json::from_str(raw).unwrap_or(Value::Null);
    parsed.get("free").and_then(|value| value.as_array()).cloned().unwrap_or_default()
}

fn world_brush_preview_json(session: &Puzzle5dPrecomputeSession, envelope: &Puzzle5dEnvelope) -> Option<String> {
    if envelope.runtime.active_tool != "brush" {
        return None;
    }
    let full_id = puzzle5d_brush_target_grip(envelope)?;
    session.brush_preview_json(&full_id, envelope.runtime.brush_candidate_index)
}
//#endregion 🔖Brush

//#region 🔖Engagement
fn puzzle5d_brush_placement_control(envelope: &Puzzle5dEnvelope, precompute: &Puzzle5dPrecomputeSession, labels: &Puzzle5dLabels) -> Option<WindowEngagementControl> {
    let target = puzzle5d_brush_target_grip(envelope)?;
    let candidates = parse_brush_candidates_free(&precompute.brush_candidates(&target));
    if candidates.is_empty() {
        return None;
    }
    let options: Vec<WindowEngagementToggleGroupOption> = candidates
        .iter()
        .enumerate()
        .map(|(index, candidate)| {
            let label = candidate.get("objectKind").and_then(|value| value.as_str()).or_else(|| candidate.get("objectKindId").and_then(|value| value.as_str())).unwrap_or("kind");
            WindowEngagementToggleGroupOption { id: format!("puzzle5d.brush.candidate.{index}"), label: label.into(), disabled: None }
        })
        .collect();
    let selected_index = envelope.runtime.brush_candidate_index.min(options.len().saturating_sub(1));
    Some(WindowEngagementControl::ToggleGroup {
        id: Some("puzzle5d-brush-placement".into()),
        label: Some(labels.placement.into()),
        value: Some(format!("puzzle5d.brush.candidate.{selected_index}")),
        options,
        disabled: None,
        on_select: Some(puzzle5d_action("engagementControlSelect", None)),
    })
}

fn puzzle5d_fill_count_control(envelope: &Puzzle5dEnvelope, labels: &Puzzle5dLabels) -> WindowEngagementControl {
    WindowEngagementControl::Slider {
        id: Some("puzzle5d-fill-count".into()),
        label: Some(format!("{} {}", labels.fill, envelope.runtime.fill_count)),
        value: envelope.runtime.fill_count as f64,
        min: 0.0,
        max: PUZZLE5D_FILL_COUNT_MAX as f64,
        step: Some(1.0),
        unit: None,
        disabled: None,
        on_change: Some(puzzle5d_action("setFillCount", None)),
        on_commit: None,
    }
}

fn puzzle5d_engagement(envelope: &Puzzle5dEnvelope, precompute: &Puzzle5dPrecomputeSession, window: &str, labels: &Puzzle5dLabels) -> WindowEngagement {
    let part_count = envelope.document.parts.len();
    let fastener_count = envelope.document.fasteners.len();
    let control = match envelope.runtime.active_tool.as_str() {
        "fill" => Some(puzzle5d_fill_count_control(envelope, labels)),
        "brush" => puzzle5d_brush_placement_control(envelope, precompute, labels),
        _ => None,
    };
    let input_value = envelope.runtime.engagement_input_by_window.get(window).cloned().unwrap_or_default();
    let placeholder = match envelope.runtime.active_tool.as_str() {
        "fill" => "Fill",
        "brush" => "Brush",
        _ => "select, brush, fill, clear",
    };
    WindowEngagement {
        session_active: Some(envelope.runtime.active_tool != "select"),
        input: Some(WindowEngagementInput {
            id: Some(format!("puzzle5d-engagement-{window}")),
            value: Some(input_value),
            placeholder: Some(placeholder.into()),
            disabled: None,
            on_change: Some(puzzle5d_action("engagementInput", Some(json!({ "window": window })))),
            on_submit: Some(puzzle5d_action("engagementSubmit", Some(json!({ "window": window })))),
            on_repeat_last: None,
            on_abort: Some(puzzle5d_action("engagementAbort", Some(json!({ "window": window })))),
        }),
        control,
        controls: None,
        status: Some(vec![WindowEngagementStatus { id: format!("puzzle5d-status-{window}"), text: format!("{part_count} parts · {fastener_count} fasteners · tool {}", envelope.runtime.active_tool) }]),
        options: Some(vec![
            WindowEngagementOption {
                id: PUZZLE5D_ENGAGEMENT_TOOL_SELECT.into(),
                label: Some(labels.select.into()),
                icon_id: Some("cursor".into()),
                pressed: Some(envelope.runtime.active_tool == "select"),
                disabled: None,
                action: Some(puzzle5d_action("engagementPossibleSelect", Some(json!({ "window": window, "possibleId": PUZZLE5D_ENGAGEMENT_TOOL_SELECT })))),
            },
            WindowEngagementOption {
                id: PUZZLE5D_ENGAGEMENT_TOOL_BRUSH.into(),
                label: Some(labels.brush.into()),
                icon_id: Some("brush".into()),
                pressed: Some(envelope.runtime.active_tool == "brush"),
                disabled: None,
                action: Some(puzzle5d_action("engagementPossibleSelect", Some(json!({ "window": window, "possibleId": PUZZLE5D_ENGAGEMENT_TOOL_BRUSH })))),
            },
            WindowEngagementOption {
                id: PUZZLE5D_ENGAGEMENT_TOOL_FILL.into(),
                label: Some(labels.fill.into()),
                icon_id: Some("fill".into()),
                pressed: Some(envelope.runtime.fill_count > 0 || envelope.runtime.active_tool == "fill"),
                disabled: None,
                action: Some(puzzle5d_action("engagementPossibleSelect", Some(json!({ "window": window, "possibleId": PUZZLE5D_ENGAGEMENT_TOOL_FILL })))),
            },
        ]),
        possible_engagements: None,
    }
}
//#endregion 🔖Engagement

//#region 🔖Measures
fn puzzle5d_lod_tier_ids() -> Vec<String> {
    serde_json::from_str::<Vec<Value>>(&puzzle_2d::puzzle_2d_lod_scale_json()).unwrap_or_default().into_iter().filter_map(|row| row.get("id").and_then(|value| value.as_str()).map(str::to_string)).collect()
}

fn puzzle5d_kind_ids(document: &Puzzle5dDocument, slice: &str) -> Vec<String> {
    let mut ids: Vec<String> = document
        .kind_catalogs
        .as_ref()
        .and_then(|catalogs| catalogs.get(slice))
        .and_then(|value| value.as_array())
        .into_iter()
        .flatten()
        .filter_map(|entry| entry.get("id").and_then(|value| value.as_str()).map(str::to_string))
        .collect();
    if ids.is_empty() {
        let mut inferred: Vec<String> = match slice {
            "parts" => document.parts.iter().map(|part| part.part_kind.clone()).collect(),
            "grips" => document.parts.iter().flat_map(|part| part.grips.iter().map(|grip| grip.grip_kind.clone())).collect(),
            _ => Vec::new(),
        };
        inferred.sort();
        inferred.dedup();
        ids = inferred;
    }
    ids
}

fn puzzle5d_lod_measure(runtime: &Puzzle5dRuntime, labels: &Puzzle5dLabels) -> WindowMeasure {
    let mut items = vec![MeasureSelectItem { id: PUZZLE5D_LOD_MODE_AUTOMATIC.into(), value: PUZZLE5D_LOD_MODE_AUTOMATIC.into(), label: labels.automatic.into() }];
    items.extend(puzzle5d_lod_tier_ids().into_iter().map(|tier| MeasureSelectItem { id: tier.clone(), value: tier.clone(), label: tier }));
    WindowMeasure::Select { id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-lod"), label: Some(labels.lod.into()), value: runtime.lod_mode.clone(), items, on_change: puzzle5d_action("setLodMode", None) }
}

fn puzzle5d_kind_weight_measures(prefix: &str, action: &str, ids: &[String], weights: &HashMap<String, f64>) -> Vec<WindowMeasure> {
    ids.iter()
        .map(|kind_id| {
            let weight = weights.get(kind_id).copied().unwrap_or(0.0);
            WindowMeasure::Slider {
                id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-{prefix}-{kind_id}"),
                label: Some(format!("{kind_id} {:.0}%", weight * 100.0)),
                value: weight,
                min: 0.0,
                max: 1.0,
                step: Some(0.01),
                on_change: puzzle5d_action(action, Some(json!({ "kindId": kind_id }))),
            }
        })
        .collect()
}

fn puzzle5d_suggestion_measures_group(envelope: &Puzzle5dEnvelope, labels: &Puzzle5dLabels) -> WindowMeasure {
    let part_ids = puzzle5d_kind_ids(&envelope.document, "parts");
    let grip_ids = puzzle5d_kind_ids(&envelope.document, "grips");
    WindowMeasure::Group {
        id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-suggestion"),
        label: labels.suggestion.into(),
        default_open: Some(false),
        children: vec![
            WindowMeasure::Slider {
                id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-suggestion-offset"),
                label: Some(labels.offset.into()),
                value: envelope.runtime.suggestion_offset,
                min: PUZZLE5D_SUGGESTION_OFFSET_MIN,
                max: PUZZLE5D_SUGGESTION_OFFSET_MAX,
                step: Some(PUZZLE5D_SUGGESTION_OFFSET_STEP),
                on_change: puzzle5d_action("setSuggestionOffset", None),
            },
            WindowMeasure::Group {
                id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-suggestion-parts"),
                label: labels.part_weights.into(),
                default_open: Some(false),
                children: puzzle5d_kind_weight_measures("part-kind", "setObjectKindWeight", &part_ids, &envelope.runtime.object_kind_weights),
            },
            WindowMeasure::Group {
                id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-suggestion-grips"),
                label: labels.grip_weights.into(),
                default_open: Some(false),
                children: puzzle5d_kind_weight_measures("grip-kind", "setVortexKindWeight", &grip_ids, &envelope.runtime.vortex_kind_weights),
            },
        ],
    }
}

fn puzzle5d_brush_measures_group(envelope: &Puzzle5dEnvelope, labels: &Puzzle5dLabels) -> WindowMeasure {
    WindowMeasure::Group {
        id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-brush"),
        label: labels.brush.into(),
        default_open: Some(false),
        children: vec![WindowMeasure::Slider {
            id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-brush-overlap"),
            label: Some(labels.overlap.into()),
            value: envelope.runtime.overlap_budget,
            min: 0.0,
            max: 0.2,
            step: Some(0.005),
            on_change: puzzle5d_action("setBrushPlacementOverlapBudget", None),
        }],
    }
}

fn puzzle5d_window_measures(window: &str, envelope: &Puzzle5dEnvelope, labels: &Puzzle5dLabels) -> Vec<WindowMeasure> {
    if window == PUZZLE5D_PLAY_WINDOW_2D {
        vec![puzzle5d_lod_measure(&envelope.runtime, labels), puzzle5d_suggestion_measures_group(envelope, labels), puzzle5d_brush_measures_group(envelope, labels)]
    } else {
        vec![puzzle5d_suggestion_measures_group(envelope, labels), puzzle5d_brush_measures_group(envelope, labels)]
    }
}
//#endregion 🔖Measures

//#region 🔖Panels
fn tree_item_with_action(id: impl Into<String>, label: impl Into<String>, icon_id: Option<&str>, action: ActionDescriptor) -> UiTreeItemNode {
    let mut item = UiTreeItemNode::base(id, label);
    item.icon_id = icon_id.map(str::to_string);
    item.action = Some(action);
    item
}

fn tree_info_item(id: impl Into<String>, label: impl Into<String>, description: Option<String>) -> UiTreeItemNode {
    let mut item = UiTreeItemNode::base(id, label);
    item.description = description;
    item
}

fn part_label(part: &Puzzle5dPart) -> String {
    if !part.part_2d.text.is_empty() {
        return part.part_2d.text.clone();
    }
    part.part_3d.label.clone().unwrap_or_else(|| part.part_kind.clone())
}

fn fastener_label(document: &Puzzle5dDocument, fastener: &Puzzle5dFastener) -> String {
    let side = |full_id: &str| find_part_by_grip_full_id(document, full_id).map(|(part, _)| part_label(part)).unwrap_or_else(|| full_id.to_string());
    format!("{} → {}", side(&fastener.source), side(&fastener.target))
}

fn document_tree_selected_ids(envelope: &Puzzle5dEnvelope) -> Vec<String> {
    let selection = &envelope.runtime.selection;
    selection
        .part_ids
        .iter()
        .map(|id| format!("puzzle5d-play-document.part.{id}"))
        .chain(selection.grip_ids.iter().map(|id| format!("puzzle5d-play-document.grip.{id}")))
        .chain(selection.fastener_ids.iter().map(|id| format!("puzzle5d-play-document.fastener.{id}")))
        .collect()
}

fn build_document_tree(envelope: &Puzzle5dEnvelope, labels: &Puzzle5dLabels) -> UiNode {
    let part_items: Vec<UiTreeItemNode> = envelope
        .document
        .parts
        .iter()
        .map(|part| {
            let grip_items: Vec<UiTreeItemNode> = part
                .grips
                .iter()
                .map(|grip| {
                    let full_id = puzzle5d_grip_full_id(&part.id, &grip.id);
                    tree_item_with_action(format!("puzzle5d-play-document.grip.{full_id}"), format!("{} ({})", grip.id, grip.grip_kind), Some("circle-dot"), puzzle5d_action("setSelection", Some(json!({ "gripIds": [full_id] }))))
                })
                .collect();
            let mut item = tree_item_with_action(format!("puzzle5d-play-document.part.{}", part.id), part_label(part), Some("box"), puzzle5d_action("setSelection", Some(json!({ "partIds": [part.id] }))));
            item.description = Some(part.part_kind.clone());
            if !grip_items.is_empty() {
                item.items = Some(grip_items);
            }
            item
        })
        .collect();
    let fastener_items: Vec<UiTreeItemNode> = envelope
        .document
        .fasteners
        .iter()
        .map(|fastener| tree_item_with_action(format!("puzzle5d-play-document.fastener.{}", fastener.id), fastener_label(&envelope.document, fastener), Some("link"), puzzle5d_action("setSelection", Some(json!({ "fastenerIds": [fastener.id] })))))
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![
            UiTreeSectionNode {
                id: "puzzle5d-play-document.parts".into(),
                label: Some(labels.parts.into()),
                default_open: Some(true),
                items: if part_items.is_empty() { vec![tree_info_item("puzzle5d-play-document.parts.empty", "(none)", None)] } else { part_items },
            },
            UiTreeSectionNode {
                id: "puzzle5d-play-document.fasteners".into(),
                label: Some(labels.fasteners.into()),
                default_open: Some(false),
                items: if fastener_items.is_empty() { vec![tree_info_item("puzzle5d-play-document.fasteners.empty", "(none)", None)] } else { fastener_items },
            },
        ],
        selected_ids: Some(document_tree_selected_ids(envelope)),
        highlighted_ids: None,
        selection_change: Some(puzzle5d_action("setSelection", None)),
        drop_action: None,
    })
}

fn catalog_kind_label(entry: &Value) -> String {
    entry
        .get("label")
        .and_then(|value| value.as_str())
        .filter(|value| !value.is_empty())
        .or_else(|| entry.get("name").and_then(|value| value.as_str()).filter(|value| !value.is_empty()))
        .or_else(|| entry.get("id").and_then(|value| value.as_str()))
        .unwrap_or("kind")
        .into()
}

/// 🖱️ MIME key `DeclarativeTreePanel` (framework/renderer/react/ui-interpreter.tsx) reads to auto-wire catalogue drag sources.
const PUZZLE5D_CATALOGUE_DRAG_MIME: &str = "application/x-semio-catalogue-item";

fn puzzle5d_catalog_item_drag_data(kind_id: &str, entry: &Value) -> HashMap<String, String> {
    let mut payload = json!({ "kindId": kind_id, "catalogSlice": "nodes" });
    if let Some(object) = payload.as_object_mut() {
        for key in ["shape", "radius", "width", "height", "iconKind"] {
            if let Some(value) = entry.get(key) {
                object.insert(key.into(), value.clone());
            }
        }
    }
    HashMap::from([(PUZZLE5D_CATALOGUE_DRAG_MIME.to_string(), payload.to_string())])
}

fn kind_catalog_section(section_id: &str, label: &str, entries: &[Value], add_action: Option<&str>) -> UiTreeSectionNode {
    let items: Vec<UiTreeItemNode> = entries
        .iter()
        .enumerate()
        .map(|(index, entry)| {
            let kind_id = entry.get("id").and_then(|value| value.as_str()).unwrap_or("kind");
            match add_action {
                Some(action) => {
                    let mut item = tree_item_with_action(format!("{section_id}.{index}.{kind_id}"), catalog_kind_label(entry), Some("box"), puzzle5d_action(action, Some(json!({ "partKind": kind_id }))));
                    item.description = Some(kind_id.into());
                    item.draggable = Some(true);
                    item.drag_data = Some(puzzle5d_catalog_item_drag_data(kind_id, entry));
                    item
                }
                None => tree_info_item(format!("{section_id}.{index}.{kind_id}"), catalog_kind_label(entry), Some(kind_id.into())),
            }
        })
        .collect();
    UiTreeSectionNode {
        id: section_id.into(),
        label: Some(label.into()),
        default_open: Some(!items.is_empty()),
        items: if items.is_empty() { vec![tree_info_item(format!("{section_id}.empty"), "(none)", None)] } else { items },
    }
}

fn build_kinds_tree(envelope: &Puzzle5dEnvelope, labels: &Puzzle5dLabels) -> UiNode {
    let catalogs = envelope.document.kind_catalogs.clone().unwrap_or(json!({}));
    let slice = |key: &str| catalogs.get(key).and_then(|value| value.as_array()).cloned().unwrap_or_default();
    let mut part_entries = slice("parts");
    if part_entries.is_empty() {
        let mut ids: Vec<String> = envelope.document.parts.iter().map(|part| part.part_kind.clone()).collect();
        ids.sort();
        ids.dedup();
        part_entries = ids.into_iter().map(|id| json!({ "id": id, "name": id })).collect();
    }
    UiNode::Tree(UiTreeNode {
        sections: vec![
            kind_catalog_section("puzzle5d-play-kinds.parts", labels.parts, &part_entries, Some("addPartKind")),
            kind_catalog_section("puzzle5d-play-kinds.grips", labels.grips, &slice("grips"), None),
            kind_catalog_section("puzzle5d-play-kinds.fasteners", labels.fasteners, &slice("fasteners"), None),
            kind_catalog_section("puzzle5d-play-kinds.ropes", labels.ropes, &slice("ropes"), None),
        ],
        selected_ids: None,
        highlighted_ids: None,
        selection_change: None,
        drop_action: None,
    })
}

fn inspector_text_field(id: &str, label: &str, value: String, action: ActionDescriptor) -> UiNode {
    UiNode::Field(UiFieldNode {
        id: id.into(),
        label: label.into(),
        description: None,
        required: None,
        error: None,
        child: Box::new(UiNode::Input(semio_framework_plugin::UiInputNode {
            id: format!("{id}.input"),
            input_kind: "text".into(),
            value,
            placeholder: None,
            commit: None,
            min: None,
            max: None,
            step: None,
            accept: None,
            on_change: action,
        })),
    })
}

fn build_part_inspector(part: &Puzzle5dPart, labels: &Puzzle5dLabels) -> UiNode {
    let origin = part.part_3d.origin;
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "puzzle5d-play-inspector.part".into(),
        label: labels.part.into(),
        default_open: None,
        fields: vec![
            ui_inspector_readonly_field("puzzle5d-play-inspector.part.id", "Id", &part.id),
            inspector_text_field("puzzle5d-play-inspector.part.kind", "Kind", part.part_kind.clone(), puzzle5d_action("patchPart", Some(json!({ "partId": part.id, "field": "partKind" })))),
            inspector_text_field("puzzle5d-play-inspector.part.label", "Label", part.part_3d.label.clone().unwrap_or_default(), puzzle5d_action("patchPart", Some(json!({ "partId": part.id, "field": "label" })))),
            inspector_text_field("puzzle5d-play-inspector.part.text", "Flat text", part.part_2d.text.clone(), puzzle5d_action("patchPart", Some(json!({ "partId": part.id, "field": "text" })))),
            inspector_text_field("puzzle5d-play-inspector.part.x", "Flat x", format!("{}", part.part_2d.x), puzzle5d_action("patchPart", Some(json!({ "partId": part.id, "field": "x" })))),
            inspector_text_field("puzzle5d-play-inspector.part.y", "Flat y", format!("{}", part.part_2d.y), puzzle5d_action("patchPart", Some(json!({ "partId": part.id, "field": "y" })))),
            inspector_text_field("puzzle5d-play-inspector.part.origin", "Volume origin", format!("{:.3}, {:.3}, {:.3}", origin[0], origin[1], origin[2]), puzzle5d_action("patchPart", Some(json!({ "partId": part.id, "field": "origin" })))),
        ],
    }])
}

fn build_grip_inspector(part: &Puzzle5dPart, grip: &Puzzle5dGrip, labels: &Puzzle5dLabels) -> UiNode {
    let full_id = puzzle5d_grip_full_id(&part.id, &grip.id);
    let position = grip.grip_3d.position;
    let direction = grip.grip_3d.direction.unwrap_or([0.0, 0.0, -1.0]);
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "puzzle5d-play-inspector.grip".into(),
        label: labels.grip.into(),
        default_open: None,
        fields: vec![
            ui_inspector_readonly_field("puzzle5d-play-inspector.grip.id", "Id", &full_id),
            inspector_text_field("puzzle5d-play-inspector.grip.kind", "Kind", grip.grip_kind.clone(), puzzle5d_action("patchGrip", Some(json!({ "gripFullId": full_id, "field": "gripKind" })))),
            inspector_text_field("puzzle5d-play-inspector.grip.angle", "Flat angle", format!("{}", grip.grip_2d.angle), puzzle5d_action("patchGrip", Some(json!({ "gripFullId": full_id, "field": "angle" })))),
            inspector_text_field("puzzle5d-play-inspector.grip.radius", "Radius", format!("{}", grip.grip_3d.radius), puzzle5d_action("patchGrip", Some(json!({ "gripFullId": full_id, "field": "radius" })))),
            inspector_text_field("puzzle5d-play-inspector.grip.position", "Position", format!("{:.3}, {:.3}, {:.3}", position[0], position[1], position[2]), puzzle5d_action("patchGrip", Some(json!({ "gripFullId": full_id, "field": "position" })))),
            inspector_text_field("puzzle5d-play-inspector.grip.direction", "Direction", format!("{:.3}, {:.3}, {:.3}", direction[0], direction[1], direction[2]), puzzle5d_action("patchGrip", Some(json!({ "gripFullId": full_id, "field": "direction" })))),
        ],
    }])
}

fn build_inspector_tree(envelope: &Puzzle5dEnvelope, labels: &Puzzle5dLabels) -> UiNode {
    if let Some(grip_full_id) = envelope.runtime.selection.grip_ids.first() {
        if let Some((part, grip)) = find_part_by_grip_full_id(&envelope.document, grip_full_id) {
            return build_grip_inspector(part, grip, labels);
        }
    }
    if let Some(part_id) = envelope.runtime.selection.part_ids.first() {
        if let Some(part) = envelope.document.parts.iter().find(|entry| &entry.id == part_id) {
            return build_part_inspector(part, labels);
        }
    }
    if let Some(fastener_id) = envelope.runtime.selection.fastener_ids.first() {
        if let Some(fastener) = envelope.document.fasteners.iter().find(|entry| &entry.id == fastener_id) {
            return ui_stack_vertical(vec![
                ui_inspector_readonly_field("puzzle5d-play-inspector.fastener.id", "Id", &fastener.id),
                ui_inspector_readonly_field("puzzle5d-play-inspector.fastener.source", "Source", &fastener.source),
                ui_inspector_readonly_field("puzzle5d-play-inspector.fastener.target", "Target", &fastener.target),
                ui_inspector_readonly_field("puzzle5d-play-inspector.fastener.kind", "Kind", fastener.fastener_kind.as_deref().unwrap_or("link")),
            ]);
        }
    }
    ui_stack_vertical(vec![
        ui_text(format!("Schema: {}", envelope.document.schema)),
        ui_text(format!("Parts: {}", envelope.document.parts.len())),
        ui_text(format!("Fasteners: {}", envelope.document.fasteners.len())),
        ui_text(format!("Tool: {}", envelope.runtime.active_tool)),
    ])
}
//#endregion 🔖Panels

//#region 🔖Puzzle5dPlayApp
pub struct Puzzle5dPlayApp {
    precompute: Puzzle5dPrecomputeSession,
    registered_mesh_urls: HashSet<String>,
}

impl Default for Puzzle5dPlayApp {
    fn default() -> Self {
        Self { precompute: Puzzle5dPrecomputeSession::new(), registered_mesh_urls: HashSet::new() }
    }
}

impl Puzzle5dPlayApp {
    fn drive_precompute(&mut self, envelope: &Puzzle5dEnvelope) {
        let _ = self.precompute.set_scene(&scene_config_json(envelope));
        let fallback = semio_framework_plugin::mesh_from_kind(PUZZLE5D_FALLBACK_MESH_KIND);
        self.precompute.register_mesh(PUZZLE5D_FALLBACK_MESH_KIND, &fallback.positions, &fallback.indices);
        for url in collect_mesh_urls(&envelope.document) {
            if !self.registered_mesh_urls.contains(&url) {
                self.precompute.register_mesh(&url, &fallback.positions, &fallback.indices);
            }
        }
        let _ = self.precompute.precompute_step(8);
    }

    fn apply_engine_brush_placement(&mut self, envelope: &Puzzle5dEnvelope, payload: &Value) -> Option<Puzzle5dEnvelope> {
        let brush_payload = serde_json::from_value::<BrushPlacePayload>(payload.clone()).ok()?;
        let fixture_json = self.precompute.apply_brush_placement_rust(&serde_json::to_string(&brush_payload).ok()?).ok()?;
        merge_engine_fixture(envelope, &fixture_json)
    }

    /// 🖌️ Paired placement for a board `brushPlace` event: the engine picks the volume pose for the flat payload's kind, both aspects land in one part.
    fn apply_board_brush_place(&mut self, envelope: &mut Puzzle5dEnvelope, payload: &Value) {
        self.drive_precompute(envelope);
        let node_kind = payload.get("nodeKind").and_then(|value| value.as_str()).unwrap_or("Part").to_string();
        let source_grip = payload.get("sourceHandleId").and_then(|value| value.as_str()).map(str::to_string).or_else(|| puzzle5d_brush_target_grip(envelope));
        if let Some(source_grip) = source_grip.as_ref() {
            let candidates = parse_brush_candidates_free(&self.precompute.brush_candidates(source_grip));
            let candidate_index = candidates
                .iter()
                .position(|candidate| candidate.get("objectKindId").or_else(|| candidate.get("objectKind")).and_then(|value| value.as_str()) == Some(node_kind.as_str()))
                .unwrap_or(envelope.runtime.brush_candidate_index);
            let engine_payload = json!({ "objectKindId": node_kind, "targetVortexFullId": source_grip, "candidateIndex": candidate_index });
            if let Some(mut next) = self.apply_engine_brush_placement(envelope, &engine_payload) {
                let previous_ids: HashSet<String> = envelope.document.parts.iter().map(|part| part.id.clone()).collect();
                let new_id = next.document.parts.iter().map(|part| part.id.clone()).find(|id| !previous_ids.contains(id));
                if let Some(new_id) = new_id {
                    let x = payload.get("x").and_then(|value| value.as_f64());
                    let y = payload.get("y").and_then(|value| value.as_f64());
                    set_part_2d_position(&mut next.document, &new_id, x, y);
                    next.runtime.selection = Puzzle5dSelection { part_ids: vec![new_id], grip_ids: Vec::new(), fastener_ids: Vec::new() };
                }
                *envelope = next;
                return;
            }
        }
        let id = payload.get("nodeId").and_then(|value| value.as_str()).map(str::to_string).unwrap_or_else(next_part_id);
        let x = payload.get("x").and_then(|value| value.as_f64()).unwrap_or(120.0);
        let y = payload.get("y").and_then(|value| value.as_f64()).unwrap_or(120.0);
        let mesh_url = resolve_part_kind_mesh_url(&node_kind, envelope.document.kind_catalogs.as_ref());
        let grips = grips_from_templates(&envelope.document, &node_kind);
        let source_world = source_grip.as_ref().and_then(|full_id| find_part_by_grip_full_id(&envelope.document, full_id).map(|(part, grip)| (world_grip_position(part, grip), world_grip_direction(part, grip))));
        let origin = source_world.map(|(position, direction)| [position[0] + direction[0], position[1] + direction[1], position[2] + direction[2]]).unwrap_or([0.0, 0.0, 0.0]);
        envelope.document.parts.push(Puzzle5dPart {
            id: id.clone(),
            part_kind: node_kind.clone(),
            part_2d: Puzzle5dPart2d { x, y, shape: "circle".into(), radius: PUZZLE5D_DEFAULT_PART_RADIUS, width: None, height: None, text: node_kind, icon_kind: None, hidden: None, locked: None },
            part_3d: Puzzle5dPart3d { origin, mesh_url, orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None, label: None },
            grips,
        });
        if let (Some(source), Some(part)) = (source_grip, envelope.document.parts.last()) {
            if let Some(grip) = part.grips.first() {
                let target = puzzle5d_grip_full_id(&part.id, &grip.id);
                envelope.document.fasteners.push(Puzzle5dFastener { id: payload.get("edgeId").and_then(|value| value.as_str()).map(str::to_string).unwrap_or_else(next_fastener_id), source, target, fastener_kind: None });
            }
        }
        envelope.runtime.selection = Puzzle5dSelection { part_ids: vec![id], grip_ids: Vec::new(), fastener_ids: Vec::new() };
    }

    fn apply_board_events_from_json(&mut self, events_json: &str, envelope: &mut Puzzle5dEnvelope) {
        let Ok(events) = serde_json::from_str::<Vec<Value>>(events_json) else {
            return;
        };
        for event in events {
            let Some(name) = event.get("name").and_then(|value| value.as_str()) else {
                continue;
            };
            let payload = event.get("payload").cloned().unwrap_or(Value::Null);
            match name {
                "camera" => {
                    if let Ok(camera) = serde_json::from_value::<Puzzle5dCamera2d>(payload) {
                        envelope.document.camera2d = camera;
                    }
                }
                "select" => {
                    if let Some(ids) = payload.get("ids").and_then(|value| serde_json::from_value::<Vec<String>>(value.clone()).ok()) {
                        envelope.runtime.selection = classify_selection(&envelope.document, &ids);
                    }
                }
                "nodeDragEnd" => {
                    for entry in payload.get("moves").and_then(|value| value.as_array()).into_iter().flatten() {
                        if let Some(id) = entry.get("id").and_then(|value| value.as_str()) {
                            set_part_2d_position(&mut envelope.document, id, entry.get("x").and_then(|value| value.as_f64()), entry.get("y").and_then(|value| value.as_f64()));
                        }
                    }
                }
                "nodeMove" => {
                    if let Some(id) = payload.get("id").and_then(|value| value.as_str()) {
                        set_part_2d_position(&mut envelope.document, id, payload.get("x").and_then(|value| value.as_f64()), payload.get("y").and_then(|value| value.as_f64()));
                    }
                }
                "brushPlace" => {
                    self.apply_board_brush_place(envelope, &payload);
                }
                "edgeCreate" => {
                    let source = payload.get("source").and_then(|value| value.as_str()).unwrap_or("").to_string();
                    let target = payload.get("target").and_then(|value| value.as_str()).unwrap_or("").to_string();
                    if !source.is_empty() && !target.is_empty() && !envelope.document.fasteners.iter().any(|entry| entry.source == source && entry.target == target || entry.source == target && entry.target == source) {
                        envelope.document.fasteners.push(Puzzle5dFastener {
                            id: payload.get("id").and_then(|value| value.as_str()).map(str::to_string).unwrap_or_else(next_fastener_id),
                            source,
                            target,
                            fastener_kind: payload.get("edgeKind").and_then(|value| value.as_str()).map(str::to_string),
                        });
                    }
                }
                "nodeDelete" => {
                    if let Some(id) = payload.get("id").and_then(|value| value.as_str()) {
                        remove_parts(&mut envelope.document, &[id.to_string()]);
                        envelope.runtime.selection = Puzzle5dSelection::default();
                    }
                }
                "edgeDelete" => {
                    if let Some(id) = payload.get("id").and_then(|value| value.as_str()) {
                        envelope.document.fasteners.retain(|fastener| fastener.id != id);
                    }
                }
                _ => {}
            }
        }
    }
}

impl PluginApp for Puzzle5dPlayApp {
    fn app_id(&self) -> &str {
        PUZZLE5D_PLAY_APP_ID
    }

    fn initial_document_json(&self) -> String {
        serde_json::to_string(&default_envelope()).expect("puzzle5d envelope json")
    }

    fn handle_action_patch_ops(&mut self, action: &str, args: Option<&Value>, document_json: &str, _view_state: &ViewState) -> Vec<String> {
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
                if let Some(json_text) = args.and_then(|value| value.get("json")).and_then(|value| value.as_str()) {
                    if let Ok(document) = serde_json::from_str::<Puzzle5dDocument>(json_text) {
                        envelope.document = document;
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "setActiveExample" => {
                let example_id = args.and_then(|value| value.get("exampleId")).and_then(|value| value.as_str()).unwrap_or("");
                envelope = if example_id.is_empty() || example_id == "empty" {
                    Puzzle5dEnvelope { document: empty_document(), runtime: Puzzle5dRuntime::default() }
                } else if example_id == PUZZLE5D_EXAMPLE_CONCRETE_FOREST || example_id == "concrete" {
                    default_envelope()
                } else if example_id == PUZZLE5D_EXAMPLE_NAKAGIN || example_id == "nakagin" {
                    envelope_from_document_json(NAKAGIN_EXAMPLE_JSON).unwrap_or_else(default_envelope)
                } else {
                    envelope
                };
                self.drive_precompute(&envelope);
                return vec![set_document_op(&envelope)];
            }
            "setSelection" | "documentSelect" => {
                if let Some(ids) = args.and_then(|value| value.get("ids")).and_then(|value| serde_json::from_value::<Vec<String>>(value.clone()).ok()) {
                    envelope.runtime.selection = classify_selection(&envelope.document, &ids);
                } else {
                    let read = |key: &str| args.and_then(|value| value.get(key)).and_then(|value| serde_json::from_value::<Vec<String>>(value.clone()).ok());
                    envelope.runtime.selection = Puzzle5dSelection { part_ids: read("partIds").unwrap_or_default(), grip_ids: read("gripIds").unwrap_or_default(), fastener_ids: read("fastenerIds").unwrap_or_default() };
                }
                return vec![set_document_op(&envelope)];
            }
            "clearSelection" => {
                envelope.runtime.selection = Puzzle5dSelection::default();
                return vec![set_document_op(&envelope)];
            }
            "selectAll" => {
                envelope.runtime.selection = Puzzle5dSelection { part_ids: envelope.document.parts.iter().map(|part| part.id.clone()).collect(), grip_ids: Vec::new(), fastener_ids: Vec::new() };
                return vec![set_document_op(&envelope)];
            }
            "deleteSelection" => {
                let selection = envelope.runtime.selection.clone();
                remove_parts(&mut envelope.document, &selection.part_ids);
                remove_grips(&mut envelope.document, &selection.grip_ids);
                envelope.document.fasteners.retain(|fastener| !selection.fastener_ids.contains(&fastener.id));
                envelope.runtime.selection = Puzzle5dSelection::default();
                return vec![set_document_op(&envelope)];
            }
            "duplicateSelection" => {
                let ids = envelope.runtime.selection.part_ids.clone();
                let clones: Vec<Puzzle5dPart> = envelope
                    .document
                    .parts
                    .iter()
                    .filter(|part| ids.contains(&part.id))
                    .map(|part| {
                        let mut clone = part.clone();
                        clone.id = next_part_id();
                        clone.part_3d.origin[0] += 0.5;
                        clone.part_3d.origin[1] += 0.5;
                        clone.part_2d.x += 48.0;
                        clone.part_2d.y += 24.0;
                        clone
                    })
                    .collect();
                if clones.is_empty() {
                    return Vec::new();
                }
                let new_ids: Vec<String> = clones.iter().map(|part| part.id.clone()).collect();
                envelope.document.parts.extend(clones);
                envelope.runtime.selection = Puzzle5dSelection { part_ids: new_ids, grip_ids: Vec::new(), fastener_ids: Vec::new() };
                return vec![set_document_op(&envelope)];
            }
            "selectSameKindSelection" | "selectSameKind" => {
                let Some(kind) = envelope.runtime.selection.part_ids.first().and_then(|id| envelope.document.parts.iter().find(|part| &part.id == id)).map(|part| part.part_kind.clone()) else {
                    return Vec::new();
                };
                envelope.runtime.selection.part_ids = envelope.document.parts.iter().filter(|part| part.part_kind == kind).map(|part| part.id.clone()).collect();
                return vec![set_document_op(&envelope)];
            }
            "addNode" => {
                let part_kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("Part").to_string();
                let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(120.0);
                let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(120.0);
                add_palette_part(&mut envelope, &part_kind, x, y);
                return vec![set_document_op(&envelope)];
            }
            "setSelectionFlag" => {
                let flag = args.and_then(|value| value.get("flag")).and_then(|value| value.as_str()).unwrap_or("");
                let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_bool()).unwrap_or(false);
                let part_ids = envelope.runtime.selection.part_ids.clone();
                for part in &mut envelope.document.parts {
                    if !part_ids.contains(&part.id) {
                        continue;
                    }
                    match flag {
                        "hidden" => part.part_2d.hidden = Some(value),
                        "locked" => part.part_2d.locked = Some(value),
                        _ => {}
                    }
                }
                if !part_ids.is_empty() && (flag == "hidden" || flag == "locked") {
                    return vec![set_document_op(&envelope)];
                }
            }
            "zoomToSelection" | "focusSelection" => {
                let Some(target) = gumball_target_world(&envelope) else {
                    return Vec::new();
                };
                let camera = &mut envelope.document.camera3d;
                let offset = [camera.position[0] - camera.target[0], camera.position[1] - camera.target[1], camera.position[2] - camera.target[2]];
                camera.target = target;
                camera.position = [target[0] + offset[0], target[1] + offset[1], target[2] + offset[2]];
                let selected_2d: Vec<(f64, f64)> = envelope.document.parts.iter().filter(|part| envelope.runtime.selection.part_ids.contains(&part.id)).map(|part| (part.part_2d.x, part.part_2d.y)).collect();
                if !selected_2d.is_empty() {
                    envelope.document.camera2d.x = selected_2d.iter().map(|(x, _)| x).sum::<f64>() / selected_2d.len() as f64;
                    envelope.document.camera2d.y = selected_2d.iter().map(|(_, y)| y).sum::<f64>() / selected_2d.len() as f64;
                }
                return vec![set_document_op(&envelope)];
            }
            "setActiveTool" => {
                let tool = args.and_then(|value| value.get("tool")).and_then(|value| value.as_str()).unwrap_or("select");
                envelope.runtime.active_tool = tool.into();
                if tool == "brush" || tool == "fill" {
                    self.drive_precompute(&envelope);
                }
                return vec![set_document_op(&envelope)];
            }
            "engagementPossibleSelect" => {
                let possible_id = args.and_then(|value| value.get("possibleId")).and_then(|value| value.as_str()).unwrap_or("");
                envelope.runtime.active_tool = match possible_id {
                    PUZZLE5D_ENGAGEMENT_TOOL_BRUSH => "brush",
                    PUZZLE5D_ENGAGEMENT_TOOL_FILL => "fill",
                    _ => "select",
                }
                .into();
                if envelope.runtime.active_tool != "select" {
                    self.drive_precompute(&envelope);
                }
                if let Some(window) = args.and_then(|value| value.get("window")).and_then(|value| value.as_str()) {
                    if PUZZLE5D_PLAY_WINDOWS.contains(&window) {
                        envelope.runtime.engagement_input_by_window.insert(window.to_string(), String::new());
                    }
                }
                return vec![set_document_op(&envelope)];
            }
            "engagementInput" => {
                let window = args.and_then(|value| value.get("window")).and_then(|value| value.as_str()).unwrap_or(PUZZLE5D_PLAY_WINDOW_2D);
                let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).unwrap_or("");
                if PUZZLE5D_PLAY_WINDOWS.contains(&window) {
                    envelope.runtime.engagement_input_by_window.insert(window.to_string(), value.to_string());
                    return vec![set_document_op(&envelope)];
                }
            }
            "engagementSubmit" => {
                let window = args.and_then(|value| value.get("window")).and_then(|value| value.as_str()).unwrap_or(PUZZLE5D_PLAY_WINDOW_2D).to_string();
                let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).map(str::trim).unwrap_or("").to_lowercase();
                match value.as_str() {
                    "select" | "brush" | "fill" => {
                        envelope.runtime.active_tool = value;
                        if envelope.runtime.active_tool != "select" {
                            self.drive_precompute(&envelope);
                        }
                    }
                    "clear" => envelope.runtime.selection = Puzzle5dSelection::default(),
                    "rectangle" | "lasso" => envelope.runtime.selection_method = value,
                    _ => {}
                }
                if PUZZLE5D_PLAY_WINDOWS.contains(&window.as_str()) {
                    envelope.runtime.engagement_input_by_window.insert(window, String::new());
                }
                return vec![set_document_op(&envelope)];
            }
            "engagementAbort" => {
                if let Some(window) = args.and_then(|value| value.get("window")).and_then(|value| value.as_str()) {
                    if PUZZLE5D_PLAY_WINDOWS.contains(&window) {
                        envelope.runtime.engagement_input_by_window.insert(window.to_string(), String::new());
                    }
                }
                if envelope.runtime.active_tool != "select" {
                    envelope.runtime.active_tool = "select".into();
                }
                return vec![set_document_op(&envelope)];
            }
            "engagementControlSelect" => {
                let candidate_id = args.and_then(|value| value.get("id").or_else(|| value.get("value"))).and_then(|value| value.as_str()).unwrap_or("");
                if let Some(index) = candidate_id.strip_prefix("puzzle5d.brush.candidate.").and_then(|rest| rest.parse::<usize>().ok()) {
                    envelope.runtime.brush_candidate_index = index;
                    return vec![set_document_op(&envelope)];
                }
            }
            "addBrushPart" | "addBrushObject" => {
                self.drive_precompute(&envelope);
                if let Some(payload_value) = args {
                    let mut payload = payload_value.clone();
                    if let Some(object) = payload.as_object_mut() {
                        if let Some(part_kind) = object.remove("partKind") {
                            object.insert("objectKindId".to_string(), part_kind);
                        }
                        if object.get("targetVortexFullId").is_none() {
                            if let Some(grip_id) = puzzle5d_brush_target_grip(&envelope) {
                                object.insert("targetVortexFullId".to_string(), json!(grip_id));
                            }
                        }
                    }
                    if let Some(next) = self.apply_engine_brush_placement(&envelope, &payload) {
                        envelope = next;
                        return vec![set_document_op(&envelope)];
                    }
                }
                let part_kind = args.and_then(|value| value.get("partKind").or_else(|| value.get("objectKindId"))).and_then(|value| value.as_str()).unwrap_or("Part").to_string();
                let payload = json!({ "nodeKind": part_kind, "x": args.and_then(|value| value.get("x")).cloned().unwrap_or(json!(120.0)), "y": args.and_then(|value| value.get("y")).cloned().unwrap_or(json!(120.0)) });
                self.apply_board_brush_place(&mut envelope, &payload);
                return vec![set_document_op(&envelope)];
            }
            "setFillCount" => {
                self.drive_precompute(&envelope);
                let count = args.and_then(|value| value.get("count").or_else(|| value.get("value"))).and_then(|value| value.as_f64()).map(|value| value.round().max(0.0) as u32).unwrap_or(0).min(PUZZLE5D_FILL_COUNT_MAX);
                envelope.runtime.fill_count = count;
                if count > 0 {
                    envelope.runtime.active_tool = "fill".into();
                    if let Ok(fixture_json) = self.precompute.apply_fill_count_rust(count) {
                        if let Some(next) = merge_engine_fixture(&envelope, &fixture_json) {
                            envelope = next;
                        }
                    }
                }
                return vec![set_document_op(&envelope)];
            }
            "cycleBrushCandidate" => {
                self.drive_precompute(&envelope);
                if let Some(grip_full_id) = puzzle5d_brush_target_grip(&envelope) {
                    let free = parse_brush_candidates_free(&self.precompute.brush_candidates(&grip_full_id)).len();
                    if free > 0 {
                        envelope.runtime.brush_candidate_index = (envelope.runtime.brush_candidate_index + 1) % free;
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
                    self.registered_mesh_urls.insert(url.to_string());
                }
                return Vec::new();
            }
            "setBrushPlacementOverlapBudget" => {
                if let Some(value) = args.and_then(|v| v.get("value")).and_then(|v| v.as_f64()) {
                    envelope.runtime.overlap_budget = value.clamp(0.0, 1.0);
                    self.drive_precompute(&envelope);
                    return vec![set_document_op(&envelope)];
                }
            }
            "setObjectKindWeight" | "setVortexKindWeight" => {
                let kind_id = args.and_then(|v| v.get("kindId")).and_then(|v| v.as_str()).unwrap_or("");
                let value = args.and_then(|v| v.get("value")).and_then(|v| v.as_f64()).unwrap_or(1.0);
                if action == "setObjectKindWeight" {
                    envelope.runtime.object_kind_weights.insert(kind_id.into(), value);
                } else {
                    envelope.runtime.vortex_kind_weights.insert(kind_id.into(), value);
                }
                self.drive_precompute(&envelope);
                return vec![set_document_op(&envelope)];
            }
            "addPartKind" => {
                let part_kind = args.and_then(|value| value.get("partKind")).and_then(|value| value.as_str()).unwrap_or("Part").to_string();
                let payload = json!({ "nodeKind": part_kind, "x": 120.0, "y": 120.0 });
                self.apply_board_brush_place(&mut envelope, &payload);
                return vec![set_document_op(&envelope)];
            }
            "patchPart" => {
                let part_id = args.and_then(|value| value.get("partId")).and_then(|value| value.as_str()).unwrap_or("");
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                let value = args.and_then(|value| value.get("value")).cloned().unwrap_or(Value::Null);
                let text = value.as_str().map(str::to_string).unwrap_or_else(|| value.to_string());
                for part in &mut envelope.document.parts {
                    if part.id != part_id {
                        continue;
                    }
                    match field {
                        "partKind" => part.part_kind = text.clone(),
                        "text" => part.part_2d.text = text.clone(),
                        "label" => part.part_3d.label = if text.is_empty() { None } else { Some(text.clone()) },
                        "x" => {
                            if let Ok(parsed) = text.trim().parse::<f64>() {
                                part.part_2d.x = parsed;
                            }
                        }
                        "y" => {
                            if let Ok(parsed) = text.trim().parse::<f64>() {
                                part.part_2d.y = parsed;
                            }
                        }
                        "origin" => {
                            if let Some(origin) = parse_vec3(&text) {
                                part.part_3d.origin = origin;
                            }
                        }
                        _ => {}
                    }
                }
                return vec![set_document_op(&envelope)];
            }
            "patchGrip" => {
                let grip_full_id = args.and_then(|value| value.get("gripFullId")).and_then(|value| value.as_str()).unwrap_or("").to_string();
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                let value = args.and_then(|value| value.get("value")).cloned().unwrap_or(Value::Null);
                let text = value.as_str().map(str::to_string).unwrap_or_else(|| value.to_string());
                for part in &mut envelope.document.parts {
                    let part_id = part.id.clone();
                    for grip in &mut part.grips {
                        if puzzle5d_grip_full_id(&part_id, &grip.id) != grip_full_id {
                            continue;
                        }
                        match field {
                            "gripKind" => {
                                grip.grip_kind = text.clone();
                                grip.grip_2d.grip_kind = text.clone();
                            }
                            "angle" => {
                                if let Ok(parsed) = text.trim().parse::<f64>() {
                                    grip.grip_2d.angle = parsed;
                                }
                            }
                            "radius" => {
                                if let Ok(parsed) = text.trim().parse::<f64>() {
                                    grip.grip_2d.radius = parsed;
                                    grip.grip_3d.radius = parsed;
                                }
                            }
                            "position" => {
                                if let Some(position) = parse_vec3(&text) {
                                    grip.grip_3d.position = position;
                                }
                            }
                            "direction" => {
                                if let Some(direction) = parse_vec3(&text) {
                                    grip.grip_3d.direction = Some(direction);
                                }
                            }
                            "label" => grip.grip_3d.label = if text.is_empty() { None } else { Some(text.clone()) },
                            _ => {}
                        }
                    }
                }
                return vec![set_document_op(&envelope)];
            }
            "setCamera" => {
                if let Some(camera) = args.and_then(|value| value.get("camera")) {
                    let surface_id = args.and_then(|value| value.get("surfaceId")).and_then(|value| value.as_str()).unwrap_or("");
                    if surface_id == PUZZLE5D_PLAY_SURFACE_2D || camera.get("position").is_none() {
                        if let Ok(parsed) = serde_json::from_value::<Puzzle5dCamera2d>(camera.clone()) {
                            envelope.document.camera2d = parsed;
                            return vec![set_document_op(&envelope)];
                        }
                    } else if let Ok(parsed) = serde_json::from_value::<Puzzle5dCamera3d>(camera.clone()) {
                        envelope.document.camera3d = parsed;
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "setCamera2d" => {
                if let Some(camera) = args.and_then(|value| value.get("camera")) {
                    if let Ok(parsed) = serde_json::from_value(camera.clone()) {
                        envelope.document.camera2d = parsed;
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "setCamera3d" => {
                if let Some(camera) = args.and_then(|value| value.get("camera")) {
                    if let Ok(parsed) = serde_json::from_value(camera.clone()) {
                        envelope.document.camera3d = parsed;
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "translateSelection" => {
                let ids = mesh_selection_ids(args, &envelope.runtime.selection.part_ids);
                let dx = args.and_then(|value| value.get("dx")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let dy = args.and_then(|value| value.get("dy")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let dz = args.and_then(|value| value.get("dz")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                for part in &mut envelope.document.parts {
                    if ids.contains(&part.id) {
                        part.part_3d.origin[0] += dx;
                        part.part_3d.origin[1] += dy;
                        part.part_3d.origin[2] += dz;
                    }
                }
                if !ids.is_empty() {
                    return vec![set_document_op(&envelope)];
                }
            }
            "rotateSelection" => {
                let ids = mesh_selection_ids(args, &envelope.runtime.selection.part_ids);
                let ax = args.and_then(|value| value.get("ax")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let ay = args.and_then(|value| value.get("ay")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let az = args.and_then(|value| value.get("az")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let angle = args.and_then(|value| value.get("angle")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let delta = quat_from_axis_angle(ax, ay, az, angle);
                for part in &mut envelope.document.parts {
                    if ids.contains(&part.id) {
                        let current = part.part_3d.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
                        part.part_3d.orientation = Some(quat_mul(delta, current));
                    }
                }
                if !ids.is_empty() {
                    return vec![set_document_op(&envelope)];
                }
            }
            "scaleSelection" => {
                let ids = mesh_selection_ids(args, &envelope.runtime.selection.part_ids);
                let sx = args.and_then(|value| value.get("sx")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                let sy = args.and_then(|value| value.get("sy")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                let sz = args.and_then(|value| value.get("sz")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                for part in &mut envelope.document.parts {
                    if ids.contains(&part.id) {
                        let current = part_scale_json(part);
                        part.part_3d.scale = Some(json!([current[0] * sx, current[1] * sy, current[2] * sz]));
                    }
                }
                if !ids.is_empty() {
                    return vec![set_document_op(&envelope)];
                }
            }
            "worldSelect" => {
                let merge = args.and_then(|value| value.get("merge")).and_then(|value| value.as_str()).unwrap_or("replace");
                let ids: Vec<String> = args.and_then(|value| value.get("ids")).and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default();
                envelope.runtime.selection.part_ids = merge_world_selection_ids(&envelope.runtime.selection.part_ids, &ids, merge);
                return vec![set_document_op(&envelope)];
            }
            "worldPick" => {
                let merge = args.and_then(|value| value.get("merge")).and_then(|value| value.as_str()).unwrap_or("replace");
                if args.and_then(|value| value.get("id")).is_none_or(|value| value.is_null()) {
                    if merge == "replace" {
                        envelope.runtime.selection.part_ids.clear();
                    }
                    return vec![set_document_op(&envelope)];
                }
                let index = args.and_then(|value| value.get("id")).and_then(|value| value.as_u64()).unwrap_or(0) as usize;
                if let Some(part) = envelope.document.parts.get(index) {
                    let id = part.id.clone();
                    envelope.runtime.selection.part_ids = match merge {
                        "add" => {
                            let mut merged = envelope.runtime.selection.part_ids.clone();
                            if !merged.contains(&id) {
                                merged.push(id);
                            }
                            merged
                        }
                        "toggle" => {
                            let mut merged = envelope.runtime.selection.part_ids.clone();
                            if let Some(position) = merged.iter().position(|entry| entry == &id) {
                                merged.remove(position);
                            } else {
                                merged.push(id);
                            }
                            merged
                        }
                        _ => vec![id],
                    };
                    return vec![set_document_op(&envelope)];
                }
            }
            "worldHover" => {
                envelope.runtime.hovered_part_id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()).map(str::to_string);
                return vec![set_document_op(&envelope)];
            }
            "setHover" => {
                envelope.runtime.hovered_part_id = args.and_then(|value| value.get("objectId")).and_then(|value| value.as_str()).map(str::to_string);
                return vec![set_document_op(&envelope)];
            }
            "setTransformTool" => {
                if let Some(tool) = args.and_then(|value| value.get("tool")).and_then(|value| value.as_str()) {
                    envelope.runtime.transform_tool = tool.into();
                    return vec![set_document_op(&envelope)];
                }
            }
            "worldVortexHover" => {
                envelope.runtime.selection.grip_ids = args.and_then(|value| value.get("fullId")).and_then(|value| value.as_str()).map(|full_id| vec![full_id.to_string()]).unwrap_or_default();
                if envelope.runtime.active_tool == "brush" && !envelope.runtime.selection.grip_ids.is_empty() {
                    self.drive_precompute(&envelope);
                }
                return vec![set_document_op(&envelope)];
            }
            "worldVortexSelect" => {
                if let Some(full_id) = args.and_then(|value| value.get("fullId")).and_then(|value| value.as_str()) {
                    envelope.runtime.selection.grip_ids = vec![full_id.to_string()];
                    envelope.runtime.selection.part_ids.clear();
                    self.drive_precompute(&envelope);
                    return vec![set_document_op(&envelope)];
                }
            }
            "worldRelocate" => {
                let object_id = args.and_then(|value| value.get("objectId")).and_then(|value| value.as_str()).unwrap_or("");
                let position = args.and_then(|value| value.get("position")).and_then(|value| serde_json::from_value::<[f64; 3]>(value.clone()).ok());
                if let (Some(part), Some(position)) = (envelope.document.parts.iter_mut().find(|part| part.id == object_id), position) {
                    part.part_3d.origin = position;
                    let source_grip = part.grips.first().map(|grip| (puzzle5d_grip_full_id(&part.id, &grip.id), world_grip_position(part, grip)));
                    if let Some((source_id, source_position)) = source_grip {
                        for other in &envelope.document.parts {
                            if other.id == object_id {
                                continue;
                            }
                            for grip in &other.grips {
                                let target_id = puzzle5d_grip_full_id(&other.id, &grip.id);
                                if target_id == source_id {
                                    continue;
                                }
                                let target_position = world_grip_position(other, grip);
                                let dx = source_position[0] - target_position[0];
                                let dy = source_position[1] - target_position[1];
                                let dz = source_position[2] - target_position[2];
                                if (dx * dx + dy * dy + dz * dz).sqrt() <= PUZZLE5D_PROXIMITY_RADIUS
                                    && !envelope.document.fasteners.iter().any(|entry| entry.source == source_id && entry.target == target_id || entry.source == target_id && entry.target == source_id)
                                {
                                    envelope.document.fasteners.push(Puzzle5dFastener { id: next_fastener_id(), source: source_id.clone(), target: target_id, fastener_kind: None });
                                }
                            }
                        }
                    }
                    self.drive_precompute(&envelope);
                    return vec![set_document_op(&envelope)];
                }
            }
            "setSelectionMethod" => {
                let method = args.and_then(|value| value.get("method")).and_then(|value| value.as_str()).unwrap_or("rectangle");
                envelope.runtime.selection_method = method.into();
                return vec![set_document_op(&envelope)];
            }
            "setLodMode" => {
                if let Some(mode) = args.and_then(|value| value.get("value").or_else(|| value.get("mode"))).and_then(|value| value.as_str()) {
                    envelope.runtime.lod_mode = mode.into();
                    return vec![set_document_op(&envelope)];
                }
            }
            "setSuggestionOffset" => {
                if let Some(distance) = args.and_then(|value| value.get("distance").or_else(|| value.get("value"))).and_then(|value| value.as_f64()) {
                    envelope.runtime.suggestion_offset = distance.clamp(PUZZLE5D_SUGGESTION_OFFSET_MIN, PUZZLE5D_SUGGESTION_OFFSET_MAX);
                    return vec![set_document_op(&envelope)];
                }
            }
            "setGridSnapEnabled" => {
                envelope.runtime.grid_snap_enabled = args.and_then(|value| value.get("enabled")).and_then(|value| value.as_bool()).unwrap_or(false);
                return vec![set_document_op(&envelope)];
            }
            "setGridFactor" => {
                if let Some(value) = args.and_then(|value| value.get("value")).and_then(|value| value.as_f64()) {
                    envelope.runtime.grid_factor = value;
                    return vec![set_document_op(&envelope)];
                }
            }
            "applyBoardEvents" => {
                if let Some(events_json) = args.and_then(|value| value.get("eventsJson")).and_then(|value| value.as_str()) {
                    self.apply_board_events_from_json(events_json, &mut envelope);
                    return vec![set_document_op(&envelope)];
                }
            }
            "worldPointerDown" | "canvasPointerDown" => return Vec::new(),
            _ => {}
        }
        Vec::new()
    }

    fn render(&self, body_key: &str, document_json: &str, view_state: &ViewState) -> UiNode {
        let envelope = parse_envelope(document_json);
        let labels = puzzle5d_labels(view_state);
        match body_key {
            PUZZLE5D_PLAY_BODY_2D => build_puzzle2d_board_scene(PUZZLE5D_PLAY_SURFACE_2D, PUZZLE5D_PLAY_CONTROLLER_ID, puzzle5d_board_scene(&envelope)),
            PUZZLE5D_PLAY_BODY_3D => {
                let brush_preview = world_brush_preview_json(&self.precompute, &envelope);
                build_world_3d_scene(
                    PUZZLE5D_PLAY_SURFACE_3D,
                    PUZZLE5D_PLAY_CONTROLLER_ID,
                    world3d_scene_extended(
                        camera3d_json(&envelope.document.camera3d),
                        world_meshes_json(&envelope.document),
                        world_instances_json(&envelope.document, &envelope.runtime),
                        world_selection_json_ex(&envelope),
                        Some(world_grips_json(&envelope.document)),
                        Some(world_fasteners_json(&envelope.document)),
                        None,
                        None,
                        brush_preview,
                        Some(world_interaction_json(&envelope.runtime)),
                        None,
                        None,
                        Some(world3d_chunking_json(256.0, 8000.0)),
                        puzzle5d_context_menu_json(&envelope, labels),
                    ),
                )
            }
            PUZZLE5D_PLAY_BODY_DOCUMENT => build_document_tree(&envelope, labels),
            PUZZLE5D_PLAY_BODY_KINDS => build_kinds_tree(&envelope, labels),
            PUZZLE5D_PLAY_BODY_INSPECTOR => build_inspector_tree(&envelope, labels),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn window_engagements(&self, document_json: &str, view_state: &ViewState) -> HashMap<String, WindowEngagement> {
        let envelope = parse_envelope(document_json);
        let labels = puzzle5d_labels(view_state);
        PUZZLE5D_PLAY_WINDOWS.iter().map(|window| (window.to_string(), puzzle5d_engagement(&envelope, &self.precompute, window, labels))).collect()
    }

    fn window_measures(&self, document_json: &str, view_state: &ViewState) -> HashMap<String, Vec<WindowMeasure>> {
        let envelope = parse_envelope(document_json);
        let labels = puzzle5d_labels(view_state);
        PUZZLE5D_PLAY_WINDOWS.iter().map(|window| (window.to_string(), puzzle5d_window_measures(window, &envelope, labels))).collect()
    }
}
//#endregion 🔖Puzzle5dPlayApp

//#region 🔖Manifest
pub fn create_puzzle5d_app() -> App {
    let envelope = default_envelope();
    let precompute = Puzzle5dPrecomputeSession::new();
    let manifest_labels = puzzle5d_labels(&ViewState::default());
    let mut app = App::from_builder(
        App::builder(PUZZLE5D_PLAY_APP_ID, "Puzzle 5D")
            .document(["semio", "puzzle", "5d"])
            .icon_id("puzzle")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind_with_engagement(PUZZLE5D_PLAY_WINDOW_2D, "Puzzle 2D", PUZZLE5D_PLAY_BODY_2D, SurfaceKind::Puzzle2dBoard, puzzle5d_engagement(&envelope, &precompute, PUZZLE5D_PLAY_WINDOW_2D, manifest_labels))
            .window_kind_with_engagement(PUZZLE5D_PLAY_WINDOW_3D, "Puzzle 3D", PUZZLE5D_PLAY_BODY_3D, SurfaceKind::World3d, puzzle5d_engagement(&envelope, &precompute, PUZZLE5D_PLAY_WINDOW_3D, manifest_labels))
            .default_layout(create_default_layout(&[PUZZLE5D_PLAY_WINDOW_2D.into(), PUZZLE5D_PLAY_WINDOW_3D.into()], "row", Some(&[50.0, 50.0]), Some(&["Puzzle 2D".into(), "Puzzle 3D".into()])))
            .panel_tab(FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, PanelGroup::Workbench, PUZZLE5D_PLAY_BODY_DOCUMENT)
            .panel_tab(FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, PanelGroup::Workbench, PUZZLE5D_PLAY_BODY_KINDS)
            .panel_tab(FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, PanelGroup::Details, PUZZLE5D_PLAY_BODY_INSPECTOR),
    );
    for window in PUZZLE5D_PLAY_WINDOWS {
        if let Some(window_kind) = app.definition.window_kinds.iter_mut().find(|window_kind| window_kind.id == window) {
            window_kind.measures = puzzle5d_window_measures(window, &envelope, manifest_labels);
        }
    }
    app.example("empty", "Empty", serde_json::to_string(&empty_document()).unwrap())
        .example(PUZZLE5D_EXAMPLE_CONCRETE_FOREST, "Concrete Forest", CONCRETE_FOREST_EXAMPLE_JSON)
        .example(PUZZLE5D_EXAMPLE_NAKAGIN, "Nakagin Capsule Tower", NAKAGIN_EXAMPLE_JSON)
        .program("puzzle5d", "Puzzle 5D", "model")
}

/// 📥 Tier C DWG mesh import — always returns the empty puzzle-5d document; never errors on a structurally valid mesh.
fn puzzle5d_document_from_mesh(_mesh: &semio_framework_plugin::MeshData) -> Result<Value, String> {
    serde_json::to_value(Puzzle5dEnvelope { document: empty_document(), runtime: Puzzle5dRuntime::default() }).map_err(|error| error.to_string())
}

pub fn register_puzzle5d_exports() {
    register_mesh_export_handlers("5d.puzzle", "puzzle5d", |_| Ok(semio_framework_plugin::mesh_from_kind("box")));
    semio_framework_os::register_mesh_dwg_import_handler("5d.puzzle", puzzle5d_document_from_mesh);
}
//#endregion 🔖Manifest

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::PluginApp;

    fn apply_ops(envelope: &Puzzle5dEnvelope, ops: &[String]) -> Puzzle5dEnvelope {
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
    fn renders_puzzle2d_board_scene() {
        let app = Puzzle5dPlayApp::default();
        let document = app.initial_document_json();
        let node = app.render(PUZZLE5D_PLAY_BODY_2D, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("puzzle2d-board"));
        assert!(json.contains("seed-left-001"));
        assert!(json.contains("activeTool"));
    }

    #[test]
    fn renders_world_3d_scene() {
        let app = Puzzle5dPlayApp::default();
        let document = app.initial_document_json();
        let node = app.render(PUZZLE5D_PLAY_BODY_3D, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("world-3d"));
        assert!(json.contains("hexagonal-cut-concrete-forest-left"));
        assert!(json.contains("transformTool"));
        assert!(json.contains("granularity"));
    }

    #[test]
    fn concrete_forest_example_parses() {
        let envelope = default_envelope();
        assert_eq!(envelope.document.schema, PUZZLE5D_SCHEMA);
        assert!(!envelope.document.parts.is_empty());
    }

    #[test]
    fn puzzle5d_document_from_mesh_returns_valid_empty_document() {
        let mesh = semio_framework_plugin::mesh_from_kind("box");
        let document = puzzle5d_document_from_mesh(&mesh).unwrap();
        let envelope: Puzzle5dEnvelope = serde_json::from_value(document).unwrap();
        assert_eq!(envelope.document.schema, PUZZLE5D_SCHEMA);
        assert!(envelope.document.parts.is_empty());
    }

    #[test]
    fn nakagin_example_parses_with_fasteners() {
        let envelope = envelope_from_document_json(NAKAGIN_EXAMPLE_JSON).expect("nakagin envelope");
        assert_eq!(envelope.document.parts.len(), 180);
        assert_eq!(envelope.document.fasteners.len(), 179);
        assert!(envelope.document.fasteners.iter().all(|fastener| fastener.source.contains(':') && fastener.target.contains(':')));
    }

    #[test]
    fn board_fixture_projects_nodes_handles_edges() {
        let envelope = envelope_from_document_json(NAKAGIN_EXAMPLE_JSON).expect("nakagin envelope");
        let fixture = board_fixture_value(&envelope.document);
        assert_eq!(fixture["schema"], PUZZLE5D_BOARD_FIXTURE_SCHEMA);
        assert_eq!(fixture["nodes"].as_array().unwrap().len(), 180);
        assert_eq!(fixture["edges"].as_array().unwrap().len(), 179);
        let node = &fixture["nodes"][0];
        assert!(node["handles"].as_array().is_some_and(|handles| !handles.is_empty()));
        assert!(node["handles"][0]["id"].as_str().unwrap().contains(':'));
    }

    #[test]
    fn apply_board_events_updates_selection_camera_and_positions() {
        let mut app = Puzzle5dPlayApp::default();
        let document = app.initial_document_json();
        let events = json!([
            { "name": "select", "payload": { "ids": ["seed-left-001"] } },
            { "name": "camera", "payload": { "x": 10.0, "y": 20.0, "zoom": 1.5 } },
            { "name": "nodeMove", "payload": { "id": "seed-left-001", "x": 111.0, "y": 222.0 } }
        ])
        .to_string();
        let ops = app.handle_action_patch_ops("applyBoardEvents", Some(&json!({ "eventsJson": events })), &document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&document), &ops);
        assert_eq!(envelope.runtime.selection.part_ids, vec!["seed-left-001".to_string()]);
        assert_eq!(envelope.document.camera2d.x, 10.0);
        assert_eq!(envelope.document.camera2d.zoom, 1.5);
        let part = envelope.document.parts.iter().find(|part| part.id == "seed-left-001").unwrap();
        assert_eq!(part.part_2d.x, 111.0);
        assert_eq!(part.part_2d.y, 222.0);
    }

    #[test]
    fn apply_board_events_edge_create_adds_fastener() {
        let mut app = Puzzle5dPlayApp::default();
        let mut envelope = default_envelope();
        envelope.document.parts.push(Puzzle5dPart {
            id: "part-b".into(),
            part_kind: "Hexagonal Cut Concrete Forest Right".into(),
            part_2d: Puzzle5dPart2d { x: 320.0, y: 93.0, shape: "circle".into(), radius: 20.0, width: None, height: None, text: "b".into(), icon_kind: None, hidden: None, locked: None },
            part_3d: Puzzle5dPart3d::default(),
            grips: vec![Puzzle5dGrip { id: "v0".into(), grip_kind: "b-l".into(), grip_2d: Puzzle5dGrip2d::default(), grip_3d: Puzzle5dGrip3d::default() }],
        });
        let document = serde_json::to_string(&envelope).unwrap();
        let events = json!([{ "name": "edgeCreate", "payload": { "id": "edge-1", "edgeKind": "link", "source": "seed-left-001:v0", "target": "part-b:v0" } }]).to_string();
        let ops = app.handle_action_patch_ops("applyBoardEvents", Some(&json!({ "eventsJson": events })), &document, &ViewState::default());
        let next = apply_ops(&envelope, &ops);
        assert_eq!(next.document.fasteners.len(), 1);
        assert_eq!(next.document.fasteners[0].source, "seed-left-001:v0");
        assert_eq!(next.document.fasteners[0].target, "part-b:v0");
    }

    #[test]
    fn world_selection_includes_gumball_fields() {
        let mut envelope = default_envelope();
        envelope.runtime.selection.part_ids = vec!["seed-left-001".into()];
        let json_text = world_selection_json_ex(&envelope);
        let value: Value = serde_json::from_str(&json_text).unwrap();
        assert_eq!(value["granularity"], "mesh");
        assert_eq!(value["transformTool"], "move");
        assert_eq!(value["gumballActive"], true);
        assert_eq!(value["activeObjectId"], "seed-left-001");
        assert!(value["gumballTarget"].is_array());
    }

    #[test]
    fn world_pick_selects_by_index() {
        let mut app = Puzzle5dPlayApp::default();
        let document = app.initial_document_json();
        let ops = app.handle_action_patch_ops("worldPick", Some(&json!({ "id": 0, "merge": "replace" })), &document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&document), &ops);
        assert_eq!(envelope.runtime.selection.part_ids, vec!["seed-left-001".to_string()]);
    }

    #[test]
    fn set_hover_and_world_hover_update_hovered_part() {
        let mut app = Puzzle5dPlayApp::default();
        let document = app.initial_document_json();
        let ops = app.handle_action_patch_ops("setHover", Some(&json!({ "objectId": "seed-left-001" })), &document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&document), &ops);
        assert_eq!(envelope.runtime.hovered_part_id.as_deref(), Some("seed-left-001"));
        let cleared_ops = app.handle_action_patch_ops("setHover", Some(&json!({ "objectId": Value::Null })), &document, &ViewState::default());
        let cleared = apply_ops(&parse_envelope(&document), &cleared_ops);
        assert_eq!(cleared.runtime.hovered_part_id, None);
    }

    #[test]
    fn set_transform_tool_updates_runtime() {
        let mut app = Puzzle5dPlayApp::default();
        let document = app.initial_document_json();
        let ops = app.handle_action_patch_ops("setTransformTool", Some(&json!({ "tool": "rotate" })), &document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&document), &ops);
        assert_eq!(envelope.runtime.transform_tool, "rotate");
    }

    #[test]
    fn context_menu_present_when_parts_selected() {
        let mut envelope = default_envelope();
        let labels = puzzle5d_labels(&ViewState::default());
        assert!(puzzle5d_context_menu_json(&envelope, labels).is_none());
        envelope.runtime.selection.part_ids = vec!["seed-left-001".into()];
        let menu = puzzle5d_context_menu_json(&envelope, labels).unwrap();
        assert!(menu.contains("duplicateSelection"));
        assert!(menu.contains("zoomToSelection"));
    }

    #[test]
    fn duplicate_selection_clones_parts_paired() {
        let mut app = Puzzle5dPlayApp::default();
        let mut envelope = default_envelope();
        envelope.runtime.selection.part_ids = vec!["seed-left-001".into()];
        let document = serde_json::to_string(&envelope).unwrap();
        let ops = app.handle_action_patch_ops("duplicateSelection", None, &document, &ViewState::default());
        let next = apply_ops(&envelope, &ops);
        assert_eq!(next.document.parts.len(), 2);
        let clone = next.document.parts.iter().find(|part| part.id != "seed-left-001").unwrap();
        assert!(clone.part_2d.x > envelope.document.parts[0].part_2d.x);
        assert!(clone.part_3d.origin[0] > envelope.document.parts[0].part_3d.origin[0]);
    }

    #[test]
    fn zoom_to_selection_targets_both_cameras() {
        let mut app = Puzzle5dPlayApp::default();
        let mut envelope = default_envelope();
        envelope.runtime.selection.part_ids = vec!["seed-left-001".into()];
        let document = serde_json::to_string(&envelope).unwrap();
        let ops = app.handle_action_patch_ops("zoomToSelection", None, &document, &ViewState::default());
        let next = apply_ops(&envelope, &ops);
        assert_eq!(next.document.camera3d.target, envelope.document.parts[0].part_3d.origin);
        assert_eq!(next.document.camera2d.x, envelope.document.parts[0].part_2d.x);
    }

    #[test]
    fn window_engagements_cover_both_windows() {
        let app = Puzzle5dPlayApp::default();
        let document = app.initial_document_json();
        let engagements = app.window_engagements(&document, &ViewState::default());
        assert!(engagements.contains_key(PUZZLE5D_PLAY_WINDOW_2D));
        assert!(engagements.contains_key(PUZZLE5D_PLAY_WINDOW_3D));
        let engagement = engagements.get(PUZZLE5D_PLAY_WINDOW_2D).unwrap();
        assert_eq!(engagement.options.as_ref().map(|options| options.len()), Some(3));
    }

    #[test]
    fn window_measures_cover_both_windows() {
        let app = Puzzle5dPlayApp::default();
        let document = app.initial_document_json();
        let measures = app.window_measures(&document, &ViewState::default());
        assert!(measures.get(PUZZLE5D_PLAY_WINDOW_2D).is_some_and(|entries| !entries.is_empty()));
        assert!(measures.get(PUZZLE5D_PLAY_WINDOW_3D).is_some_and(|entries| !entries.is_empty()));
    }

    #[test]
    fn engagement_possible_select_switches_tool() {
        let mut app = Puzzle5dPlayApp::default();
        let document = app.initial_document_json();
        let ops = app.handle_action_patch_ops("engagementPossibleSelect", Some(&json!({ "window": PUZZLE5D_PLAY_WINDOW_3D, "possibleId": PUZZLE5D_ENGAGEMENT_TOOL_BRUSH })), &document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&document), &ops);
        assert_eq!(envelope.runtime.active_tool, "brush");
    }

    #[test]
    fn catalogue_derives_from_kind_catalogs() {
        let app = Puzzle5dPlayApp::default();
        let document = app.initial_document_json();
        let node = app.render(PUZZLE5D_PLAY_BODY_KINDS, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Hexagonal Cut Concrete Forest Left"));
        assert!(json.contains("Hexagonal Cut Concrete Forest Right"));
        assert!(json.contains("b-l"));
        assert!(json.contains("puzzle5d-play-kinds.ropes"));
    }

    #[test]
    fn document_tree_reflects_selection_and_fasteners() {
        let mut envelope = envelope_from_document_json(NAKAGIN_EXAMPLE_JSON).expect("nakagin envelope");
        envelope.runtime.selection.part_ids = vec![envelope.document.parts[0].id.clone()];
        let node = build_document_tree(&envelope, puzzle5d_labels(&ViewState::default()));
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("puzzle5d-play-document.fasteners"));
        assert!(json.contains(&format!("puzzle5d-play-document.part.{}", envelope.document.parts[0].id)));
    }

    #[test]
    fn set_selection_classifies_mixed_ids() {
        let mut app = Puzzle5dPlayApp::default();
        let envelope = envelope_from_document_json(NAKAGIN_EXAMPLE_JSON).expect("nakagin envelope");
        let part_id = envelope.document.parts[0].id.clone();
        let grip_full_id = puzzle5d_grip_full_id(&part_id, &envelope.document.parts[0].grips[0].id);
        let fastener_id = envelope.document.fasteners[0].id.clone();
        let document = serde_json::to_string(&envelope).unwrap();
        let ops = app.handle_action_patch_ops("setSelection", Some(&json!({ "ids": [part_id, grip_full_id, fastener_id] })), &document, &ViewState::default());
        let next = apply_ops(&envelope, &ops);
        assert_eq!(next.runtime.selection.part_ids.len(), 1);
        assert_eq!(next.runtime.selection.grip_ids.len(), 1);
        assert_eq!(next.runtime.selection.fastener_ids.len(), 1);
    }

    #[test]
    fn patch_part_updates_flat_and_volume_fields() {
        let mut app = Puzzle5dPlayApp::default();
        let document = app.initial_document_json();
        let ops = app.handle_action_patch_ops("patchPart", Some(&json!({ "partId": "seed-left-001", "field": "origin", "value": "1, 2, 3" })), &document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&document), &ops);
        assert_eq!(envelope.document.parts[0].part_3d.origin, [1.0, 2.0, 3.0]);
        let ops = app.handle_action_patch_ops("patchPart", Some(&json!({ "partId": "seed-left-001", "field": "x", "value": "42.5" })), &document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&document), &ops);
        assert_eq!(envelope.document.parts[0].part_2d.x, 42.5);
    }

    #[test]
    fn patch_grip_updates_fields() {
        let mut app = Puzzle5dPlayApp::default();
        let document = app.initial_document_json();
        let ops = app.handle_action_patch_ops("patchGrip", Some(&json!({ "gripFullId": "seed-left-001:v0", "field": "angle", "value": "1.5707" })), &document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&document), &ops);
        assert_eq!(envelope.document.parts[0].grips[0].grip_2d.angle, 1.5707);
    }

    #[test]
    fn merge_engine_fixture_preserves_flat_aspects() {
        let envelope = default_envelope();
        let seed_2d = envelope.document.parts[0].part_2d.clone();
        let engine_fixture = json!({
            "objects": [
                { "id": "seed-left-001", "objectKind": "Hexagonal Cut Concrete Forest Left", "origin": [1.0, 1.0, 1.0], "orientation": [0.0, 0.0, 0.0, 1.0],
                  "vortices": envelope.document.parts[0].grips.iter().map(|grip| json!({ "id": grip.id, "vortexKind": grip.grip_kind, "position": grip.grip_3d.position, "direction": grip.grip_3d.direction })).collect::<Vec<_>>() },
                { "id": "brush-1", "objectKind": "Hexagonal Cut Concrete Forest Right", "origin": [8.0, 0.0, 0.0], "orientation": [0.0, 0.0, 0.0, 1.0],
                  "vortices": [{ "id": "v0", "vortexKind": "b-r", "position": [0.0, 0.0, 0.0], "direction": [0.0, -1.0, 0.0] }] }
            ],
            "attractions": [ { "id": "att-1", "attracting": "seed-left-001:v0", "attracted": "brush-1:v0" } ]
        })
        .to_string();
        let next = merge_engine_fixture(&envelope, &engine_fixture).expect("merged envelope");
        let seed = next.document.parts.iter().find(|part| part.id == "seed-left-001").unwrap();
        assert_eq!(seed.part_2d, seed_2d);
        assert_eq!(seed.part_3d.origin, [1.0, 1.0, 1.0]);
        let placed = next.document.parts.iter().find(|part| part.id == "brush-1").unwrap();
        assert!(placed.part_2d.x != 0.0 || placed.part_2d.y != 0.0);
        assert_eq!(next.document.fasteners.len(), 1);
        assert_eq!(next.document.fasteners[0].source, "seed-left-001:v0");
    }

    #[test]
    fn delete_selection_removes_parts_grips_and_fasteners() {
        let mut app = Puzzle5dPlayApp::default();
        let mut envelope = envelope_from_document_json(NAKAGIN_EXAMPLE_JSON).expect("nakagin envelope");
        let part_id = envelope.document.parts[0].id.clone();
        envelope.runtime.selection.part_ids = vec![part_id.clone()];
        let fasteners_touching: usize = envelope.document.fasteners.iter().filter(|fastener| fastener.source.starts_with(&part_id) || fastener.target.starts_with(&part_id)).count();
        assert!(fasteners_touching > 0);
        let document = serde_json::to_string(&envelope).unwrap();
        let ops = app.handle_action_patch_ops("deleteSelection", None, &document, &ViewState::default());
        let next = apply_ops(&envelope, &ops);
        assert!(next.document.parts.iter().all(|part| part.id != part_id));
        assert!(next.document.fasteners.iter().all(|fastener| !fastener.source.starts_with(&part_id) && !fastener.target.starts_with(&part_id)));
    }

    #[test]
    fn world_relocate_moves_part_and_creates_proximity_fastener() {
        let mut app = Puzzle5dPlayApp::default();
        let grip = |id: &str| Puzzle5dGrip { id: id.into(), grip_kind: "k".into(), grip_2d: Puzzle5dGrip2d::default(), grip_3d: Puzzle5dGrip3d { position: [0.0, 0.0, 0.0], direction: None, radius: 0.36, label: None } };
        let part = |id: &str, origin: [f64; 3]| Puzzle5dPart {
            id: id.into(),
            part_kind: "Test".into(),
            part_2d: Puzzle5dPart2d::default(),
            part_3d: Puzzle5dPart3d { origin, mesh_url: None, orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None, label: None },
            grips: vec![grip("g0")],
        };
        let mut document = empty_document();
        document.parts = vec![part("part-a", [0.0, 0.0, 0.0]), part("part-b", [10.0, 10.0, 10.0])];
        let envelope = Puzzle5dEnvelope { document, runtime: Puzzle5dRuntime::default() };
        let document_json = serde_json::to_string(&envelope).unwrap();
        let ops = app.handle_action_patch_ops("worldRelocate", Some(&json!({ "objectId": "part-b", "position": [0.0, 0.0, 0.0] })), &document_json, &ViewState::default());
        let next = apply_ops(&envelope, &ops);
        assert_eq!(next.document.parts.iter().find(|part| part.id == "part-b").unwrap().part_3d.origin, [0.0, 0.0, 0.0]);
        assert_eq!(next.document.fasteners.len(), 1);
        assert_eq!(next.document.fasteners[0].source, "part-b:g0");
        assert_eq!(next.document.fasteners[0].target, "part-a:g0");
    }

    #[test]
    fn set_brush_placement_overlap_budget_clamps_value() {
        let mut app = Puzzle5dPlayApp::default();
        let document = app.initial_document_json();
        let ops = app.handle_action_patch_ops("setBrushPlacementOverlapBudget", Some(&json!({ "value": 5.0 })), &document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&document), &ops);
        assert_eq!(envelope.runtime.overlap_budget, 1.0);
    }

    #[test]
    fn set_object_and_vortex_kind_weight_updates_runtime_maps() {
        let mut app = Puzzle5dPlayApp::default();
        let document = app.initial_document_json();
        let object_ops = app.handle_action_patch_ops("setObjectKindWeight", Some(&json!({ "kindId": "Hexagonal Cut Concrete Forest Left", "value": 2.0 })), &document, &ViewState::default());
        let with_object_weight = apply_ops(&parse_envelope(&document), &object_ops);
        assert_eq!(with_object_weight.runtime.object_kind_weights.get("Hexagonal Cut Concrete Forest Left"), Some(&2.0));
        let vortex_ops = app.handle_action_patch_ops("setVortexKindWeight", Some(&json!({ "kindId": "b-l", "value": 0.5 })), &document, &ViewState::default());
        let with_vortex_weight = apply_ops(&parse_envelope(&document), &vortex_ops);
        assert_eq!(with_vortex_weight.runtime.vortex_kind_weights.get("b-l"), Some(&0.5));
    }

    #[test]
    fn world_vortex_hover_and_select_update_grip_selection() {
        let mut app = Puzzle5dPlayApp::default();
        let document = app.initial_document_json();
        let full_id = "seed-left-001:v0";
        let hover_ops = app.handle_action_patch_ops("worldVortexHover", Some(&json!({ "fullId": full_id })), &document, &ViewState::default());
        let hovered = apply_ops(&parse_envelope(&document), &hover_ops);
        assert_eq!(hovered.runtime.selection.grip_ids, vec![full_id.to_string()]);
        let select_ops = app.handle_action_patch_ops("worldVortexSelect", Some(&json!({ "fullId": full_id })), &document, &ViewState::default());
        let selected = apply_ops(&parse_envelope(&document), &select_ops);
        assert_eq!(selected.runtime.selection.grip_ids, vec![full_id.to_string()]);
        assert!(selected.runtime.selection.part_ids.is_empty());
    }

    #[test]
    fn set_camera_routes_by_surface() {
        let mut app = Puzzle5dPlayApp::default();
        let document = app.initial_document_json();
        let ops_2d = app.handle_action_patch_ops("setCamera", Some(&json!({ "surfaceId": PUZZLE5D_PLAY_SURFACE_2D, "camera": { "x": 5.0, "y": 6.0, "zoom": 2.0 } })), &document, &ViewState::default());
        let envelope_2d = apply_ops(&parse_envelope(&document), &ops_2d);
        assert_eq!(envelope_2d.document.camera2d.x, 5.0);
        let ops_3d = app.handle_action_patch_ops("setCamera", Some(&json!({ "surfaceId": PUZZLE5D_PLAY_SURFACE_3D, "camera": { "position": [1.0, 2.0, 3.0], "target": [0.0, 0.0, 0.0], "zoom": 1.0 } })), &document, &ViewState::default());
        let envelope_3d = apply_ops(&parse_envelope(&document), &ops_3d);
        assert_eq!(envelope_3d.document.camera3d.position, [1.0, 2.0, 3.0]);
    }

    #[test]
    fn set_fixture_json_replaces_document() {
        let mut app = Puzzle5dPlayApp::default();
        let document = app.initial_document_json();
        let replacement = serde_json::to_string(&empty_document()).unwrap();
        let ops = app.handle_action_patch_ops("setFixtureJson", Some(&json!({ "json": replacement })), &document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&document), &ops);
        assert!(envelope.document.parts.is_empty());
    }

    #[test]
    fn add_part_kind_appends_paired_part() {
        let mut app = Puzzle5dPlayApp::default();
        let document = app.initial_document_json();
        let ops = app.handle_action_patch_ops("addPartKind", Some(&json!({ "partKind": "Hexagonal Cut Concrete Forest Right" })), &document, &ViewState::default());
        let envelope = apply_ops(&parse_envelope(&document), &ops);
        let added = envelope.document.parts.iter().find(|part| part.part_kind == "Hexagonal Cut Concrete Forest Right").expect("added part");
        assert!(!added.grips.is_empty());
        assert!(resolve_part_mesh_url(added, envelope.document.kind_catalogs.as_ref()).is_some_and(|url| url.contains("right")));
    }

    #[test]
    fn engine_kind_catalogs_map_grips_to_vortices() {
        let envelope = default_envelope();
        let catalogs = engine_kind_catalogs_value(&envelope.document).expect("engine catalogs");
        let object = &catalogs["objects"][0];
        assert!(object.get("grips").is_none());
        assert!(object["vortices"].as_array().is_some_and(|vortices| !vortices.is_empty()));
        assert!(object["vortices"][0]["vortexKind"].is_string());
    }

    #[test]
    fn puzzle5d_labels_resolve_native_english_by_default() {
        let app = Puzzle5dPlayApp::default();
        let document = app.initial_document_json();
        let mut envelope = parse_envelope(&document);
        envelope.runtime.selection.part_ids = vec![envelope.document.parts[0].id.clone()];
        let selected_document = serde_json::to_string(&envelope).unwrap();

        let document_json = serde_json::to_string(&app.render(PUZZLE5D_PLAY_BODY_DOCUMENT, &document, &ViewState::default())).unwrap();
        assert!(document_json.contains("\"Parts\""));
        assert!(document_json.contains("\"Fasteners\""));

        let kinds_json = serde_json::to_string(&app.render(PUZZLE5D_PLAY_BODY_KINDS, &document, &ViewState::default())).unwrap();
        assert!(kinds_json.contains("\"Grips\""));
        assert!(kinds_json.contains("\"Ropes\""));

        let inspector_json = serde_json::to_string(&app.render(PUZZLE5D_PLAY_BODY_INSPECTOR, &selected_document, &ViewState::default())).unwrap();
        assert!(inspector_json.contains("\"Part\""));

        let engagements = app.window_engagements(&document, &ViewState::default());
        let engagement_json = serde_json::to_string(&engagements.get(PUZZLE5D_PLAY_WINDOW_2D).unwrap()).unwrap();
        assert!(engagement_json.contains("\"Select\""));
        assert!(engagement_json.contains("\"Brush\""));
        assert!(engagement_json.contains("\"Fill\""));

        let measures = app.window_measures(&document, &ViewState::default());
        let measures_json = serde_json::to_string(&measures.get(PUZZLE5D_PLAY_WINDOW_2D).unwrap()).unwrap();
        assert!(measures_json.contains("\"LOD\""));
        assert!(measures_json.contains("\"Automatic\""));
        assert!(measures_json.contains("Suggestion"));
        assert!(measures_json.contains("Offset"));
    }

    #[test]
    fn puzzle5d_labels_resolve_native_german_locale() {
        let app = Puzzle5dPlayApp::default();
        let document = app.initial_document_json();
        let mut envelope = parse_envelope(&document);
        envelope.runtime.selection.part_ids = vec![envelope.document.parts[0].id.clone()];
        let selected_document = serde_json::to_string(&envelope).unwrap();
        let view_state = ViewState { locale: Some("de".into()), ..ViewState::default() };

        let document_json = serde_json::to_string(&app.render(PUZZLE5D_PLAY_BODY_DOCUMENT, &document, &view_state)).unwrap();
        assert!(document_json.contains("\"Teile\""));
        assert!(document_json.contains("\"Verbinder\""));

        let kinds_json = serde_json::to_string(&app.render(PUZZLE5D_PLAY_BODY_KINDS, &document, &view_state)).unwrap();
        assert!(kinds_json.contains("\"Griffe\""));
        assert!(kinds_json.contains("\"Seile\""));

        let inspector_json = serde_json::to_string(&app.render(PUZZLE5D_PLAY_BODY_INSPECTOR, &selected_document, &view_state)).unwrap();
        assert!(inspector_json.contains("\"Teil\""));

        let engagements = app.window_engagements(&document, &view_state);
        let engagement_json = serde_json::to_string(&engagements.get(PUZZLE5D_PLAY_WINDOW_2D).unwrap()).unwrap();
        assert!(engagement_json.contains("\"Auswählen\""));
        assert!(engagement_json.contains("\"Pinsel\""));
        assert!(engagement_json.contains("\"Füllen\""));

        let measures = app.window_measures(&document, &view_state);
        let measures_json = serde_json::to_string(&measures.get(PUZZLE5D_PLAY_WINDOW_2D).unwrap()).unwrap();
        assert!(measures_json.contains("Automatisch"));
        assert!(measures_json.contains("Vorschlag"));
        assert!(measures_json.contains("Versatz"));
    }
}
//#endregion 🧪Tests
