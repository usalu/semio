//! ♻️ Parametric graph rewriting for trinity graphs with optional WASM canvas host.

use infinite_board_port_directed::{
    compute_edge_bezier_points, distance_between,
    force_graph::{apply_force_graph_layout_to_fixture_v1_value, ForceGraphLayoutOptions},
    BoardEngine, CanvasPalette, HandleRole,
};
use infinite_board_port_directed_normal::BoardHost;
pub use infinite_cavas as cavas;
use serde::{Deserialize, Serialize};
use std::cell::Cell;
use std::collections::HashMap;
use trinity_jack::{execute, parse};
use trinity_ram::{create_trinity_graph_envelope, dispatch_trinity_graph_operations, port_key, Graph, GraphFixture, Node, PortDirection, PropertyValue, TrinityGraphOperation, TrinityGraphStore};

pub use trinity_jack::{complete as complete_jack, parse as parse_jack, run as run_jack, run_json as run_jack_json, tokenize as tokenize_jack, Completion as JackCompletion, Pattern, QueryResult, QueryResultKind, TokenSpan as JackTokenSpan};
pub use trinity_ram::{self, Camera, Manifest};

//#region ⚠️ Errors
/// ⚠️ Trinity rewrite-engine errors.
#[derive(Debug, thiserror::Error)]
pub enum TrinityRewriteError {
    /// 🧩 Trinity graph fixture load/validation/mutation failure.
    #[error(transparent)]
    Graph(#[from] trinity_ram::TrinityRamError),
    /// 🧭 VCS store/dispatch failure.
    #[error(transparent)]
    Vcs(#[from] vcs::VcsError),
    /// 🧬 JSON (de)serialization failure.
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    /// 🔤 Jack query parse/execute failure (`trinity_jack`'s own API is not yet thiserror-migrated).
    #[error("{0}")]
    Jack(String),
    /// 📐 Force-directed layout failure (`infinite_board_port_directed`'s own API is not yet thiserror-migrated).
    #[error("{0}")]
    Layout(String),
    /// 🎨 Canvas theme merge failure (`infinite_board_port_directed`'s own API is not yet thiserror-migrated).
    #[error("{0}")]
    CanvasTheme(String),
    #[error("force layout fixture missing nodes")]
    ForceLayoutFixtureMissingNodes,
}
//#endregion ⚠️ Errors

type TrinityBoardEngine = BoardEngine;

const TRINITY_HANDLE_RADIUS: f64 = 5.0;
const TRINITY_BOARD_PORT_HANDLE_KIND: &str = "port";
const TRINITY_DEFAULT_NODE_RADIUS: f64 = 44.0;
const TRINITY_BOARD_KIND_CATALOGS_JSON: &str = "{\"handleKinds\":[{\"id\":\"port\",\"name\":\"Port\",\"color\":\"#6b7280\"}],\"edgeKinds\":[{\"id\":\"Connection\",\"name\":\"Connection\",\"color\":\"#94a3b8\"}]}";
const TRINITY_EDGE_STROKE: f64 = 1.5;

// #region 🔖Rewrite
/// ◀️ Left-hand side pattern for rewriting.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lhs {
    pub pattern: PatternJson,
    #[serde(default)]
    pub where_clause: Option<String>,
}

/// 🏷️ Parameter kind for parametric rewrite rules.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ParameterKind {
    String,
    Number,
    Boolean,
}

/// 🎛️ Parameter declaration on the right-hand side.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterSpec {
    pub name: String,
    pub kind: ParameterKind,
    pub default: PropertyValue,
}

/// ▶️ Right-hand side mutation for rewriting.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rhs {
    #[serde(default)]
    pub create: Vec<PatternJson>,
    #[serde(default)]
    pub delete: Vec<String>,
    #[serde(default)]
    pub set: Vec<AssignmentJson>,
    #[serde(default)]
    pub merge: Vec<PatternJson>,
    #[serde(default)]
    pub parameters: Vec<ParameterSpec>,
}

/// 📜 Rewrite rule.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rule {
    pub name: String,
    pub lhs: Lhs,
    pub rhs: Rhs,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatternJson {
    pub left_var: String,
    pub left_kind: String,
    #[serde(default)]
    pub edge_var: Option<String>,
    #[serde(default)]
    pub edge_kind: Option<String>,
    #[serde(default)]
    pub right_var: Option<String>,
    #[serde(default)]
    pub right_kind: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssignmentJson {
    pub var: String,
    pub prop: String,
    pub value: PropertyValue,
}

impl PatternJson {
    fn to_jack_pattern(&self) -> Pattern {
        use trinity_jack::{PatternEdge, PatternNode};
        let left = PatternNode { var: self.left_var.clone(), kind: self.left_kind.clone() };
        if let (Some(right_var), Some(right_kind)) = (&self.right_var, &self.right_kind) {
            Pattern { nodes: vec![left], edge: Some(PatternEdge { var: self.edge_var.clone(), kind: self.edge_kind.clone(), directed: true, right: PatternNode { var: right_var.clone(), kind: right_kind.clone() } }) }
        } else {
            Pattern { nodes: vec![left], edge: None }
        }
    }
}

fn pattern_to_match_clause(pattern: &PatternJson) -> String {
    let p = pattern.to_jack_pattern();
    let left = format!("({}:{} )", p.nodes[0].var, p.nodes[0].kind).replace(" )", ")");
    if let Some(edge) = &p.edge {
        let edge_mid = match (&edge.var, &edge.kind) {
            (Some(v), Some(k)) => format!("[{v}:{k}]"),
            (Some(v), None) => format!("[{v}]"),
            (None, Some(k)) => format!("[:{k}]"),
            (None, None) => "[]".into(),
        };
        format!("({}:{} )-{edge_mid}->({}:{} )", p.nodes[0].var, p.nodes[0].kind, edge.right.var, edge.right.kind).replace(" )", ")")
    } else {
        left
    }
}

fn parse_bindings_json(bindings_json: &str) -> Result<HashMap<String, PropertyValue>, TrinityRewriteError> {
    if bindings_json.trim().is_empty() {
        return Ok(HashMap::new());
    }
    Ok(serde_json::from_str(bindings_json)?)
}

fn parameter_defaults(rule: &Rule) -> HashMap<String, PropertyValue> {
    let mut defaults = HashMap::new();
    for param in &rule.rhs.parameters {
        defaults.insert(param.name.clone(), param.default.clone());
    }
    defaults
}

fn effective_bindings(rule: &Rule, bindings: &HashMap<String, PropertyValue>) -> HashMap<String, PropertyValue> {
    let mut merged = parameter_defaults(rule);
    for (key, value) in bindings {
        merged.insert(key.clone(), value.clone());
    }
    merged
}

fn resolve_parameter_value(rule: &Rule, bindings: &HashMap<String, PropertyValue>, value: &PropertyValue) -> PropertyValue {
    if let PropertyValue::String(s) = value {
        if let Some(name) = s.strip_prefix('$') {
            if !name.is_empty() {
                if let Some(resolved) = bindings.get(name) {
                    return resolved.clone();
                }
                for param in &rule.rhs.parameters {
                    if param.name == name {
                        return param.default.clone();
                    }
                }
            }
        }
    }
    value.clone()
}

fn assignment_value_jack(rule: &Rule, bindings: &HashMap<String, PropertyValue>, value: &PropertyValue) -> String {
    let resolved = resolve_parameter_value(rule, bindings, value);
    match resolved {
        PropertyValue::Null => "null".into(),
        PropertyValue::Bool(b) => b.to_string(),
        PropertyValue::Number(n) => n.to_string(),
        PropertyValue::String(s) => format!("'{s}'"),
        PropertyValue::Array(_) | PropertyValue::Object(_) => serde_json::to_string(&resolved).unwrap_or_else(|_| "null".into()),
    }
}

/// 🧵 Build the Jack query string for a rewrite rule without executing it.
pub fn build_rule_query(rule: &Rule, bindings: &HashMap<String, PropertyValue>) -> String {
    let effective = effective_bindings(rule, bindings);
    let mut query = format!("MATCH {}", pattern_to_match_clause(&rule.lhs.pattern));
    if let Some(where_clause) = &rule.lhs.where_clause {
        if !where_clause.trim().is_empty() {
            query.push_str(&format!(" WHERE {where_clause}"));
        }
    }
    for del in &rule.rhs.delete {
        query.push_str(&format!(" DELETE {del}"));
    }
    for set in &rule.rhs.set {
        let val = assignment_value_jack(rule, &effective, &set.value);
        query.push_str(&format!(" SET {}.{} = {val}", set.var, set.prop));
    }
    for create in &rule.rhs.create {
        query.push_str(&format!(" CREATE {}", pattern_to_match_clause(create)));
    }
    for merge in &rule.rhs.merge {
        query.push_str(&format!(" MERGE {}", pattern_to_match_clause(merge)));
    }
    query
}

/// ♻️ Apply a rewrite rule to a graph.
pub fn apply_rule(graph: &mut Graph, rule: &Rule, bindings: &HashMap<String, PropertyValue>) -> Result<QueryResult, TrinityRewriteError> {
    let query = build_rule_query(rule, bindings);
    let parsed = parse(&query).map_err(TrinityRewriteError::Jack)?;
    let (result, operations) = execute(graph, &parsed).map_err(TrinityRewriteError::Jack)?;
    if !operations.is_empty() {
        let fixture = trinity_ram::apply_trinity_graph_operations(graph.to_fixture(), &operations)?;
        *graph = Graph::from_fixture(fixture)?;
    }
    Ok(result)
}

/// ♻️ Apply a rewrite rule from JSON.
pub fn apply_rule_json(graph: &mut Graph, rule_json: &str, bindings_json: &str) -> Result<String, TrinityRewriteError> {
    let rule: Rule = serde_json::from_str(rule_json)?;
    let bindings = parse_bindings_json(bindings_json)?;
    let result = apply_rule(graph, &rule, &bindings)?;
    Ok(serde_json::to_string(&ApplyRuleResult { fixture: graph.fixture_json()?, query: result })?)
}

/// 🧵 Build a rewrite rule Jack query from JSON without a graph.
pub fn rule_query_json(rule_json: &str, bindings_json: &str) -> Result<String, TrinityRewriteError> {
    let rule: Rule = serde_json::from_str(rule_json)?;
    let bindings = parse_bindings_json(bindings_json)?;
    let query = build_rule_query(&rule, &bindings);
    Ok(serde_json::to_string(&RuleQueryResult { query })?)
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ApplyRuleResult {
    fixture: String,
    query: QueryResult,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RuleQueryResult {
    query: String,
}
// #endregion 🔖Rewrite

// #region 🔖RuleVcs
use vcs::{create_document_vcs_envelope, DocumentVcsCommand, DocumentVcsEnvelope, DocumentVcsStore, Operation, OperationDiff};

/// 📐 The full rewrite-rule document: before fixture, LHS/RHS patterns, parameter bindings, and rule-graph layout overrides.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RewriteRuleState {
    pub before_fixture_json: String,
    pub lhs_json: String,
    pub rhs_json: String,
    #[serde(default)]
    pub parameter_bindings: HashMap<String, PropertyValue>,
    #[serde(default)]
    pub rule_layout: HashMap<String, (f64, f64)>,
}

/// 🔁 Whole-state snapshot diff: the rule document is one small unit, so history stores full pre/post states rather than field-level patches.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RewriteRuleDiff {
    pub next: Option<RewriteRuleState>,
}

impl OperationDiff<RewriteRuleState> for RewriteRuleDiff {
    fn apply(&self, projection: &RewriteRuleState) -> RewriteRuleState {
        self.next.clone().unwrap_or_else(|| projection.clone())
    }

    fn absorb(&mut self, other: Self) {
        if other.next.is_some() {
            self.next = other.next;
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum RewriteRuleOperation {
    SetState { state: RewriteRuleState },
}

impl Operation<RewriteRuleState> for RewriteRuleOperation {
    type Diff = RewriteRuleDiff;

    fn diff(&self, _projection: &RewriteRuleState) -> Self::Diff {
        match self {
            RewriteRuleOperation::SetState { state } => RewriteRuleDiff { next: Some(state.clone()) },
        }
    }

    fn backwards(&self, projection: &RewriteRuleState) -> Vec<Self> {
        vec![RewriteRuleOperation::SetState { state: projection.clone() }]
    }
}

pub type RewriteRuleEnvelope = DocumentVcsEnvelope<RewriteRuleState, RewriteRuleOperation>;
pub type RewriteRuleStore = DocumentVcsStore<RewriteRuleState, RewriteRuleOperation>;

pub const REWRITE_RULE_SCHEMA: &str = "trinity.rewrite.rule";

pub fn create_rewrite_rule_envelope(id: &str, state: RewriteRuleState) -> RewriteRuleEnvelope {
    create_document_vcs_envelope(REWRITE_RULE_SCHEMA, id, state, None)
}

pub fn dispatch_rewrite_rule_state(store: &mut RewriteRuleStore, state: RewriteRuleState) -> Result<(), TrinityRewriteError> {
    let current = store.projection()?;
    if current == state {
        return Ok(());
    }
    store.dispatch(DocumentVcsCommand::Apply { operations: vec![RewriteRuleOperation::SetState { state }], description: None }).map_err(TrinityRewriteError::from)
}
// #endregion 🔖RuleVcs

// #region 🔖Lod
use cavas::lod::{Lod, LodScale};

const TRINITY_LODS: &[Lod; 6] = &[
    Lod { id: "minimap", name: "Minimap", description: "Whole-graph silhouette; edges and node fills only.", max_zoom: 0.15 },
    Lod { id: "overview", name: "Overview", description: "Topology without labels or port handles.", max_zoom: 0.35 },
    Lod { id: "compact", name: "Compact", description: "Abbreviated node names.", max_zoom: 0.55 },
    Lod { id: "normal", name: "Normal", description: "Full node names.", max_zoom: 1.25 },
    Lod { id: "detail", name: "Detail", description: "Node names and port handles.", max_zoom: 2.5 },
    Lod { id: "micro", name: "Micro", description: "Maximum port-graph fidelity.", max_zoom: f64::INFINITY },
];

const TRINITY_LOD_SCALE: LodScale = LodScale { lods: TRINITY_LODS };

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TrinityDrawLod {
    Minimap,
    Overview,
    Compact,
    Normal,
    Detail,
    Micro,
}

impl TrinityDrawLod {
    fn label(self) -> &'static str {
        match self {
            Self::Minimap => "minimap",
            Self::Overview => "overview",
            Self::Compact => "compact",
            Self::Normal => "normal",
            Self::Detail => "detail",
            Self::Micro => "micro",
        }
    }

    fn from_id(id: &str) -> Option<Self> {
        Some(match id {
            "minimap" => Self::Minimap,
            "overview" => Self::Overview,
            "compact" => Self::Compact,
            "normal" => Self::Normal,
            "detail" => Self::Detail,
            "micro" => Self::Micro,
            _ => return None,
        })
    }

    fn from_scale_index(index: usize) -> Self {
        match index {
            0 => Self::Minimap,
            1 => Self::Overview,
            2 => Self::Compact,
            3 => Self::Normal,
            4 => Self::Detail,
            _ => Self::Micro,
        }
    }

    fn handles_visible(self) -> bool {
        matches!(self, Self::Detail | Self::Micro)
    }

    fn labels_visible(self) -> bool {
        !matches!(self, Self::Minimap | Self::Overview)
    }

    fn full_labels(self) -> bool {
        matches!(self, Self::Normal | Self::Detail | Self::Micro)
    }
}

fn trinity_lod_index(zoom: f64) -> usize {
    TRINITY_LOD_SCALE.resolve_index(zoom.max(0.05))
}

fn trinity_abbreviate_label(name: &str) -> String {
    let trimmed = name.trim();
    if trimmed.len() <= 4 {
        return trimmed.to_string();
    }
    trimmed.chars().take(3).collect()
}

pub fn trinity_lod_scale_json() -> String {
    let rows: Vec<serde_json::Value> = TRINITY_LODS
        .iter()
        .map(|lod| {
            serde_json::json!({
                "id": lod.id,
                "name": lod.name,
                "description": lod.description,
                "maxZoom": lod.max_zoom,
            })
        })
        .collect();
    serde_json::to_string(&rows).unwrap_or_else(|_| "[]".into())
}

fn trinity_node_radius(node: &Node) -> f64 {
    let w = if node.width > 0.0 { node.width } else { 88.0 };
    let h = if node.height > 0.0 { node.height } else { 40.0 };
    (w.max(h) * 0.5).max(TRINITY_DEFAULT_NODE_RADIUS * 0.5)
}

fn trinity_circle_port_angle(index: usize, count: usize, left: bool) -> f64 {
    let base = if left { std::f64::consts::PI } else { 0.0 };
    let spread = 0.35;
    let t = (index as f64 + 0.5) / count.max(1) as f64 - 0.5;
    base + t * spread
}

fn trinity_graph_to_board_fixture(graph: &Graph) -> serde_json::Value {
    let nodes: Vec<serde_json::Value> = graph
        .nodes
        .values()
        .map(|node| {
            let radius = trinity_node_radius(node);
            let in_ports: Vec<_> = node.ports.iter().filter(|port| port.direction == PortDirection::In).collect();
            let out_ports: Vec<_> = node.ports.iter().filter(|port| port.direction == PortDirection::Out).collect();
            let mut handles = Vec::new();
            for (index, port) in in_ports.iter().enumerate() {
                handles.push(serde_json::json!({
                    "id": port_key(&node.id, &port.id),
                    "handleKind": TRINITY_BOARD_PORT_HANDLE_KIND,
                    "angle": trinity_circle_port_angle(index, in_ports.len(), true),
                }));
            }
            for (index, port) in out_ports.iter().enumerate() {
                handles.push(serde_json::json!({
                    "id": port_key(&node.id, &port.id),
                    "handleKind": TRINITY_BOARD_PORT_HANDLE_KIND,
                    "angle": trinity_circle_port_angle(index, out_ports.len(), false),
                }));
            }
            serde_json::json!({
                "id": node.id,
                "x": node.x,
                "y": node.y,
                "radius": radius,
                "shape": "circle",
                "text": node.name,
                "nodeKind": node.kind,
                "handles": handles,
            })
        })
        .collect();
    let edges: Vec<serde_json::Value> = graph
        .edges
        .values()
        .map(|edge| {
            serde_json::json!({
                "id": edge.id,
                "source": edge.source,
                "target": edge.target,
                "edgeKind": edge.kind,
            })
        })
        .collect();
    serde_json::json!({
        "schema": "puzzle.2d.fixture",
        "camera": {
            "x": graph.camera.x,
            "y": graph.camera.y,
            "zoom": graph.camera.zoom,
        },
        "nodes": nodes,
        "edges": edges,
    })
}

fn trinity_graph_to_force_layout_fixture(graph: &Graph) -> serde_json::Value {
    let nodes: Vec<serde_json::Value> = graph
        .nodes
        .values()
        .map(|node| {
            let radius = trinity_node_radius(node);
            let handles: Vec<serde_json::Value> = node.ports.iter().map(|port| serde_json::json!({ "id": port_key(&node.id, &port.id) })).collect();
            serde_json::json!({
                "id": node.id,
                "x": node.x,
                "y": node.y,
                "radius": radius,
                "shape": "circle",
                "handles": handles,
            })
        })
        .collect();
    let edges: Vec<serde_json::Value> = graph.edges.values().map(|edge| serde_json::json!({ "source": edge.source, "target": edge.target })).collect();
    serde_json::json!({
        "schema": GraphFixture::SCHEMA,
        "nodes": nodes,
        "edges": edges,
    })
}

fn apply_force_layout_positions_to_trinity_graph(graph: &mut Graph, fixture: &serde_json::Value) -> Result<(), TrinityRewriteError> {
    let nodes = fixture.get("nodes").and_then(|v| v.as_array()).ok_or(TrinityRewriteError::ForceLayoutFixtureMissingNodes)?;
    for node in nodes {
        let Some(obj) = node.as_object() else {
            continue;
        };
        let Some(id) = obj.get("id").and_then(|v| v.as_str()) else {
            continue;
        };
        let Some(entry) = graph.nodes.get_mut(id) else {
            continue;
        };
        if let Some(x) = obj.get("x").and_then(|v| v.as_f64()) {
            entry.x = x;
        }
        if let Some(y) = obj.get("y").and_then(|v| v.as_f64()) {
            entry.y = y;
        }
    }
    Ok(())
}

fn force_layout_reposition_operations(fixture: &GraphFixture) -> Result<Vec<TrinityGraphOperation>, TrinityRewriteError> {
    let mut graph = Graph::from_fixture(fixture.clone())?;
    apply_force_layout_to_trinity_graph(&mut graph)?;
    let next = graph.to_fixture();
    let mut operations = Vec::new();
    for node in &next.nodes {
        let Some(prev) = fixture.nodes.iter().find(|entry| entry.id == node.id) else {
            continue;
        };
        if (prev.x - node.x).abs() > 1e-6 || (prev.y - node.y).abs() > 1e-6 {
            operations.push(TrinityGraphOperation::Reposition { id: node.id.clone(), x: node.x, y: node.y });
        }
    }
    Ok(operations)
}

fn apply_force_layout_to_trinity_graph(graph: &mut Graph) -> Result<(), TrinityRewriteError> {
    let mut fixture = trinity_graph_to_force_layout_fixture(graph);
    apply_force_graph_layout_to_fixture_v1_value(&mut fixture, &ForceGraphLayoutOptions::default()).map_err(TrinityRewriteError::Layout)?;
    apply_force_layout_positions_to_trinity_graph(graph, &fixture)
}
// #endregion 🔖Lod

// #region 🔖TrinityHost
/// 🖥️ Retained trinity graph host on the directed port board engine.
pub struct TrinityHost {
    pub graph: Graph,
    store: TrinityGraphStore,
    pub engine: TrinityBoardEngine,
    board: BoardHost,
    pub canvas_theme: CanvasPalette,
    width: u32,
    height: u32,
    dpr: f64,
    node_id_map: HashMap<u64, String>,
    handle_key_map: HashMap<u64, String>,
    edge_id_map: HashMap<u64, String>,
    last_logged_lod: Cell<i8>,
    automatic_lod: bool,
    forced_draw_lod: Option<TrinityDrawLod>,
}

impl TrinityHost {
    pub fn from_graph(graph: Graph) -> Self {
        let fixture = graph.to_fixture();
        let store = TrinityGraphStore::new(create_trinity_graph_envelope("trinity-host", fixture));
        let graph = Graph::from_fixture(store.projection().expect("projection")).expect("graph");
        let mut host = Self {
            graph,
            store,
            engine: TrinityBoardEngine::new(),
            board: BoardHost::new(),
            canvas_theme: CanvasPalette::default(),
            width: 1,
            height: 1,
            dpr: 1.0,
            node_id_map: HashMap::new(),
            handle_key_map: HashMap::new(),
            edge_id_map: HashMap::new(),
            last_logged_lod: Cell::new(-1),
            automatic_lod: true,
            forced_draw_lod: None,
        };
        host.rebuild_engine();
        host
    }

    pub fn load_fixture_json(json: &str) -> Result<Self, TrinityRewriteError> {
        let graph = Graph::load_json(json)?;
        Ok(Self::from_graph(graph))
    }

    fn refresh_graph_from_store(&mut self) -> Result<(), TrinityRewriteError> {
        self.graph = Graph::from_fixture(self.store.projection()?)?;
        Ok(())
    }

    fn dispatch(&mut self, operations: Vec<TrinityGraphOperation>) -> Result<(), TrinityRewriteError> {
        dispatch_trinity_graph_operations(&mut self.store, operations)?;
        self.refresh_graph_from_store()
    }

    pub fn undo(&mut self) -> Result<(), TrinityRewriteError> {
        use vcs::DocumentVcsCommand;
        self.store.dispatch(DocumentVcsCommand::Undo)?;
        self.refresh_graph_from_store()?;
        self.rebuild_engine();
        Ok(())
    }

    pub fn redo(&mut self) -> Result<(), TrinityRewriteError> {
        use vcs::DocumentVcsCommand;
        self.store.dispatch(DocumentVcsCommand::Redo)?;
        self.refresh_graph_from_store()?;
        self.rebuild_engine();
        Ok(())
    }

    pub fn commit_checkpoint(&mut self, message: Option<String>) -> Result<(), TrinityRewriteError> {
        use vcs::DocumentVcsCommand;
        self.store.dispatch(DocumentVcsCommand::CommitCheckpoint { message, authors: Vec::new() }).map_err(TrinityRewriteError::from)
    }

    pub fn store_generation(&self) -> u64 {
        self.store.generation()
    }

    pub fn fixture_json(&self) -> Result<String, TrinityRewriteError> {
        Ok(self.graph.fixture_json()?)
    }

    pub fn set_viewport(&mut self, width: u32, height: u32, dpr: f64) {
        self.width = width.max(1);
        self.height = height.max(1);
        self.dpr = dpr.max(1.0);
        self.board.set_size(self.width, self.height, self.dpr);
    }

    pub fn set_camera(&mut self, x: f64, y: f64, zoom: f64) {
        self.graph.camera.x = x;
        self.graph.camera.y = y;
        self.graph.camera.zoom = zoom;
        self.engine.set_camera(x, y, zoom);
        self.board.set_camera_silent(x, y, zoom);
    }

    pub fn set_canvas_theme_from_json(&mut self, json: &str) -> Result<(), TrinityRewriteError> {
        self.canvas_theme.merge_from_json(json).map_err(TrinityRewriteError::CanvasTheme)?;
        self.board.canvas_theme = self.canvas_theme.clone();
        Ok(())
    }

    pub fn pointer_down(&mut self, x: f64, y: f64, extend: bool) {
        let world = self.screen_to_world(x, y);
        self.engine.pointer_down(world.x, world.y, extend);
    }

    pub fn pointer_move(&mut self, x: f64, y: f64) {
        let world = self.screen_to_world(x, y);
        self.engine.pointer_move(world.x, world.y);
        self.sync_ephemeral_positions_from_engine();
    }

    pub fn pointer_up(&mut self, x: f64, y: f64) {
        let world = self.screen_to_world(x, y);
        self.engine.pointer_up(world.x, world.y);
        if let Err(err) = self.commit_drag_positions() {
            eprintln!("[DEBUG] trinity drag commit failed: {err}");
        }
        self.rebuild_engine();
    }

    pub fn reorganize(&mut self) {
        match force_layout_reposition_operations(&self.store.projection().unwrap_or_else(|_| self.graph.to_fixture())) {
            Ok(operations) if !operations.is_empty() => {
                if let Err(err) = self.dispatch(operations) {
                    eprintln!("[DEBUG] trinity reorganize dispatch failed: {err}");
                    return;
                }
                self.rebuild_engine();
            }
            Ok(_) => {}
            Err(err) => eprintln!("[DEBUG] trinity reorganize force layout failed: {err}"),
        }
    }

    pub fn run_jack(&mut self, query: &str) -> Result<QueryResult, TrinityRewriteError> {
        let parsed = parse(query).map_err(TrinityRewriteError::Jack)?;
        let (result, operations) = execute(&self.graph, &parsed).map_err(TrinityRewriteError::Jack)?;
        if !operations.is_empty() {
            self.dispatch(operations)?;
            self.rebuild_engine();
        }
        Ok(result)
    }

    pub fn run_jack_json(&mut self, query: &str) -> Result<String, TrinityRewriteError> {
        let result = self.run_jack(query)?;
        Ok(serde_json::to_string(&result)?)
    }

    pub fn run_jack_with_fixture_json(&mut self, query: &str) -> Result<String, TrinityRewriteError> {
        let result = self.run_jack(query)?;
        let fixture_json = self.fixture_json()?;
        let out = JackRunWithFixture { result, fixture_json };
        Ok(serde_json::to_string(&out)?)
    }

    pub fn tokenize_jack_json(&self, source: &str) -> Result<String, TrinityRewriteError> {
        let tokens = tokenize_jack(source);
        Ok(serde_json::to_string(&tokens)?)
    }

    pub fn complete_jack_json(&self, source: &str, cursor: usize) -> Result<String, TrinityRewriteError> {
        let items = complete_jack(&self.graph, source, cursor);
        Ok(serde_json::to_string(&items)?)
    }

    pub fn apply_rewrite_json(&mut self, rule_json: &str, bindings_json: &str) -> Result<String, TrinityRewriteError> {
        let rule: Rule = serde_json::from_str(rule_json)?;
        let bindings = parse_bindings_json(bindings_json)?;
        let query = build_rule_query(&rule, &bindings);
        let parsed = parse(&query).map_err(TrinityRewriteError::Jack)?;
        let (result, operations) = execute(&self.graph, &parsed).map_err(TrinityRewriteError::Jack)?;
        if !operations.is_empty() {
            self.dispatch(operations)?;
            self.rebuild_engine();
        }
        Ok(serde_json::to_string(&ApplyRuleResult { fixture: self.fixture_json()?, query: result })?)
    }

    pub fn node_overlays_json(&self) -> Result<String, TrinityRewriteError> {
        Ok("[]".into())
    }

    pub fn draw_lod_label(&self) -> &'static str {
        self.draw_lod_for_frame().label()
    }

    pub fn set_automatic_lod(&mut self, enabled: bool) {
        self.automatic_lod = enabled;
        self.board.set_automatic_lod(enabled);
    }

    pub fn set_forced_draw_lod_label(&mut self, label: &str) {
        self.forced_draw_lod = if label.is_empty() { None } else { TrinityDrawLod::from_id(label) };
        self.board.set_forced_draw_lod_label(label);
    }

    fn draw_lod_for_frame(&self) -> TrinityDrawLod {
        if !self.automatic_lod {
            if let Some(forced) = self.forced_draw_lod {
                return forced;
            }
        }
        TrinityDrawLod::from_scale_index(trinity_lod_index(self.graph.camera.zoom))
    }

    pub fn wheel_screen(&mut self, sx: f64, sy: f64, delta_y: f64) {
        use cavas::camera::{wheel_screen, Camera as CavasCamera, Viewport};
        let viewport = Viewport { width: self.width, height: self.height, dpr: self.dpr };
        let mut cam = CavasCamera { x: self.graph.camera.x, y: self.graph.camera.y, zoom: self.graph.camera.zoom };
        wheel_screen(&mut cam, &viewport, sx, sy, delta_y);
        self.set_camera(cam.x, cam.y, cam.zoom);
    }

    pub fn selected_node_ids_json(&self) -> Result<String, TrinityRewriteError> {
        let mut ids = Vec::new();
        for &nid in &self.engine.selection.node_ids {
            if let Some(tid) = self.node_id_map.get(&nid) {
                ids.push(tid.clone());
            }
        }
        for &hid in &self.engine.selection.handle_ids {
            if let Some(handle) = self.engine.handles.get(&hid) {
                if let Some(tid) = self.node_id_map.get(&handle.node_id) {
                    if !ids.iter().any(|row| row == tid) {
                        ids.push(tid.clone());
                    }
                }
            }
        }
        Ok(serde_json::to_string(&ids)?)
    }

    pub fn set_highlighted_node_ids_json(&mut self, json: &str) -> Result<(), TrinityRewriteError> {
        let ids: Vec<String> = serde_json::from_str(json)?;
        self.board.set_highlighted_ids(ids);
        Ok(())
    }

    fn screen_to_world(&self, sx: f64, sy: f64) -> cavas::Point {
        use cavas::camera::{screen_to_world, Camera as CavasCamera, Viewport};
        use cavas::Point;
        let cam = CavasCamera { x: self.graph.camera.x, y: self.graph.camera.y, zoom: self.graph.camera.zoom };
        let viewport = Viewport { width: self.width, height: self.height, dpr: self.dpr };
        screen_to_world(&cam, &viewport, Point::new(sx, sy))
    }

    fn sync_ephemeral_positions_from_engine(&mut self) {
        for (&nid, widget_id) in &self.node_id_map {
            if let Some(node) = self.engine.nodes.get(&nid) {
                if let Some(entry) = self.graph.nodes.get_mut(widget_id) {
                    entry.x = node.center.x;
                    entry.y = node.center.y;
                }
            }
        }
        self.sync_board_from_graph();
    }

    fn commit_drag_positions(&mut self) -> Result<(), TrinityRewriteError> {
        let projection = self.store.projection()?;
        let mut operations = Vec::new();
        for (nid, widget_id) in &self.node_id_map {
            let Some(engine_node) = self.engine.nodes.get(nid) else {
                continue;
            };
            let Some(fixture_node) = projection.nodes.iter().find(|node| node.id == *widget_id) else {
                continue;
            };
            if (fixture_node.x - engine_node.center.x).abs() > 1e-6 || (fixture_node.y - engine_node.center.y).abs() > 1e-6 {
                operations.push(TrinityGraphOperation::Reposition { id: widget_id.clone(), x: engine_node.center.x, y: engine_node.center.y });
            }
        }
        if operations.is_empty() {
            return Ok(());
        }
        self.dispatch(operations)
    }

    fn sync_positions_from_engine(&mut self) {
        self.sync_ephemeral_positions_from_engine();
    }

    fn sync_board_from_graph(&mut self) {
        let _ = self.board.set_board_kind_catalogs_from_json(TRINITY_BOARD_KIND_CATALOGS_JSON);
        let fixture = trinity_graph_to_board_fixture(&self.graph);
        if !self.board.parse_fixture_v1(&fixture) {
            eprintln!("[DEBUG] trinity board fixture parse failed");
        }
        self.board.set_size(self.width, self.height, self.dpr);
        self.board.canvas_theme = self.canvas_theme.clone();
        self.board.set_automatic_lod(self.automatic_lod);
        if let Some(lod) = self.forced_draw_lod {
            self.board.set_forced_draw_lod_label(lod.label());
        }
    }

    fn rebuild_engine(&mut self) {
        self.graph.recompute_derived();
        self.engine = TrinityBoardEngine::new();
        self.node_id_map.clear();
        self.handle_key_map.clear();
        self.edge_id_map.clear();
        let (cx, cy, zoom) = (self.graph.camera.x, self.graph.camera.y, self.graph.camera.zoom);
        self.engine.set_camera(cx, cy, zoom);
        let mut next_node: u64 = 1;
        let mut next_handle: u64 = 10;
        let mut handle_map: HashMap<String, u64> = HashMap::new();
        for node in self.graph.nodes.values() {
            let nid = next_node;
            next_node += 1;
            self.node_id_map.insert(nid, node.id.clone());
            let radius = trinity_node_radius(node);
            self.engine.create_node(nid, node.x, node.y, radius, true);
            let in_count = node.ports.iter().filter(|port| port.direction == PortDirection::In).count();
            let out_count = node.ports.iter().filter(|port| port.direction == PortDirection::Out).count();
            let mut in_idx = 0usize;
            let mut out_idx = 0usize;
            for port in &node.ports {
                let hid = next_handle;
                next_handle += 1;
                let angle = match port.direction {
                    PortDirection::In => {
                        let angle = trinity_circle_port_angle(in_idx, in_count, true);
                        in_idx += 1;
                        angle
                    }
                    PortDirection::Out => {
                        let angle = trinity_circle_port_angle(out_idx, out_count, false);
                        out_idx += 1;
                        angle
                    }
                };
                let public_key = port_key(&node.id, &port.id);
                handle_map.insert(trinity_port_handle_key(&node.id, &port.id, port.direction == PortDirection::In), hid);
                self.handle_key_map.insert(hid, public_key);
                self.engine.create_handle(hid, nid, angle);
                if let Some(handle) = self.engine.handles.get_mut(&hid) {
                    handle.radius = TRINITY_HANDLE_RADIUS;
                }
                self.engine.set_handle_role(hid, if port.direction == PortDirection::In { HandleRole::Target } else { HandleRole::Source });
            }
        }
        let mut eid: u64 = 100;
        for edge in self.graph.edges.values() {
            let (source_node, source_port) = trinity_port_endpoint_parts(&edge.source);
            let (target_node, target_port) = trinity_port_endpoint_parts(&edge.target);
            let src = handle_map.get(&trinity_port_handle_key(&source_node, &source_port, false)).copied();
            let tgt = handle_map.get(&trinity_port_handle_key(&target_node, &target_port, true)).copied();
            if let (Some(s), Some(t)) = (src, tgt) {
                self.engine.create_edge(eid, s, t);
                self.edge_id_map.insert(eid, edge.id.clone());
                eid += 1;
            }
        }
        self.engine.set_next_edge_id(eid);
        self.sync_board_from_graph();
    }

    pub fn paint_scene(&self, scene: &mut cavas::Scene, _viewport_w: u32, _viewport_h: u32, _dpr: f64) {
        let lod_index = trinity_lod_index(self.graph.camera.zoom) as i8;
        if self.last_logged_lod.get() != lod_index {
            self.last_logged_lod.set(lod_index);
        }
        let board_scene = self.board.build_vector_scene();
        scene.append(&board_scene, None);
        let _ = distance_between;
        let _ = compute_edge_bezier_points;
    }
}

fn trinity_port_endpoint_parts(endpoint: &str) -> (String, String) {
    endpoint.split_once(':').map(|(n, p)| (n.to_string(), p.to_string())).unwrap_or_else(|| (endpoint.to_string(), String::new()))
}

fn trinity_port_handle_key(node_id: &str, port_id: &str, input: bool) -> String {
    format!("{}:{}:{}", node_id, if input { "in" } else { "out" }, port_id)
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct JackRunWithFixture {
    #[serde(flatten)]
    result: QueryResult,
    fixture_json: String,
}
// #endregion 🔖TrinityHost

//#region 🔖WasmBridge
#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
    use std::cell::RefCell;
    use trinity_ram::{create_trinity_graph_envelope, empty_trinity_graph_fixture, TrinityGraphEnvelope, TrinityGraphStore};
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct TrinityRewriteDocumentVcs {
        store: RefCell<TrinityGraphStore>,
    }

    #[wasm_bindgen]
    impl TrinityRewriteDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<TrinityRewriteDocumentVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: TrinityGraphEnvelope = serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    TrinityGraphStore::new(envelope)
                }
                None => TrinityGraphStore::new(create_trinity_graph_envelope("trinity-rewrite", empty_trinity_graph_fixture())),
            };
            Ok(Self { store: RefCell::new(store) })
        }

        #[wasm_bindgen(js_name = dispatchJson)]
        pub fn dispatch_json(&self, command_json: &str) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_json(command_json).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = projectionJson)]
        pub fn projection_json(&self) -> Result<String, JsValue> {
            self.store.borrow().projection_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = envelopeJson)]
        pub fn envelope_json(&self) -> Result<String, JsValue> {
            self.store.borrow().envelope_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = generation)]
        pub fn generation(&self) -> u32 {
            self.store.borrow().generation() as u32
        }
    }
}
//#endregion 🔖WasmBridge

// #region 🔖WasmSession
#[cfg(target_arch = "wasm32")]
mod wasm_session {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;
    use trinity_ram::GraphFixture;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_futures::future_to_promise;
    use web_sys::HtmlCanvasElement;

    struct TrinitySessionInner {
        host: TrinityHost,
        gpu: cavas::gpu_session::CanvasGpuSession,
        width: u32,
        height: u32,
        dpr: f64,
    }

    #[wasm_bindgen]
    pub struct TrinitySession {
        state: Rc<RefCell<TrinitySessionInner>>,
    }

    #[wasm_bindgen]
    impl TrinitySession {
        #[wasm_bindgen(constructor)]
        pub fn new() -> Self {
            let fixture = include_str!("../../../example/nakagin-capsule-tower.trinity.json");
            let host = TrinityHost::load_fixture_json(fixture).unwrap_or_else(|_| {
                let empty =
                    GraphFixture { schema: GraphFixture::SCHEMA.into(), name: "empty".into(), manifest_id: Some("nakagin".into()), manifest: Manifest::nakagin_default(), camera: Camera::default(), nodes: vec![], edges: vec![], root_node_id: None };
                TrinityHost::from_graph(Graph::from_fixture(empty).expect("hardcoded empty fixture with a compile-time-valid manifest id is always graph-valid"))
            });
            Self { state: Rc::new(RefCell::new(TrinitySessionInner { host, gpu: cavas::gpu_session::CanvasGpuSession::default(), width: 1, height: 1, dpr: 1.0 })) }
        }

        #[wasm_bindgen(js_name = loadFixtureJson)]
        pub fn load_fixture_json(&self, json: &str) -> Result<(), JsValue> {
            let host = TrinityHost::load_fixture_json(json).map_err(|e| JsValue::from_str(&e.to_string()))?;
            self.state.borrow_mut().host = host;
            Ok(())
        }

        #[wasm_bindgen(js_name = fixtureJson)]
        pub fn fixture_json(&self) -> Result<String, JsValue> {
            self.state.borrow().host.fixture_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = nodeOverlaysJson)]
        pub fn node_overlays_json(&self) -> Result<String, JsValue> {
            self.state.borrow().host.node_overlays_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = attachCanvas)]
        pub fn attach_canvas(&mut self, canvas: HtmlCanvasElement, logical_w: u32, logical_h: u32, dpr: f64) -> js_sys::Promise {
            let inner = self.state.clone();
            let lw = logical_w.max(1);
            let lh = logical_h.max(1);
            let dpr = dpr.max(1.0);
            let pw = ((lw as f64 * dpr).round() as u32).max(1);
            let ph = ((lh as f64 * dpr).round() as u32).max(1);
            future_to_promise(async move {
                let (render_ctx, renderer, surface) = cavas::gpu_session::CanvasGpuSession::create_canvas_surface(canvas.clone(), pw, ph).await.map_err(|err| JsValue::from_str(&err))?;
                let mut g = inner.borrow_mut();
                g.width = lw;
                g.height = lh;
                g.dpr = dpr;
                g.host.set_viewport(lw, lh, dpr);
                g.gpu.finish_attach(canvas, render_ctx, renderer, surface);
                Ok(JsValue::UNDEFINED)
            })
        }

        #[wasm_bindgen(js_name = gpuReady)]
        pub fn gpu_ready(&self) -> bool {
            self.state.borrow().gpu.gpu_ready()
        }

        #[wasm_bindgen(js_name = detachGpu)]
        pub fn detach_gpu(&mut self) {
            self.state.borrow_mut().gpu.detach();
        }

        #[wasm_bindgen(js_name = setSize)]
        pub fn set_size(&mut self, width: u32, height: u32, dpr: f64) {
            let mut inner = self.state.borrow_mut();
            inner.width = width.max(1);
            inner.height = height.max(1);
            inner.dpr = dpr.max(1.0);
            let (w, h, d) = (inner.width, inner.height, inner.dpr);
            inner.host.set_viewport(w, h, d);
            let pw = ((w as f64 * d).round() as u32).max(1);
            let ph = ((h as f64 * d).round() as u32).max(1);
            inner.gpu.resize_surface(pw, ph);
        }

        #[wasm_bindgen(js_name = setCamera)]
        pub fn set_camera(&self, x: f64, y: f64, zoom: f64) {
            self.state.borrow_mut().host.set_camera(x, y, zoom);
        }

        #[wasm_bindgen(js_name = lodScaleJson)]
        pub fn lod_scale_json(&self) -> String {
            trinity_lod_scale_json()
        }

        #[wasm_bindgen(js_name = setAutomaticLod)]
        pub fn set_automatic_lod(&self, enabled: bool) {
            self.state.borrow_mut().host.set_automatic_lod(enabled);
        }

        #[wasm_bindgen(js_name = setForcedDrawLodLabel)]
        pub fn set_forced_draw_lod_label(&self, label: &str) {
            self.state.borrow_mut().host.set_forced_draw_lod_label(label);
        }

        #[wasm_bindgen(js_name = drawLodLabel)]
        pub fn draw_lod_label(&self) -> String {
            self.state.borrow().host.draw_lod_label().to_string()
        }

        #[wasm_bindgen(js_name = pointerDown)]
        pub fn pointer_down(&self, x: f64, y: f64, extend: bool) {
            self.state.borrow_mut().host.pointer_down(x, y, extend);
        }

        #[wasm_bindgen(js_name = pointerMove)]
        pub fn pointer_move(&self, x: f64, y: f64) {
            self.state.borrow_mut().host.pointer_move(x, y);
        }

        #[wasm_bindgen(js_name = pointerUp)]
        pub fn pointer_up(&self, x: f64, y: f64) {
            self.state.borrow_mut().host.pointer_up(x, y);
        }

        #[wasm_bindgen(js_name = wheelScreen)]
        pub fn wheel_screen(&self, x: f64, y: f64, delta_y: f64) {
            self.state.borrow_mut().host.wheel_screen(x, y, delta_y);
        }

        #[wasm_bindgen(js_name = selectedNodeIdsJson)]
        pub fn selected_node_ids_json(&self) -> Result<String, JsValue> {
            self.state.borrow().host.selected_node_ids_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = setHighlightedNodeIdsJson)]
        pub fn set_highlighted_node_ids_json(&mut self, json: &str) -> Result<(), JsValue> {
            self.state.borrow_mut().host.set_highlighted_node_ids_json(json).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = reorganize)]
        pub fn reorganize(&self, _options_json: &str) -> Result<(), JsValue> {
            self.state.borrow_mut().host.reorganize();
            Ok(())
        }

        #[wasm_bindgen(js_name = setCanvasThemeJson)]
        pub fn set_canvas_theme_json(&mut self, json: &str) {
            let _ = self.state.borrow_mut().host.set_canvas_theme_from_json(json);
        }

        #[wasm_bindgen(js_name = renderFrame)]
        pub fn render_frame(&self) -> Result<(), JsValue> {
            let mut inner = self.state.borrow_mut();
            let clear = inner.host.canvas_theme.raster_clear;
            let scene = inner.host.board.build_vector_scene();
            inner.gpu.render_frame(&scene, clear)
        }

        #[wasm_bindgen(js_name = runJackJson)]
        pub fn run_jack_json(&self, query: &str) -> Result<String, JsValue> {
            self.state.borrow_mut().host.run_jack_json(query).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = runJackJsonWithFixture)]
        pub fn run_jack_json_with_fixture(&self, query: &str) -> Result<String, JsValue> {
            self.state.borrow_mut().host.run_jack_with_fixture_json(query).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = tokenizeJackJson)]
        pub fn tokenize_jack_json(&self, source: &str) -> Result<String, JsValue> {
            self.state.borrow().host.tokenize_jack_json(source).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = completeJackJson)]
        pub fn complete_jack_json(&self, source: &str, cursor: usize) -> Result<String, JsValue> {
            self.state.borrow().host.complete_jack_json(source, cursor).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = applyRewriteJson)]
        pub fn apply_rewrite_json(&self, rule_json: &str, bindings_json: &str) -> Result<String, JsValue> {
            self.state.borrow_mut().host.apply_rewrite_json(rule_json, bindings_json).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = undo)]
        pub fn undo(&self) -> Result<(), JsValue> {
            self.state.borrow_mut().host.undo().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = redo)]
        pub fn redo(&self) -> Result<(), JsValue> {
            self.state.borrow_mut().host.redo().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = commitCheckpoint)]
        pub fn commit_checkpoint(&self, message: &str) -> Result<(), JsValue> {
            let message = if message.is_empty() { None } else { Some(message.to_string()) };
            self.state.borrow_mut().host.commit_checkpoint(message).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = storeGeneration)]
        pub fn store_generation(&self) -> u64 {
            self.state.borrow().host.store_generation()
        }
    }

    #[wasm_bindgen(js_name = ruleQueryJson)]
    pub fn rule_query_json(rule_json: &str, bindings_json: &str) -> Result<String, JsValue> {
        super::rule_query_json(rule_json, bindings_json).map_err(|e| JsValue::from_str(&e.to_string()))
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm_session::TrinitySession;
// #endregion 🔖WasmSession

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn nakagin_graph() -> Graph {
        let json = include_str!("../../../example/nakagin-capsule-tower.trinity.json");
        let mut g = Graph::load_json(json).unwrap();
        g.recompute_derived();
        g
    }

    #[test]
    fn nakagin_fixture_loads() {
        let g = nakagin_graph();
        assert_eq!(g.nodes.len(), 9);
        assert_eq!(g.edges.len(), 6);
    }

    #[test]
    fn nakagin_flat_position_derived() {
        let g = nakagin_graph();
        let root = g.node("7dc5b737-3b6b-4068-b315-b7bacc91c2e1").unwrap();
        let flat = root.properties.get("flatPosition").unwrap().as_object().unwrap();
        assert_eq!(flat.get("u").and_then(PropertyValue::as_f64), Some(0.0));
        let capsule = g.node("6947a41b-8c6d-4291-bdd8-96cd535c78fc").unwrap();
        let cflat = capsule.properties.get("flatPosition").unwrap().as_object().unwrap();
        assert!(cflat.get("v").and_then(PropertyValue::as_f64).unwrap_or(0.0) > 0.0);
    }

    #[test]
    fn jack_query_on_nakagin() {
        let mut g = nakagin_graph();
        let result = run_jack(&mut g, "MATCH (a:Piece) WHERE a.name = 'b' RETURN a.name").unwrap();
        assert_eq!(result.rows.len(), 1);
    }

    #[test]
    fn rewrite_rule_labels_core() {
        let mut g = nakagin_graph();
        let rule = Rule {
            name: "label-core".into(),
            lhs: Lhs { pattern: PatternJson { left_var: "a".into(), left_kind: "Piece".into(), edge_var: None, edge_kind: None, right_var: None, right_kind: None }, where_clause: Some("a.name = 'b'".into()) },
            rhs: Rhs { create: vec![], delete: vec![], set: vec![AssignmentJson { var: "a".into(), prop: "label".into(), value: PropertyValue::String("nakagin-core".into()) }], merge: vec![], parameters: vec![] },
        };
        apply_rule(&mut g, &rule, &HashMap::new()).unwrap();
        let core = g.node("7dc5b737-3b6b-4068-b315-b7bacc91c2e1").unwrap();
        assert_eq!(core.properties.get("label"), Some(&PropertyValue::String("nakagin-core".into())));
    }

    #[test]
    fn rewrite_rule_parameter_substitution() {
        let mut g = nakagin_graph();
        let rule = Rule {
            name: "label-core".into(),
            lhs: Lhs { pattern: PatternJson { left_var: "a".into(), left_kind: "Piece".into(), edge_var: None, edge_kind: None, right_var: None, right_kind: None }, where_clause: Some("a.name = 'b'".into()) },
            rhs: Rhs {
                create: vec![],
                delete: vec![],
                set: vec![AssignmentJson { var: "a".into(), prop: "label".into(), value: PropertyValue::String("$label".into()) }],
                merge: vec![],
                parameters: vec![ParameterSpec { name: "label".into(), kind: ParameterKind::String, default: PropertyValue::String("nakagin-core".into()) }],
            },
        };
        apply_rule(&mut g, &rule, &HashMap::new()).unwrap();
        let core = g.node("7dc5b737-3b6b-4068-b315-b7bacc91c2e1").unwrap();
        assert_eq!(core.properties.get("label"), Some(&PropertyValue::String("nakagin-core".into())));

        let mut g2 = nakagin_graph();
        let mut bindings = HashMap::new();
        bindings.insert("label".into(), PropertyValue::String("override-core".into()));
        apply_rule(&mut g2, &rule, &bindings).unwrap();
        let core2 = g2.node("7dc5b737-3b6b-4068-b315-b7bacc91c2e1").unwrap();
        assert_eq!(core2.properties.get("label"), Some(&PropertyValue::String("override-core".into())));

        let query = build_rule_query(&rule, &bindings);
        assert!(query.contains("SET a.label = 'override-core'"));
    }

    #[test]
    fn rewrite_labeled_fixture_reloads() {
        let json = include_str!("../../../example/nakagin-capsule-tower.trinity.json");
        let mut g = Graph::load_json(json).unwrap();
        let rule = Rule {
            name: "label-core".into(),
            lhs: Lhs { pattern: PatternJson { left_var: "a".into(), left_kind: "Piece".into(), edge_var: None, edge_kind: None, right_var: None, right_kind: None }, where_clause: Some("a.name = 'b'".into()) },
            rhs: Rhs { create: vec![], delete: vec![], set: vec![AssignmentJson { var: "a".into(), prop: "label".into(), value: PropertyValue::String("nakagin-core".into()) }], merge: vec![], parameters: vec![] },
        };
        apply_rule(&mut g, &rule, &HashMap::new()).unwrap();
        let fixture_json = g.fixture_json().unwrap();
        let reloaded = Graph::load_json(&fixture_json).unwrap();
        let core = reloaded.node("7dc5b737-3b6b-4068-b315-b7bacc91c2e1").unwrap();
        assert_eq!(core.properties.get("label"), Some(&PropertyValue::String("nakagin-core".into())));
    }

    #[test]
    fn trinity_host_rebuilds_engine() {
        let host = TrinityHost::from_graph(nakagin_graph());
        assert_eq!(host.engine.nodes.len(), 9);
        assert!(!host.engine.edges.is_empty());
        assert!(!host.engine.enforce_acyclic);
        assert_eq!(host.board.nodes.len(), 9);
        assert!(host.board.nodes.values().all(|node| matches!(node.shape, infinite_board_port_directed::NodeShape::Circle)));
    }

    #[test]
    fn trinity_host_reorganize_moves_nodes() {
        let mut host = TrinityHost::from_graph(nakagin_graph());
        let before: Vec<(f64, f64)> = host.graph.nodes.values().map(|n| (n.x, n.y)).collect();
        host.reorganize();
        let after: Vec<(f64, f64)> = host.graph.nodes.values().map(|n| (n.x, n.y)).collect();
        assert_ne!(before, after);
    }

    #[test]
    fn trinity_host_tokenize_jack_json() {
        let host = TrinityHost::from_graph(nakagin_graph());
        let json = host.tokenize_jack_json("MATCH (a:Piece)").unwrap();
        let tokens: Vec<JackTokenSpan> = serde_json::from_str(&json).unwrap();
        assert!(tokens.iter().any(|row| row.start == 0));
    }

    #[test]
    fn trinity_host_complete_jack_json() {
        let host = TrinityHost::from_graph(nakagin_graph());
        let json = host.complete_jack_json("MAT", 3).unwrap();
        let items: Vec<JackCompletion> = serde_json::from_str(&json).unwrap();
        assert!(items.iter().any(|row| row.label == "MATCH"));
    }

    #[test]
    fn trinity_host_jack_create_undo() {
        let mut host = TrinityHost::from_graph(nakagin_graph());
        let before = host.graph.nodes.len();
        host.run_jack("CREATE (n:Piece)").unwrap();
        assert_eq!(host.graph.nodes.len(), before + 1);
        host.undo().unwrap();
        assert_eq!(host.graph.nodes.len(), before);
    }
}
// #endregion 🔖Tests

pub use trinity_ram::TRINITY_GRAPH_SCHEMA;
