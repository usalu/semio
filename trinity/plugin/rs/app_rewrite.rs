//! ♻️ Trinity Rewrite plugin — parametric rewrite play app bundled as a hot-swappable WASM component.

use semio_framework_plugin::{PanelGroup, 
    build_node_graph_scene, build_text_editor_scene, ui_declarative_sections_to_tree, ui_inspector_groups_to_tree,
    ui_inspector_mixed_text, ui_inspector_readonly_field, ui_text, App, CommandDescriptor, NodeGraphScene, PluginApp,
    PluginBundle, TextEditorScene, UiFieldNode, UiInspectorFieldGroup, UiNode, UiSectionNode, UiTreeItemNode,
    UiTreeNode, UiTreeSectionNode, ViewState, WindowLayout, WindowLayoutAxisNode, WindowLayoutChild, WindowLayoutRoot,
    WindowLayoutStackNode, WindowLayoutWindowNode, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
    FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, UI_INSPECTOR_MIXED_PLACEHOLDER,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::LazyLock;
use trinity_ram::{Graph, GraphFixture, Node, PortDirection, PropertyValue};
use trinity_rewrite::{apply_rule, build_rule_query, rule_query_json, Lhs, ParameterKind, ParameterSpec, Rhs, Rule, PatternJson};

//#region 🔖Constants
const TRINITY_REWRITE_PLAY_APP_ID: &str = "trinity-rewrite-play";
const TRINITY_REWRITE_PLAY_CONTROLLER_ID: &str = "trinity-rewrite-play";
const TRINITY_REWRITE_PLAY_SURFACE_BEFORE: &str = "trinity.rewrite.before";
const TRINITY_REWRITE_PLAY_SURFACE_AFTER: &str = "trinity.rewrite.after";
const TRINITY_REWRITE_PLAY_SURFACE_LHS: &str = "trinity.rewrite.lhs";
const TRINITY_REWRITE_PLAY_SURFACE_RHS: &str = "trinity.rewrite.rhs";
const TRINITY_REWRITE_PLAY_SURFACE_JACK: &str = "trinity.rewrite.jack";
const TRINITY_REWRITE_PLAY_SURFACE_PARAMETERS: &str = "trinity.rewrite.parameters";
const TRINITY_REWRITE_PLAY_BODY_BEFORE: &str = "trinity.rewrite.play.before";
const TRINITY_REWRITE_PLAY_BODY_AFTER: &str = "trinity.rewrite.play.after";
const TRINITY_REWRITE_PLAY_BODY_LHS: &str = "trinity.rewrite.play.lhs";
const TRINITY_REWRITE_PLAY_BODY_RHS: &str = "trinity.rewrite.play.rhs";
const TRINITY_REWRITE_PLAY_BODY_JACK: &str = "trinity.rewrite.play.jack";
const TRINITY_REWRITE_PLAY_BODY_PARAMETERS: &str = "trinity.rewrite.play.parameters";
const TRINITY_REWRITE_PLAY_BODY_DOCUMENT: &str = "trinity.rewrite.play.document";
const TRINITY_REWRITE_PLAY_BODY_CATALOGUE: &str = "trinity.rewrite.play.catalogue";
const TRINITY_REWRITE_PLAY_BODY_INSPECTION: &str = "trinity.rewrite.play.inspection";
const TRINITY_REWRITE_PLAY_WINDOW_BEFORE: &str = "trinity-rewrite-before";
const TRINITY_REWRITE_PLAY_WINDOW_AFTER: &str = "trinity-rewrite-after";
const TRINITY_REWRITE_PLAY_WINDOW_LHS: &str = "trinity-rewrite-lhs";
const TRINITY_REWRITE_PLAY_WINDOW_RHS: &str = "trinity-rewrite-rhs";
const TRINITY_REWRITE_PLAY_WINDOW_JACK: &str = "trinity-rewrite-jack";
const TRINITY_REWRITE_PLAY_WINDOW_PARAMETERS: &str = "trinity-rewrite-parameters";
const TRINITY_REWRITE_PLAY_RULE_NAME: &str = "label-core";

const NAKAGIN_FIXTURE_JSON: &str = include_str!("../../example/nakagin-capsule-tower.trinity.json");

const DEFAULT_LHS_JSON: &str = r#"{
  "pattern": {
    "leftVar": "a",
    "leftKind": "Piece",
    "edgeVar": "r",
    "edgeKind": "Connection",
    "rightVar": "b",
    "rightKind": "Piece"
  },
  "whereClause": "a.name = 'b'"
}"#;

const DEFAULT_RHS_JSON: &str = r#"{
  "create": [],
  "delete": [],
  "set": [{ "var": "a", "prop": "label", "value": "$label" }],
  "merge": [],
  "parameters": [{ "name": "label", "kind": "string", "default": "nakagin-core" }]
}"#;
//#endregion 🔖Constants

//#region 🔖Envelope
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RewritePlayRuntime {
    #[serde(default)]
    selected_node_ids: Vec<String>,
    #[serde(default)]
    parameter_bindings: HashMap<String, PropertyValue>,
    #[serde(default)]
    reorganize_epoch: u64,
    #[serde(default)]
    active_hover_var: String,
    #[serde(default)]
    hover_epoch: u64,
    #[serde(default)]
    active_select_var: String,
    #[serde(default)]
    select_epoch: u64,
    #[serde(default)]
    lhs_graph_hover_id: String,
    #[serde(default)]
    rhs_graph_hover_id: String,
    #[serde(default)]
    undo_stack: Vec<String>,
    #[serde(default)]
    redo_stack: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TrinityRewriteEnvelope {
    before_fixture_json: String,
    after_fixture_json: String,
    lhs_json: String,
    rhs_json: String,
    #[serde(default)]
    runtime: RewritePlayRuntime,
}

fn default_envelope() -> TrinityRewriteEnvelope {
    let mut envelope = TrinityRewriteEnvelope {
        before_fixture_json: NAKAGIN_FIXTURE_JSON.into(),
        after_fixture_json: NAKAGIN_FIXTURE_JSON.into(),
        lhs_json: DEFAULT_LHS_JSON.into(),
        rhs_json: DEFAULT_RHS_JSON.into(),
        runtime: RewritePlayRuntime::default(),
    };
    envelope.runtime.parameter_bindings = default_parameter_bindings(&envelope.rhs_json);
    envelope.after_fixture_json = apply_rewrite_to_fixture(&envelope.before_fixture_json, &envelope);
    envelope
}

fn parse_envelope(document_json: &str) -> TrinityRewriteEnvelope {
    serde_json::from_str(document_json).unwrap_or_else(|_| default_envelope())
}

fn set_document_op(envelope: &TrinityRewriteEnvelope) -> String {
    json!({ "op": "setDocument", "document": envelope }).to_string()
}

fn rewrite_cmd(command: &str, args: Option<Value>) -> CommandDescriptor {
    CommandDescriptor {
        controller_id: TRINITY_REWRITE_PLAY_CONTROLLER_ID.into(),
        command: command.into(),
        args,
    }
}

fn parse_fixture_json(json: &str) -> Option<GraphFixture> {
    GraphFixture::from_json(json).ok()
}

fn default_parameter_bindings(rhs_json: &str) -> HashMap<String, PropertyValue> {
    let Ok(rhs) = serde_json::from_str::<Rhs>(rhs_json) else {
        return HashMap::new();
    };
    rhs.parameters
        .iter()
        .map(|param| (param.name.clone(), param.default.clone()))
        .collect()
}

fn build_rule_from_envelope(envelope: &TrinityRewriteEnvelope) -> Result<Rule, String> {
    let lhs: Lhs = serde_json::from_str(&envelope.lhs_json).map_err(|e| e.to_string())?;
    let rhs: Rhs = serde_json::from_str(&envelope.rhs_json).map_err(|e| e.to_string())?;
    Ok(Rule {
        name: TRINITY_REWRITE_PLAY_RULE_NAME.into(),
        lhs,
        rhs,
    })
}

fn compiled_jack_query(envelope: &TrinityRewriteEnvelope) -> String {
    let rule_json = match build_rule_from_envelope(envelope) {
        Ok(rule) => serde_json::to_string(&rule).unwrap_or_default(),
        Err(_) => return String::new(),
    };
    let bindings_json = serde_json::to_string(&envelope.runtime.parameter_bindings).unwrap_or_else(|_| "{}".into());
    rule_query_json(&rule_json, &bindings_json)
        .ok()
        .and_then(|json| serde_json::from_str::<Value>(&json).ok())
        .and_then(|value| value.get("query").and_then(|query| query.as_str()).map(str::to_string))
        .unwrap_or_else(|| {
            build_rule_from_envelope(envelope)
                .map(|rule| build_rule_query(&rule, &envelope.runtime.parameter_bindings))
                .unwrap_or_default()
        })
}

fn apply_rewrite_to_fixture(before_json: &str, envelope: &TrinityRewriteEnvelope) -> String {
    let Ok(mut graph) = Graph::load_json(before_json) else {
        return before_json.into();
    };
    let Ok(rule) = build_rule_from_envelope(envelope) else {
        return before_json.into();
    };
    if apply_rule(&mut graph, &rule, &envelope.runtime.parameter_bindings).is_ok() {
        graph.fixture_json().unwrap_or_else(|_| before_json.into())
    } else {
        before_json.into()
    }
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

fn sync_select_var_from_node(envelope: &mut TrinityRewriteEnvelope, fixture_json: &str, node_id: &str) {
    if let Some(fixture) = parse_fixture_json(fixture_json) {
        if let Some(node) = fixture.nodes.iter().find(|node| node.id == node_id) {
            if let Some(var) = var_from_node_name(&node.name) {
                envelope.runtime.active_select_var = var;
            }
        }
    }
}

fn sync_hover_var_from_node(envelope: &mut TrinityRewriteEnvelope, fixture_json: &str, node_id: &str) {
    if let Some(fixture) = parse_fixture_json(fixture_json) {
        if let Some(node) = fixture.nodes.iter().find(|node| node.id == node_id) {
            if let Some(var) = var_from_node_name(&node.name) {
                envelope.runtime.active_hover_var = var;
            }
        }
    }
    envelope.runtime.hover_epoch += 1;
}

fn apply_rewrite_node_graph_edit_ops(envelope: &mut TrinityRewriteEnvelope, ops: &[Value]) -> bool {
    let mut changed = false;
    for op in ops {
        match op.get("op").and_then(|value| value.as_str()).unwrap_or("") {
            "setFixture" => {
                if let Some(fixture_json) = op.get("fixtureJson").and_then(|value| value.as_str()) {
                    if parse_fixture_json(fixture_json).is_some() {
                        let before_ids: std::collections::HashSet<String> = parse_fixture_json(&envelope.before_fixture_json)
                            .map(|fixture| fixture.nodes.into_iter().map(|node| node.id).collect())
                            .unwrap_or_default();
                        let matches_before = parse_fixture_json(fixture_json)
                            .map(|fixture| {
                                !fixture.nodes.is_empty()
                                    && fixture.nodes.iter().all(|node| before_ids.contains(&node.id))
                            })
                            .unwrap_or(false);
                        if matches_before {
                            envelope.before_fixture_json = fixture_json.into();
                        } else {
                            envelope.after_fixture_json = fixture_json.into();
                        }
                        envelope.after_fixture_json =
                            apply_rewrite_to_fixture(&envelope.before_fixture_json, envelope);
                        changed = true;
                    }
                }
            }
            "deleteSelection" => {
                if !envelope.runtime.selected_node_ids.is_empty() {
                    push_undo(envelope);
                    let ids = envelope.runtime.selected_node_ids.clone();
                    if let Some(mut fixture) = parse_fixture_json(&envelope.before_fixture_json) {
                        fixture.nodes.retain(|node| !ids.contains(&node.id));
                        fixture.edges.retain(|edge| {
                            let from = edge.source.split(':').next().unwrap_or(&edge.source);
                            let to = edge.target.split(':').next().unwrap_or(&edge.target);
                            !ids.iter().any(|id| id == from || id == to)
                        });
                        if let Ok(json) = Graph::from_fixture(fixture).and_then(|graph| graph.fixture_json()) {
                            envelope.before_fixture_json = json;
                            envelope.after_fixture_json =
                                apply_rewrite_to_fixture(&envelope.before_fixture_json, envelope);
                            envelope.runtime.selected_node_ids.clear();
                            changed = true;
                        }
                    }
                }
            }
            _ => {}
        }
    }
    changed
}

fn snapshot_envelope_json(envelope: &TrinityRewriteEnvelope) -> String {
    serde_json::to_string(envelope).unwrap_or_default()
}

fn push_undo(envelope: &mut TrinityRewriteEnvelope) {
    envelope.runtime.undo_stack.push(snapshot_envelope_json(envelope));
    if envelope.runtime.undo_stack.len() > 32 {
        envelope.runtime.undo_stack.remove(0);
    }
    envelope.runtime.redo_stack.clear();
}

fn patch_fixture_nodes(fixture_json: &str, node_ids: &[String], field: &str, value: &str) -> Option<String> {
    let mut fixture = GraphFixture::from_json(fixture_json).ok()?;
    for node in fixture.nodes.iter_mut() {
        if !node_ids.iter().any(|id| id == &node.id) {
            continue;
        }
        match field {
            "name" => node.name = value.into(),
            "kind" => node.kind = value.into(),
            _ => {}
        }
    }
    Graph::from_fixture(fixture).ok()?.fixture_json().ok()
}

fn pattern_graph_fixture(patterns: &[PatternJson], title: &str) -> GraphFixture {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    for (index, pattern) in patterns.iter().enumerate() {
        let left_id = format!("lhs-{}-{}", pattern.left_var, index);
        nodes.push(Node {
            id: left_id.clone(),
            name: format!("{}:{}", pattern.left_var, pattern.left_kind),
            kind: pattern.left_kind.clone(),
            x: (index as f64) * 220.0,
            y: 0.0,
            width: 120.0,
            height: 56.0,
            ports: vec![],
            properties: Default::default(),
        });
        if let (Some(right_var), Some(right_kind)) = (&pattern.right_var, &pattern.right_kind) {
            let right_id = format!("rhs-{}-{}", right_var, index);
            nodes.push(Node {
                id: right_id.clone(),
                name: format!("{right_var}:{right_kind}"),
                kind: right_kind.clone(),
                x: (index as f64) * 220.0 + 180.0,
                y: 0.0,
                width: 120.0,
                height: 56.0,
                ports: vec![],
                properties: Default::default(),
            });
            let edge_id = format!("edge-{index}");
            let edge_kind = pattern.edge_kind.clone().unwrap_or_else(|| "Connection".into());
            edges.push(trinity_ram::Edge {
                id: edge_id,
                kind: edge_kind,
                source: format!("{left_id}:out"),
                target: format!("{right_id}:in"),
                properties: Default::default(),
            });
        }
    }
    GraphFixture {
        schema: GraphFixture::SCHEMA.into(),
        name: title.into(),
        manifest_id: Some("nakagin".into()),
        manifest: trinity_ram::Manifest::nakagin_default(),
        camera: trinity_ram::Camera { x: 0.0, y: 0.0, zoom: 1.0 },
        nodes,
        edges,
        root_node_id: None,
    }
}

fn semantic_rule_node(id: &str, kind: &str, name: &str, x: f64, y: f64) -> Node {
    Node {
        id: id.into(),
        name: name.into(),
        kind: kind.into(),
        x,
        y,
        width: 160.0,
        height: 56.0,
        ports: vec![],
        properties: Default::default(),
    }
}

fn lhs_semantic_graph_fixture(lhs: &Lhs) -> GraphFixture {
    let mut nodes = vec![semantic_rule_node(
        "lhs-match",
        "rewrite.match",
        &format!("{}:{}", lhs.pattern.left_var, lhs.pattern.left_kind),
        0.0,
        0.0,
    )];
    let mut edges = Vec::new();
    if let Some(where_clause) = lhs.where_clause.as_deref().filter(|value| !value.trim().is_empty()) {
        nodes.push(semantic_rule_node("lhs-where", "rewrite.where", where_clause, 220.0, 80.0));
        edges.push(trinity_ram::Edge {
            id: "lhs-match-where".into(),
            kind: "rewrite.flow".into(),
            source: "lhs-match:out".into(),
            target: "lhs-where:in".into(),
            properties: Default::default(),
        });
    }
    GraphFixture {
        schema: GraphFixture::SCHEMA.into(),
        name: "lhs".into(),
        manifest_id: Some("nakagin".into()),
        manifest: trinity_ram::Manifest::nakagin_default(),
        camera: trinity_ram::Camera { x: 0.0, y: 0.0, zoom: 1.0 },
        nodes,
        edges,
        root_node_id: None,
    }
}

fn rhs_semantic_graph_fixture(rhs: &Rhs) -> GraphFixture {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let mut y = 0.0;
    for (index, pattern) in rhs.create.iter().enumerate() {
        let id = format!("rhs-create-{index}");
        nodes.push(semantic_rule_node(
            &id,
            "rewrite.create",
            &format!("{}:{}", pattern.left_var, pattern.left_kind),
            (index as f64) * 220.0,
            y,
        ));
    }
    y += 80.0;
    for (index, pattern) in rhs.merge.iter().enumerate() {
        let id = format!("rhs-merge-{index}");
        nodes.push(semantic_rule_node(
            &id,
            "rewrite.merge",
            &format!("{}:{}", pattern.left_var, pattern.left_kind),
            (index as f64) * 220.0,
            y,
        ));
    }
    y += 80.0;
    for (index, assignment) in rhs.set.iter().enumerate() {
        let id = format!("rhs-set-{index}");
        nodes.push(semantic_rule_node(
            &id,
            "rewrite.set",
            &format!("{}.{} = {:?}", assignment.var, assignment.prop, assignment.value),
            (index as f64) * 220.0,
            y,
        ));
    }
    y += 80.0;
    for (index, name) in rhs.delete.iter().enumerate() {
        let id = format!("rhs-delete-{index}");
        nodes.push(semantic_rule_node(&id, "rewrite.delete", name, (index as f64) * 220.0, y));
    }
    y += 80.0;
    for (index, parameter) in rhs.parameters.iter().enumerate() {
        let id = format!("rhs-parameter-{index}");
        let kind = match parameter.kind {
            ParameterKind::String => "string",
            ParameterKind::Number => "number",
            ParameterKind::Boolean => "boolean",
        };
        nodes.push(semantic_rule_node(
            &id,
            "rewrite.parameter",
            &format!("{}:{kind}", parameter.name),
            (index as f64) * 220.0,
            y,
        ));
    }
    if nodes.is_empty() {
        nodes.push(semantic_rule_node("rhs-empty", "rewrite.create", "result:Piece", 0.0, 0.0));
    }
    GraphFixture {
        schema: GraphFixture::SCHEMA.into(),
        name: "rhs".into(),
        manifest_id: Some("nakagin".into()),
        manifest: trinity_ram::Manifest::nakagin_default(),
        camera: trinity_ram::Camera { x: 0.0, y: 0.0, zoom: 1.0 },
        nodes,
        edges,
        root_node_id: None,
    }
}

fn lhs_graph_fixture_json(lhs_json: &str) -> String {
    let Ok(lhs) = serde_json::from_str::<Lhs>(lhs_json) else {
        return NAKAGIN_FIXTURE_JSON.into();
    };
    Graph::from_fixture(lhs_semantic_graph_fixture(&lhs))
        .ok()
        .and_then(|graph| graph.fixture_json().ok())
        .unwrap_or_else(|| NAKAGIN_FIXTURE_JSON.into())
}

fn rhs_graph_fixture_json(rhs_json: &str) -> String {
    let Ok(rhs) = serde_json::from_str::<Rhs>(rhs_json) else {
        return NAKAGIN_FIXTURE_JSON.into();
    };
    Graph::from_fixture(rhs_semantic_graph_fixture(&rhs))
        .ok()
        .and_then(|graph| graph.fixture_json().ok())
        .unwrap_or_else(|| NAKAGIN_FIXTURE_JSON.into())
}

fn node_id_for_var(fixture_json: &str, var: &str) -> Option<String> {
    if var.is_empty() {
        return None;
    }
    let fixture = GraphFixture::from_json(fixture_json).ok()?;
    fixture
        .nodes
        .iter()
        .find(|node| {
            node.name.starts_with(&format!("{var}:"))
                || node.name == var
                || var_from_node_name(&node.name).as_deref() == Some(var)
        })
        .map(|node| node.id.clone())
}

fn graph_hover_json(fixture_json: &str, hover_var: &str, hover_node_id: &str) -> Option<String> {
    let node_id = if !hover_node_id.is_empty() {
        Some(hover_node_id.to_string())
    } else {
        node_id_for_var(fixture_json, hover_var)
    }?;
    Some(json!({ "nodeId": node_id }).to_string())
}

fn graph_selection_json(fixture_json: &str, select_var: &str, selected_ids: &[String]) -> Option<String> {
    if !selected_ids.is_empty() {
        return serde_json::to_string(selected_ids).ok();
    }
    node_id_for_var(fixture_json, select_var).map(|id| serde_json::to_string(&vec![id]).unwrap_or_else(|_| "[]".into()))
}

fn var_from_node_name(name: &str) -> Option<String> {
    let trimmed = name.trim();
    if let Some((var, _)) = trimmed.split_once(':') {
        return Some(var.trim().into());
    }
    if let Some((var, _)) = trimmed.split_once(" : ") {
        return Some(var.trim().into());
    }
    None
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

fn split_endpoint(endpoint: &str) -> (String, String) {
    endpoint
        .split_once(':')
        .map(|(node, port)| (node.to_string(), port.to_string()))
        .unwrap_or_else(|| (endpoint.to_string(), "in".into()))
}

fn port_endpoint(node_id: &str, port_id: &str) -> String {
    format!("{node_id}:{port_id}")
}

fn fixture_to_media_graph(fixture: &GraphFixture) -> (String, String, String) {
    let nodes: Vec<MediaGraphNodeRecord> = fixture.nodes.iter().map(node_to_media_record).collect();
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

fn build_document_tree(envelope: &TrinityRewriteEnvelope) -> UiNode {
    let Some(fixture) = parse_fixture_json(&envelope.before_fixture_json) else {
        return ui_text("Invalid trinity fixture");
    };
    let node_items: Vec<UiTreeItemNode> = fixture
        .nodes
        .iter()
        .map(|node| UiTreeItemNode {
            id: format!("trinity-document.node.{}", node.id),
            label: if node.name.is_empty() { node.id.clone() } else { node.name.clone() },
            description: Some(node.kind.clone()),
            icon_id: None,
            selected: None,
            default_open: None,
            command: Some(rewrite_cmd("setSelection", Some(json!({ "ids": [node.id] })))),
        hover_command: None,
        unhover_command: None,
        actions: None,
            draggable: None,
            drag_data: None,
            items: None,
            control: None,
            is_hidden: None,
        })
        .collect();
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "trinity-document.nodes".into(),
            label: Some("Pieces".into()),
            default_open: Some(true),
            items: node_items,
        }],
        selected_ids: Some(
            envelope
                .runtime
                .selected_node_ids
                .iter()
                .map(|id| format!("trinity-document.node.{id}"))
                .collect(),
        ),
        highlighted_ids: None,
        selection_change: Some(rewrite_cmd("setSelection", Some(json!({ "ids": [] })))),
    })
}

fn build_catalogue_tree() -> UiNode {
    UiNode::Tree(UiTreeNode {
        sections: vec![UiTreeSectionNode {
            id: "trinity-catalogue.kinds".into(),
            label: Some("Catalogue".into()),
            default_open: Some(true),
            items: vec![
                tree_item("trinity-catalogue.piece", "Piece"),
                tree_item("trinity-catalogue.connection", "Connection"),
                tree_item("trinity-catalogue.connector", "Connector"),
            ],
        }],
        selected_ids: Some(vec![]),
        highlighted_ids: None,
        selection_change: None,
    })
}

fn build_inspector_tree(envelope: &TrinityRewriteEnvelope) -> UiNode {
    let Some(fixture) = parse_fixture_json(&envelope.before_fixture_json) else {
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
        return ui_text("Piece not found");
    }
    let name_mixed = ui_inspector_mixed_text(&nodes.iter().map(|node| node.name.clone()).collect::<Vec<_>>());
    let kind_mixed = ui_inspector_mixed_text(&nodes.iter().map(|node| node.kind.clone()).collect::<Vec<_>>());
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "trinity-inspector.identity".into(),
        label: "Identity".into(),
        default_open: None,
        fields: vec![
            ui_inspector_readonly_field(
                "trinity-inspector.name",
                "Name",
                if name_mixed.placeholder.is_none() {
                    name_mixed.value
                } else {
                    name_mixed.placeholder.unwrap_or_else(|| UI_INSPECTOR_MIXED_PLACEHOLDER.into())
                },
            ),
            ui_inspector_readonly_field(
                "trinity-inspector.kind",
                "Kind",
                if kind_mixed.placeholder.is_none() {
                    nodes.first().map(|node| node.kind.clone()).unwrap_or_default()
                } else {
                    kind_mixed.placeholder.unwrap_or_else(|| UI_INSPECTOR_MIXED_PLACEHOLDER.into())
                },
            ),
        ],
    }])
}

fn build_parameters_panel(envelope: &TrinityRewriteEnvelope) -> UiNode {
    let Ok(rhs) = serde_json::from_str::<Rhs>(&envelope.rhs_json) else {
        return ui_text("Invalid RHS");
    };
    let mut children: Vec<UiNode> = Vec::new();
    for param in &rhs.parameters {
        let value = envelope
            .runtime
            .parameter_bindings
            .get(&param.name)
            .cloned()
            .unwrap_or_else(|| param.default.clone());
        let display = match value {
            PropertyValue::String(text) => text,
            PropertyValue::Number(number) => number.to_string(),
            PropertyValue::Bool(flag) => flag.to_string(),
            _ => String::new(),
        };
        children.push(semio_framework_plugin::UiNode::Field(UiFieldNode {
            id: format!("trinity-rewrite.param.{}", param.name),
            label: param.name.clone(),
            child: semio_framework_plugin::UiControlNode::Input(semio_framework_plugin::UiInputNode {
                id: format!("trinity-rewrite.param.{}.input", param.name),
                input_kind: match param.kind {
                    ParameterKind::Number => "number",
                    ParameterKind::Boolean => "text",
                    ParameterKind::String => "text",
                }
                .into(),
                value: display,
                placeholder: Some(param.kind_label()),
                commit: Some("blur".into()),
                on_change: rewrite_cmd("setParameter", Some(json!({ "name": param.name }))),
            }),
        }));
    }
    if children.is_empty() {
        children.push(ui_text("No parameters declared on RHS."));
    }
    ui_declarative_sections_to_tree(&[UiSectionNode {
        id: "trinity-rewrite.parameters".into(),
        label: Some("Parameters".into()),
        default_open: Some(true),
        children,
    }])
}

trait ParameterKindLabel {
    fn kind_label(&self) -> String;
}

impl ParameterKindLabel for ParameterSpec {
    fn kind_label(&self) -> String {
        match self.kind {
            ParameterKind::String => "string".into(),
            ParameterKind::Number => "number".into(),
            ParameterKind::Boolean => "boolean".into(),
        }
    }
}
//#endregion 🔖Panels

//#region 🔖Render
fn render_rule_graph(
    surface_id: &str,
    fixture_json: &str,
    envelope: &TrinityRewriteEnvelope,
    hover_node_id: &str,
) -> UiNode {
    let fixture = parse_fixture_json(fixture_json).unwrap_or_else(|| GraphFixture::from_json(fixture_json).unwrap_or_else(|_| GraphFixture::from_json(NAKAGIN_FIXTURE_JSON).unwrap()));
    let (nodes_json, edges_json, viewport_json) = fixture_to_media_graph(&fixture);
    let hover_json = graph_hover_json(fixture_json, &envelope.runtime.active_hover_var, hover_node_id);
    let selection_json = graph_selection_json(fixture_json, &envelope.runtime.active_select_var, &envelope.runtime.selected_node_ids);
    build_node_graph_scene(
        surface_id,
        TRINITY_REWRITE_PLAY_CONTROLLER_ID,
        NodeGraphScene {
            hover_json,
            selection_json,
            ..NodeGraphScene::base(nodes_json, edges_json, viewport_json)
        },
    )
}

fn render_fixture_graph(surface_id: &str, fixture_json: &str, envelope: &TrinityRewriteEnvelope) -> UiNode {
    render_rule_graph(surface_id, fixture_json, envelope, "")
}

fn render_text_editor(surface_id: &str, buffer: &str, language: &str) -> UiNode {
    build_text_editor_scene(
        surface_id,
        TRINITY_REWRITE_PLAY_CONTROLLER_ID,
        TextEditorScene::base(buffer.into(), Some(language.into()), None),
    )
}
//#endregion 🔖Render

//#region 🔖TrinityRewritePlayApp
pub struct TrinityRewritePlayApp;

impl PluginApp for TrinityRewritePlayApp {
    fn app_id(&self) -> &str {
        TRINITY_REWRITE_PLAY_APP_ID
    }

    fn initial_document_json(&self) -> String {
        serde_json::to_string(&default_envelope()).expect("trinity rewrite envelope json")
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
            "setSelection" | "selectNode" | "nodeGraphSelect" => {
                envelope.runtime.selected_node_ids = selection_ids(args);
                if let Some(node_id) = envelope.runtime.selected_node_ids.first().cloned() {
                    let before_json = envelope.before_fixture_json.clone();
                    let after_json = envelope.after_fixture_json.clone();
                    sync_select_var_from_node(&mut envelope, &before_json, &node_id);
                    if envelope.runtime.active_select_var.is_empty() {
                        sync_select_var_from_node(&mut envelope, &after_json, &node_id);
                    }
                    envelope.runtime.select_epoch += 1;
                }
                return vec![set_document_op(&envelope)];
            }
            "nodeGraphHover" => {
                let node_id = args
                    .and_then(|value| value.get("hoverJson"))
                    .and_then(|value| {
                        if value.is_null() {
                            None
                        } else if let Some(text) = value.as_str() {
                            serde_json::from_str::<Value>(text)
                                .ok()
                                .and_then(|parsed| parsed.get("nodeId").and_then(|id| id.as_str().map(str::to_string)))
                        } else {
                            value.get("nodeId").and_then(|id| id.as_str().map(str::to_string))
                        }
                    });
                if let Some(node_id) = node_id {
                    let before_json = envelope.before_fixture_json.clone();
                    let after_json = envelope.after_fixture_json.clone();
                    sync_hover_var_from_node(&mut envelope, &before_json, &node_id);
                    if envelope.runtime.active_hover_var.is_empty() {
                        sync_hover_var_from_node(&mut envelope, &after_json, &node_id);
                    }
                    return vec![set_document_op(&envelope)];
                }
            }
            "nodeGraphViewport" => {
                if let Some(viewport_json) = args.and_then(|value| value.get("viewportJson")).and_then(|value| value.as_str()) {
                    if let Ok(camera) = serde_json::from_str::<trinity_ram::Camera>(viewport_json) {
                        if let Some(mut fixture) = parse_fixture_json(&envelope.before_fixture_json) {
                            fixture.camera = camera;
                            if let Ok(json) = Graph::from_fixture(fixture).and_then(|graph| graph.fixture_json()) {
                                envelope.before_fixture_json = json;
                                envelope.after_fixture_json =
                                    apply_rewrite_to_fixture(&envelope.before_fixture_json, &envelope);
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
                if apply_rewrite_node_graph_edit_ops(&mut envelope, &ops) {
                    return vec![set_document_op(&envelope)];
                }
            }
            "setLhsJson" => {
                if let Some(value) = args.and_then(|v| v.get("value")).and_then(|v| v.as_str()) {
                    push_undo(&mut envelope);
                    envelope.lhs_json = value.into();
                    envelope.after_fixture_json = apply_rewrite_to_fixture(&envelope.before_fixture_json, &envelope);
                    return vec![set_document_op(&envelope)];
                }
            }
            "setRhsJson" => {
                if let Some(value) = args.and_then(|v| v.get("value")).and_then(|v| v.as_str()) {
                    push_undo(&mut envelope);
                    envelope.rhs_json = value.into();
                    envelope.runtime.parameter_bindings = default_parameter_bindings(&envelope.rhs_json);
                    envelope.after_fixture_json = apply_rewrite_to_fixture(&envelope.before_fixture_json, &envelope);
                    return vec![set_document_op(&envelope)];
                }
            }
            "setParameter" => {
                let name = args.and_then(|v| v.get("name")).and_then(|v| v.as_str()).unwrap_or("");
                let value = args.and_then(|v| v.get("value")).and_then(|v| v.as_str()).unwrap_or("");
                if !name.is_empty() {
                    let Ok(rhs) = serde_json::from_str::<Rhs>(&envelope.rhs_json) else {
                        return Vec::new();
                    };
                    let kind = rhs
                        .parameters
                        .iter()
                        .find(|param| param.name == name)
                        .map(|param| param.kind.clone());
                    let parsed = match kind {
                        Some(ParameterKind::Number) => value.parse::<f64>().ok().map(PropertyValue::Number),
                        Some(ParameterKind::Boolean) => Some(PropertyValue::Bool(value.eq_ignore_ascii_case("true"))),
                        Some(ParameterKind::String) | None => Some(PropertyValue::String(value.into())),
                    };
                    if let Some(parsed) = parsed {
                        push_undo(&mut envelope);
                        envelope.runtime.parameter_bindings.insert(name.into(), parsed);
                        envelope.after_fixture_json =
                            apply_rewrite_to_fixture(&envelope.before_fixture_json, &envelope);
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "recomputeRewrite" | "reorganize" => {
                envelope.runtime.reorganize_epoch += 1;
                envelope.after_fixture_json = apply_rewrite_to_fixture(&envelope.before_fixture_json, &envelope);
                return vec![set_document_op(&envelope)];
            }
            "resetRule" => {
                envelope.lhs_json = DEFAULT_LHS_JSON.into();
                envelope.rhs_json = DEFAULT_RHS_JSON.into();
                envelope.runtime.parameter_bindings = default_parameter_bindings(&envelope.rhs_json);
                envelope.before_fixture_json = NAKAGIN_FIXTURE_JSON.into();
                envelope.after_fixture_json = apply_rewrite_to_fixture(&envelope.before_fixture_json, &envelope);
                return vec![set_document_op(&envelope)];
            }
            "graphPointerDown" => {
                if let Some(node_id) = args.and_then(|v| v.get("nodeId")).and_then(|v| v.as_str()) {
                    envelope.runtime.selected_node_ids = vec![node_id.into()];
                    return vec![set_document_op(&envelope)];
                }
            }
            "patchTrinityNodes" => {
                let node_ids: Vec<String> = args
                    .and_then(|v| v.get("nodeIds"))
                    .and_then(|v| serde_json::from_value(v.clone()).ok())
                    .unwrap_or_default();
                let field = args.and_then(|v| v.get("field")).and_then(|v| v.as_str()).unwrap_or("");
                let value = args.and_then(|v| v.get("value")).and_then(|v| v.as_str()).map(str::trim).unwrap_or("");
                if !node_ids.is_empty() && !field.is_empty() && !value.is_empty() {
                    push_undo(&mut envelope);
                    if let Some(next) = patch_fixture_nodes(&envelope.before_fixture_json, &node_ids, field, value) {
                        envelope.before_fixture_json = next;
                        envelope.after_fixture_json =
                            apply_rewrite_to_fixture(&envelope.before_fixture_json, &envelope);
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "setLhsGraphHover" | "setBeforeGraphHover" => {
                if let Some(id) = args.and_then(|v| v.get("id")).and_then(|v| v.as_str()) {
                    envelope.runtime.lhs_graph_hover_id = id.into();
                    if let Some(fixture) = parse_fixture_json(&envelope.before_fixture_json) {
                        if let Some(node) = fixture.nodes.iter().find(|node| node.id == id) {
                            if let Some(var) = var_from_node_name(&node.name) {
                                envelope.runtime.active_hover_var = var;
                            }
                        }
                    }
                    envelope.runtime.hover_epoch += 1;
                    return vec![set_document_op(&envelope)];
                }
            }
            "setRhsGraphHover" | "setAfterGraphHover" => {
                if let Some(id) = args.and_then(|v| v.get("id")).and_then(|v| v.as_str()) {
                    envelope.runtime.rhs_graph_hover_id = id.into();
                    if let Some(fixture) = parse_fixture_json(&envelope.after_fixture_json) {
                        if let Some(node) = fixture.nodes.iter().find(|node| node.id == id) {
                            if let Some(var) = var_from_node_name(&node.name) {
                                envelope.runtime.active_hover_var = var;
                            }
                        }
                    }
                    envelope.runtime.hover_epoch += 1;
                    return vec![set_document_op(&envelope)];
                }
            }
            "setJackSelect" | "textSelect" => {
                if let Some(var) = args.and_then(|v| v.get("var")).and_then(|v| v.as_str()) {
                    envelope.runtime.active_select_var = var.into();
                } else if let Some(start) = args.and_then(|v| v.get("start")).and_then(|v| v.as_u64()) {
                    let query = compiled_jack_query(&envelope);
                    let text = query.as_str();
                    let offset = start as usize;
                    if offset < text.len() {
                        let slice = &text[offset..];
                        let token: String = slice
                            .chars()
                            .take_while(|ch| ch.is_ascii_alphanumeric() || *ch == '_')
                            .collect();
                        if !token.is_empty() {
                            envelope.runtime.active_select_var = token;
                        }
                    }
                }
                envelope.runtime.select_epoch += 1;
                return vec![set_document_op(&envelope)];
            }
            "setJackHover" | "textHover" => {
                if let Some(var) = args.and_then(|v| v.get("var")).and_then(|v| v.as_str()) {
                    envelope.runtime.active_hover_var = var.into();
                } else if let Some(offset) = args.and_then(|v| v.get("offset")).and_then(|v| v.as_u64()) {
                    let query = compiled_jack_query(&envelope);
                    let text = query.as_str();
                    let offset = offset as usize;
                    if offset < text.len() {
                        let slice = &text[offset..];
                        let token: String = slice
                            .chars()
                            .take_while(|ch| ch.is_ascii_alphanumeric() || *ch == '_')
                            .collect();
                        if !token.is_empty() {
                            envelope.runtime.active_hover_var = token;
                        }
                    }
                }
                envelope.runtime.hover_epoch += 1;
                return vec![set_document_op(&envelope)];
            }
            "setLhsGraphSelect" | "setBeforeGraphSelect" => {
                if let Some(id) = args.and_then(|v| v.get("id")).and_then(|v| v.as_str()) {
                    envelope.runtime.selected_node_ids = vec![id.into()];
                    if let Some(fixture) = parse_fixture_json(&envelope.before_fixture_json) {
                        if let Some(node) = fixture.nodes.iter().find(|node| node.id == id) {
                            if let Some(var) = var_from_node_name(&node.name) {
                                envelope.runtime.active_select_var = var;
                            }
                        }
                    }
                    envelope.runtime.select_epoch += 1;
                    return vec![set_document_op(&envelope)];
                }
            }
            "setRhsGraphSelect" | "setAfterGraphSelect" => {
                if let Some(id) = args.and_then(|v| v.get("id")).and_then(|v| v.as_str()) {
                    envelope.runtime.selected_node_ids = vec![id.into()];
                    if let Some(fixture) = parse_fixture_json(&envelope.after_fixture_json) {
                        if let Some(node) = fixture.nodes.iter().find(|node| node.id == id) {
                            if let Some(var) = var_from_node_name(&node.name) {
                                envelope.runtime.active_select_var = var;
                            }
                        }
                    }
                    envelope.runtime.select_epoch += 1;
                    return vec![set_document_op(&envelope)];
                }
            }
            "setJackSelect" => {
                if let Some(var) = args.and_then(|v| v.get("var")).and_then(|v| v.as_str()) {
                    envelope.runtime.active_select_var = var.into();
                } else if let Some(offset) = args.and_then(|v| v.get("offset")).and_then(|v| v.as_u64()) {
                    let query = compiled_jack_query(&envelope);
                    let text = query.as_str();
                    let offset = offset as usize;
                    if offset < text.len() {
                        let slice = &text[offset..];
                        let token: String = slice
                            .chars()
                            .take_while(|ch| ch.is_ascii_alphanumeric() || *ch == '_')
                            .collect();
                        if !token.is_empty() {
                            envelope.runtime.active_select_var = token;
                        }
                    }
                }
                envelope.runtime.select_epoch += 1;
                return vec![set_document_op(&envelope)];
            }
            "undo" => {
                if let Some(previous_json) = envelope.runtime.undo_stack.pop() {
                    envelope.runtime.redo_stack.push(snapshot_envelope_json(&envelope));
                    if let Ok(previous) = serde_json::from_str(&previous_json) {
                        return vec![set_document_op(&previous)];
                    }
                }
            }
            "redo" => {
                if let Some(next_json) = envelope.runtime.redo_stack.pop() {
                    envelope.runtime.undo_stack.push(snapshot_envelope_json(&envelope));
                    if let Ok(next) = serde_json::from_str(&next_json) {
                        return vec![set_document_op(&next)];
                    }
                }
            }
            _ => {}
        }
        Vec::new()
    }

    fn render(&self, body_key: &str, document_json: &str, _view_state: &ViewState) -> UiNode {
        let envelope = parse_envelope(document_json);
        match body_key {
            TRINITY_REWRITE_PLAY_BODY_BEFORE => {
                render_fixture_graph(TRINITY_REWRITE_PLAY_SURFACE_BEFORE, &envelope.before_fixture_json, &envelope)
            }
            TRINITY_REWRITE_PLAY_BODY_AFTER => {
                render_fixture_graph(TRINITY_REWRITE_PLAY_SURFACE_AFTER, &envelope.after_fixture_json, &envelope)
            }
            TRINITY_REWRITE_PLAY_BODY_LHS => render_rule_graph(
                TRINITY_REWRITE_PLAY_SURFACE_LHS,
                &lhs_graph_fixture_json(&envelope.lhs_json),
                &envelope,
                &envelope.runtime.lhs_graph_hover_id,
            ),
            TRINITY_REWRITE_PLAY_BODY_RHS => render_rule_graph(
                TRINITY_REWRITE_PLAY_SURFACE_RHS,
                &rhs_graph_fixture_json(&envelope.rhs_json),
                &envelope,
                &envelope.runtime.rhs_graph_hover_id,
            ),
            TRINITY_REWRITE_PLAY_BODY_JACK => {
                render_text_editor(TRINITY_REWRITE_PLAY_SURFACE_JACK, &compiled_jack_query(&envelope), "jack")
            }
            TRINITY_REWRITE_PLAY_BODY_PARAMETERS => build_parameters_panel(&envelope),
            TRINITY_REWRITE_PLAY_BODY_DOCUMENT => build_document_tree(&envelope),
            TRINITY_REWRITE_PLAY_BODY_CATALOGUE => build_catalogue_tree(),
            TRINITY_REWRITE_PLAY_BODY_INSPECTION => build_inspector_tree(&envelope),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }
}
//#endregion 🔖TrinityRewritePlayApp

//#region 🔖Manifest
fn rewrite_window_stack(id: &str, title: &str, size: Option<f64>) -> WindowLayoutChild {
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

fn rewrite_layout() -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Axis(WindowLayoutAxisNode {
            kind: "column".into(),
            size: None,
            children: vec![
                WindowLayoutChild::Axis(WindowLayoutAxisNode {
                    kind: "row".into(),
                    size: Some(0.5),
                    children: vec![
                        rewrite_window_stack(TRINITY_REWRITE_PLAY_WINDOW_LHS, "LHS", Some(0.34)),
                        rewrite_window_stack(TRINITY_REWRITE_PLAY_WINDOW_RHS, "RHS", Some(0.34)),
                        rewrite_window_stack(TRINITY_REWRITE_PLAY_WINDOW_JACK, "Jack", Some(0.32)),
                    ],
                }),
                WindowLayoutChild::Axis(WindowLayoutAxisNode {
                    kind: "row".into(),
                    size: Some(0.5),
                    children: vec![
                        rewrite_window_stack(TRINITY_REWRITE_PLAY_WINDOW_PARAMETERS, "Parameters", Some(0.34)),
                        rewrite_window_stack(TRINITY_REWRITE_PLAY_WINDOW_BEFORE, "Before", Some(0.33)),
                        rewrite_window_stack(TRINITY_REWRITE_PLAY_WINDOW_AFTER, "After", Some(0.33)),
                    ],
                }),
            ],
        }),
    }
}

pub fn create_rewrite_app() -> App {
    App::from_builder(
        App::builder(TRINITY_REWRITE_PLAY_APP_ID, "Trinity Rewrite").document(["semio", "trinity", "rewrite"])
            .icon_id("trinity-rewrite")
            .mode("explore", "Explore")
            .default_mode_id("explore")
            .window_kind(TRINITY_REWRITE_PLAY_WINDOW_BEFORE, "Before", TRINITY_REWRITE_PLAY_BODY_BEFORE)
            .window_kind(TRINITY_REWRITE_PLAY_WINDOW_AFTER, "After", TRINITY_REWRITE_PLAY_BODY_AFTER)
            .window_kind(TRINITY_REWRITE_PLAY_WINDOW_LHS, "LHS", TRINITY_REWRITE_PLAY_BODY_LHS)
            .window_kind(TRINITY_REWRITE_PLAY_WINDOW_RHS, "RHS", TRINITY_REWRITE_PLAY_BODY_RHS)
            .window_kind(TRINITY_REWRITE_PLAY_WINDOW_JACK, "Jack", TRINITY_REWRITE_PLAY_BODY_JACK)
            .window_kind(
                TRINITY_REWRITE_PLAY_WINDOW_PARAMETERS,
                "Parameters",
                TRINITY_REWRITE_PLAY_BODY_PARAMETERS,
            )
            .default_layout(rewrite_layout())
            .panel_tab(
                FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
                FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
                PanelGroup::Workbench,
                TRINITY_REWRITE_PLAY_BODY_DOCUMENT,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                PanelGroup::Workbench,
                TRINITY_REWRITE_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                PanelGroup::Details,
                TRINITY_REWRITE_PLAY_BODY_INSPECTION,
            ),
    )
    .example("label-core", "Label Core", serde_json::to_string(&default_envelope()).unwrap())
    .program("trinity-rewrite", "Trinity Rewrite", "graph")
}

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_before_and_after_graphs() {
        let app = TrinityRewritePlayApp;
        let document = app.initial_document_json();
        let before = app.render(TRINITY_REWRITE_PLAY_BODY_BEFORE, &document, &ViewState::default());
        let after = app.render(TRINITY_REWRITE_PLAY_BODY_AFTER, &document, &ViewState::default());
        let before_json = serde_json::to_string(&before).unwrap();
        let after_json = serde_json::to_string(&after).unwrap();
        assert!(before_json.contains("node-graph"));
        assert!(after_json.contains("node-graph"));
    }

    #[test]
    fn compiles_jack_query_from_rule() {
        let envelope = default_envelope();
        let query = compiled_jack_query(&envelope);
        assert!(query.contains("MATCH"));
        assert!(query.contains("SET"));
    }

    #[test]
    fn apply_rewrite_changes_after_fixture() {
        let envelope = default_envelope();
        assert_ne!(envelope.before_fixture_json, envelope.after_fixture_json);
    }

    #[test]
    fn renders_lhs_rhs_graphs() {
        let app = TrinityRewritePlayApp;
        let document = app.initial_document_json();
        let lhs = app.render(TRINITY_REWRITE_PLAY_BODY_LHS, &document, &ViewState::default());
        let rhs = app.render(TRINITY_REWRITE_PLAY_BODY_RHS, &document, &ViewState::default());
        let lhs_json = serde_json::to_string(&lhs).unwrap();
        let rhs_json = serde_json::to_string(&rhs).unwrap();
        assert!(lhs_json.contains("node-graph"));
        assert!(rhs_json.contains("node-graph"));
    }
}
//#endregion 🧪Tests
