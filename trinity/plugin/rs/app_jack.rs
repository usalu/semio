//! 🔱 Trinity Jack plugin — jack query play app bundled as a hot-swappable WASM component.

use semio_framework_plugin::{SurfaceKind, PanelGroup,
    build_node_graph_scene, build_table_scene, build_text_editor_scene,
    text_identifier_occurrences_json, tool_button, tool_collection,
    ui_declarative_sections_to_tree, ui_inspector_groups_to_tree, ui_inspector_mixed_text,
    ui_inspector_readonly_field, ui_text, App, ActionDescriptor, NodeGraphScene, PluginApp,
    TableScene, TextEditorScene, ToolCategory, ToolNode, UiFieldNode, UiInspectorFieldGroup, UiNode, UiSectionNode, UiTreeItemNode,
    UiTreeNode, UiTreeSectionNode, ViewState, WindowLayout, WindowLayoutAxisNode, WindowLayoutChild,
    WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode, WindowMeasure, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
    FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, UI_INSPECTOR_MIXED_PLACEHOLDER,
};
use semio_framework_plugin::layout::MeasureSelectItem;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use trinity_jack::{complete, execute, format as jack_format, lint, parse, run_json, semantic_tokens, QueryResult, QueryResultKind};
use trinity_ram::{Graph, GraphFixture, Node, PortDirection, PropertyValue, create_trinity_graph_envelope, dispatch_trinity_graph_ops, TrinityGraphEnvelope, TrinityGraphStore};
use vcs::DocumentVcsCommand;

//#region 🔖Constants
const TRINITY_JACK_PLAY_APP_ID: &str = "trinity-jack-play";
const TRINITY_JACK_PLAY_CONTROLLER_ID: &str = "trinity-jack-play";
const TRINITY_JACK_PLAY_SURFACE_GRAPH: &str = "trinity.jack.play";
const TRINITY_JACK_PLAY_SURFACE_EDITOR: &str = "trinity.jack.editor";
const TRINITY_JACK_PLAY_SURFACE_RESULTS: &str = "trinity.jack.results";
const TRINITY_JACK_PLAY_BODY_GRAPH: &str = "trinity.jack.play.main";
const TRINITY_JACK_PLAY_BODY_EDITOR: &str = "trinity.jack.play.editor";
const TRINITY_JACK_PLAY_BODY_RESULTS: &str = "trinity.jack.play.results";
const TRINITY_JACK_PLAY_BODY_DOCUMENT: &str = "trinity.jack.play.document";
const TRINITY_JACK_PLAY_BODY_CATALOGUE: &str = "trinity.jack.play.catalogue";
const TRINITY_JACK_PLAY_BODY_INSPECTION: &str = "trinity.jack.play.inspection";
const TRINITY_JACK_PLAY_WINDOW_GRAPH: &str = "trinity-jack-graph";
const TRINITY_JACK_PLAY_WINDOW_EDITOR: &str = "trinity-jack-editor";
const TRINITY_JACK_PLAY_WINDOW_RESULTS: &str = "trinity-jack-results";

const NAKAGIN_FIXTURE_JSON: &str = include_str!("../../example/nakagin-capsule-tower.trinity.json");
const BRANCH_FIXTURE_JSON: &str = include_str!("../../example/branch-chain.trinity.json");

const TRINITY_JACK_DEFAULT_QUERY: &str =
    "MATCH (a:Piece)-[r:Connection]->(b:Piece) WHERE a.name = 'b' AND b.name != 'b' RETURN a.name, b.name, b.label";

const TRINITY_LOD_MODE_AUTOMATIC: &str = "automatic";
//#endregion 🔖Constants

//#region 🔖Envelope
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TrinityEditorSelection {
    start: usize,
    end: usize,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TrinityJackRuntime {
    #[serde(default)]
    selected_node_ids: Vec<String>,
    #[serde(default)]
    active_fixture_id: String,
    #[serde(default)]
    jack_query: String,
    #[serde(default)]
    jack_result_json: String,
    #[serde(default)]
    editor_engagement_input: String,
    #[serde(default)]
    graph_engagement_input: String,
    #[serde(default)]
    results_engagement_input: String,
    #[serde(default)]
    reorganize_epoch: u64,
    #[serde(default)]
    editor_selection: Option<TrinityEditorSelection>,
    #[serde(default)]
    lod_mode_by_window: BTreeMap<String, String>,
    #[serde(default)]
    revision: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TrinityJackEnvelope {
    fixture_json: String,
    #[serde(default)]
    graph_vcs: Option<TrinityGraphEnvelope>,
    #[serde(default)]
    graph_applied_edit_ids: Vec<String>,
    #[serde(default)]
    runtime: TrinityJackRuntime,
}

fn default_envelope() -> TrinityJackEnvelope {
    TrinityJackEnvelope {
        fixture_json: NAKAGIN_FIXTURE_JSON.into(),
        graph_vcs: None,
        graph_applied_edit_ids: Vec::new(),
        runtime: TrinityJackRuntime {
            active_fixture_id: "nakagin".into(),
            jack_query: TRINITY_JACK_DEFAULT_QUERY.into(),
            ..Default::default()
        },
    }
}

fn parse_envelope(document_json: &str) -> TrinityJackEnvelope {
    serde_json::from_str(document_json).unwrap_or_else(|_| default_envelope())
}

fn set_document_op(envelope: &TrinityJackEnvelope) -> String {
    json!({ "op": "setDocument", "document": envelope }).to_string()
}

fn jack_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor {
        controller_id: TRINITY_JACK_PLAY_CONTROLLER_ID.into(),
        action: action.into(),
        args,
    }
}

fn parse_fixture_json(json: &str) -> Option<GraphFixture> {
    GraphFixture::from_json(json).ok()
}

fn load_graph(fixture_json: &str) -> Graph {
    Graph::load_json(fixture_json).unwrap_or_else(|_| {
        let fixture = GraphFixture::from_json(NAKAGIN_FIXTURE_JSON).expect("nakagin fixture");
        Graph::from_fixture(fixture).expect("nakagin graph")
    })
}

fn fixture_with_derived(fixture_json: &str) -> Option<GraphFixture> {
    let mut graph = Graph::load_json(fixture_json).ok()?;
    graph.recompute_derived();
    Some(graph.to_fixture())
}

fn fixture_json_for_preset(preset_id: &str) -> Option<&'static str> {
    match preset_id {
        "nakagin" | "nakagin-capsule-tower" => Some(NAKAGIN_FIXTURE_JSON),
        "branch-chain" => Some(BRANCH_FIXTURE_JSON),
        _ => None,
    }
}

fn preset_query(preset_id: &str) -> &'static str {
    match preset_id {
        "branch-chain" => "MATCH (a:Piece)-[r:Connection]->(b:Piece) RETURN a, r, b",
        _ => TRINITY_JACK_DEFAULT_QUERY,
    }
}

fn property_value_to_string(value: &PropertyValue) -> String {
    match value {
        PropertyValue::String(text) => text.clone(),
        PropertyValue::Number(number) => number.to_string(),
        PropertyValue::Bool(flag) => flag.to_string(),
        PropertyValue::Null => "null".into(),
        PropertyValue::Array(items) => serde_json::to_string(items).unwrap_or_else(|_| "[]".into()),
        PropertyValue::Object(map) => serde_json::to_string(map).unwrap_or_else(|_| "{}".into()),
    }
}

fn graph_store_from_envelope(envelope: &TrinityJackEnvelope) -> TrinityGraphStore {
    if let Some(vcs) = &envelope.graph_vcs {
        let mut store = TrinityGraphStore::new(vcs.clone());
        store.set_envelope(vcs.clone(), envelope.graph_applied_edit_ids.clone());
        return store;
    }
    let fixture = GraphFixture::from_json(&envelope.fixture_json)
        .or_else(|_| GraphFixture::from_json(NAKAGIN_FIXTURE_JSON))
        .unwrap_or_else(|_| trinity_ram::empty_trinity_graph_fixture());
    TrinityGraphStore::new(create_trinity_graph_envelope("trinity-jack", fixture))
}

fn sync_envelope_from_store(envelope: &mut TrinityJackEnvelope, store: &TrinityGraphStore) {
    envelope.graph_vcs = Some(store.envelope().clone());
    envelope.graph_applied_edit_ids = store.applied_edit_ids().to_vec();
    if let Ok(fixture) = store.projection() {
        if let Ok(json) = Graph::from_fixture(fixture).and_then(|graph| graph.fixture_json()) {
            envelope.fixture_json = json;
        }
    }
}

fn run_jack_on_fixture(fixture_json: &str, query: &str) -> (String, String) {
    let mut graph = load_graph(fixture_json);
    match run_json(&mut graph, query) {
        Ok(result_json) => {
            let fixture_out = graph.fixture_json().unwrap_or_else(|_| fixture_json.into());
            (result_json, fixture_out)
        }
        Err(error) => (
            serde_json::to_string(&json!({ "error": error })).unwrap_or_else(|_| "{}".into()),
            fixture_json.into(),
        ),
    }
}

fn run_jack_with_vcs(envelope: &mut TrinityJackEnvelope, query: &str) -> Result<(), String> {
    let mut store = graph_store_from_envelope(envelope);
    let graph = load_graph(&envelope.fixture_json);
    let parsed = parse(query)?;
    let (result, ops) = execute(&graph, &parsed)?;
    envelope.runtime.jack_result_json = serde_json::to_string(&result).map_err(|e| e.to_string())?;
    if !ops.is_empty() {
        dispatch_trinity_graph_ops(&mut store, ops)?;
        sync_envelope_from_store(envelope, &store);
    }
    Ok(())
}

fn force_layout_fixture_json(fixture_json: &str) -> Option<String> {
    let mut fixture = GraphFixture::from_json(fixture_json).ok()?;
    if fixture.nodes.is_empty() {
        return None;
    }
    use mathematical_core::force_layout::{run_force_layout, ForceLayoutOptions, Vec2};
    let mut positions: Vec<Vec2> = fixture.nodes.iter().map(|node| Vec2::new(node.x, node.y)).collect();
    let radii: Vec<f64> = fixture.nodes.iter().map(|node| (node.width.max(48.0) + node.height.max(24.0)) * 0.25).collect();
    let id_to_index: std::collections::HashMap<String, usize> = fixture
        .nodes
        .iter()
        .enumerate()
        .map(|(index, node)| (node.id.clone(), index))
        .collect();
    let mut edge_pairs = Vec::new();
    for edge in &fixture.edges {
        let (source_node, _) = split_endpoint(&edge.source);
        let (target_node, _) = split_endpoint(&edge.target);
        if let (Some(a), Some(b)) = (id_to_index.get(&source_node), id_to_index.get(&target_node)) {
            edge_pairs.push((*a, *b));
        }
    }
    let pin = vec![None; positions.len()];
    run_force_layout(
        &mut positions,
        &radii,
        &edge_pairs,
        &pin,
        &ForceLayoutOptions {
            iterations: 120,
            ..ForceLayoutOptions::default()
        },
    );
    for (index, node) in fixture.nodes.iter_mut().enumerate() {
        node.x = positions[index].x;
        node.y = positions[index].y;
    }
    Graph::from_fixture(fixture).ok()?.fixture_json().ok()
}

fn selection_ids(args: Option<&Value>) -> Vec<String> {
    args.and_then(|value| value.get("nodeIds"))
        .and_then(|value| serde_json::from_value(value.clone()).ok())
        .or_else(|| {
            args.and_then(|value| value.get("ids"))
                .and_then(|value| serde_json::from_value(value.clone()).ok())
        })
        .or_else(|| {
            args.and_then(|value| value.get("nodeId"))
                .and_then(|value| value.as_str())
                .map(|id| vec![id.to_string()])
        })
        .unwrap_or_default()
}

fn remove_nodes_from_fixture_json(fixture_json: &str, node_ids: &[String]) -> Option<String> {
    let mut fixture = parse_fixture_json(fixture_json)?;
    fixture.nodes.retain(|node| !node_ids.contains(&node.id));
    fixture.edges.retain(|edge| {
        let from = edge.source.split(':').next().unwrap_or(&edge.source);
        let to = edge.target.split(':').next().unwrap_or(&edge.target);
        !node_ids.iter().any(|id| id == from || id == to)
    });
    Graph::from_fixture(fixture).ok()?.fixture_json().ok()
}

fn apply_node_graph_edit_ops(envelope: &mut TrinityJackEnvelope, ops: &[Value]) -> bool {
    let mut changed = false;
    for op in ops {
        match op.get("op").and_then(|value| value.as_str()).unwrap_or("") {
            "setFixture" => {
                if let Some(fixture_json) = op.get("fixtureJson").and_then(|value| value.as_str()) {
                    if parse_fixture_json(fixture_json).is_some() {
                        envelope.fixture_json = fixture_json.into();
                        changed = true;
                    }
                }
            }
            "deleteSelection" => {
                if !envelope.runtime.selected_node_ids.is_empty() {
                    if let Some(next) =
                        remove_nodes_from_fixture_json(&envelope.fixture_json, &envelope.runtime.selected_node_ids)
                    {
                        envelope.fixture_json = next;
                        envelope.runtime.selected_node_ids.clear();
                        changed = true;
                    }
                }
            }
            _ => {}
        }
    }
    changed
}
//#endregion 🔖Envelope

//#region 🔖Lod
fn trinity_lod_tier_rows() -> Vec<Value> {
    serde_json::from_str(&trinity_rewrite::trinity_lod_scale_json()).unwrap_or_default()
}

fn trinity_lod_measure(window_id: &str, current_mode: &str) -> WindowMeasure {
    let mut items = vec![MeasureSelectItem {
        id: TRINITY_LOD_MODE_AUTOMATIC.into(),
        value: TRINITY_LOD_MODE_AUTOMATIC.into(),
        label: "Automatic".into(),
    }];
    items.extend(trinity_lod_tier_rows().into_iter().filter_map(|row| {
        let id = row.get("id")?.as_str()?.to_string();
        let name = row.get("name").and_then(|value| value.as_str()).unwrap_or(&id).to_string();
        Some(MeasureSelectItem { id: id.clone(), value: id, label: name })
    }));
    WindowMeasure::Select {
        id: format!("{window_id}-lod"),
        label: Some("LOD".into()),
        value: current_mode.into(),
        items,
        on_change: jack_action("setLodMode", Some(json!({ "windowId": window_id }))),
    }
}

fn trinity_lod_json_for_window(runtime: &TrinityJackRuntime, window_id: &str) -> Option<String> {
    let mode = runtime.lod_mode_by_window.get(window_id).map(String::as_str).unwrap_or(TRINITY_LOD_MODE_AUTOMATIC);
    if mode == TRINITY_LOD_MODE_AUTOMATIC {
        Some(json!({ "automatic": true }).to_string())
    } else {
        Some(json!({ "automatic": false, "forcedLabel": mode }).to_string())
    }
}
//#endregion 🔖Lod

//#region 🔖MediaGraph
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MediaGraphPortRecord {
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MediaGraphNodeRecord {
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    inputs: Vec<MediaGraphPortRecord>,
    outputs: Vec<MediaGraphPortRecord>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MediaGraphEdgeRecord {
    id: String,
    source_node_id: String,
    source_port_id: String,
    target_node_id: String,
    target_port_id: String,
}

fn port_endpoint(node_id: &str, port_id: &str) -> String {
    format!("{node_id}:{port_id}")
}

fn split_endpoint(endpoint: &str) -> (String, String) {
    endpoint
        .split_once(':')
        .map(|(node, port)| (node.to_string(), port.to_string()))
        .unwrap_or_else(|| (endpoint.to_string(), "in".into()))
}

fn fixture_to_media_graph(fixture: &GraphFixture) -> (String, String, String) {
    let nodes: Vec<MediaGraphNodeRecord> = fixture
        .nodes
        .iter()
        .map(|node| node_to_media_record(node))
        .collect();
    let edges: Vec<MediaGraphEdgeRecord> = fixture
        .edges
        .iter()
        .map(|edge| {
            let (source_node_id, source_port_id) = split_endpoint(&edge.source);
            let (target_node_id, target_port_id) = split_endpoint(&edge.target);
            MediaGraphEdgeRecord {
                id: edge.id.clone(),
                source_node_id,
                source_port_id,
                target_node_id,
                target_port_id,
            }
        })
        .collect();
    let viewport = serde_json::to_string(&fixture.camera).unwrap_or_else(|_| r#"{"x":0,"y":0,"zoom":1}"#.into());
    (
        serde_json::to_string(&nodes).unwrap_or_else(|_| "[]".into()),
        serde_json::to_string(&edges).unwrap_or_else(|_| "[]".into()),
        viewport,
    )
}

fn node_to_media_record(node: &Node) -> MediaGraphNodeRecord {
    let width = if node.width > 0.0 { node.width } else { 96.0 };
    let height = if node.height > 0.0 { node.height } else { 48.0 };
    MediaGraphNodeRecord {
        id: node.id.clone(),
        label: Some(if node.name.is_empty() { node.id.clone() } else { node.name.clone() }),
        x: node.x,
        y: node.y,
        width,
        height,
        inputs: node
            .ports
            .iter()
            .filter(|port| port.direction == PortDirection::In)
            .map(|port| MediaGraphPortRecord {
                id: port_endpoint(&node.id, &port.id),
                label: Some(port.id.clone()),
            })
            .collect(),
        outputs: node
            .ports
            .iter()
            .filter(|port| port.direction == PortDirection::Out)
            .map(|port| MediaGraphPortRecord {
                id: port_endpoint(&node.id, &port.id),
                label: Some(port.id.clone()),
            })
            .collect(),
    }
}

fn result_to_table(result_json: &str) -> (String, String) {
    let parsed: QueryResult = serde_json::from_str(result_json).unwrap_or(QueryResult::table(vec![], vec![]));
    let columns: Vec<Value> = parsed
        .columns
        .iter()
        .map(|column| json!({ "id": column, "label": column }))
        .collect();
    let rows: Vec<Value> = parsed
        .rows
        .iter()
        .enumerate()
        .map(|(index, row)| {
            let mut record = serde_json::Map::new();
            record.insert("index".into(), json!(index + 1));
            for (column, value) in parsed.columns.iter().zip(row.iter()) {
                record.insert(column.clone(), json!(property_value_to_string(value)));
            }
            Value::Object(record)
        })
        .collect();
    (
        serde_json::to_string(&columns).unwrap_or_else(|_| "[]".into()),
        serde_json::to_string(&rows).unwrap_or_else(|_| "[]".into()),
    )
}
//#endregion 🔖MediaGraph

//#region 🔖Terminology
/// 🗣️ Complete UI label set for the Jack query app; one field per label makes every locale combination compile-checked.
struct TrinityJackLabels {
    pieces: &'static str,
    connections: &'static str,
    fixtures: &'static str,
    example_queries: &'static str,
    manifest_kinds: &'static str,
    piece: &'static str,
    connection: &'static str,
    connector: &'static str,
    geometry: &'static str,
    identity: &'static str,
    history: &'static str,
    query: &'static str,
}

const TRINITY_JACK_LABELS_NATIVE_EN: TrinityJackLabels = TrinityJackLabels {
    pieces: "Pieces",
    connections: "Connections",
    fixtures: "Fixtures",
    example_queries: "Example queries",
    manifest_kinds: "Manifest kinds",
    piece: "Piece",
    connection: "Connection",
    connector: "Connector",
    geometry: "Geometry",
    identity: "Identity",
    history: "History",
    query: "Query",
};

const TRINITY_JACK_LABELS_NATIVE_DE: TrinityJackLabels = TrinityJackLabels {
    pieces: "Stücke",
    connections: "Verbindungen",
    fixtures: "Fixturen",
    example_queries: "Beispielabfragen",
    manifest_kinds: "Manifestarten",
    piece: "Stück",
    connection: "Verbindung",
    connector: "Verbinder",
    geometry: "Geometrie",
    identity: "Identität",
    history: "Verlauf",
    query: "Abfrage",
};

/// 🗣️ Resolves the active label set from the shell-provided locale; unknown locales fall back to native English.
fn trinity_jack_labels(view_state: &ViewState) -> &'static TrinityJackLabels {
    let is_de = view_state.locale.as_deref().is_some_and(|locale| locale.starts_with("de"));
    if is_de { &TRINITY_JACK_LABELS_NATIVE_DE } else { &TRINITY_JACK_LABELS_NATIVE_EN }
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
    description: Option<String>,
    action: ActionDescriptor,
) -> UiTreeItemNode {
    UiTreeItemNode {
        id: id.into(),
        label: label.into(),
        description,
        icon_id: None,
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

fn flat_position_uv(node: &Node) -> (String, String) {
    let Some(flat) = node.properties.get("flatPosition").and_then(PropertyValue::as_object) else {
        return (String::new(), String::new());
    };
    let format_axis = |axis: &str| flat.get(axis).and_then(PropertyValue::as_f64).map(|value| format!("{value:.2}")).unwrap_or_default();
    (format_axis("u"), format_axis("v"))
}

fn build_document_tree(envelope: &TrinityJackEnvelope, labels: &TrinityJackLabels) -> UiNode {
    let Some(fixture) = parse_fixture_json(&envelope.fixture_json) else {
        return ui_text("Invalid trinity fixture");
    };
    let node_items: Vec<UiTreeItemNode> = fixture
        .nodes
        .iter()
        .map(|node| {
            tree_item_with_action(
                format!("trinity-document.node.{}", node.id),
                if node.name.is_empty() { node.id.clone() } else { node.name.clone() },
                Some(node.kind.clone()),
                jack_action("setSelection", Some(json!({ "ids": [node.id] }))),
            )
        })
        .collect();
    let edge_items: Vec<UiTreeItemNode> = fixture
        .edges
        .iter()
        .map(|edge| tree_item(
            format!("trinity-document.edge.{}", edge.id),
            format!("{} → {}", edge.source, edge.target),
        ))
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![
            UiTreeSectionNode {
                id: "trinity-document.nodes".into(),
                label: Some(labels.pieces.into()),
                default_open: Some(true),
                items: node_items,
            },
            UiTreeSectionNode {
                id: "trinity-document.edges".into(),
                label: Some(labels.connections.into()),
                default_open: Some(false),
                items: edge_items,
            },
        ],
        selected_ids: Some(
            envelope
                .runtime
                .selected_node_ids
                .iter()
                .map(|id| format!("trinity-document.node.{id}"))
                .collect(),
        ),
        highlighted_ids: None,
        selection_change: Some(jack_action("setSelection", Some(json!({ "ids": [] })))),
        drop_action: None,
    })
}

fn build_catalogue_tree(envelope: &TrinityJackEnvelope, labels: &TrinityJackLabels) -> UiNode {
    let fixtures = [("nakagin", "Nakagin — Table"), ("branch-chain", "Branch — Graph")];
    let examples = [
        ("where-or", "Where Or", "MATCH (a:Piece) WHERE a.name = 't_f0_b_c0' OR a.name = 't_f0_b_c1' RETURN a.name"),
        ("return-graph", "Return Graph", "MATCH (a:Piece)-[r:Connection]->(b:Piece) WHERE a.name = 'b' RETURN a, r, b"),
        ("set-label", "Set Label", "MATCH (a:Piece) WHERE a.name = 'b' SET a.label = 'demo-label'"),
        ("set-position", "Set Position", "MATCH (a:Piece) WHERE a.name = 'b' SET a.x = 300, a.y = 120"),
        ("create-node", "Create Node", "CREATE (n:Piece)"),
        ("create-edge", "Create Edge", "MATCH (a:Piece), (b:Piece) WHERE a.name = 'b' AND b.name != 'b' CREATE (a)-[:Connection]->(b)"),
        ("delete-leaf", "Delete Leaf", "MATCH (n:Piece) WHERE n.name = 'b' DELETE n"),
        ("merge-edge", "Merge Edge", "MERGE (x:Piece)-[:Connection]->(y:Piece)"),
    ];
    UiNode::Tree(UiTreeNode {
        sections: vec![
            UiTreeSectionNode {
                id: "trinity-jack-catalogue.fixtures".into(),
                label: Some(labels.fixtures.into()),
                default_open: Some(true),
                items: fixtures
                    .iter()
                    .map(|(id, label)| {
                        tree_item_with_action(
                            format!("trinity-jack-catalogue.fixture.{id}"),
                            *label,
                            Some(preset_query(id).into()),
                            jack_action("setActiveExample", Some(json!({ "exampleId": id }))),
                        )
                    })
                    .collect(),
            },
            UiTreeSectionNode {
                id: "trinity-jack-catalogue.examples".into(),
                label: Some(labels.example_queries.into()),
                default_open: Some(true),
                items: examples
                    .iter()
                    .map(|(id, label, query)| {
                        tree_item_with_action(
                            format!("trinity-jack-catalogue.example.{id}"),
                            *label,
                            Some((*query).into()),
                            jack_action("loadExampleQuery", Some(json!({ "query": query }))),
                        )
                    })
                    .collect(),
            },
            UiTreeSectionNode {
                id: "trinity-jack-catalogue.kinds".into(),
                label: Some(labels.manifest_kinds.into()),
                default_open: Some(false),
                items: vec![
                    tree_item("trinity-jack-catalogue.piece", labels.piece),
                    tree_item("trinity-jack-catalogue.connection", labels.connection),
                    tree_item("trinity-jack-catalogue.connector", labels.connector),
                ],
            },
        ],
        selected_ids: if envelope.runtime.active_fixture_id.is_empty() {
            Some(vec![])
        } else {
            Some(vec![format!(
                "trinity-jack-catalogue.fixture.{}",
                envelope.runtime.active_fixture_id
            )])
        },
        highlighted_ids: None,
        selection_change: None,
        drop_action: None,
    })
}

fn build_inspector_tree(envelope: &TrinityJackEnvelope, term_labels: &TrinityJackLabels) -> UiNode {
    let Some(fixture) = parse_fixture_json(&envelope.fixture_json) else {
        return ui_text("Invalid trinity fixture");
    };
    if envelope.runtime.selected_node_ids.is_empty() {
        return ui_declarative_sections_to_tree(&[UiSectionNode {
            id: "trinity-inspector.empty".into(),
            label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
            default_open: Some(true),
            children: vec![ui_text("Select one or more pieces")],
        }]);
    }
    let nodes: Vec<&Node> = envelope
        .runtime
        .selected_node_ids
        .iter()
        .filter_map(|id| fixture.nodes.iter().find(|node| &node.id == id))
        .collect();
    if nodes.is_empty() {
        return ui_declarative_sections_to_tree(&[UiSectionNode {
            id: "trinity-inspector.missing".into(),
            label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
            default_open: Some(true),
            children: vec![ui_text("Piece not found")],
        }]);
    }
    let node_ids: Vec<String> = nodes.iter().map(|node| node.id.clone()).collect();
    let name_mixed = ui_inspector_mixed_text(&nodes.iter().map(|node| node.name.clone()).collect::<Vec<_>>());
    let kind_mixed = ui_inspector_mixed_text(&nodes.iter().map(|node| node.kind.clone()).collect::<Vec<_>>());
    let port_counts: Vec<String> = nodes.iter().map(|node| node.ports.len().to_string()).collect();
    let ports_mixed = ui_inspector_mixed_text(&port_counts);
    let derived_fixture = fixture_with_derived(&envelope.fixture_json);
    let derived_uv = |id: &str| -> (String, String) {
        derived_fixture
            .as_ref()
            .and_then(|fixture| fixture.nodes.iter().find(|node| node.id == id))
            .map(flat_position_uv)
            .unwrap_or_default()
    };
    let u_values: Vec<String> = node_ids.iter().map(|id| derived_uv(id).0).collect();
    let v_values: Vec<String> = node_ids.iter().map(|id| derived_uv(id).1).collect();
    let u_mixed = ui_inspector_mixed_text(&u_values);
    let v_mixed = ui_inspector_mixed_text(&v_values);
    ui_inspector_groups_to_tree(&[
        UiInspectorFieldGroup {
            id: "trinity-inspector.geometry".into(),
            label: term_labels.geometry.into(),
            default_open: None,
            fields: vec![
                ui_inspector_readonly_field(
                    "trinity-inspector.flat-u",
                    "Flat U",
                    if u_mixed.placeholder.is_none() {
                        u_values.first().cloned().unwrap_or_default()
                    } else {
                        u_mixed.placeholder.unwrap_or_else(|| UI_INSPECTOR_MIXED_PLACEHOLDER.into())
                    },
                ),
                ui_inspector_readonly_field(
                    "trinity-inspector.flat-v",
                    "Flat V",
                    if v_mixed.placeholder.is_none() {
                        v_values.first().cloned().unwrap_or_default()
                    } else {
                        v_mixed.placeholder.unwrap_or_else(|| UI_INSPECTOR_MIXED_PLACEHOLDER.into())
                    },
                ),
                ui_inspector_readonly_field(
                    "trinity-inspector.ports",
                    "Connectors",
                    if ports_mixed.placeholder.is_none() {
                        port_counts.first().cloned().unwrap_or_default()
                    } else {
                        ports_mixed.placeholder.unwrap_or_else(|| UI_INSPECTOR_MIXED_PLACEHOLDER.into())
                    },
                ),
            ],
        },
        UiInspectorFieldGroup {
            id: "trinity-inspector.identity".into(),
            label: term_labels.identity.into(),
            default_open: None,
            fields: vec![
                semio_framework_plugin::UiNode::Field(UiFieldNode {
                    id: "trinity-inspector.name".into(),
                    label: "Name".into(),
                    child: Box::new(semio_framework_plugin::UiNode::Input(semio_framework_plugin::UiInputNode {
                        id: "trinity-inspector.name.input".into(),
                        input_kind: "text".into(),
                        value: name_mixed.value,
                        placeholder: name_mixed.placeholder,
                        commit: None,
                        on_change: jack_action(
                            "patchTrinityNodes",
                            Some(json!({ "nodeIds": node_ids, "field": "name" })),
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
                ui_inspector_readonly_field(
                    "trinity-inspector.kind",
                    "Kind",
                    if kind_mixed.placeholder.is_none() {
                        nodes.first().map(|node| node.kind.clone()).unwrap_or_default()
                    } else {
                        kind_mixed.placeholder.unwrap_or_else(|| UI_INSPECTOR_MIXED_PLACEHOLDER.into())
                    },
                ),
                ui_inspector_readonly_field(
                    "trinity-inspector.id",
                    "Id",
                    if node_ids.len() == 1 {
                        node_ids.first().cloned().unwrap_or_default()
                    } else {
                        format!("{} selected", node_ids.len())
                    },
                ),
            ],
        },
    ])
}
//#endregion 🔖Panels

//#region 🔖Render
fn render_graph(envelope: &TrinityJackEnvelope) -> UiNode {
    let fixture = parse_fixture_json(&envelope.fixture_json).unwrap_or_else(|| GraphFixture::from_json(NAKAGIN_FIXTURE_JSON).unwrap());
    let (nodes_json, edges_json, viewport_json) = fixture_to_media_graph(&fixture);
    let selection_json = if envelope.runtime.selected_node_ids.is_empty() {
        None
    } else {
        serde_json::to_string(&envelope.runtime.selected_node_ids).ok()
    };
    build_node_graph_scene(
        TRINITY_JACK_PLAY_SURFACE_GRAPH,
        TRINITY_JACK_PLAY_CONTROLLER_ID,
        NodeGraphScene {
            selection_json,
            context_menu_json: Some(
                r#"[{"id":"delete-selection","label":"Delete selection","action":"nodeGraphEdit","args":{"ops":[{"op":"deleteSelection"}]}}]"#.into(),
            ),
            lod_json: trinity_lod_json_for_window(&envelope.runtime, TRINITY_JACK_PLAY_WINDOW_GRAPH),
            ..NodeGraphScene::base(nodes_json, edges_json, viewport_json)
        },
    )
}

fn render_editor(envelope: &TrinityJackEnvelope) -> UiNode {
    let query = &envelope.runtime.jack_query;
    let graph = load_graph(&envelope.fixture_json);
    let cursor = envelope.runtime.editor_selection.as_ref().map(|selection| selection.end).unwrap_or(0);
    let selection_json = envelope
        .runtime
        .editor_selection
        .as_ref()
        .map(|selection| json!({ "start": selection.start, "end": selection.end }).to_string());
    build_text_editor_scene(
        TRINITY_JACK_PLAY_SURFACE_EDITOR,
        TRINITY_JACK_PLAY_CONTROLLER_ID,
        TextEditorScene {
            selection_json,
            tokens_json: serde_json::to_string(&semantic_tokens(query)).ok(),
            diagnostics_json: serde_json::to_string(&lint(&graph, query)).ok(),
            completions_json: serde_json::to_string(&complete(&graph, query, cursor)).ok(),
            occurrences_json: text_identifier_occurrences_json(query, cursor),
            ..TextEditorScene::base(query.clone(), Some("jack".into()), None)
        },
    )
}

fn render_results(envelope: &TrinityJackEnvelope) -> UiNode {
    let result: QueryResult = serde_json::from_str(&envelope.runtime.jack_result_json).unwrap_or(QueryResult::table(vec![], vec![]));
    if result.kind == QueryResultKind::Graph {
        if let Some(fixture) = &result.graph_fixture {
            let (nodes_json, edges_json, viewport_json) = fixture_to_media_graph(fixture);
            return build_node_graph_scene(
                TRINITY_JACK_PLAY_SURFACE_RESULTS,
                TRINITY_JACK_PLAY_CONTROLLER_ID,
                NodeGraphScene::base(nodes_json, edges_json, viewport_json),
            );
        }
    }
    let (columns_json, rows_json) = result_to_table(&envelope.runtime.jack_result_json);
    build_table_scene(
        TRINITY_JACK_PLAY_SURFACE_RESULTS,
        TRINITY_JACK_PLAY_CONTROLLER_ID,
        TableScene { columns_json, rows_json },
    )
}
//#endregion 🔖Render

//#region 🔖TrinityJackPlayApp
pub struct TrinityJackPlayApp;

impl PluginApp for TrinityJackPlayApp {
    fn app_id(&self) -> &str {
        TRINITY_JACK_PLAY_APP_ID
    }

    fn initial_document_json(&self) -> String {
        let mut envelope = default_envelope();
        let (result_json, fixture_json) = run_jack_on_fixture(&envelope.fixture_json, &envelope.runtime.jack_query);
        envelope.runtime.jack_result_json = result_json;
        envelope.fixture_json = fixture_json;
        serde_json::to_string(&envelope).expect("trinity jack envelope json")
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
                if let Some(next) = args.and_then(|value| value.get("document")) {
                    if let Ok(parsed) = serde_json::from_value(next.clone()) {
                        return vec![set_document_op(&parsed)];
                    }
                }
            }
            "setSelection" | "selectNode" | "nodeGraphSelect" => {
                envelope.runtime.selected_node_ids = selection_ids(args);
                return vec![set_document_op(&envelope)];
            }
            "nodeGraphHover" => return Vec::new(),
            "nodeGraphViewport" => {
                if let Some(viewport_json) = args.and_then(|value| value.get("viewportJson")).and_then(|value| value.as_str()) {
                    if let Ok(camera) = serde_json::from_str::<trinity_ram::Camera>(viewport_json) {
                        if let Some(mut fixture) = parse_fixture_json(&envelope.fixture_json) {
                            fixture.camera = camera;
                            if let Ok(json) = Graph::from_fixture(fixture).and_then(|graph| graph.fixture_json()) {
                                envelope.fixture_json = json;
                                return vec![set_document_op(&envelope)];
                            }
                        }
                    }
                }
            }
            "nodeGraphEdit" => {
                let ops = args
                    .and_then(|value| value.get("ops"))
                    .and_then(|value| value.as_array())
                    .cloned()
                    .unwrap_or_default();
                if apply_node_graph_edit_ops(&mut envelope, &ops) {
                    return vec![set_document_op(&envelope)];
                }
            }
            "textEdit" => {
                if let Some(text) = args.and_then(|v| v.get("text")).and_then(|v| v.as_str()) {
                    envelope.runtime.jack_query = text.into();
                    return vec![set_document_op(&envelope)];
                }
            }
            "textSelect" => {
                let start = args.and_then(|v| v.get("start")).and_then(|v| v.as_u64()).unwrap_or(0);
                let end = args.and_then(|v| v.get("end")).and_then(|v| v.as_u64()).unwrap_or(start);
                envelope.runtime.editor_selection = Some(TrinityEditorSelection { start: start as usize, end: end as usize });
                return vec![set_document_op(&envelope)];
            }
            "textHover" => return Vec::new(),
            "requestCompletions" => {
                envelope.runtime.revision += 1;
                return vec![set_document_op(&envelope)];
            }
            "formatDocument" => {
                if let Ok(formatted) = jack_format(&envelope.runtime.jack_query) {
                    envelope.runtime.jack_query = formatted;
                }
                return vec![set_document_op(&envelope)];
            }
            "setLodMode" => {
                if let (Some(window_id), Some(value)) = (
                    args.and_then(|v| v.get("windowId")).and_then(|v| v.as_str()),
                    args.and_then(|v| v.get("value")).and_then(|v| v.as_str()),
                ) {
                    envelope.runtime.lod_mode_by_window.insert(window_id.into(), value.into());
                    return vec![set_document_op(&envelope)];
                }
            }
            "loadExampleQuery" => {
                if let Some(query) = args.and_then(|v| v.get("query")).and_then(|v| v.as_str()) {
                    envelope.runtime.jack_query = query.into();
                    let (result_json, fixture_json) = run_jack_on_fixture(&envelope.fixture_json, query);
                    envelope.runtime.jack_result_json = result_json;
                    envelope.fixture_json = fixture_json;
                    return vec![set_document_op(&envelope)];
                }
            }
            "runJackQuery" | "submit" => {
                let query = args
                    .and_then(|v| v.get("query"))
                    .and_then(|v| v.as_str())
                    .filter(|value| !value.trim().is_empty())
                    .map(str::to_string)
                    .unwrap_or_else(|| envelope.runtime.jack_query.clone());
                envelope.runtime.jack_query = query.clone();
                if run_jack_with_vcs(&mut envelope, &query).is_err() {
                    let (result_json, fixture_json) = run_jack_on_fixture(&envelope.fixture_json, &query);
                    envelope.runtime.jack_result_json = result_json;
                    envelope.fixture_json = fixture_json;
                }
                envelope.runtime.results_engagement_input.clear();
                return vec![set_document_op(&envelope)];
            }
            "setActiveExample" => {
                let example_id = args.and_then(|v| v.get("exampleId")).and_then(|v| v.as_str()).unwrap_or("");
                if let Some(json) = fixture_json_for_preset(example_id) {
                    if parse_fixture_json(json).is_some() {
                        envelope.fixture_json = json.into();
                        envelope.runtime.active_fixture_id = example_id.into();
                        envelope.runtime.jack_query = preset_query(example_id).into();
                        let (result_json, fixture_json) =
                            run_jack_on_fixture(&envelope.fixture_json, &envelope.runtime.jack_query);
                        envelope.runtime.jack_result_json = result_json;
                        envelope.fixture_json = fixture_json;
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "patchTrinityNodes" => {
                let node_ids: Vec<String> = args
                    .and_then(|v| v.get("nodeIds"))
                    .and_then(|v| serde_json::from_value(v.clone()).ok())
                    .unwrap_or_default();
                let field = args.and_then(|v| v.get("field")).and_then(|v| v.as_str()).unwrap_or("");
                let value = args.and_then(|v| v.get("value")).and_then(|v| v.as_str()).map(str::trim).unwrap_or("");
                if field == "name" && !node_ids.is_empty() && !value.is_empty() {
                    let escaped = value.replace('\'', "\\'");
                    let fixture = parse_fixture_json(&envelope.fixture_json);
                    if let Some(fixture) = fixture {
                        let queries: Vec<String> = node_ids
                            .iter()
                            .filter_map(|id| {
                                fixture.nodes.iter().find(|node| &node.id == id).map(|node| {
                                    format!(
                                        "MATCH (n:{}) WHERE n.id = '{id}' SET n.name = '{escaped}'",
                                        node.kind
                                    )
                                })
                            })
                            .collect();
                        if !queries.is_empty() {
                            let query = queries.join("\n");
                            let (result_json, fixture_json) = run_jack_on_fixture(&envelope.fixture_json, &query);
                            envelope.runtime.jack_result_json = result_json;
                            envelope.fixture_json = fixture_json;
                            return vec![set_document_op(&envelope)];
                        }
                    }
                }
            }
            "reorganize" => {
                if let Some(next_json) = force_layout_fixture_json(&envelope.fixture_json) {
                    if let (Ok(before), Ok(after)) = (
                        GraphFixture::from_json(&envelope.fixture_json),
                        GraphFixture::from_json(&next_json),
                    ) {
                        let ops: Vec<trinity_ram::TrinityGraphOp> = after
                            .nodes
                            .iter()
                            .filter_map(|node| {
                                let prev = before.nodes.iter().find(|entry| entry.id == node.id)?;
                                if (prev.x - node.x).abs() > 1e-6 || (prev.y - node.y).abs() > 1e-6 {
                                    Some(trinity_ram::TrinityGraphOp::Reposition {
                                        id: node.id.clone(),
                                        x: node.x,
                                        y: node.y,
                                    })
                                } else {
                                    None
                                }
                            })
                            .collect();
                        if !ops.is_empty() {
                            let mut store = graph_store_from_envelope(&envelope);
                            if dispatch_trinity_graph_ops(&mut store, ops).is_ok() {
                                sync_envelope_from_store(&mut envelope, &store);
                            }
                        } else {
                            envelope.fixture_json = next_json;
                        }
                    } else {
                        envelope.fixture_json = next_json;
                    }
                }
                envelope.runtime.reorganize_epoch += 1;
                return vec![set_document_op(&envelope)];
            }
            "undo" => {
                let mut store = graph_store_from_envelope(&envelope);
                if store.dispatch(DocumentVcsCommand::Undo).is_ok() {
                    sync_envelope_from_store(&mut envelope, &store);
                    return vec![set_document_op(&envelope)];
                }
            }
            "redo" => {
                let mut store = graph_store_from_envelope(&envelope);
                if store.dispatch(DocumentVcsCommand::Redo).is_ok() {
                    sync_envelope_from_store(&mut envelope, &store);
                    return vec![set_document_op(&envelope)];
                }
            }
            "commitCheckpoint" => {
                let mut store = graph_store_from_envelope(&envelope);
                if store
                    .dispatch(DocumentVcsCommand::CommitCheckpoint {
                        message: args.and_then(|v| v.get("message")).and_then(|v| v.as_str()).map(str::to_string),
                        authors: Vec::new(),
                    })
                    .is_ok()
                {
                    sync_envelope_from_store(&mut envelope, &store);
                    return vec![set_document_op(&envelope)];
                }
            }
            "editorEngagementInput" => {
                if let Some(value) = args.and_then(|v| v.get("value")).and_then(|v| v.as_str()) {
                    envelope.runtime.editor_engagement_input = value.into();
                    return vec![set_document_op(&envelope)];
                }
            }
            "graphEngagementInput" => {
                if let Some(value) = args.and_then(|v| v.get("value")).and_then(|v| v.as_str()) {
                    envelope.runtime.graph_engagement_input = value.into();
                    return vec![set_document_op(&envelope)];
                }
            }
            "resultsEngagementInput" => {
                if let Some(value) = args.and_then(|v| v.get("value")).and_then(|v| v.as_str()) {
                    envelope.runtime.results_engagement_input = value.into();
                    return vec![set_document_op(&envelope)];
                }
            }
            "graphPointerDown" => {
                if let Some(node_id) = args.and_then(|v| v.get("nodeId")).and_then(|v| v.as_str()) {
                    envelope.runtime.selected_node_ids = vec![node_id.into()];
                    return vec![set_document_op(&envelope)];
                }
            }
            _ => {}
        }
        Vec::new()
    }

    fn render(&self, body_key: &str, document_json: &str, view_state: &ViewState) -> UiNode {
        let envelope = parse_envelope(document_json);
        let labels = trinity_jack_labels(view_state);
        match body_key {
            TRINITY_JACK_PLAY_BODY_GRAPH => render_graph(&envelope),
            TRINITY_JACK_PLAY_BODY_EDITOR => render_editor(&envelope),
            TRINITY_JACK_PLAY_BODY_RESULTS => render_results(&envelope),
            TRINITY_JACK_PLAY_BODY_DOCUMENT => build_document_tree(&envelope, labels),
            TRINITY_JACK_PLAY_BODY_CATALOGUE => build_catalogue_tree(&envelope, labels),
            TRINITY_JACK_PLAY_BODY_INSPECTION => build_inspector_tree(&envelope, labels),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn tools(&self, _document_json: &str, view_state: &ViewState) -> Vec<ToolNode> {
        let labels = trinity_jack_labels(view_state);
        vec![
            tool_collection(
                "trinity-jack-history",
                "clock",
                labels.history,
                vec![
                    tool_button("trinity-jack-undo", "undo-2", "Undo", jack_action("undo", None)),
                    tool_button("trinity-jack-redo", "redo-2", "Redo", jack_action("redo", None)),
                    tool_button("trinity-jack-checkpoint", "git-commit", "Checkpoint", jack_action("commitCheckpoint", None)),
                ],
            )
            .with_category(ToolCategory::History),
            tool_collection(
                "trinity-jack-query",
                "code",
                labels.query,
                vec![
                    tool_button("trinity-jack-run", "play", "Run", jack_action("runJackQuery", None)),
                    tool_button("trinity-jack-reorganize", "rotate-cw", "Reorganize", jack_action("reorganize", None)),
                ],
            )
            .with_category(ToolCategory::Actions),
        ]
    }

    fn window_measures(&self, document_json: &str, _view_state: &ViewState) -> std::collections::HashMap<String, Vec<WindowMeasure>> {
        let envelope = parse_envelope(document_json);
        let mode = envelope
            .runtime
            .lod_mode_by_window
            .get(TRINITY_JACK_PLAY_WINDOW_GRAPH)
            .map(String::as_str)
            .unwrap_or(TRINITY_LOD_MODE_AUTOMATIC);
        std::collections::HashMap::from([(
            TRINITY_JACK_PLAY_WINDOW_GRAPH.to_string(),
            vec![trinity_lod_measure(TRINITY_JACK_PLAY_WINDOW_GRAPH, mode)],
        )])
    }
}
//#endregion 🔖TrinityJackPlayApp

//#region 🔖Manifest
fn jack_window_stack(id: &str, title: &str, size: Option<f64>) -> WindowLayoutChild {
    WindowLayoutChild::Stack(WindowLayoutStackNode {
        kind: "stack".into(),
        size,
        active_window_kind_id: None,
        children: vec![WindowLayoutWindowNode {
            kind: "window".into(),
            window_kind_id: id.into(),
            title: Some(title.into()),
            instance_id: None,
            template_id: None,
        }],
    })
}

fn jack_layout() -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Axis(WindowLayoutAxisNode {
            kind: "row".into(),
            size: None,
            children: vec![
                WindowLayoutChild::Stack(WindowLayoutStackNode {
                    kind: "stack".into(),
                    size: Some(0.6),
                    active_window_kind_id: None,
                    children: vec![WindowLayoutWindowNode {
                        kind: "window".into(),
                        window_kind_id: TRINITY_JACK_PLAY_WINDOW_GRAPH.into(),
                        title: Some("Nakagin Graph".into()),
                        instance_id: None,
                        template_id: None,
                    }],
                }),
                WindowLayoutChild::Axis(WindowLayoutAxisNode {
                    kind: "column".into(),
                    size: Some(0.4),
                    children: vec![
                        jack_window_stack(TRINITY_JACK_PLAY_WINDOW_EDITOR, "Jack Query", Some(0.55)),
                        jack_window_stack(TRINITY_JACK_PLAY_WINDOW_RESULTS, "Results", Some(0.45)),
                    ],
                }),
            ],
        }),
    }
}

pub fn create_trinity_jack_app() -> App {
    App::from_builder(
        App::builder(TRINITY_JACK_PLAY_APP_ID, "Trinity Jack").document(["semio", "trinity", "jack"])
            .icon_id("trinity")
            .mode("explore", "Explore")
            .default_mode_id("explore")
            .window_kind(TRINITY_JACK_PLAY_WINDOW_GRAPH, "Nakagin Graph", TRINITY_JACK_PLAY_BODY_GRAPH, SurfaceKind::NodeGraph)
            .window_kind(TRINITY_JACK_PLAY_WINDOW_EDITOR, "Jack Query", TRINITY_JACK_PLAY_BODY_EDITOR, SurfaceKind::TextEditor)
            .window_kind(TRINITY_JACK_PLAY_WINDOW_RESULTS, "Results", TRINITY_JACK_PLAY_BODY_RESULTS, SurfaceKind::Table)
            .default_layout(jack_layout())
            .panel_tab(
                FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
                FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
                PanelGroup::Workbench,
                TRINITY_JACK_PLAY_BODY_DOCUMENT,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                PanelGroup::Workbench,
                TRINITY_JACK_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                PanelGroup::Details,
                TRINITY_JACK_PLAY_BODY_INSPECTION,
            )
            .operation("nodeGraphEdit", "Edit Graph")
            .operation("patchTrinityNodes", "Patch Nodes")
            .operation("reorganize", "Reorganize")
            .operation("runJackQuery", "Run Jack Query")
            .view_action("setSelection", "Set Selection")
            .view_action("selectNode", "Select Node")
            .view_action("nodeGraphSelect", "Select Graph Node")
            .view_action("nodeGraphHover", "Hover Graph Node")
            .view_action("nodeGraphViewport", "Set Graph Viewport")
            .view_action("textEdit", "Edit Jack Query")
            .view_action("textSelect", "Select Jack Query Text")
            .view_action("textHover", "Hover Jack Query Text")
            .view_action("requestCompletions", "Request Completions")
            .view_action("formatDocument", "Format Jack Query")
            .view_action("submit", "Submit Jack Query")
            .view_action("setLodMode", "Set LOD Mode")
            .view_action("loadExampleQuery", "Load Example Query")
            .view_action("editorEngagementInput", "Editor Engagement Input")
            .view_action("graphEngagementInput", "Graph Engagement Input")
            .view_action("resultsEngagementInput", "Results Engagement Input")
            .view_action("graphPointerDown", "Graph Pointer Down")
            .shell_action("setDocument", "Set Document")
            .shell_action("setActiveExample", "Set Active Example")
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .keybinding("mod+alt+s", "commitCheckpoint"),
    )
    .example("nakagin", "Nakagin", serde_json::to_string(&default_envelope()).unwrap())
    .program("trinity", "Trinity", "graph")
}

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_node_graph_scene() {
        let app = TrinityJackPlayApp;
        let document = app.initial_document_json();
        let node = app.render(TRINITY_JACK_PLAY_BODY_GRAPH, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("node-graph"));
    }

    #[test]
    fn renders_jack_editor() {
        let app = TrinityJackPlayApp;
        let document = app.initial_document_json();
        let node = app.render(TRINITY_JACK_PLAY_BODY_EDITOR, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("text-editor"));
        assert!(json.contains(TRINITY_JACK_DEFAULT_QUERY));
    }

    #[test]
    fn run_query_updates_fixture() {
        let mut app = TrinityJackPlayApp;
        let document = app.initial_document_json();
        let mut next = document;
        for op in app.handle_action_patch_ops("runJackQuery", None, &next, &ViewState::default()) {
            if let Ok(value) = serde_json::from_str::<Value>(&op) {
                if let Some(doc) = value.get("document") {
                    next = doc.to_string();
                }
            }
        }
        let envelope = parse_envelope(&next);
        assert!(parse_fixture_json(&envelope.fixture_json).is_some());
        assert!(!envelope.runtime.jack_result_json.is_empty());
    }

    #[test]
    fn node_graph_select_updates_selection() {
        let mut app = TrinityJackPlayApp;
        let document = app.initial_document_json();
        let fixture = parse_fixture_json(NAKAGIN_FIXTURE_JSON).expect("fixture");
        let node_id = fixture.nodes.first().expect("node").id.clone();
        let ops = app.handle_action_patch_ops(
            "nodeGraphSelect",
            Some(&json!({ "nodeIds": [node_id.clone()] })),
            &document,
            &ViewState::default(),
        );
        assert!(!ops.is_empty());
        let next = ops
            .first()
            .and_then(|op| serde_json::from_str::<Value>(op).ok())
            .and_then(|value| value.get("document").cloned())
            .expect("document op");
        let envelope = serde_json::from_value::<TrinityJackEnvelope>(next).expect("envelope");
        assert_eq!(envelope.runtime.selected_node_ids, vec![node_id]);
    }

    #[test]
    fn nakagin_fixture_has_nodes() {
        let fixture = parse_fixture_json(NAKAGIN_FIXTURE_JSON).expect("nakagin fixture");
        assert!(!fixture.nodes.is_empty());
    }

    #[test]
    fn editor_scene_has_tokens_and_diagnostics() {
        let app = TrinityJackPlayApp;
        let document = app.initial_document_json();
        let node = app.render(TRINITY_JACK_PLAY_BODY_EDITOR, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("tokensJson"));
        assert!(json.contains("diagnosticsJson"));
        assert!(json.contains("completionsJson"));
    }

    #[test]
    fn text_edit_updates_query() {
        let mut app = TrinityJackPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_action_patch_ops(
            "textEdit",
            Some(&json!({ "text": "MATCH (a:Piece) RETURN a.name" })),
            &document,
            &ViewState::default(),
        );
        let next = ops.first().and_then(|op| serde_json::from_str::<Value>(op).ok()).and_then(|value| value.get("document").cloned()).expect("document op");
        let envelope = serde_json::from_value::<TrinityJackEnvelope>(next).expect("envelope");
        assert_eq!(envelope.runtime.jack_query, "MATCH (a:Piece) RETURN a.name");
    }

    #[test]
    fn graph_scene_has_lod_json() {
        let app = TrinityJackPlayApp;
        let document = app.initial_document_json();
        let node = app.render(TRINITY_JACK_PLAY_BODY_GRAPH, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("lodJson"));
        assert!(json.contains("automatic"));
    }

    #[test]
    fn set_lod_mode_persists_per_window() {
        let mut app = TrinityJackPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_action_patch_ops(
            "setLodMode",
            Some(&json!({ "windowId": TRINITY_JACK_PLAY_WINDOW_GRAPH, "value": "minimap" })),
            &document,
            &ViewState::default(),
        );
        let next = ops.first().and_then(|op| serde_json::from_str::<Value>(op).ok()).and_then(|value| value.get("document").cloned()).expect("document op");
        let envelope = serde_json::from_value::<TrinityJackEnvelope>(next).expect("envelope");
        assert_eq!(envelope.runtime.lod_mode_by_window.get(TRINITY_JACK_PLAY_WINDOW_GRAPH).map(String::as_str), Some("minimap"));
    }

    #[test]
    fn return_graph_example_renders_node_graph_in_results() {
        let mut app = TrinityJackPlayApp;
        let document = app.initial_document_json();
        let ops = app.handle_action_patch_ops(
            "loadExampleQuery",
            Some(&json!({ "query": "MATCH (a:Piece)-[r:Connection]->(b:Piece) WHERE a.name = 'b' RETURN a, r, b" })),
            &document,
            &ViewState::default(),
        );
        let next = ops.first().cloned().and_then(|op| serde_json::from_str::<Value>(&op).ok()).and_then(|value| value.get("document").cloned()).expect("document op");
        let next_json = next.to_string();
        let node = app.render(TRINITY_JACK_PLAY_BODY_RESULTS, &next_json, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("node-graph"));
    }

    #[test]
    fn catalogue_has_eight_example_queries() {
        let app = TrinityJackPlayApp;
        let document = app.initial_document_json();
        let node = app.render(TRINITY_JACK_PLAY_BODY_CATALOGUE, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        for id in ["where-or", "return-graph", "set-label", "set-position", "create-node", "create-edge", "delete-leaf", "merge-edge"] {
            assert!(json.contains(id), "missing example query {id}");
        }
    }

    #[test]
    fn inspector_has_flat_position_fields() {
        let mut app = TrinityJackPlayApp;
        let document = app.initial_document_json();
        let fixture = parse_fixture_json(NAKAGIN_FIXTURE_JSON).expect("fixture");
        let node_id = fixture.nodes.first().expect("node").id.clone();
        let ops = app.handle_action_patch_ops("nodeGraphSelect", Some(&json!({ "nodeIds": [node_id] })), &document, &ViewState::default());
        let next = ops.first().cloned().and_then(|op| serde_json::from_str::<Value>(&op).ok()).and_then(|value| value.get("document").cloned()).expect("document op").to_string();
        let node = app.render(TRINITY_JACK_PLAY_BODY_INSPECTION, &next, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Flat U"));
        assert!(json.contains("Flat V"));
    }

    #[test]
    fn tools_include_run_jack_query() {
        let app = TrinityJackPlayApp;
        let document = app.initial_document_json();
        let tools = app.tools(&document, &ViewState::default());
        let json = serde_json::to_string(&tools).unwrap();
        assert!(json.contains("runJackQuery"));
        assert!(json.contains("undo"));
    }

    #[test]
    fn trinity_jack_labels_resolve_native_by_default() {
        let app = TrinityJackPlayApp;
        let document = app.initial_document_json();
        let node = app.render(TRINITY_JACK_PLAY_BODY_DOCUMENT, &document, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("\"Pieces\""));
        assert!(json.contains("\"Connections\""));
        assert!(!json.contains("Stücke"));
    }

    #[test]
    fn trinity_jack_labels_translate_panels_in_german() {
        let app = TrinityJackPlayApp;
        let document = app.initial_document_json();
        let view_state = ViewState { locale: Some("de".into()), ..ViewState::default() };
        let document_tree = app.render(TRINITY_JACK_PLAY_BODY_DOCUMENT, &document, &view_state);
        let document_json = serde_json::to_string(&document_tree).unwrap();
        assert!(document_json.contains("Stücke"));
        assert!(document_json.contains("Verbindungen"));
        assert!(!document_json.contains("\"Pieces\""));
        let catalogue_tree = app.render(TRINITY_JACK_PLAY_BODY_CATALOGUE, &document, &view_state);
        let catalogue_json = serde_json::to_string(&catalogue_tree).unwrap();
        assert!(catalogue_json.contains("Fixturen"));
        assert!(catalogue_json.contains("Beispielabfragen"));
        assert!(catalogue_json.contains("Manifestarten"));
        let tools = app.tools(&document, &view_state);
        let tools_json = serde_json::to_string(&tools).unwrap();
        assert!(tools_json.contains("Verlauf"));
        assert!(tools_json.contains("Abfrage"));
    }

    #[test]
    fn undo_restores_fixture_across_separate_dispatches() {
        let mut app = TrinityJackPlayApp;
        let document = app.initial_document_json();
        let query = "MATCH (a:Piece) WHERE a.name = 'b' SET a.label = 'undo-test-label'";
        let run_ops = app.handle_action_patch_ops("runJackQuery", Some(&json!({ "query": query })), &document, &ViewState::default());
        let ran_json = run_ops
            .first()
            .and_then(|op| serde_json::from_str::<Value>(op).ok())
            .and_then(|value| value.get("document").cloned())
            .expect("document op")
            .to_string();
        let ran_envelope = parse_envelope(&ran_json);
        assert!(ran_envelope.fixture_json.contains("undo-test-label"), "SET should have applied the label");
        let undo_ops = app.handle_action_patch_ops("undo", None, &ran_json, &ViewState::default());
        assert!(!undo_ops.is_empty(), "undo should succeed in a fresh dispatch after a prior edit");
        let undone_envelope = parse_envelope(
            &undo_ops
                .first()
                .and_then(|op| serde_json::from_str::<Value>(op).ok())
                .and_then(|value| value.get("document").cloned())
                .expect("document op")
                .to_string(),
        );
        assert!(!undone_envelope.fixture_json.contains("undo-test-label"), "undo should revert the label");
    }
}
//#endregion 🧪Tests
