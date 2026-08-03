//! ♻️ Trinity Rewrite plugin — parametric rewrite play app bundled as a hot-swappable WASM plugin.
//!
//! 📌️ B1: the pure-trait migration — `TrinityRewritePlayApp` is a unit struct; every former
//! `RewritePlayRuntime` field (selection, hover/select var, camera, LOD, …) now lives in
//! `rewrite_engine::RewriteConfig`, written via `rewrite_op::RewriteConfigOperation`s (real
//! `backwards`, no ad hoc `InverseAction`); every action dispatches through the single typed
//! `rewrite_protocol::TrinityRewriteCommand` channel via `DocumentApp::handle`. Mirrors
//! `shooting_ui::ShootingPlayApp`/`trinity_jack_ui::TrinityJackPlayApp` (the B1 pilot + its jack
//! sibling) — see their doc comments for the full rationale.

use semio_framework_plugin::{SurfaceKind, PanelGroup,
    app_labels, build_node_graph_scene, build_text_editor_scene, text_identifier_bounds_at,
    localized_label_map, tree_item, tree_item_with_action,
    ui_declarative_sections_to_tree, ui_inspector_groups_to_tree,
    ui_inspector_mixed_text, ui_inspector_readonly_field, ui_text, ActionArgDef, ActionArgOption, ActionKind, App, AppActionRegistry, ActionDescriptor, AppLabelsOverlay, AppLabelsOverlayExt, ContextMenuItemSpec, ContextMenuRequest,
    ConfigView, DocumentApp, DocumentView, Emit, LocaleLabels, Media, MediaError, MediaPayload, MeasureSelectItem, NodeGraphScene, NodeGraphNodeRecord, NodeGraphEdgeRecord, NodeGraphPortRecord, NodeGraphViewport, NodeGraphHover, MediaClass, MediaForm, MediaType, PanelTreeBuilder,
    TextEditorScene, UiFieldNode, UiInspectorFieldGroup, UiNode, UiPresence, UiSectionNode, UiTreeItemNode,
    ViewState, WindowLayout, WindowLayoutAxisNode, WindowLayoutChild, WindowLayoutRoot,
    WindowLayoutStackNode, WindowLayoutWindowNode, WindowMeasure, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
    FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, UI_INSPECTOR_MIXED_PLACEHOLDER,
};
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashMap};
use trinity_jack::semantic_tokens;
use trinity_ram::{Camera, Graph, GraphFixture, Node, PortDirection, PropertyValue, TRINITY_GRAPH_SCHEMA};
use rewrite::{LayoutPoint, RewriteRuleState, REWRITE_RULE_SCHEMA};
use rewrite_engine::{
    apply_rule, build_rule_query, rule_query_json, rewrite_io, trinity_lod_scale_json,
    AssignmentJson, Lhs, ParameterKind, ParameterSpec, Rhs, Rule, PatternJson, RewriteConfig,
};
use rewrite_op::{RewriteConfigOperation, RewriteRuleOperation};
use rewrite_protocol::TrinityRewriteCommand;
use store::{DocumentDsl, DocumentPack};

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

//#region 🔖️Locale
/// 🗣️ B1: `cfg.locale`-driven counterparts to the deleted `ViewState`-driven
/// `semio_framework_plugin::is_de_locale`/`resolve_labels` — mirrors `shooting_ui`/`trinity_jack_ui`.
fn is_de_locale(cfg: &RewriteConfig) -> bool {
    cfg.locale.starts_with("de")
}

fn resolve_labels<L: LocaleLabels>(cfg: &RewriteConfig) -> &'static L {
    if is_de_locale(cfg) { L::locale_labels_de() } else { L::locale_labels_en() }
}
//#endregion 🔖️Locale

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
/// before-fixture's initial framing is consumed into the app's live config camera.
fn seed_before_pane_camera(state: &RewriteRuleState) -> Camera {
    parse_fixture_json(&state.before_fixture_json).map(|fixture| fixture.camera).unwrap_or_default()
}

fn rewrite_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor {
        controller_id: TRINITY_REWRITE_PLAY_CONTROLLER_ID.into(),
        action: action.into(),
        args: semio_framework_plugin::optional_json_to_dsl(args),
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

/// ♻️ Pure computation of the rule-applied result graph — reused both by the `After` pane's render
/// and by `DocumentApp::export_media`'s `"graph:out"` port.
fn after_fixture_json(state: &RewriteRuleState) -> String {
    apply_rewrite_to_fixture(&state.before_fixture_json, state)
}

fn sync_select_var_from_node(fixture_json: &str, node_id: &str) -> Option<String> {
    let fixture = parse_fixture_json(fixture_json)?;
    let node = fixture.nodes.iter().find(|node| node.id == node_id)?;
    var_from_node_name(&node.name)
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

/// 🖊️ Applies node-graph editor operations (drag layout / delete-selection) to `state` in place,
/// returning `(document_changed, should_clear_selection)` — B1: pure (no `&mut runtime` — the caller
/// wraps a changed document in a `SetState` operation and an emptied selection in a `SetSelection`
/// config operation instead of mutating an app-struct `RefCell` directly).
fn apply_rewrite_node_graph_edit_operations(state: &mut RewriteRuleState, selected_node_ids: &[String], surface_id: &str, operations: &[Value]) -> (bool, bool) {
    let mut changed = false;
    let mut clear_selection = false;
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
                if selected_node_ids.is_empty() {
                    continue;
                }
                if surface_id == TRINITY_REWRITE_PLAY_SURFACE_BEFORE {
                    if let Some(mut fixture) = parse_fixture_json(&state.before_fixture_json) {
                        fixture.nodes.retain(|node| !selected_node_ids.contains(&node.id));
                        fixture.edges.retain(|edge| {
                            let from = trinity_ram::port_node_id(&edge.source).unwrap_or(&edge.source);
                            let to = trinity_ram::port_node_id(&edge.target).unwrap_or(&edge.target);
                            !selected_node_ids.iter().any(|id| id == from || id == to)
                        });
                        if let Ok(json) = Graph::from_fixture(fixture).and_then(|graph| graph.fixture_json()) {
                            state.before_fixture_json = json;
                            clear_selection = true;
                            changed = true;
                        }
                    }
                } else if surface_id == TRINITY_REWRITE_PLAY_SURFACE_LHS || surface_id == TRINITY_REWRITE_PLAY_SURFACE_RHS {
                    let mut deleted = false;
                    for id in selected_node_ids {
                        deleted |= delete_rule_clause(state, id);
                    }
                    if deleted {
                        clear_selection = true;
                        changed = true;
                    }
                }
            }
            _ => {}
        }
    }
    (changed, clear_selection)
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

fn graph_hover(fixture_json: &str, hover_var: &str, hover_node_id: &str) -> Option<NodeGraphHover> {
    let node_id = if !hover_node_id.is_empty() {
        Some(hover_node_id.to_string())
    } else {
        node_id_for_var(fixture_json, hover_var)
    }?;
    Some(NodeGraphHover { node_id: Some(node_id) })
}

fn graph_selection(fixture_json: &str, select_var: &str, selected_ids: &[String]) -> Vec<String> {
    if !selected_ids.is_empty() {
        return selected_ids.to_vec();
    }
    node_id_for_var(fixture_json, select_var).into_iter().collect()
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

fn fixture_to_workflow(fixture: &GraphFixture) -> (Vec<NodeGraphNodeRecord>, Vec<NodeGraphEdgeRecord>, NodeGraphViewport) {
    let nodes: Vec<NodeGraphNodeRecord> = fixture.nodes.iter().map(node_to_workflow_record).collect();
    let edges: Vec<NodeGraphEdgeRecord> = fixture
        .edges
        .iter()
        .map(|edge| {
            let (source_node_id, source_port_id) = split_endpoint(&edge.source);
            let (target_node_id, target_port_id) = split_endpoint(&edge.target);
            NodeGraphEdgeRecord {
                id: edge.id.clone(),
                source_node_id,
                source_port_id,
                target_node_id,
                target_port_id,
                label: None,
            }
        })
        .collect();
    let viewport = NodeGraphViewport { x: fixture.camera.x, y: fixture.camera.y, zoom: fixture.camera.zoom };
    (nodes, edges, viewport)
}

fn node_to_workflow_record(node: &Node) -> NodeGraphNodeRecord {
    let width = if node.width > 0.0 { node.width } else { 96.0 };
    let height = if node.height > 0.0 { node.height } else { 48.0 };
    NodeGraphNodeRecord {
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
            .map(|port| NodeGraphPortRecord {
                id: trinity_ram::port_key(&node.id, &port.id),
                label: Some(port.id.clone()),
                ..Default::default()
            })
            .collect(),
        outputs: node
            .ports
            .iter()
            .filter(|port| port.direction == PortDirection::Out)
            .map(|port| NodeGraphPortRecord {
                id: trinity_ram::port_key(&node.id, &port.id),
                label: Some(port.id.clone()),
                ..Default::default()
            })
            .collect(),
        ..Default::default()
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
        ("patchNodes", "Patch Nodes", "Knoten aktualisieren"),
        ("nodeGraphEdit", "Edit Graph", "Graph bearbeiten"),
        ("setViewport", "Set Graph Viewport", "Graph-Ansicht festlegen"),
        ("setLhsJson", "Set LHS Json", "LHS-JSON festlegen"),
        ("setRhsJson", "Set RHS Json", "RHS-JSON festlegen"),
        ("setSelection", "Set Selection", "Auswahl festlegen"),
        ("nodeGraphHover", "Hover Graph Node", "Graph-Knoten hovern"),
        ("graphPointerDown", "Graph Pointer Down", "Graph-Zeiger gedrückt"),
        ("textSelect", "Select Text", "Text auswählen"),
        ("textHover", "Hover Text", "Text hovern"),
        ("reorganize", "Reorganize", "Neu anordnen"),
        ("setLodMode", "Set LOD Mode", "LOD-Modus festlegen"),
    ])
}
//#endregion 🔖️CommandLabels

//#region 🔖️Panels
fn build_document_tree(state: &RewriteRuleState, cfg: &RewriteConfig, labels: &TrinityRewriteLabels) -> UiNode {
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
    let selected = cfg.selected_node_ids.iter().map(|id| builder.item_id("node", id)).collect();
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

fn build_inspector_tree(state: &RewriteRuleState, cfg: &RewriteConfig, term_labels: &TrinityRewriteLabels) -> UiNode {
    let Some(fixture) = parse_fixture_json(&state.before_fixture_json) else {
        return ui_declarative_sections_to_tree(&[UiSectionNode {
            id: "trinity-inspector.empty".into(),
            label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
            default_open: Some(true),
            children: vec![ui_text("Invalid trinity fixture")],
            presence: UiPresence::default(),
            menu: None,
        }]);
    };
    if cfg.selected_node_ids.is_empty() {
        return ui_declarative_sections_to_tree(&[UiSectionNode {
            id: "trinity-inspector.empty".into(),
            label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
            default_open: Some(true),
            presence: UiPresence::default(),
            children: vec![ui_text("Select one or more pieces")],
            menu: None,
        }]);
    }
    let nodes: Vec<&Node> = cfg
        .selected_node_ids
        .iter()
        .filter_map(|id| fixture.nodes.iter().find(|node| &node.id == id))
        .collect();
    if nodes.is_empty() {
        return ui_declarative_sections_to_tree(&[UiSectionNode {
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
                UiNode::Field(UiFieldNode {presence: UiPresence::default(),
                    id: "trinity-inspector.name".into(),
                    label: "Name".into(),
                    child: Box::new(UiNode::Input(semio_framework_plugin::UiInputNode {presence: UiPresence::default(),
                        id: "trinity-inspector.name.input".into(),
                        input_kind: "text".into(),
                        value: name_mixed.value,
                        placeholder: name_mixed.placeholder,
                        commit: None,
                        on_change: rewrite_action("patchNodes", Some(json!({ "nodeIds": node_ids, "field": "name" }))),
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
        children.push(UiNode::Field(UiFieldNode {presence: UiPresence::default(),
            id: format!("trinity-rewrite.param.{}", param.name),
            label: param.name.clone(),
            child: Box::new(UiNode::Input(semio_framework_plugin::UiInputNode {presence: UiPresence::default(),
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
fn rewrite_lod_json_for_window(cfg: &RewriteConfig, window_id: &str) -> Option<String> {
    let mode = cfg.lod_mode_by_window.get(window_id).map(String::as_str).unwrap_or(TRINITY_LOD_MODE_AUTOMATIC);
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
    cfg: &RewriteConfig,
    hover_node_id: &str,
    editable: bool,
    camera_override: Option<&Camera>,
) -> UiNode {
    let fixture = parse_fixture_json(fixture_json).unwrap_or_else(|| GraphFixture::parse_dsl(NAKAGIN_FIXTURE_DSL).unwrap());
    let (nodes, edges, fixture_viewport) = fixture_to_workflow(&fixture);
    let viewport = camera_override.map(|camera| NodeGraphViewport { x: camera.x, y: camera.y, zoom: camera.zoom }).unwrap_or(fixture_viewport);
    let hover = graph_hover(fixture_json, &cfg.active_hover_var, hover_node_id);
    let selection = graph_selection(fixture_json, &cfg.active_select_var, &cfg.selected_node_ids);
    build_node_graph_scene(
        surface_id,
        TRINITY_REWRITE_PLAY_CONTROLLER_ID,
        NodeGraphScene {
            hover,
            selection,
            lod_json: rewrite_lod_json_for_window(cfg, window_id),
            editable: editable.then_some(true),
            ..NodeGraphScene::base(nodes, edges, viewport)
        },
    )
}

fn render_fixture_graph(surface_id: &str, window_id: &str, fixture_json: &str, cfg: &RewriteConfig, editable: bool, camera_override: Option<&Camera>) -> UiNode {
    render_rule_graph(surface_id, window_id, fixture_json, cfg, "", editable, camera_override)
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

fn render_jack_editor(state: &RewriteRuleState, cfg: &RewriteConfig) -> UiNode {
    let query = compiled_jack_query(state);
    let active_var = if !cfg.active_hover_var.is_empty() {
        cfg.active_hover_var.as_str()
    } else {
        cfg.active_select_var.as_str()
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
/// projection. B1: unit struct — every former `RewritePlayRuntime`/`self.runtime` field now lives in
/// `rewrite_engine::RewriteConfig` (see `DocumentApp::Config`), written through
/// `rewrite_op::RewriteConfigOperation`s. Every rule/parameter/before-fixture mutation flows through
/// the single LWW `RewriteRuleOperation::SetState`.
#[derive(Default)]
pub struct TrinityRewritePlayApp;

impl DocumentApp for TrinityRewritePlayApp {
    type Projection = RewriteRuleState;
    type Operation = RewriteRuleOperation;
    type Config = RewriteConfig;
    type ConfigOperation = RewriteConfigOperation;
    type Command = TrinityRewriteCommand;

    fn app_id(&self) -> &str {
        TRINITY_REWRITE_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        REWRITE_RULE_SCHEMA
    }

    fn initial_projection(&self) -> RewriteRuleState {
        default_rule_state()
    }

    fn initial_config(&self) -> RewriteConfig {
        let projection = self.initial_projection();
        RewriteConfig { before_pane_camera: seed_before_pane_camera(&projection), ..RewriteConfig::default() }
    }

    fn io(&self) -> Option<semio_framework_plugin::AppIo> {
        Some(rewrite_io())
    }

    fn whole_document_operation(&self, projection: RewriteRuleState) -> Option<RewriteRuleOperation> {
        Some(RewriteRuleOperation::SetState { state: projection })
    }

    /// 🔌️ `"graph:in"` loads an incoming `trinity.graph` pack as this rule's `before_fixture_json`
    /// working graph — reuses the whole-state `SetState` operation, the same seam
    /// `apply_rewrite_node_graph_edit_operations`'s `"setFixture"` sub-op writes through. `"document:in"`
    /// reimplements (rather than delegates to, no supertrait call exists in Rust) the default
    /// `DocumentApp::import_media` body for the rule document itself.
    fn import_media(&self, port: &str, media: &Media, doc: &DocumentView<'_, RewriteRuleState>) -> Result<Emit<RewriteRuleOperation, RewriteConfigOperation>, MediaError> {
        match port {
            "graph:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "graph:in importer only accepts a Structured (base64 pack) payload".into()));
                };
                let bytes = store::pack_rt::pack_value_from_base64(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let fixture = <GraphFixture as DocumentPack>::decode_pack(&bytes).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let fixture_json = fixture.to_json().map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let mut next = doc.projection.clone();
                next.before_fixture_json = fixture_json;
                Ok(Emit::operations(vec![RewriteRuleOperation::SetState { state: next }]))
            }
            "document:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "default document:in importer only accepts a Structured (base64 pack) payload".into()));
                };
                let bytes = store::pack_rt::pack_value_from_base64(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let projection = <RewriteRuleState as DocumentPack>::decode_pack(&bytes).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                match self.whole_document_operation(projection) {
                    Some(operation) => Ok(Emit::operations(vec![operation])),
                    None => Err(MediaError::NotImplemented),
                }
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🔌️ `"graph:out"` re-emits the rule-applied result graph — `after_fixture_json`'s pure
    /// computation, the same one the `After` pane renders — alongside the implicit `"document:out"`
    /// (the whole rule document pack), reimplemented here for the same reason as `import_media` above.
    fn export_media(&self, port: &str, doc: &DocumentView<'_, RewriteRuleState>) -> Result<Media, MediaError> {
        match port {
            "graph:out" => {
                let fixture_json = after_fixture_json(doc.projection);
                let fixture = GraphFixture::from_json(&fixture_json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let bytes = DocumentPack::encode_pack(&fixture);
                Ok(Media {
                    media_type: MediaType { class: MediaClass::Graph, form: MediaForm::Trinity },
                    payload: MediaPayload::Structured { schema: TRINITY_GRAPH_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) },
                })
            }
            "document:out" => {
                let media_type = self.io().map(|io| io.document_media_type).unwrap_or(MediaType { class: MediaClass::Data, form: MediaForm::Value });
                let bytes = doc.projection.encode_pack();
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: self.document_schema().to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🏷️ Maps each `TrinityRewriteCommand` variant back to the action id it was declared under in
    /// `create_rewrite_app` — used by `VcsDocumentApp` for command-log labeling and the registry's
    /// View/Shell kind-discipline check.
    fn command_id(&self, command: &TrinityRewriteCommand) -> &str {
        match command {
            TrinityRewriteCommand::NodeGraphEdit { .. } => "nodeGraphEdit",
            TrinityRewriteCommand::SetLhsJson { .. } => "setLhsJson",
            TrinityRewriteCommand::SetRhsJson { .. } => "setRhsJson",
            TrinityRewriteCommand::SetParameter { .. } => "setParameter",
            TrinityRewriteCommand::AddRuleClause { .. } => "addRuleClause",
            TrinityRewriteCommand::ResetRule => "resetRule",
            TrinityRewriteCommand::PatchNodes { .. } => "patchNodes",
            TrinityRewriteCommand::SetSelection { .. } => "setSelection",
            TrinityRewriteCommand::NodeGraphHover { .. } => "nodeGraphHover",
            TrinityRewriteCommand::SetViewport { .. } => "setViewport",
            TrinityRewriteCommand::GraphPointerDown { .. } => "graphPointerDown",
            TrinityRewriteCommand::TextSelect { .. } => "textSelect",
            TrinityRewriteCommand::TextHover { .. } => "textHover",
            TrinityRewriteCommand::Reorganize => "reorganize",
            TrinityRewriteCommand::SetLodMode { .. } => "setLodMode",
            TrinityRewriteCommand::SetLocale { .. } => "setLocale",
        }
    }

    fn handle(
        &self,
        command: &TrinityRewriteCommand,
        doc: &DocumentView<'_, RewriteRuleState>,
        cfg: &ConfigView<'_, RewriteConfig>,
    ) -> Emit<RewriteRuleOperation, RewriteConfigOperation> {
        let state = doc.projection;
        let config = cfg.projection;
        match command {
            TrinityRewriteCommand::NodeGraphEdit { surface_id, operations_json } => {
                let operations: Vec<Value> = serde_json::from_str(operations_json).unwrap_or_default();
                let mut next = state.clone();
                let (changed, clear_selection) = apply_rewrite_node_graph_edit_operations(&mut next, &config.selected_node_ids, surface_id, &operations);
                if !changed {
                    return Emit::default();
                }
                let config_operations = if clear_selection { vec![RewriteConfigOperation::SetSelection { node_ids: Vec::new() }] } else { Vec::new() };
                Emit { document_operations: vec![RewriteRuleOperation::SetState { state: next }], config_operations, ..Default::default() }
            }
            TrinityRewriteCommand::SetLhsJson { value } => {
                let mut next = state.clone();
                next.lhs_json = value.clone();
                if &next == state { Emit::default() } else { Emit::operations(vec![RewriteRuleOperation::SetState { state: next }]) }
            }
            TrinityRewriteCommand::SetRhsJson { value } => {
                let mut next = state.clone();
                next.rhs_json = value.clone();
                next.parameter_bindings = default_parameter_bindings(&next.rhs_json);
                if &next == state { Emit::default() } else { Emit::operations(vec![RewriteRuleOperation::SetState { state: next }]) }
            }
            TrinityRewriteCommand::SetParameter { name, value } => {
                if name.is_empty() {
                    return Emit::default();
                }
                let Ok(rhs) = serde_json::from_str::<Rhs>(&state.rhs_json) else {
                    return Emit::default();
                };
                let kind = rhs.parameters.iter().find(|param| &param.name == name).map(|param| param.kind.clone());
                let parsed = match kind {
                    Some(ParameterKind::Number) => value.parse::<f64>().ok().map(PropertyValue::Number),
                    Some(ParameterKind::Boolean) => Some(PropertyValue::Bool(value.eq_ignore_ascii_case("true"))),
                    Some(ParameterKind::String) | None => Some(PropertyValue::String(value.clone())),
                };
                match parsed {
                    Some(parsed) => {
                        let mut next = state.clone();
                        next.parameter_bindings.insert(name.clone(), parsed);
                        if &next == state { Emit::default() } else { Emit::operations(vec![RewriteRuleOperation::SetState { state: next }]) }
                    }
                    None => Emit::default(),
                }
            }
            TrinityRewriteCommand::AddRuleClause { kind } => {
                let mut next = state.clone();
                if add_rule_clause(&mut next, kind) {
                    Emit::operations(vec![RewriteRuleOperation::SetState { state: next }])
                } else {
                    Emit::default()
                }
            }
            TrinityRewriteCommand::ResetRule => {
                let next = default_rule_state();
                let camera = seed_before_pane_camera(&next);
                let config_operations = vec![RewriteConfigOperation::SetBeforePaneCamera { camera }];
                if &next == state {
                    Emit::config(config_operations)
                } else {
                    Emit { document_operations: vec![RewriteRuleOperation::SetState { state: next }], config_operations, ..Default::default() }
                }
            }
            TrinityRewriteCommand::PatchNodes { node_ids, field, value } => {
                let trimmed = value.trim();
                if node_ids.is_empty() || field.is_empty() || trimmed.is_empty() {
                    return Emit::default();
                }
                match patch_fixture_nodes(&state.before_fixture_json, node_ids, field, trimmed) {
                    Some(patched) => {
                        let mut next = state.clone();
                        next.before_fixture_json = patched;
                        if &next == state { Emit::default() } else { Emit::operations(vec![RewriteRuleOperation::SetState { state: next }]) }
                    }
                    None => Emit::default(),
                }
            }
            TrinityRewriteCommand::SetSelection { ids, surface_id } => {
                let mut config_operations = vec![RewriteConfigOperation::SetSelection { node_ids: ids.clone() }];
                if let Some(node_id) = ids.first() {
                    let fixture_json = fixture_json_for_surface(surface_id.as_deref().unwrap_or(""), state);
                    if let Some(var) = sync_select_var_from_node(&fixture_json, node_id) {
                        config_operations.push(RewriteConfigOperation::SetActiveSelectVar { value: var });
                    }
                    config_operations.push(RewriteConfigOperation::SetSelectEpoch { value: config.select_epoch + 1 });
                }
                Emit::config(config_operations)
            }
            TrinityRewriteCommand::NodeGraphHover { surface_id, node_id } => match node_id {
                Some(node_id) => {
                    let fixture_json = fixture_json_for_surface(surface_id.as_deref().unwrap_or(""), state);
                    let mut config_operations = vec![RewriteConfigOperation::SetHoverEpoch { value: config.hover_epoch + 1 }];
                    if let Some(var) = sync_select_var_from_node(&fixture_json, node_id) {
                        config_operations.push(RewriteConfigOperation::SetActiveHoverVar { value: var });
                    }
                    Emit::config(config_operations)
                }
                None => Emit::default(),
            },
            TrinityRewriteCommand::SetViewport { surface_id, viewport_json } => {
                if surface_id.as_deref() == Some(TRINITY_REWRITE_PLAY_SURFACE_BEFORE) {
                    match serde_json::from_str::<Camera>(viewport_json) {
                        Ok(camera) => Emit::config(vec![RewriteConfigOperation::SetBeforePaneCamera { camera }]),
                        Err(_) => Emit::default(),
                    }
                } else {
                    Emit::default()
                }
            }
            TrinityRewriteCommand::GraphPointerDown { node_id } => {
                Emit::config(vec![RewriteConfigOperation::SetSelection { node_ids: node_id.clone().map(|id| vec![id]).unwrap_or_default() }])
            }
            TrinityRewriteCommand::TextSelect { var, start } => {
                let mut config_operations = vec![RewriteConfigOperation::SetSelectEpoch { value: config.select_epoch + 1 }];
                if let Some(var) = var {
                    config_operations.push(RewriteConfigOperation::SetActiveSelectVar { value: var.clone() });
                } else if let Some(start) = start {
                    if let Some(token) = jack_token_at_offset(&compiled_jack_query(state), *start as usize) {
                        config_operations.push(RewriteConfigOperation::SetActiveSelectVar { value: token });
                    }
                }
                Emit::config(config_operations)
            }
            TrinityRewriteCommand::TextHover { var, offset } => {
                let mut config_operations = vec![RewriteConfigOperation::SetHoverEpoch { value: config.hover_epoch + 1 }];
                if let Some(var) = var {
                    config_operations.push(RewriteConfigOperation::SetActiveHoverVar { value: var.clone() });
                } else if let Some(offset) = offset {
                    if let Some(token) = jack_token_at_offset(&compiled_jack_query(state), *offset as usize) {
                        config_operations.push(RewriteConfigOperation::SetActiveHoverVar { value: token });
                    }
                }
                Emit::config(config_operations)
            }
            TrinityRewriteCommand::Reorganize => Emit::config(vec![RewriteConfigOperation::SetReorganizeEpoch { value: config.reorganize_epoch + 1 }]),
            TrinityRewriteCommand::SetLodMode { window_id, value } => Emit::config(vec![RewriteConfigOperation::SetLodMode { window_id: window_id.clone(), value: value.clone() }]),
            TrinityRewriteCommand::SetLocale { value } => Emit::config(vec![RewriteConfigOperation::SetLocale { value: value.clone() }]),
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, RewriteRuleState>, cfg: &ConfigView<'_, RewriteConfig>) -> UiNode {
        let state = doc.projection;
        let config = cfg.projection;
        let labels = resolve_labels::<TrinityRewriteLabels>(config);
        match body_key {
            TRINITY_REWRITE_PLAY_BODY_BEFORE => render_fixture_graph(
                TRINITY_REWRITE_PLAY_SURFACE_BEFORE,
                TRINITY_REWRITE_PLAY_WINDOW_BEFORE,
                &state.before_fixture_json,
                config,
                true,
                Some(&config.before_pane_camera),
            ),
            TRINITY_REWRITE_PLAY_BODY_AFTER => render_fixture_graph(
                TRINITY_REWRITE_PLAY_SURFACE_AFTER,
                TRINITY_REWRITE_PLAY_WINDOW_AFTER,
                &after_fixture_json(state),
                config,
                false,
                None,
            ),
            TRINITY_REWRITE_PLAY_BODY_LHS => render_fixture_graph(
                TRINITY_REWRITE_PLAY_SURFACE_LHS,
                TRINITY_REWRITE_PLAY_WINDOW_LHS,
                &lhs_graph_fixture_json(&state.lhs_json, &state.rule_layout),
                config,
                true,
                None,
            ),
            TRINITY_REWRITE_PLAY_BODY_RHS => render_fixture_graph(
                TRINITY_REWRITE_PLAY_SURFACE_RHS,
                TRINITY_REWRITE_PLAY_WINDOW_RHS,
                &rhs_graph_fixture_json(&state.rhs_json, &state.rule_layout),
                config,
                true,
                None,
            ),
            TRINITY_REWRITE_PLAY_BODY_JACK => render_jack_editor(state, config),
            TRINITY_REWRITE_PLAY_BODY_PARAMETERS => build_parameters_panel(state, labels),
            TRINITY_REWRITE_PLAY_BODY_DOCUMENT => build_document_tree(state, config, labels),
            TRINITY_REWRITE_PLAY_BODY_CATALOGUE => build_catalogue_tree(labels),
            TRINITY_REWRITE_PLAY_BODY_INSPECTION => build_inspector_tree(state, config, labels),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn window_measures(&self, _doc: &DocumentView<'_, RewriteRuleState>, cfg: &ConfigView<'_, RewriteConfig>) -> HashMap<String, Vec<WindowMeasure>> {
        let config = cfg.projection;
        let mode_for = |window_id: &str| config.lod_mode_by_window.get(window_id).map(String::as_str).unwrap_or(TRINITY_LOD_MODE_AUTOMATIC);
        HashMap::from([
            (TRINITY_REWRITE_PLAY_WINDOW_BEFORE.to_string(), vec![trinity_rewrite_lod_measure(TRINITY_REWRITE_PLAY_WINDOW_BEFORE, mode_for(TRINITY_REWRITE_PLAY_WINDOW_BEFORE))]),
            (TRINITY_REWRITE_PLAY_WINDOW_AFTER.to_string(), vec![trinity_rewrite_lod_measure(TRINITY_REWRITE_PLAY_WINDOW_AFTER, mode_for(TRINITY_REWRITE_PLAY_WINDOW_AFTER))]),
            (TRINITY_REWRITE_PLAY_WINDOW_LHS.to_string(), vec![trinity_rewrite_lod_measure(TRINITY_REWRITE_PLAY_WINDOW_LHS, mode_for(TRINITY_REWRITE_PLAY_WINDOW_LHS))]),
            (TRINITY_REWRITE_PLAY_WINDOW_RHS.to_string(), vec![trinity_rewrite_lod_measure(TRINITY_REWRITE_PLAY_WINDOW_RHS, mode_for(TRINITY_REWRITE_PLAY_WINDOW_RHS))]),
        ])
    }

    fn app_labels(&self, cfg: &ConfigView<'_, RewriteConfig>) -> AppLabelsOverlay {
        let labels = resolve_labels::<TrinityRewriteLabels>(cfg.projection);
        AppLabelsOverlay::default()
            .window_kind_label(TRINITY_REWRITE_PLAY_WINDOW_BEFORE, labels.window_before)
            .window_kind_label(TRINITY_REWRITE_PLAY_WINDOW_AFTER, labels.window_after)
            .window_kind_label(TRINITY_REWRITE_PLAY_WINDOW_LHS, labels.window_lhs)
            .window_kind_label(TRINITY_REWRITE_PLAY_WINDOW_RHS, labels.window_rhs)
            .window_kind_label(TRINITY_REWRITE_PLAY_WINDOW_JACK, labels.window_jack)
            .window_kind_label(TRINITY_REWRITE_PLAY_WINDOW_PARAMETERS, labels.window_parameters)
            .action_labels(trinity_rewrite_action_labels(is_de_locale(cfg.projection)))
    }

    fn context_menu(
        &self,
        request: &ContextMenuRequest,
        _doc: &DocumentView<'_, RewriteRuleState>,
        cfg: &ConfigView<'_, RewriteConfig>,
        registry: &AppActionRegistry,
    ) -> Vec<ContextMenuItemSpec> {
        use semio_framework_plugin::{node_graph_delete_selection_spec, selection_domains_from_surface, Menu, NodeGraphDeleteDispatch};

        let is_de = is_de_locale(cfg.projection);
        let selected = cfg.projection.selected_node_ids.clone();
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
            .operation("patchNodes", "Patch Nodes")
            .operation("nodeGraphEdit", "Edit Graph")
            // 🛠️ Dev-only raw rule editors — kept out of the command palette.
            .action_with(semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::new_catalog("setLhsJson", "Set LHS Json", ActionKind::Operation) })
            .action_with(semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::new_catalog("setRhsJson", "Set RHS Json", ActionKind::Operation) })
            // 👁️ Ephemeral view state — selection, hover, text cursor, recompute/layout, LOD.
            .view_action("setSelection", "Set Selection")
            .view_action("nodeGraphHover", "Hover Graph Node")
            .view_action("setViewport", "Set Graph Viewport")
            .view_action("graphPointerDown", "Graph Pointer Down")
            .view_action("textSelect", "Select Text")
            .view_action("textHover", "Hover Text")
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
            .keybinding("mod+alt+s", "commitCheckpoint")
            .io(rewrite_io()),
    )
    .example("label-core", "Label Core", default_rule_state().print_dsl(), "file-text")
    .workflow("trinity-rewrite", "Trinity Rewrite", "graph")
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{testkit, PluginApp, VcsDocumentApp};

    fn meta(actor: &str) -> semio_framework_plugin::ActionMeta {
        testkit::meta(actor)
    }

    fn new_app() -> VcsDocumentApp<TrinityRewritePlayApp> {
        testkit::new_app::<TrinityRewritePlayApp>()
    }

    #[test]
    fn renders_before_and_after_graphs() {
        let mut app = new_app();
        let before = app.render(TRINITY_REWRITE_PLAY_BODY_BEFORE, None, &ViewState::default()).expect("render");
        let after = app.render(TRINITY_REWRITE_PLAY_BODY_AFTER, None, &ViewState::default()).expect("render");
        assert!(serde_json::to_string(&before).unwrap().contains("node-graph"));
        assert!(serde_json::to_string(&after).unwrap().contains("node-graph"));
    }

    /// 🎥️ `setViewport` on the Before pane is config-only: it writes the config's `before_pane_camera`
    /// and emits zero document operations (no whole-document `SetState` replace per pan tick), and the
    /// Before pane's render composes that config camera over `before_fixture_json` instead of
    /// round-tripping it through the document.
    #[test]
    fn set_viewport_writes_before_pane_config_camera_without_document_operations() {
        let mut app = new_app();
        let before_state = app.projection().unwrap();
        let result = app
            .dispatch_typed(
                TrinityRewriteCommand::SetViewport { surface_id: Some(TRINITY_REWRITE_PLAY_SURFACE_BEFORE.into()), viewport_json: json!({ "x": 10.0, "y": 20.0, "zoom": 2.5 }).to_string() },
                &meta("local"),
            )
            .expect("viewport");
        assert!(result.operations.is_empty(), "camera is a config-only command, no document operations");
        assert_eq!(app.projection().unwrap(), before_state, "document is untouched by a viewport pan");
        let before = app.render(TRINITY_REWRITE_PLAY_BODY_BEFORE, None, &ViewState::default()).expect("render");
        assert!(serde_json::to_string(&before).unwrap().contains("2.5"), "render reads the live config camera");
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
        let result = app.dispatch_typed(TrinityRewriteCommand::SetParameter { name: "label".into(), value: "changed".into() }, &meta("local")).expect("set parameter");
        assert_eq!(result.operations.len(), 1, "a parameter edit is a single SetState operation");
        assert_eq!(app.projection().unwrap().parameter_bindings.get("label").cloned(), Some(PropertyValue::String("changed".into())));
        app.handle_action("undo", None, &meta("local")).expect("undo");
        assert_eq!(app.projection().unwrap().parameter_bindings.get("label").cloned(), Some(PropertyValue::String("nakagin-core".into())));
    }

    #[test]
    fn add_and_delete_rhs_set_clause() {
        let mut app = new_app();
        app.dispatch_typed(TrinityRewriteCommand::AddRuleClause { kind: "set".into() }, &meta("local")).expect("add clause");
        let rhs: Rhs = serde_json::from_str(&app.projection().unwrap().rhs_json).unwrap();
        assert_eq!(rhs.set.len(), 2);
        // deleteSelection requires a prior selection; select the newly added clause first (config).
        app.dispatch_typed(TrinityRewriteCommand::SetSelection { ids: vec!["rhs-set-1".into()], surface_id: Some(TRINITY_REWRITE_PLAY_SURFACE_RHS.into()) }, &meta("local")).expect("select");
        let result = app
            .dispatch_typed(
                TrinityRewriteCommand::NodeGraphEdit { surface_id: TRINITY_REWRITE_PLAY_SURFACE_RHS.into(), operations_json: json!([{ "operation": "deleteSelection" }]).to_string() },
                &meta("local"),
            )
            .expect("delete selection");
        assert!(!result.operations.is_empty());
        let rhs: Rhs = serde_json::from_str(&app.projection().unwrap().rhs_json).unwrap();
        assert_eq!(rhs.set.len(), 1);
    }

    #[test]
    fn jack_view_has_occurrences_after_select() {
        let mut app = new_app();
        let result = app.dispatch_typed(TrinityRewriteCommand::TextSelect { var: Some("a".into()), start: None }, &meta("local")).expect("text select");
        assert!(result.operations.is_empty(), "text selection is a config-only command, no document operations");
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
        app.dispatch_typed(TrinityRewriteCommand::SetLocale { value: "de-DE".into() }, &meta("local")).expect("set locale");
        let document_json = serde_json::to_string(&app.render(TRINITY_REWRITE_PLAY_BODY_DOCUMENT, None, &ViewState::default()).expect("render")).unwrap();
        assert!(document_json.contains("Stücke"));
        assert!(!document_json.contains("\"Pieces\""));
        let catalogue_json = serde_json::to_string(&app.render(TRINITY_REWRITE_PLAY_BODY_CATALOGUE, None, &ViewState::default()).expect("render")).unwrap();
        assert!(catalogue_json.contains("Katalog"));
        assert!(catalogue_json.contains("Zu LHS hinzufügen"));
        assert!(catalogue_json.contains("Zu RHS hinzufügen"));
        let parameters_json = serde_json::to_string(&app.render(TRINITY_REWRITE_PLAY_BODY_PARAMETERS, None, &ViewState::default()).expect("render")).unwrap();
        assert!(parameters_json.contains("\"Parameter\""));
        let action_labels = app.app_labels().action_labels;
        assert_eq!(action_labels.get("resetRule").map(String::as_str), Some("Regel zurücksetzen"));
    }

    #[test]
    fn set_lhs_json_undo_redo_round_trip() {
        let mut app = new_app();
        let original = app.projection().unwrap().lhs_json;
        let next_lhs = r#"{"pattern":{"leftVar":"x","leftKind":"Piece","edgeVar":"r","edgeKind":"Connection","rightVar":"y","rightKind":"Piece"}}"#;
        app.dispatch_typed(TrinityRewriteCommand::SetLhsJson { value: next_lhs.into() }, &meta("local")).expect("set lhs");
        assert_eq!(app.projection().unwrap().lhs_json, next_lhs);
        app.handle_action("undo", None, &meta("local")).expect("undo");
        assert_eq!(app.projection().unwrap().lhs_json, original);
        app.handle_action("redo", None, &meta("local")).expect("redo");
        assert_eq!(app.projection().unwrap().lhs_json, next_lhs);
    }

    #[test]
    fn export_media_graph_out_reflects_rule_applied_fixture() {
        let mut app = new_app();
        let graph_out = app.export_media("graph:out").expect("graph:out export");
        let MediaPayload::Structured { json, .. } = graph_out.payload else { panic!("structured payload") };
        let bytes = store::pack_rt::pack_value_from_base64(&json).expect("decode base64");
        let fixture = <GraphFixture as DocumentPack>::decode_pack(&bytes).expect("decode pack");
        let expected = GraphFixture::from_json(&after_fixture_json(&app.projection().unwrap())).unwrap();
        assert_eq!(fixture.nodes.len(), expected.nodes.len());
    }
}
//#endregion 🧪️Tests
