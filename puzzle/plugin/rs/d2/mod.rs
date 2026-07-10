//! 🧩 Puzzle 2D plugin — declarative puzzle 2d play app bundled as a hot-swappable WASM component.

use puzzle_2d::{handle_position_on_circle, handle_position_on_rectangle, puzzle_2d_lod_scale_json, puzzle_board_host, BoardHost, Point, Puzzle2dExtension, BOARD_CAMERA_ZOOM_MAX, BOARD_CAMERA_ZOOM_MIN};
use semio_framework_plugin::{
    build_canvas_2d_scene, build_puzzle2d_board_scene, create_default_layout,
    layout::{MeasureSelectItem, WindowEngagementStatus, WindowEngagementToggleGroupOption},
    ui_inspector_readonly_field, ui_stack_vertical, ui_text, App, CommandDescriptor, PanelGroup, PluginApp, PluginBundle, Puzzle2dBoardScene, SurfaceKind, UiNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, ViewState, WindowEngagement,
    WindowEngagementControl, WindowEngagementInput, WindowEngagementOption, WindowMeasure, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::LazyLock;

//#region 🔖Constants
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
const CONCRETE_FOREST_EXAMPLE_JSON: &str = include_str!("../../../2d/example/concrete-forest.2d.json");
const NAKAGIN_EXAMPLE_JSON: &str = include_str!("../../../2d/example/nakagin-capsule-tower.2d.json");
const PUZZLE2D_ENGAGEMENT_TOOL_SELECT: &str = "puzzle2d.tool.select";
const PUZZLE2D_ENGAGEMENT_TOOL_BRUSH: &str = "puzzle2d.tool.brush";
const PUZZLE2D_ENGAGEMENT_TOOL_FILL: &str = "puzzle2d.tool.fill";
const BOARD_DEFAULT_WIDTH: u32 = 1024;
const BOARD_DEFAULT_HEIGHT: u32 = 768;

//#region 🔖PaneConstants
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
//#endregion 🔖PaneConstants

//#region 🔖EngagementConstants
const PUZZLE2D_SUGGESTION_OFFSET_MIN: f64 = 0.0;
const PUZZLE2D_SUGGESTION_OFFSET_MAX: f64 = 160.0;
const PUZZLE2D_SUGGESTION_OFFSET_STEP: f64 = 4.0;
const PUZZLE2D_FILL_COUNT_MAX: u32 = 1000;
/// 📶 Mirrors `ui_styling::metrics::board::SUGGESTION_OFFSET`; kept local since the plugin crate has no styling dependency.
const PUZZLE2D_DEFAULT_SUGGESTION_OFFSET: f64 = 80.0;
//#endregion 🔖EngagementConstants

static NODE_ID_COUNTER: AtomicU32 = AtomicU32::new(0);
//#endregion 🔖Constants

//#region 🔖Envelope
fn default_active_tool() -> String {
    "select".into()
}

fn default_selection_method() -> String {
    "rectangle".into()
}

fn default_grid_factor() -> f64 {
    1.0
}

fn default_suggestion_offset() -> f64 {
    PUZZLE2D_DEFAULT_SUGGESTION_OFFSET
}

/// 📶 Overview/selection default to automatic LOD; detail defaults to a fixed "detail" tier, matching the pre-migration triptych.
fn default_lod_mode_by_pane() -> BTreeMap<String, String> {
    BTreeMap::from([(PUZZLE2D_PANE_OVERVIEW.to_string(), PUZZLE2D_LOD_MODE_AUTOMATIC.to_string()), (PUZZLE2D_PANE_DETAIL.to_string(), "detail".to_string()), (PUZZLE2D_PANE_SELECTION.to_string(), PUZZLE2D_LOD_MODE_AUTOMATIC.to_string())])
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Puzzle2dPlayRuntime {
    #[serde(default)]
    selected_ids: Vec<String>,
    #[serde(default = "default_active_tool")]
    active_tool: String,
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
}

/// ⚠️ Explicit impl (not `#[derive(Default)]`) so Rust construction matches the serde field defaults above.
impl Default for Puzzle2dPlayRuntime {
    fn default() -> Self {
        Self {
            selected_ids: Vec::new(),
            active_tool: default_active_tool(),
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
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Puzzle2dPlayEnvelope {
    fixture: Value,
    #[serde(default)]
    runtime: Puzzle2dPlayRuntime,
}

fn default_empty_fixture() -> Value {
    json!({
        "schema": PUZZLE2D_FIXTURE_SCHEMA,
        "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
        "nodes": [],
        "edges": [],
        "wires": []
    })
}

fn default_envelope() -> Puzzle2dPlayEnvelope {
    Puzzle2dPlayEnvelope { fixture: default_empty_fixture(), runtime: Puzzle2dPlayRuntime::default() }
}

fn parse_envelope(document_json: &str) -> Puzzle2dPlayEnvelope {
    serde_json::from_str(document_json).unwrap_or_else(|_| default_envelope())
}

fn set_document_op(envelope: &Puzzle2dPlayEnvelope) -> String {
    json!({ "op": "setDocument", "document": envelope }).to_string()
}

fn puzzle2d_cmd(command: &str, args: Option<Value>) -> CommandDescriptor {
    CommandDescriptor { controller_id: PUZZLE2D_PLAY_CONTROLLER_ID.into(), command: command.into(), args }
}

fn selection_ids(args: Option<&Value>) -> Vec<String> {
    args.and_then(|value| value.get("ids")).and_then(|value| serde_json::from_value(value.clone()).ok()).or_else(|| args.and_then(|value| value.get("id")).and_then(|value| value.as_str()).map(|id| vec![id.to_string()])).unwrap_or_default()
}

fn fixture_camera(fixture: &Value) -> (f64, f64, f64) {
    let camera = fixture.get("camera");
    (
        camera.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(0.0),
        camera.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(0.0),
        camera.and_then(|value| value.get("zoom")).and_then(|value| value.as_f64()).unwrap_or(1.0),
    )
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

fn add_node_to_fixture(fixture: &mut Value, kind: Option<&str>) {
    let Some(obj) = fixture.as_object_mut() else {
        return;
    };
    let nodes = obj.entry("nodes".to_string()).or_insert_with(|| json!([]));
    let Some(nodes) = nodes.as_array_mut() else {
        return;
    };
    let node_kind = kind.unwrap_or("node");
    let id = new_node_id("node");
    nodes.push(json!({
        "id": id,
        "nodeKind": node_kind,
        "shape": "circle",
        "x": 0.0,
        "y": 0.0,
        "radius": 24.0,
        "text": id,
        "handles": []
    }));
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

fn set_fixture_camera(fixture: &mut Value, camera: &Value) {
    if let Some(obj) = fixture.as_object_mut() {
        obj.insert("camera".to_string(), camera.clone());
    }
}

fn puzzle_extension_id() -> &'static str {
    let _extension = Puzzle2dExtension;
    "puzzle.2d"
}
//#endregion 🔖Envelope

//#region 🔖BoardHost
fn sync_host_from_envelope(host: &mut BoardHost, envelope: &Puzzle2dPlayEnvelope) {
    host.set_size(BOARD_DEFAULT_WIDTH, BOARD_DEFAULT_HEIGHT, 1.0);
    let _ = host.parse_fixture_v1(&envelope.fixture);
    host.set_selection_ids(&envelope.runtime.selected_ids);
    host.set_active_tool(&envelope.runtime.active_tool);
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

fn apply_board_events_from_json(events_json: &str, envelope: &mut Puzzle2dPlayEnvelope) {
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
                if let Some(obj) = envelope.fixture.as_object_mut() {
                    obj.insert("camera".into(), payload);
                }
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
                            patch_inspector_nodes(&mut envelope.fixture, &[id.to_string()], "x", &json!(x));
                        }
                        if let Some(y) = entry.get("y").and_then(|value| value.as_f64()) {
                            patch_inspector_nodes(&mut envelope.fixture, &[id.to_string()], "y", &json!(y));
                        }
                    }
                }
            }
            "nodeMove" => {
                let Some(id) = payload.get("id").and_then(|value| value.as_str()) else {
                    continue;
                };
                if let Some(x) = payload.get("x").and_then(|value| value.as_f64()) {
                    patch_inspector_nodes(&mut envelope.fixture, &[id.to_string()], "x", &json!(x));
                }
                if let Some(y) = payload.get("y").and_then(|value| value.as_f64()) {
                    patch_inspector_nodes(&mut envelope.fixture, &[id.to_string()], "y", &json!(y));
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

fn apply_host_events(host: &mut BoardHost, envelope: &mut Puzzle2dPlayEnvelope) {
    let events_raw = host.drain_events_json();
    apply_board_events_from_json(&events_raw, envelope);
    envelope.runtime.selected_ids = host.selection.iter().cloned().collect();
    let (camera_x, camera_y, zoom) = fixture_camera(&envelope.fixture);
    if (host.camera.x - camera_x).abs() > 1e-9 || (host.camera.y - camera_y).abs() > 1e-9 || (host.camera.zoom - zoom).abs() > 1e-9 {
        set_fixture_camera(&mut envelope.fixture, &json!({ "x": host.camera.x, "y": host.camera.y, "zoom": host.camera.zoom }));
    }
}

fn apply_brush_place_payload(fixture: &mut Value, payload: &Value) {
    let node_id = payload.get("nodeId").and_then(|value| value.as_str()).map(str::to_string).unwrap_or_else(|| new_node_id("node"));
    let edge_id = payload.get("edgeId").and_then(|value| value.as_str()).map(str::to_string).unwrap_or_else(|| new_node_id("edge"));
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

/// 🖌️ Brush candidate rows (`{nodeKind, targetHandleIndex}`) drained from the host into a placement toggle-group control.
fn puzzle2d_brush_placement_control(envelope: &Puzzle2dPlayEnvelope) -> Option<WindowEngagementControl> {
    if envelope.runtime.brush_candidates.is_empty() {
        return None;
    }
    let options: Vec<WindowEngagementToggleGroupOption> = envelope
        .runtime
        .brush_candidates
        .iter()
        .enumerate()
        .map(|(index, candidate)| {
            let node_kind = candidate.get("nodeKind").and_then(|value| value.as_str()).or_else(|| candidate.as_str()).unwrap_or("kind");
            WindowEngagementToggleGroupOption { id: format!("puzzle2d.brush.candidate.{index}"), label: node_kind.into(), disabled: None }
        })
        .collect();
    let selected_index = envelope.runtime.brush_candidate_index.min(options.len().saturating_sub(1));
    Some(WindowEngagementControl::ToggleGroup {
        id: Some("puzzle2d-brush-placement".into()),
        label: Some("Placement".into()),
        value: Some(format!("puzzle2d.brush.candidate.{selected_index}")),
        options,
        disabled: None,
        on_select: Some(puzzle2d_cmd("engagementControlSelect", None)),
    })
}

/// 🪣 Fill-count slider control shown while the fill tool builds a brush-fill session.
fn puzzle2d_fill_count_control(envelope: &Puzzle2dPlayEnvelope) -> WindowEngagementControl {
    WindowEngagementControl::Slider {
        id: Some("puzzle2d-fill-count".into()),
        label: Some(format!("Fill {}", envelope.runtime.fill_count)),
        value: envelope.runtime.fill_count as f64,
        min: 0.0,
        max: PUZZLE2D_FILL_COUNT_MAX as f64,
        step: Some(1.0),
        unit: None,
        disabled: None,
        on_change: Some(puzzle2d_cmd("setFillCount", None)),
        on_commit: None,
    }
}

fn puzzle2d_engagement(envelope: &Puzzle2dPlayEnvelope, host: &BoardHost, pane: &str) -> WindowEngagement {
    let overlay: Value = serde_json::from_str(&host.overlay_paint_state_json()).unwrap_or(Value::Null);
    let pane_lod_mode = envelope.runtime.lod_mode_by_pane.get(pane).map(String::as_str).unwrap_or(PUZZLE2D_LOD_MODE_AUTOMATIC);
    let lod = overlay.get("lod").and_then(|value| value.as_str()).unwrap_or(if pane_lod_mode == PUZZLE2D_LOD_MODE_AUTOMATIC { "auto" } else { pane_lod_mode });
    let node_count = fixture_nodes(&envelope.fixture).len();
    let edge_count = fixture_edges(&envelope.fixture).len();
    let control = match envelope.runtime.active_tool.as_str() {
        "fill" => Some(puzzle2d_fill_count_control(envelope)),
        "brush" => puzzle2d_brush_placement_control(envelope),
        _ => None,
    };
    let input_value = envelope.runtime.engagement_input_by_pane.get(pane).cloned().unwrap_or_default();
    let placeholder = match envelope.runtime.active_tool.as_str() {
        "fill" => "Fill",
        "brush" => "Brush",
        _ => "select, brush, fill, clear",
    };
    WindowEngagement {
        session_active: Some(envelope.runtime.active_tool != "select"),
        input: Some(WindowEngagementInput {
            id: Some("puzzle2d-engagement".into()),
            value: Some(input_value),
            placeholder: Some(placeholder.into()),
            disabled: None,
            on_change: Some(puzzle2d_cmd("engagementInput", Some(json!({ "pane": pane })))),
            on_submit: Some(puzzle2d_cmd("engagementSubmit", Some(json!({ "pane": pane })))),
            on_repeat_last: None,
            on_abort: Some(puzzle2d_cmd("engagementAbort", Some(json!({ "pane": pane })))),
        }),
        control,
        controls: None,
        status: Some(vec![WindowEngagementStatus { id: "puzzle2d-board-status".into(), text: format!("{node_count} nodes · {edge_count} edges · LOD {lod}") }]),
        options: Some(vec![
            WindowEngagementOption {
                id: PUZZLE2D_ENGAGEMENT_TOOL_SELECT.into(),
                label: Some("Select".into()),
                icon_id: Some("cursor".into()),
                pressed: Some(envelope.runtime.active_tool == "select"),
                disabled: None,
                command: Some(puzzle2d_cmd("engagementPossibleSelect", Some(json!({ "pane": pane, "possibleId": PUZZLE2D_ENGAGEMENT_TOOL_SELECT })))),
            },
            WindowEngagementOption {
                id: PUZZLE2D_ENGAGEMENT_TOOL_BRUSH.into(),
                label: Some("Brush".into()),
                icon_id: Some("brush".into()),
                pressed: Some(envelope.runtime.active_tool == "brush"),
                disabled: None,
                command: Some(puzzle2d_cmd("engagementPossibleSelect", Some(json!({ "pane": pane, "possibleId": PUZZLE2D_ENGAGEMENT_TOOL_BRUSH })))),
            },
            WindowEngagementOption {
                id: PUZZLE2D_ENGAGEMENT_TOOL_FILL.into(),
                label: Some("Fill".into()),
                icon_id: Some("fill".into()),
                pressed: Some(envelope.runtime.fill_count > 0 || envelope.runtime.active_tool == "fill"),
                disabled: None,
                command: Some(puzzle2d_cmd("engagementPossibleSelect", Some(json!({ "pane": pane, "possibleId": PUZZLE2D_ENGAGEMENT_TOOL_FILL })))),
            },
        ]),
        possible_engagements: None,
    }
}
//#endregion 🔖BoardHost

//#region 🔖Canvas
fn fixture_wires(fixture: &Value) -> &[Value] {
    fixture.get("wires").and_then(|value| value.as_array()).map(|values| values.as_slice()).unwrap_or(&[])
}

fn fixture_handles(fixture: &Value) -> Vec<Value> {
    fixture_nodes(fixture).iter().flat_map(|node| node.get("handles").and_then(|value| value.as_array()).into_iter().flatten().cloned()).collect()
}

fn fixture_endpoint_xy(fixture: &Value, endpoint_id: &str) -> Option<(f64, f64)> {
    if let Some((node_id, handle_id)) = endpoint_id.split_once(':') {
        let node = fixture_nodes(fixture).iter().find(|node| node.get("id").and_then(|value| value.as_str()) == Some(node_id))?;
        let cx = node.get("x").and_then(|value| value.as_f64())?;
        let cy = node.get("y").and_then(|value| value.as_f64())?;
        let handle = node.get("handles").and_then(|value| value.as_array()).into_iter().flatten().find(|handle| handle.get("id").and_then(|value| value.as_str()) == Some(handle_id))?;
        let angle = handle.get("angle").and_then(|value| value.as_f64()).unwrap_or(0.0);
        let point = if node.get("shape").and_then(|value| value.as_str()) == Some("rectangle") {
            let width = node.get("width").and_then(|value| value.as_f64()).unwrap_or(48.0);
            let height = node.get("height").and_then(|value| value.as_f64()).unwrap_or(48.0);
            handle_position_on_rectangle(Point::new(cx, cy), width, height, angle)
        } else {
            let radius = node.get("radius").and_then(|value| value.as_f64()).unwrap_or(24.0);
            handle_position_on_circle(Point::new(cx, cy), radius, angle)
        };
        return Some((point.x, point.y));
    }
    let node = fixture_nodes(fixture).iter().find(|node| node.get("id").and_then(|value| value.as_str()) == Some(endpoint_id))?;
    Some((node.get("x").and_then(|value| value.as_f64()).unwrap_or(0.0), node.get("y").and_then(|value| value.as_f64()).unwrap_or(0.0)))
}

//#region 🔖PaneCamera
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

/// 📐 World-space center and half-span of every node's extent (circle radius or rectangle half-size), used to frame pane cameras.
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

/// 📷 Triptych camera for a pane: overview is zoomed out and centered on the fixture, detail zooms into the last-placed node, selection frames a lower-left quadrant — mirrors the pre-migration `puzzle2dPlayTriptychCameraForPane`.
fn puzzle2d_pane_camera(fixture: &Value, pane: &str) -> (f64, f64, f64) {
    let (camera_x, camera_y, camera_zoom) = fixture_camera(fixture);
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

/// 🖱️ Recovers the pane id from the `canvas-2d-host` surface id echoed back into pointer-command args.
fn pane_from_surface_id(surface_id: &str) -> &'static str {
    if surface_id.ends_with(PUZZLE2D_PANE_DETAIL) {
        PUZZLE2D_PANE_DETAIL
    } else if surface_id.ends_with(PUZZLE2D_PANE_SELECTION) {
        PUZZLE2D_PANE_SELECTION
    } else {
        PUZZLE2D_PANE_OVERVIEW
    }
}

//#endregion 🔖PaneCamera

fn puzzle2d_board_scene(fixture: &Value, selected: &[String], pane: &str) -> Puzzle2dBoardScene {
    let (camera_x, camera_y, zoom) = puzzle2d_pane_camera(fixture, pane);
    let camera_json = json!({ "x": camera_x, "y": camera_y, "zoom": zoom }).to_string();
    let kind_catalogs_json = fixture.get("meta").and_then(|value| value.get("kindCatalogs")).map(|value| value.to_string()).unwrap_or_else(|| "{}".into());
    let selection_json = serde_json::to_string(selected).unwrap_or_else(|_| "[]".into());
    Puzzle2dBoardScene { fixture_json: fixture.to_string(), camera_json, kind_catalogs_json, selection_json, interactive: pane == PUZZLE2D_PANE_OVERVIEW, hovered_id: None, active_tool: None }
}

fn render_canvas(fixture: &Value, selected: &[String], pane: &str) -> UiNode {
    build_puzzle2d_board_scene(format!("{PUZZLE2D_PLAY_SURFACE_ID}.{pane}"), PUZZLE2D_PLAY_CONTROLLER_ID, puzzle2d_board_scene(fixture, selected, pane))
}

fn force_layout_fixture(fixture: &mut Value) {
    let Ok(layout_json) = puzzle_2d::apply_force_graph_layout_to_fixture_v1_json(&fixture.to_string(), r#"{"mode":"force-graph"}"#) else {
        return;
    };
    if let Ok(parsed) = serde_json::from_str(&layout_json) {
        *fixture = parsed;
    }
}

fn patch_inspector_nodes(fixture: &mut Value, ids: &[String], field: &str, value: &Value) {
    if let Some(nodes) = fixture.get_mut("nodes").and_then(|entry| entry.as_array_mut()) {
        for node in nodes {
            let Some(id) = node.get("id").and_then(|entry| entry.as_str()) else {
                continue;
            };
            if !ids.is_empty() && !ids.contains(&id.to_string()) {
                continue;
            }
            if let Some(obj) = node.as_object_mut() {
                obj.insert(field.to_string(), value.clone());
            }
        }
    }
}
//#endregion 🔖Canvas

//#region 🔖DocumentPanel
fn tree_item_with_command(id: impl Into<String>, label: impl Into<String>, description: Option<String>, command: CommandDescriptor) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.into(),
        label: label.into(),
        description,
        icon_id: None,
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

fn render_document_panel(envelope: &Puzzle2dPlayEnvelope) -> UiNode {
    let fixture = &envelope.fixture;
    let node_items: Vec<UiTreeItemNode> = fixture_nodes(fixture)
        .iter()
        .filter_map(|node| {
            let id = node.get("id")?.as_str()?;
            Some(tree_item_with_command(format!("puzzle2d-play-document.node.{id}"), node_label(node), node.get("nodeKind").and_then(|value| value.as_str()).map(str::to_string), puzzle2d_cmd("setSelection", Some(json!({ "ids": [id] })))))
        })
        .collect();
    let edge_items: Vec<UiTreeItemNode> = fixture_edges(fixture)
        .iter()
        .filter_map(|edge| {
            let id = edge.get("id")?.as_str()?;
            Some(tree_item_with_command(format!("puzzle2d-play-document.edge.{id}"), edge_label(edge, fixture), edge.get("edgeKind").and_then(|value| value.as_str()).map(str::to_string), puzzle2d_cmd("setSelection", Some(json!({ "ids": [id] })))))
        })
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![
            UiTreeSectionNode {
                id: "puzzle2d-play-document.nodes".into(),
                label: Some("Nodes".into()),
                default_open: Some(true),
                items: if node_items.is_empty() {
                    vec![UiTreeItemNode {
                        id: "puzzle2d-play-document.nodes.empty".into(),
                        label: "(none)".into(),
                        description: None,
                        icon_id: None,
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
                    }]
                } else {
                    node_items
                },
            },
            UiTreeSectionNode {
                id: "puzzle2d-play-document.edges".into(),
                label: Some("Edges".into()),
                default_open: Some(false),
                items: if edge_items.is_empty() {
                    vec![UiTreeItemNode {
                        id: "puzzle2d-play-document.edges.empty".into(),
                        label: "(none)".into(),
                        description: None,
                        icon_id: None,
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
                    }]
                } else {
                    edge_items
                },
            },
        ],
        selected_ids: Some(document_tree_selected_ids(fixture, &envelope.runtime.selected_ids)),
        highlighted_ids: None,
        selection_change: Some(puzzle2d_cmd("setSelection", None)),
    })
}
//#endregion 🔖DocumentPanel

//#region 🔖CataloguePanel
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

fn kind_catalog_section(section_id: &str, label: &str, entries: &[Value]) -> UiTreeSectionNode {
    let items: Vec<UiTreeItemNode> = entries
        .iter()
        .enumerate()
        .map(|(index, entry)| {
            let kind_id = entry.get("id").and_then(|value| value.as_str()).unwrap_or("kind");
            UiTreeItemNode {
                id: format!("{section_id}.{index}.{kind_id}"),
                label: catalog_kind_label(entry),
                description: Some(kind_id.into()),
                icon_id: None,
                selected: None,
                default_open: None,
                command: Some(puzzle2d_cmd("addNode", Some(json!({ "kind": kind_id })))),
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
        .collect();
    UiTreeSectionNode {
        id: section_id.into(),
        label: Some(label.into()),
        default_open: Some(true),
        items: if items.is_empty() {
            vec![UiTreeItemNode {
                id: format!("{section_id}.empty"),
                label: "(none)".into(),
                description: None,
                icon_id: None,
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
            }]
        } else {
            items
        },
    }
}

fn render_catalogue_panel(fixture: &Value) -> UiNode {
    let inferred_nodes = inferred_kind_entries(fixture, "nodes");
    let inferred_handles = inferred_kind_entries(fixture, "handles");
    let inferred_edges = inferred_kind_entries(fixture, "edges");
    let node_entries = kind_catalog_entries(fixture, "nodes").unwrap_or(inferred_nodes.as_slice());
    let handle_entries = kind_catalog_entries(fixture, "handles").unwrap_or(inferred_handles.as_slice());
    let edge_entries = kind_catalog_entries(fixture, "edges").unwrap_or(inferred_edges.as_slice());
    UiNode::Tree(UiTreeNode {
        sections: vec![
            kind_catalog_section("puzzle2d-play-kinds.nodes", "Nodes", &node_entries),
            kind_catalog_section("puzzle2d-play-kinds.handles", "Handles", &handle_entries),
            kind_catalog_section("puzzle2d-play-kinds.edges", "Edges", &edge_entries),
        ],
        selected_ids: None,
        highlighted_ids: None,
        selection_change: None,
    })
}
//#endregion 🔖CataloguePanel

//#region 🔖InspectorPanel
fn render_properties_panel(envelope: &Puzzle2dPlayEnvelope) -> UiNode {
    let selected_nodes: Vec<&Value> = envelope.runtime.selected_ids.iter().filter_map(|id| fixture_nodes(&envelope.fixture).iter().find(|node| node.get("id").and_then(|value| value.as_str()) == Some(id.as_str()))).collect();
    if selected_nodes.is_empty() {
        return ui_stack_vertical(vec![
            ui_text(format!("Schema: {PUZZLE2D_FIXTURE_SCHEMA}")),
            ui_text(format!("Extension: {}", puzzle_extension_id())),
            ui_text(format!("Nodes: {}", fixture_nodes(&envelope.fixture).len())),
            ui_text(format!("Edges: {}", fixture_edges(&envelope.fixture).len())),
        ]);
    }
    let node = selected_nodes[0];
    ui_stack_vertical(vec![
        ui_inspector_readonly_field("puzzle2d-play-inspector.id", "Id", node.get("id").and_then(|value| value.as_str()).unwrap_or("").to_string()),
        ui_inspector_readonly_field("puzzle2d-play-inspector.node-kind", "Node Kind", node.get("nodeKind").and_then(|value| value.as_str()).unwrap_or("—").to_string()),
        ui_inspector_readonly_field("puzzle2d-play-inspector.x", "X", node.get("x").and_then(|value| value.as_f64()).map(|value| value.to_string()).unwrap_or_else(|| "—".into())),
        ui_inspector_readonly_field("puzzle2d-play-inspector.y", "Y", node.get("y").and_then(|value| value.as_f64()).map(|value| value.to_string()).unwrap_or_else(|| "—".into())),
    ])
}
//#endregion 🔖InspectorPanel

//#region 🔖Measures
fn puzzle2d_lod_tier_ids() -> Vec<String> {
    serde_json::from_str::<Vec<Value>>(&puzzle_2d_lod_scale_json()).unwrap_or_default().into_iter().filter_map(|row| row.get("id").and_then(|value| value.as_str()).map(str::to_string)).collect()
}

fn puzzle2d_kind_ids(fixture: &Value, field: &str) -> Vec<String> {
    let inferred = inferred_kind_entries(fixture, field);
    let entries = kind_catalog_entries(fixture, field).unwrap_or(inferred.as_slice());
    entries.iter().filter_map(|entry| entry.get("id").and_then(|value| value.as_str()).map(str::to_string)).collect()
}

/// 📶 Per-pane LOD select measure: "Automatic" plus every scale tier (minimap…micro), persisted via `setLodModeForPane`.
fn puzzle2d_lod_measure(pane: &str, current_mode: &str) -> WindowMeasure {
    let mut items = vec![MeasureSelectItem { id: PUZZLE2D_LOD_MODE_AUTOMATIC.into(), value: PUZZLE2D_LOD_MODE_AUTOMATIC.into(), label: "Automatic".into() }];
    items.extend(puzzle2d_lod_tier_ids().into_iter().map(|tier| MeasureSelectItem { id: tier.clone(), value: tier.clone(), label: tier }));
    WindowMeasure::Select { id: format!("{pane}-lod"), label: Some("LOD".into()), value: current_mode.into(), items, on_change: puzzle2d_cmd("setLodModeForPane", Some(json!({ "pane": pane }))) }
}

fn puzzle2d_kind_weight_measures(prefix: &str, ids: &[String], weights: &BTreeMap<String, f64>, catalog_slice: &str) -> Vec<WindowMeasure> {
    ids.iter()
        .map(|kind_id| {
            let weight = weights.get(kind_id).copied().unwrap_or(0.0);
            WindowMeasure::Slider {
                id: format!("{PUZZLE2D_PLAY_CONTROLLER_ID}-{prefix}-{kind_id}"),
                label: Some(format!("{kind_id} {:.0}%", weight * 100.0)),
                value: weight,
                min: 0.0,
                max: 1.0,
                step: Some(0.01),
                on_change: puzzle2d_cmd("setBrushKindWeights", Some(json!({ "kindId": kind_id, "catalogSlice": catalog_slice }))),
            }
        })
        .collect()
}

/// 🎚️ Suggestion offset slider plus node/handle kind-weight sliders, calling `setSuggestionOffset`/`setBrushKindWeights`.
fn puzzle2d_suggestion_measures_group(envelope: &Puzzle2dPlayEnvelope) -> WindowMeasure {
    let node_ids = puzzle2d_kind_ids(&envelope.fixture, "nodes");
    let handle_ids = puzzle2d_kind_ids(&envelope.fixture, "handles");
    WindowMeasure::Group {
        id: format!("{PUZZLE2D_PLAY_CONTROLLER_ID}-suggestion"),
        label: "Suggestion".into(),
        default_open: Some(false),
        children: vec![
            WindowMeasure::Slider {
                id: format!("{PUZZLE2D_PLAY_CONTROLLER_ID}-suggestion-offset"),
                label: Some("Offset".into()),
                value: envelope.runtime.suggestion_offset,
                min: PUZZLE2D_SUGGESTION_OFFSET_MIN,
                max: PUZZLE2D_SUGGESTION_OFFSET_MAX,
                step: Some(PUZZLE2D_SUGGESTION_OFFSET_STEP),
                on_change: puzzle2d_cmd("setSuggestionOffset", None),
            },
            WindowMeasure::Group {
                id: format!("{PUZZLE2D_PLAY_CONTROLLER_ID}-suggestion-distribution-nodes"),
                label: "Node Weights".into(),
                default_open: Some(false),
                children: puzzle2d_kind_weight_measures("node-kind", &node_ids, &envelope.runtime.node_kind_weights, "nodes"),
            },
            WindowMeasure::Group {
                id: format!("{PUZZLE2D_PLAY_CONTROLLER_ID}-suggestion-distribution-handles"),
                label: "Handle Weights".into(),
                default_open: Some(false),
                children: puzzle2d_kind_weight_measures("handle-kind", &handle_ids, &envelope.runtime.handle_kind_weights, "handles"),
            },
        ],
    }
}

fn puzzle2d_window_measures(pane: &str, envelope: &Puzzle2dPlayEnvelope) -> Vec<WindowMeasure> {
    let mode = envelope.runtime.lod_mode_by_pane.get(pane).map(String::as_str).unwrap_or(PUZZLE2D_LOD_MODE_AUTOMATIC);
    vec![puzzle2d_lod_measure(pane, mode), puzzle2d_suggestion_measures_group(envelope)]
}
//#endregion 🔖Measures

//#region 🔖Puzzle2dPlayApp
pub struct Puzzle2dPlayApp {
    host: BoardHost,
}

impl Default for Puzzle2dPlayApp {
    fn default() -> Self {
        Self { host: puzzle_board_host() }
    }
}

impl PluginApp for Puzzle2dPlayApp {
    fn app_id(&self) -> &str {
        PUZZLE2D_PLAY_APP_ID
    }

    fn initial_document_json(&self) -> String {
        serde_json::to_string(&default_envelope()).expect("puzzle2d envelope json")
    }

    fn handle_command_patch_ops(&mut self, command: &str, args: Option<&Value>, document_json: &str, _view_state: &ViewState) -> Vec<String> {
        let mut envelope = parse_envelope(document_json);
        sync_host_from_envelope(&mut self.host, &envelope);
        let ops = match command {
            "setDocument" => {
                if let Some(next) = args.and_then(|value| value.get("document")) {
                    if let Ok(parsed) = serde_json::from_value(next.clone()) {
                        return vec![set_document_op(&parsed)];
                    }
                }
                Vec::new()
            }
            "setSelection" | "documentSelect" => {
                envelope.runtime.selected_ids = selection_ids(args);
                self.host.set_selection_ids(&envelope.runtime.selected_ids);
                vec![set_document_op(&envelope)]
            }
            "addNode" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str());
                add_node_to_fixture(&mut envelope.fixture, kind);
                vec![set_document_op(&envelope)]
            }
            "deleteSelection" => {
                self.host.delete_selection();
                delete_selection_from_fixture(&mut envelope.fixture, &envelope.runtime.selected_ids);
                envelope.runtime.selected_ids.clear();
                vec![set_document_op(&envelope)]
            }
            "setCamera" => {
                if let Some(camera) = args.and_then(|value| value.get("camera")) {
                    if let (Some(x), Some(y), Some(zoom)) = (camera.get("x").and_then(|value| value.as_f64()), camera.get("y").and_then(|value| value.as_f64()), camera.get("zoom").and_then(|value| value.as_f64())) {
                        self.host.set_camera(x, y, zoom);
                    }
                    set_fixture_camera(&mut envelope.fixture, camera);
                    vec![set_document_op(&envelope)]
                } else {
                    Vec::new()
                }
            }
            "setActiveExample" => {
                let example_id = args.and_then(|value| value.get("exampleId")).and_then(|value| value.as_str()).unwrap_or("");
                envelope.fixture = if example_id.is_empty() || example_id == "empty" {
                    default_empty_fixture()
                } else if example_id == PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID || example_id == "concrete" {
                    serde_json::from_str(CONCRETE_FOREST_EXAMPLE_JSON).unwrap_or_else(|_| default_empty_fixture())
                } else if example_id == PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID || example_id == "nakagin" {
                    serde_json::from_str(NAKAGIN_EXAMPLE_JSON).unwrap_or_else(|_| default_empty_fixture())
                } else {
                    default_empty_fixture()
                };
                envelope.runtime = Puzzle2dPlayRuntime::default();
                vec![set_document_op(&envelope)]
            }
            "setActiveTool" => {
                if let Some(tool) = args.and_then(|value| value.get("tool")).and_then(|value| value.as_str()) {
                    envelope.runtime.active_tool = tool.into();
                    self.host.set_active_tool(tool);
                    vec![set_document_op(&envelope)]
                } else {
                    Vec::new()
                }
            }
            "engagementPossibleSelect" => {
                let possible_id = args.and_then(|value| value.get("possibleId")).and_then(|value| value.as_str()).unwrap_or("");
                let pane = args.and_then(|value| value.get("pane")).and_then(|value| value.as_str()).unwrap_or(PUZZLE2D_PANE_OVERVIEW);
                envelope.runtime.active_tool = match possible_id {
                    PUZZLE2D_ENGAGEMENT_TOOL_BRUSH => "brush",
                    PUZZLE2D_ENGAGEMENT_TOOL_FILL => "fill",
                    _ => "select",
                }
                .into();
                self.host.set_active_tool(&envelope.runtime.active_tool);
                if PUZZLE2D_PANES.contains(&pane) {
                    envelope.runtime.engagement_input_by_pane.insert(pane.to_string(), String::new());
                }
                vec![set_document_op(&envelope)]
            }
            "engagementInput" => {
                let pane = args.and_then(|value| value.get("pane")).and_then(|value| value.as_str()).unwrap_or(PUZZLE2D_PANE_OVERVIEW);
                let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).unwrap_or("");
                if PUZZLE2D_PANES.contains(&pane) {
                    envelope.runtime.engagement_input_by_pane.insert(pane.to_string(), value.to_string());
                    vec![set_document_op(&envelope)]
                } else {
                    Vec::new()
                }
            }
            "engagementSubmit" => {
                let pane = args.and_then(|value| value.get("pane")).and_then(|value| value.as_str()).unwrap_or(PUZZLE2D_PANE_OVERVIEW).to_string();
                let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).map(str::trim).unwrap_or("").to_lowercase();
                let applied = match value.as_str() {
                    "select" => {
                        envelope.runtime.active_tool = "select".into();
                        self.host.set_active_tool("select");
                        true
                    }
                    "brush" => {
                        envelope.runtime.active_tool = "brush".into();
                        self.host.set_active_tool("brush");
                        true
                    }
                    "fill" => {
                        envelope.runtime.active_tool = "fill".into();
                        self.host.set_active_tool("fill");
                        true
                    }
                    "clear" => {
                        envelope.runtime.selected_ids.clear();
                        self.host.set_selection_ids(&[]);
                        true
                    }
                    "rectangle" => {
                        envelope.runtime.selection_method = "rectangle".into();
                        self.host.set_selection_options("rectangle", "replace", true, true, true);
                        true
                    }
                    "lasso" => {
                        envelope.runtime.selection_method = "lasso".into();
                        self.host.set_selection_options("lasso", "replace", true, true, true);
                        true
                    }
                    _ => false,
                };
                if applied && PUZZLE2D_PANES.contains(&pane.as_str()) {
                    envelope.runtime.engagement_input_by_pane.insert(pane, String::new());
                }
                vec![set_document_op(&envelope)]
            }
            "engagementAbort" => {
                let pane = args.and_then(|value| value.get("pane")).and_then(|value| value.as_str()).unwrap_or(PUZZLE2D_PANE_OVERVIEW);
                if PUZZLE2D_PANES.contains(&pane) {
                    envelope.runtime.engagement_input_by_pane.insert(pane.to_string(), String::new());
                }
                if envelope.runtime.active_tool != "select" {
                    envelope.runtime.active_tool = "select".into();
                    self.host.set_active_tool("select");
                }
                vec![set_document_op(&envelope)]
            }
            "engagementControlSelect" => {
                let candidate_id = args.and_then(|value| value.get("id").or_else(|| value.get("value"))).and_then(|value| value.as_str()).unwrap_or("");
                if let Some(index) = candidate_id.strip_prefix("puzzle2d.brush.candidate.").and_then(|rest| rest.parse::<usize>().ok()) {
                    self.host.brush_set_candidate_index(index);
                    envelope.runtime.brush_candidate_index = index;
                    vec![set_document_op(&envelope)]
                } else {
                    Vec::new()
                }
            }
            "setLodModeForPane" => {
                let pane = args.and_then(|value| value.get("pane")).and_then(|value| value.as_str()).unwrap_or("");
                let mode = args.and_then(|value| value.get("value")).and_then(|value| value.as_str());
                if let (true, Some(mode)) = (PUZZLE2D_PANES.contains(&pane), mode) {
                    envelope.runtime.lod_mode_by_pane.insert(pane.to_string(), mode.to_string());
                    if pane == PUZZLE2D_PANE_OVERVIEW {
                        if mode == PUZZLE2D_LOD_MODE_AUTOMATIC {
                            self.host.set_automatic_lod(true);
                        } else {
                            self.host.set_automatic_lod(false);
                            self.host.set_forced_draw_lod_label(mode);
                        }
                    }
                    vec![set_document_op(&envelope)]
                } else {
                    Vec::new()
                }
            }
            "setGridSnapEnabled" => {
                let enabled = args.and_then(|value| value.get("enabled")).and_then(|value| value.as_bool()).unwrap_or(false);
                envelope.runtime.grid_snap_enabled = enabled;
                self.host.set_grid_snap_enabled(enabled);
                vec![set_document_op(&envelope)]
            }
            "setGridFactor" => {
                if let Some(value) = args.and_then(|value| value.get("value")).and_then(|value| value.as_f64()) {
                    envelope.runtime.grid_factor = value;
                    let _ = self.host.set_grid_factor(value);
                    vec![set_document_op(&envelope)]
                } else {
                    Vec::new()
                }
            }
            "setSelectionMethod" => {
                let method = args.and_then(|value| value.get("method")).and_then(|value| value.as_str()).unwrap_or("rectangle");
                envelope.runtime.selection_method = method.into();
                self.host.set_selection_options(method, "replace", true, true, true);
                vec![set_document_op(&envelope)]
            }
            "setBrushKindWeights" => {
                if let Some(weights) = args.and_then(|value| value.get("weights")) {
                    envelope.runtime.node_kind_weights = weights.get("nodeWeights").and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default();
                    envelope.runtime.handle_kind_weights = weights.get("handleWeights").and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default();
                } else if let Some(kind_id) = args.and_then(|value| value.get("kindId")).and_then(|value| value.as_str()) {
                    let weight = args.and_then(|value| value.get("value")).and_then(|value| value.as_f64()).unwrap_or(0.0).clamp(0.0, 1.0);
                    let slice = args.and_then(|value| value.get("catalogSlice")).and_then(|value| value.as_str()).unwrap_or("nodes");
                    if slice == "handles" {
                        envelope.runtime.handle_kind_weights.insert(kind_id.to_string(), weight);
                    } else {
                        envelope.runtime.node_kind_weights.insert(kind_id.to_string(), weight);
                    }
                }
                if let Ok(weights_json) = serde_json::to_string(&json!({
                    "nodeWeights": envelope.runtime.node_kind_weights,
                    "handleWeights": envelope.runtime.handle_kind_weights,
                })) {
                    self.host.set_brush_kind_weights(&weights_json);
                }
                vec![set_document_op(&envelope)]
            }
            "setBrushNodeSize" => {
                if let Some(size) = args.and_then(|value| value.get("size")).and_then(|value| value.as_f64()) {
                    self.host.set_brush_node_size(size);
                }
                Vec::new()
            }
            "setSuggestionOffset" => {
                let distance = args.and_then(|value| value.get("distance").or_else(|| value.get("value"))).and_then(|value| value.as_f64());
                if let Some(distance) = distance {
                    let clamped = distance.clamp(PUZZLE2D_SUGGESTION_OFFSET_MIN, PUZZLE2D_SUGGESTION_OFFSET_MAX);
                    envelope.runtime.suggestion_offset = clamped;
                    self.host.set_suggestion_offset(clamped);
                    vec![set_document_op(&envelope)]
                } else {
                    Vec::new()
                }
            }
            "brushCycleCandidate" => {
                let forward = args.and_then(|value| value.get("forward")).and_then(|value| value.as_bool()).unwrap_or(true);
                self.host.brush_cycle_candidate(forward);
                envelope.runtime.brush_candidate_index = envelope.runtime.brush_candidate_index.saturating_add(1);
                vec![set_document_op(&envelope)]
            }
            "brushSetCandidateIndex" => {
                if let Some(index) = args.and_then(|value| value.get("index")).and_then(|value| value.as_u64()) {
                    self.host.brush_set_candidate_index(index as usize);
                    envelope.runtime.brush_candidate_index = index as usize;
                    vec![set_document_op(&envelope)]
                } else {
                    Vec::new()
                }
            }
            "brushOpenSlot" => {
                if let Some(handle_id) = args.and_then(|value| value.get("handleId")).and_then(|value| value.as_str()) {
                    self.host.brush_open_slot(handle_id);
                }
                Vec::new()
            }
            "brushCommitSlot" => {
                self.host.brush_commit_slot();
                apply_host_events(&mut self.host, &mut envelope);
                vec![set_document_op(&envelope)]
            }
            "brushCancelSlot" => {
                self.host.brush_cancel_slot();
                Vec::new()
            }
            "setFillCount" => {
                let count = args.and_then(|value| value.get("count").or_else(|| value.get("value"))).and_then(|value| value.as_u64()).unwrap_or(0).min(u64::from(PUZZLE2D_FILL_COUNT_MAX)) as u32;
                envelope.runtime.fill_count = count;
                envelope.runtime.active_tool = "fill".into();
                self.host.set_active_tool("brush");
                self.host.brush_fill_session_begin(count, 1);
                let step = self.host.brush_fill_session_step(count.max(1));
                if let Ok(progress) = serde_json::from_str::<Value>(&step) {
                    if let Some(placements) = progress.get("placements").and_then(|value| value.as_array()) {
                        for placement in placements {
                            apply_brush_place_payload(&mut envelope.fixture, placement);
                        }
                    }
                }
                vec![set_document_op(&envelope)]
            }
            "brushFillSessionBegin" => {
                let max_count = args.and_then(|value| value.get("maxCount")).and_then(|value| value.as_u64()).unwrap_or(0) as u32;
                let seed = args.and_then(|value| value.get("seed")).and_then(|value| value.as_u64()).unwrap_or(1) as u32;
                self.host.brush_fill_session_begin(max_count, u64::from(seed));
                Vec::new()
            }
            "brushFillSessionStep" => {
                let budget = args.and_then(|value| value.get("chunkBudget")).and_then(|value| value.as_u64()).unwrap_or(8) as u32;
                let step = self.host.brush_fill_session_step(budget);
                if let Ok(progress) = serde_json::from_str::<Value>(&step) {
                    if let Some(placements) = progress.get("placements").and_then(|value| value.as_array()) {
                        for placement in placements {
                            apply_brush_place_payload(&mut envelope.fixture, placement);
                        }
                    }
                }
                vec![set_document_op(&envelope)]
            }
            "brushFillSessionClear" => {
                self.host.brush_fill_session_clear();
                envelope.runtime.fill_count = 0;
                vec![set_document_op(&envelope)]
            }
            "patchInspectorNodes" => {
                let ids: Vec<String> = args.and_then(|value| value.get("ids")).and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_else(|| envelope.runtime.selected_ids.clone());
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                let value = args.and_then(|value| value.get("value")).cloned().unwrap_or(Value::Null);
                if !field.is_empty() {
                    patch_inspector_nodes(&mut envelope.fixture, &ids, field, &value);
                    vec![set_document_op(&envelope)]
                } else {
                    Vec::new()
                }
            }
            "forceLayout" | "reorganize" => {
                force_layout_fixture(&mut envelope.fixture);
                vec![set_document_op(&envelope)]
            }
            "redrawHandles" => {
                if let Ok(next) = puzzle_2d::apply_edge_handle_snap_to_fixture_v1_json(&envelope.fixture.to_string()) {
                    if let Ok(parsed) = serde_json::from_str(&next) {
                        envelope.fixture = parsed;
                    }
                }
                vec![set_document_op(&envelope)]
            }
            "selectAll" => {
                let ids: Vec<String> = fixture_nodes(&envelope.fixture).iter().filter_map(|node| node.get("id").and_then(|value| value.as_str()).map(str::to_string)).collect();
                envelope.runtime.selected_ids = ids.clone();
                self.host.set_selection_ids(&ids);
                vec![set_document_op(&envelope)]
            }
            "clearSelection" => {
                envelope.runtime.selected_ids.clear();
                self.host.set_selection_ids(&[]);
                vec![set_document_op(&envelope)]
            }
            "focusSelection" => {
                if envelope.runtime.selected_ids.is_empty() {
                    Vec::new()
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
                        let camera = json!({
                            "x": (min_x + max_x) * 0.5,
                            "y": (min_y + max_y) * 0.5,
                            "zoom": 1.0,
                        });
                        set_fixture_camera(&mut envelope.fixture, &camera);
                        if let (Some(x), Some(y), Some(zoom)) = (camera.get("x").and_then(|value| value.as_f64()), camera.get("y").and_then(|value| value.as_f64()), camera.get("zoom").and_then(|value| value.as_f64())) {
                            self.host.set_camera(x, y, zoom);
                        }
                        vec![set_document_op(&envelope)]
                    } else {
                        Vec::new()
                    }
                }
            }
            "applyBoardEvents" => {
                if let Some(events_json) = args.and_then(|value| value.get("eventsJson")).and_then(|value| value.as_str()) {
                    apply_board_events_from_json(events_json, &mut envelope);
                    vec![set_document_op(&envelope)]
                } else {
                    Vec::new()
                }
            }
            "lodScaleJson" => {
                let _ = puzzle_2d_lod_scale_json();
                Vec::new()
            }
            _ => Vec::new(),
        };
        apply_host_events(&mut self.host, &mut envelope);
        if !ops.is_empty() {
            return ops;
        }
        Vec::new()
    }

    fn render(&self, body_key: &str, document_json: &str, _view_state: &ViewState) -> UiNode {
        let envelope = parse_envelope(document_json);
        match body_key {
            PUZZLE2D_PLAY_BODY_OVERVIEW => render_canvas(&envelope.fixture, &envelope.runtime.selected_ids, PUZZLE2D_PANE_OVERVIEW),
            PUZZLE2D_PLAY_BODY_DETAIL => render_canvas(&envelope.fixture, &envelope.runtime.selected_ids, PUZZLE2D_PANE_DETAIL),
            PUZZLE2D_PLAY_BODY_SELECTION => render_canvas(&envelope.fixture, &envelope.runtime.selected_ids, PUZZLE2D_PANE_SELECTION),
            PUZZLE2D_PLAY_BODY_LAYERS => render_document_panel(&envelope),
            PUZZLE2D_PLAY_BODY_CATALOGUE => render_catalogue_panel(&envelope.fixture),
            PUZZLE2D_PLAY_BODY_PROPERTIES => render_properties_panel(&envelope),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn window_engagements(&self, document_json: &str, _view_state: &ViewState) -> HashMap<String, WindowEngagement> {
        let envelope = parse_envelope(document_json);
        PUZZLE2D_PANES.iter().map(|pane| (pane.to_string(), puzzle2d_engagement(&envelope, &self.host, pane))).collect()
    }

    fn window_measures(&self, document_json: &str, _view_state: &ViewState) -> HashMap<String, Vec<WindowMeasure>> {
        let envelope = parse_envelope(document_json);
        PUZZLE2D_PANES.iter().map(|pane| (pane.to_string(), puzzle2d_window_measures(pane, &envelope))).collect()
    }
}
//#endregion 🔖Puzzle2dPlayApp

//#region 🔖AppFactory
pub fn create_puzzle2d_app() -> App {
    let mut host = puzzle_board_host();
    let envelope = default_envelope();
    sync_host_from_envelope(&mut host, &envelope);
    let mut app = App::from_builder(
        App::builder(PUZZLE2D_PLAY_APP_ID, "Puzzle 2D")
            .document(["semio", "puzzle", "2d"])
            .icon_id("puzzle2d")
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind_with_engagement(PUZZLE2D_PANE_OVERVIEW, "Overview", PUZZLE2D_PLAY_BODY_OVERVIEW, SurfaceKind::Canvas2d, puzzle2d_engagement(&envelope, &host, PUZZLE2D_PANE_OVERVIEW))
            .window_kind_with_engagement(PUZZLE2D_PANE_DETAIL, "Detail", PUZZLE2D_PLAY_BODY_DETAIL, SurfaceKind::Canvas2d, puzzle2d_engagement(&envelope, &host, PUZZLE2D_PANE_DETAIL))
            .window_kind_with_engagement(PUZZLE2D_PANE_SELECTION, "Selection", PUZZLE2D_PLAY_BODY_SELECTION, SurfaceKind::Canvas2d, puzzle2d_engagement(&envelope, &host, PUZZLE2D_PANE_SELECTION))
            .panel_tab("framework.panel.document", FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, PanelGroup::Workbench, PUZZLE2D_PLAY_BODY_LAYERS)
            .panel_tab("framework.panel.catalogue", FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, PanelGroup::Workbench, PUZZLE2D_PLAY_BODY_CATALOGUE)
            .panel_tab("framework.panel.inspection", FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, PanelGroup::Details, PUZZLE2D_PLAY_BODY_PROPERTIES)
            .default_layout(create_default_layout(&[PUZZLE2D_PANE_OVERVIEW.into(), PUZZLE2D_PANE_DETAIL.into(), PUZZLE2D_PANE_SELECTION.into()], "row", Some(&[50.0, 25.0, 25.0]), Some(&["Overview".into(), "Detail".into(), "Selection".into()]))),
    );
    for pane in PUZZLE2D_PANES {
        if let Some(window) = app.definition.window_kinds.iter_mut().find(|window| window.id == pane) {
            window.measures = puzzle2d_window_measures(pane, &envelope);
        }
    }
    app.example("empty", "Empty", serde_json::to_string(&default_envelope()).unwrap())
        .example(
            PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID,
            "Concrete Forest",
            serde_json::to_string(&Puzzle2dPlayEnvelope { fixture: serde_json::from_str(CONCRETE_FOREST_EXAMPLE_JSON).unwrap_or_else(|_| default_empty_fixture()), runtime: Puzzle2dPlayRuntime::default() }).unwrap(),
        )
        .example(
            PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID,
            "Nakagin Capsule Tower",
            serde_json::to_string(&Puzzle2dPlayEnvelope { fixture: serde_json::from_str(NAKAGIN_EXAMPLE_JSON).unwrap_or_else(|_| default_empty_fixture()), runtime: Puzzle2dPlayRuntime::default() }).unwrap(),
        )
        .program("puzzle2d", "Puzzle 2D", "layout")
}

fn puzzle2d_document_json_to_svg(value: &Value) -> Result<(String, u32, u32), String> {
    semio_framework_os::title_card_svg(value, "Puzzle 2D", 1024, 768)
}

pub fn register_puzzle2d_exports() {
    semio_framework_os::register_2d_svg_png_export_handlers("2d.puzzle", "puzzle2d", puzzle2d_document_json_to_svg);
}
//#endregion 🔖AppFactory

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::PluginApp;

    #[test]
    fn renders_puzzle2d_board_scene() {
        let app = Puzzle2dPlayApp::default();
        let document = app.initial_document_json();
        let node = app.render(PUZZLE2D_PLAY_BODY_OVERVIEW, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("puzzle2d-board"));
    }

    #[test]
    fn app_definition_has_three_lod_pane_window_kinds() {
        let app = create_puzzle2d_app();
        let ids: Vec<&str> = app.definition.window_kinds.iter().map(|window| window.id.as_str()).collect();
        assert_eq!(ids, vec![PUZZLE2D_PANE_OVERVIEW, PUZZLE2D_PANE_DETAIL, PUZZLE2D_PANE_SELECTION]);
        for window in &app.definition.window_kinds {
            assert!(window.engagement.is_some(), "pane {} must have engagement", window.id);
            assert!(!window.measures.is_empty(), "pane {} must have LOD/suggestion measures", window.id);
        }
    }

    #[test]
    fn renders_distinct_canvas_per_pane() {
        let app = Puzzle2dPlayApp::default();
        let envelope = Puzzle2dPlayEnvelope { fixture: serde_json::from_str(CONCRETE_FOREST_EXAMPLE_JSON).unwrap(), runtime: Puzzle2dPlayRuntime::default() };
        let document = serde_json::to_string(&envelope).unwrap();
        let overview_zoom = board_scene_zoom(&app.render(PUZZLE2D_PLAY_BODY_OVERVIEW, &document, &ViewState::default()));
        let detail_zoom = board_scene_zoom(&app.render(PUZZLE2D_PLAY_BODY_DETAIL, &document, &ViewState::default()));
        let selection_zoom = board_scene_zoom(&app.render(PUZZLE2D_PLAY_BODY_SELECTION, &document, &ViewState::default()));
        assert!(detail_zoom > overview_zoom, "detail {detail_zoom} should zoom in past overview {overview_zoom}");
        assert!(overview_zoom > selection_zoom, "overview {overview_zoom} should zoom in past selection {selection_zoom}");
    }

    fn board_scene_zoom(node: &UiNode) -> f64 {
        match node {
            UiNode::ComponentScene(scene) => {
                let camera_json = scene.puzzle2d_board.as_ref().expect("puzzle2d board scene").camera_json.clone();
                serde_json::from_str::<Value>(&camera_json).ok().and_then(|value| value.get("zoom").and_then(|zoom| zoom.as_f64())).unwrap_or(1.0)
            }
            other => panic!("expected component scene, got {other:?}"),
        }
    }

    #[test]
    fn set_lod_mode_for_pane_persists_per_pane_state() {
        let mut app = Puzzle2dPlayApp::default();
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops("setLodModeForPane", Some(&json!({ "pane": PUZZLE2D_PANE_DETAIL, "value": "compact" })), &document, &ViewState::default());
        let envelope: Puzzle2dPlayEnvelope = apply_document_op(&document, &ops[0]);
        assert_eq!(envelope.runtime.lod_mode_by_pane.get(PUZZLE2D_PANE_DETAIL).map(String::as_str), Some("compact"));
        assert_eq!(envelope.runtime.lod_mode_by_pane.get(PUZZLE2D_PANE_OVERVIEW).map(String::as_str), Some(PUZZLE2D_LOD_MODE_AUTOMATIC));
    }

    #[test]
    fn engagement_input_and_submit_round_trip_sets_active_tool() {
        let mut app = Puzzle2dPlayApp::default();
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops("engagementInput", Some(&json!({ "pane": PUZZLE2D_PANE_OVERVIEW, "value": "brush" })), &document, &ViewState::default());
        let envelope: Puzzle2dPlayEnvelope = apply_document_op(&document, &ops[0]);
        assert_eq!(envelope.runtime.engagement_input_by_pane.get(PUZZLE2D_PANE_OVERVIEW).map(String::as_str), Some("brush"));
        let document = serde_json::to_string(&envelope).unwrap();
        let ops = app.handle_command_patch_ops("engagementSubmit", Some(&json!({ "pane": PUZZLE2D_PANE_OVERVIEW, "value": "brush" })), &document, &ViewState::default());
        let envelope: Puzzle2dPlayEnvelope = apply_document_op(&document, &ops[0]);
        assert_eq!(envelope.runtime.active_tool, "brush");
        assert_eq!(envelope.runtime.engagement_input_by_pane.get(PUZZLE2D_PANE_OVERVIEW).map(String::as_str), Some(""));
    }

    #[test]
    fn engagement_reflects_fill_slider_and_brush_candidate_control() {
        let app = Puzzle2dPlayApp::default();
        let mut envelope = parse_envelope(&app.initial_document_json());
        envelope.runtime.active_tool = "fill".into();
        envelope.runtime.fill_count = 7;
        let document = serde_json::to_string(&envelope).unwrap();
        let engagements = app.window_engagements(&document, &ViewState::default());
        let overview = engagements.get(PUZZLE2D_PANE_OVERVIEW).expect("overview engagement");
        assert!(matches!(overview.control, Some(WindowEngagementControl::Slider { .. })), "fill tool should show a slider control");
        assert!(overview.input.is_some(), "engagement REPL input should be present");

        envelope.runtime.active_tool = "brush".into();
        envelope.runtime.brush_candidates = vec![json!({ "nodeKind": "heavy", "targetHandleIndex": 0 }), json!({ "nodeKind": "light", "targetHandleIndex": 1 })];
        let document = serde_json::to_string(&envelope).unwrap();
        let engagements = app.window_engagements(&document, &ViewState::default());
        let overview = engagements.get(PUZZLE2D_PANE_OVERVIEW).expect("overview engagement");
        assert!(matches!(overview.control, Some(WindowEngagementControl::ToggleGroup { .. })), "brush tool with candidates should show a placement control");
    }

    #[test]
    fn suggestion_offset_and_brush_kind_weight_commands_persist() {
        let mut app = Puzzle2dPlayApp::default();
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops("setSuggestionOffset", Some(&json!({ "value": 40.0 })), &document, &ViewState::default());
        let envelope: Puzzle2dPlayEnvelope = apply_document_op(&document, &ops[0]);
        assert_eq!(envelope.runtime.suggestion_offset, 40.0);

        let document = serde_json::to_string(&envelope).unwrap();
        let ops = app.handle_command_patch_ops("setBrushKindWeights", Some(&json!({ "kindId": "heavy", "catalogSlice": "nodes", "value": 0.75 })), &document, &ViewState::default());
        let envelope: Puzzle2dPlayEnvelope = apply_document_op(&document, &ops[0]);
        assert_eq!(envelope.runtime.node_kind_weights.get("heavy").copied(), Some(0.75));
    }

    #[test]
    fn document_panel_lists_nodes_section() {
        let app = Puzzle2dPlayApp::default();
        let envelope = Puzzle2dPlayEnvelope { fixture: serde_json::from_str(CONCRETE_FOREST_EXAMPLE_JSON).unwrap(), runtime: Puzzle2dPlayRuntime::default() };
        let document = serde_json::to_string(&envelope).unwrap();
        let node = app.render(PUZZLE2D_PLAY_BODY_LAYERS, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("puzzle2d-play-document.nodes"));
        assert!(json.contains("seed-left-001"));
    }

    #[test]
    fn add_node_command_appends_node() {
        let mut app = Puzzle2dPlayApp::default();
        let document = app.initial_document_json();
        let ops = app.handle_command_patch_ops("addNode", Some(&json!({ "kind": "node" })), &document, &ViewState::default());
        assert_eq!(ops.len(), 1);
        let envelope: Puzzle2dPlayEnvelope = apply_document_op(&document, &ops[0]);
        assert_eq!(envelope.fixture.get("nodes").and_then(|value| value.as_array()).map(|values| values.len()), Some(1));
    }

    #[test]
    fn apply_board_events_selects_node_from_client_session() {
        let mut app = Puzzle2dPlayApp::default();
        let envelope = Puzzle2dPlayEnvelope { fixture: serde_json::from_str(CONCRETE_FOREST_EXAMPLE_JSON).unwrap(), runtime: Puzzle2dPlayRuntime::default() };
        let document = serde_json::to_string(&envelope).unwrap();
        let events_json = serde_json::json!([{
            "name": "select",
            "payload": { "ids": ["seed-left-001"] }
        }])
        .to_string();
        let ops = app.handle_command_patch_ops("applyBoardEvents", Some(&json!({ "eventsJson": events_json })), &document, &ViewState::default());
        let envelope: Puzzle2dPlayEnvelope = apply_document_op(&document, &ops[0]);
        assert!(envelope.runtime.selected_ids.iter().any(|id| id == "seed-left-001"), "expected seed-left-001 selected, got {:?}", envelope.runtime.selected_ids);
    }

    #[test]
    fn apply_board_events_camera_round_trips_to_overview_scene() {
        let mut app = Puzzle2dPlayApp::default();
        let envelope = Puzzle2dPlayEnvelope { fixture: serde_json::from_str(CONCRETE_FOREST_EXAMPLE_JSON).unwrap(), runtime: Puzzle2dPlayRuntime::default() };
        let document = serde_json::to_string(&envelope).unwrap();
        let camera = json!({ "x": 345.0, "y": -123.0, "zoom": 4.25 });
        let events_json = serde_json::json!([{
            "name": "camera",
            "payload": camera
        }])
        .to_string();
        let ops = app.handle_command_patch_ops("applyBoardEvents", Some(&json!({ "eventsJson": events_json })), &document, &ViewState::default());
        let envelope: Puzzle2dPlayEnvelope = apply_document_op(&document, &ops[0]);
        let node = app.render(PUZZLE2D_PLAY_BODY_OVERVIEW, &serde_json::to_string(&envelope).unwrap(), &ViewState::default());
        let scene_camera = board_scene_camera(&node);
        assert_eq!(scene_camera, camera);
    }

    fn board_scene_camera(node: &UiNode) -> Value {
        match node {
            UiNode::ComponentScene(scene) => {
                let camera_json = scene.puzzle2d_board.as_ref().expect("puzzle2d board scene").camera_json.clone();
                serde_json::from_str::<Value>(&camera_json).expect("camera json")
            }
            other => panic!("expected component scene, got {other:?}"),
        }
    }

    fn apply_document_op(document_json: &str, op_json: &str) -> Puzzle2dPlayEnvelope {
        let mut envelope = parse_envelope(document_json);
        if let Ok(op) = serde_json::from_str::<Value>(op_json) {
            if op.get("op").and_then(|value| value.as_str()) == Some("setDocument") {
                if let Some(document) = op.get("document") {
                    if let Ok(parsed) = serde_json::from_value(document.clone()) {
                        envelope = parsed;
                    }
                }
            }
        }
        envelope
    }
}
//#endregion 🧪Tests
