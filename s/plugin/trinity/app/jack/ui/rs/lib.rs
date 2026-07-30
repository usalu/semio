//! 🔱 Trinity Jack plugin — jack query play app bundled as a hot-swappable WASM plugin.

use semio_framework_plugin::{SurfaceKind, PanelGroup,
    app_labels, build_node_graph_scene, build_table_scene, build_text_editor_scene,
    is_de_locale, localized_label_map, resolve_labels, text_identifier_occurrences_json, tree_item, tree_item_with_action,
    ui_declarative_sections_to_tree, ui_inspector_groups_to_tree, ui_inspector_mixed_text,
    ui_inspector_readonly_field, ui_text, ActionArgDef, ActionArgOption, ActionDefinition, ActionEmit, ActionKind, App, ActionDescriptor, AppLabelsOverlay, AppLabelsOverlayExt, DocumentApp,
    DocumentView, MeasureSelectItem, NodeGraphScene, MediaClass, MediaForm, MediaType, OsMediaCapability, PanelTreeBuilder, ArtifactKindSpec,
    TableScene, TextEditorScene, UiFieldNode, UiInspectorFieldGroup, UiNode, UiPresence, UiSectionNode, UiTreeItemNode,
    ViewState, WindowLayout, WindowLayoutAxisNode, WindowLayoutChild,
    WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode, WindowMeasure, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
    FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, UI_INSPECTOR_MIXED_PLACEHOLDER,
};
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashMap};
use trinity_jack::{complete, execute, format as jack_format, lint, parse, semantic_tokens, QueryResult, QueryResultKind};
use trinity_ram::{Camera, Graph, GraphFixture, Node, PortDirection, PropertyValue, TrinityGraphOperation, TRINITY_GRAPH_SCHEMA};
use store::DocumentDsl;

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

const NAKAGIN_FIXTURE_DSL: &str = include_str!("../../../../example/nakagin-capsule-tower.trinity");
const BRANCH_FIXTURE_DSL: &str = include_str!("../../../../example/branch-chain.trinity");

const TRINITY_JACK_DEFAULT_QUERY: &str =
    "MATCH (a:Piece)-[r:Connection]->(b:Piece) WHERE a.name = 'b' AND b.name != 'b' RETURN a.name, b.name, b.label";

const TRINITY_LOD_MODE_AUTOMATIC: &str = "automatic";
//#endregion 🔖Constants

//#region 🔖Types
/// 🎯 Ephemeral editor selection range (offsets into the jack query text) — pure runtime.
#[derive(Clone, Debug, Default, PartialEq)]
struct TrinityEditorSelection {
    start: usize,
    end: usize,
}

/// 🎛️ Ephemeral view state (selection, query draft, results, LOD, engagement inputs) — lives on the
/// app struct, never in the document, so it never pollutes undo history. The document projection is
/// the bare {@link GraphFixture}.
#[derive(Clone, Debug, Default, PartialEq)]
struct TrinityJackRuntime {
    selected_node_ids: Vec<String>,
    active_fixture_id: String,
    jack_query: String,
    jack_result_json: String,
    editor_engagement_input: String,
    graph_engagement_input: String,
    results_engagement_input: String,
    reorganize_epoch: u64,
    editor_selection: Option<TrinityEditorSelection>,
    lod_mode_by_window: BTreeMap<String, String>,
    revision: u64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct WorkflowDiagramPortRecord {
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct WorkflowNodeRecord {
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    inputs: Vec<WorkflowDiagramPortRecord>,
    outputs: Vec<WorkflowDiagramPortRecord>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct WorkflowEdgeRecord {
    id: String,
    source_node_id: String,
    source_port_id: String,
    target_node_id: String,
    target_port_id: String,
}
//#endregion 🔖Types

//#region 🔖DocumentHelpers
/// 📦 The default trinity graph fixture (Nakagin capsule tower) — the initial document projection.
fn default_fixture() -> GraphFixture {
    GraphFixture::parse_dsl(NAKAGIN_FIXTURE_DSL).unwrap_or_else(|_| trinity_ram::empty_trinity_graph_fixture())
}

/// 🌱 Seeds the runtime with the default query and its result table so the Results window is populated on load.
fn seeded_jack_runtime() -> TrinityJackRuntime {
    let (result_json, _) = run_jack_query(&default_fixture(), TRINITY_JACK_DEFAULT_QUERY);
    TrinityJackRuntime {
        active_fixture_id: "nakagin".into(),
        jack_query: TRINITY_JACK_DEFAULT_QUERY.into(),
        jack_result_json: result_json,
        ..Default::default()
    }
}

/// 🔎 Runs a jack query against the fixture, returning `(result_json, forward operations)`; a parse/execute
/// failure yields an error result and no operations (no document mutation).
fn run_jack_query(fixture: &GraphFixture, query: &str) -> (String, Vec<TrinityGraphOperation>) {
    let graph = match Graph::from_fixture(fixture.clone()) {
        Ok(graph) => graph,
        Err(error) => return (error_result_json(&error.to_string()), Vec::new()),
    };
    let parsed = match parse(query) {
        Ok(parsed) => parsed,
        Err(error) => return (error_result_json(&error), Vec::new()),
    };
    match execute(&graph, &parsed) {
        Ok((result, operations)) => (serde_json::to_string(&result).unwrap_or_default(), operations),
        Err(error) => (error_result_json(&error), Vec::new()),
    }
}

fn error_result_json(message: &str) -> String {
    json!({ "error": message }).to_string()
}

fn jack_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor {
        controller_id: TRINITY_JACK_PLAY_CONTROLLER_ID.into(),
        action: action.into(),
        args,
    }
}

fn graph_from_fixture_or_default(fixture: &GraphFixture) -> Graph {
    Graph::from_fixture(fixture.clone()).unwrap_or_else(|_| Graph::from_fixture(default_fixture()).expect("nakagin graph"))
}

/// 🧮 Clones the fixture and recomputes its derived (flat-position) node properties for the inspector.
fn fixture_with_derived(fixture: &GraphFixture) -> Option<GraphFixture> {
    let mut graph = Graph::from_fixture(fixture.clone()).ok()?;
    graph.recompute_derived();
    Some(graph.to_fixture())
}

fn fixture_dsl_for_preset(preset_id: &str) -> Option<&'static str> {
    match preset_id {
        "nakagin" | "nakagin-capsule-tower" => Some(NAKAGIN_FIXTURE_DSL),
        "branch-chain" => Some(BRANCH_FIXTURE_DSL),
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

/// 🧲 Re-runs force layout on the fixture, returning the repositioned fixture (or `None` if empty).
fn force_layout_fixture(fixture: &GraphFixture) -> Option<GraphFixture> {
    let mut fixture = fixture.clone();
    if fixture.nodes.is_empty() {
        return None;
    }
    use mathematical_graph_drawing::force::{run_force_layout, ForceLayoutOptions};
    use mathematical_geometry::Vec2;
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
    Some(fixture)
}

/// 🧭 Emits a `Reposition` operation for every node whose position differs between `before` and `after`.
fn reposition_operations(before: &GraphFixture, after: &GraphFixture) -> Vec<TrinityGraphOperation> {
    after
        .nodes
        .iter()
        .filter_map(|node| {
            let prev = before.nodes.iter().find(|entry| entry.id == node.id)?;
            if (prev.x - node.x).abs() > 1e-6 || (prev.y - node.y).abs() > 1e-6 {
                Some(TrinityGraphOperation::Reposition { id: node.id.clone(), x: node.x, y: node.y })
            } else {
                None
            }
        })
        .collect()
}

/// 🎯 Selection ids from an action's args — delegates to the SDK's shared `ids`-array reader, falling
/// back to a singular `nodeIds`/`nodeId` key for actions dispatched from the node-graph scene surface.
fn selection_ids(args: Option<&Value>) -> Vec<String> {
    args.and_then(|value| value.get("nodeIds"))
        .and_then(|value| serde_json::from_value(value.clone()).ok())
        .or_else(|| Some(semio_framework_plugin::selection_ids(args)).filter(|ids: &Vec<String>| !ids.is_empty()))
        .or_else(|| {
            args.and_then(|value| value.get("nodeId"))
                .and_then(|value| value.as_str())
                .map(|id| vec![id.to_string()])
        })
        .unwrap_or_default()
}

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
/// 🩹 Delegates to `trinity_ram::parse_port_key` (the one place the `nodeId@portId` convention is
/// owned) instead of hand-rolling a second splitter here.
fn split_endpoint(endpoint: &str) -> (String, String) {
    trinity_ram::parse_port_key(endpoint).map_or_else(|| (endpoint.to_string(), "in".into()), |(n, p)| (n.to_string(), p.to_string()))
}

fn fixture_to_workflow(fixture: &GraphFixture) -> (String, String, String) {
    let nodes: Vec<WorkflowNodeRecord> = fixture
        .nodes
        .iter()
        .map(|node| node_to_workflow_record(node))
        .collect();
    let edges: Vec<WorkflowEdgeRecord> = fixture
        .edges
        .iter()
        .map(|edge| {
            let (source_node_id, source_port_id) = split_endpoint(&edge.source);
            let (target_node_id, target_port_id) = split_endpoint(&edge.target);
            WorkflowEdgeRecord {
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

fn node_to_workflow_record(node: &Node) -> WorkflowNodeRecord {
    let width = if node.width > 0.0 { node.width } else { 96.0 };
    let height = if node.height > 0.0 { node.height } else { 48.0 };
    WorkflowNodeRecord {
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
            .map(|port| WorkflowDiagramPortRecord {
                id: trinity_ram::port_key(&node.id, &port.id),
                label: Some(port.id.clone()),
            })
            .collect(),
        outputs: node
            .ports
            .iter()
            .filter(|port| port.direction == PortDirection::Out)
            .map(|port| WorkflowDiagramPortRecord {
                id: trinity_ram::port_key(&node.id, &port.id),
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
//#endregion 🔖DocumentHelpers

//#region 🔖Terminology
/// 🗣️ Complete UI label set for the Jack query app; one field per label makes every locale combination compile-checked.
app_labels! {
    struct TrinityJackLabels {
        pieces: &'static str = en: "Pieces", de: "Stücke";
        connections: &'static str = en: "Connections", de: "Verbindungen";
        fixtures: &'static str = en: "Fixtures", de: "Fixturen";
        example_queries: &'static str = en: "Example queries", de: "Beispielabfragen";
        manifest_kinds: &'static str = en: "Manifest kinds", de: "Manifestarten";
        piece: &'static str = en: "Piece", de: "Stück";
        connection: &'static str = en: "Connection", de: "Verbindung";
        connector: &'static str = en: "Connector", de: "Verbinder";
        geometry: &'static str = en: "Geometry", de: "Geometrie";
        identity: &'static str = en: "Identity", de: "Identität";
        history: &'static str = en: "History", de: "Verlauf";
        query: &'static str = en: "Query", de: "Abfrage";
        window_graph: &'static str = en: "Nakagin Graph", de: "Nakagin-Graph";
        window_editor: &'static str = en: "Jack Query", de: "Jack-Abfrage";
        window_results: &'static str = en: "Results", de: "Ergebnisse";
    }
}
//#endregion 🔖Terminology

//#region 🔖CommandLabels
/// 🗣️ (action id) -> localized label for every operation/view-action declared in `create_trinity_jack_app`'s static
/// manifest — the manifest itself has no `view_state`/locale parameter, so this overlay is how the command palette
/// and Actions rail get a translated label without threading locale through the whole builder chain.
fn trinity_jack_action_labels(is_de: bool) -> HashMap<String, String> {
    localized_label_map(is_de, &[
        ("nodeGraphEdit", "Edit Graph", "Graph bearbeiten"),
        ("nodeGraphViewport", "Set Graph Viewport", "Graph-Ansicht festlegen"),
        ("patchTrinityNodes", "Patch Nodes", "Knoten aktualisieren"),
        ("reorganize", "Reorganize", "Neu anordnen"),
        ("runJackQuery", "Run Jack Query", "Jack-Abfrage ausführen"),
        ("submit", "Submit Jack Query", "Jack-Abfrage absenden"),
        ("loadExampleQuery", "Load Example Query", "Beispielabfrage laden"),
        ("setActiveExample", "Set Active Example", "Aktives Beispiel festlegen"),
        ("setSelection", "Set Selection", "Auswahl festlegen"),
        ("selectNode", "Select Node", "Knoten auswählen"),
        ("nodeGraphSelect", "Select Graph Node", "Graph-Knoten auswählen"),
        ("nodeGraphHover", "Hover Graph Node", "Graph-Knoten hovern"),
        ("textEdit", "Edit Jack Query", "Jack-Abfrage bearbeiten"),
        ("textSelect", "Select Jack Query Text", "Jack-Abfragetext auswählen"),
        ("textHover", "Hover Jack Query Text", "Jack-Abfragetext hovern"),
        ("requestCompletions", "Request Completions", "Vervollständigungen anfordern"),
        ("formatDocument", "Format Jack Query", "Jack-Abfrage formatieren"),
        ("setLodMode", "Set LOD Mode", "LOD-Modus festlegen"),
        ("editorEngagementInput", "Editor Engagement Input", "Editor-Eingabe"),
        ("graphEngagementInput", "Graph Engagement Input", "Graph-Eingabe"),
        ("resultsEngagementInput", "Results Engagement Input", "Ergebnis-Eingabe"),
        ("graphPointerDown", "Graph Pointer Down", "Graph-Zeiger gedrückt"),
    ])
}
//#endregion 🔖CommandLabels

//#region 🔖Panels
fn flat_position_uv(node: &Node) -> (String, String) {
    let Some(flat) = node.properties.get("flatPosition").and_then(PropertyValue::as_object) else {
        return (String::new(), String::new());
    };
    let format_axis = |axis: &str| flat.get(axis).and_then(PropertyValue::as_f64).map(|value| format!("{value:.2}")).unwrap_or_default();
    (format_axis("u"), format_axis("v"))
}

fn build_document_tree(fixture: &GraphFixture, runtime: &TrinityJackRuntime, labels: &TrinityJackLabels) -> UiNode {
    let builder = PanelTreeBuilder::new("trinity-document");
    let node_items: Vec<UiTreeItemNode> = fixture
        .nodes
        .iter()
        .map(|node| {
            tree_item_with_action(
                builder.item_id("node", &node.id),
                if node.name.is_empty() { node.id.clone() } else { node.name.clone() },
                Some(node.kind.clone()),
                jack_action("setSelection", Some(json!({ "ids": [node.id] }))),
            )
        })
        .collect();
    let edge_items: Vec<UiTreeItemNode> = fixture
        .edges
        .iter()
        .map(|edge| tree_item(builder.item_id("edge", &edge.id), format!("{} → {}", edge.source, edge.target)))
        .collect();
    let selected = runtime.selected_node_ids.iter().map(|id| builder.item_id("node", id)).collect();
    builder
        .section("trinity-document.nodes", Some(labels.pieces.into()), true, node_items)
        .section("trinity-document.edges", Some(labels.connections.into()), false, edge_items)
        .selected(selected)
        .selection_change(jack_action("setSelection", Some(json!({ "ids": [] }))))
        .build()
}

fn build_catalogue_tree(runtime: &TrinityJackRuntime, labels: &TrinityJackLabels) -> UiNode {
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
    let builder = PanelTreeBuilder::new("trinity-jack-catalogue");
    let fixture_items: Vec<UiTreeItemNode> = fixtures
        .iter()
        .map(|(id, label)| {
            tree_item_with_action(
                builder.item_id("fixture", id),
                *label,
                Some(preset_query(id).into()),
                jack_action("setActiveExample", Some(json!({ "exampleId": id }))),
            )
        })
        .collect();
    let example_items: Vec<UiTreeItemNode> = examples
        .iter()
        .map(|(id, label, query)| {
            tree_item_with_action(
                builder.item_id("example", id),
                *label,
                Some((*query).into()),
                jack_action("loadExampleQuery", Some(json!({ "query": query }))),
            )
        })
        .collect();
    let selected = if runtime.active_fixture_id.is_empty() { vec![] } else { vec![builder.item_id("fixture", &runtime.active_fixture_id)] };
    builder
        .section("trinity-jack-catalogue.fixtures", Some(labels.fixtures.into()), true, fixture_items)
        .section("trinity-jack-catalogue.examples", Some(labels.example_queries.into()), true, example_items)
        .section(
            "trinity-jack-catalogue.kinds",
            Some(labels.manifest_kinds.into()),
            false,
            vec![
                tree_item("trinity-jack-catalogue.piece", labels.piece),
                tree_item("trinity-jack-catalogue.connection", labels.connection),
                tree_item("trinity-jack-catalogue.connector", labels.connector),
            ],
        )
        .selected(selected)
        .build()
}

fn build_inspector_tree(fixture: &GraphFixture, runtime: &TrinityJackRuntime, term_labels: &TrinityJackLabels) -> UiNode {
    if runtime.selected_node_ids.is_empty() {
        return ui_declarative_sections_to_tree(&[UiSectionNode {
            id: "trinity-inspector.empty".into(),
            label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
            default_open: Some(true),
            presence: UiPresence::default(),
            children: vec![ui_text("Select one or more pieces")],
        }]);
    }
    let nodes: Vec<&Node> = runtime
        .selected_node_ids
        .iter()
        .filter_map(|id| fixture.nodes.iter().find(|node| &node.id == id))
        .collect();
    if nodes.is_empty() {
        return ui_declarative_sections_to_tree(&[UiSectionNode {
            id: "trinity-inspector.missing".into(),
            label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
            default_open: Some(true),
            presence: UiPresence::default(),
            children: vec![ui_text("Piece not found")],
        }]);
    }
    let node_ids: Vec<String> = nodes.iter().map(|node| node.id.clone()).collect();
    let name_mixed = ui_inspector_mixed_text(&nodes.iter().map(|node| node.name.clone()).collect::<Vec<_>>());
    let kind_mixed = ui_inspector_mixed_text(&nodes.iter().map(|node| node.kind.clone()).collect::<Vec<_>>());
    let port_counts: Vec<String> = nodes.iter().map(|node| node.ports.len().to_string()).collect();
    let ports_mixed = ui_inspector_mixed_text(&port_counts);
    let derived_fixture = fixture_with_derived(fixture);
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
        UiInspectorFieldGroup { presence: UiPresence::default(),
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
            presence: UiPresence::default(),
            id: "trinity-inspector.identity".into(),
            label: term_labels.identity.into(),
            default_open: None,
            fields: vec![
                semio_framework_plugin::UiNode::Field(UiFieldNode {presence: UiPresence::default(), 
                    id: "trinity-inspector.name".into(),
                    label: "Name".into(),
                    child: Box::new(semio_framework_plugin::UiNode::Input(semio_framework_plugin::UiInputNode {presence: UiPresence::default(), 
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
fn render_graph(fixture: &GraphFixture, runtime: &TrinityJackRuntime) -> UiNode {
    let (nodes_json, edges_json, viewport_json) = fixture_to_workflow(fixture);
    let selection_json = if runtime.selected_node_ids.is_empty() {
        None
    } else {
        serde_json::to_string(&runtime.selected_node_ids).ok()
    };
    build_node_graph_scene(
        TRINITY_JACK_PLAY_SURFACE_GRAPH,
        TRINITY_JACK_PLAY_CONTROLLER_ID,
        NodeGraphScene {
            selection_json,
            context_menu_json: Some(
                r#"[{"id":"delete-selection","label":"Delete selection","icon":"trash","action":"nodeGraphEdit","args":{"operations":[{"operation":"deleteSelection"}]},"destructive":true}]"#.into(),
            ),
            lod_json: trinity_lod_json_for_window(runtime, TRINITY_JACK_PLAY_WINDOW_GRAPH),
            ..NodeGraphScene::base(nodes_json, edges_json, viewport_json)
        },
    )
}

fn render_editor(fixture: &GraphFixture, runtime: &TrinityJackRuntime) -> UiNode {
    let query = &runtime.jack_query;
    let graph = graph_from_fixture_or_default(fixture);
    let cursor = runtime.editor_selection.as_ref().map(|selection| selection.end).unwrap_or(0);
    let selection_json = runtime
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

fn render_results(runtime: &TrinityJackRuntime) -> UiNode {
    let result: QueryResult = serde_json::from_str(&runtime.jack_result_json).unwrap_or(QueryResult::table(vec![], vec![]));
    if result.kind == QueryResultKind::Graph {
        if let Some(fixture) = &result.graph_fixture {
            let (nodes_json, edges_json, viewport_json) = fixture_to_workflow(fixture);
            return build_node_graph_scene(
                TRINITY_JACK_PLAY_SURFACE_RESULTS,
                TRINITY_JACK_PLAY_CONTROLLER_ID,
                NodeGraphScene::base(nodes_json, edges_json, viewport_json),
            );
        }
    }
    let (columns_json, rows_json) = result_to_table(&runtime.jack_result_json);
    build_table_scene(
        TRINITY_JACK_PLAY_SURFACE_RESULTS,
        TRINITY_JACK_PLAY_CONTROLLER_ID,
        TableScene::base(columns_json, rows_json),
    )
}
//#endregion 🔖Render

//#region 🔖TrinityJackPlayApp
/// 🔱 Trinity Jack play app — a jack-query editor over a live {@link GraphFixture} projection; all
/// document mutation flows through {@link TrinityGraphOperation}, all editor/selection/LOD state is runtime.
pub struct TrinityJackPlayApp {
    runtime: TrinityJackRuntime,
}

impl Default for TrinityJackPlayApp {
    fn default() -> Self {
        Self { runtime: seeded_jack_runtime() }
    }
}

impl DocumentApp for TrinityJackPlayApp {
    type Projection = GraphFixture;
    type Operation = TrinityGraphOperation;

    fn app_id(&self) -> &str {
        TRINITY_JACK_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        TRINITY_GRAPH_SCHEMA
    }

    fn initial_projection(&self) -> GraphFixture {
        default_fixture()
    }

    fn handle_action(
        &mut self,
        action: &str,
        args: Option<&Value>,
        doc: &DocumentView<'_, GraphFixture>,
        _view_state: &ViewState,
    ) -> ActionEmit<TrinityGraphOperation> {
        let fixture = doc.projection;
        match action {
            "setSelection" | "selectNode" | "nodeGraphSelect" => {
                self.runtime.selected_node_ids = selection_ids(args);
                ActionEmit::default()
            }
            "nodeGraphHover" | "textHover" => ActionEmit::default(),
            "nodeGraphViewport" => {
                if let Some(viewport_json) = args.and_then(|value| value.get("viewportJson")).and_then(|value| value.as_str()) {
                    if let Ok(camera) = serde_json::from_str::<Camera>(viewport_json) {
                        return ActionEmit::amend(vec![TrinityGraphOperation::SetCamera { camera }], "viewport");
                    }
                }
                ActionEmit::default()
            }
            "nodeGraphEdit" => {
                let operations = args
                    .and_then(|value| value.get("operations"))
                    .and_then(|value| value.as_array())
                    .cloned()
                    .unwrap_or_default();
                let mut emitted: Vec<TrinityGraphOperation> = Vec::new();
                let mut has_set_fixture = false;
                for operation in operations {
                    match operation.get("operation").and_then(|value| value.as_str()).unwrap_or("") {
                        "setFixture" => {
                            if let Some(next) = operation.get("fixtureJson").and_then(|value| value.as_str()).and_then(|json| GraphFixture::from_json(json).ok()) {
                                emitted.push(TrinityGraphOperation::SetFixture { fixture: next });
                                has_set_fixture = true;
                            }
                        }
                        "deleteSelection" => {
                            let deletes: Vec<TrinityGraphOperation> = self
                                .runtime
                                .selected_node_ids
                                .iter()
                                .filter(|id| fixture.nodes.iter().any(|node| &node.id == *id))
                                .map(|id| TrinityGraphOperation::DeleteNode { id: id.clone() })
                                .collect();
                            if !deletes.is_empty() {
                                self.runtime.selected_node_ids.clear();
                                emitted.extend(deletes);
                            }
                        }
                        _ => {}
                    }
                }
                if emitted.is_empty() {
                    ActionEmit::default()
                } else if has_set_fixture {
                    ActionEmit::amend(emitted, "node-graph-edit")
                } else {
                    ActionEmit::operations(emitted)
                }
            }
            "textEdit" => {
                if let Some(text) = args.and_then(|v| v.get("text")).and_then(|v| v.as_str()) {
                    self.runtime.jack_query = text.into();
                }
                ActionEmit::default()
            }
            "textSelect" => {
                let start = args.and_then(|v| v.get("start")).and_then(|v| v.as_u64()).unwrap_or(0);
                let end = args.and_then(|v| v.get("end")).and_then(|v| v.as_u64()).unwrap_or(start);
                self.runtime.editor_selection = Some(TrinityEditorSelection { start: start as usize, end: end as usize });
                ActionEmit::default()
            }
            "requestCompletions" => {
                self.runtime.revision += 1;
                ActionEmit::default()
            }
            "formatDocument" => {
                if let Ok(formatted) = jack_format(&self.runtime.jack_query) {
                    self.runtime.jack_query = formatted;
                }
                ActionEmit::default()
            }
            "setLodMode" => {
                if let (Some(window_id), Some(value)) = (
                    args.and_then(|v| v.get("windowId")).and_then(|v| v.as_str()),
                    args.and_then(|v| v.get("value")).and_then(|v| v.as_str()),
                ) {
                    self.runtime.lod_mode_by_window.insert(window_id.into(), value.into());
                }
                ActionEmit::default()
            }
            "loadExampleQuery" => {
                if let Some(query) = args.and_then(|v| v.get("query")).and_then(|v| v.as_str()) {
                    self.runtime.jack_query = query.into();
                    let (result_json, operations) = run_jack_query(fixture, query);
                    self.runtime.jack_result_json = result_json;
                    return ActionEmit::operations(operations);
                }
                ActionEmit::default()
            }
            "runJackQuery" | "submit" => {
                let query = args
                    .and_then(|v| v.get("query"))
                    .and_then(|v| v.as_str())
                    .filter(|value| !value.trim().is_empty())
                    .map(str::to_string)
                    .unwrap_or_else(|| self.runtime.jack_query.clone());
                self.runtime.jack_query = query.clone();
                let (result_json, operations) = run_jack_query(fixture, &query);
                self.runtime.jack_result_json = result_json;
                self.runtime.results_engagement_input.clear();
                ActionEmit::operations(operations)
            }
            "setActiveExample" => {
                let example_id = args.and_then(|v| v.get("exampleId")).and_then(|v| v.as_str()).unwrap_or("");
                if let Some(next) = fixture_dsl_for_preset(example_id).and_then(|dsl| GraphFixture::parse_dsl(dsl).ok()) {
                    self.runtime.active_fixture_id = example_id.into();
                    self.runtime.jack_query = preset_query(example_id).into();
                    let (result_json, _) = run_jack_query(&next, &self.runtime.jack_query);
                    self.runtime.jack_result_json = result_json;
                    return ActionEmit::operations(vec![TrinityGraphOperation::SetFixture { fixture: next }]);
                }
                ActionEmit::default()
            }
            "patchTrinityNodes" => {
                let node_ids: Vec<String> = args
                    .and_then(|v| v.get("nodeIds"))
                    .and_then(|v| serde_json::from_value(v.clone()).ok())
                    .unwrap_or_default();
                let field = args.and_then(|v| v.get("field")).and_then(|v| v.as_str()).unwrap_or("");
                let value = args.and_then(|v| v.get("value")).and_then(|v| v.as_str()).map(str::trim).unwrap_or("");
                if field == "name" && !node_ids.is_empty() && !value.is_empty() {
                    let operations: Vec<TrinityGraphOperation> = node_ids
                        .iter()
                        .filter(|id| fixture.nodes.iter().any(|node| &node.id == *id))
                        .map(|id| TrinityGraphOperation::Rename { id: id.clone(), name: value.into() })
                        .collect();
                    return ActionEmit::operations(operations);
                }
                ActionEmit::default()
            }
            "reorganize" => {
                self.runtime.reorganize_epoch += 1;
                match force_layout_fixture(fixture) {
                    Some(after) => ActionEmit::operations(reposition_operations(fixture, &after)),
                    None => ActionEmit::default(),
                }
            }
            "editorEngagementInput" => {
                if let Some(value) = args.and_then(|v| v.get("value")).and_then(|v| v.as_str()) {
                    self.runtime.editor_engagement_input = value.into();
                }
                ActionEmit::default()
            }
            "graphEngagementInput" => {
                if let Some(value) = args.and_then(|v| v.get("value")).and_then(|v| v.as_str()) {
                    self.runtime.graph_engagement_input = value.into();
                }
                ActionEmit::default()
            }
            "resultsEngagementInput" => {
                if let Some(value) = args.and_then(|v| v.get("value")).and_then(|v| v.as_str()) {
                    self.runtime.results_engagement_input = value.into();
                }
                ActionEmit::default()
            }
            "graphPointerDown" => {
                if let Some(node_id) = args.and_then(|v| v.get("nodeId")).and_then(|v| v.as_str()) {
                    self.runtime.selected_node_ids = vec![node_id.into()];
                }
                ActionEmit::default()
            }
            _ => ActionEmit::default(),
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, GraphFixture>, view_state: &ViewState) -> UiNode {
        let fixture = doc.projection;
        let labels = resolve_labels::<TrinityJackLabels>(view_state);
        match body_key {
            TRINITY_JACK_PLAY_BODY_GRAPH => render_graph(fixture, &self.runtime),
            TRINITY_JACK_PLAY_BODY_EDITOR => render_editor(fixture, &self.runtime),
            TRINITY_JACK_PLAY_BODY_RESULTS => render_results(&self.runtime),
            TRINITY_JACK_PLAY_BODY_DOCUMENT => build_document_tree(fixture, &self.runtime, labels),
            TRINITY_JACK_PLAY_BODY_CATALOGUE => build_catalogue_tree(&self.runtime, labels),
            TRINITY_JACK_PLAY_BODY_INSPECTION => build_inspector_tree(fixture, &self.runtime, labels),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn window_measures(&self, _doc: &DocumentView<'_, GraphFixture>, _view_state: &ViewState) -> HashMap<String, Vec<WindowMeasure>> {
        let mode = self
            .runtime
            .lod_mode_by_window
            .get(TRINITY_JACK_PLAY_WINDOW_GRAPH)
            .map(String::as_str)
            .unwrap_or(TRINITY_LOD_MODE_AUTOMATIC);
        HashMap::from([(
            TRINITY_JACK_PLAY_WINDOW_GRAPH.to_string(),
            vec![trinity_lod_measure(TRINITY_JACK_PLAY_WINDOW_GRAPH, mode)],
        )])
    }

    fn app_labels(&self, view_state: &ViewState) -> AppLabelsOverlay {
        let labels = resolve_labels::<TrinityJackLabels>(view_state);
        AppLabelsOverlay::default()
            .window_kind_label(TRINITY_JACK_PLAY_WINDOW_GRAPH, labels.window_graph)
            .window_kind_label(TRINITY_JACK_PLAY_WINDOW_EDITOR, labels.window_editor)
            .window_kind_label(TRINITY_JACK_PLAY_WINDOW_RESULTS, labels.window_results)
            .action_labels(trinity_jack_action_labels(is_de_locale(view_state)))
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
            .artifact_kind(ArtifactKindSpec {
                id: "graph.trinity".into(),
                name: "Trinity Graph".into(),
                source_format: "trinity.graph".into(),
                component_kind: "trinity".into(),
                dimension: "graph".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::Graph, form: MediaForm::Trinity },
                schema: "trinity.graph".into(),
                export_formats: vec![],
                import_formats: vec![],
            })
            .icon_id("trinity")
            .mode("explore", "Explore")
            .default_mode_id("explore")
            .window_kind(TRINITY_JACK_PLAY_WINDOW_GRAPH, "Nakagin Graph", TRINITY_JACK_PLAY_BODY_GRAPH, SurfaceKind::NodeGraph, "graph-dag")
            .window_kind(TRINITY_JACK_PLAY_WINDOW_EDITOR, "Jack Query", TRINITY_JACK_PLAY_BODY_EDITOR, SurfaceKind::TextEditor, "document-jack")
            .window_kind(TRINITY_JACK_PLAY_WINDOW_RESULTS, "Results", TRINITY_JACK_PLAY_BODY_RESULTS, SurfaceKind::Table, "table-2")
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
            .operation("nodeGraphViewport", "Set Graph Viewport")
            .operation("patchTrinityNodes", "Patch Nodes")
            .operation("reorganize", "Reorganize")
            .operation("runJackQuery", "Run Jack Query")
            .operation("submit", "Submit Jack Query")
            .operation("loadExampleQuery", "Load Example Query")
            .operation("setActiveExample", "Set Active Example")
            .view_action("setSelection", "Set Selection")
            .view_action("selectNode", "Select Node")
            .view_action("nodeGraphSelect", "Select Graph Node")
            .view_action("nodeGraphHover", "Hover Graph Node")
            .view_action("textEdit", "Edit Jack Query")
            .view_action("textSelect", "Select Jack Query Text")
            .view_action("textHover", "Hover Jack Query Text")
            .view_action("requestCompletions", "Request Completions")
            .view_action("formatDocument", "Format Jack Query")
            .view_action("setLodMode", "Set LOD Mode")
            .view_action("editorEngagementInput", "Editor Engagement Input")
            .view_action("graphEngagementInput", "Graph Engagement Input")
            .view_action("resultsEngagementInput", "Results Engagement Input")
            .view_action("graphPointerDown", "Graph Pointer Down")
            // 📝 Staged argument forms for the panel-visible preset loaders.
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", "Fixture", vec![
                    ActionArgOption::new("nakagin", "Nakagin — Table"),
                    ActionArgOption::new("branch-chain", "Branch — Graph"),
                ]).required(),
            ])
            .action_args("loadExampleQuery", vec![
                ActionArgDef::select("query", "Example", vec![
                    ActionArgOption::new("MATCH (a:Piece) WHERE a.name = 't_f0_b_c0' OR a.name = 't_f0_b_c1' RETURN a.name", "Where Or"),
                    ActionArgOption::new("MATCH (a:Piece)-[r:Connection]->(b:Piece) WHERE a.name = 'b' RETURN a, r, b", "Return Graph"),
                    ActionArgOption::new("MATCH (a:Piece) WHERE a.name = 'b' SET a.label = 'demo-label'", "Set Label"),
                    ActionArgOption::new("MATCH (a:Piece) WHERE a.name = 'b' SET a.x = 300, a.y = 120", "Set Position"),
                    ActionArgOption::new("CREATE (n:Piece)", "Create Node"),
                    ActionArgOption::new("MATCH (a:Piece), (b:Piece) WHERE a.name = 'b' AND b.name != 'b' CREATE (a)-[:Connection]->(b)", "Create Edge"),
                    ActionArgOption::new("MATCH (n:Piece) WHERE n.name = 'b' DELETE n", "Delete Leaf"),
                    ActionArgOption::new("MERGE (x:Piece)-[:Connection]->(y:Piece)", "Merge Edge"),
                ]).required(),
            ])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .keybinding("mod+alt+s", "commitCheckpoint"),
    )
    .example("nakagin", "Nakagin", default_fixture().print_dsl())
    .workflow("trinity", "Trinity", "graph")
}
//#endregion 🔖Manifest

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{testkit, ActionMeta, PluginApp, VcsDocumentApp};

    fn meta(actor: &str) -> ActionMeta {
        testkit::meta(actor)
    }

    fn new_app() -> VcsDocumentApp<TrinityJackPlayApp> {
        testkit::new_app()
    }

    fn node_id_at(app: &VcsDocumentApp<TrinityJackPlayApp>, index: usize) -> String {
        app.projection().expect("projection").nodes[index].id.clone()
    }

    #[test]
    fn renders_node_graph_scene() {
        let mut app = new_app();
        let node = app.render(TRINITY_JACK_PLAY_BODY_GRAPH, None, &ViewState::default()).expect("render");
        assert!(serde_json::to_string(&node).unwrap().contains("node-graph"));
    }

    #[test]
    fn renders_jack_editor() {
        let mut app = new_app();
        let node = app.render(TRINITY_JACK_PLAY_BODY_EDITOR, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("text-editor"));
        assert!(json.contains(TRINITY_JACK_DEFAULT_QUERY));
    }

    #[test]
    fn run_query_populates_results_and_a_set_query_mutates_projection() {
        let mut app = new_app();
        // The seeded default query (read-only) already populated the results runtime on construction.
        app.render(TRINITY_JACK_PLAY_BODY_RESULTS, None, &ViewState::default()).expect("render");
        let result = app
            .handle_action(
                "runJackQuery",
                Some(&json!({ "query": "MATCH (a:Piece) WHERE a.name = 'b' SET a.label = 'ran-label'" })),
                &ViewState::default(),
                &meta("local"),
            )
            .expect("run");
        assert!(!result.operations.is_empty(), "a SET query emits operations");
        let projection = app.projection().expect("projection");
        assert!(serde_json::to_string(&projection).unwrap().contains("ran-label"));
    }

    #[test]
    fn node_graph_select_updates_selection_and_document_tree() {
        let mut app = new_app();
        let node_id = node_id_at(&app, 0);
        let result = app
            .handle_action("nodeGraphSelect", Some(&json!({ "nodeIds": [node_id.clone()] })), &ViewState::default(), &meta("local"))
            .expect("select");
        assert!(result.operations.is_empty(), "selection is a view action, no operations");
        let tree = app.render(TRINITY_JACK_PLAY_BODY_DOCUMENT, None, &ViewState::default()).expect("render");
        assert!(serde_json::to_string(&tree).unwrap().contains(&format!("trinity-document.node.{node_id}")));
    }

    #[test]
    fn nakagin_fixture_has_nodes() {
        assert!(!default_fixture().nodes.is_empty());
    }

    #[test]
    fn editor_scene_has_tokens_and_diagnostics() {
        let mut app = new_app();
        let node = app.render(TRINITY_JACK_PLAY_BODY_EDITOR, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("tokensJson"));
        assert!(json.contains("diagnosticsJson"));
        assert!(json.contains("completionsJson"));
    }

    #[test]
    fn text_edit_updates_query_without_operations() {
        let mut app = new_app();
        let result = app
            .handle_action("textEdit", Some(&json!({ "text": "MATCH (a:Piece) RETURN a.name" })), &ViewState::default(), &meta("local"))
            .expect("edit");
        assert!(result.operations.is_empty());
        let node = app.render(TRINITY_JACK_PLAY_BODY_EDITOR, None, &ViewState::default()).expect("render");
        assert!(serde_json::to_string(&node).unwrap().contains("MATCH (a:Piece) RETURN a.name"));
    }

    #[test]
    fn graph_scene_has_lod_json() {
        let mut app = new_app();
        let node = app.render(TRINITY_JACK_PLAY_BODY_GRAPH, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("lodJson"));
        assert!(json.contains("automatic"));
    }

    #[test]
    fn set_lod_mode_persists_per_window() {
        let mut app = new_app();
        app.handle_action(
            "setLodMode",
            Some(&json!({ "windowId": TRINITY_JACK_PLAY_WINDOW_GRAPH, "value": "minimap" })),
            &ViewState::default(),
            &meta("local"),
        )
        .expect("lod");
        let measures = app.window_measures(&ViewState::default());
        let graph_measures = serde_json::to_string(&measures[TRINITY_JACK_PLAY_WINDOW_GRAPH]).unwrap();
        assert!(graph_measures.contains("minimap"));
    }

    #[test]
    fn return_graph_example_renders_node_graph_in_results() {
        let mut app = new_app();
        app.handle_action(
            "loadExampleQuery",
            Some(&json!({ "query": "MATCH (a:Piece)-[r:Connection]->(b:Piece) WHERE a.name = 'b' RETURN a, r, b" })),
            &ViewState::default(),
            &meta("local"),
        )
        .expect("load example");
        let node = app.render(TRINITY_JACK_PLAY_BODY_RESULTS, None, &ViewState::default()).expect("render");
        assert!(serde_json::to_string(&node).unwrap().contains("node-graph"));
    }

    #[test]
    fn catalogue_has_eight_example_queries() {
        let mut app = new_app();
        let node = app.render(TRINITY_JACK_PLAY_BODY_CATALOGUE, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        for id in ["where-or", "return-graph", "set-label", "set-position", "create-node", "create-edge", "delete-leaf", "merge-edge"] {
            assert!(json.contains(id), "missing example query {id}");
        }
    }

    #[test]
    fn inspector_has_flat_position_fields() {
        let mut app = new_app();
        let node_id = node_id_at(&app, 0);
        app.handle_action("nodeGraphSelect", Some(&json!({ "nodeIds": [node_id] })), &ViewState::default(), &meta("local")).expect("select");
        let node = app.render(TRINITY_JACK_PLAY_BODY_INSPECTION, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Flat U"));
        assert!(json.contains("Flat V"));
    }

    // 🧰 `VcsDocumentApp::tools()` no longer exists — utility bars are now derived by the renderer
    // from the utility registry, which this app declares none of. `runJackQuery` is a plain
    // operation and `undo` is a framework-injected History action; both still live in the
    // static `AppDefinition.actions` list (undo/redo render via the History rail, not a
    // per-app utility bar) — assert on that surface instead.
    #[test]
    fn app_definition_declares_run_jack_query_and_history_actions() {
        let definition = create_trinity_jack_app().definition;
        let action_ids: Vec<&str> = definition.actions.iter().map(|action| action.id.as_str()).collect();
        assert!(action_ids.contains(&"runJackQuery"));
        assert!(action_ids.contains(&"undo"));
    }

    #[test]
    fn trinity_jack_labels_resolve_native_by_default() {
        let mut app = new_app();
        let node = app.render(TRINITY_JACK_PLAY_BODY_DOCUMENT, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("\"Pieces\""));
        assert!(json.contains("\"Connections\""));
        assert!(!json.contains("Stücke"));
    }

    #[test]
    fn trinity_jack_labels_translate_panels_in_german() {
        let mut app = new_app();
        let view_state = ViewState { locale: Some("de".into()), ..ViewState::default() };
        let document_json = serde_json::to_string(&app.render(TRINITY_JACK_PLAY_BODY_DOCUMENT, None, &view_state).expect("render")).unwrap();
        assert!(document_json.contains("Stücke"));
        assert!(document_json.contains("Verbindungen"));
        assert!(!document_json.contains("\"Pieces\""));
        let catalogue_json = serde_json::to_string(&app.render(TRINITY_JACK_PLAY_BODY_CATALOGUE, None, &view_state).expect("render")).unwrap();
        assert!(catalogue_json.contains("Fixturen"));
        assert!(catalogue_json.contains("Beispielabfragen"));
        assert!(catalogue_json.contains("Manifestarten"));
        // 🧰 `VcsDocumentApp::tools()` no longer exists (see the removed utility-bar test above); the
        // "Verlauf" (History rail group) label had no per-app surface even before removal — only
        // the `runJackQuery` action label is this app's own to assert on.
        let action_labels = app.app_labels(&view_state).action_labels;
        assert_eq!(action_labels.get("runJackQuery").map(String::as_str), Some("Jack-Abfrage ausführen"));
    }

    #[test]
    fn patch_trinity_nodes_emits_rename_operation() {
        let mut app = new_app();
        let node_id = node_id_at(&app, 0);
        let result = app
            .handle_action(
                "patchTrinityNodes",
                Some(&json!({ "nodeIds": [node_id.clone()], "field": "name", "value": "renamed-node" })),
                &ViewState::default(),
                &meta("local"),
            )
            .expect("patch");
        assert_eq!(result.operations.len(), 1);
        let renamed = app.projection().expect("projection").nodes.iter().find(|node| node.id == node_id).unwrap().name.clone();
        assert_eq!(renamed, "renamed-node");
    }

    #[test]
    fn node_graph_viewport_emits_coalesced_set_camera() {
        let mut app = new_app();
        for zoom in [1.5, 2.0, 2.5] {
            app.handle_action(
                "nodeGraphViewport",
                Some(&json!({ "viewportJson": json!({ "x": 10.0, "y": 20.0, "zoom": zoom }).to_string() })),
                &ViewState::default(),
                &meta("local"),
            )
            .expect("viewport");
        }
        assert_eq!(app.projection().expect("projection").camera.zoom, 2.5);
        // Coalesced into a single undo step: undo restores the original camera, not an intermediate zoom.
        app.handle_action("undo", None, &ViewState::default(), &meta("local")).expect("undo");
        assert_eq!(app.projection().expect("projection").camera.zoom, default_fixture().camera.zoom);
    }

    #[test]
    fn set_active_example_replaces_fixture_via_set_fixture_operation() {
        let mut app = new_app();
        let before = app.projection().expect("projection").name.clone();
        let result = app
            .handle_action("setActiveExample", Some(&json!({ "exampleId": "branch-chain" })), &ViewState::default(), &meta("local"))
            .expect("example");
        assert_eq!(result.operations.len(), 1);
        let after = app.projection().expect("projection").name.clone();
        assert_ne!(before, after, "loading a different preset replaces the fixture");
    }

    #[test]
    fn node_graph_edit_delete_selection_emits_delete_node_operations() {
        let mut app = new_app();
        let node_id = node_id_at(&app, 0);
        app.handle_action("nodeGraphSelect", Some(&json!({ "nodeIds": [node_id.clone()] })), &ViewState::default(), &meta("local")).expect("select");
        let before = app.projection().expect("projection").nodes.len();
        let result = app
            .handle_action("nodeGraphEdit", Some(&json!({ "operations": [{ "operation": "deleteSelection" }] })), &ViewState::default(), &meta("local"))
            .expect("delete");
        assert!(!result.operations.is_empty());
        assert_eq!(app.projection().expect("projection").nodes.len(), before - 1);
    }

    #[test]
    fn undo_redo_round_trip_through_the_wrapper() {
        let mut app = new_app();
        let node_id = node_id_at(&app, 0);
        let before_name = app.projection().unwrap().nodes.iter().find(|n| n.id == node_id).unwrap().name.clone();
        testkit::assert_undo_redo_round_trip(
            &mut app,
            "patchTrinityNodes",
            Some(&json!({ "nodeIds": [node_id.clone()], "field": "name", "value": "undo-me" })),
            |app| app.projection().unwrap().nodes.iter().find(|n| n.id == node_id).unwrap().name.clone(),
            before_name,
            "undo-me".to_string(),
        );
    }

    /// 🧪 The definitional merge proof: two instances start from the same fixture, rename DISJOINT
    /// nodes (A renames node 0, B renames node 1), and exchanging operations over a `MemoryBackbone`
    /// converges both to contain BOTH renames — impossible under whole-document `setDocument` LWW.
    #[test]
    fn two_instances_converge_disjoint_edits_via_backbone() {
        let seed = new_app();
        let node0 = node_id_at(&seed, 0);
        let node1 = node_id_at(&seed, 1);
        drop(seed);
        testkit::assert_two_instances_converge::<TrinityJackPlayApp, _>(
            "mem://trinity-jack-convergence",
            ("patchTrinityNodes", Some(&json!({ "nodeIds": [node0.clone()], "field": "name", "value": "Renamed By A" }))),
            ("patchTrinityNodes", Some(&json!({ "nodeIds": [node1.clone()], "field": "name", "value": "Renamed By B" }))),
            |app| {
                let projection = app.projection().unwrap();
                (
                    projection.nodes.iter().find(|n| n.id == node0).unwrap().name.clone(),
                    projection.nodes.iter().find(|n| n.id == node1).unwrap().name.clone(),
                )
            },
        );
    }

    #[test]
    fn ingest_operations_is_idempotent() {
        let seed = new_app();
        let node_id = node_id_at(&seed, 0);
        drop(seed);
        testkit::assert_ingest_idempotent::<TrinityJackPlayApp, _>(
            "patchTrinityNodes",
            Some(&json!({ "nodeIds": [node_id.clone()], "field": "name", "value": "Hero" })),
            |app| app.projection().unwrap().nodes.iter().find(|n| n.id == node_id).unwrap().name.clone(),
        );
    }
}
//#endregion 🧪Tests
