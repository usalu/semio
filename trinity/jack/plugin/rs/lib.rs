//! 🔱 Trinity Jack plugin — jack query play app bundled as a hot-swappable WASM component.

use semio_framework_plugin::{
    build_node_graph_scene, build_table_scene, build_text_editor_scene, create_default_layout,
    ui_declarative_sections_to_tree, ui_inspector_groups_to_tree, ui_inspector_mixed_text,
    ui_inspector_readonly_field, ui_text, App, CommandDescriptor, NodeGraphScene, PluginApp, PluginBundle,
    TableScene, TextEditorScene, UiFieldNode, UiInspectorFieldGroup, UiNode, UiSectionNode, UiTreeItemNode,
    UiTreeNode, UiTreeSectionNode, ViewState, WindowLayout, WindowLayoutAxisNode, WindowLayoutChild,
    WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
    FRAMEWORK_PANEL_TAB_HIERARCHY_ID, FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, UI_INSPECTOR_MIXED_PLACEHOLDER,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::LazyLock;
use trinity_jack::{execute, parse, run_json, QueryResult, QueryResultKind};
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
const TRINITY_JACK_PLAY_BODY_HIERARCHY: &str = "trinity.jack.play.hierarchy";
const TRINITY_JACK_PLAY_BODY_CATALOGUE: &str = "trinity.jack.play.catalogue";
const TRINITY_JACK_PLAY_BODY_INSPECTION: &str = "trinity.jack.play.inspection";
const TRINITY_JACK_PLAY_WINDOW_GRAPH: &str = "trinity-jack-graph";
const TRINITY_JACK_PLAY_WINDOW_EDITOR: &str = "trinity-jack-editor";
const TRINITY_JACK_PLAY_WINDOW_RESULTS: &str = "trinity-jack-results";

const NAKAGIN_FIXTURE_JSON: &str = include_str!("../../../example/nakagin-capsule-tower.trinity.json");
const BRANCH_FIXTURE_JSON: &str = include_str!("../../../example/branch-chain.trinity.json");

const TRINITY_JACK_DEFAULT_QUERY: &str =
    "MATCH (a:Piece)-[r:Connection]->(b:Piece) WHERE a.name = 'b' AND b.name != 'b' RETURN a.name, b.name, b.label";
//#endregion 🔖Constants

//#region 🔖Envelope
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
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TrinityJackEnvelope {
    fixture_json: String,
    #[serde(default)]
    graph_vcs: Option<TrinityGraphEnvelope>,
    #[serde(default)]
    runtime: TrinityJackRuntime,
}

fn default_envelope() -> TrinityJackEnvelope {
    TrinityJackEnvelope {
        fixture_json: NAKAGIN_FIXTURE_JSON.into(),
        graph_vcs: None,
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

fn jack_cmd(command: &str, args: Option<Value>) -> CommandDescriptor {
    CommandDescriptor {
        controller_id: TRINITY_JACK_PLAY_CONTROLLER_ID.into(),
        command: command.into(),
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
        return TrinityGraphStore::new(vcs.clone());
    }
    let fixture = GraphFixture::from_json(&envelope.fixture_json)
        .or_else(|_| GraphFixture::from_json(NAKAGIN_FIXTURE_JSON))
        .unwrap_or_else(|_| trinity_ram::empty_trinity_graph_fixture());
    TrinityGraphStore::new(create_trinity_graph_envelope("trinity-jack", fixture))
}

fn sync_envelope_from_store(envelope: &mut TrinityJackEnvelope, store: &TrinityGraphStore) {
    envelope.graph_vcs = Some(store.envelope().clone());
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
    args.and_then(|value| value.get("ids"))
        .and_then(|value| serde_json::from_value(value.clone()).ok())
        .unwrap_or_default()
}
//#endregion 🔖Envelope

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
    if parsed.kind == QueryResultKind::Graph {
        if let Some(fixture) = parsed.graph_fixture {
            let columns = vec![json!({ "id": "index", "label": "#" }), json!({ "id": "id", "label": "Id" }), json!({ "id": "name", "label": "Name" }), json!({ "id": "kind", "label": "Kind" })];
            let rows: Vec<Value> = fixture
                .nodes
                .iter()
                .enumerate()
                .map(|(index, node)| {
                    json!({
                        "index": index + 1,
                        "id": node.id,
                        "name": node.name,
                        "kind": node.kind,
                    })
                })
                .collect();
            return (
                serde_json::to_string(&columns).unwrap_or_else(|_| "[]".into()),
                serde_json::to_string(&rows).unwrap_or_else(|_| "[]".into()),
            );
        }
    }
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

fn tree_item_with_command(
    id: impl Into<String>,
    label: impl Into<String>,
    description: Option<String>,
    command: CommandDescriptor,
) -> UiTreeItemNode {
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

fn build_hierarchy_tree(envelope: &TrinityJackEnvelope) -> UiNode {
    let Some(fixture) = parse_fixture_json(&envelope.fixture_json) else {
        return ui_text("Invalid trinity fixture");
    };
    let node_items: Vec<UiTreeItemNode> = fixture
        .nodes
        .iter()
        .map(|node| {
            tree_item_with_command(
                format!("trinity-hierarchy.node.{}", node.id),
                if node.name.is_empty() { node.id.clone() } else { node.name.clone() },
                Some(node.kind.clone()),
                jack_cmd("setSelection", Some(json!({ "ids": [node.id] }))),
            )
        })
        .collect();
    let edge_items: Vec<UiTreeItemNode> = fixture
        .edges
        .iter()
        .map(|edge| tree_item(
            format!("trinity-hierarchy.edge.{}", edge.id),
            format!("{} → {}", edge.source, edge.target),
        ))
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![
            UiTreeSectionNode {
                id: "trinity-hierarchy.nodes".into(),
                label: Some("Pieces".into()),
                default_open: Some(true),
                items: node_items,
            },
            UiTreeSectionNode {
                id: "trinity-hierarchy.edges".into(),
                label: Some("Connections".into()),
                default_open: Some(false),
                items: edge_items,
            },
        ],
        selected_ids: Some(
            envelope
                .runtime
                .selected_node_ids
                .iter()
                .map(|id| format!("trinity-hierarchy.node.{id}"))
                .collect(),
        ),
        highlighted_ids: None,
        selection_change: Some(jack_cmd("setSelection", Some(json!({ "ids": [] })))),
    })
}

fn build_catalogue_tree(envelope: &TrinityJackEnvelope) -> UiNode {
    let fixtures = [("nakagin", "Nakagin — Table"), ("branch-chain", "Branch — Graph")];
    let examples = [
        ("where-or", "Where Or", "MATCH (a:Piece) WHERE a.name = 't_f0_b_c0' OR a.name = 't_f0_b_c1' RETURN a.name"),
        ("return-graph", "Return Graph", "MATCH (a:Piece)-[r:Connection]->(b:Piece) WHERE a.name = 'b' RETURN a, r, b"),
        ("set-label", "Set Label", "MATCH (a:Piece) WHERE a.name = 'b' SET a.label = 'demo-label'"),
    ];
    UiNode::Tree(UiTreeNode {
        sections: vec![
            UiTreeSectionNode {
                id: "trinity-jack-catalogue.fixtures".into(),
                label: Some("Fixtures".into()),
                default_open: Some(true),
                items: fixtures
                    .iter()
                    .map(|(id, label)| {
                        tree_item_with_command(
                            format!("trinity-jack-catalogue.fixture.{id}"),
                            *label,
                            Some(preset_query(id).into()),
                            jack_cmd("setActiveExample", Some(json!({ "exampleId": id }))),
                        )
                    })
                    .collect(),
            },
            UiTreeSectionNode {
                id: "trinity-jack-catalogue.examples".into(),
                label: Some("Example queries".into()),
                default_open: Some(true),
                items: examples
                    .iter()
                    .map(|(id, label, query)| {
                        tree_item_with_command(
                            format!("trinity-jack-catalogue.example.{id}"),
                            *label,
                            Some((*query).into()),
                            jack_cmd("loadExampleQuery", Some(json!({ "query": query }))),
                        )
                    })
                    .collect(),
            },
            UiTreeSectionNode {
                id: "trinity-jack-catalogue.kinds".into(),
                label: Some("Manifest kinds".into()),
                default_open: Some(false),
                items: vec![
                    tree_item("trinity-jack-catalogue.piece", "Piece"),
                    tree_item("trinity-jack-catalogue.connection", "Connection"),
                    tree_item("trinity-jack-catalogue.connector", "Connector"),
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
    })
}

fn build_inspector_tree(envelope: &TrinityJackEnvelope) -> UiNode {
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
    ui_inspector_groups_to_tree(&[
        UiInspectorFieldGroup {
            id: "trinity-inspector.geometry".into(),
            label: "Geometry".into(),
            default_open: None,
            fields: vec![ui_inspector_readonly_field(
                "trinity-inspector.ports",
                "Connectors",
                if ports_mixed.placeholder.is_none() {
                    port_counts.first().cloned().unwrap_or_default()
                } else {
                    ports_mixed.placeholder.unwrap_or_else(|| UI_INSPECTOR_MIXED_PLACEHOLDER.into())
                },
            )],
        },
        UiInspectorFieldGroup {
            id: "trinity-inspector.identity".into(),
            label: "Identity".into(),
            default_open: None,
            fields: vec![
                semio_framework_plugin::UiNode::Field(UiFieldNode {
                    id: "trinity-inspector.name".into(),
                    label: "Name".into(),
                    child: semio_framework_plugin::UiControlNode::Input(semio_framework_plugin::UiInputNode {
                        id: "trinity-inspector.name.input".into(),
                        input_kind: "text".into(),
                        value: name_mixed.value,
                        placeholder: name_mixed.placeholder,
                        commit: None,
                        on_change: jack_cmd(
                            "patchTrinityNodes",
                            Some(json!({ "nodeIds": node_ids, "field": "name" })),
                        ),
                    }),
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
    build_node_graph_scene(
        TRINITY_JACK_PLAY_SURFACE_GRAPH,
        TRINITY_JACK_PLAY_CONTROLLER_ID,
        NodeGraphScene::base(nodes_json, edges_json, viewport_json),
    )
}

fn render_editor(envelope: &TrinityJackEnvelope) -> UiNode {
    build_text_editor_scene(
        TRINITY_JACK_PLAY_SURFACE_EDITOR,
        TRINITY_JACK_PLAY_CONTROLLER_ID,
        TextEditorScene::base(envelope.runtime.jack_query.clone(), Some("jack".into()), None),
    )
}

fn render_results(envelope: &TrinityJackEnvelope) -> UiNode {
    let (columns_json, rows_json) = result_to_table(&envelope.runtime.jack_result_json);
    build_table_scene(
        TRINITY_JACK_PLAY_SURFACE_RESULTS,
        TRINITY_JACK_PLAY_CONTROLLER_ID,
        TableScene { columns_json, rows_json },
    )
}
//#endregion 🔖Render

//#region 🔖TrinityJackPlayApp
struct TrinityJackPlayApp;

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
                if let Some(next) = args.and_then(|value| value.get("document")) {
                    if let Ok(parsed) = serde_json::from_value(next.clone()) {
                        return vec![set_document_op(&parsed)];
                    }
                }
            }
            "setSelection" => {
                envelope.runtime.selected_node_ids = selection_ids(args);
                return vec![set_document_op(&envelope)];
            }
            "setJackQuery" => {
                if let Some(value) = args.and_then(|v| v.get("value")).and_then(|v| v.as_str()) {
                    envelope.runtime.jack_query = value.into();
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
            "runJackQuery" => {
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

    fn render(&self, body_key: &str, document_json: &str, _view_state: &ViewState) -> UiNode {
        let envelope = parse_envelope(document_json);
        match body_key {
            TRINITY_JACK_PLAY_BODY_GRAPH => render_graph(&envelope),
            TRINITY_JACK_PLAY_BODY_EDITOR => render_editor(&envelope),
            TRINITY_JACK_PLAY_BODY_RESULTS => render_results(&envelope),
            TRINITY_JACK_PLAY_BODY_HIERARCHY => build_hierarchy_tree(&envelope),
            TRINITY_JACK_PLAY_BODY_CATALOGUE => build_catalogue_tree(&envelope),
            TRINITY_JACK_PLAY_BODY_INSPECTION => build_inspector_tree(&envelope),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }
}
//#endregion 🔖TrinityJackPlayApp

//#region 🔖Manifest
fn jack_window_stack(id: &str, title: &str, size: Option<f64>) -> WindowLayoutChild {
    WindowLayoutChild::Stack(WindowLayoutStackNode {
        kind: "stack".into(),
        size,
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

fn create_trinity_jack_app() -> App {
    App::from_builder(
        App::builder(TRINITY_JACK_PLAY_APP_ID, "Trinity Jack")
            .icon_id("trinity")
            .mode("explore", "Explore")
            .default_mode_id("explore")
            .window_kind(TRINITY_JACK_PLAY_WINDOW_GRAPH, "Nakagin Graph", TRINITY_JACK_PLAY_BODY_GRAPH)
            .window_kind(TRINITY_JACK_PLAY_WINDOW_EDITOR, "Jack Query", TRINITY_JACK_PLAY_BODY_EDITOR)
            .window_kind(TRINITY_JACK_PLAY_WINDOW_RESULTS, "Results", TRINITY_JACK_PLAY_BODY_RESULTS)
            .default_layout(jack_layout())
            .panel_tab(
                FRAMEWORK_PANEL_TAB_HIERARCHY_ID,
                FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
                "workbench",
                TRINITY_JACK_PLAY_BODY_HIERARCHY,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                "workbench",
                TRINITY_JACK_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                "details",
                TRINITY_JACK_PLAY_BODY_INSPECTION,
            )
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .keybinding("mod+alt+s", "commitCheckpoint"),
    )
    .example("nakagin", "Nakagin", serde_json::to_string(&default_envelope()).unwrap())
    .program("trinity", "Trinity", "graph")
}

fn bundle() -> PluginBundle {
    PluginBundle::new("trinity", "Trinity", "0.1.0").register_app(create_trinity_jack_app(), || {
        Box::new(TrinityJackPlayApp)
    })
}

static _PLUGIN_INIT: LazyLock<()> = LazyLock::new(|| semio_framework_plugin::install_plugin_bundle(bundle()));

semio_framework_plugin::wasm_plugin_exports!();
//#endregion 🔖Manifest

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
        for op in app.handle_command("runJackQuery", None, &next, &ViewState::default()) {
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
    fn nakagin_fixture_has_nodes() {
        let fixture = parse_fixture_json(NAKAGIN_FIXTURE_JSON).expect("nakagin fixture");
        assert!(!fixture.nodes.is_empty());
    }
}
//#endregion 🧪Tests
