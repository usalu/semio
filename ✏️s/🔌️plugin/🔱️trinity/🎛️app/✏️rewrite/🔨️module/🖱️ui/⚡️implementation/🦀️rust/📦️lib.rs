//! ♻️ Trinity Rewrite plugin — parametric rewrite play app bundled as a hot-swappable WASM plugin.

use semio_framework_plugin::{SurfaceKind, PanelGroup,
    app_labels, build_node_graph_scene, build_text_editor_scene, text_identifier_bounds_at,
    is_de_locale, localized_label_map, resolve_labels, tree_item, tree_item_with_action,
    ui_declarative_sections_to_tree, ui_inspector_groups_to_tree,
    ui_inspector_mixed_text, ui_inspector_readonly_field, ui_text, ActionArgDef, ActionArgOption, ActionDefinition, ActionEmit, ActionKind, App, AppActionRegistry, ActionDescriptor, AppLabelsOverlay, AppLabelsOverlayExt, ContextMenuItemSpec, ContextMenuRequest,
    DocumentApp, DocumentView, MeasureSelectItem, NodeGraphScene, PanelTreeBuilder,
    TextEditorScene, UiFieldNode, UiInspectorFieldGroup, UiNode, UiPresence, UiSectionNode, UiTreeItemNode,
    ViewState, WindowLayout, WindowLayoutAxisNode, WindowLayoutChild, WindowLayoutRoot,
    WindowLayoutStackNode, WindowLayoutWindowNode, WindowMeasure, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
    FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, UI_INSPECTOR_MIXED_PLACEHOLDER,
};
use serde::Serialize;
use serde_json::{json, Value};
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use trinity_jack::semantic_tokens;
use trinity_ram::{Camera, Graph, GraphFixture, Node, PortDirection, PropertyValue};
use rewrite::{LayoutPoint, RewriteRuleState, REWRITE_RULE_SCHEMA};
use rewrite_engine::{
    apply_rule, build_rule_query, rule_query_json, trinity_lod_scale_json,
    AssignmentJson, Lhs, ParameterKind, ParameterSpec, Rhs, Rule, PatternJson,
};
use rewrite_op::RewriteRuleOperation;
use store::DocumentDsl;

//#region 🔖️Constants
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

const NAKAGIN_FIXTURE_DSL: &str = include_str!("../../../../../../../../../✏️s/🔌️plugin/🔱️trinity/📚️example/🔱️nakagin-capsule-tower.trinity");

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
//#endregion 🔖️Constants

//#region 🔖️Types
/// 🎛️ Ephemeral view state (selection, hover/select var focus, epochs, LOD) — lives on the app
/// struct, never in the document. The document projection is the {@link RewriteRuleState}.
#[derive(Clone, Debug, Default, PartialEq)]
struct RewritePlayRuntime {
    selected_node_ids: Vec<String>,
    /// 📷️ Live viewport pan/zoom of the Before pane — session-only (never a VCS operation); seeded
    /// once from the initial before-fixture's seed-only `camera` field, then only ever written by
    /// `nodeGraphViewport`. The before-pane's render composes it over `before_fixture_json` at
    /// render time instead of round-tripping it through the document.
    before_pane_camera: Camera,
    reorganize_epoch: u64,
    active_hover_var: String,
    hover_epoch: u64,
    active_select_var: String,
    select_epoch: u64,
    lod_mode_by_window: BTreeMap<String, String>,
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
//#endregion 🔖️Types

//#region 🔖️DocumentHelpers
/// 📦️ JSON text of the bundled Nakagin fixture — `RewriteRuleState`'s own `_json` fields keep their
/// JSON contract (see `patch_fixture_nodes`/`parse_fixture_json`), so the `.trinity` DSL source is
/// parsed once and re-serialized here rather than propagating DSL text into those fields.
fn nakagin_fixture_json() -> String {
    GraphFixture::parse_dsl(NAKAGIN_FIXTURE_DSL).expect("bundled nakagin fixture parses").to_json().expect("fixture serializes")
}

fn default_rule_state() -> RewriteRuleState {
    let mut state = RewriteRuleState {
        before_fixture_json: nakagin_fixture_json(),
        lhs_json: DEFAULT_LHS_JSON.into(),
        rhs_json: DEFAULT_RHS_JSON.into(),
        parameter_bindings: BTreeMap::new(),
        rule_layout: BTreeMap::new(),
    };
    state.parameter_bindings = default_parameter_bindings(&state.rhs_json);
    state
}

/// 🌱️ Reads `RewriteRuleState.before_fixture_json`'s seed-only `camera` field once — the one place a
/// before-fixture's initial framing is consumed into the app's live runtime camera.
fn seed_before_pane_camera(state: &RewriteRuleState) -> Camera {
    parse_fixture_json(&state.before_fixture_json).map(|fixture| fixture.camera).unwrap_or_default()
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

fn default_parameter_bindings(rhs_json: &str) -> BTreeMap<String, PropertyValue> {
    let Ok(rhs) = serde_json::from_str::<Rhs>(rhs_json) else {
        return BTreeMap::new();
    };
    rhs.parameters
        .iter()
        .map(|param| (param.name.clone(), param.default.clone()))
        .collect()
}

/// 📤️ Emits a `SetState` operation iff `next` differs from `current` (mirrors the store's LWW no-operation guard),
/// so view-neutral re-computations don't record empty history entries.
fn set_state_emit(current: &RewriteRuleState, next: RewriteRuleState) -> ActionEmit<RewriteRuleOperation> {
    if &next == current {
        ActionEmit::default()
    } else {
        ActionEmit::operations(vec![RewriteRuleOperation::SetState { state: next }])
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

/// 🎯️ Selection ids from an action's args — delegates to the SDK's shared `ids`-array reader, falling
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

fn sync_select_var_from_node(runtime: &mut RewritePlayRuntime, fixture_json: &str, node_id: &str) {
    if let Some(fixture) = parse_fixture_json(fixture_json) {
        if let Some(node) = fixture.nodes.iter().find(|node| node.id == node_id) {
            if let Some(var) = var_from_node_name(&node.name) {
                runtime.active_select_var = var;
            }
        }
    }
}

fn sync_hover_var_from_node(runtime: &mut RewritePlayRuntime, fixture_json: &str, node_id: &str) {
    if let Some(fixture) = parse_fixture_json(fixture_json) {
        if let Some(node) = fixture.nodes.iter().find(|node| node.id == node_id) {
            if let Some(var) = var_from_node_name(&node.name) {
                runtime.active_hover_var = var;
            }
        }
    }
    runtime.hover_epoch += 1;
}

/// 🧭️ Resolves which fixture backs a given rewrite graph surface (Before/After/LHS/RHS), for hover/select var lookup.
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

fn apply_semantic_layout_edit(rule_layout: &mut BTreeMap<String, LayoutPoint>, current_fixture_json: &str, edited_fixture_json: &str) -> bool {
    let (Some(current), Some(edited)) = (parse_fixture_json(current_fixture_json), parse_fixture_json(edited_fixture_json)) else {
        return false;
    };
    let mut changed = false;
    for node in &edited.nodes {
        let Some(prev) = current.nodes.iter().find(|entry| entry.id == node.id) else {
            continue;
        };
        if (prev.x - node.x).abs() > 1e-6 || (prev.y - node.y).abs() > 1e-6 {
            rule_layout.insert(node.id.clone(), LayoutPoint { x: node.x, y: node.y });
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

/// ➕️ Appends a default instance of `clause_kind` to the rule (rewrite.where/create/merge/set/delete/parameter).
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

/// 🖊️ Applies node-graph editor operations (drag layout / delete-selection) in place to `state`, returning
/// whether anything changed; the caller wraps the result in a `SetState` operation.
fn apply_rewrite_node_graph_edit_operations(state: &mut RewriteRuleState, runtime: &mut RewritePlayRuntime, surface_id: &str, operations: &[Value]) -> bool {
    let mut changed = false;
    for operation in operations {
        match operation.get("operation").and_then(|value| value.as_str()).unwrap_or("") {
            "setFixture" => {
                let Some(fixture_json) = operation.get("fixtureJson").and_then(|value| value.as_str()) else {
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
                if runtime.selected_node_ids.is_empty() {
                    continue;
                }
                if surface_id == TRINITY_REWRITE_PLAY_SURFACE_BEFORE {
                    let ids = runtime.selected_node_ids.clone();
                    if let Some(mut fixture) = parse_fixture_json(&state.before_fixture_json) {
                        fixture.nodes.retain(|node| !ids.contains(&node.id));
                        fixture.edges.retain(|edge| {
                            let from = trinity_ram::port_node_id(&edge.source).unwrap_or(&edge.source);
                            let to = trinity_ram::port_node_id(&edge.target).unwrap_or(&edge.target);
                            !ids.iter().any(|id| id == from || id == to)
                        });
                        if let Ok(json) = Graph::from_fixture(fixture).and_then(|graph| graph.fixture_json()) {
                            state.before_fixture_json = json;
                            runtime.selected_node_ids.clear();
                            changed = true;
                        }
                    }
                } else if surface_id == TRINITY_REWRITE_PLAY_SURFACE_LHS || surface_id == TRINITY_REWRITE_PLAY_SURFACE_RHS {
                    let ids = runtime.selected_node_ids.clone();
                    let mut deleted = false;
                    for id in &ids {
                        deleted |= delete_rule_clause(state, id);
                    }
                    if deleted {
                        runtime.selected_node_ids.clear();
                        changed = true;
                    }
                }
            }
            _ => {}
        }
    }
    changed
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

fn semantic_rule_node(id: &str, kind: &str, name: &str, x: f64, y: f64, rule_layout: &BTreeMap<String, LayoutPoint>) -> Node {
    let (x, y) = rule_layout.get(id).map(|point| (point.x, point.y)).unwrap_or((x, y));
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

fn lhs_semantic_graph_fixture(lhs: &Lhs, rule_layout: &BTreeMap<String, LayoutPoint>) -> GraphFixture {
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
            source: "lhs-match@out".into(),
            target: "lhs-where@in".into(),
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

fn rhs_semantic_graph_fixture(rhs: &Rhs, rule_layout: &BTreeMap<String, LayoutPoint>) -> GraphFixture {
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

fn lhs_graph_fixture_json(lhs_json: &str, rule_layout: &BTreeMap<String, LayoutPoint>) -> String {
    let Ok(lhs) = serde_json::from_str::<Lhs>(lhs_json) else {
        return nakagin_fixture_json();
    };
    Graph::from_fixture(lhs_semantic_graph_fixture(&lhs, rule_layout))
        .ok()
        .and_then(|graph| graph.fixture_json().ok())
        .unwrap_or_else(nakagin_fixture_json)
}

fn rhs_graph_fixture_json(rhs_json: &str, rule_layout: &BTreeMap<String, LayoutPoint>) -> String {
    let Ok(rhs) = serde_json::from_str::<Rhs>(rhs_json) else {
        return nakagin_fixture_json();
    };
    Graph::from_fixture(rhs_semantic_graph_fixture(&rhs, rule_layout))
        .ok()
        .and_then(|graph| graph.fixture_json().ok())
        .unwrap_or_else(nakagin_fixture_json)
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
/// 🩹️ Delegates to `trinity_ram::parse_port_key` (the one place the `nodeId@portId` convention is
/// owned) instead of hand-rolling a second splitter here.
fn split_endpoint(endpoint: &str) -> (String, String) {
    trinity_ram::parse_port_key(endpoint).map_or_else(|| (endpoint.to_string(), "in".into()), |(n, p)| (n.to_string(), p.to_string()))
}

fn fixture_to_workflow(fixture: &GraphFixture) -> (String, String, String) {
    let nodes: Vec<WorkflowNodeRecord> = fixture.nodes.iter().map(node_to_workflow_record).collect();
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
//#endregion 🔖️DocumentHelpers

//#region 🔖️Terminology
/// 🗣️ Complete UI label set for the Rewrite rule app; one field per label makes every locale combination compile-checked.
app_labels! {
    struct TrinityRewriteLabels {
        pieces: &'static str = en: "Pieces", de: "Stücke";
        piece: &'static str = en: "Piece", de: "Stück";
        connection: &'static str = en: "Connection", de: "Verbindung";
        connector: &'static str = en: "Connector", de: "Verbinder";
        catalogue: &'static str = en: "Catalogue", de: "Katalog";
        add_to_lhs: &'static str = en: "Add to LHS", de: "Zu LHS hinzufügen";
        add_to_rhs: &'static str = en: "Add to RHS", de: "Zu RHS hinzufügen";
        parameters: &'static str = en: "Parameters", de: "Parameter";
        geometry: &'static str = en: "Geometry", de: "Geometrie";
        identity: &'static str = en: "Identity", de: "Identität";
        history: &'static str = en: "History", de: "Verlauf";
        rule: &'static str = en: "Rule", de: "Regel";
        window_before: &'static str = en: "Before", de: "Vorher";
        window_after: &'static str = en: "After", de: "Nachher";
        window_lhs: &'static str = en: "LHS", de: "LHS";
        window_rhs: &'static str = en: "RHS", de: "RHS";
        window_jack: &'static str = en: "Jack", de: "Jack";
        window_parameters: &'static str = en: "Parameters", de: "Parameter";
    }
}
//#endregion 🔖️Terminology

//#region 🔖️CommandLabels
/// 🗣️ (action id) -> localized label for every operation/view-action declared in `create_rewrite_app`'s static
/// manifest — the manifest itself has no `view_state`/locale parameter, so this overlay is how the command palette
/// and Actions rail get a translated label without threading locale through the whole builder chain.
fn trinity_rewrite_action_labels(is_de: bool) -> HashMap<String, String> {
    localized_label_map(is_de, &[
        ("addRuleClause", "Add Rule Clause", "Regelklausel hinzufügen"),
        ("resetRule", "Reset Rule", "Regel zurücksetzen"),
        ("setParameter", "Set Parameter", "Parameter festlegen"),
        ("patchTrinityNodes", "Patch Nodes", "Knoten aktualisieren"),
        ("nodeGraphEdit", "Edit Graph", "Graph bearbeiten"),
        ("nodeGraphViewport", "Set Graph Viewport", "Graph-Ansicht festlegen"),
        ("setLhsJson", "Set LHS Json", "LHS-JSON festlegen"),
        ("setRhsJson", "Set RHS Json", "RHS-JSON festlegen"),
        ("setSelection", "Set Selection", "Auswahl festlegen"),
        ("selectNode", "Select Node", "Knoten auswählen"),
        ("nodeGraphSelect", "Select Graph Node", "Graph-Knoten auswählen"),
        ("nodeGraphHover", "Hover Graph Node", "Graph-Knoten hovern"),
        ("graphPointerDown", "Graph Pointer Down", "Graph-Zeiger gedrückt"),
        ("textSelect", "Select Text", "Text auswählen"),
        ("textHover", "Hover Text", "Text hovern"),
        ("recomputeRewrite", "Recompute Rewrite", "Rewrite neu berechnen"),
        ("reorganize", "Reorganize", "Neu anordnen"),
        ("setLodMode", "Set LOD Mode", "LOD-Modus festlegen"),
    ])
}
//#endregion 🔖️CommandLabels

//#region 🔖️Panels
fn build_document_tree(state: &RewriteRuleState, runtime: &RewritePlayRuntime, labels: &TrinityRewriteLabels) -> UiNode {
    let Some(fixture) = parse_fixture_json(&state.before_fixture_json) else {
        return ui_text("Invalid trinity fixture");
    };
    let builder = PanelTreeBuilder::new("trinity-document");
    let node_items: Vec<UiTreeItemNode> = fixture
        .nodes
        .iter()
        .map(|node| {
            tree_item_with_action(
                builder.item_id("node", &node.id),
                if node.name.is_empty() { node.id.clone() } else { node.name.clone() },
                Some(node.kind.clone()),
                rewrite_action("setSelection", Some(json!({ "ids": [node.id] }))),
            )
        })
        .collect();
    let selected = runtime.selected_node_ids.iter().map(|id| builder.item_id("node", id)).collect();
    builder
        .section("trinity-document.nodes", Some(labels.pieces.into()), true, node_items)
        .selected(selected)
        .selection_change(rewrite_action("setSelection", Some(json!({ "ids": [] }))))
        .build()
}

fn catalogue_add_item(id: &str, label: &str, clause_kind: &str) -> UiTreeItemNode {
    UiTreeItemNode {
        ..tree_item_with_action(id, label, None, rewrite_action("addRuleClause", Some(json!({ "kind": clause_kind }))))
    }
}

fn build_catalogue_tree(labels: &TrinityRewriteLabels) -> UiNode {
    PanelTreeBuilder::new("trinity-catalogue")
        .section(
            "trinity-catalogue.kinds",
            Some(labels.catalogue.into()),
            true,
            vec![
                tree_item("trinity-catalogue.piece", labels.piece),
                tree_item("trinity-catalogue.connection", labels.connection),
                tree_item("trinity-catalogue.connector", labels.connector),
            ],
        )
        .section(
            "trinity-catalogue.lhs",
            Some(labels.add_to_lhs.into()),
            true,
            vec![catalogue_add_item("trinity-catalogue.add-where", "Where clause", "where")],
        )
        .section(
            "trinity-catalogue.rhs",
            Some(labels.add_to_rhs.into()),
            true,
            vec![
                catalogue_add_item("trinity-catalogue.add-create", "Create pattern", "create"),
                catalogue_add_item("trinity-catalogue.add-merge", "Merge pattern", "merge"),
                catalogue_add_item("trinity-catalogue.add-set", "Set assignment", "set"),
                catalogue_add_item("trinity-catalogue.add-delete", "Delete pattern", "delete"),
                catalogue_add_item("trinity-catalogue.add-parameter", "Parameter", "parameter"),
            ],
        )
        .selected(vec![])
        .build()
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

fn build_inspector_tree(state: &RewriteRuleState, runtime: &RewritePlayRuntime, term_labels: &TrinityRewriteLabels) -> UiNode {
    let Some(fixture) = parse_fixture_json(&state.before_fixture_json) else {
        return ui_declarative_sections_to_tree(&[semio_framework_plugin::UiSectionNode {
            id: "trinity-inspector.empty".into(),
            label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
            default_open: Some(true),
            children: vec![ui_text("Invalid trinity fixture")],
            presence: UiPresence::default(),
            menu: None,
        }]);
    };
    if runtime.selected_node_ids.is_empty() {
        return ui_declarative_sections_to_tree(&[UiSectionNode {
            id: "trinity-inspector.empty".into(),
            label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
            default_open: Some(true),
            presence: UiPresence::default(),
            children: vec![ui_text("Select one or more pieces")],
            menu: None,
        }]);
    }
    let nodes: Vec<&Node> = runtime
        .selected_node_ids
        .iter()
        .filter_map(|id| fixture.nodes.iter().find(|node| &node.id == id))
        .collect();
    if nodes.is_empty() {
        return ui_declarative_sections_to_tree(&[semio_framework_plugin::UiSectionNode {
            id: "trinity-inspector.empty".into(),
            label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
            default_open: Some(true),
            children: vec![ui_text("Piece not found")],
            presence: UiPresence::default(),
            menu: None,
        }]);
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
        UiInspectorFieldGroup { presence: UiPresence::default(),
            id: "trinity-inspector.geometry".into(),
            label: term_labels.geometry.into(),
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
                        on_change: rewrite_action("patchTrinityNodes", Some(json!({ "nodeIds": node_ids, "field": "name" }))),
                        min: None,
                        max: None,
                        step: None,
                        accept: None,
                        menu: None,
                    })),
                    description: None,
                    required: None,
                    error: None,
                    menu: None,
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

fn build_parameters_panel(state: &RewriteRuleState, labels: &TrinityRewriteLabels) -> UiNode {
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
        children.push(semio_framework_plugin::UiNode::Field(UiFieldNode {presence: UiPresence::default(), 
            id: format!("trinity-rewrite.param.{}", param.name),
            label: param.name.clone(),
            child: Box::new(semio_framework_plugin::UiNode::Input(semio_framework_plugin::UiInputNode {presence: UiPresence::default(), 
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
                menu: None,
            })),
            description: None,
            required: None,
            error: None,
            menu: None,
        }));
    }
    if children.is_empty() {
        children.push(ui_text("No parameters declared on RHS."));
    }
    ui_declarative_sections_to_tree(&[UiSectionNode {
        id: "trinity-rewrite.parameters".into(),
        label: Some(labels.parameters.into()),
        default_open: Some(true),
        presence: UiPresence::default(),
        children,
        menu: None,
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
//#endregion 🔖️Panels

//#region 🔖️Render
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
    runtime: &RewritePlayRuntime,
    hover_node_id: &str,
    editable: bool,
    camera_override: Option<&Camera>,
) -> UiNode {
    let fixture = parse_fixture_json(fixture_json).unwrap_or_else(|| GraphFixture::parse_dsl(NAKAGIN_FIXTURE_DSL).unwrap());
    let (nodes_json, edges_json, fixture_viewport_json) = fixture_to_workflow(&fixture);
    let viewport_json = camera_override.map(|camera| serde_json::to_string(camera).unwrap_or_else(|_| fixture_viewport_json.clone())).unwrap_or(fixture_viewport_json);
    let hover_json = graph_hover_json(fixture_json, &runtime.active_hover_var, hover_node_id);
    let selection_json = graph_selection_json(fixture_json, &runtime.active_select_var, &runtime.selected_node_ids);
    build_node_graph_scene(
        surface_id,
        TRINITY_REWRITE_PLAY_CONTROLLER_ID,
        NodeGraphScene {
            hover_json,
            selection_json,
            lod_json: rewrite_lod_json_for_window(runtime, window_id),
            editable: editable.then_some(true),
            ..NodeGraphScene::base(nodes_json, edges_json, viewport_json)
        },
    )
}

fn render_fixture_graph(surface_id: &str, window_id: &str, fixture_json: &str, runtime: &RewritePlayRuntime, editable: bool, camera_override: Option<&Camera>) -> UiNode {
    render_rule_graph(surface_id, window_id, fixture_json, runtime, "", editable, camera_override)
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

fn render_jack_editor(state: &RewriteRuleState, runtime: &RewritePlayRuntime) -> UiNode {
    let query = compiled_jack_query(state);
    let active_var = if !runtime.active_hover_var.is_empty() {
        runtime.active_hover_var.as_str()
    } else {
        runtime.active_select_var.as_str()
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
//#endregion 🔖️Render

//#region 🔖️TrinityRewritePlayApp
/// ♻️ Trinity Rewrite play app — a parametric-rewrite editor over a {@link RewriteRuleState}
/// projection. Every rule/parameter/before-fixture mutation flows through the single LWW
/// {@link RewriteRuleOperation::SetState}; hover/select var focus, epochs and LOD are runtime.
pub struct TrinityRewritePlayApp {
    runtime: RefCell<RewritePlayRuntime>,
}

impl Default for TrinityRewritePlayApp {
    fn default() -> Self {
        Self { runtime: RefCell::new(RewritePlayRuntime { before_pane_camera: seed_before_pane_camera(&default_rule_state()), ..Default::default() }) }
    }
}

impl DocumentApp for TrinityRewritePlayApp {
    type Projection = RewriteRuleState;
    type Operation = RewriteRuleOperation;
        type Config = semio_framework_plugin::NoConfig;
        type ConfigOperation = semio_framework_plugin::NoConfigOperation;

    fn app_id(&self) -> &str {
        TRINITY_REWRITE_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        REWRITE_RULE_SCHEMA
    }

    fn initial_projection(&self) -> RewriteRuleState {
        default_rule_state()
    }

    fn handle_action(
        &self,
        action: &str,
        args: Option<&Value>,
        doc: &DocumentView<'_, RewriteRuleState>,
        _cfg: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>,
        _view_state: &ViewState,
    ) -> ActionEmit<RewriteRuleOperation> {
        let state = doc.projection;
        let mut runtime = self.runtime.borrow_mut();
        match action {
            "setSelection" | "selectNode" | "nodeGraphSelect" => {
                runtime.selected_node_ids = selection_ids(args);
                let surface_id = args.and_then(|value| value.get("surfaceId")).and_then(|value| value.as_str()).unwrap_or("");
                if let Some(node_id) = runtime.selected_node_ids.first().cloned() {
                    let fixture_json = fixture_json_for_surface(surface_id, state);
                    sync_select_var_from_node(&mut runtime, &fixture_json, &node_id);
                    runtime.select_epoch += 1;
                }
                ActionEmit::default()
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
                    let fixture_json = fixture_json_for_surface(surface_id, state);
                    sync_hover_var_from_node(&mut runtime, &fixture_json, &node_id);
                }
                ActionEmit::default()
            }
            "nodeGraphViewport" => {
                let surface_id = args.and_then(|value| value.get("surfaceId")).and_then(|value| value.as_str()).unwrap_or("");
                if surface_id == TRINITY_REWRITE_PLAY_SURFACE_BEFORE {
                    if let Some(viewport_json) = args.and_then(|value| value.get("viewportJson")).and_then(|value| value.as_str()) {
                        if let Ok(camera) = serde_json::from_str::<Camera>(viewport_json) {
                            runtime.before_pane_camera = camera;
                        }
                    }
                }
                ActionEmit::default()
            }
            "nodeGraphEdit" => {
                let surface_id = args.and_then(|value| value.get("surfaceId")).and_then(|value| value.as_str()).unwrap_or("");
                let operations = args
                    .and_then(|value| value.get("operations"))
                    .and_then(|value| value.as_array())
                    .cloned()
                    .unwrap_or_default();
                let mut next = state.clone();
                if apply_rewrite_node_graph_edit_operations(&mut next, &mut runtime, surface_id, &operations) {
                    set_state_emit(state, next)
                } else {
                    ActionEmit::default()
                }
            }
            "setLhsJson" => {
                if let Some(value) = args.and_then(|v| v.get("value")).and_then(|v| v.as_str()) {
                    let mut next = state.clone();
                    next.lhs_json = value.into();
                    return set_state_emit(state, next);
                }
                ActionEmit::default()
            }
            "setRhsJson" => {
                if let Some(value) = args.and_then(|v| v.get("value")).and_then(|v| v.as_str()) {
                    let mut next = state.clone();
                    next.rhs_json = value.into();
                    next.parameter_bindings = default_parameter_bindings(&next.rhs_json);
                    return set_state_emit(state, next);
                }
                ActionEmit::default()
            }
            "setParameter" => {
                let name = args.and_then(|v| v.get("name")).and_then(|v| v.as_str()).unwrap_or("");
                let value = args.and_then(|v| v.get("value")).and_then(|v| v.as_str()).unwrap_or("");
                if !name.is_empty() {
                    let mut next = state.clone();
                    let Ok(rhs) = serde_json::from_str::<Rhs>(&next.rhs_json) else {
                        return ActionEmit::default();
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
                        next.parameter_bindings.insert(name.into(), parsed);
                        return set_state_emit(state, next);
                    }
                }
                ActionEmit::default()
            }
            "addRuleClause" => {
                let kind = args.and_then(|v| v.get("kind")).and_then(|v| v.as_str()).unwrap_or("");
                let mut next = state.clone();
                if add_rule_clause(&mut next, kind) {
                    return set_state_emit(state, next);
                }
                ActionEmit::default()
            }
            "recomputeRewrite" | "reorganize" => {
                runtime.reorganize_epoch += 1;
                ActionEmit::default()
            }
            "resetRule" => {
                let next = default_rule_state();
                runtime.before_pane_camera = seed_before_pane_camera(&next);
                set_state_emit(state, next)
            }
            "graphPointerDown" => {
                if let Some(node_id) = args.and_then(|v| v.get("nodeId")).and_then(|v| v.as_str()) {
                    runtime.selected_node_ids = vec![node_id.into()];
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
                if !node_ids.is_empty() && !field.is_empty() && !value.is_empty() {
                    let mut next = state.clone();
                    if let Some(patched) = patch_fixture_nodes(&next.before_fixture_json, &node_ids, field, value) {
                        next.before_fixture_json = patched;
                        return set_state_emit(state, next);
                    }
                }
                ActionEmit::default()
            }
            "textSelect" => {
                if let Some(var) = args.and_then(|v| v.get("var")).and_then(|v| v.as_str()) {
                    runtime.active_select_var = var.into();
                } else if let Some(start) = args.and_then(|v| v.get("start")).and_then(|v| v.as_u64()) {
                    if let Some(token) = jack_token_at_offset(&compiled_jack_query(state), start as usize) {
                        runtime.active_select_var = token;
                    }
                }
                runtime.select_epoch += 1;
                ActionEmit::default()
            }
            "textHover" => {
                if let Some(var) = args.and_then(|v| v.get("var")).and_then(|v| v.as_str()) {
                    runtime.active_hover_var = var.into();
                } else if let Some(offset) = args.and_then(|v| v.get("offset")).and_then(|v| v.as_u64()) {
                    if let Some(token) = jack_token_at_offset(&compiled_jack_query(state), offset as usize) {
                        runtime.active_hover_var = token;
                    }
                }
                runtime.hover_epoch += 1;
                ActionEmit::default()
            }
            "setLodMode" => {
                if let (Some(window_id), Some(value)) = (
                    args.and_then(|v| v.get("windowId")).and_then(|v| v.as_str()),
                    args.and_then(|v| v.get("value")).and_then(|v| v.as_str()),
                ) {
                    runtime.lod_mode_by_window.insert(window_id.into(), value.into());
                }
                ActionEmit::default()
            }
            _ => ActionEmit::default(),
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, RewriteRuleState>, _cfg: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>, view_state: &ViewState) -> UiNode {
        let state = doc.projection;
        let runtime = &*self.runtime.borrow();
        let labels = resolve_labels::<TrinityRewriteLabels>(view_state);
        match body_key {
            TRINITY_REWRITE_PLAY_BODY_BEFORE => render_fixture_graph(
                TRINITY_REWRITE_PLAY_SURFACE_BEFORE,
                TRINITY_REWRITE_PLAY_WINDOW_BEFORE,
                &state.before_fixture_json,
                runtime,
                true,
                Some(&runtime.before_pane_camera),
            ),
            TRINITY_REWRITE_PLAY_BODY_AFTER => render_fixture_graph(
                TRINITY_REWRITE_PLAY_SURFACE_AFTER,
                TRINITY_REWRITE_PLAY_WINDOW_AFTER,
                &after_fixture_json(state),
                runtime,
                false,
                None,
            ),
            TRINITY_REWRITE_PLAY_BODY_LHS => render_fixture_graph(
                TRINITY_REWRITE_PLAY_SURFACE_LHS,
                TRINITY_REWRITE_PLAY_WINDOW_LHS,
                &lhs_graph_fixture_json(&state.lhs_json, &state.rule_layout),
                runtime,
                true,
                None,
            ),
            TRINITY_REWRITE_PLAY_BODY_RHS => render_fixture_graph(
                TRINITY_REWRITE_PLAY_SURFACE_RHS,
                TRINITY_REWRITE_PLAY_WINDOW_RHS,
                &rhs_graph_fixture_json(&state.rhs_json, &state.rule_layout),
                runtime,
                true,
                None,
            ),
            TRINITY_REWRITE_PLAY_BODY_JACK => render_jack_editor(state, runtime),
            TRINITY_REWRITE_PLAY_BODY_PARAMETERS => build_parameters_panel(state, labels),
            TRINITY_REWRITE_PLAY_BODY_DOCUMENT => build_document_tree(state, runtime, labels),
            TRINITY_REWRITE_PLAY_BODY_CATALOGUE => build_catalogue_tree(labels),
            TRINITY_REWRITE_PLAY_BODY_INSPECTION => build_inspector_tree(state, runtime, labels),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn window_measures(&self, _doc: &DocumentView<'_, RewriteRuleState>, _cfg: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>, _view_state: &ViewState) -> HashMap<String, Vec<WindowMeasure>> {
        let runtime = self.runtime.borrow();
        let mode_for = |window_id: &str| runtime.lod_mode_by_window.get(window_id).map(String::as_str).unwrap_or(TRINITY_LOD_MODE_AUTOMATIC);
        HashMap::from([
            (TRINITY_REWRITE_PLAY_WINDOW_BEFORE.to_string(), vec![trinity_rewrite_lod_measure(TRINITY_REWRITE_PLAY_WINDOW_BEFORE, mode_for(TRINITY_REWRITE_PLAY_WINDOW_BEFORE))]),
            (TRINITY_REWRITE_PLAY_WINDOW_AFTER.to_string(), vec![trinity_rewrite_lod_measure(TRINITY_REWRITE_PLAY_WINDOW_AFTER, mode_for(TRINITY_REWRITE_PLAY_WINDOW_AFTER))]),
            (TRINITY_REWRITE_PLAY_WINDOW_LHS.to_string(), vec![trinity_rewrite_lod_measure(TRINITY_REWRITE_PLAY_WINDOW_LHS, mode_for(TRINITY_REWRITE_PLAY_WINDOW_LHS))]),
            (TRINITY_REWRITE_PLAY_WINDOW_RHS.to_string(), vec![trinity_rewrite_lod_measure(TRINITY_REWRITE_PLAY_WINDOW_RHS, mode_for(TRINITY_REWRITE_PLAY_WINDOW_RHS))]),
        ])
    }

    fn app_labels(&self, view_state: &ViewState) -> AppLabelsOverlay {
        let labels = resolve_labels::<TrinityRewriteLabels>(view_state);
        AppLabelsOverlay::default()
            .window_kind_label(TRINITY_REWRITE_PLAY_WINDOW_BEFORE, labels.window_before)
            .window_kind_label(TRINITY_REWRITE_PLAY_WINDOW_AFTER, labels.window_after)
            .window_kind_label(TRINITY_REWRITE_PLAY_WINDOW_LHS, labels.window_lhs)
            .window_kind_label(TRINITY_REWRITE_PLAY_WINDOW_RHS, labels.window_rhs)
            .window_kind_label(TRINITY_REWRITE_PLAY_WINDOW_JACK, labels.window_jack)
            .window_kind_label(TRINITY_REWRITE_PLAY_WINDOW_PARAMETERS, labels.window_parameters)
            .action_labels(trinity_rewrite_action_labels(is_de_locale(view_state)))
    }

    fn context_menu(
        &self,
        request: &ContextMenuRequest,
        _doc: &DocumentView<'_, RewriteRuleState>,
        _cfg: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>,
        view_state: &ViewState,
        registry: &AppActionRegistry,
    ) -> Vec<ContextMenuItemSpec> {
        use semio_framework_plugin::{node_graph_delete_selection_spec, selection_domains_from_surface, Menu, NodeGraphDeleteDispatch};

        let is_de = is_de_locale(view_state);
        let selected = self.runtime.borrow().selected_node_ids.clone();
        let (nodes, edges) = selection_domains_from_surface(request.surface.as_ref(), &selected, &[]);
        let mut menu = Menu::of(registry);
        if let Some(spec) = node_graph_delete_selection_spec("Delete selection", is_de, nodes.len(), edges.len(), NodeGraphDeleteDispatch::ViaNodeGraphEdit) {
            menu = menu.item(spec);
        }
        menu.build()
    }
}
//#endregion 🔖️TrinityRewritePlayApp

//#region 🔖️Manifest
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
            .mode("explore", "Explore", "compass")
            .default_mode_id("explore")
            .window_kind(TRINITY_REWRITE_PLAY_WINDOW_BEFORE, "Before", TRINITY_REWRITE_PLAY_BODY_BEFORE, SurfaceKind::NodeGraph, "git-branch")
            .window_kind(TRINITY_REWRITE_PLAY_WINDOW_AFTER, "After", TRINITY_REWRITE_PLAY_BODY_AFTER, SurfaceKind::NodeGraph, "arrow-right")
            .window_kind(TRINITY_REWRITE_PLAY_WINDOW_LHS, "LHS", TRINITY_REWRITE_PLAY_BODY_LHS, SurfaceKind::NodeGraph, "trinity-lhs")
            .window_kind(TRINITY_REWRITE_PLAY_WINDOW_RHS, "RHS", TRINITY_REWRITE_PLAY_BODY_RHS, SurfaceKind::NodeGraph, "trinity-rhs")
            .window_kind(TRINITY_REWRITE_PLAY_WINDOW_JACK, "Jack", TRINITY_REWRITE_PLAY_BODY_JACK, SurfaceKind::TextEditor, "document-jack")
            .window_kind(
                TRINITY_REWRITE_PLAY_WINDOW_PARAMETERS,
                "Parameters",
                TRINITY_REWRITE_PLAY_BODY_PARAMETERS,
                SurfaceKind::Canvas2d,
                "settings-2",
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
            // ✏️ Document-mutating actions — dispatched as VCS operations with true inverses.
            .operation("addRuleClause", "Add Rule Clause")
            .operation("resetRule", "Reset Rule")
            .operation("setParameter", "Set Parameter")
            .operation("patchTrinityNodes", "Patch Nodes")
            .operation("nodeGraphEdit", "Edit Graph")
            // 🛠️ Dev-only raw rule editors — kept out of the command palette.
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog("setLhsJson", "Set LHS Json", ActionKind::Operation) })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog("setRhsJson", "Set RHS Json", ActionKind::Operation) })
            // 👁️ Ephemeral view state — selection, hover, text cursor, recompute/layout, LOD.
            .view_action("setSelection", "Set Selection")
            .view_action("selectNode", "Select Node")
            .view_action("nodeGraphSelect", "Select Graph Node")
            .view_action("nodeGraphHover", "Hover Graph Node")
            .view_action("nodeGraphViewport", "Set Graph Viewport")
            .view_action("graphPointerDown", "Graph Pointer Down")
            .view_action("textSelect", "Select Text")
            .view_action("textHover", "Hover Text")
            .view_action("recomputeRewrite", "Recompute Rewrite")
            .view_action("reorganize", "Reorganize")
            .view_action("setLodMode", "Set LOD Mode")
            // 📝️ Staged argument forms.
            .action_args("addRuleClause", vec![
                ActionArgDef::select("kind", "Clause", vec![
                    ActionArgOption::new("where", "Where"),
                    ActionArgOption::new("create", "Create"),
                    ActionArgOption::new("merge", "Merge"),
                    ActionArgOption::new("set", "Set"),
                    ActionArgOption::new("delete", "Delete"),
                    ActionArgOption::new("parameter", "Parameter"),
                ]).required(),
            ])
            .action_args("setLhsJson", vec![ActionArgDef::text("value", "LHS JSON").required()])
            .action_args("setRhsJson", vec![ActionArgDef::text("value", "RHS JSON").required()])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .keybinding("mod+alt+s", "commitCheckpoint"),
    )
    .example("label-core", "Label Core", default_rule_state().print_dsl(), "file-text")
    .workflow("trinity-rewrite", "Trinity Rewrite", "graph")
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{testkit, ActionMeta, PluginApp, VcsDocumentApp};

    fn meta(actor: &str) -> ActionMeta {
        testkit::meta(actor)
    }

    fn new_app() -> VcsDocumentApp<TrinityRewritePlayApp> {
        testkit::new_app()
    }

    fn dispatch(app: &mut VcsDocumentApp<TrinityRewritePlayApp>, action: &str, args: Option<&Value>) -> semio_framework_plugin::kernel::InvocationResult {
        app.handle_action(action, args, &ViewState::default(), &meta("local")).expect("dispatch")
    }

    #[test]
    fn renders_before_and_after_graphs() {
        let mut app = new_app();
        let before = app.render(TRINITY_REWRITE_PLAY_BODY_BEFORE, None, &ViewState::default()).expect("render");
        let after = app.render(TRINITY_REWRITE_PLAY_BODY_AFTER, None, &ViewState::default()).expect("render");
        assert!(serde_json::to_string(&before).unwrap().contains("node-graph"));
        assert!(serde_json::to_string(&after).unwrap().contains("node-graph"));
    }

    /// 🎥️ `nodeGraphViewport` on the Before pane is `ActionKind::View`: it writes the app's own
    /// runtime camera and emits zero operations (no whole-document `SetState` replace per pan tick),
    /// and the Before pane's render composes that runtime camera over `before_fixture_json` instead
    /// of round-tripping it through the document.
    #[test]
    fn node_graph_viewport_writes_before_pane_runtime_camera_without_operations() {
        let mut app = new_app();
        let before_state = app.projection().unwrap();
        let result = dispatch(
            &mut app,
            "nodeGraphViewport",
            Some(&json!({ "surfaceId": TRINITY_REWRITE_PLAY_SURFACE_BEFORE, "viewportJson": json!({ "x": 10.0, "y": 20.0, "zoom": 2.5 }).to_string() })),
        );
        assert!(result.operations.is_empty(), "camera is a view action, no operations");
        assert_eq!(app.projection().unwrap(), before_state, "document is untouched by a viewport pan");
        let before = app.render(TRINITY_REWRITE_PLAY_BODY_BEFORE, None, &ViewState::default()).expect("render");
        assert!(serde_json::to_string(&before).unwrap().contains("2.5"), "render reads the live runtime camera");
    }

    #[test]
    fn compiles_jack_query_from_rule() {
        let query = compiled_jack_query(&default_rule_state());
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
        let mut app = new_app();
        let lhs_json = serde_json::to_string(&app.render(TRINITY_REWRITE_PLAY_BODY_LHS, None, &ViewState::default()).expect("render")).unwrap();
        let rhs_json = serde_json::to_string(&app.render(TRINITY_REWRITE_PLAY_BODY_RHS, None, &ViewState::default()).expect("render")).unwrap();
        assert!(lhs_json.contains("node-graph"));
        assert!(rhs_json.contains("node-graph"));
        assert!(lhs_json.contains("\"editable\":true"));
        assert!(rhs_json.contains("\"editable\":true"));
    }

    #[test]
    fn set_parameter_emits_one_op_and_is_undoable() {
        let mut app = new_app();
        let result = dispatch(&mut app, "setParameter", Some(&json!({ "name": "label", "value": "changed" })));
        assert_eq!(result.operations.len(), 1, "a parameter edit is a single SetState operation");
        assert_eq!(app.projection().unwrap().parameter_bindings.get("label").cloned(), Some(PropertyValue::String("changed".into())));
        dispatch(&mut app, "undo", None);
        assert_eq!(app.projection().unwrap().parameter_bindings.get("label").cloned(), Some(PropertyValue::String("nakagin-core".into())));
    }

    #[test]
    fn commit_checkpoint_records_change_and_stays_undoable() {
        let mut app = new_app();
        dispatch(&mut app, "setParameter", Some(&json!({ "name": "label", "value": "changed" })));
        dispatch(&mut app, "commitCheckpoint", None);
        let files = app.document_pack().expect("document pack");
        let parsed: store::ParsedDocumentText<RewriteRuleState, RewriteRuleOperation> = store::parse_document_pack(&files.pack, &files.spr).expect("parse document pack");
        assert!(!parsed.envelope.vcs.checkpoints.is_empty(), "checkpoint should be recorded");
        dispatch(&mut app, "undo", None);
        assert_eq!(app.projection().unwrap().parameter_bindings.get("label").cloned(), Some(PropertyValue::String("nakagin-core".into())));
    }

    #[test]
    fn add_and_delete_rhs_set_clause() {
        let mut app = new_app();
        dispatch(&mut app, "addRuleClause", Some(&json!({ "kind": "set" })));
        let rhs: Rhs = serde_json::from_str(&app.projection().unwrap().rhs_json).unwrap();
        assert_eq!(rhs.set.len(), 2);
        // deleteSelection requires a prior selection; select the newly added clause first (runtime).
        dispatch(&mut app, "setSelection", Some(&json!({ "ids": ["rhs-set-1"], "surfaceId": TRINITY_REWRITE_PLAY_SURFACE_RHS })));
        let result = dispatch(&mut app, "nodeGraphEdit", Some(&json!({ "surfaceId": TRINITY_REWRITE_PLAY_SURFACE_RHS, "operations": [{ "operation": "deleteSelection" }] })));
        assert!(!result.operations.is_empty());
        let rhs: Rhs = serde_json::from_str(&app.projection().unwrap().rhs_json).unwrap();
        assert_eq!(rhs.set.len(), 1);
    }

    #[test]
    fn jack_view_has_occurrences_after_select() {
        let mut app = new_app();
        let result = dispatch(&mut app, "textSelect", Some(&json!({ "var": "a" })));
        assert!(result.operations.is_empty(), "text selection is a view action, no operations");
        let node = app.render(TRINITY_REWRITE_PLAY_BODY_JACK, None, &ViewState::default()).expect("render");
        assert!(serde_json::to_string(&node).unwrap().contains("occurrencesJson"));
    }

    #[test]
    fn graph_scenes_have_lod_json() {
        let mut app = new_app();
        let before = app.render(TRINITY_REWRITE_PLAY_BODY_BEFORE, None, &ViewState::default()).expect("render");
        assert!(serde_json::to_string(&before).unwrap().contains("lodJson"));
    }

    // 🧰️ `VcsDocumentApp::tools()` no longer exists — utility bars are now derived by the renderer
    // from the utility registry, which this app declares none of. `reorganize` is a plain view
    // action and `undo` is a framework-injected History action; both still live in the static
    // `AppDefinition.actions` list.
    #[test]
    fn app_definition_declares_reorganize_and_history_actions() {
        let definition = create_rewrite_app().definition;
        let action_ids: Vec<&str> = definition.actions.iter().map(|action| action.id.as_str()).collect();
        assert!(action_ids.contains(&"undo"));
        assert!(action_ids.contains(&"reorganize"));
    }

    #[test]
    fn trinity_rewrite_labels_resolve_native_by_default() {
        let mut app = new_app();
        let json = serde_json::to_string(&app.render(TRINITY_REWRITE_PLAY_BODY_DOCUMENT, None, &ViewState::default()).expect("render")).unwrap();
        assert!(json.contains("\"Pieces\""));
        assert!(!json.contains("Stücke"));
    }

    #[test]
    fn trinity_rewrite_labels_translate_panels_in_german() {
        let mut app = new_app();
        let view_state = ViewState { locale: Some("de".into()), ..ViewState::default() };
        let document_json = serde_json::to_string(&app.render(TRINITY_REWRITE_PLAY_BODY_DOCUMENT, None, &view_state).expect("render")).unwrap();
        assert!(document_json.contains("Stücke"));
        assert!(!document_json.contains("\"Pieces\""));
        let catalogue_json = serde_json::to_string(&app.render(TRINITY_REWRITE_PLAY_BODY_CATALOGUE, None, &view_state).expect("render")).unwrap();
        assert!(catalogue_json.contains("Katalog"));
        assert!(catalogue_json.contains("Zu LHS hinzufügen"));
        assert!(catalogue_json.contains("Zu RHS hinzufügen"));
        let parameters_json = serde_json::to_string(&app.render(TRINITY_REWRITE_PLAY_BODY_PARAMETERS, None, &view_state).expect("render")).unwrap();
        assert!(parameters_json.contains("\"Parameter\""));
        // 🧰️ `VcsDocumentApp::tools()` no longer exists (see the removed utility-bar test above); the
        // "Verlauf" (History rail group) label had no per-app surface even before removal — only
        // the `resetRule` action label is this app's own to assert on.
        let action_labels = app.app_labels(&view_state).action_labels;
        assert_eq!(action_labels.get("resetRule").map(String::as_str), Some("Regel zurücksetzen"));
    }

    #[test]
    fn set_lhs_json_undo_redo_round_trip() {
        let mut app = new_app();
        let original = app.projection().unwrap().lhs_json;
        let next_lhs = r#"{"pattern":{"leftVar":"x","leftKind":"Piece","edgeVar":"r","edgeKind":"Connection","rightVar":"y","rightKind":"Piece"}}"#;
        testkit::assert_undo_redo_round_trip(
            &mut app,
            "setLhsJson",
            Some(&json!({ "value": next_lhs })),
            |app| app.projection().unwrap().lhs_json,
            original,
            next_lhs.to_string(),
        );
    }
}
//#endregion 🧪️Tests
