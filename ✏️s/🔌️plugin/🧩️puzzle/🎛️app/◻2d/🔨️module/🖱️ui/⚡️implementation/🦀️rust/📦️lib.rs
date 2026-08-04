//! 🧩️ Puzzle 2d app — `DocumentApp` impl, render, manifest (constitutional: ui).

use puzzle_2d::Puzzle2dProjection;
use puzzle_2d_engine::{puzzle_2d_lod_scale_json, puzzle_board_host, BoardHost, Puzzle2dExtension, BOARD_CAMERA_ZOOM_MAX, BOARD_CAMERA_ZOOM_MIN};
use puzzle_2d_op::{puzzle2d_document_delta_operations, Puzzle2dOperation, Puzzle2dPlayProjection};
use semio_framework_plugin::kernel::HostEffect;
use semio_framework_plugin::{
    build_board2d_scene, create_default_layout, tree_item, tree_item_with_action, ui_inspector_groups_to_tree, ui_inspector_mixed_text, ui_inspector_readonly_field, ui_inspector_stepper_field, ui_stack_vertical, ui_text, ActionArgDef,
    ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, App, AppIo, AppLabels, ArtifactKindSpec, ArtifactPresentation, Board2dScene, ConfigView, DocumentApp, DocumentView, Emit, Label, Locale, LocalizedLabel, MeasureSelectItem, Media,
    MediaClass, MediaError, MediaForm, MediaPortDirection, MediaPortSpec, MediaType, OsMediaCapability, OsMediaFormat, PanelGroup, PanelTreeBuilder, PortMultiplicity, SurfaceKind, Terminology, ToolRef, UiInspectorFieldGroup, UiNode, UiPresence,
    UiTreeItemNode, UiTreeNode, UiTreeSectionNode, UtilityCategory, UtilityDefinition, ViewState, WindowEngagement, WindowEngagementInput, WindowEngagementStatus, WindowMeasure, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
    FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, SET_ACTIVE_UTILITY_ACTION_ID,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::LazyLock;

//#region 🔖️Constants
const PUZZLE2D_PLAY_APP_ID: &str = "puzzle2d-play";
const PUZZLE2D_PLAY_CONTROLLER_ID: &str = "puzzle2d-play";
const PUZZLE2D_PLAY_SURFACE_ID: &str = "puzzle2d.play.composite";
const PUZZLE2D_PLAY_BODY_OVERVIEW: &str = "puzzle2d.play.overview";
const PUZZLE2D_PLAY_BODY_DETAIL: &str = "puzzle2d.play.detail";
const PUZZLE2D_PLAY_BODY_SELECTION: &str = "puzzle2d.play.selection";
const PUZZLE2D_PLAY_BODY_LAYERS: &str = "puzzle2d.play.layers";
const PUZZLE2D_PLAY_BODY_CATALOGUE: &str = "puzzle2d.play.catalogue";
const PUZZLE2D_PLAY_BODY_PROPERTIES: &str = "puzzle2d.play.properties";
const PUZZLE2D_FIXTURE_SCHEMA: &str = "puzzle.2d.fixture";
const PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID: &str = "concrete-forest";
const PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID: &str = "nakagin-capsule-tower";
const CONCRETE_FOREST_EXAMPLE_DSL: &str = puzzle_2d_dsl::PUZZLE2D_CONCRETE_FOREST_EXAMPLE_TEXT;
const NAKAGIN_EXAMPLE_DSL: &str = puzzle_2d_dsl::PUZZLE2D_NAKAGIN_EXAMPLE_TEXT;
/// 🌉️ This app's own `Puzzle2dScene.fixture` (and `DocumentApp::Projection`) stays a bare
/// `serde_json::Value` — see `puzzle_2d`'s `🔖️ValueBridge` region — so the DSL-text example
/// fixtures are parsed once into the typed `Puzzle2dProjection` and re-serialized to the JSON
/// string this module's `serde_json::from_str`/`.example(...)` call sites expect. The typed bridge
/// still carries a mandatory `camera` block (that DSL-derive struct is out of scope here — see
/// `.🦑️repo/🎫️tickets/…/convertpuzzle2d3d5dtotypeddslderiveengine`) — strip it before handing the JSON
/// back, since the play app's own document must never carry a `"camera"` key (see `setCamera`'s
/// `ActionKind::View`): leaving it in would permanently trip `puzzle2d_document_delta_operations`'s
/// known-keys guard on every subsequent action.
fn parse_example_dsl_without_camera(dsl_text: &str, label: &str) -> String {
    let projection = <Puzzle2dProjection as store::DocumentDsl>::parse_dsl(dsl_text).unwrap_or_else(|error| panic!("{label} example fixture parses as dsl: {error}"));
    let mut value = serde_json::to_value(&projection).unwrap_or_else(|error| panic!("serialize {label} example fixture: {error}"));
    if let Some(object) = value.as_object_mut() {
        object.remove("camera");
    }
    serde_json::to_string(&value).unwrap_or_else(|error| panic!("re-serialize {label} example fixture: {error}"))
}
static CONCRETE_FOREST_EXAMPLE_JSON: LazyLock<String> = LazyLock::new(|| parse_example_dsl_without_camera(CONCRETE_FOREST_EXAMPLE_DSL, "concrete-forest"));
static NAKAGIN_EXAMPLE_JSON: LazyLock<String> = LazyLock::new(|| parse_example_dsl_without_camera(NAKAGIN_EXAMPLE_DSL, "nakagin"));
/// 🧰️ The three canvas utilities declared to the framework utility bar (host-owned active utility, never a doc field).
const PUZZLE2D_UTILITY_SELECT: &str = "select";
const PUZZLE2D_UTILITY_BRUSH: &str = "brush";
const PUZZLE2D_UTILITY_FILL: &str = "fill";
const BOARD_DEFAULT_WIDTH: u32 = 1024;
const BOARD_DEFAULT_HEIGHT: u32 = 768;

//#region 🔖️PaneConstants
const PUZZLE2D_PANE_OVERVIEW: &str = "2d-overview";
const PUZZLE2D_PANE_DETAIL: &str = "2d-detail";
const PUZZLE2D_PANE_SELECTION: &str = "2d-selection";
const PUZZLE2D_PANES: [&str; 3] = [PUZZLE2D_PANE_OVERVIEW, PUZZLE2D_PANE_DETAIL, PUZZLE2D_PANE_SELECTION];
const PUZZLE2D_LOD_MODE_AUTOMATIC: &str = "automatic";
const PUZZLE2D_VIEWPORT_REF_SHORT_PX: f64 = 640.0;
const PUZZLE2D_VIEWPORT_MARGIN: f64 = 0.18;
const PUZZLE2D_VIEWPORT_FRAMING_HALF_SPAN_SCALE: f64 = 2.25;
const PUZZLE2D_VIEWPORT_ZOOM_BOOST: f64 = 2.5;
const PUZZLE2D_PANE_ZOOM_SCALE_OVERVIEW: f64 = 0.68;
const PUZZLE2D_PANE_ZOOM_SCALE_DETAIL: f64 = 2.15;
const PUZZLE2D_PANE_ZOOM_SCALE_SELECTION: f64 = 0.36;
//#endregion 🔖️PaneConstants

//#region 🔖️EngagementConstants
const PUZZLE2D_SUGGESTION_OFFSET_MIN: f64 = 0.0;
const PUZZLE2D_SUGGESTION_OFFSET_MAX: f64 = 160.0;
const PUZZLE2D_SUGGESTION_OFFSET_STEP: f64 = 4.0;
const PUZZLE2D_FILL_COUNT_MAX: u32 = 1000;
/// 📶️ Mirrors `ui_styling::metrics::board::SUGGESTION_OFFSET`; kept local since the plugin crate has no styling dependency.
const PUZZLE2D_DEFAULT_SUGGESTION_OFFSET: f64 = 80.0;
//#endregion 🔖️EngagementConstants

static NODE_ID_COUNTER: AtomicU32 = AtomicU32::new(0);
//#endregion 🔖️Constants

//#region 🔖️Envelope
fn default_selection_method() -> String {
    "rectangle".into()
}

fn default_grid_factor() -> f64 {
    1.0
}

fn default_suggestion_offset() -> f64 {
    PUZZLE2D_DEFAULT_SUGGESTION_OFFSET
}

/// 📶️ Overview/selection default to automatic LOD; detail defaults to a fixed "detail" tier, matching the pre-migration triptych.
fn default_lod_mode_by_pane() -> BTreeMap<String, String> {
    BTreeMap::from([(PUZZLE2D_PANE_OVERVIEW.to_string(), PUZZLE2D_LOD_MODE_AUTOMATIC.to_string()), (PUZZLE2D_PANE_DETAIL.to_string(), "detail".to_string()), (PUZZLE2D_PANE_SELECTION.to_string(), PUZZLE2D_LOD_MODE_AUTOMATIC.to_string())])
}

fn default_camera_zoom() -> f64 {
    1.0
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle2dPlayRuntime {
    #[serde(default)]
    selected_ids: Vec<String>,
    /// 🎥️ The canvas camera (pan/zoom) — session-only view state, never a document/fixture field
    /// (see `setCamera`'s `ActionKind::View`): moving the camera must never create a VCS edit.
    #[serde(default)]
    camera_x: f64,
    #[serde(default)]
    camera_y: f64,
    #[serde(default = "default_camera_zoom")]
    camera_zoom: f64,
    #[serde(default = "default_lod_mode_by_pane")]
    lod_mode_by_pane: BTreeMap<String, String>,
    #[serde(default)]
    engagement_input_by_pane: BTreeMap<String, String>,
    #[serde(default)]
    brush_candidate_index: usize,
    #[serde(default)]
    brush_candidates: Vec<Value>,
    #[serde(default)]
    brush_candidate_source_handle_id: String,
    #[serde(default)]
    fill_count: u32,
    #[serde(default = "default_selection_method")]
    selection_method: String,
    #[serde(default)]
    grid_snap_enabled: bool,
    #[serde(default = "default_grid_factor")]
    grid_factor: f64,
    #[serde(default = "default_suggestion_offset")]
    suggestion_offset: f64,
    #[serde(default)]
    node_kind_weights: BTreeMap<String, f64>,
    #[serde(default)]
    handle_kind_weights: BTreeMap<String, f64>,
    /// 🧰️ B1: host-owned active utility per pane — was host-pushed `view_state.active_utility_by_window_id`;
    /// now the app itself persists it (see `SET_ACTIVE_UTILITY_ACTION_ID`'s arm in `handle_action_impl`).
    #[serde(default)]
    active_utility_by_window_id: BTreeMap<String, String>,
    /// 🗣️ B1: BCP-47 locale tag — was host-pushed `view_state.locale` (read via the deleted
    /// `semio_framework_plugin::is_de_locale(&ViewState)`; see the local `is_de_locale` below).
    #[serde(default = "default_locale")]
    locale: String,
    /// 🗣️ B1: terminology id ("native" default, or "reuse") — was host-pushed `view_state.terminology`.
    #[serde(default = "default_terminology")]
    terminology: String,
}

fn default_locale() -> String {
    "en-US".into()
}

fn default_terminology() -> String {
    "native".into()
}

/// ⚠️ Explicit impl (not `#[derive(Default)]`) so Rust construction matches the serde field defaults above.
impl Default for Puzzle2dPlayRuntime {
    fn default() -> Self {
        Self {
            selected_ids: Vec::new(),
            camera_x: 0.0,
            camera_y: 0.0,
            camera_zoom: default_camera_zoom(),
            lod_mode_by_pane: default_lod_mode_by_pane(),
            engagement_input_by_pane: BTreeMap::new(),
            brush_candidate_index: 0,
            brush_candidates: Vec::new(),
            brush_candidate_source_handle_id: String::new(),
            fill_count: 0,
            selection_method: default_selection_method(),
            grid_snap_enabled: false,
            grid_factor: default_grid_factor(),
            suggestion_offset: default_suggestion_offset(),
            node_kind_weights: BTreeMap::new(),
            handle_kind_weights: BTreeMap::new(),
            active_utility_by_window_id: BTreeMap::new(),
            locale: default_locale(),
            terminology: default_terminology(),
        }
    }
}

//#region 🔖️Config
/// 🧮️ B1: puzzle2d's real `DocumentApp::Config` — `Puzzle2dPlayRuntime` itself doubles as the config
/// record (an alias, not a new type), mirroring `puzzle_3d_ui::Puzzle3dConfig = Puzzle3dRuntime`, so
/// every existing helper taking `&Puzzle2dPlayRuntime` throughout this file keeps working unchanged;
/// only `Puzzle2dPlayApp`'s own ambient `RefCell<Puzzle2dPlayRuntime>` field is gone — every read now
/// comes from `cfg.projection`, every write flows out as a `Puzzle2dConfigOperation` in the returned
/// `Emit` instead of a silent `self` mutation.
pub type Puzzle2dConfig = Puzzle2dPlayRuntime;

impl store::DocumentDsl for Puzzle2dPlayRuntime {
    const EXTENSION: &'static str = "puzzle2dcfg";

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(text).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }
}

impl store::DocumentPack for Puzzle2dPlayRuntime {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        dsl::to_dsl_value(self).map_err(store::PackError::Schema)?.encode_pack_with(options)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let value = dsl::DslValue::decode_pack_with(bytes, options)?;
        dsl::from_dsl_value(value).map_err(store::PackError::Schema)
    }
}

store::impl_whole_record_config!(Puzzle2dPlayRuntime);


/// @emoji 🧮️ B1: `Puzzle2dConfig`'s operation enum — lives here (in `ui`, not a lower `op`/`engine`
/// crate) since `Puzzle2dConfig` is a type alias for the ui-crate-local `Puzzle2dPlayRuntime`. Mirrors
/// `puzzle_3d_ui::Puzzle3dConfigOperation`'s single-generic-`Snapshot`-variant pattern exactly: every
/// real config edit is captured as "the whole config after this edit"; `backwards()` restores the
/// whole-config snapshot from just before, regardless of what changed.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Puzzle2dConfigOperation {
    Snapshot { config: Puzzle2dConfig },
}

impl protocol::Operation<Puzzle2dConfig> for Puzzle2dConfigOperation {
    type Diff = Puzzle2dConfig;

    fn diff(&self, _base: &Puzzle2dConfig) -> Puzzle2dConfig {
        match self {
            Puzzle2dConfigOperation::Snapshot { config } => config.clone(),
        }
    }

    fn backwards(&self, base: &Puzzle2dConfig) -> Vec<Self> {
        vec![Puzzle2dConfigOperation::Snapshot { config: base.clone() }]
    }
}

impl protocol::OpBinary for Puzzle2dConfigOperation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        serde_json::to_vec(self).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        serde_json::from_slice(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
    }
}

impl protocol::OpText for Puzzle2dConfigOperation {
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }
}
//#endregion 🔖️Config

/// 🗣️ B1: local replacement for the deleted `semio_framework_plugin::is_de_locale(&ViewState)`.
fn is_de_locale(config: &Puzzle2dConfig) -> bool {
    config.locale.starts_with("de")
}

/// 🧾️ Transient render/mutation bundle pairing the persisted projection (the bare fixture json)
/// with the app's ephemeral view state. It is never persisted — the {@link VcsDocumentApp} store
/// owns the fixture as its projection and {@link Puzzle2dConfig} owns the runtime — but rebuilding
/// it per call lets the panel/canvas/engagement helpers keep their existing `&scene` signatures.
struct Puzzle2dScene {
    fixture: Value,
    runtime: Puzzle2dPlayRuntime,
    /// 🧰️ The host-owned active utility for this render/mutation, sourced from
    /// `Puzzle2dConfig::active_utility_by_window_id` (defaulting to `select`) — never a document field.
    active_utility: String,
}

fn default_empty_fixture() -> Value {
    json!({
        "schema": PUZZLE2D_FIXTURE_SCHEMA,
        "nodes": [],
        "edges": []
    })
}

fn example_fixture(json_text: &str) -> Value {
    serde_json::from_str(json_text).unwrap_or_else(|_| default_empty_fixture())
}

fn puzzle2d_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    semio_framework_plugin::ActionFactory::new(PUZZLE2D_PLAY_CONTROLLER_ID).action(action, args)
}

/// 🪟️ B1: was host-pushed `view_state.window_instances` filtered by `window_kind_id`; puzzle2d has
/// three DISTINCT pane kinds (unlike puzzle3d's split-top/perspective, which are several instances of
/// ONE kind), and — unlike puzzle3d's per-window `window_options` — `Puzzle2dConfig` carries no field
/// that ever differs between two instances of the SAME pane kind, so a self-maintained multi-instance
/// registry (puzzle3d's `window_ids`) would only ever produce byte-identical duplicate entries here.
/// Always exactly one instance, keyed by the pane kind id itself.
fn window_instance_ids(pane: &str) -> Vec<String> {
    vec![pane.to_string()]
}

/// 🧰️ B1: the host-owned active utility for `window_id`'s pane, now real VCS'd config (was host-pushed
/// `view_state.active_utility_by_window_id`/`view_state.active_utility_id`) — see
/// `SET_ACTIVE_UTILITY_ACTION_ID`'s arm in `handle_action_impl`, the only writer.
fn puzzle2d_active_utility(config: &Puzzle2dConfig, window_id: Option<&str>) -> String {
    if let Some(wid) = window_id {
        if let Some(utility) = config.active_utility_by_window_id.get(wid) {
            return utility.clone();
        }
    }
    PUZZLE2D_UTILITY_SELECT.into()
}

/// 🎯️ `semio_framework_plugin::selection_ids`'s "ids" array plus a singular "id" fallback —
/// this app's actions accept either shape depending on the caller.
fn selection_ids(args: Option<&Value>) -> Vec<String> {
    let ids = semio_framework_plugin::selection_ids(args);
    if !ids.is_empty() {
        return ids;
    }
    args.and_then(|value| value.get("id")).and_then(|value| value.as_str()).map(|id| vec![id.to_string()]).unwrap_or_default()
}

/// 🎥️ The camera lives on `Puzzle2dPlayRuntime` — session-only view state, never a fixture field
/// (see `setCamera`'s `ActionKind::View`).
fn runtime_camera(runtime: &Puzzle2dPlayRuntime) -> (f64, f64, f64) {
    (runtime.camera_x, runtime.camera_y, runtime.camera_zoom)
}

fn fixture_nodes(fixture: &Value) -> &[Value] {
    fixture.get("nodes").and_then(|value| value.as_array()).map(|values| values.as_slice()).unwrap_or(&[])
}

fn fixture_edges(fixture: &Value) -> &[Value] {
    fixture.get("edges").and_then(|value| value.as_array()).map(|values| values.as_slice()).unwrap_or(&[])
}

fn kind_catalog_entries<'a>(fixture: &'a Value, key: &str) -> Option<&'a [Value]> {
    fixture.get("meta").and_then(|value| value.get("kindCatalogs")).and_then(|value| value.get(key)).and_then(|value| value.as_array()).map(|values| values.as_slice())
}

fn new_node_id(prefix: &str) -> String {
    let serial = NODE_ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
    format!("{prefix}-{serial}")
}

fn add_node_to_fixture(fixture: &mut Value, kind: Option<&str>, args: Option<&Value>) {
    let Some(obj) = fixture.as_object_mut() else {
        return;
    };
    let nodes = obj.entry("nodes".to_string()).or_insert_with(|| json!([]));
    let Some(nodes) = nodes.as_array_mut() else {
        return;
    };
    let node_kind = kind.unwrap_or("node");
    let id = new_node_id("node");
    let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(0.0);
    let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(0.0);
    let shape = args.and_then(|value| value.get("shape")).and_then(|value| value.as_str()).unwrap_or("circle");
    let mut node = json!({
        "id": id,
        "nodeKind": node_kind,
        "shape": shape,
        "x": x,
        "y": y,
        "text": id,
        "handles": []
    });
    if shape == "rectangle" {
        node["width"] = json!(args.and_then(|value| value.get("width")).and_then(|value| value.as_f64()).unwrap_or(48.0));
        node["height"] = json!(args.and_then(|value| value.get("height")).and_then(|value| value.as_f64()).unwrap_or(48.0));
    } else {
        node["radius"] = json!(args.and_then(|value| value.get("radius")).and_then(|value| value.as_f64()).unwrap_or(24.0));
    }
    if let Some(icon_kind) = args.and_then(|value| value.get("iconKind")) {
        node["iconKind"] = icon_kind.clone();
    }
    nodes.push(node);
}

fn delete_selection_from_fixture(fixture: &mut Value, selected: &[String]) {
    if selected.is_empty() {
        return;
    }
    let selected: HashSet<&str> = selected.iter().map(String::as_str).collect();
    let node_ids: HashSet<String> = fixture_nodes(fixture).iter().filter_map(|node| node.get("id").and_then(|value| value.as_str())).filter(|id| selected.contains(id)).map(str::to_string).collect();
    let handle_ids: HashSet<String> = fixture_nodes(fixture)
        .iter()
        .flat_map(|node| node.get("handles").and_then(|value| value.as_array()).into_iter().flatten().filter_map(|handle| handle.get("id").and_then(|value| value.as_str())))
        .filter(|id| selected.contains(id))
        .map(str::to_string)
        .collect();
    if let Some(nodes) = fixture.get_mut("nodes").and_then(|value| value.as_array_mut()) {
        *nodes = nodes
            .iter()
            .filter(|node| node.get("id").and_then(|value| value.as_str()).is_none_or(|id| !node_ids.contains(id)))
            .map(|node| {
                let mut next = node.clone();
                if let Some(handles) = next.get_mut("handles").and_then(|value| value.as_array_mut()) {
                    handles.retain(|handle| handle.get("id").and_then(|value| value.as_str()).is_none_or(|id| !handle_ids.contains(id)));
                }
                next
            })
            .collect();
    }
    if let Some(edges) = fixture.get_mut("edges").and_then(|value| value.as_array_mut()) {
        edges.retain(|edge| {
            let id_ok = edge.get("id").and_then(|value| value.as_str()).is_none_or(|id| !selected.contains(id));
            let source = edge.get("source").and_then(|value| value.as_str()).unwrap_or("");
            let target = edge.get("target").and_then(|value| value.as_str()).unwrap_or("");
            id_ok && !node_ids.contains(source) && !node_ids.contains(target) && !handle_ids.contains(source) && !handle_ids.contains(target)
        });
    }
}

/// 🙈️ Patches `hidden`/`locked` onto every selected node, handle, and edge in the fixture.
fn apply_selection_flag(fixture: &mut Value, selected: &[String], flag: &str, value: bool) {
    if selected.is_empty() {
        return;
    }
    let selected: HashSet<&str> = selected.iter().map(String::as_str).collect();
    let key = if flag == "locked" { "locked" } else { "hidden" };
    if let Some(nodes) = fixture.get_mut("nodes").and_then(|entry| entry.as_array_mut()) {
        for node in nodes.iter_mut() {
            let node_selected = node.get("id").and_then(|entry| entry.as_str()).is_some_and(|id| selected.contains(id));
            if let Some(handles) = node.get_mut("handles").and_then(|entry| entry.as_array_mut()) {
                for handle in handles.iter_mut() {
                    let handle_selected = handle.get("id").and_then(|entry| entry.as_str()).is_some_and(|id| selected.contains(id));
                    if handle_selected {
                        if let Some(obj) = handle.as_object_mut() {
                            obj.insert(key.to_string(), json!(value));
                        }
                    }
                }
            }
            if node_selected {
                if let Some(obj) = node.as_object_mut() {
                    obj.insert(key.to_string(), json!(value));
                }
            }
        }
    }
    if let Some(edges) = fixture.get_mut("edges").and_then(|entry| entry.as_array_mut()) {
        for edge in edges.iter_mut() {
            let edge_selected = edge.get("id").and_then(|entry| entry.as_str()).is_some_and(|id| selected.contains(id));
            if edge_selected {
                if let Some(obj) = edge.as_object_mut() {
                    obj.insert(key.to_string(), json!(value));
                }
            }
        }
    }
}

/// 📋️ Clones every selected node (+24/+24 offset, fresh node+handle ids) and any edge whose both endpoints were cloned; returns the new node ids.
fn duplicate_selection_in_fixture(fixture: &mut Value, selected: &[String]) -> Vec<String> {
    if selected.is_empty() {
        return Vec::new();
    }
    let selected_set: HashSet<&str> = selected.iter().map(String::as_str).collect();
    let mut id_remap: HashMap<String, String> = HashMap::new();
    let mut new_ids: Vec<String> = Vec::new();

    let source_nodes: Vec<Value> = fixture_nodes(fixture).iter().filter(|node| node.get("id").and_then(|value| value.as_str()).is_some_and(|id| selected_set.contains(id))).cloned().collect();

    let new_nodes: Vec<Value> = source_nodes
        .into_iter()
        .map(|mut node| {
            let old_id = node.get("id").and_then(|value| value.as_str()).unwrap_or_default().to_string();
            let new_id = new_node_id("node");
            id_remap.insert(old_id, new_id.clone());
            if let Some(obj) = node.as_object_mut() {
                obj.insert("id".into(), json!(new_id));
                if let Some(x) = obj.get("x").and_then(|value| value.as_f64()) {
                    obj.insert("x".into(), json!(x + 24.0));
                }
                if let Some(y) = obj.get("y").and_then(|value| value.as_f64()) {
                    obj.insert("y".into(), json!(y + 24.0));
                }
                if let Some(handles) = obj.get_mut("handles").and_then(|value| value.as_array_mut()) {
                    for handle in handles.iter_mut() {
                        let old_handle_id = handle.get("id").and_then(|value| value.as_str()).unwrap_or_default().to_string();
                        let suffix = old_handle_id.rsplit(':').next().unwrap_or(old_handle_id.as_str());
                        let new_handle_id = format!("{new_id}:{suffix}");
                        id_remap.insert(old_handle_id, new_handle_id.clone());
                        if let Some(hobj) = handle.as_object_mut() {
                            hobj.insert("id".into(), json!(new_handle_id));
                        }
                    }
                }
            }
            new_ids.push(new_id);
            node
        })
        .collect();

    if let Some(nodes) = fixture.get_mut("nodes").and_then(|value| value.as_array_mut()) {
        nodes.extend(new_nodes);
    }

    let new_edges: Vec<Value> = fixture_edges(fixture)
        .iter()
        .filter_map(|edge| {
            let source = edge.get("source").and_then(|value| value.as_str()).unwrap_or("");
            let target = edge.get("target").and_then(|value| value.as_str()).unwrap_or("");
            let (new_source, new_target) = (id_remap.get(source)?, id_remap.get(target)?);
            let mut clone = edge.clone();
            if let Some(obj) = clone.as_object_mut() {
                obj.insert("id".into(), json!(new_node_id("edge")));
                obj.insert("source".into(), json!(new_source));
                obj.insert("target".into(), json!(new_target));
            }
            Some(clone)
        })
        .collect();
    if !new_edges.is_empty() {
        if let Some(edges) = fixture.get_mut("edges").and_then(|value| value.as_array_mut()) {
            edges.extend(new_edges);
        }
    }

    new_ids
}

/// 🎯️ Every node/handle id sharing a `nodeKind`/`handleKind` with anything currently selected.
fn select_same_kind_ids(fixture: &Value, selected: &[String]) -> Vec<String> {
    let selected_set: HashSet<&str> = selected.iter().map(String::as_str).collect();
    let mut node_kinds: HashSet<&str> = HashSet::new();
    let mut handle_kinds: HashSet<&str> = HashSet::new();
    for node in fixture_nodes(fixture) {
        if node.get("id").and_then(|value| value.as_str()).is_some_and(|id| selected_set.contains(id)) {
            if let Some(kind) = node.get("nodeKind").and_then(|value| value.as_str()) {
                node_kinds.insert(kind);
            }
        }
        for handle in node.get("handles").and_then(|value| value.as_array()).into_iter().flatten() {
            if handle.get("id").and_then(|value| value.as_str()).is_some_and(|id| selected_set.contains(id)) {
                if let Some(kind) = handle.get("handleKind").and_then(|value| value.as_str()) {
                    handle_kinds.insert(kind);
                }
            }
        }
    }
    let mut ids: Vec<String> = Vec::new();
    for node in fixture_nodes(fixture) {
        if node.get("nodeKind").and_then(|value| value.as_str()).is_some_and(|kind| node_kinds.contains(kind)) {
            if let Some(id) = node.get("id").and_then(|value| value.as_str()) {
                ids.push(id.to_string());
            }
        }
        for handle in node.get("handles").and_then(|value| value.as_array()).into_iter().flatten() {
            if handle.get("handleKind").and_then(|value| value.as_str()).is_some_and(|kind| handle_kinds.contains(kind)) {
                if let Some(id) = handle.get("id").and_then(|value| value.as_str()) {
                    ids.push(id.to_string());
                }
            }
        }
    }
    ids
}

/// 🎥️ Writes an `{ x, y, zoom }` camera payload into `Puzzle2dPlayRuntime` — session-only view
/// state, never the fixture (see `setCamera`'s `ActionKind::View`).
fn set_runtime_camera(runtime: &mut Puzzle2dPlayRuntime, camera: &Value) {
    if let Some(x) = camera.get("x").and_then(Value::as_f64) {
        runtime.camera_x = x;
    }
    if let Some(y) = camera.get("y").and_then(Value::as_f64) {
        runtime.camera_y = y;
    }
    if let Some(zoom) = camera.get("zoom").and_then(Value::as_f64) {
        runtime.camera_zoom = zoom;
    }
}

fn puzzle_extension_id() -> &'static str {
    let _extension = Puzzle2dExtension;
    "puzzle.2d"
}
//#endregion 🔖️Envelope

//#region 🔖️BoardHost
/// 🧱️ The expensive half of syncing `host` from `envelope`: a full `clear_scene()` + rebuild of
/// every node/handle/edge plus the kind-catalog/kind-compat re-push. Only needed when
/// `envelope.fixture` content actually changed — gated by `last_synced_fixture` in `handle_action`.
fn sync_host_fixture_content(host: &mut BoardHost, envelope: &Puzzle2dScene) {
    let _ = host.parse_fixture_v1(&envelope.fixture);
    if let Some(catalogs) = envelope.fixture.get("meta").and_then(|value| value.get("kindCatalogs")) {
        if let Ok(json) = serde_json::to_string(catalogs) {
            let _ = host.set_board_kind_catalogs_from_json(&json);
        }
    }
    if let Some(compat) = envelope.fixture.get("meta").and_then(|value| value.get("kindCompatibility")).or_else(|| envelope.fixture.get("kindCompatibility")) {
        if let Ok(json) = serde_json::to_string(compat) {
            let _ = host.set_handle_link_compat_from_json(&json);
        }
    }
}

/// 🪶️ The cheap half of syncing `host` from `envelope`: plain setters mirroring ephemeral runtime
/// view state (selection/utility/grid/LOD/…) — must run on every action regardless of whether the
/// fixture content changed, since this state itself changes every action.
fn sync_host_runtime_state(host: &mut BoardHost, envelope: &Puzzle2dScene) {
    host.set_size(BOARD_DEFAULT_WIDTH, BOARD_DEFAULT_HEIGHT, 1.0);
    host.set_selection_ids(&envelope.runtime.selected_ids);
    host.set_active_utility(&envelope.active_utility);
    let overview_lod_mode = envelope.runtime.lod_mode_by_pane.get(PUZZLE2D_PANE_OVERVIEW).map(String::as_str).unwrap_or(PUZZLE2D_LOD_MODE_AUTOMATIC);
    if overview_lod_mode == PUZZLE2D_LOD_MODE_AUTOMATIC {
        host.set_automatic_lod(true);
    } else {
        host.set_automatic_lod(false);
        host.set_forced_draw_lod_label(overview_lod_mode);
    }
    host.set_grid_snap_enabled(envelope.runtime.grid_snap_enabled);
    let _ = host.set_grid_factor(envelope.runtime.grid_factor);
    host.set_suggestion_offset(envelope.runtime.suggestion_offset);
    if let Ok(weights_json) = serde_json::to_string(&json!({
        "nodeWeights": envelope.runtime.node_kind_weights,
        "handleWeights": envelope.runtime.handle_kind_weights,
    })) {
        host.set_brush_kind_weights(&weights_json);
    }
    host.set_selection_options(&envelope.runtime.selection_method, "replace", true, true, true);
}

fn sync_host_from_envelope(host: &mut BoardHost, envelope: &Puzzle2dScene) {
    sync_host_fixture_content(host, envelope);
    sync_host_runtime_state(host, envelope);
}

fn apply_board_events_from_json(events_json: &str, envelope: &mut Puzzle2dScene) {
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
                set_runtime_camera(&mut envelope.runtime, &payload);
            }
            "select" => {
                if let Some(ids) = payload.get("ids").and_then(|value| serde_json::from_value(value.clone()).ok()) {
                    envelope.runtime.selected_ids = ids;
                }
            }
            "nodeDragEnd" => {
                if let Some(moves) = payload.get("moves").and_then(|value| value.as_array()) {
                    for entry in moves {
                        let Some(id) = entry.get("id").and_then(|value| value.as_str()) else {
                            continue;
                        };
                        if let Some(x) = entry.get("x").and_then(|value| value.as_f64()) {
                            patch_inspector_nodes(&mut envelope.fixture, &[id.to_string()], "x", Some(&json!(x)), None);
                        }
                        if let Some(y) = entry.get("y").and_then(|value| value.as_f64()) {
                            patch_inspector_nodes(&mut envelope.fixture, &[id.to_string()], "y", Some(&json!(y)), None);
                        }
                    }
                }
            }
            "nodeMove" => {
                let Some(id) = payload.get("id").and_then(|value| value.as_str()) else {
                    continue;
                };
                if let Some(x) = payload.get("x").and_then(|value| value.as_f64()) {
                    patch_inspector_nodes(&mut envelope.fixture, &[id.to_string()], "x", Some(&json!(x)), None);
                }
                if let Some(y) = payload.get("y").and_then(|value| value.as_f64()) {
                    patch_inspector_nodes(&mut envelope.fixture, &[id.to_string()], "y", Some(&json!(y)), None);
                }
            }
            "brushPlace" => {
                apply_brush_place_payload(&mut envelope.fixture, &payload);
            }
            "edgeCreate" => {
                if let Some(edges) = envelope.fixture.get_mut("edges").and_then(|value| value.as_array_mut()) {
                    edges.push(payload);
                }
            }
            "nodeDelete" => {
                if let Some(id) = payload.get("id").and_then(|value| value.as_str()) {
                    envelope.runtime.selected_ids = vec![id.to_string()];
                    delete_selection_from_fixture(&mut envelope.fixture, &envelope.runtime.selected_ids);
                    envelope.runtime.selected_ids.clear();
                }
            }
            "edgeDelete" => {
                if let Some(id) = payload.get("id").and_then(|value| value.as_str()) {
                    if let Some(edges) = envelope.fixture.get_mut("edges").and_then(|value| value.as_array_mut()) {
                        edges.retain(|edge| edge.get("id").and_then(|value| value.as_str()) != Some(id));
                    }
                }
            }
            "brushCandidates" => {
                if let Some(candidates) = payload.get("candidates").and_then(|value| value.as_array()) {
                    envelope.runtime.brush_candidates = candidates.clone();
                }
                if let Some(source) = payload.get("sourceHandleId").and_then(|value| value.as_str()) {
                    envelope.runtime.brush_candidate_source_handle_id = source.to_string();
                }
                if let Some(index) = payload.get("index").and_then(|value| value.as_u64()) {
                    envelope.runtime.brush_candidate_index = index as usize;
                }
            }
            _ => {}
        }
    }
}

/// 🐢️ `UiDirtyScope.windowBodies`/`.panelBodies` are matched against `AppDefinition.windowKinds[].bodyKey`
/// on the shell side (`buildUiRefreshRequest`'s `uiRefreshWantsWindow`), so these must be the body-key
/// constants (`puzzle2d.play.overview`, …) — *not* the pane/kind-id constants (`PUZZLE2D_PANES`,
/// `2d-overview`, …), which are a different id space used to key utilities/engagements/measures.
const PUZZLE2D_WINDOW_BODY_KEYS: [&str; 3] = [PUZZLE2D_PLAY_BODY_OVERVIEW, PUZZLE2D_PLAY_BODY_DETAIL, PUZZLE2D_PLAY_BODY_SELECTION];

/// 🐢️ Classifies a batch of board events into the narrowest `UiDirtyScope` that covers all of them —
/// `applyBoardEvents` fires on every select/drag/zoom, so getting this right is most of the
/// perf-round-3 win. Unrecognized/empty event batches fall back to `Full` (safe default).
fn puzzle2d_board_events_scope(events: &[Value]) -> semio_framework_core::kernel::UiDirtyScope {
    use semio_framework_core::kernel::UiDirtyScope;
    if events.is_empty() {
        return UiDirtyScope::None;
    }
    let panes: Vec<String> = PUZZLE2D_WINDOW_BODY_KEYS.iter().map(|body_key| body_key.to_string()).collect();
    let mut window_bodies = false;
    let mut panel_layers = false;
    let mut panel_properties = false;
    let mut engagements = false;
    let mut measures = false;
    let mut recognized_all = true;
    for event in events {
        let Some(name) = event.get("name").and_then(|value| value.as_str()) else {
            recognized_all = false;
            continue;
        };
        match name {
            "camera" => {
                window_bodies = true;
            }
            "select" => {
                window_bodies = true;
                panel_layers = true;
                panel_properties = true;
                engagements = true;
            }
            "nodeMove" | "nodeDragEnd" => {
                window_bodies = true;
                panel_properties = true;
            }
            "brushPlace" | "edgeCreate" | "edgeDelete" | "nodeDelete" => {
                window_bodies = true;
                panel_layers = true;
                panel_properties = true;
                engagements = true;
                measures = true;
            }
            "brushCandidates" => {
                window_bodies = true;
                engagements = true;
            }
            _ => recognized_all = false,
        }
    }
    if !recognized_all {
        return UiDirtyScope::Full;
    }
    let mut panel_bodies = Vec::new();
    if panel_layers {
        panel_bodies.push(PUZZLE2D_PLAY_BODY_LAYERS.to_string());
    }
    if panel_properties {
        panel_bodies.push(PUZZLE2D_PLAY_BODY_PROPERTIES.to_string());
    }
    UiDirtyScope::Partial { window_bodies: if window_bodies { panes } else { Vec::new() }, panel_bodies, utilities: false, tools: false, engagements, measures, labels: false }
}

/// 🐢️ Narrow `UiDirtyScope` shared by pure view/selection/camera actions that only touch the 3
/// canvas panes (never a panel or engagement/measure/utility refresh).
fn puzzle2d_window_only_scope() -> semio_framework_core::kernel::UiDirtyScope {
    semio_framework_core::kernel::UiDirtyScope::Partial {
        window_bodies: PUZZLE2D_WINDOW_BODY_KEYS.iter().map(|body_key| body_key.to_string()).collect(),
        panel_bodies: Vec::new(),
        utilities: false,
        tools: false,
        engagements: false,
        measures: false,
        labels: false,
    }
}

/// 🐢️ Narrow `UiDirtyScope` for actions that additionally change the engagement bar (active utility,
/// brush weights, LOD/grid settings, engagement text input) but never touch document content.
fn puzzle2d_window_and_engagements_scope() -> semio_framework_core::kernel::UiDirtyScope {
    semio_framework_core::kernel::UiDirtyScope::Partial {
        window_bodies: PUZZLE2D_WINDOW_BODY_KEYS.iter().map(|body_key| body_key.to_string()).collect(),
        panel_bodies: Vec::new(),
        utilities: false,
        tools: false,
        engagements: true,
        measures: false,
        labels: false,
    }
}

/// 🐢️ Narrow `UiDirtyScope` for settings surfaced in the measures sidebar (LOD mode, grid, brush
/// weights, suggestion offset — see `puzzle2d_window_measures`) but that never touch document
/// content or the engagement bar.
fn puzzle2d_window_and_measures_scope() -> semio_framework_core::kernel::UiDirtyScope {
    semio_framework_core::kernel::UiDirtyScope::Partial {
        window_bodies: PUZZLE2D_WINDOW_BODY_KEYS.iter().map(|body_key| body_key.to_string()).collect(),
        panel_bodies: Vec::new(),
        utilities: false,
        tools: false,
        engagements: false,
        measures: true,
        labels: false,
    }
}

/// 🐢️ Narrow `UiDirtyScope` for a runtime-only selection change: the 3 canvas panes plus the
/// layers/properties panels (which highlight the selection) and the engagement bar.
fn puzzle2d_select_scope() -> semio_framework_core::kernel::UiDirtyScope {
    semio_framework_core::kernel::UiDirtyScope::Partial {
        window_bodies: PUZZLE2D_WINDOW_BODY_KEYS.iter().map(|body_key| body_key.to_string()).collect(),
        panel_bodies: vec![PUZZLE2D_PLAY_BODY_LAYERS.to_string(), PUZZLE2D_PLAY_BODY_PROPERTIES.to_string()],
        utilities: false,
        tools: false,
        engagements: true,
        measures: false,
        labels: false,
    }
}

/// 🪞️ Re-syncs `envelope.runtime.selected_ids` from `self.host` for engine-driven selection changes
/// (e.g. `delete_selection`, brush commit). Camera is deliberately NOT mirrored here: every action
/// that moves the camera (`setCamera`, `focusSelection`, the `camera` board event) already writes
/// `envelope.runtime`'s camera fields directly — re-deriving it from `host.camera` here used to
/// blindly overwrite that write with the *pre-action* host camera (since nothing had told
/// `self.host` about the new value yet), silently reverting every plain `camera` echo from the client.
fn apply_host_events(host: &mut BoardHost, envelope: &mut Puzzle2dScene) {
    let events_raw = host.drain_events_json();
    apply_board_events_from_json(&events_raw, envelope);
    envelope.runtime.selected_ids = host.selection.iter().cloned().collect();
}

/// 🎲️ Re-mints a node id when it collides with an existing one — client-side brush serials restart every session.
fn unique_node_id(fixture: &Value, candidate: String) -> String {
    if fixture_nodes(fixture).iter().any(|node| node.get("id").and_then(|value| value.as_str()) == Some(candidate.as_str())) {
        new_node_id("node")
    } else {
        candidate
    }
}

fn unique_edge_id(fixture: &Value, candidate: String) -> String {
    if fixture_edges(fixture).iter().any(|edge| edge.get("id").and_then(|value| value.as_str()) == Some(candidate.as_str())) {
        new_node_id("edge")
    } else {
        candidate
    }
}

fn apply_brush_place_payload(fixture: &mut Value, payload: &Value) {
    let node_id = unique_node_id(fixture, payload.get("nodeId").and_then(|value| value.as_str()).map(str::to_string).unwrap_or_else(|| new_node_id("node")));
    let edge_id = unique_edge_id(fixture, payload.get("edgeId").and_then(|value| value.as_str()).map(str::to_string).unwrap_or_else(|| new_node_id("edge")));
    let node_kind = payload.get("nodeKind").and_then(|value| value.as_str()).unwrap_or("node");
    let x = payload.get("x").and_then(|value| value.as_f64()).unwrap_or(0.0);
    let y = payload.get("y").and_then(|value| value.as_f64()).unwrap_or(0.0);
    let shape = payload.get("shape").and_then(|value| value.as_str()).unwrap_or("circle");
    let mut node = json!({
        "id": node_id,
        "nodeKind": node_kind,
        "shape": shape,
        "x": x,
        "y": y,
        "text": node_kind,
        "handles": payload.get("handles").cloned().unwrap_or_else(|| json!([])),
    });
    if shape == "rectangle" {
        node["width"] = json!(payload.get("width").and_then(|value| value.as_f64()).unwrap_or(48.0));
        node["height"] = json!(payload.get("height").and_then(|value| value.as_f64()).unwrap_or(48.0));
    } else {
        node["radius"] = json!(payload.get("radius").and_then(|value| value.as_f64()).unwrap_or(24.0));
    }
    if let Some(icon) = payload.get("iconKind") {
        node["iconKind"] = icon.clone();
    }
    if let Some(nodes) = fixture.get_mut("nodes").and_then(|value| value.as_array_mut()) {
        nodes.push(node);
    }
    let source = payload.get("sourceHandleId").and_then(|value| value.as_str()).unwrap_or("");
    if !source.is_empty() {
        if let Some(edges) = fixture.get_mut("edges").and_then(|value| value.as_array_mut()) {
            edges.push(json!({
                "id": edge_id,
                "edgeKind": "link",
                "source": source,
                "target": format!("{node_id}:v{}", payload.get("targetHandleIndex").and_then(|value| value.as_u64()).unwrap_or(0)),
            }));
        }
    }
}

/// 🖌️ Utility Options group for the brush utility — suggestion offset, per-kind distribution trees,
/// and (when candidates exist) the placement picker. Tagged `active_utility_id: Some("brush")`.
fn puzzle2d_brush_utility_options(envelope: &Puzzle2dScene, labels: &Puzzle2dLabels) -> WindowMeasure {
    let node_ids = puzzle2d_kind_ids(&envelope.fixture, "nodes");
    let handle_ids = puzzle2d_kind_ids(&envelope.fixture, "handles");
    let mut children = vec![
        WindowMeasure::Slider {
            id: format!("{PUZZLE2D_PLAY_CONTROLLER_ID}-suggestion-offset"),
            label: Some(format!("{} ({})", labels.suggestion.as_str(), labels.offset.as_str())),
            value: envelope.runtime.suggestion_offset,
            min: PUZZLE2D_SUGGESTION_OFFSET_MIN,
            max: PUZZLE2D_SUGGESTION_OFFSET_MAX,
            step: Some(PUZZLE2D_SUGGESTION_OFFSET_STEP),
            ready: None,
            loading: None,
            waiting: None,
            disabled: None,
            reveal: None,
            on_change: puzzle2d_action("setSuggestionOffset", None),
        },
        WindowMeasure::Group {
            id: format!("{PUZZLE2D_PLAY_CONTROLLER_ID}-suggestion-distribution-nodes"),
            label: format!("{} ({:.0}%)", labels.node_weights.as_str(), puzzle2d_kind_weight_sum(&envelope.runtime.node_kind_weights, &node_ids) * 100.0),
            default_open: Some(false),
            active_utility_id: None,
            value: None,
            min: None,
            max: None,
            step: None,
            ready: None,
            loading: None,
            waiting: None,
            on_change: None,
            children: puzzle2d_kind_weight_measures("node-kind", &node_ids, &envelope.runtime.node_kind_weights, "nodes"),
        },
        WindowMeasure::Group {
            id: format!("{PUZZLE2D_PLAY_CONTROLLER_ID}-suggestion-distribution-handles"),
            label: format!("{} ({:.0}%)", labels.handle_weights.as_str(), puzzle2d_kind_weight_sum(&envelope.runtime.handle_kind_weights, &handle_ids) * 100.0),
            default_open: Some(false),
            active_utility_id: None,
            value: None,
            min: None,
            max: None,
            step: None,
            ready: None,
            loading: None,
            waiting: None,
            on_change: None,
            children: puzzle2d_kind_weight_measures("handle-kind", &handle_ids, &envelope.runtime.handle_kind_weights, "handles"),
        },
    ];
    if !envelope.runtime.brush_candidates.is_empty() {
        let items: Vec<MeasureSelectItem> = envelope
            .runtime
            .brush_candidates
            .iter()
            .enumerate()
            .map(|(index, candidate)| {
                let node_kind = candidate.get("nodeKind").and_then(|value| value.as_str()).or_else(|| candidate.as_str()).unwrap_or("kind");
                let id = format!("puzzle2d.brush.candidate.{index}");
                MeasureSelectItem { id: id.clone(), value: id, label: node_kind.into() }
            })
            .collect();
        let selected_index = envelope.runtime.brush_candidate_index.min(items.len().saturating_sub(1));
        children.push(WindowMeasure::Select {
            id: "puzzle2d-brush-placement".into(),
            label: Some(labels.placement.into()),
            value: format!("puzzle2d.brush.candidate.{selected_index}"),
            items,
            on_change: puzzle2d_action("engagementControlSelect", None),
        });
    }
    WindowMeasure::Group {
        id: "puzzle2d-utility-options-brush".into(),
        label: labels.brush.into(),
        default_open: Some(true),
        active_utility_id: Some(PUZZLE2D_UTILITY_BRUSH.into()),
        children,
        value: None,
        min: None,
        max: None,
        step: None,
        ready: None,
        loading: None,
        waiting: None,
        on_change: None,
    }
}

/// 🛠️ Fill tool options — the fill-count slider, surfaced in the mode-level tool panel while the
/// fill tool is active (not a window utility-options group; fill is a whole-document generator).
fn puzzle2d_fill_tool_measures(envelope: &Puzzle2dScene, labels: &Puzzle2dLabels) -> WindowMeasure {
    WindowMeasure::Group {
        id: "puzzle2d-tool-options-fill".into(),
        label: labels.fill.into(),
        default_open: Some(true),
        active_utility_id: None,
        value: None,
        min: None,
        max: None,
        step: None,
        ready: None,
        loading: None,
        waiting: None,
        on_change: None,
        children: vec![WindowMeasure::Slider {
            id: "puzzle2d-fill-count".into(),
            label: Some(labels.count.into()),
            value: envelope.runtime.fill_count as f64,
            min: 0.0,
            max: PUZZLE2D_FILL_COUNT_MAX as f64,
            step: Some(1.0),
            ready: None,
            loading: None,
            waiting: None,
            disabled: None,
            reveal: None,
            on_change: puzzle2d_action("setFillCount", None),
        }],
    }
}

fn puzzle2d_engagement(envelope: &Puzzle2dScene, host: &BoardHost, pane: &str, labels: &Puzzle2dLabels) -> WindowEngagement {
    let overlay: Value = serde_json::from_str(&host.overlay_paint_state_json()).unwrap_or(Value::Null);
    let pane_lod_mode = envelope.runtime.lod_mode_by_pane.get(pane).map(String::as_str).unwrap_or(PUZZLE2D_LOD_MODE_AUTOMATIC);
    let lod = overlay.get("lod").and_then(|value| value.as_str()).unwrap_or(if pane_lod_mode == PUZZLE2D_LOD_MODE_AUTOMATIC { "auto" } else { pane_lod_mode });
    let node_count = fixture_nodes(&envelope.fixture).len();
    let edge_count = fixture_edges(&envelope.fixture).len();
    let input_value = envelope.runtime.engagement_input_by_pane.get(pane).cloned().unwrap_or_default();
    let placeholder = match envelope.active_utility.as_str() {
        "brush" => "Brush",
        _ => "select, brush, clear",
    };
    WindowEngagement {
        session_active: Some(envelope.active_utility != "select"),
        input: Some(WindowEngagementInput {
            id: Some("puzzle2d-engagement".into()),
            value: Some(input_value),
            placeholder: Some(placeholder.into()),
            disabled: None,
            on_change: Some(puzzle2d_action("engagementInput", Some(json!({ "pane": pane })))),
            on_submit: Some(puzzle2d_action("engagementSubmit", Some(json!({ "pane": pane })))),
            on_repeat_last: None,
            on_abort: Some(puzzle2d_action("engagementAbort", Some(json!({ "pane": pane })))),
        }),
        control: None,
        controls: None,
        status: Some(vec![WindowEngagementStatus { id: "puzzle2d-board-status".into(), text: format!("{node_count} {} · {edge_count} {} · {} {lod}", labels.nodes.as_str(), labels.edges.as_str(), labels.lod.as_str()) }]),
        // 🧰️ The select/brush/fill switcher now lives in the framework utility bar (declared via `.utility` +
        // `.window_kind_utilities`), so the engagement no longer duplicates it as toggle options.
        options: None,
        possible_engagements: None,
    }
}
//#endregion 🔖️BoardHost

//#region 🔖️Canvas
//#region 🔖️PaneCamera
fn puzzle2d_pane_zoom_scale(pane: &str) -> f64 {
    match pane {
        PUZZLE2D_PANE_DETAIL => PUZZLE2D_PANE_ZOOM_SCALE_DETAIL,
        PUZZLE2D_PANE_SELECTION => PUZZLE2D_PANE_ZOOM_SCALE_SELECTION,
        _ => PUZZLE2D_PANE_ZOOM_SCALE_OVERVIEW,
    }
}

fn puzzle2d_clamp_zoom(value: f64) -> f64 {
    value.clamp(BOARD_CAMERA_ZOOM_MIN, BOARD_CAMERA_ZOOM_MAX)
}

/// 📐️ World-space center and half-span of every node's extent (circle radius or rectangle half-size), used to frame pane cameras.
fn puzzle2d_fixture_world_bounds(fixture: &Value) -> (f64, f64, f64) {
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    for node in fixture_nodes(fixture) {
        let (Some(x), Some(y)) = (node.get("x").and_then(|value| value.as_f64()), node.get("y").and_then(|value| value.as_f64())) else {
            continue;
        };
        let (half_w, half_h) = if node.get("shape").and_then(|value| value.as_str()) == Some("rectangle") {
            (node.get("width").and_then(|value| value.as_f64()).unwrap_or(48.0) * 0.5, node.get("height").and_then(|value| value.as_f64()).unwrap_or(48.0) * 0.5)
        } else {
            let radius = node.get("radius").and_then(|value| value.as_f64()).unwrap_or(24.0);
            (radius, radius)
        };
        min_x = min_x.min(x - half_w);
        max_x = max_x.max(x + half_w);
        min_y = min_y.min(y - half_h);
        max_y = max_y.max(y + half_h);
    }
    if !min_x.is_finite() {
        return (0.0, 0.0, 400.0);
    }
    let half_span = (max_x - min_x).max(max_y - min_y).max(1.0) * 0.5;
    ((min_x + max_x) * 0.5, (min_y + max_y) * 0.5, half_span)
}

/// 📷️ Triptych camera for a pane: overview is zoomed out and centered on the fixture, detail zooms into the last-placed node, selection frames a lower-left quadrant — mirrors the pre-migration `puzzle2dPlayTriptychCameraForPane`.
fn puzzle2d_pane_camera(fixture: &Value, runtime: &Puzzle2dPlayRuntime, pane: &str) -> (f64, f64, f64) {
    let (camera_x, camera_y, camera_zoom) = runtime_camera(runtime);
    if pane == PUZZLE2D_PANE_OVERVIEW {
        return (camera_x, camera_y, puzzle2d_clamp_zoom(camera_zoom));
    }
    let (cx, cy, half_span) = puzzle2d_fixture_world_bounds(fixture);
    let usable = PUZZLE2D_VIEWPORT_REF_SHORT_PX * (1.0 - 2.0 * PUZZLE2D_VIEWPORT_MARGIN);
    let world_span = (2.0 * half_span * PUZZLE2D_VIEWPORT_FRAMING_HALF_SPAN_SCALE).max(1.0);
    let base_zoom = puzzle2d_clamp_zoom((usable / world_span) * PUZZLE2D_VIEWPORT_ZOOM_BOOST);
    let zoom = puzzle2d_clamp_zoom(base_zoom * puzzle2d_pane_zoom_scale(pane));
    match pane {
        PUZZLE2D_PANE_DETAIL => {
            let nodes = fixture_nodes(fixture);
            let detail_node = nodes.get(nodes.len().saturating_sub(1).min(42));
            let x = detail_node.and_then(|node| node.get("x")).and_then(|value| value.as_f64()).unwrap_or(cx) + camera_x * 0.02;
            let y = detail_node.and_then(|node| node.get("y")).and_then(|value| value.as_f64()).unwrap_or(cy) + camera_y * 0.02;
            (x, y, zoom)
        }
        PUZZLE2D_PANE_SELECTION => (cx - half_span * 0.28 + camera_x * 0.06, cy + half_span * 0.22 + camera_y * 0.05, zoom),
        _ => (cx + camera_x * 0.04, cy + camera_y * 0.03, zoom),
    }
}

//#endregion 🔖️PaneCamera

/// 🗄️ Caches the last serialized fixture keyed by an fnv1a hash of the raw `document_json` it came from, so the overview/detail/selection panes of the same `refreshUi` tick reuse one `String` instead of each re-serializing the whole fixture graph.
static PUZZLE2D_FIXTURE_JSON_CACHE: LazyLock<std::sync::Mutex<Option<(u64, String)>>> = LazyLock::new(|| std::sync::Mutex::new(None));

fn fnv1a_hash(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn cached_fixture_json(document_json: &str, fixture: &Value) -> String {
    let key = fnv1a_hash(document_json.as_bytes());
    let mut cache = PUZZLE2D_FIXTURE_JSON_CACHE.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if let Some((cached_key, cached_json)) = cache.as_ref() {
        if *cached_key == key {
            return cached_json.clone();
        }
    }
    let json = fixture.to_string();
    *cache = Some((key, json.clone()));
    json
}

fn puzzle2d_board_scene(document_json: &str, envelope: &Puzzle2dScene, pane: &str) -> Board2dScene {
    let fixture = &envelope.fixture;
    let (camera_x, camera_y, zoom) = puzzle2d_pane_camera(fixture, &envelope.runtime, pane);
    let camera_json = json!({ "x": camera_x, "y": camera_y, "zoom": zoom }).to_string();
    let glyph_catalogs_json = fixture.get("meta").and_then(|value| value.get("kindCatalogs")).map(|value| value.to_string()).unwrap_or_else(|| "{}".into());
    let selection_json = serde_json::to_string(&envelope.runtime.selected_ids).unwrap_or_else(|_| "[]".into());
    let brush_weights_json = serde_json::to_string(&json!({
        "nodeWeights": envelope.runtime.node_kind_weights,
        "handleWeights": envelope.runtime.handle_kind_weights,
    }))
    .unwrap_or_else(|_| "{}".into());
    let placement_compatibility_json = fixture.get("meta").and_then(|value| value.get("kindCompatibility")).or_else(|| fixture.get("kindCompatibility")).map(|value| value.to_string()).unwrap_or_else(|| "[]".into());
    let lod_mode = envelope.runtime.lod_mode_by_pane.get(pane).cloned().unwrap_or_else(|| PUZZLE2D_LOD_MODE_AUTOMATIC.to_string());
    Board2dScene {
        fixture_json: cached_fixture_json(document_json, fixture),
        camera_json,
        glyph_catalogs_json,
        selection_json,
        interactive: pane == PUZZLE2D_PANE_OVERVIEW,
        hovered_id: None,
        active_utility: Some(envelope.active_utility.clone()),
        selection_method: envelope.runtime.selection_method.clone(),
        grid_snap_enabled: envelope.runtime.grid_snap_enabled,
        grid_factor: envelope.runtime.grid_factor,
        suggestion_offset: envelope.runtime.suggestion_offset,
        brush_weights_json,
        placement_compatibility_json,
        lod_mode,
    }
}

fn render_canvas(document_json: &str, envelope: &Puzzle2dScene, pane: &str) -> UiNode {
    build_board2d_scene(format!("{PUZZLE2D_PLAY_SURFACE_ID}.{pane}"), PUZZLE2D_PLAY_CONTROLLER_ID, puzzle2d_board_scene(document_json, envelope, pane))
}

fn force_layout_fixture(fixture: &mut Value) {
    let Ok(layout_json) = puzzle_2d_engine::apply_force_graph_layout_to_fixture_v1_json(&fixture.to_string(), r#"{"mode":"force-graph"}"#) else {
        return;
    };
    if let Ok(parsed) = serde_json::from_str(&layout_json) {
        *fixture = parsed;
    }
}

/** @emoji 📐️ Patches `field` on every selected node: an absolute `value` sets it directly on all
 * of them, otherwise a numeric `delta` is added to each node's own current `field` value —
 * offset-preserving across a multi-select where nodes start at different positions. */
fn patch_inspector_nodes(fixture: &mut Value, ids: &[String], field: &str, value: Option<&Value>, delta: Option<&Value>) {
    if let Some(nodes) = fixture.get_mut("nodes").and_then(|entry| entry.as_array_mut()) {
        for node in nodes {
            let Some(id) = node.get("id").and_then(|entry| entry.as_str()).map(str::to_string) else {
                continue;
            };
            if !ids.is_empty() && !ids.contains(&id) {
                continue;
            }
            let resolved = if let Some(absolute) = value {
                Some(absolute.clone())
            } else if let Some(delta) = delta.and_then(Value::as_f64) {
                let current = node.get(field).and_then(Value::as_f64).unwrap_or(0.0);
                Some(json!(current + delta))
            } else {
                None
            };
            if let (Some(obj), Some(resolved)) = (node.as_object_mut(), resolved) {
                obj.insert(field.to_string(), resolved);
            }
        }
    }
}
//#endregion 🔖️Canvas

//#region 🔖️Terminology
// 🗣️ Complete UI label set for the 2d app; one field per label makes every terminology×locale
// combination compile-checked via `semio_framework_plugin::app_labels!` (see ticket
// 26/08/03/COMPILE-TIME-CHECKED-UI-LABELS-ACROSS-LOCALE-TERMINOLOGY-AND-BRAND). Fields whose reuse
// cells repeat the native text verbatim were previously inherited via `..PUZZLE2D_LABELS_NATIVE_EN`
// struct-update syntax — the new macro has no implicit inheritance, so those cells are now spelled
// out explicitly (same text, four times).
semio_framework_plugin::app_labels! {
    struct Puzzle2dLabels {
        // entity nouns — remapped under the "reuse" terminology
        nodes: native_en "Nodes", native_de "Knoten", reuse_en "Building components", reuse_de "Baukomponenten";
        handles: native_en "Handles", native_de "Anschlüsse", reuse_en "Connection points", reuse_de "Verbindungspunkte";
        // document tree / catalogue section labels
        edges: native_en "Edges", native_de "Kanten", reuse_en "Edges", reuse_de "Kanten";
        none: native_en "(none)", native_de "(keine)", reuse_en "(none)", reuse_de "(keine)";
        // window-kind titles (window headers / tab titles)
        window_overview: native_en "Overview", native_de "Übersicht", reuse_en "Assembly", reuse_de "Baugruppe";
        window_detail: native_en "Detail", native_de "Detail", reuse_en "Connection Detail", reuse_de "Verbindungsdetail";
        window_selection: native_en "Selection", native_de "Auswahl", reuse_en "Component Selection", reuse_de "Komponentenauswahl";
        // properties panel summary labels
        schema: native_en "Schema", native_de "Schema", reuse_en "Schema", reuse_de "Schema";
        extension: native_en "Extension", native_de "Erweiterung", reuse_en "Extension", reuse_de "Erweiterung";
        // inspector field labels
        id: native_en "Id", native_de "Id", reuse_en "Id", reuse_de "Id";
        node_kind: native_en "Node Kind", native_de "Knotenart", reuse_en "Node Kind", reuse_de "Knotenart";
        x: native_en "X", native_de "X", reuse_en "X", reuse_de "X";
        y: native_en "Y", native_de "Y", reuse_en "Y", reuse_de "Y";
        // measures
        automatic: native_en "Automatic", native_de "Automatisch", reuse_en "Automatic", reuse_de "Automatisch";
        lod: native_en "LOD", native_de "LOD", reuse_en "LOD", reuse_de "LOD";
        suggestion: native_en "Suggestion", native_de "Vorschlag", reuse_en "Suggestion", reuse_de "Vorschlag";
        offset: native_en "Offset", native_de "Versatz", reuse_en "Offset", reuse_de "Versatz";
        node_weights: native_en "Node Weights", native_de "Knotengewichte", reuse_en "Node Weights", reuse_de "Knotengewichte";
        handle_weights: native_en "Handle Weights", native_de "Anschlussgewichte", reuse_en "Handle Weights", reuse_de "Anschlussgewichte";
        // engagement
        select: native_en "Select", native_de "Auswählen", reuse_en "Select", reuse_de "Auswählen";
        brush: native_en "Brush", native_de "Pinsel", reuse_en "Brush", reuse_de "Pinsel";
        fill: native_en "Fill", native_de "Füllen", reuse_en "Fill", reuse_de "Füllen";
        count: native_en "Count", native_de "Anzahl", reuse_en "Count", reuse_de "Anzahl";
        placement: native_en "Placement", native_de "Platzierung", reuse_en "Placement", reuse_de "Platzierung";
        // example picker
        example_concrete_forest: native_en "Concrete Forest", native_de "Betonwald", reuse_en "Abbau Aufbau", reuse_de "Abbau Aufbau";
    }
}

/// 🗣️ Resolves the active label set from `Puzzle2dConfig`'s own persisted locale/terminology
/// strings (B1: was `view_state.locale`/`view_state.terminology`) through the generated
/// `Puzzle2dLabels::labels` (`AppLabels`) exhaustive resolver.
fn puzzle2d_labels(config: &Puzzle2dConfig) -> &'static Puzzle2dLabels {
    let locale = if is_de_locale(config) { Locale::De } else { Locale::En };
    let terminology = if config.terminology.as_str() == "reuse" { Terminology::Reuse } else { Terminology::Native };
    Puzzle2dLabels::labels(locale, terminology)
}

/// 🗺️ Builds the full `LocalizedLabel` matrix for one `Puzzle2dLabels` field — for the static
/// manifest (`create_puzzle2d_app`), which must carry every (terminology, locale) cell up front
/// rather than a single resolved-at-render-time `LabelText` (see `puzzle2d_labels`).
fn puzzle2d_localized(field: impl Fn(&Puzzle2dLabels) -> semio_framework_plugin::LabelText) -> LocalizedLabel {
    LocalizedLabel::from_fn(|terminology, locale| field(Puzzle2dLabels::labels(locale, terminology)).as_str().to_string())
}
//#endregion 🔖️Terminology

//#region 🔖️DocumentPanel
fn node_label(node: &Value) -> String {
    node.get("text").and_then(|value| value.as_str()).filter(|value| !value.is_empty()).or_else(|| node.get("id").and_then(|value| value.as_str())).unwrap_or("node").into()
}

fn edge_label(edge: &Value, fixture: &Value) -> String {
    let source = edge.get("source").and_then(|value| value.as_str()).unwrap_or("?");
    let target = edge.get("target").and_then(|value| value.as_str()).unwrap_or("?");
    let source_label = fixture_nodes(fixture).iter().find(|node| node.get("id").and_then(|value| value.as_str()) == Some(source)).map(node_label).unwrap_or_else(|| source.into());
    let target_label = fixture_nodes(fixture).iter().find(|node| node.get("id").and_then(|value| value.as_str()) == Some(target)).map(node_label).unwrap_or_else(|| target.into());
    format!("{source_label} → {target_label}")
}

fn document_tree_selected_ids(fixture: &Value, selected: &[String]) -> Vec<String> {
    selected
        .iter()
        .filter_map(|id| {
            if fixture_nodes(fixture).iter().any(|node| node.get("id").and_then(|value| value.as_str()) == Some(id.as_str())) {
                return Some(format!("puzzle2d-play-document.node.{id}"));
            }
            if fixture_edges(fixture).iter().any(|edge| edge.get("id").and_then(|value| value.as_str()) == Some(id.as_str())) {
                return Some(format!("puzzle2d-play-document.edge.{id}"));
            }
            None
        })
        .collect()
}

fn render_document_panel(envelope: &Puzzle2dScene, labels: &Puzzle2dLabels) -> UiNode {
    let fixture = &envelope.fixture;
    let node_items: Vec<UiTreeItemNode> = fixture_nodes(fixture)
        .iter()
        .filter_map(|node| {
            let id = node.get("id")?.as_str()?;
            Some(tree_item_with_action(
                format!("puzzle2d-play-document.node.{id}"),
                Label::data(node_label(node)),
                node.get("nodeKind").and_then(|value| value.as_str()).map(str::to_string),
                puzzle2d_action("setSelection", Some(json!({ "ids": [id] }))),
            ))
        })
        .collect();
    let edge_items: Vec<UiTreeItemNode> = fixture_edges(fixture)
        .iter()
        .filter_map(|edge| {
            let id = edge.get("id")?.as_str()?;
            Some(tree_item_with_action(
                format!("puzzle2d-play-document.edge.{id}"),
                Label::data(edge_label(edge, fixture)),
                edge.get("edgeKind").and_then(|value| value.as_str()).map(str::to_string),
                puzzle2d_action("setSelection", Some(json!({ "ids": [id] }))),
            ))
        })
        .collect();
    PanelTreeBuilder::new("puzzle2d-play-document")
        .section_or_placeholder("puzzle2d-play-document.nodes", Some(labels.nodes.into()), true, node_items, labels.none)
        .section_or_placeholder("puzzle2d-play-document.edges", Some(labels.edges.into()), false, edge_items, labels.none)
        .selected(document_tree_selected_ids(fixture, &envelope.runtime.selected_ids))
        .selection_change(puzzle2d_action("setSelection", None))
        .build()
}
//#endregion 🔖️DocumentPanel

//#region 🔖️CataloguePanel
fn catalog_kind_label(entry: &Value) -> String {
    entry.get("name").and_then(|value| value.as_str()).filter(|value| !value.is_empty()).or_else(|| entry.get("id").and_then(|value| value.as_str())).unwrap_or("kind").into()
}

fn inferred_kind_entries(fixture: &Value, field: &str) -> Vec<Value> {
    let mut ids = BTreeSet::new();
    match field {
        "nodes" => {
            for node in fixture_nodes(fixture) {
                if let Some(kind) = node.get("nodeKind").and_then(|value| value.as_str()) {
                    ids.insert(kind.to_string());
                }
            }
        }
        "handles" => {
            for node in fixture_nodes(fixture) {
                if let Some(handles) = node.get("handles").and_then(|value| value.as_array()) {
                    for handle in handles {
                        if let Some(kind) = handle.get("handleKind").and_then(|value| value.as_str()) {
                            ids.insert(kind.to_string());
                        }
                    }
                }
            }
        }
        "edges" => {
            for edge in fixture_edges(fixture) {
                if let Some(kind) = edge.get("edgeKind").and_then(|value| value.as_str()) {
                    ids.insert(kind.to_string());
                }
            }
        }
        _ => {}
    }
    ids.into_iter().map(|id| json!({ "id": id, "name": id })).collect()
}

/// 🖱️ MIME key `DeclarativeTreePanel` (framework/renderer/react/ui-interpreter.tsx) reads to auto-wire catalogue drag sources.
const PUZZLE2D_CATALOGUE_DRAG_MIME: &str = "application/x-semio-catalogue-item";

fn puzzle2d_catalog_item_drag_data(slice: &str, kind_id: &str, entry: &Value) -> HashMap<String, String> {
    let mut payload = json!({ "kindId": kind_id, "catalogSlice": slice });
    if let Some(obj) = payload.as_object_mut() {
        if let Some(shape) = entry.get("shape") {
            obj.insert("shape".into(), shape.clone());
        }
        if let Some(radius) = entry.get("radius") {
            obj.insert("radius".into(), radius.clone());
        }
        if let Some(width) = entry.get("width") {
            obj.insert("width".into(), width.clone());
        }
        if let Some(height) = entry.get("height") {
            obj.insert("height".into(), height.clone());
        }
        if let Some(icon_kind) = entry.get("iconKind") {
            obj.insert("iconKind".into(), icon_kind.clone());
        }
    }
    HashMap::from([(PUZZLE2D_CATALOGUE_DRAG_MIME.to_string(), payload.to_string())])
}

fn kind_catalog_section(section_id: &str, slice: &str, label: impl Into<Label>, entries: &[Value], labels: &Puzzle2dLabels) -> UiTreeSectionNode {
    let items: Vec<UiTreeItemNode> = entries
        .iter()
        .enumerate()
        .map(|(index, entry)| {
            let kind_id = entry.get("id").and_then(|value| value.as_str()).unwrap_or("kind");
            let draggable = slice == "nodes";
            UiTreeItemNode {
                presence: UiPresence::default(),
                id: format!("{section_id}.{index}.{kind_id}"),
                label: Label::data(catalog_kind_label(entry)),
                description: Some(kind_id.into()),
                icon_id: None,
                default_open: None,
                action: Some(puzzle2d_action("addNode", Some(json!({ "kind": kind_id })))),
                hover_action: None,
                unhover_action: None,
                actions: None,
                draggable: draggable.then_some(true),
                drag_data: draggable.then(|| puzzle2d_catalog_item_drag_data(slice, kind_id, entry)),
                items: None,
                control: None,
                dimmed: None,
                menu: None,
            }
        })
        .collect();
    UiTreeSectionNode { presence: UiPresence::default(), id: section_id.into(), label: Some(label.into()), default_open: Some(true), items: if items.is_empty() { vec![tree_item(format!("{section_id}.empty"), labels.none)] } else { items } }
}

fn render_catalogue_panel(fixture: &Value, labels: &Puzzle2dLabels) -> UiNode {
    let inferred_nodes = inferred_kind_entries(fixture, "nodes");
    let inferred_handles = inferred_kind_entries(fixture, "handles");
    let inferred_edges = inferred_kind_entries(fixture, "edges");
    let node_entries = kind_catalog_entries(fixture, "nodes").unwrap_or(inferred_nodes.as_slice());
    let handle_entries = kind_catalog_entries(fixture, "handles").unwrap_or(inferred_handles.as_slice());
    let edge_entries = kind_catalog_entries(fixture, "edges").unwrap_or(inferred_edges.as_slice());
    UiNode::Tree(UiTreeNode {
        presence: UiPresence::default(),
        sections: vec![
            kind_catalog_section("puzzle2d-play-kinds.nodes", "nodes", labels.nodes, &node_entries, labels),
            kind_catalog_section("puzzle2d-play-kinds.handles", "handles", labels.handles, &handle_entries, labels),
            kind_catalog_section("puzzle2d-play-kinds.edges", "edges", labels.edges, &edge_entries, labels),
        ],
        selected_ids: None,
        highlighted_ids: None,
        selection_change: None,
        drop_action: None,
        menu: None,
    })
}
//#endregion 🔖️CataloguePanel

//#region 🔖️InspectorPanel
fn render_properties_panel(envelope: &Puzzle2dScene, labels: &Puzzle2dLabels) -> UiNode {
    let selected_nodes: Vec<&Value> = envelope.runtime.selected_ids.iter().filter_map(|id| fixture_nodes(&envelope.fixture).iter().find(|node| node.get("id").and_then(|value| value.as_str()) == Some(id.as_str()))).collect();
    if selected_nodes.is_empty() {
        return ui_stack_vertical(vec![
            ui_text(Label::data(format!("{}: {PUZZLE2D_FIXTURE_SCHEMA}", labels.schema.as_str()))),
            ui_text(Label::data(format!("{}: {}", labels.extension.as_str(), puzzle_extension_id()))),
            ui_text(Label::data(format!("{}: {}", labels.nodes.as_str(), fixture_nodes(&envelope.fixture).len()))),
            ui_text(Label::data(format!("{}: {}", labels.edges.as_str(), fixture_edges(&envelope.fixture).len()))),
        ]);
    }
    let ids: Vec<String> = selected_nodes.iter().filter_map(|node| node.get("id").and_then(|value| value.as_str()).map(str::to_string)).collect();
    let ids_json = json!(ids);
    let patch_cmd = |field: &str| puzzle2d_action("patchInspectorNodes", Some(json!({ "ids": ids_json, "field": field })));
    let kinds: Vec<String> = selected_nodes.iter().map(|node| node.get("nodeKind").and_then(|value| value.as_str()).unwrap_or("").to_string()).collect();
    let xs: Vec<f64> = selected_nodes.iter().map(|node| node.get("x").and_then(|value| value.as_f64()).unwrap_or(0.0)).collect();
    let ys: Vec<f64> = selected_nodes.iter().map(|node| node.get("y").and_then(|value| value.as_f64()).unwrap_or(0.0)).collect();
    let id_text = if let [id] = ids.as_slice() { id.clone() } else { format!("{} nodes", ids.len()) };
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "puzzle2d-play-inspector".into(),
        label: labels.node_kind.into(),
        default_open: Some(true),
        presence: UiPresence::default(),
        fields: vec![
            ui_inspector_readonly_field("puzzle2d-play-inspector.id", labels.id, id_text),
            ui_inspector_readonly_field("puzzle2d-play-inspector.node-kind", labels.node_kind, ui_inspector_mixed_text(&kinds).value),
            ui_inspector_stepper_field("puzzle2d-play-inspector.x", labels.x, &xs, 1.0, patch_cmd("x")),
            ui_inspector_stepper_field("puzzle2d-play-inspector.y", labels.y, &ys, 1.0, patch_cmd("y")),
        ],
    }])
}
//#endregion 🔖️InspectorPanel

//#region 🔖️Measures
fn puzzle2d_lod_tier_ids() -> Vec<String> {
    serde_json::from_str::<Vec<Value>>(&puzzle_2d_lod_scale_json()).unwrap_or_default().into_iter().filter_map(|row| row.get("id").and_then(|value| value.as_str()).map(str::to_string)).collect()
}

fn puzzle2d_kind_ids(fixture: &Value, field: &str) -> Vec<String> {
    let inferred = inferred_kind_entries(fixture, field);
    let entries = kind_catalog_entries(fixture, field).unwrap_or(inferred.as_slice());
    entries.iter().filter_map(|entry| entry.get("id").and_then(|value| value.as_str()).map(str::to_string)).collect()
}

fn puzzle2d_uniform_kind_weights(ids: &[String]) -> BTreeMap<String, f64> {
    if ids.is_empty() {
        return BTreeMap::new();
    }
    let weight = 1.0 / ids.len() as f64;
    ids.iter().map(|id| (id.clone(), weight)).collect()
}

fn puzzle2d_normalize_kind_weight_group(weights: &BTreeMap<String, f64>, kind_ids: &[String], changed_id: &str, new_value: f64) -> BTreeMap<String, f64> {
    if kind_ids.is_empty() {
        return BTreeMap::new();
    }
    if kind_ids.len() == 1 {
        return BTreeMap::from([(kind_ids[0].clone(), 1.0)]);
    }
    let new_value = new_value.clamp(0.0, 1.0);
    let others: Vec<&String> = kind_ids.iter().filter(|id| id.as_str() != changed_id).collect();
    let remainder = (1.0 - new_value).max(0.0);
    let other_sum: f64 = others.iter().map(|id| weights.get(*id).copied().unwrap_or(0.0)).sum();
    let mut next = BTreeMap::new();
    next.insert(changed_id.to_string(), new_value);
    if remainder <= f64::EPSILON {
        for id in others {
            next.insert((*id).clone(), 0.0);
        }
        return next;
    }
    if other_sum <= f64::EPSILON {
        let each = remainder / others.len() as f64;
        for id in others {
            next.insert((*id).clone(), each);
        }
    } else {
        for id in others {
            let old = weights.get(id).copied().unwrap_or(0.0);
            next.insert((*id).clone(), old / other_sum * remainder);
        }
    }
    next
}

fn puzzle2d_ensure_catalog_kind_weights(weights: &mut BTreeMap<String, f64>, kind_ids: &[String]) {
    if kind_ids.is_empty() {
        return;
    }
    if weights.is_empty() || kind_ids.iter().any(|id| !weights.contains_key(id)) {
        *weights = puzzle2d_uniform_kind_weights(kind_ids);
        return;
    }
    let sum: f64 = kind_ids.iter().map(|id| weights.get(id).copied().unwrap_or(0.0)).sum();
    if (sum - 1.0).abs() > 0.001 {
        for id in kind_ids {
            if let Some(weight) = weights.get_mut(id) {
                *weight /= sum;
            }
        }
    }
}

fn puzzle2d_kind_weight_sum(weights: &BTreeMap<String, f64>, kind_ids: &[String]) -> f64 {
    kind_ids.iter().map(|id| weights.get(id).copied().unwrap_or(0.0)).sum()
}

/// 📶️ Per-pane LOD select measure: "Automatic" plus every scale tier (minimap…micro), persisted via `setLodModeForPane`.
fn puzzle2d_lod_measure(pane: &str, current_mode: &str, labels: &Puzzle2dLabels) -> WindowMeasure {
    let mut items = vec![MeasureSelectItem { id: PUZZLE2D_LOD_MODE_AUTOMATIC.into(), value: PUZZLE2D_LOD_MODE_AUTOMATIC.into(), label: labels.automatic.into() }];
    items.extend(puzzle2d_lod_tier_ids().into_iter().map(|tier| MeasureSelectItem { id: tier.clone(), value: tier.clone(), label: tier }));
    WindowMeasure::Select { id: format!("{pane}-lod"), label: Some(labels.lod.into()), value: current_mode.into(), items, on_change: puzzle2d_action("setLodModeForPane", Some(json!({ "pane": pane }))) }
}

fn puzzle2d_kind_weight_measures(prefix: &str, ids: &[String], weights: &BTreeMap<String, f64>, catalog_slice: &str) -> Vec<WindowMeasure> {
    ids.iter()
        .map(|kind_id| {
            let weight = weights.get(kind_id).copied().unwrap_or_else(|| if ids.is_empty() { 0.0 } else { 1.0 / ids.len() as f64 });
            WindowMeasure::Slider {
                id: format!("{PUZZLE2D_PLAY_CONTROLLER_ID}-{prefix}-{kind_id}"),
                label: Some(format!("{kind_id} {:.0}%", weight * 100.0)),
                value: weight,
                min: 0.0,
                max: 1.0,
                step: Some(0.01),
                ready: None,
                loading: None,
                waiting: None,
                disabled: None,
                reveal: None,
                on_change: puzzle2d_action("setBrushKindWeights", Some(json!({ "kindId": kind_id, "catalogSlice": catalog_slice }))),
            }
        })
        .collect()
}

fn puzzle2d_window_measures(pane: &str, envelope: &Puzzle2dScene, labels: &Puzzle2dLabels) -> Vec<WindowMeasure> {
    let mode = envelope.runtime.lod_mode_by_pane.get(pane).map(String::as_str).unwrap_or(PUZZLE2D_LOD_MODE_AUTOMATIC);
    vec![puzzle2d_lod_measure(pane, mode, labels), puzzle2d_brush_utility_options(envelope, labels)]
}
//#endregion 🔖️Measures

//#region 🔖️Puzzle2dPlayApp
/// 🧩️ Puzzle-2d play app. Owns the `BoardHost` engine and ephemeral view `runtime`; the persisted
/// document (the bare fixture json) lives in the wrapping `VcsDocumentApp`'s operation store. Each action
/// rehydrates the host from the projection, mutates a transient {@link Puzzle2dScene}, then emits
/// the granular operation delta (`puzzle2d_document_delta_operations`) turning the old fixture into the new.
pub struct Puzzle2dPlayApp {
    host: RefCell<BoardHost>,
    /// 🗄️ The fixture content last parsed into `host` via `parse_fixture_v1` — lets `handle_action_impl`
    /// skip that full clear-scene-and-rebuild (and the kind-catalog/kind-compat re-push) on the
    /// large majority of actions (select/camera/utility/…) that never touch fixture content.
    last_synced_fixture: RefCell<Option<Value>>,
}

impl Default for Puzzle2dPlayApp {
    fn default() -> Self {
        Self { host: RefCell::new(puzzle_board_host()), last_synced_fixture: RefCell::new(None) }
    }
}

/// 🖱️ On-demand puzzle 2d board context menu from selection snapshot. Grouped disclosure:
/// toggleHidden/toggleLocked/duplicate/focusSelection stay top-level (the four most frequent
/// verbs); selectSameKind folds into the "selection" taxonomy group; deleteSelection stays the
/// destructive tail. `organize_context_menu` (applied automatically at the
/// `VcsDocumentApp::context_menu` funnel) sorts groups into `RIBBON_PARENT_CATEGORIES` order and
/// inserts the pre-destructive separator itself, so no manual `.separator()` calls are needed
/// here (unlike the pre-migration hand-written `sep-selection`/`sep-delete` rows).
fn puzzle2d_context_menu_items(registry: &semio_framework_plugin::AppActionRegistry, fixture: &Value, selected: &[String], is_de: bool) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
    use semio_framework_plugin::{selection_count_phrase, ContextMenuItemSpec, Menu};
    // 🧩️ Bespoke-row helper (dynamic label/icon/args/disabled per selection state — not a plain
    // declared-action lookup) — appended via `Menu::item(...)`, the documented escape hatch.
    let item = |id: &str, label: &str, icon: &str, action: &str, args: Option<Value>, destructive: bool, disabled: bool| ContextMenuItemSpec {
        id: id.into(),
        label: Some(label.into()),
        icon: Some(icon.into()),
        action: Some(action.into()),
        args: semio_framework_plugin::optional_json_to_dsl(args),
        destructive: destructive.then_some(true),
        disabled: disabled.then_some(true),
        ..Default::default()
    };
    if selected.is_empty() {
        return Menu::of(registry).item(item("selectAll", if is_de { "Alles auswählen" } else { "Select All" }, "select-all", "selectAll", None, false, false)).build();
    }
    let selected_set: std::collections::HashSet<&str> = selected.iter().map(String::as_str).collect();
    let mut entities: Vec<&Value> = Vec::new();
    let mut has_selected_node = false;
    if let Some(nodes) = fixture.get("nodes").and_then(|v| v.as_array()) {
        for node in nodes {
            if node.get("id").and_then(|v| v.as_str()).is_some_and(|id| selected_set.contains(id)) {
                entities.push(node);
                has_selected_node = true;
            }
            if let Some(handles) = node.get("handles").and_then(|v| v.as_array()) {
                for handle in handles {
                    if handle.get("id").and_then(|v| v.as_str()).is_some_and(|id| selected_set.contains(id)) {
                        entities.push(handle);
                    }
                }
            }
        }
    }
    if let Some(edges) = fixture.get("edges").and_then(|v| v.as_array()) {
        for edge in edges {
            if edge.get("id").and_then(|v| v.as_str()).is_some_and(|id| selected_set.contains(id)) {
                entities.push(edge);
            }
        }
    }
    let any_visible = entities.iter().any(|entity| entity.get("hidden").and_then(|v| v.as_bool()) != Some(true));
    let any_unlocked = entities.iter().any(|entity| entity.get("locked").and_then(|v| v.as_bool()) != Some(true));
    let phrase = selection_count_phrase(is_de, &[(selected.len(), if is_de { "Element" } else { "item" }, if is_de { "Elemente" } else { "items" })]);
    Menu::of(registry)
        .item(item(
            "toggleHidden",
            if any_visible {
                if is_de {
                    "Ausblenden"
                } else {
                    "Hide"
                }
            } else {
                if is_de {
                    "Einblenden"
                } else {
                    "Show"
                }
            },
            if any_visible { "eye-off" } else { "eye" },
            "setSelectionFlag",
            Some(json!({ "flag": "hidden", "value": any_visible })),
            false,
            false,
        ))
        .item(item(
            "toggleLocked",
            if any_unlocked {
                if is_de {
                    "Sperren"
                } else {
                    "Lock"
                }
            } else {
                if is_de {
                    "Entsperren"
                } else {
                    "Unlock"
                }
            },
            if any_unlocked { "lock" } else { "lock-open" },
            "setSelectionFlag",
            Some(json!({ "flag": "locked", "value": any_unlocked })),
            false,
            false,
        ))
        .item(item("duplicate", if is_de { "Duplizieren" } else { "Duplicate" }, "copy", "duplicateSelection", None, false, !has_selected_node))
        .item(item("focusSelection", if is_de { "Auf Auswahl zoomen" } else { "Zoom to selection" }, "crosshair", "focusSelection", None, false, false))
        .group("selection", |m| m.item(item("selectSameKind", if is_de { "Gleiche Art auswählen" } else { "Select same kind" }, "layers", "selectSameKind", None, false, false)))
        .item(item("deleteSelection", &format!("{} ({phrase})", if is_de { "Löschen" } else { "Delete" }), "trash", "deleteSelection", None, true, false))
        .build()
}

//#region 🔖️Puzzle2dCommand
/// @emoji 🎯️ B1: `Puzzle2dPlayApp::Command` — the SOLE dispatch surface, one variant per declared
/// action (mirrors every `.operation(...)`/`.view_action(...)`/`.action_with(...)` id
/// `create_puzzle2d_app` registers below, plus the framework-injected `setActiveUtility` and the new
/// `setLocale`/`setTerminology` — B1 additions now that `ViewState` no longer carries them). Each
/// variant carries `window_id` (was host-pushed `view_state.window_id`) plus `args` (the action's
/// original `{...}` JSON payload, unchanged) — `handle` reconstructs the exact `(action, args,
/// window_id)` triple `handle_action_impl` (the preserved pre-B1 business logic) already expects, so
/// every arm's internal `args.get("field")` extraction stays byte-for-byte identical to the pre-B1
/// implementation. `OpBinary` is a plain JSON-bytes bridge (not `#[derive(dsl::DslOps)]`) — mirrors
/// `puzzle_3d_ui::Puzzle3dCommand`'s identical "local JSON bridge" idiom: a generic `args: Value` field
/// is not representable in the DSL grammar the `#[derive(dsl::DslOps)]` macro targets.
macro_rules! puzzle2d_command_variants {
    ($($Variant:ident = $id:tt),* $(,)?) => {
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum Puzzle2dCommand {
            $($Variant { window_id: Option<String>, args: Option<Value> }),*
        }

        impl Puzzle2dCommand {
            /// 🏷️ The action id this variant was declared under — used both for `command_id()`
            /// (command-log labeling / registry kind-discipline) and to reconstruct the exact
            /// `action: &str` `handle_action_impl` dispatches on.
            fn action_id(&self) -> &'static str {
                match self {
                    $(Puzzle2dCommand::$Variant { .. } => $id),*
                }
            }

            fn window_id(&self) -> Option<&str> {
                match self {
                    $(Puzzle2dCommand::$Variant { window_id, .. } => window_id.as_deref()),*
                }
            }

            fn args(&self) -> Option<&Value> {
                match self {
                    $(Puzzle2dCommand::$Variant { args, .. } => args.as_ref()),*
                }
            }

            /// 🧪️ Test-only reverse of `action_id()` — builds the variant for a given action id, for
            /// the existing test module's `dispatch_action(...)` helper (see `//#region 🧪️Tests`).
            /// Panics on an unknown action id (a test bug, not a runtime path).
            #[cfg(test)]
            fn from_action(action: &str, args: Option<Value>, window_id: Option<String>) -> Self {
                match action {
                    $($id => Puzzle2dCommand::$Variant { window_id, args }),*,
                    other => panic!("unknown puzzle2d action id in test: {other}"),
                }
            }
        }
    };
}

puzzle2d_command_variants! {
    AddNode = "addNode",
    SetActiveExample = "setActiveExample",
    DeleteSelection = "deleteSelection",
    DuplicateSelection = "duplicateSelection",
    ForceLayout = "forceLayout",
    FocusSelection = "focusSelection",
    SelectAll = "selectAll",
    ClearSelection = "clearSelection",
    SelectSameKind = "selectSameKind",
    SetSelectionFlag = "setSelectionFlag",
    PatchInspectorNodes = "patchInspectorNodes",
    RedrawHandles = "redrawHandles",
    Reorganize = "reorganize",
    ApplyBoardEvents = "applyBoardEvents",
    SetFillCount = "setFillCount",
    BrushFillSessionStep = "brushFillSessionStep",
    BrushCommitSlot = "brushCommitSlot",
    SetCamera = "setCamera",
    SetSelection = "setSelection",
    DocumentSelect = "documentSelect",
    EngagementInput = "engagementInput",
    EngagementSubmit = "engagementSubmit",
    EngagementAbort = "engagementAbort",
    EngagementControlSelect = "engagementControlSelect",
    SetLodModeForPane = "setLodModeForPane",
    SetGridSnapEnabled = "setGridSnapEnabled",
    SetGridFactor = "setGridFactor",
    SetSelectionMethod = "setSelectionMethod",
    SetBrushKindWeights = "setBrushKindWeights",
    SetBrushNodeSize = "setBrushNodeSize",
    SetSuggestionOffset = "setSuggestionOffset",
    BrushCycleCandidate = "brushCycleCandidate",
    BrushSetCandidateIndex = "brushSetCandidateIndex",
    BrushOpenSlot = "brushOpenSlot",
    BrushCancelSlot = "brushCancelSlot",
    BrushFillSessionBegin = "brushFillSessionBegin",
    BrushFillSessionClear = "brushFillSessionClear",
    LodScaleJson = "lodScaleJson",
    SetActiveUtility = SET_ACTIVE_UTILITY_ACTION_ID,
    // 🗣️ B1: locale/terminology used to be host-pushed `ViewState` fields with no app-level action of
    // their own; now that `ViewState` is gone from the app-facing surface, they need a real Command.
    SetLocale = "setLocale",
    SetTerminology = "setTerminology",
}

impl protocol::OpBinary for Puzzle2dCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        serde_json::to_vec(self).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        serde_json::from_slice(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
    }
}
//#endregion 🔖️Puzzle2dCommand

impl DocumentApp for Puzzle2dPlayApp {
    type Projection = Puzzle2dPlayProjection;
    type Operation = Puzzle2dOperation;
    type Config = Puzzle2dConfig;
    type ConfigOperation = Puzzle2dConfigOperation;
    type Command = Puzzle2dCommand;

    fn app_id(&self) -> &str {
        PUZZLE2D_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        PUZZLE2D_FIXTURE_SCHEMA
    }

    fn initial_projection(&self) -> Puzzle2dPlayProjection {
        Puzzle2dPlayProjection(default_empty_fixture())
    }

    /// 🏷️ Maps each `Puzzle2dCommand` variant back to the action id it was declared under.
    fn command_id(&self, command: &Puzzle2dCommand) -> &str {
        command.action_id()
    }

    /// @emoji 🧩️ B1: thin typed-command adapter — reconstructs the exact `(action, args, window_id)`
    /// triple the preserved pre-B1 `handle_action_impl` (see its doc comment, in the `impl
    /// Puzzle2dPlayApp` block below) already expects, from the typed `Puzzle2dCommand`.
    fn handle(&self, command: &Puzzle2dCommand, doc: &DocumentView<'_, Puzzle2dPlayProjection>, cfg: &ConfigView<'_, Puzzle2dConfig>) -> Result<Emit<Puzzle2dOperation, Puzzle2dConfigOperation>, Fault> {
        self.handle_action_impl(command.action_id(), command.args(), command.window_id(), doc, cfg.projection)
    }

    /// 🔌️ Declares puzzle2d's typed media I/O surface — the implicit document ports plus `kit:in`
    /// (see `import_media` below for why it stays `NotImplemented`) and `design:out`.
    fn io(&self) -> Option<AppIo> {
        Some(
            AppIo::from_document("puzzle.2d", MediaType { class: MediaClass::TwoD, form: MediaForm::Design }, ArtifactPresentation { id: "2d.puzzle".into(), name: "2D Puzzle".into(), dimension: "2d".into(), component_kind: "puzzle2d".into() })
                .with_ports(vec![
                    MediaPortSpec {
                        id: "kit:in".into(),
                        label: "Kit Catalog".into(),
                        direction: MediaPortDirection::In,
                        media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
                        kind_id: Some("kit.catalog".into()),
                        required: false,
                        multiplicity: PortMultiplicity::Many,
                    },
                    MediaPortSpec {
                        id: "design:out".into(),
                        label: "Puzzle Design".into(),
                        direction: MediaPortDirection::Out,
                        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Design },
                        kind_id: Some("2d.puzzle".into()),
                        required: false,
                        multiplicity: PortMultiplicity::Many,
                    },
                ]),
        )
    }

    /// 🚧️ `kit:in` is declared (see `io()`) so a future producer/consumer pairing is possible, but
    /// puzzle2d's own "kind catalogs" (node/handle kind weights, keyed by this app's own node/handle
    /// kind vocabulary) are structurally UNRELATED to `kit.catalog`'s shape (block3d's object/vortex-kind
    /// vocabulary — meshes, 3D vortex positions, cable/attraction kinds), unlike puzzle3d's `kit:in`,
    /// which DOES share block3d's object-kind vocabulary. There is no honest mapping to fabricate, so
    /// this always reports `NotImplemented` — no normalization is attempted.
    fn import_media(&self, _port: &str, _media: &Media, _doc: &DocumentView<'_, Puzzle2dPlayProjection>) -> Result<Emit<Puzzle2dOperation, Puzzle2dConfigOperation>, MediaError> {
        Err(MediaError::NotImplemented)
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, Puzzle2dPlayProjection>, cfg: &ConfigView<'_, Puzzle2dConfig>) -> UiNode {
        let config = cfg.projection;
        let document_json = doc.projection.0.to_string();
        // 🪟️ B1: `body_key` already determines the pane deterministically (unlike puzzle3d's
        // single-window-kind `render`, which has to fall back to `config.window_ids.first()`), so the
        // active utility resolves off the real targeted pane instead of an ambiguous stand-in.
        let pane = match body_key {
            PUZZLE2D_PLAY_BODY_OVERVIEW => Some(PUZZLE2D_PANE_OVERVIEW),
            PUZZLE2D_PLAY_BODY_DETAIL => Some(PUZZLE2D_PANE_DETAIL),
            PUZZLE2D_PLAY_BODY_SELECTION => Some(PUZZLE2D_PANE_SELECTION),
            _ => None,
        };
        let envelope = Puzzle2dScene { fixture: doc.projection.0.clone(), runtime: config.clone(), active_utility: puzzle2d_active_utility(config, pane) };
        let labels = puzzle2d_labels(config);
        match body_key {
            PUZZLE2D_PLAY_BODY_OVERVIEW => render_canvas(&document_json, &envelope, PUZZLE2D_PANE_OVERVIEW),
            PUZZLE2D_PLAY_BODY_DETAIL => render_canvas(&document_json, &envelope, PUZZLE2D_PANE_DETAIL),
            PUZZLE2D_PLAY_BODY_SELECTION => render_canvas(&document_json, &envelope, PUZZLE2D_PANE_SELECTION),
            PUZZLE2D_PLAY_BODY_LAYERS => render_document_panel(&envelope, labels),
            PUZZLE2D_PLAY_BODY_CATALOGUE => render_catalogue_panel(&envelope.fixture, labels),
            PUZZLE2D_PLAY_BODY_PROPERTIES => render_properties_panel(&envelope, labels),
            _ => ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    fn window_engagements(&self, doc: &DocumentView<'_, Puzzle2dPlayProjection>, cfg: &ConfigView<'_, Puzzle2dConfig>) -> HashMap<String, WindowEngagement> {
        let config = cfg.projection;
        let labels = puzzle2d_labels(config);
        // 🪟️ One entry per live window INSTANCE of each pane kind — see `window_instance_ids`'s
        // docstring for why puzzle2d always has exactly one instance per pane (no split tracking).
        PUZZLE2D_PANES
            .iter()
            .flat_map(|pane| {
                window_instance_ids(pane).into_iter().map(|wid| {
                    let envelope = Puzzle2dScene { fixture: doc.projection.0.clone(), runtime: config.clone(), active_utility: puzzle2d_active_utility(config, Some(&wid)) };
                    (wid, puzzle2d_engagement(&envelope, &self.host.borrow(), pane, labels))
                })
            })
            .collect()
    }

    fn window_measures(&self, doc: &DocumentView<'_, Puzzle2dPlayProjection>, cfg: &ConfigView<'_, Puzzle2dConfig>) -> HashMap<String, Vec<WindowMeasure>> {
        let config = cfg.projection;
        let labels = puzzle2d_labels(config);
        PUZZLE2D_PANES
            .iter()
            .flat_map(|pane| {
                window_instance_ids(pane).into_iter().map(|wid| {
                    let envelope = Puzzle2dScene { fixture: doc.projection.0.clone(), runtime: config.clone(), active_utility: puzzle2d_active_utility(config, Some(&wid)) };
                    (wid, puzzle2d_window_measures(pane, &envelope, labels))
                })
            })
            .collect()
    }

    fn tool_measures(&self, doc: &DocumentView<'_, Puzzle2dPlayProjection>, cfg: &ConfigView<'_, Puzzle2dConfig>) -> HashMap<String, Vec<WindowMeasure>> {
        let config = cfg.projection;
        let envelope = Puzzle2dScene { fixture: doc.projection.0.clone(), runtime: config.clone(), active_utility: puzzle2d_active_utility(config, None) };
        let labels = puzzle2d_labels(config);
        HashMap::from([(PUZZLE2D_UTILITY_FILL.to_string(), vec![puzzle2d_fill_tool_measures(&envelope, labels)])])
    }

    fn context_menu(
        &self,
        request: &semio_framework_plugin::ContextMenuRequest,
        doc: &DocumentView<'_, Puzzle2dPlayProjection>,
        cfg: &ConfigView<'_, Puzzle2dConfig>,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        let config = cfg.projection;
        let is_de = is_de_locale(config);
        let mut selected = config.selected_ids.clone();
        if let Some(surface) = request.surface.as_ref() {
            let ids: Vec<String> = surface.selection.iter().flat_map(|g| g.ids.iter().cloned()).collect();
            if !ids.is_empty() {
                selected = ids;
            }
        }
        puzzle2d_context_menu_items(registry, &doc.projection.0, &selected, is_de)
    }
}

impl Puzzle2dPlayApp {
    /// @emoji 🧩️ B1: the pure per-action core, dispatched into by `DocumentApp::handle` above with
    /// `action`/`args`/`window_id` reconstructed 1:1 from the typed `Puzzle2dCommand` — everything past
    /// this adapter boundary is the ORIGINAL pre-B1 business logic, unchanged, now reading a passed-in
    /// `Puzzle2dConfig` snapshot instead of an ambient `self.runtime` `RefCell` and returning a real
    /// `Emit` (document + config operations) instead of mutating `self` and returning a bare
    /// document-only `ActionEmit`.
    fn handle_action_impl(&self, action: &str, args: Option<&Value>, window_id: Option<&str>, doc: &DocumentView<'_, Puzzle2dPlayProjection>, config: &Puzzle2dConfig) -> Emit<Puzzle2dOperation, Puzzle2dConfigOperation> {
        let before = doc.projection.0.clone();
        let active_utility = puzzle2d_active_utility(config, window_id);
        let mut envelope = Puzzle2dScene { fixture: before.clone(), runtime: config.clone(), active_utility: active_utility.clone() };
        // 🐢️ `sync_host_fixture_content` (`parse_fixture_v1`) does a full `clear_scene()` + rebuild of
        // every node/handle/edge — skip it when the fixture content is byte-identical to what `host`
        // already has (the common case: select/camera/utility/… actions never touch fixture content).
        if self.last_synced_fixture.borrow().as_ref() != Some(&envelope.fixture) {
            sync_host_fixture_content(&mut self.host.borrow_mut(), &envelope);
            // 🧹️ `parse_fixture_v1` always `clear_scene()`s then rebuilds, so it unconditionally emits
            // an `edgeCreate` for every edge as a side effect of parsing — not a real structural
            // change. Discard that parse-induced noise now so `apply_host_events` below only sees
            // events genuinely produced by *this* action's own engine calls (delete_selection, brush
            // operations, …); otherwise those spurious edgeCreate events get replayed into
            // `envelope.fixture.edges` on the *next* action, duplicating every edge every action.
            let _ = self.host.borrow_mut().drain_events_json();
            *self.last_synced_fixture.borrow_mut() = Some(envelope.fixture.clone());
        }
        sync_host_runtime_state(&mut self.host.borrow_mut(), &envelope);
        // 🎥️ No action coalesces anymore: `setCamera` used to be the sole `coalesce_key` writer, but
        // it is now a View-kind action that never touches the document (see `ActionKind::View` above).
        let coalesce_key: Option<String> = None;
        let mut effects: Vec<HostEffect> = Vec::new();
        // 🐢️ Default to Full (safe: every unrecognized/rare action re-renders everything, same as
        // before this ticket); the narrow-tier arms below override it to the smallest scope that
        // actually covers what they touch.
        let mut ui_scope = semio_framework_core::kernel::UiDirtyScope::Full;
        match action {
            "setSelection" | "documentSelect" => {
                envelope.runtime.selected_ids = selection_ids(args);
                self.host.borrow_mut().set_selection_ids(&envelope.runtime.selected_ids);
                ui_scope = puzzle2d_select_scope();
            }
            "addNode" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str());
                add_node_to_fixture(&mut envelope.fixture, kind, args);
                {}
            }
            "deleteSelection" => {
                self.host.borrow_mut().delete_selection();
                delete_selection_from_fixture(&mut envelope.fixture, &envelope.runtime.selected_ids);
                envelope.runtime.selected_ids.clear();
                {}
            }
            "setSelectionFlag" => {
                let flag = args.and_then(|value| value.get("flag")).and_then(|value| value.as_str()).unwrap_or("hidden");
                let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_bool()).unwrap_or(true);
                apply_selection_flag(&mut envelope.fixture, &envelope.runtime.selected_ids, flag, value);
                {}
            }
            "duplicateSelection" => {
                let new_ids = duplicate_selection_in_fixture(&mut envelope.fixture, &envelope.runtime.selected_ids);
                if new_ids.is_empty() {
                    {}
                } else {
                    envelope.runtime.selected_ids = new_ids;
                    self.host.borrow_mut().set_selection_ids(&envelope.runtime.selected_ids);
                    {}
                }
            }
            "selectSameKind" => {
                let ids = select_same_kind_ids(&envelope.fixture, &envelope.runtime.selected_ids);
                if ids.is_empty() {
                    {}
                } else {
                    envelope.runtime.selected_ids = ids;
                    self.host.borrow_mut().set_selection_ids(&envelope.runtime.selected_ids);
                    {}
                }
            }
            "setCamera" => {
                // 🎥️ View-kind: the camera is session-only runtime state, never a document edit — no
                // operation is emitted (nothing to coalesce, so no `coalesce_key` either).
                if let Some(camera) = args.and_then(|value| value.get("camera")) {
                    if let (Some(x), Some(y), Some(zoom)) = (camera.get("x").and_then(|value| value.as_f64()), camera.get("y").and_then(|value| value.as_f64()), camera.get("zoom").and_then(|value| value.as_f64())) {
                        self.host.borrow_mut().set_camera(x, y, zoom);
                    }
                    set_runtime_camera(&mut envelope.runtime, camera);
                    ui_scope = puzzle2d_window_only_scope();
                }
            }
            "setActiveExample" => {
                let example_id = args.and_then(|value| value.get("exampleId")).and_then(|value| value.as_str()).unwrap_or("");
                envelope.fixture = if example_id.is_empty() {
                    default_empty_fixture()
                } else if example_id == PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID || example_id == "concrete" {
                    serde_json::from_str(CONCRETE_FOREST_EXAMPLE_JSON.as_str()).unwrap_or_else(|_| default_empty_fixture())
                } else if example_id == PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID || example_id == "nakagin" {
                    serde_json::from_str(NAKAGIN_EXAMPLE_JSON.as_str()).unwrap_or_else(|_| default_empty_fixture())
                } else {
                    default_empty_fixture()
                };
                envelope.runtime = Puzzle2dPlayRuntime::default();
                {}
            }
            SET_ACTIVE_UTILITY_ACTION_ID => {
                // 🧰️ B1: this Command IS the utility switch now (was host-applied ambient
                // `view_state.active_utility_id`/`active_utility_by_window_id` — the host no longer owns
                // that state, `Puzzle2dConfig` does), so this arm must itself persist the new value
                // before clearing any in-progress brush/fill scratch.
                if let Some(utility_id) = args.and_then(|value| value.get("utilityId")).and_then(|value| value.as_str()) {
                    let wid = window_id.unwrap_or(PUZZLE2D_PANE_OVERVIEW).to_string();
                    envelope.runtime.active_utility_by_window_id.insert(wid, utility_id.to_string());
                }
                self.host.borrow_mut().brush_fill_session_clear();
                self.host.borrow_mut().brush_cancel_slot();
                let _ = self.host.borrow_mut().drain_events_json();
                envelope.runtime.fill_count = 0;
                envelope.runtime.brush_candidates.clear();
                envelope.runtime.brush_candidate_index = 0;
                envelope.runtime.brush_candidate_source_handle_id = String::new();
                for pane in PUZZLE2D_PANES {
                    envelope.runtime.engagement_input_by_pane.insert(pane.to_string(), String::new());
                }
            }
            "engagementInput" => {
                let pane = args.and_then(|value| value.get("pane")).and_then(|value| value.as_str()).unwrap_or(PUZZLE2D_PANE_OVERVIEW);
                let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).unwrap_or("");
                if PUZZLE2D_PANES.contains(&pane) {
                    envelope.runtime.engagement_input_by_pane.insert(pane.to_string(), value.to_string());
                    ui_scope = puzzle2d_window_and_engagements_scope();
                }
            }
            "engagementSubmit" => {
                let pane = args.and_then(|value| value.get("pane")).and_then(|value| value.as_str()).unwrap_or(PUZZLE2D_PANE_OVERVIEW).to_string();
                let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).map(str::trim).unwrap_or("").to_lowercase();
                let applied = match value.as_str() {
                    "select" | "brush" => {
                        // 🧰️ Reconcile the engagement text-command utility switch through the host-owned
                        // active utility: point the local engine now and let the framework persist the new
                        // `view_state.active_utility_id` for the pane via `HostEffect::SetActiveUtility`.
                        self.host.borrow_mut().set_active_utility(value.as_str());
                        effects.push(HostEffect::SetActiveUtility { window_id: pane.clone(), utility_id: value.clone() });
                        true
                    }
                    "fill" => {
                        // 🛠️ Fill is a mode-level tool, not a window utility — activate it through
                        // `HostEffect::SetActiveTool`, leaving this window's active utility untouched.
                        effects.push(HostEffect::SetActiveTool { tool_id: PUZZLE2D_UTILITY_FILL.into() });
                        true
                    }
                    "clear" => {
                        envelope.runtime.selected_ids.clear();
                        self.host.borrow_mut().set_selection_ids(&[]);
                        true
                    }
                    "rectangle" => {
                        envelope.runtime.selection_method = "rectangle".into();
                        self.host.borrow_mut().set_selection_options("rectangle", "replace", true, true, true);
                        true
                    }
                    "lasso" => {
                        envelope.runtime.selection_method = "lasso".into();
                        self.host.borrow_mut().set_selection_options("lasso", "replace", true, true, true);
                        true
                    }
                    _ => false,
                };
                if applied && PUZZLE2D_PANES.contains(&pane.as_str()) {
                    envelope.runtime.engagement_input_by_pane.insert(pane, String::new());
                }
                {}
            }
            "engagementAbort" => {
                let pane = args.and_then(|value| value.get("pane")).and_then(|value| value.as_str()).unwrap_or(PUZZLE2D_PANE_OVERVIEW);
                if PUZZLE2D_PANES.contains(&pane) {
                    envelope.runtime.engagement_input_by_pane.insert(pane.to_string(), String::new());
                }
                if active_utility != PUZZLE2D_UTILITY_SELECT {
                    self.host.borrow_mut().set_active_utility(PUZZLE2D_UTILITY_SELECT);
                    effects.push(HostEffect::SetActiveUtility { window_id: pane.to_string(), utility_id: PUZZLE2D_UTILITY_SELECT.into() });
                }
                {}
            }
            "engagementControlSelect" => {
                let candidate_id = args.and_then(|value| value.get("id").or_else(|| value.get("value"))).and_then(|value| value.as_str()).unwrap_or("");
                if let Some(index) = candidate_id.strip_prefix("puzzle2d.brush.candidate.").and_then(|rest| rest.parse::<usize>().ok()) {
                    self.host.borrow_mut().brush_set_candidate_index(index);
                    envelope.runtime.brush_candidate_index = index;
                    {}
                } else {
                    {}
                }
            }
            "setLodModeForPane" => {
                let pane = args.and_then(|value| value.get("pane")).and_then(|value| value.as_str()).unwrap_or("");
                let mode = args.and_then(|value| value.get("value")).and_then(|value| value.as_str());
                if let (true, Some(mode)) = (PUZZLE2D_PANES.contains(&pane), mode) {
                    envelope.runtime.lod_mode_by_pane.insert(pane.to_string(), mode.to_string());
                    if pane == PUZZLE2D_PANE_OVERVIEW {
                        if mode == PUZZLE2D_LOD_MODE_AUTOMATIC {
                            self.host.borrow_mut().set_automatic_lod(true);
                        } else {
                            self.host.borrow_mut().set_automatic_lod(false);
                            self.host.borrow_mut().set_forced_draw_lod_label(mode);
                        }
                    }
                    ui_scope = puzzle2d_window_and_measures_scope();
                }
            }
            "setGridSnapEnabled" => {
                let enabled = args.and_then(|value| value.get("enabled")).and_then(|value| value.as_bool()).unwrap_or(false);
                envelope.runtime.grid_snap_enabled = enabled;
                self.host.borrow_mut().set_grid_snap_enabled(enabled);
                ui_scope = puzzle2d_window_and_measures_scope();
            }
            "setGridFactor" => {
                if let Some(value) = args.and_then(|value| value.get("value")).and_then(|value| value.as_f64()) {
                    envelope.runtime.grid_factor = value;
                    let _ = self.host.borrow_mut().set_grid_factor(value);
                    ui_scope = puzzle2d_window_and_measures_scope();
                }
            }
            "setSelectionMethod" => {
                let method = args.and_then(|value| value.get("method")).and_then(|value| value.as_str()).unwrap_or("rectangle");
                envelope.runtime.selection_method = method.into();
                self.host.borrow_mut().set_selection_options(method, "replace", true, true, true);
                ui_scope = puzzle2d_window_only_scope();
            }
            "setBrushKindWeights" => {
                let node_ids = puzzle2d_kind_ids(&envelope.fixture, "nodes");
                let handle_ids = puzzle2d_kind_ids(&envelope.fixture, "handles");
                puzzle2d_ensure_catalog_kind_weights(&mut envelope.runtime.node_kind_weights, &node_ids);
                puzzle2d_ensure_catalog_kind_weights(&mut envelope.runtime.handle_kind_weights, &handle_ids);
                if let Some(weights) = args.and_then(|value| value.get("weights")) {
                    envelope.runtime.node_kind_weights = weights.get("nodeWeights").and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default();
                    envelope.runtime.handle_kind_weights = weights.get("handleWeights").and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default();
                } else if let Some(kind_id) = args.and_then(|value| value.get("kindId")).and_then(|value| value.as_str()) {
                    let weight = args.and_then(|value| value.get("value")).and_then(|value| value.as_f64()).unwrap_or(0.0).clamp(0.0, 1.0);
                    let slice = args.and_then(|value| value.get("catalogSlice")).and_then(|value| value.as_str()).unwrap_or("nodes");
                    if slice == "handles" {
                        envelope.runtime.handle_kind_weights = puzzle2d_normalize_kind_weight_group(&envelope.runtime.handle_kind_weights, &handle_ids, kind_id, weight);
                    } else {
                        envelope.runtime.node_kind_weights = puzzle2d_normalize_kind_weight_group(&envelope.runtime.node_kind_weights, &node_ids, kind_id, weight);
                    }
                }
                if let Ok(weights_json) = serde_json::to_string(&json!({
                    "nodeWeights": envelope.runtime.node_kind_weights,
                    "handleWeights": envelope.runtime.handle_kind_weights,
                })) {
                    self.host.borrow_mut().set_brush_kind_weights(&weights_json);
                }
                ui_scope = puzzle2d_window_and_measures_scope();
            }
            "setBrushNodeSize" => {
                if let Some(size) = args.and_then(|value| value.get("size")).and_then(|value| value.as_f64()) {
                    self.host.borrow_mut().set_brush_node_size(size);
                    ui_scope = puzzle2d_window_only_scope();
                }
            }
            "setSuggestionOffset" => {
                let distance = args.and_then(|value| value.get("distance").or_else(|| value.get("value"))).and_then(|value| value.as_f64());
                if let Some(distance) = distance {
                    let clamped = distance.clamp(PUZZLE2D_SUGGESTION_OFFSET_MIN, PUZZLE2D_SUGGESTION_OFFSET_MAX);
                    envelope.runtime.suggestion_offset = clamped;
                    self.host.borrow_mut().set_suggestion_offset(clamped);
                    ui_scope = puzzle2d_window_and_measures_scope();
                }
            }
            "brushCycleCandidate" => {
                let forward = args.and_then(|value| value.get("forward")).and_then(|value| value.as_bool()).unwrap_or(true);
                self.host.borrow_mut().brush_cycle_candidate(forward);
                envelope.runtime.brush_candidate_index = envelope.runtime.brush_candidate_index.saturating_add(1);
                ui_scope = puzzle2d_window_and_engagements_scope();
            }
            "brushSetCandidateIndex" => {
                if let Some(index) = args.and_then(|value| value.get("index")).and_then(|value| value.as_u64()) {
                    self.host.borrow_mut().brush_set_candidate_index(index as usize);
                    envelope.runtime.brush_candidate_index = index as usize;
                    ui_scope = puzzle2d_window_and_engagements_scope();
                }
            }
            "brushOpenSlot" => {
                if let Some(handle_id) = args.and_then(|value| value.get("handleId")).and_then(|value| value.as_str()) {
                    self.host.borrow_mut().brush_open_slot(handle_id);
                }
                {}
            }
            "brushCommitSlot" => {
                self.host.borrow_mut().brush_commit_slot();
                apply_host_events(&mut self.host.borrow_mut(), &mut envelope);
                {}
            }
            "brushCancelSlot" => {
                self.host.borrow_mut().brush_cancel_slot();
                {}
            }
            "setFillCount" => {
                let count = args.and_then(|value| value.get("count").or_else(|| value.get("value"))).and_then(|value| value.as_f64()).map(|value| value.round().max(0.0) as u32).unwrap_or(0).min(PUZZLE2D_FILL_COUNT_MAX);
                envelope.runtime.fill_count = count;
                effects.push(HostEffect::SetActiveTool { tool_id: PUZZLE2D_UTILITY_FILL.into() });
                self.host.borrow_mut().set_active_utility("brush");
                self.host.borrow_mut().brush_fill_session_begin(count, 1);
                let step = self.host.borrow_mut().brush_fill_session_step(count.max(1));
                if let Ok(progress) = serde_json::from_str::<Value>(&step) {
                    if let Some(placements) = progress.get("placements").and_then(|value| value.as_array()) {
                        for placement in placements {
                            apply_brush_place_payload(&mut envelope.fixture, placement);
                        }
                    }
                }
                {}
            }
            "brushFillSessionBegin" => {
                let max_count = args.and_then(|value| value.get("maxCount")).and_then(|value| value.as_u64()).unwrap_or(0) as u32;
                let seed = args.and_then(|value| value.get("seed")).and_then(|value| value.as_u64()).unwrap_or(1) as u32;
                self.host.borrow_mut().brush_fill_session_begin(max_count, u64::from(seed));
                {}
            }
            "brushFillSessionStep" => {
                let budget = args.and_then(|value| value.get("chunkBudget")).and_then(|value| value.as_u64()).unwrap_or(8) as u32;
                let step = self.host.borrow_mut().brush_fill_session_step(budget);
                if let Ok(progress) = serde_json::from_str::<Value>(&step) {
                    if let Some(placements) = progress.get("placements").and_then(|value| value.as_array()) {
                        for placement in placements {
                            apply_brush_place_payload(&mut envelope.fixture, placement);
                        }
                    }
                }
                {}
            }
            "brushFillSessionClear" => {
                self.host.borrow_mut().brush_fill_session_clear();
                envelope.runtime.fill_count = 0;
                {}
            }
            "patchInspectorNodes" => {
                let ids: Vec<String> = args.and_then(|value| value.get("ids")).and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_else(|| envelope.runtime.selected_ids.clone());
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                let value = args.and_then(|value| value.get("value"));
                let delta = args.and_then(|value| value.get("delta"));
                if !field.is_empty() {
                    patch_inspector_nodes(&mut envelope.fixture, &ids, field, value, delta);
                    {}
                } else {
                    {}
                }
            }
            "forceLayout" | "reorganize" => {
                force_layout_fixture(&mut envelope.fixture);
                {}
            }
            "redrawHandles" => {
                if let Ok(next) = puzzle_2d_engine::apply_edge_handle_snap_to_fixture_v1_json(&envelope.fixture.to_string()) {
                    if let Ok(parsed) = serde_json::from_str(&next) {
                        envelope.fixture = parsed;
                    }
                }
                {}
            }
            "selectAll" => {
                let ids: Vec<String> = fixture_nodes(&envelope.fixture).iter().filter_map(|node| node.get("id").and_then(|value| value.as_str()).map(str::to_string)).collect();
                envelope.runtime.selected_ids = ids.clone();
                self.host.borrow_mut().set_selection_ids(&ids);
                ui_scope = puzzle2d_select_scope();
            }
            "clearSelection" => {
                envelope.runtime.selected_ids.clear();
                self.host.borrow_mut().set_selection_ids(&[]);
                ui_scope = puzzle2d_select_scope();
            }
            "focusSelection" => {
                if envelope.runtime.selected_ids.is_empty() {
                    {}
                } else {
                    let mut min_x = f64::INFINITY;
                    let mut min_y = f64::INFINITY;
                    let mut max_x = f64::NEG_INFINITY;
                    let mut max_y = f64::NEG_INFINITY;
                    for node in fixture_nodes(&envelope.fixture) {
                        let Some(id) = node.get("id").and_then(|value| value.as_str()) else {
                            continue;
                        };
                        if !envelope.runtime.selected_ids.iter().any(|selected| selected == id) {
                            continue;
                        }
                        let x = node.get("x").and_then(|value| value.as_f64()).unwrap_or(0.0);
                        let y = node.get("y").and_then(|value| value.as_f64()).unwrap_or(0.0);
                        let radius = node.get("radius").and_then(|value| value.as_f64()).unwrap_or(24.0);
                        min_x = min_x.min(x - radius);
                        min_y = min_y.min(y - radius);
                        max_x = max_x.max(x + radius);
                        max_y = max_y.max(y + radius);
                    }
                    if min_x.is_finite() {
                        // 🎥️ The camera is session-only runtime state (see `setCamera`'s
                        // `ActionKind::View`) — write it directly, never the fixture.
                        let camera = json!({
                            "x": (min_x + max_x) * 0.5,
                            "y": (min_y + max_y) * 0.5,
                            "zoom": 1.0,
                        });
                        set_runtime_camera(&mut envelope.runtime, &camera);
                        self.host.borrow_mut().set_camera(envelope.runtime.camera_x, envelope.runtime.camera_y, envelope.runtime.camera_zoom);
                        {}
                    } else {
                        {}
                    }
                }
            }
            "applyBoardEvents" => {
                if let Some(events_json) = args.and_then(|value| value.get("eventsJson")).and_then(|value| value.as_str()) {
                    ui_scope = serde_json::from_str::<Vec<Value>>(events_json).map(|events| puzzle2d_board_events_scope(&events)).unwrap_or(semio_framework_core::kernel::UiDirtyScope::Full);
                    apply_board_events_from_json(events_json, &mut envelope);
                    // 🪞️ `apply_host_events` below trusts `host.selection` as the post-action source of
                    // truth and overwrites `envelope.runtime.selected_ids` with it — mirror the new
                    // selection into the host now (as every other selection-setting arm already does)
                    // or the just-applied `select`/`brushCandidates` selection is silently reverted.
                    self.host.borrow_mut().set_selection_ids(&envelope.runtime.selected_ids);
                }
            }
            "lodScaleJson" => {
                let _ = puzzle_2d_lod_scale_json();
                ui_scope = semio_framework_core::kernel::UiDirtyScope::None;
            }
            "setLocale" => {
                if let Some(value) = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()) {
                    envelope.runtime.locale = value.into();
                }
            }
            "setTerminology" => {
                if let Some(value) = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()) {
                    envelope.runtime.terminology = value.into();
                }
            }
            _ => {}
        }
        apply_host_events(&mut self.host.borrow_mut(), &mut envelope);
        let operations = puzzle2d_document_delta_operations(&before, &envelope.fixture);
        // 🐢️ Safety net: a `None` scope claims nothing needs re-rendering — never pair that with an
        // actual document mutation (would silently desync remote clients' UI from the committed operation).
        if !operations.is_empty() && matches!(ui_scope, semio_framework_core::kernel::UiDirtyScope::None) {
            ui_scope = semio_framework_core::kernel::UiDirtyScope::Full;
        }
        // 🧮️ B1: only a REAL config change becomes a `Puzzle2dConfigOperation` — `PartialEq` (derived)
        // makes this cheap, and keeps a pure read-only action from creating a no-op undo entry.
        let config_operations = if &envelope.runtime != config { vec![Puzzle2dConfigOperation::Snapshot { config: envelope.runtime }] } else { Vec::new() };
        Emit { document_operations: operations, config_operations, coalesce_key, effects, ui_scope, ..Default::default() }
    }
}

//#endregion 🔖️Puzzle2dPlayApp

//#region 🔖️Manifest
/// 🛠️ An internal (non-palette) action declaration — the pointer/gesture/inspector/engagement-bound
/// vocabulary dispatched by the canvas/panels, never surfaced as a standalone command palette entry.
fn puzzle2d_internal_action(id: &str, label: impl Into<LocalizedLabel>, kind: ActionKind) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog(id, label, kind) }
}

/// 🧰️ One canvas utility declaration (host-owned active utility). Select/brush/fill are this window's entire
/// top-level exclusive utility set — not a sub-collection — so each carries `group: None` and renders as
/// its own flat utility bar icon (matching the `process` utility bar), never a collapsed dropdown.
fn puzzle2d_utility(id: &str, label: impl Into<LocalizedLabel>, icon: &str, category: UtilityCategory) -> UtilityDefinition {
    UtilityDefinition { category: Some(category), ..UtilityDefinition::new(id, label, icon) }
}

pub fn create_puzzle2d_app() -> App {
    let mut host = puzzle_board_host();
    let envelope = Puzzle2dScene { fixture: default_empty_fixture(), runtime: Puzzle2dPlayRuntime::default(), active_utility: PUZZLE2D_UTILITY_SELECT.into() };
    sync_host_from_envelope(&mut host, &envelope);
    let labels = puzzle2d_labels(&Puzzle2dConfig::default());
    let mut app = App::from_builder(
        App::builder(PUZZLE2D_PLAY_APP_ID, LocalizedLabel::native("Puzzle 2D", "Puzzle 2D"))
            .document(["semio", "puzzle", "2d"])
            .artifact_kind(ArtifactKindSpec {
                id: "2d.puzzle".into(),
                name: "2D Puzzle".into(),
                source_format: "puzzle.2d".into(),
                component_kind: "puzzle2d".into(),
                dimension: "2d".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Design },
                schema: "puzzle.2d".into(),
                export_formats: vec![OsMediaFormat::Svg, OsMediaFormat::Png],
                import_formats: vec![OsMediaFormat::Svg, OsMediaFormat::Png],
            })
            .icon_id("puzzle")
            .terminology("reuse")
            .terminology_document("reuse", ["Entwerfen mit Bestand", "puzzle", "2d"])
            .mode("edit", LocalizedLabel::native("Edit", "Bearbeiten"), "pencil")
            .default_mode_id("edit")
            .window_kind_with_engagement(PUZZLE2D_PANE_OVERVIEW, puzzle2d_localized(|l| l.window_overview), PUZZLE2D_PLAY_BODY_OVERVIEW, SurfaceKind::Canvas2d, puzzle2d_engagement(&envelope, &host, PUZZLE2D_PANE_OVERVIEW, labels), "layout-grid")
            .window_kind_with_engagement(PUZZLE2D_PANE_DETAIL, puzzle2d_localized(|l| l.window_detail), PUZZLE2D_PLAY_BODY_DETAIL, SurfaceKind::Canvas2d, puzzle2d_engagement(&envelope, &host, PUZZLE2D_PANE_DETAIL, labels), "focus")
            .window_kind_with_engagement(PUZZLE2D_PANE_SELECTION, puzzle2d_localized(|l| l.window_selection), PUZZLE2D_PLAY_BODY_SELECTION, SurfaceKind::Canvas2d, puzzle2d_engagement(&envelope, &host, PUZZLE2D_PANE_SELECTION, labels), "crosshair")
            .panel_tab("framework.panel.document", LocalizedLabel::native(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, "Dokument"), PanelGroup::Workbench, PUZZLE2D_PLAY_BODY_LAYERS)
            .panel_tab("framework.panel.catalogue", LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"), PanelGroup::Workbench, PUZZLE2D_PLAY_BODY_CATALOGUE)
            .panel_tab("framework.panel.inspection", LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"), PanelGroup::Details, PUZZLE2D_PLAY_BODY_PROPERTIES)
            // ✏️ Palette-visible content operations.
            .operation("addNode", LocalizedLabel::native("Add Node", "Knoten hinzufügen"))
            .operation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
            // 🗂️ Referenced by `puzzle2d_context_menu_items` — categorized for grouped-context-menu disclosure.
            .action_with(ActionDefinition::new_catalog("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"), ActionKind::Operation).with_category("selection"))
            .keybinding("delete,backspace", "deleteSelection")
            .action_with(ActionDefinition::new_catalog("duplicateSelection", LocalizedLabel::native("Duplicate Selection", "Auswahl duplizieren"), ActionKind::Operation).with_category("create"))
            .operation("forceLayout", LocalizedLabel::native("Force Layout", "Kraftbasiertes Layout"))
            .action_with(ActionDefinition::new_catalog("focusSelection", LocalizedLabel::native("Focus Selection", "Auswahl fokussieren"), ActionKind::Operation).with_category("view"))
            // 👁️ Palette-visible ephemeral view/selection commands.
            .action_with(ActionDefinition::new_catalog("selectAll", LocalizedLabel::native("Select All", "Alles auswählen"), ActionKind::View).with_category("selection"))
            .view_action("clearSelection", LocalizedLabel::native("Clear Selection", "Auswahl aufheben"))
            .action_with(ActionDefinition::new_catalog("selectSameKind", LocalizedLabel::native("Select Same Kind", "Gleiche Art auswählen"), ActionKind::View).with_category("selection"))
            // 🔧️ Internal content operations — inspector/panel/board/import-bound, not palette commands.
            // 🗂️ Referenced by `puzzle2d_context_menu_items` (toggleHidden/toggleLocked rows) — categorized for grouped-context-menu disclosure.
            .action_with(puzzle2d_internal_action("setSelectionFlag", LocalizedLabel::native("Set Selection Flag", "Auswahlmarkierung festlegen"), ActionKind::Operation).with_category("settings"))
            .action_with(puzzle2d_internal_action("patchInspectorNodes", LocalizedLabel::native("Patch Inspector Nodes", "Inspektorknoten aktualisieren"), ActionKind::Operation))
            .action_with(puzzle2d_internal_action("redrawHandles", LocalizedLabel::native("Redraw Handles", "Anschlüsse neu zeichnen"), ActionKind::Operation))
            .action_with(puzzle2d_internal_action("reorganize", LocalizedLabel::native("Reorganize", "Neu anordnen"), ActionKind::Operation))
            .action_with(puzzle2d_internal_action("applyBoardEvents", LocalizedLabel::native("Apply Board Events", "Board-Ereignisse anwenden"), ActionKind::Operation))
            .action_with(puzzle2d_internal_action("setFillCount", LocalizedLabel::native("Set Fill Count", "Füllanzahl festlegen"), ActionKind::Operation))
            .action_with(puzzle2d_internal_action("brushFillSessionStep", LocalizedLabel::native("Brush Fill Session Step", "Pinsel-Füllsitzung-Schritt"), ActionKind::Operation))
            .action_with(puzzle2d_internal_action("brushCommitSlot", LocalizedLabel::native("Brush Commit Slot", "Pinsel-Platz übernehmen"), ActionKind::Operation))
            // 🖱️ Internal pointer/gesture/engagement view vocabulary — pure runtime/host state, emit no operations.
            // 🎥️ `setCamera` is session-only view state (never a VCS edit — see `Puzzle2dPlayRuntime`'s
            // camera fields), so it belongs in this View-kind group, not the operations above.
            .action_with(puzzle2d_internal_action("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("setSelection", LocalizedLabel::native("Set Selection", "Auswahl festlegen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("documentSelect", LocalizedLabel::native("Document Select", "Dokument auswählen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("engagementInput", LocalizedLabel::native("Engagement Input", "Eingabe"), ActionKind::View))
            .action_with(puzzle2d_internal_action("engagementSubmit", LocalizedLabel::native("Engagement Submit", "Eingabe bestätigen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("engagementAbort", LocalizedLabel::native("Engagement Abort", "Eingabe abbrechen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("engagementControlSelect", LocalizedLabel::native("Engagement Control Select", "Eingabesteuerung auswählen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("setLodModeForPane", LocalizedLabel::native("Set LOD Mode For Pane", "LOD-Modus für Bereich festlegen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("setGridSnapEnabled", LocalizedLabel::native("Set Grid Snap Enabled", "Rasterfang aktivieren"), ActionKind::View))
            .action_with(puzzle2d_internal_action("setGridFactor", LocalizedLabel::native("Set Grid Factor", "Rasterfaktor festlegen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("setSelectionMethod", LocalizedLabel::native("Set Selection Method", "Auswahlmethode festlegen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("setBrushKindWeights", LocalizedLabel::native("Set Brush Kind Weights", "Pinsel-Artgewichte festlegen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("setBrushNodeSize", LocalizedLabel::native("Set Brush Node Size", "Pinsel-Knotengröße festlegen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("setSuggestionOffset", LocalizedLabel::native("Set Suggestion Offset", "Vorschlagsversatz festlegen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("brushCycleCandidate", LocalizedLabel::native("Brush Cycle Candidate", "Pinselkandidat wechseln"), ActionKind::View))
            .action_with(puzzle2d_internal_action("brushSetCandidateIndex", LocalizedLabel::native("Brush Set Candidate Index", "Pinselkandidatenindex festlegen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("brushOpenSlot", LocalizedLabel::native("Brush Open Slot", "Pinsel-Platz öffnen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("brushCancelSlot", LocalizedLabel::native("Brush Cancel Slot", "Pinsel-Platz abbrechen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("brushFillSessionBegin", LocalizedLabel::native("Brush Fill Session Begin", "Pinsel-Füllsitzung beginnen"), ActionKind::View))
            .action_with(puzzle2d_internal_action("brushFillSessionClear", LocalizedLabel::native("Brush Fill Session Clear", "Pinsel-Füllsitzung leeren"), ActionKind::View))
            .action_with(puzzle2d_internal_action("lodScaleJson", LocalizedLabel::native("LOD Scale Json", "LOD-Skalierung-Json"), ActionKind::View))
            // 📝️ Staged palette args for the two content commands that need a target.
            .action_args("addNode", vec![
                ActionArgDef::select("kind", LocalizedLabel::native("Kind", "Art"), vec![ActionArgOption::new("node", LocalizedLabel::native("Node", "Knoten"))]).required().default_value("node"),
            ])
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![
                    ActionArgOption::new(PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID, puzzle2d_localized(|l| l.example_concrete_forest)),
                    ActionArgOption::new(PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID, LocalizedLabel::native("Nakagin Capsule Tower", "Nakagin Capsule Tower")),
                ]).required().default_value(PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID),
            ])
            // 🧰️ Canvas utilities — one exclusive set, active utility host-owned (never a document operation). The
            // select/brush switcher is rendered by the framework utility bar for the interactive pane.
            .utility(puzzle2d_utility(PUZZLE2D_UTILITY_SELECT, puzzle2d_localized(|l| l.select), "mouse-pointer", UtilityCategory::Selection))
            .utility(puzzle2d_utility(PUZZLE2D_UTILITY_BRUSH, puzzle2d_localized(|l| l.brush), "paintbrush", UtilityCategory::Utilities))
            .window_kind_utilities(PUZZLE2D_PANE_OVERVIEW, vec![PUZZLE2D_UTILITY_SELECT.into(), PUZZLE2D_UTILITY_BRUSH.into()])
            // 🛠️ Fill is a mode-level tool (a whole-document generator), not a window utility.
            .tool_simple(PUZZLE2D_UTILITY_FILL, puzzle2d_localized(|l| l.fill), "paint-bucket")
            .mode_tools("edit", vec![ToolRef::new(PUZZLE2D_UTILITY_FILL)])
            .default_layout(create_default_layout(&[PUZZLE2D_PANE_OVERVIEW.into(), PUZZLE2D_PANE_DETAIL.into(), PUZZLE2D_PANE_SELECTION.into()], "row", Some(&[50.0, 25.0, 25.0]), Some(&["Overview".into(), "Detail".into(), "Selection".into()]))),
    );
    for pane in PUZZLE2D_PANES {
        if let Some(window) = app.definition.window_kinds.iter_mut().find(|window| window.id == pane) {
            window.options.measures = puzzle2d_window_measures(pane, &envelope, labels);
        }
    }
    app.example(PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID, puzzle2d_localized(|l| l.example_concrete_forest), serde_json::to_string(&example_fixture(CONCRETE_FOREST_EXAMPLE_JSON.as_str())).unwrap(), "list-tree")
        .example(PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID, LocalizedLabel::native("Nakagin Capsule Tower", "Nakagin Capsule Tower"), serde_json::to_string(&example_fixture(NAKAGIN_EXAMPLE_JSON.as_str())).unwrap(), "building")
        .workflow("puzzle2d", "Puzzle 2D", "layout")
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn puzzle2d_document_json_to_svg(value: &Value) -> Result<(String, u32, u32), String> {
    semio_framework_os::title_card_svg(value, "Puzzle 2D", 1024, 768)
}

/// 📥️ Tier C DWG import — the puzzle-2d fixture only supports circle/rectangle nodes (no polygonal
/// outlines), so this always returns an empty board; never errors on a structurally valid DWG.
/// The DWG's extents no longer frame a camera here: the camera is session-only `Puzzle2dPlayRuntime`
/// state (see `setCamera`'s `ActionKind::View`), and this import path produces a bare document with
/// no live app instance to receive that runtime write.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn puzzle2d_document_json_from_dwg(_drawing: &semio_framework_core::DwgDrawing) -> Result<Value, String> {
    Ok(default_empty_fixture())
}

pub fn register_puzzle2d_exports() {
    // 🗂️ Registers `Puzzle2dPlayProjection`'s pack<->dsl codec under its real `document_schema()`
    // string so `framework/sync`'s `FolderEndpoint::Pack` can print/parse puzzle-2d play documents
    // without depending on this crate's concrete `Projection`/`Operation` types.
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<Puzzle2dPlayApp>(PUZZLE2D_FIXTURE_SCHEMA);
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    {
        semio_framework_os::register_2d_export_handlers("2d.puzzle", "puzzle2d", puzzle2d_document_json_to_svg);
        semio_framework_os::register_dwg_import_handler("2d.puzzle", puzzle2d_document_json_from_dwg);
    }
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{testkit, PluginApp, VcsDocumentApp, FRAMEWORK_HISTORY_BODY_KEY};
    use store::{Backbone, BackboneMessage, MemoryBackbone};

    /// 🧰️ A registry-backed app so kind discipline (View/Shell actions must emit no operations) and the utility
    /// contract are enforced exactly as in production (`VcsDocumentApp::with_registry`).
    fn registry_app() -> VcsDocumentApp<Puzzle2dPlayApp> {
        testkit::new_app_with_registry::<Puzzle2dPlayApp>(create_puzzle2d_app)
    }

    /// 🧪️ B1: test-only replacement for the deleted `VcsDocumentApp::handle_action` app-dispatch path
    /// (that method is FRAMEWORK-reserved now — see its doc comment in `semio_framework_plugin`; an
    /// app's own actions go exclusively through the typed `Self::Command` channel). Reconstructs the
    /// `Puzzle2dCommand` from the same `(action, args, window_id)` triple every pre-B1 test already
    /// passed and dispatches it via `VcsDocumentApp::dispatch_typed`.
    fn dispatch_action(app: &mut VcsDocumentApp<Puzzle2dPlayApp>, action: &str, args: Option<&Value>, window_id: Option<&str>, meta: &semio_framework_plugin::ActionMeta) -> Result<semio_framework_plugin::InvocationResult, String> {
        // 🕰️ Framework-reserved verbs (undo/redo/checkpoint/…) stay on `handle_action` — B1 keeps that
        // path FRAMEWORK-only, an app's own actions go through the typed `Self::Command` channel below.
        if matches!(action, "undo" | "redo" | "commitCheckpoint" | "createAlternative" | "switchAlternative" | "checkoutCheckpoint" | "copy" | "cut" | "paste" | "revertToCommand" | "historyFilter" | "noteShellCommand") {
            return app.handle_action(action, args, meta);
        }
        app.dispatch_typed(Puzzle2dCommand::from_action(action, args.cloned(), window_id.map(str::to_string)), meta)
    }

    fn concrete_forest_app() -> VcsDocumentApp<Puzzle2dPlayApp> {
        let mut app = testkit::new_app::<Puzzle2dPlayApp>();
        dispatch_action(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID })), None, &testkit::meta("local")).expect("load concrete forest");
        app
    }

    /// 🎥️ Recovers the rendered pane camera `(x, y, zoom)` from a rendered `UiNode`'s embedded
    /// `Board2dScene.cameraJson` — the only externally observable surface for the runtime camera
    /// (see `setCamera`'s `ActionKind::View`: the camera is never a document field, so it cannot be
    /// read back off `app.projection()`).
    fn rendered_camera(rendered: &UiNode) -> (f64, f64, f64) {
        fn find_camera_json(value: &Value) -> Option<String> {
            if let Some(json) = value.get("cameraJson").and_then(Value::as_str) {
                return Some(json.to_string());
            }
            match value {
                Value::Object(map) => map.values().find_map(find_camera_json),
                Value::Array(items) => items.iter().find_map(find_camera_json),
                _ => None,
            }
        }
        let value = serde_json::to_value(rendered).expect("serialize rendered node");
        let camera_json = find_camera_json(&value).expect("rendered scene must carry cameraJson");
        let camera: Value = serde_json::from_str(&camera_json).expect("cameraJson parses");
        (camera.get("x").and_then(Value::as_f64).unwrap_or(f64::NAN), camera.get("y").and_then(Value::as_f64).unwrap_or(f64::NAN), camera.get("zoom").and_then(Value::as_f64).unwrap_or(f64::NAN))
    }

    #[test]
    fn renders_puzzle2d_board_scene() {
        let mut app = testkit::new_app::<Puzzle2dPlayApp>();
        let node = app.render(PUZZLE2D_PLAY_BODY_OVERVIEW, None, &ViewState::default()).expect("render");
        assert!(serde_json::to_string(&node).unwrap().contains("board-2d"));
    }

    #[test]
    fn add_node_action_emits_upsert_op_and_appends_node() {
        let mut app = testkit::new_app::<Puzzle2dPlayApp>();
        let result = dispatch_action(&mut app, "addNode", Some(&json!({ "kind": "node" })), None, &testkit::meta("local")).expect("add node");
        assert_eq!(result.operations.len(), 1, "addNode must emit exactly one granular operation");
        assert_eq!(fixture_nodes(&app.projection().expect("projection").0).len(), 1);
    }

    #[test]
    fn set_active_example_loads_concrete_forest_via_operations() {
        let mut app = testkit::new_app::<Puzzle2dPlayApp>();
        dispatch_action(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID })), None, &testkit::meta("local")).expect("load example");
        assert!(!fixture_nodes(&app.projection().expect("projection").0).is_empty());
    }

    /// 📦️ `Puzzle2dPlayProjection`'s pack encoding round-trips through the same `(RecordSpec,
    /// RecordValue)` pair its `parse_dsl`/`print_dsl` do (both delegate to the underlying
    /// `serde_json::Value` bridge impls), reusing the same concrete-forest fixture the DSL-facing
    /// tests above already construct.
    #[test]
    fn puzzle2d_play_projection_pack_round_trips() {
        let app = concrete_forest_app();
        store::test_support::assert_dsl_pack_equivalence(&app.projection().expect("projection"));
    }

    #[test]
    fn select_then_delete_selection_removes_the_node() {
        let mut app = testkit::new_app::<Puzzle2dPlayApp>();
        dispatch_action(&mut app, "addNode", Some(&json!({ "kind": "node" })), None, &testkit::meta("local")).expect("add node");
        let node_id = fixture_nodes(&app.projection().expect("projection").0)[0].get("id").and_then(|value| value.as_str()).unwrap().to_string();
        dispatch_action(&mut app, "setSelection", Some(&json!({ "ids": [node_id] })), None, &testkit::meta("local")).expect("select");
        dispatch_action(&mut app, "deleteSelection", None, None, &testkit::meta("local")).expect("delete");
        assert!(fixture_nodes(&app.projection().expect("projection").0).is_empty());
    }

    #[test]
    fn undo_redo_round_trip_through_the_wrapper() {
        let mut app = testkit::new_app::<Puzzle2dPlayApp>();
        dispatch_action(&mut app, "addNode", Some(&json!({ "kind": "node" })), None, &testkit::meta("local")).expect("add");
        assert_eq!(fixture_nodes(&app.projection().expect("projection").0).len(), 1);
        dispatch_action(&mut app, "undo", None, None, &testkit::meta("local")).expect("undo");
        assert_eq!(fixture_nodes(&app.projection().expect("projection").0).len(), 0);
        dispatch_action(&mut app, "redo", None, None, &testkit::meta("local")).expect("redo");
        assert_eq!(fixture_nodes(&app.projection().expect("projection").0).len(), 1);
    }

    /// 🎥️ `setCamera` is session-only view state (see `ActionKind::View`): a camera drag never
    /// creates a VCS edit, so there is nothing to coalesce and nothing for `undo` to revert.
    #[test]
    fn set_camera_is_session_only_and_never_undoable() {
        let mut app = testkit::new_app::<Puzzle2dPlayApp>();
        for x in [1.0, 2.0, 3.0] {
            let result = dispatch_action(&mut app, "setCamera", Some(&json!({ "camera": { "x": x, "y": 0.0, "zoom": 1.0 } })), None, &testkit::meta("local")).expect("camera");
            assert!(result.operations.is_empty(), "setCamera must never produce a document operation");
        }
        let rendered = app.render(PUZZLE2D_PLAY_BODY_OVERVIEW, None, &ViewState::default()).expect("render");
        assert_eq!(rendered_camera(&rendered).0, 3.0, "the camera must update immediately in the rendered scene");
        let undo = dispatch_action(&mut app, "undo", None, None, &testkit::meta("local")).expect("undo");
        assert!(undo.operations.is_empty(), "there is no document edit to undo");
        let rendered_after_undo = app.render(PUZZLE2D_PLAY_BODY_OVERVIEW, None, &ViewState::default()).expect("render");
        assert_eq!(rendered_camera(&rendered_after_undo).0, 3.0, "the camera is session state — undo must not revert it");
    }

    /// 🐢️ Regression test for a perf-round-2 bug: `sync_host_from_envelope`'s `parse_fixture_v1`
    /// always `clear_scene()`s then rebuilds, so every edge looked "new" and got re-`push_event`'d
    /// as `edgeCreate` — which `apply_host_events` then replayed into `envelope.fixture.edges` on
    /// the *next* action, duplicating every edge once per action forever. Repeated no-operation actions
    /// (here: repeated selects) must leave the edge count untouched.
    #[test]
    fn repeated_actions_do_not_duplicate_edges() {
        let mut app = testkit::new_app::<Puzzle2dPlayApp>();
        dispatch_action(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID })), None, &testkit::meta("local")).expect("load nakagin");
        let edge_count = |app: &VcsDocumentApp<Puzzle2dPlayApp>| fixture_edges(&app.projection().expect("projection").0).len();
        let before = edge_count(&app);
        assert!(before > 0, "fixture must have edges for this regression test to be meaningful");
        let node_id = fixture_nodes(&app.projection().expect("projection").0)[0].get("id").and_then(|value| value.as_str()).unwrap().to_string();
        for _ in 0..5 {
            dispatch_action(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": json!([{ "name": "select", "payload": { "ids": [node_id] } }]).to_string() })), None, &testkit::meta("local")).expect("select");
        }
        assert_eq!(edge_count(&app), before, "selecting repeatedly must not grow the edges array");
    }

    /// 🪞️ Regression test: `applyBoardEvents`'s `select` case only mutated `envelope.runtime`, never
    /// `self.host`, so `apply_host_events`'s `host.selection`-is-truth re-sync silently reverted the
    /// selection to whatever `self.host` held before the action (empty, on a fresh sync).
    #[test]
    fn apply_board_events_select_persists_across_the_next_action() {
        let mut app = concrete_forest_app();
        let node_id = fixture_nodes(&app.projection().expect("projection").0)[0].get("id").and_then(|value| value.as_str()).unwrap().to_string();
        dispatch_action(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": json!([{ "name": "select", "payload": { "ids": [node_id] } }]).to_string() })), None, &testkit::meta("local")).expect("select");
        let rendered_once = serde_json::to_string(&app.render(PUZZLE2D_PLAY_BODY_OVERVIEW, None, &ViewState::default()).expect("render")).unwrap();
        assert!(rendered_once.contains(&node_id), "selection must be visible immediately after the select action");
        // A second, unrelated action used to silently clear the selection via the stale `host.selection` re-sync.
        dispatch_action(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": "[]" })), None, &testkit::meta("local")).expect("no-operation");
        let rendered_twice = serde_json::to_string(&app.render(PUZZLE2D_PLAY_BODY_OVERVIEW, None, &ViewState::default()).expect("render")).unwrap();
        assert!(rendered_twice.contains(&node_id), "selection must survive a subsequent unrelated action");
    }

    /// 🪞️ Regression test: `apply_host_events` used to epsilon-compare `host.camera` (still the
    /// *pre-action* value) against the runtime and blindly overwrite it, reverting a plain `camera`
    /// board event (used for the live wheel-zoom echo) before it ever committed. The camera is
    /// session-only `Puzzle2dPlayRuntime` state (see `setCamera`'s `ActionKind::View`) — it must
    /// commit into the rendered scene and never produce a document operation.
    #[test]
    fn apply_board_events_camera_event_commits() {
        let mut app = testkit::new_app::<Puzzle2dPlayApp>();
        let result = dispatch_action(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": json!([{ "name": "camera", "payload": { "x": 5.0, "y": 6.0, "zoom": 1.2 } }]).to_string() })), None, &testkit::meta("local")).expect("camera event");
        assert!(result.operations.is_empty(), "a camera board event must never produce a document operation");
        let rendered = app.render(PUZZLE2D_PLAY_BODY_OVERVIEW, None, &ViewState::default()).expect("render");
        let (x, y, zoom) = rendered_camera(&rendered);
        assert_eq!(x, 5.0);
        assert_eq!(y, 6.0);
        assert_eq!(zoom, 1.2);
    }

    /// 🐢️ A pure selection change is runtime state, not document state — it must not produce any
    /// `KernelOperation`s (previously it fell back to a whole-document `ReplaceDocument` once the
    /// edge-duplication bug made `before` and `after` genuinely diverge).
    #[test]
    fn select_action_emits_no_operations() {
        let mut app = concrete_forest_app();
        let node_id = fixture_nodes(&app.projection().expect("projection").0)[0].get("id").and_then(|value| value.as_str()).unwrap().to_string();
        let result = dispatch_action(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": json!([{ "name": "select", "payload": { "ids": [node_id] } }]).to_string() })), None, &testkit::meta("local")).expect("select");
        assert!(result.operations.is_empty(), "selection must not produce document operations");
    }

    /// 🐢️ Perf round 3: a select event must declare a narrow `Partial` ui_scope (the 3 canvas panes +
    /// layers/properties panels + engagements) — never `Full`, or the shell's batched `refresh-ui`
    /// call degrades back to fetching everything on every select.
    #[test]
    fn select_action_declares_partial_ui_scope() {
        use semio_framework_core::kernel::UiDirtyScope;
        let mut app = concrete_forest_app();
        let node_id = fixture_nodes(&app.projection().expect("projection").0)[0].get("id").and_then(|value| value.as_str()).unwrap().to_string();
        let result = dispatch_action(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": json!([{ "name": "select", "payload": { "ids": [node_id] } }]).to_string() })), None, &testkit::meta("local")).expect("select");
        match result.ui_scope {
            UiDirtyScope::Partial { window_bodies, panel_bodies, engagements, measures, utilities, tools, labels } => {
                // 🐢️ Regression: `window_bodies` must list the window *body keys* (matched against
                // `AppDefinition.windowKinds[].bodyKey` by the shell's `buildUiRefreshRequest`), not
                // the pane/kind-id constants (`PUZZLE2D_PANES`) — those are a different id space.
                assert_eq!(window_bodies, vec![PUZZLE2D_PLAY_BODY_OVERVIEW, PUZZLE2D_PLAY_BODY_DETAIL, PUZZLE2D_PLAY_BODY_SELECTION], "window_bodies must be body keys, not pane ids");
                assert!(panel_bodies.contains(&PUZZLE2D_PLAY_BODY_LAYERS.to_string()));
                assert!(panel_bodies.contains(&PUZZLE2D_PLAY_BODY_PROPERTIES.to_string()));
                assert!(engagements, "select must refresh the engagement bar");
                assert!(!measures, "select must not force a measures refresh");
                assert!(!utilities);
                assert!(!tools);
                assert!(!labels);
            }
            other => panic!("expected a Partial ui_scope for select, got {other:?}"),
        }
    }

    /// 🐢️ Perf round 3: a camera-only board event touches only the 3 canvas panes — no panels,
    /// engagements, measures, or utilities.
    #[test]
    fn camera_event_declares_window_only_ui_scope() {
        use semio_framework_core::kernel::UiDirtyScope;
        let mut app = testkit::new_app::<Puzzle2dPlayApp>();
        let result = dispatch_action(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": json!([{ "name": "camera", "payload": { "x": 1.0, "y": 2.0, "zoom": 1.0 } }]).to_string() })), None, &testkit::meta("local")).expect("camera event");
        match result.ui_scope {
            UiDirtyScope::Partial { window_bodies, panel_bodies, engagements, measures, utilities, tools, labels } => {
                assert_eq!(window_bodies.len(), 3);
                assert_eq!(panel_bodies, vec![FRAMEWORK_HISTORY_BODY_KEY.to_string()], "window-only scope still gains the history panel body");
                assert!(!engagements && !measures && !utilities && !tools && !labels);
            }
            other => panic!("expected a Partial ui_scope for a camera event, got {other:?}"),
        }
    }

    /// 🐢️ Perf round 3: an empty `applyBoardEvents` batch (no-operation) must declare nothing beyond the
    /// history panel body — the View action still logs a command-history entry, but no board surface is dirtied.
    #[test]
    fn empty_board_events_declare_none_ui_scope() {
        use semio_framework_core::kernel::UiDirtyScope;
        let mut app = testkit::new_app::<Puzzle2dPlayApp>();
        let result = dispatch_action(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": "[]" })), None, &testkit::meta("local")).expect("no-operation");
        match result.ui_scope {
            UiDirtyScope::Partial { window_bodies, panel_bodies, engagements, measures, utilities, tools, labels } => {
                assert!(window_bodies.is_empty(), "empty board events must not dirty any window body");
                assert_eq!(panel_bodies, vec![FRAMEWORK_HISTORY_BODY_KEY.to_string()]);
                assert!(!engagements && !measures && !utilities && !tools && !labels);
            }
            other => panic!("empty board events must declare only the history panel body, got {other:?}"),
        }
    }

    /// 🐢️ Perf round 3: cold-tier structural actions (document operations) must keep the safe `Full`
    /// default — no puzzle2d scope helper narrows them.
    #[test]
    fn add_node_action_declares_full_ui_scope() {
        use semio_framework_core::kernel::UiDirtyScope;
        let mut app = testkit::new_app::<Puzzle2dPlayApp>();
        let result = dispatch_action(&mut app, "addNode", Some(&json!({ "kind": "node" })), None, &testkit::meta("local")).expect("add node");
        assert!(matches!(result.ui_scope, UiDirtyScope::Full), "addNode must stay Full, got {:?}", result.ui_scope);
    }

    #[test]
    fn document_panel_lists_nodes_section() {
        let mut app = concrete_forest_app();
        let json = serde_json::to_string(&app.render(PUZZLE2D_PLAY_BODY_LAYERS, None, &ViewState::default()).expect("render")).unwrap();
        assert!(json.contains("puzzle2d-play-document.nodes"));
        assert!(json.contains("seed-left-001"));
    }

    /// 🗣️ B1: locale/terminology are now real VCS'd `Puzzle2dConfig` state (was a per-call `ViewState`
    /// override) — dispatch `setLocale`/`setTerminology` to change them, then render.
    #[test]
    fn labels_resolve_native_english_and_german_and_reuse() {
        let mut app = concrete_forest_app();
        let english = serde_json::to_string(&app.render(PUZZLE2D_PLAY_BODY_LAYERS, None, &ViewState::default()).expect("render")).unwrap();
        assert!(english.contains("\"Nodes\"") && english.contains("\"Edges\""));
        dispatch_action(&mut app, "setLocale", Some(&json!({ "value": "de" })), None, &testkit::meta("local")).expect("setLocale");
        let german = serde_json::to_string(&app.render(PUZZLE2D_PLAY_BODY_LAYERS, None, &ViewState::default()).expect("render")).unwrap();
        assert!(german.contains("\"Knoten\"") && german.contains("\"Kanten\""));
        dispatch_action(&mut app, "setLocale", Some(&json!({ "value": "en" })), None, &testkit::meta("local")).expect("setLocale");
        dispatch_action(&mut app, "setTerminology", Some(&json!({ "value": "reuse" })), None, &testkit::meta("local")).expect("setTerminology");
        let reuse = serde_json::to_string(&app.render(PUZZLE2D_PLAY_BODY_LAYERS, None, &ViewState::default()).expect("render")).unwrap();
        assert!(reuse.contains("Building components"));
    }

    #[test]
    fn app_definition_has_three_lod_pane_window_kinds() {
        let app = create_puzzle2d_app();
        let ids: Vec<&str> = app.definition.window_kinds.iter().map(|window| window.id.as_str()).collect();
        assert_eq!(ids, vec![PUZZLE2D_PANE_OVERVIEW, PUZZLE2D_PANE_DETAIL, PUZZLE2D_PANE_SELECTION]);
        for window in &app.definition.window_kinds {
            assert!(window.options.engagement.as_option().is_some(), "pane {} must have engagement", window.id);
            assert!(!window.options.measures.is_empty(), "pane {} must have LOD/suggestion measures", window.id);
        }
    }

    /// 🎥️ The camera is session-only runtime state, never a document field — a DWG import (which has
    /// no live app instance to receive a runtime write) must produce a bare empty board with no
    /// `"camera"` key at all, regardless of the drawing's extents.
    #[test]
    fn dwg_import_returns_empty_board_with_no_camera_field() {
        let mut drawing = semio_framework_core::DwgDrawing::default();
        drawing.extmin = [0.0, 0.0, 0.0];
        drawing.extmax = [100.0, 200.0, 0.0];
        let fixture = puzzle2d_document_json_from_dwg(&drawing).unwrap();
        assert_eq!(fixture.get("schema").and_then(|value| value.as_str()), Some(PUZZLE2D_FIXTURE_SCHEMA));
        assert!(fixture_nodes(&fixture).is_empty());
        assert!(fixture.get("camera").is_none(), "the document must never carry a camera field");
    }

    /// 🧪️ Definitional convergence proof: two instances on one backbone make DISJOINT node edits
    /// (each adds its own node) and, after exchanging operations, both converge to contain BOTH nodes —
    /// impossible under whole-document `setDocument` snapshots, which would clobber one side.
    #[test]
    fn two_instances_converge_disjoint_node_edits_via_backbone() {
        let mut instance_a = testkit::new_app::<Puzzle2dPlayApp>();
        let mut instance_b = testkit::new_app::<Puzzle2dPlayApp>();
        let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://puzzle2d-convergence", "mem://puzzle2d-convergence");
        instance_a.attach_backbone(Box::new(backbone_a)).expect("attach a");
        instance_b.attach_backbone(Box::new(backbone_b)).expect("attach b");

        dispatch_action(&mut instance_a, "addNode", Some(&json!({ "kind": "seed" })), None, &testkit::meta("actor-a")).expect("a adds node");
        dispatch_action(&mut instance_b, "addNode", Some(&json!({ "kind": "other" })), None, &testkit::meta("actor-b")).expect("b adds node");

        // A neutral history action always calls store.dispatch(), which pumps inbound operations first.
        dispatch_action(&mut instance_a, "commitCheckpoint", None, None, &testkit::meta("actor-a")).expect("pump a");
        dispatch_action(&mut instance_b, "commitCheckpoint", None, None, &testkit::meta("actor-b")).expect("pump b");

        assert_eq!(fixture_nodes(&instance_a.projection().expect("projection").0).len(), 2, "instance A must contain both nodes");
        assert_eq!(fixture_nodes(&instance_b.projection().expect("projection").0).len(), 2, "instance B must contain both nodes");
    }

    #[test]
    fn ingest_operations_is_idempotent() {
        let mut sender = testkit::new_app::<Puzzle2dPlayApp>();
        let (near, mut far) = MemoryBackbone::pair("mem://puzzle2d-doc", "mem://puzzle2d-doc");
        sender.attach_backbone(Box::new(near)).expect("attach");
        dispatch_action(&mut sender, "addNode", Some(&json!({ "kind": "seed" })), None, &testkit::meta("local")).expect("add");

        let mut envelopes = Vec::new();
        for message in far.receive().expect("receive") {
            if let BackboneMessage::Operations { envelopes: operations } = message {
                envelopes.extend(operations);
            }
        }
        assert!(!envelopes.is_empty(), "the applied operation must flow onto the channel");
        let operations = protocol::encode_envelopes(&envelopes);

        let mut receiver = testkit::new_app::<Puzzle2dPlayApp>();
        receiver.ingest_operations(&operations).expect("ingest once");
        receiver.ingest_operations(&operations).expect("ingest twice");
        assert_eq!(fixture_nodes(&receiver.projection().expect("projection").0).len(), 1, "feeding the same operation twice must not double-apply");
    }

    /// 🧰️ B1: `setActiveUtility` is a real typed `Puzzle2dCommand` now (was a host-applied `ViewState`
    /// notification): switching utilities must still emit no DOCUMENT operations — the new value lands
    /// in `Puzzle2dConfig::active_utility_by_window_id` as a config operation instead, never re-emits a
    /// `SetActiveUtility` effect (the caller already knows), and the document's own undo stack is
    /// untouched by it.
    #[test]
    fn utility_switch_emits_no_ops_and_no_history() {
        let mut app = registry_app();
        let result = dispatch_action(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": PUZZLE2D_UTILITY_BRUSH })), Some(PUZZLE2D_PANE_OVERVIEW), &testkit::meta("local")).expect("switch utility");
        assert!(result.operations.is_empty(), "a utility switch must not produce document operations");
        let can_undo = dispatch_action(&mut app, "undo", None, None, &testkit::meta("local"));
        assert!(can_undo.map(|r| r.operations.is_empty()).unwrap_or(true), "a utility switch must not have created a document undo step");
    }

    /// 🧰️ The app declares exactly the select/brush canvas utilities and binds them to the interactive
    /// overview pane; fill is declared as a mode-level tool instead (see `tool_registry_declares_fill_tool`).
    #[test]
    fn utility_registry_declares_utilities() {
        let definition = create_puzzle2d_app().definition;
        let ids: Vec<&str> = definition.utilities.iter().map(|utility| utility.id.as_str()).collect();
        assert_eq!(ids, vec![PUZZLE2D_UTILITY_SELECT, PUZZLE2D_UTILITY_BRUSH]);
        let overview = definition.window_kinds.iter().find(|window| window.id == PUZZLE2D_PANE_OVERVIEW).expect("overview pane");
        let overview_utilities: Vec<&str> = overview.utilities.iter().map(|utility| utility.as_str()).collect();
        assert_eq!(overview_utilities, vec![PUZZLE2D_UTILITY_SELECT, PUZZLE2D_UTILITY_BRUSH]);
        assert!(definition.actions.iter().any(|action| action.id == SET_ACTIVE_UTILITY_ACTION_ID), "declaring utilities must inject the setActiveUtility action");
        // 🧰️ D-1: select/brush are this window's whole exclusive utility set, NOT a sub-collection, so
        // each carries `group: None` and renders as a flat utility bar icon (never one collapsed dropdown).
        for utility in &definition.utilities {
            assert_eq!(utility.group, None, "utility {} must render flat (no shared group)", utility.id);
        }
    }

    /// 🛠️ Fill is a mode-level tool (a whole-document generator), not a window utility.
    #[test]
    fn tool_registry_declares_fill_tool() {
        use semio_framework_plugin::{ToolRef, SET_ACTIVE_TOOL_ACTION_ID};
        let definition = create_puzzle2d_app().definition;
        let tool_ids: Vec<&str> = definition.tools.iter().map(|tool| tool.id.as_str()).collect();
        assert_eq!(tool_ids, vec![PUZZLE2D_UTILITY_FILL]);
        assert_eq!(definition.modes[0].tools, vec![ToolRef::new(PUZZLE2D_UTILITY_FILL)]);
        assert!(definition.actions.iter().any(|action| action.id == SET_ACTIVE_TOOL_ACTION_ID), "declaring tools must inject the setActiveTool action");
    }

    /// 🛠️ Fill's count slider is a tool measure keyed by the fill tool id, not a window utility-options group.
    #[test]
    fn fill_count_slider_is_a_tool_measure() {
        let labels = puzzle2d_labels(&Puzzle2dConfig::default());
        let host = puzzle_board_host();
        let mut fill_runtime = Puzzle2dPlayRuntime::default();
        fill_runtime.fill_count = 3;
        let fill_scene = Puzzle2dScene { fixture: default_empty_fixture(), runtime: fill_runtime, active_utility: PUZZLE2D_UTILITY_SELECT.into() };
        let fill_measure = puzzle2d_fill_tool_measures(&fill_scene, labels);
        assert!(matches!(&fill_measure, WindowMeasure::Group { id, active_utility_id: None, .. } if id == "puzzle2d-tool-options-fill"));
        assert!(!puzzle2d_window_measures(PUZZLE2D_PANE_OVERVIEW, &fill_scene, labels).iter().any(|measure| matches!(measure, WindowMeasure::Group { id, .. } if id == "puzzle2d-tool-options-fill")), "fill must no longer surface in window_measures");
        assert!(puzzle2d_engagement(&fill_scene, &host, PUZZLE2D_PANE_OVERVIEW, labels).control.is_none(), "fill engagement HUD must no longer carry the relocated control");
    }

    #[test]
    fn brush_params_are_tagged_utility_options_not_engagement_controls() {
        let labels = puzzle2d_labels(&Puzzle2dConfig::default());
        let host = puzzle_board_host();
        let group_tag = |measures: &[WindowMeasure], id: &str| {
            measures.iter().find_map(|measure| match measure {
                WindowMeasure::Group { id: gid, active_utility_id, .. } if gid == id => Some(active_utility_id.clone()),
                _ => None,
            })
        };
        // 🖌️ Brush candidate picker becomes a fill-utility-sibling tagged group, present only once the host
        // has candidates to place (empty ⇒ absent, matching the old gated-control behaviour).
        let empty_brush = Puzzle2dScene { fixture: default_empty_fixture(), runtime: Puzzle2dPlayRuntime::default(), active_utility: PUZZLE2D_UTILITY_BRUSH.into() };
        assert_eq!(group_tag(&puzzle2d_window_measures(PUZZLE2D_PANE_OVERVIEW, &empty_brush, labels), "puzzle2d-utility-options-brush"), Some(Some(PUZZLE2D_UTILITY_BRUSH.into())));
        let mut brush_runtime = Puzzle2dPlayRuntime::default();
        brush_runtime.brush_candidates = vec![json!({ "nodeKind": "node" })];
        let brush_scene = Puzzle2dScene { fixture: default_empty_fixture(), runtime: brush_runtime, active_utility: PUZZLE2D_UTILITY_BRUSH.into() };
        let brush_measures = puzzle2d_window_measures(PUZZLE2D_PANE_OVERVIEW, &brush_scene, labels);
        assert_eq!(group_tag(&brush_measures, "puzzle2d-utility-options-brush"), Some(Some(PUZZLE2D_UTILITY_BRUSH.into())));
        assert!(puzzle2d_engagement(&brush_scene, &host, PUZZLE2D_PANE_OVERVIEW, labels).control.is_none(), "brush engagement HUD must no longer carry the relocated control");
    }

    /// 🧭️ Kind discipline: every View-declared runtime/host action must run through the registry
    /// without tripping the "must not emit operations" guard (proving each is correctly classified).
    #[test]
    fn kind_weight_group_normalizes_to_sum_one() {
        let ids = vec!["a".into(), "b".into(), "c".into()];
        let initial = puzzle2d_uniform_kind_weights(&ids);
        let next = puzzle2d_normalize_kind_weight_group(&initial, &ids, "a", 0.5);
        let sum: f64 = ids.iter().map(|id| next.get(id).copied().unwrap_or(0.0)).sum();
        assert!((sum - 1.0).abs() < 0.001, "expected normalized weights to sum to 1, got {sum}");
        assert!((next.get("a").copied().unwrap_or(0.0) - 0.5).abs() < 0.001);
    }

    #[test]
    fn view_actions_emit_no_ops_through_the_registry() {
        let mut app = registry_app();
        dispatch_action(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID })), None, &testkit::meta("local")).expect("load example");
        let node_id = fixture_nodes(&app.projection().expect("projection").0)[0].get("id").and_then(|value| value.as_str()).unwrap().to_string();
        let view_dispatches: Vec<(&str, Value)> = vec![
            ("setSelection", json!({ "ids": [node_id.clone()] })),
            ("setCamera", json!({ "camera": { "x": 7.0, "y": 8.0, "zoom": 1.5 } })),
            ("selectAll", Value::Null),
            ("selectSameKind", Value::Null),
            ("clearSelection", Value::Null),
            ("setSelectionMethod", json!({ "method": "lasso" })),
            ("setGridSnapEnabled", json!({ "enabled": true })),
            ("setGridFactor", json!({ "value": 2.0 })),
            ("setLodModeForPane", json!({ "pane": PUZZLE2D_PANE_OVERVIEW, "value": "detail" })),
            ("setBrushKindWeights", json!({ "kindId": "node", "value": 0.5 })),
            ("setBrushNodeSize", json!({ "size": 12.0 })),
            ("setSuggestionOffset", json!({ "value": 40.0 })),
            ("engagementInput", json!({ "pane": PUZZLE2D_PANE_OVERVIEW, "value": "brush" })),
            ("engagementSubmit", json!({ "pane": PUZZLE2D_PANE_OVERVIEW, "value": "brush" })),
            ("engagementAbort", json!({ "pane": PUZZLE2D_PANE_OVERVIEW })),
            ("brushCycleCandidate", json!({ "forward": true })),
            ("brushSetCandidateIndex", json!({ "index": 0 })),
            ("brushFillSessionBegin", json!({ "maxCount": 4, "seed": 1 })),
            ("brushFillSessionClear", Value::Null),
            ("lodScaleJson", Value::Null),
        ];
        for (action, args) in view_dispatches {
            let args_ref = (!args.is_null()).then_some(&args);
            let result = dispatch_action(&mut app, action, args_ref, None, &testkit::meta("local")).unwrap_or_else(|error| panic!("view action '{action}' must not error: {error}"));
            assert!(result.operations.is_empty(), "view action '{action}' must not emit document operations");
        }
    }

    /// 🗂️ Grouped-context-menu disclosure: the top-level row budget stays small (leaves+groups
    /// combined) and the known `deleteSelection` destructive row stays last — mirrors
    /// `dag_ui`'s `context_menu_grouped_disclosure_stays_within_budget_and_keeps_destructive_last`.
    #[test]
    fn context_menu_grouped_disclosure_stays_within_budget_and_keeps_destructive_last() {
        use semio_framework_plugin::{ContextMenuRequest, UiMenuRef};

        let mut app = registry_app();
        dispatch_action(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID })), None, &testkit::meta("local")).expect("load example");
        let node_id = fixture_nodes(&app.projection().expect("projection").0)[0].get("id").and_then(|value| value.as_str()).unwrap().to_string();
        dispatch_action(&mut app, "setSelection", Some(&json!({ "ids": [node_id.clone()] })), None, &testkit::meta("local")).expect("select node");
        let request = ContextMenuRequest { menu: UiMenuRef { id: "puzzle2d".into(), args: None }, surface: None, window_instance_id: None, point: None };
        let menu = app.context_menu(&request);
        assert!(menu.len() <= 9, "top-level menu (leaves+groups+separator) should stay within the row budget: {menu:?}");
        let last = menu.last().expect("grouped disclosure menu should not be empty");
        assert_eq!(last.id, "deleteSelection", "the destructive row must stay last as a top-level leaf");
        assert_eq!(last.destructive, Some(true), "the destructive row must carry destructive: true");
    }
}
//#endregion 🧪️Tests
