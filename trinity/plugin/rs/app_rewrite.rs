//! ♻️ Trinity Rewrite plugin — parametric rewrite play app bundled as a hot-swappable WASM component.

use semio_framework_plugin::{SurfaceKind, PanelGroup,
    build_node_graph_scene, build_text_editor_scene, text_identifier_bounds_at,
    tool_button, tool_collection,
    ui_declarative_sections_to_tree, ui_inspector_groups_to_tree,
    ui_inspector_mixed_text, ui_inspector_readonly_field, ui_text, App, ActionDescriptor, NodeGraphScene, PluginApp,
    TextEditorScene, ToolNode, UiFieldNode, UiInspectorFieldGroup, UiNode, UiSectionNode, UiTreeItemNode,
    UiTreeNode, UiTreeSectionNode, ViewState, WindowLayout, WindowLayoutAxisNode, WindowLayoutChild, WindowLayoutRoot,
    WindowLayoutStackNode, WindowLayoutWindowNode, WindowMeasure, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
    FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, UI_INSPECTOR_MIXED_PLACEHOLDER,
};
use semio_framework_plugin::layout::MeasureSelectItem;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashMap};
use trinity_jack::semantic_tokens;
use trinity_ram::{Graph, GraphFixture, Node, PortDirection, PropertyValue};
use trinity_rewrite::{
    apply_rule, build_rule_query, rule_query_json, trinity_lod_scale_json,
    create_rewrite_rule_envelope, dispatch_rewrite_rule_state, AssignmentJson, Lhs, ParameterKind, ParameterSpec, Rhs, Rule,
    PatternJson, RewriteRuleEnvelope, RewriteRuleState, RewriteRuleStore,
};
use vcs::DocumentVcsCommand;

//#region 🔖Constants
const TRINITY_REWRITE_PLAY_APP_ID: &str = "trinity-rewrite-play";
const TRINITY_REWRITE_PLAY_CONTROLLER_ID: &str = "trinity-rewrite-play";
const TRINITY_REWRITE_PLAY_SURFACE_BEFORE: &str = "trinity.rewrite.before";
const TRINITY_REWRITE_PLAY_SURFACE_AFTER: &str = "trinity.rewrite.after";
const TRINITY_REWRITE_PLAY_SURFACE_LHS: &str = "trinity.rewrite.lhs";
const TRINITY_REWRITE_PLAY_SURFACE_RHS: &str = "trinity.rewrite.rhs";
const TRINITY_REWRITE_PLAY_SURFACE_JACK: &str = "trinity.rewrite.jack";
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

const TRINITY_LOD_MODE_AUTOMATIC: &str = "automatic";
//#endregion 🔖Constants

//#region 🔖Envelope
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RewritePlayRuntime {
    #[serde(default)]
    selected_node_ids: Vec<String>,
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
    lod_mode_by_window: BTreeMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TrinityRewriteEnvelope {
    rule_vcs: RewriteRuleEnvelope,
    #[serde(default)]
    rule_applied_edit_ids: Vec<String>,
    #[serde(default)]
    runtime: RewritePlayRuntime,
}

fn default_rule_state() -> RewriteRuleState {
    let mut state = RewriteRuleState {
        before_fixture_json: NAKAGIN_FIXTURE_JSON.into(),
        lhs_json: DEFAULT_LHS_JSON.into(),
        rhs_json: DEFAULT_RHS_JSON.into(),
        parameter_bindings: HashMap::new(),
        rule_layout: HashMap::new(),
    };
    state.parameter_bindings = default_parameter_bindings(&state.rhs_json);
    state
}

fn default_envelope() -> TrinityRewriteEnvelope {
    TrinityRewriteEnvelope {
        rule_vcs: create_rewrite_rule_envelope(TRINITY_REWRITE_PLAY_APP_ID, default_rule_state()),
        rule_applied_edit_ids: Vec::new(),
        runtime: RewritePlayRuntime::default(),
    }
}

fn parse_envelope(document_json: &str) -> TrinityRewriteEnvelope {
    serde_json::from_str(document_json).unwrap_or_else(|_| default_envelope())
}

fn set_document_op(envelope: &TrinityRewriteEnvelope) -> String {
    json!({ "op": "setDocument", "document": envelope }).to_string()
}

fn rewrite_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor {
        controller_id: TRINITY_REWRITE_PLAY_CONTROLLER_ID.into(),
        action: action.into(),
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

fn rule_store_from_envelope(envelope: &TrinityRewriteEnvelope) -> RewriteRuleStore {
    let mut store = RewriteRuleStore::new(envelope.rule_vcs.clone());
    store.set_envelope(envelope.rule_vcs.clone(), envelope.rule_applied_edit_ids.clone());
    store
}

fn rule_state(envelope: &TrinityRewriteEnvelope) -> RewriteRuleState {
    rule_store_from_envelope(envelope).projection().unwrap_or_else(|_| default_rule_state())
}

fn sync_envelope_from_rule_store(envelope: &mut TrinityRewriteEnvelope, store: &RewriteRuleStore) {
    envelope.rule_vcs = store.envelope().clone();
    envelope.rule_applied_edit_ids = store.applied_edit_ids().to_vec();
}

fn apply_rule_state(envelope: &mut TrinityRewriteEnvelope, next: RewriteRuleState) -> bool {
    let mut store = rule_store_from_envelope(envelope);
    if dispatch_rewrite_rule_state(&mut store, next).is_ok() {
        sync_envelope_from_rule_store(envelope, &store);
        true
    } else {
        false
    }
}

fn build_rule_from_state(state: &RewriteRuleState) -> Result<Rule, String> {
    let lhs: Lhs = serde_json::from_str(&state.lhs_json).map_err(|e| e.to_string())?;
    let rhs: Rhs = serde_json::from_str(&state.rhs_json).map_err(|e| e.to_string())?;
    Ok(Rule {
        name: TRINITY_REWRITE_PLAY_RULE_NAME.into(),
        lhs,
        rhs,
    })
}

fn compiled_jack_query(state: &RewriteRuleState) -> String {
    let rule_json = match build_rule_from_state(state) {
        Ok(rule) => serde_json::to_string(&rule).unwrap_or_default(),
        Err(_) => return String::new(),
    };
    let bindings_json = serde_json::to_string(&state.parameter_bindings).unwrap_or_else(|_| "{}".into());
    rule_query_json(&rule_json, &bindings_json)
        .ok()
        .and_then(|json| serde_json::from_str::<Value>(&json).ok())
        .and_then(|value| value.get("query").and_then(|query| query.as_str()).map(str::to_string))
        .unwrap_or_else(|| {
            build_rule_from_state(state)
                .map(|rule| build_rule_query(&rule, &state.parameter_bindings))
                .unwrap_or_default()
        })
}

fn apply_rewrite_to_fixture(before_json: &str, state: &RewriteRuleState) -> String {
    let Ok(mut graph) = Graph::load_json(before_json) else {
        return before_json.into();
    };
    let Ok(rule) = build_rule_from_state(state) else {
        return before_json.into();
    };
    if apply_rule(&mut graph, &rule, &state.parameter_bindings).is_ok() {
        graph.fixture_json().unwrap_or_else(|_| before_json.into())
    } else {
        before_json.into()
    }
}

fn after_fixture_json(state: &RewriteRuleState) -> String {
    apply_rewrite_to_fixture(&state.before_fixture_json, state)
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

/// 🧭 Resolves which fixture backs a given rewrite graph surface (Before/After/LHS/RHS), for hover/select var lookup.
fn fixture_json_for_surface(surface_id: &str, state: &RewriteRuleState) -> String {
    if surface_id == TRINITY_REWRITE_PLAY_SURFACE_AFTER {
        after_fixture_json(state)
    } else if surface_id == TRINITY_REWRITE_PLAY_SURFACE_LHS {
        lhs_graph_fixture_json(&state.lhs_json, &state.rule_layout)
    } else if surface_id == TRINITY_REWRITE_PLAY_SURFACE_RHS {
        rhs_graph_fixture_json(&state.rhs_json, &state.rule_layout)
    } else {
        state.before_fixture_json.clone()
    }
}

fn apply_semantic_layout_edit(rule_layout: &mut HashMap<String, (f64, f64)>, current_fixture_json: &str, edited_fixture_json: &str) -> bool {
    let (Some(current), Some(edited)) = (parse_fixture_json(current_fixture_json), parse_fixture_json(edited_fixture_json)) else {
        return false;
    };
    let mut changed = false;
    for node in &edited.nodes {
        let Some(prev) = current.nodes.iter().find(|entry| entry.id == node.id) else {
            continue;
        };
        if (prev.x - node.x).abs() > 1e-6 || (prev.y - node.y).abs() > 1e-6 {
            rule_layout.insert(node.id.clone(), (node.x, node.y));
            changed = true;
        }
    }
    changed
}

enum RuleClauseRef {
    LhsWhere,
    RhsCreate(usize),
    RhsMerge(usize),
    RhsSet(usize),
    RhsDelete(usize),
    RhsParameter(usize),
}

fn parse_clause_ref(node_id: &str) -> Option<RuleClauseRef> {
    if node_id == "lhs-where" {
        return Some(RuleClauseRef::LhsWhere);
    }
    let (prefix, index) = node_id.rsplit_once('-')?;
    let index: usize = index.parse().ok()?;
    match prefix {
        "rhs-create" => Some(RuleClauseRef::RhsCreate(index)),
        "rhs-merge" => Some(RuleClauseRef::RhsMerge(index)),
        "rhs-set" => Some(RuleClauseRef::RhsSet(index)),
        "rhs-delete" => Some(RuleClauseRef::RhsDelete(index)),
        "rhs-parameter" => Some(RuleClauseRef::RhsParameter(index)),
        _ => None,
    }
}

fn remove_at<T>(items: &mut Vec<T>, index: usize) -> bool {
    if index < items.len() {
        items.remove(index);
        true
    } else {
        false
    }
}

fn delete_rule_clause(state: &mut RewriteRuleState, node_id: &str) -> bool {
    let Some(clause_ref) = parse_clause_ref(node_id) else {
        return false;
    };
    let Ok(mut lhs) = serde_json::from_str::<Lhs>(&state.lhs_json) else {
        return false;
    };
    let Ok(mut rhs) = serde_json::from_str::<Rhs>(&state.rhs_json) else {
        return false;
    };
    let changed = match clause_ref {
        RuleClauseRef::LhsWhere => {
            let had = lhs.where_clause.is_some();
            lhs.where_clause = None;
            had
        }
        RuleClauseRef::RhsCreate(index) => remove_at(&mut rhs.create, index),
        RuleClauseRef::RhsMerge(index) => remove_at(&mut rhs.merge, index),
        RuleClauseRef::RhsSet(index) => remove_at(&mut rhs.set, index),
        RuleClauseRef::RhsDelete(index) => remove_at(&mut rhs.delete, index),
        RuleClauseRef::RhsParameter(index) => {
            if index < rhs.parameters.len() {
                let removed = rhs.parameters.remove(index);
                state.parameter_bindings.remove(&removed.name);
                true
            } else {
                false
            }
        }
    };
    if changed {
        state.lhs_json = serde_json::to_string(&lhs).unwrap_or_default();
        state.rhs_json = serde_json::to_string(&rhs).unwrap_or_default();
        state.rule_layout.remove(node_id);
    }
    changed
}

/// ➕ Appends a default instance of `clause_kind` to the rule (rewrite.where/create/merge/set/delete/parameter).
fn add_rule_clause(state: &mut RewriteRuleState, clause_kind: &str) -> bool {
    let Ok(mut lhs) = serde_json::from_str::<Lhs>(&state.lhs_json) else {
        return false;
    };
    let Ok(mut rhs) = serde_json::from_str::<Rhs>(&state.rhs_json) else {
        return false;
    };
    let left_var = lhs.pattern.left_var.clone();
    let changed = match clause_kind {
        "where" => {
            if lhs.where_clause.is_some() {
                false
            } else {
                lhs.where_clause = Some(format!("{left_var}.name = 'value'"));
                true
            }
        }
        "create" => {
            rhs.create.push(PatternJson { left_var: "n".into(), left_kind: "Piece".into(), edge_var: None, edge_kind: None, right_var: None, right_kind: None });
            true
        }
        "merge" => {
            rhs.merge.push(PatternJson { left_var: "n".into(), left_kind: "Piece".into(), edge_var: None, edge_kind: None, right_var: None, right_kind: None });
            true
        }
        "set" => {
            rhs.set.push(AssignmentJson { var: left_var, prop: "label".into(), value: PropertyValue::String(String::new()) });
            true
        }
        "delete" => {
            rhs.delete.push(left_var);
            true
        }
        "parameter" => {
            let name = format!("param{}", rhs.parameters.len());
            state.parameter_bindings.insert(name.clone(), PropertyValue::String(String::new()));
            rhs.parameters.push(ParameterSpec { name, kind: ParameterKind::String, default: PropertyValue::String(String::new()) });
            true
        }
        _ => false,
    };
    if changed {
        state.lhs_json = serde_json::to_string(&lhs).unwrap_or_default();
        state.rhs_json = serde_json::to_string(&rhs).unwrap_or_default();
    }
    changed
}

fn apply_rewrite_node_graph_edit_ops(envelope: &mut TrinityRewriteEnvelope, surface_id: &str, ops: &[Value]) -> bool {
    let mut state = rule_state(envelope);
    let mut changed = false;
    for op in ops {
        match op.get("op").and_then(|value| value.as_str()).unwrap_or("") {
            "setFixture" => {
                let Some(fixture_json) = op.get("fixtureJson").and_then(|value| value.as_str()) else {
                    continue;
                };
                if parse_fixture_json(fixture_json).is_none() {
                    continue;
                }
                if surface_id == TRINITY_REWRITE_PLAY_SURFACE_BEFORE {
                    state.before_fixture_json = fixture_json.into();
                    changed = true;
                } else if surface_id == TRINITY_REWRITE_PLAY_SURFACE_LHS {
                    let current = lhs_graph_fixture_json(&state.lhs_json, &state.rule_layout);
                    changed |= apply_semantic_layout_edit(&mut state.rule_layout, &current, fixture_json);
                } else if surface_id == TRINITY_REWRITE_PLAY_SURFACE_RHS {
                    let current = rhs_graph_fixture_json(&state.rhs_json, &state.rule_layout);
                    changed |= apply_semantic_layout_edit(&mut state.rule_layout, &current, fixture_json);
                }
            }
            "deleteSelection" => {
                if envelope.runtime.selected_node_ids.is_empty() {
                    continue;
                }
                if surface_id == TRINITY_REWRITE_PLAY_SURFACE_BEFORE {
                    let ids = envelope.runtime.selected_node_ids.clone();
                    if let Some(mut fixture) = parse_fixture_json(&state.before_fixture_json) {
                        fixture.nodes.retain(|node| !ids.contains(&node.id));
                        fixture.edges.retain(|edge| {
                            let from = edge.source.split(':').next().unwrap_or(&edge.source);
                            let to = edge.target.split(':').next().unwrap_or(&edge.target);
                            !ids.iter().any(|id| id == from || id == to)
                        });
                        if let Ok(json) = Graph::from_fixture(fixture).and_then(|graph| graph.fixture_json()) {
                            state.before_fixture_json = json;
                            envelope.runtime.selected_node_ids.clear();
                            changed = true;
                        }
                    }
                } else if surface_id == TRINITY_REWRITE_PLAY_SURFACE_LHS || surface_id == TRINITY_REWRITE_PLAY_SURFACE_RHS {
                    let ids = envelope.runtime.selected_node_ids.clone();
                    let mut deleted = false;
                    for id in &ids {
                        deleted |= delete_rule_clause(&mut state, id);
                    }
                    if deleted {
                        envelope.runtime.selected_node_ids.clear();
                        changed = true;
                    }
                }
            }
            _ => {}
        }
    }
    changed && apply_rule_state(envelope, state)
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

fn semantic_rule_node(id: &str, kind: &str, name: &str, x: f64, y: f64, rule_layout: &HashMap<String, (f64, f64)>) -> Node {
    let (x, y) = rule_layout.get(id).copied().unwrap_or((x, y));
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

fn lhs_semantic_graph_fixture(lhs: &Lhs, rule_layout: &HashMap<String, (f64, f64)>) -> GraphFixture {
    let mut nodes = vec![semantic_rule_node(
        "lhs-match",
        "rewrite.match",
        &format!("{}:{}", lhs.pattern.left_var, lhs.pattern.left_kind),
        0.0,
        0.0,
        rule_layout,
    )];
    let mut edges = Vec::new();
    if let Some(where_clause) = lhs.where_clause.as_deref().filter(|value| !value.trim().is_empty()) {
        nodes.push(semantic_rule_node("lhs-where", "rewrite.where", where_clause, 220.0, 80.0, rule_layout));
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

fn rhs_semantic_graph_fixture(rhs: &Rhs, rule_layout: &HashMap<String, (f64, f64)>) -> GraphFixture {
    let mut nodes = Vec::new();
    let edges = Vec::new();
    let mut y = 0.0;
    for (index, pattern) in rhs.create.iter().enumerate() {
        let id = format!("rhs-create-{index}");
        nodes.push(semantic_rule_node(
            &id,
            "rewrite.create",
            &format!("{}:{}", pattern.left_var, pattern.left_kind),
            (index as f64) * 220.0,
            y,
            rule_layout,
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
            rule_layout,
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
            rule_layout,
        ));
    }
    y += 80.0;
    for (index, name) in rhs.delete.iter().enumerate() {
        let id = format!("rhs-delete-{index}");
        nodes.push(semantic_rule_node(&id, "rewrite.delete", name, (index as f64) * 220.0, y, rule_layout));
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
            rule_layout,
        ));
    }
    if nodes.is_empty() {
        nodes.push(semantic_rule_node("rhs-empty", "rewrite.create", "result:Piece", 0.0, 0.0, rule_layout));
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

fn lhs_graph_fixture_json(lhs_json: &str, rule_layout: &HashMap<String, (f64, f64)>) -> String {
    let Ok(lhs) = serde_json::from_str::<Lhs>(lhs_json) else {
        return NAKAGIN_FIXTURE_JSON.into();
    };
    Graph::from_fixture(lhs_semantic_graph_fixture(&lhs, rule_layout))
        .ok()
        .and_then(|graph| graph.fixture_json().ok())
        .unwrap_or_else(|| NAKAGIN_FIXTURE_JSON.into())
}

fn rhs_graph_fixture_json(rhs_json: &str, rule_layout: &HashMap<String, (f64, f64)>) -> String {
    let Ok(rhs) = serde_json::from_str::<Rhs>(rhs_json) else {
        return NAKAGIN_FIXTURE_JSON.into();
    };
    Graph::from_fixture(rhs_semantic_graph_fixture(&rhs, rule_layout))
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

fn tree_item_with_action(id: impl Into<String>, label: impl Into<String>, description: Option<String>, action: ActionDescriptor) -> UiTreeItemNode {
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

fn build_document_tree(envelope: &TrinityRewriteEnvelope) -> UiNode {
    let state = rule_state(envelope);
    let Some(fixture) = parse_fixture_json(&state.before_fixture_json) else {
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
            action: Some(rewrite_action("setSelection", Some(json!({ "ids": [node.id] })))),
        hover_action: None,
        unhover_action: None,
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
        selection_change: Some(rewrite_action("setSelection", Some(json!({ "ids": [] })))),
        drop_action: None,
    })
}

fn catalogue_add_item(id: &str, label: &str, clause_kind: &str) -> UiTreeItemNode {
    tree_item_with_action(id, label, None, rewrite_action("addRuleClause", Some(json!({ "kind": clause_kind }))))
}

fn build_catalogue_tree() -> UiNode {
    UiNode::Tree(UiTreeNode {
        sections: vec![
            UiTreeSectionNode {
                id: "trinity-catalogue.kinds".into(),
                label: Some("Catalogue".into()),
                default_open: Some(true),
                items: vec![
                    tree_item("trinity-catalogue.piece", "Piece"),
                    tree_item("trinity-catalogue.connection", "Connection"),
                    tree_item("trinity-catalogue.connector", "Connector"),
                ],
            },
            UiTreeSectionNode {
                id: "trinity-catalogue.lhs".into(),
                label: Some("Add to LHS".into()),
                default_open: Some(true),
                items: vec![catalogue_add_item("trinity-catalogue.add-where", "Where clause", "where")],
            },
            UiTreeSectionNode {
                id: "trinity-catalogue.rhs".into(),
                label: Some("Add to RHS".into()),
                default_open: Some(true),
                items: vec![
                    catalogue_add_item("trinity-catalogue.add-create", "Create pattern", "create"),
                    catalogue_add_item("trinity-catalogue.add-merge", "Merge pattern", "merge"),
                    catalogue_add_item("trinity-catalogue.add-set", "Set assignment", "set"),
                    catalogue_add_item("trinity-catalogue.add-delete", "Delete pattern", "delete"),
                    catalogue_add_item("trinity-catalogue.add-parameter", "Parameter", "parameter"),
                ],
            },
        ],
        selected_ids: Some(vec![]),
        highlighted_ids: None,
        selection_change: None,
        drop_action: None,
    })
}

fn flat_position_uv(node: &Node) -> (String, String) {
    let Some(flat) = node.properties.get("flatPosition").and_then(PropertyValue::as_object) else {
        return (String::new(), String::new());
    };
    let format_axis = |axis: &str| flat.get(axis).and_then(PropertyValue::as_f64).map(|value| format!("{value:.2}")).unwrap_or_default();
    (format_axis("u"), format_axis("v"))
}

fn fixture_with_derived(fixture_json: &str) -> Option<GraphFixture> {
    let mut graph = Graph::load_json(fixture_json).ok()?;
    graph.recompute_derived();
    Some(graph.to_fixture())
}

fn build_inspector_tree(envelope: &TrinityRewriteEnvelope) -> UiNode {
    let state = rule_state(envelope);
    let Some(fixture) = parse_fixture_json(&state.before_fixture_json) else {
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
    let node_ids: Vec<String> = nodes.iter().map(|node| node.id.clone()).collect();
    let name_mixed = ui_inspector_mixed_text(&nodes.iter().map(|node| node.name.clone()).collect::<Vec<_>>());
    let kind_mixed = ui_inspector_mixed_text(&nodes.iter().map(|node| node.kind.clone()).collect::<Vec<_>>());
    let derived_fixture = fixture_with_derived(&state.before_fixture_json);
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
            label: "Geometry".into(),
            default_open: None,
            fields: vec![
                ui_inspector_readonly_field(
                    "trinity-inspector.flat-u",
                    "Flat U",
                    if u_mixed.placeholder.is_none() { u_values.first().cloned().unwrap_or_default() } else { u_mixed.placeholder.unwrap_or_else(|| UI_INSPECTOR_MIXED_PLACEHOLDER.into()) },
                ),
                ui_inspector_readonly_field(
                    "trinity-inspector.flat-v",
                    "Flat V",
                    if v_mixed.placeholder.is_none() { v_values.first().cloned().unwrap_or_default() } else { v_mixed.placeholder.unwrap_or_else(|| UI_INSPECTOR_MIXED_PLACEHOLDER.into()) },
                ),
            ],
        },
        UiInspectorFieldGroup {
            id: "trinity-inspector.identity".into(),
            label: "Identity".into(),
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
                        on_change: rewrite_action("patchTrinityNodes", Some(json!({ "nodeIds": node_ids, "field": "name" }))),
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
            ],
        },
    ])
}

fn build_parameters_panel(envelope: &TrinityRewriteEnvelope) -> UiNode {
    let state = rule_state(envelope);
    let Ok(rhs) = serde_json::from_str::<Rhs>(&state.rhs_json) else {
        return ui_text("Invalid RHS");
    };
    let mut children: Vec<UiNode> = Vec::new();
    for param in &rhs.parameters {
        let value = state
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
            child: Box::new(semio_framework_plugin::UiNode::Input(semio_framework_plugin::UiInputNode {
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
                on_change: rewrite_action("setParameter", Some(json!({ "name": param.name }))),
                min: None,
                max: None,
                step: None,
                accept: None,
            })),
            description: None,
            required: None,
            error: None,
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
const DELETE_SELECTION_CONTEXT_MENU: &str =
    r#"[{"id":"delete-selection","label":"Delete selection","action":"nodeGraphEdit","args":{"ops":[{"op":"deleteSelection"}]}}]"#;

fn rewrite_lod_json_for_window(runtime: &RewritePlayRuntime, window_id: &str) -> Option<String> {
    let mode = runtime.lod_mode_by_window.get(window_id).map(String::as_str).unwrap_or(TRINITY_LOD_MODE_AUTOMATIC);
    if mode == TRINITY_LOD_MODE_AUTOMATIC {
        Some(json!({ "automatic": true }).to_string())
    } else {
        Some(json!({ "automatic": false, "forcedLabel": mode }).to_string())
    }
}

fn trinity_rewrite_lod_measure(window_id: &str, current_mode: &str) -> WindowMeasure {
    let mut items = vec![MeasureSelectItem { id: TRINITY_LOD_MODE_AUTOMATIC.into(), value: TRINITY_LOD_MODE_AUTOMATIC.into(), label: "Automatic".into() }];
    let rows: Vec<Value> = serde_json::from_str(&trinity_lod_scale_json()).unwrap_or_default();
    items.extend(rows.into_iter().filter_map(|row| {
        let id = row.get("id")?.as_str()?.to_string();
        let name = row.get("name").and_then(|value| value.as_str()).unwrap_or(&id).to_string();
        Some(MeasureSelectItem { id: id.clone(), value: id, label: name })
    }));
    WindowMeasure::Select {
        id: format!("{window_id}-lod"),
        label: Some("LOD".into()),
        value: current_mode.into(),
        items,
        on_change: rewrite_action("setLodMode", Some(json!({ "windowId": window_id }))),
    }
}

fn jack_token_at_offset(text: &str, offset: usize) -> Option<String> {
    if offset >= text.len() {
        return None;
    }
    let slice = &text[offset..];
    let token: String = slice.chars().take_while(|ch| ch.is_ascii_alphanumeric() || *ch == '_').collect();
    if token.is_empty() {
        None
    } else {
        Some(token)
    }
}

fn render_rule_graph(
    surface_id: &str,
    window_id: &str,
    fixture_json: &str,
    envelope: &TrinityRewriteEnvelope,
    hover_node_id: &str,
    editable: bool,
) -> UiNode {
    let fixture = parse_fixture_json(fixture_json).unwrap_or_else(|| GraphFixture::from_json(NAKAGIN_FIXTURE_JSON).unwrap());
    let (nodes_json, edges_json, viewport_json) = fixture_to_media_graph(&fixture);
    let hover_json = graph_hover_json(fixture_json, &envelope.runtime.active_hover_var, hover_node_id);
    let selection_json = graph_selection_json(fixture_json, &envelope.runtime.active_select_var, &envelope.runtime.selected_node_ids);
    build_node_graph_scene(
        surface_id,
        TRINITY_REWRITE_PLAY_CONTROLLER_ID,
        NodeGraphScene {
            hover_json,
            selection_json,
            lod_json: rewrite_lod_json_for_window(&envelope.runtime, window_id),
            editable: editable.then_some(true),
            context_menu_json: editable.then(|| DELETE_SELECTION_CONTEXT_MENU.into()),
            ..NodeGraphScene::base(nodes_json, edges_json, viewport_json)
        },
    )
}

fn render_fixture_graph(surface_id: &str, window_id: &str, fixture_json: &str, envelope: &TrinityRewriteEnvelope, editable: bool) -> UiNode {
    render_rule_graph(surface_id, window_id, fixture_json, envelope, "", editable)
}

fn var_occurrences_json(text: &str, var: &str) -> Option<String> {
    if var.is_empty() {
        return None;
    }
    let mut ranges = Vec::new();
    let mut scan = 0usize;
    while let Some(found) = text[scan..].find(var) {
        let at = scan + found;
        let end = at + var.len();
        if text_identifier_bounds_at(text, at) == Some((at, end)) {
            ranges.push(json!({ "start": at, "end": end }));
        }
        scan = at + var.len();
    }
    if ranges.is_empty() {
        return None;
    }
    let ranges_json = serde_json::to_string(&ranges).unwrap_or_else(|_| "[]".into());
    Some(json!({ "selection": ranges_json, "hover": ranges_json }).to_string())
}

fn render_jack_editor(envelope: &TrinityRewriteEnvelope) -> UiNode {
    let state = rule_state(envelope);
    let query = compiled_jack_query(&state);
    let active_var = if !envelope.runtime.active_hover_var.is_empty() {
        envelope.runtime.active_hover_var.as_str()
    } else {
        envelope.runtime.active_select_var.as_str()
    };
    build_text_editor_scene(
        TRINITY_REWRITE_PLAY_SURFACE_JACK,
        TRINITY_REWRITE_PLAY_CONTROLLER_ID,
        TextEditorScene {
            tokens_json: serde_json::to_string(&semantic_tokens(&query)).ok(),
            occurrences_json: var_occurrences_json(&query, active_var),
            ..TextEditorScene::base(query, Some("jack".into()), None)
        },
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
                let surface_id = args.and_then(|value| value.get("surfaceId")).and_then(|value| value.as_str()).unwrap_or("");
                if let Some(node_id) = envelope.runtime.selected_node_ids.first().cloned() {
                    let state = rule_state(&envelope);
                    let fixture_json = fixture_json_for_surface(surface_id, &state);
                    sync_select_var_from_node(&mut envelope, &fixture_json, &node_id);
                    envelope.runtime.select_epoch += 1;
                }
                return vec![set_document_op(&envelope)];
            }
            "nodeGraphHover" => {
                let surface_id = args.and_then(|value| value.get("surfaceId")).and_then(|value| value.as_str()).unwrap_or("");
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
                    let state = rule_state(&envelope);
                    let fixture_json = fixture_json_for_surface(surface_id, &state);
                    sync_hover_var_from_node(&mut envelope, &fixture_json, &node_id);
                    return vec![set_document_op(&envelope)];
                }
            }
            "nodeGraphViewport" => {
                let surface_id = args.and_then(|value| value.get("surfaceId")).and_then(|value| value.as_str()).unwrap_or("");
                if surface_id == TRINITY_REWRITE_PLAY_SURFACE_BEFORE {
                    if let Some(viewport_json) = args.and_then(|value| value.get("viewportJson")).and_then(|value| value.as_str()) {
                        if let Ok(camera) = serde_json::from_str::<trinity_ram::Camera>(viewport_json) {
                            let mut state = rule_state(&envelope);
                            if let Some(mut fixture) = parse_fixture_json(&state.before_fixture_json) {
                                fixture.camera = camera;
                                if let Ok(json) = Graph::from_fixture(fixture).and_then(|graph| graph.fixture_json()) {
                                    state.before_fixture_json = json;
                                    if apply_rule_state(&mut envelope, state) {
                                        return vec![set_document_op(&envelope)];
                                    }
                                }
                            }
                        }
                    }
                }
            }
            "nodeGraphEdit" => {
                let surface_id = args.and_then(|value| value.get("surfaceId")).and_then(|value| value.as_str()).unwrap_or("");
                let ops = args
                    .and_then(|value| value.get("ops"))
                    .and_then(|value| value.as_array())
                    .cloned()
                    .unwrap_or_default();
                if apply_rewrite_node_graph_edit_ops(&mut envelope, surface_id, &ops) {
                    return vec![set_document_op(&envelope)];
                }
            }
            "setLhsJson" => {
                if let Some(value) = args.and_then(|v| v.get("value")).and_then(|v| v.as_str()) {
                    let mut state = rule_state(&envelope);
                    state.lhs_json = value.into();
                    if apply_rule_state(&mut envelope, state) {
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "setRhsJson" => {
                if let Some(value) = args.and_then(|v| v.get("value")).and_then(|v| v.as_str()) {
                    let mut state = rule_state(&envelope);
                    state.rhs_json = value.into();
                    state.parameter_bindings = default_parameter_bindings(&state.rhs_json);
                    if apply_rule_state(&mut envelope, state) {
                        return vec![set_document_op(&envelope)];
                    }
                }
            }
            "setParameter" => {
                let name = args.and_then(|v| v.get("name")).and_then(|v| v.as_str()).unwrap_or("");
                let value = args.and_then(|v| v.get("value")).and_then(|v| v.as_str()).unwrap_or("");
                if !name.is_empty() {
                    let mut state = rule_state(&envelope);
                    let Ok(rhs) = serde_json::from_str::<Rhs>(&state.rhs_json) else {
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
                        state.parameter_bindings.insert(name.into(), parsed);
                        if apply_rule_state(&mut envelope, state) {
                            return vec![set_document_op(&envelope)];
                        }
                    }
                }
            }
            "addRuleClause" => {
                let kind = args.and_then(|v| v.get("kind")).and_then(|v| v.as_str()).unwrap_or("");
                let mut state = rule_state(&envelope);
                if add_rule_clause(&mut state, kind) && apply_rule_state(&mut envelope, state) {
                    return vec![set_document_op(&envelope)];
                }
            }
            "recomputeRewrite" | "reorganize" => {
                envelope.runtime.reorganize_epoch += 1;
                return vec![set_document_op(&envelope)];
            }
            "resetRule" => {
                if apply_rule_state(&mut envelope, default_rule_state()) {
                    return vec![set_document_op(&envelope)];
                }
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
                    let mut state = rule_state(&envelope);
                    if let Some(next) = patch_fixture_nodes(&state.before_fixture_json, &node_ids, field, value) {
                        state.before_fixture_json = next;
                        if apply_rule_state(&mut envelope, state) {
                            return vec![set_document_op(&envelope)];
                        }
                    }
                }
            }
            "textSelect" => {
                if let Some(var) = args.and_then(|v| v.get("var")).and_then(|v| v.as_str()) {
                    envelope.runtime.active_select_var = var.into();
                } else if let Some(start) = args.and_then(|v| v.get("start")).and_then(|v| v.as_u64()) {
                    if let Some(token) = jack_token_at_offset(&compiled_jack_query(&rule_state(&envelope)), start as usize) {
                        envelope.runtime.active_select_var = token;
                    }
                }
                envelope.runtime.select_epoch += 1;
                return vec![set_document_op(&envelope)];
            }
            "textHover" => {
                if let Some(var) = args.and_then(|v| v.get("var")).and_then(|v| v.as_str()) {
                    envelope.runtime.active_hover_var = var.into();
                } else if let Some(offset) = args.and_then(|v| v.get("offset")).and_then(|v| v.as_u64()) {
                    if let Some(token) = jack_token_at_offset(&compiled_jack_query(&rule_state(&envelope)), offset as usize) {
                        envelope.runtime.active_hover_var = token;
                    }
                }
                envelope.runtime.hover_epoch += 1;
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
            "undo" => {
                let mut store = rule_store_from_envelope(&envelope);
                if store.dispatch(DocumentVcsCommand::Undo).is_ok() {
                    sync_envelope_from_rule_store(&mut envelope, &store);
                    return vec![set_document_op(&envelope)];
                }
            }
            "redo" => {
                let mut store = rule_store_from_envelope(&envelope);
                if store.dispatch(DocumentVcsCommand::Redo).is_ok() {
                    sync_envelope_from_rule_store(&mut envelope, &store);
                    return vec![set_document_op(&envelope)];
                }
            }
            "commitCheckpoint" => {
                let mut store = rule_store_from_envelope(&envelope);
                if store
                    .dispatch(DocumentVcsCommand::CommitCheckpoint {
                        message: args.and_then(|v| v.get("message")).and_then(|v| v.as_str()).map(str::to_string),
                        authors: Vec::new(),
                    })
                    .is_ok()
                {
                    sync_envelope_from_rule_store(&mut envelope, &store);
                    return vec![set_document_op(&envelope)];
                }
            }
            _ => {}
        }
        Vec::new()
    }

    fn render(&self, body_key: &str, document_json: &str, _view_state: &ViewState) -> UiNode {
        let envelope = parse_envelope(document_json);
        let state = rule_state(&envelope);
        match body_key {
            TRINITY_REWRITE_PLAY_BODY_BEFORE => render_fixture_graph(
                TRINITY_REWRITE_PLAY_SURFACE_BEFORE,
                TRINITY_REWRITE_PLAY_WINDOW_BEFORE,
                &state.before_fixture_json,
                &envelope,
                true,
            ),
            TRINITY_REWRITE_PLAY_BODY_AFTER => render_fixture_graph(
                TRINITY_REWRITE_PLAY_SURFACE_AFTER,
                TRINITY_REWRITE_PLAY_WINDOW_AFTER,
                &after_fixture_json(&state),
                &envelope,
                false,
            ),
            TRINITY_REWRITE_PLAY_BODY_LHS => render_fixture_graph(
                TRINITY_REWRITE_PLAY_SURFACE_LHS,
                TRINITY_REWRITE_PLAY_WINDOW_LHS,
                &lhs_graph_fixture_json(&state.lhs_json, &state.rule_layout),
                &envelope,
                true,
            ),
            TRINITY_REWRITE_PLAY_BODY_RHS => render_fixture_graph(
                TRINITY_REWRITE_PLAY_SURFACE_RHS,
                TRINITY_REWRITE_PLAY_WINDOW_RHS,
                &rhs_graph_fixture_json(&state.rhs_json, &state.rule_layout),
                &envelope,
                true,
            ),
            TRINITY_REWRITE_PLAY_BODY_JACK => render_jack_editor(&envelope),
            TRINITY_REWRITE_PLAY_BODY_PARAMETERS => build_parameters_panel(&envelope),
            TRINITY_REWRITE_PLAY_BODY_DOCUMENT => build_document_tree(&envelope),
            TRINITY_REWRITE_PLAY_BODY_CATALOGUE => build_catalogue_tree(),
            TRINITY_REWRITE_PLAY_BODY_INSPECTION => build_inspector_tree(&envelope),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn tools(&self, _document_json: &str, _view_state: &ViewState) -> Vec<ToolNode> {
        vec![
            tool_collection(
                "trinity-rewrite-history",
                "clock",
                "History",
                vec![
                    tool_button("trinity-rewrite-undo", "undo-2", "Undo", rewrite_action("undo", None)),
                    tool_button("trinity-rewrite-redo", "redo-2", "Redo", rewrite_action("redo", None)),
                    tool_button("trinity-rewrite-checkpoint", "git-commit", "Checkpoint", rewrite_action("commitCheckpoint", None)),
                ],
            ),
            tool_collection(
                "trinity-rewrite-rule",
                "code",
                "Rule",
                vec![tool_button("trinity-rewrite-reorganize", "rotate-cw", "Reorganize", rewrite_action("reorganize", None))],
            ),
        ]
    }

    fn window_measures(&self, document_json: &str, _view_state: &ViewState) -> std::collections::HashMap<String, Vec<WindowMeasure>> {
        let envelope = parse_envelope(document_json);
        let mode_for = |window_id: &str| envelope.runtime.lod_mode_by_window.get(window_id).map(String::as_str).unwrap_or(TRINITY_LOD_MODE_AUTOMATIC);
        std::collections::HashMap::from([
            (TRINITY_REWRITE_PLAY_WINDOW_BEFORE.to_string(), vec![trinity_rewrite_lod_measure(TRINITY_REWRITE_PLAY_WINDOW_BEFORE, mode_for(TRINITY_REWRITE_PLAY_WINDOW_BEFORE))]),
            (TRINITY_REWRITE_PLAY_WINDOW_AFTER.to_string(), vec![trinity_rewrite_lod_measure(TRINITY_REWRITE_PLAY_WINDOW_AFTER, mode_for(TRINITY_REWRITE_PLAY_WINDOW_AFTER))]),
            (TRINITY_REWRITE_PLAY_WINDOW_LHS.to_string(), vec![trinity_rewrite_lod_measure(TRINITY_REWRITE_PLAY_WINDOW_LHS, mode_for(TRINITY_REWRITE_PLAY_WINDOW_LHS))]),
            (TRINITY_REWRITE_PLAY_WINDOW_RHS.to_string(), vec![trinity_rewrite_lod_measure(TRINITY_REWRITE_PLAY_WINDOW_RHS, mode_for(TRINITY_REWRITE_PLAY_WINDOW_RHS))]),
        ])
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
            .window_kind(TRINITY_REWRITE_PLAY_WINDOW_BEFORE, "Before", TRINITY_REWRITE_PLAY_BODY_BEFORE, SurfaceKind::NodeGraph)
            .window_kind(TRINITY_REWRITE_PLAY_WINDOW_AFTER, "After", TRINITY_REWRITE_PLAY_BODY_AFTER, SurfaceKind::NodeGraph)
            .window_kind(TRINITY_REWRITE_PLAY_WINDOW_LHS, "LHS", TRINITY_REWRITE_PLAY_BODY_LHS, SurfaceKind::NodeGraph)
            .window_kind(TRINITY_REWRITE_PLAY_WINDOW_RHS, "RHS", TRINITY_REWRITE_PLAY_BODY_RHS, SurfaceKind::NodeGraph)
            .window_kind(TRINITY_REWRITE_PLAY_WINDOW_JACK, "Jack", TRINITY_REWRITE_PLAY_BODY_JACK, SurfaceKind::TextEditor)
            .window_kind(
                TRINITY_REWRITE_PLAY_WINDOW_PARAMETERS,
                "Parameters",
                TRINITY_REWRITE_PLAY_BODY_PARAMETERS,
                SurfaceKind::Canvas2d,
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
            )
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .keybinding("mod+alt+s", "commitCheckpoint"),
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
        let state = default_rule_state();
        let query = compiled_jack_query(&state);
        assert!(query.contains("MATCH"));
        assert!(query.contains("SET"));
    }

    #[test]
    fn apply_rewrite_changes_after_fixture() {
        let state = default_rule_state();
        assert_ne!(state.before_fixture_json, after_fixture_json(&state));
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
        assert!(lhs_json.contains("\"editable\":true"));
        assert!(rhs_json.contains("\"editable\":true"));
    }

    fn apply_and_get_envelope(app: &mut TrinityRewritePlayApp, action: &str, args: Option<&Value>, document: &str) -> TrinityRewriteEnvelope {
        let ops = app.handle_action_patch_ops(action, args, document, &ViewState::default());
        let next = ops.first().cloned().and_then(|op| serde_json::from_str::<Value>(&op).ok()).and_then(|value| value.get("document").cloned()).expect("document op");
        serde_json::from_value(next).expect("envelope")
    }

    #[test]
    fn set_parameter_is_undoable() {
        let mut app = TrinityRewritePlayApp;
        let document = app.initial_document_json();
        let after_set = apply_and_get_envelope(&mut app, "setParameter", Some(&json!({ "name": "label", "value": "changed" })), &document);
        let set_json = serde_json::to_string(&after_set).unwrap();
        let after_undo = apply_and_get_envelope(&mut app, "undo", None, &set_json);
        let restored_state = rule_state(&after_undo);
        assert_eq!(restored_state.parameter_bindings.get("label").cloned(), Some(PropertyValue::String("nakagin-core".into())));
    }

    #[test]
    fn commit_checkpoint_records_change_and_stays_undoable() {
        let mut app = TrinityRewritePlayApp;
        let document = app.initial_document_json();
        let after_set = apply_and_get_envelope(&mut app, "setParameter", Some(&json!({ "name": "label", "value": "changed" })), &document);
        let set_json = serde_json::to_string(&after_set).unwrap();
        let after_checkpoint = apply_and_get_envelope(&mut app, "commitCheckpoint", None, &set_json);
        assert!(!after_checkpoint.rule_vcs.vcs.checkpoints.is_empty(), "checkpoint should be recorded");
        let checkpoint_json = serde_json::to_string(&after_checkpoint).unwrap();
        let undone_envelope = apply_and_get_envelope(&mut app, "undo", None, &checkpoint_json);
        assert_eq!(rule_state(&undone_envelope).parameter_bindings.get("label").cloned(), Some(PropertyValue::String("nakagin-core".into())));
    }

    #[test]
    fn add_and_delete_rhs_set_clause() {
        let mut app = TrinityRewritePlayApp;
        let document = app.initial_document_json();
        let after_add = apply_and_get_envelope(&mut app, "addRuleClause", Some(&json!({ "kind": "set" })), &document);
        let rhs: Rhs = serde_json::from_str(&rule_state(&after_add).rhs_json).unwrap();
        assert_eq!(rhs.set.len(), 2);
        // deleteSelection requires a prior selection; select the newly added clause first.
        let mut selected = after_add.clone();
        selected.runtime.selected_node_ids = vec!["rhs-set-1".into()];
        let selected_json = serde_json::to_string(&selected).unwrap();
        let ops = app.handle_action_patch_ops(
            "nodeGraphEdit",
            Some(&json!({ "surfaceId": TRINITY_REWRITE_PLAY_SURFACE_RHS, "ops": [{ "op": "deleteSelection" }] })),
            &selected_json,
            &ViewState::default(),
        );
        assert!(!ops.is_empty());
        let next = ops.first().and_then(|op| serde_json::from_str::<Value>(op).ok()).and_then(|value| value.get("document").cloned()).expect("document op");
        let envelope: TrinityRewriteEnvelope = serde_json::from_value(next).unwrap();
        let rhs: Rhs = serde_json::from_str(&rule_state(&envelope).rhs_json).unwrap();
        assert_eq!(rhs.set.len(), 1);
    }

    #[test]
    fn jack_view_has_occurrences_after_select() {
        let mut app = TrinityRewritePlayApp;
        let document = app.initial_document_json();
        let after_select = apply_and_get_envelope(&mut app, "textSelect", Some(&json!({ "var": "a" })), &document);
        let selected_json = serde_json::to_string(&after_select).unwrap();
        let node = app.render(TRINITY_REWRITE_PLAY_BODY_JACK, &selected_json, &ViewState::default());
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("occurrencesJson"));
    }

    #[test]
    fn graph_scenes_have_lod_json() {
        let app = TrinityRewritePlayApp;
        let document = app.initial_document_json();
        let before = app.render(TRINITY_REWRITE_PLAY_BODY_BEFORE, &document, &ViewState::default());
        let json = serde_json::to_string(&before).unwrap();
        assert!(json.contains("lodJson"));
    }

    #[test]
    fn tools_include_history_and_reorganize() {
        let app = TrinityRewritePlayApp;
        let document = app.initial_document_json();
        let tools = app.tools(&document, &ViewState::default());
        let json = serde_json::to_string(&tools).unwrap();
        assert!(json.contains("undo"));
        assert!(json.contains("reorganize"));
    }
}
//#endregion 🧪Tests
