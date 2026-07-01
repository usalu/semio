//! ♻️ Parametric graph rewriting for trinity graphs with optional WASM canvas host.

pub use infinite_cavas as cavas;
use mathematical_graph_port_directed::{
    geometry::{compute_edge_bezier_points, distance_between, handle_outward_at_node_rim, handle_exterior_cap_fill_path, handle_exterior_cap_stroke_path},
    BoardEngine, HandleRole, VelloThemePalette,
};
use serde::{Deserialize, Serialize};
use std::cell::Cell;
use std::collections::HashMap;
use trinity_jack::{execute, parse};
use trinity_ram::{Graph, PortDirection, PropertyValue, port_key};

pub use trinity_jack::{
    complete as complete_jack, parse as parse_jack, run as run_jack, run_json as run_jack_json, tokenize as tokenize_jack,
    Completion as JackCompletion, Pattern, QueryResult, QueryResultKind, TokenSpan as JackTokenSpan,
};
pub use trinity_ram::{self, CameraV1, Manifest};

type TrinityBoardEngine = BoardEngine;

const TRINITY_HANDLE_RADIUS: f64 = 5.0;
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
            Pattern {
                nodes: vec![left],
                edge: Some(PatternEdge {
                    var: self.edge_var.clone(),
                    kind: self.edge_kind.clone(),
                    directed: true,
                    right: PatternNode { var: right_var.clone(), kind: right_kind.clone() },
                }),
            }
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
        format!(
            "({}:{} )-{edge_mid}->({}:{} )",
            p.nodes[0].var, p.nodes[0].kind, edge.right.var, edge.right.kind
        )
        .replace(" )", ")")
    } else {
        left
    }
}

fn parse_bindings_json(bindings_json: &str) -> Result<HashMap<String, PropertyValue>, String> {
    if bindings_json.trim().is_empty() {
        return Ok(HashMap::new());
    }
    serde_json::from_str(bindings_json).map_err(|e| e.to_string())
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
pub fn apply_rule(graph: &mut Graph, rule: &Rule, bindings: &HashMap<String, PropertyValue>) -> Result<QueryResult, String> {
    let query = build_rule_query(rule, bindings);
    execute(graph, &parse(&query)?)
}

/// ♻️ Apply a rewrite rule from JSON.
pub fn apply_rule_json(graph: &mut Graph, rule_json: &str, bindings_json: &str) -> Result<String, String> {
    let rule: Rule = serde_json::from_str(rule_json).map_err(|e| e.to_string())?;
    let bindings = parse_bindings_json(bindings_json)?;
    let result = apply_rule(graph, &rule, &bindings)?;
    Ok(serde_json::to_string(&ApplyRuleResult { fixture: graph.fixture_json()?, query: result }).map_err(|e| e.to_string())?)
}

/// 🧵 Build a rewrite rule Jack query from JSON without a graph.
pub fn rule_query_json(rule_json: &str, bindings_json: &str) -> Result<String, String> {
    let rule: Rule = serde_json::from_str(rule_json).map_err(|e| e.to_string())?;
    let bindings = parse_bindings_json(bindings_json)?;
    let query = build_rule_query(&rule, &bindings);
    Ok(serde_json::to_string(&RuleQueryResult { query }).map_err(|e| e.to_string())?)
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
// #endregion 🔖Lod

// #region 🔖TrinityHost
/// 🖥️ Retained trinity graph host on the directed port board engine.
pub struct TrinityHost {
    pub graph: Graph,
    pub engine: TrinityBoardEngine,
    pub vello_theme: VelloThemePalette,
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
        let mut host = Self {
            graph,
            engine: TrinityBoardEngine::new(),
            vello_theme: VelloThemePalette::default(),
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

    pub fn load_fixture_json(json: &str) -> Result<Self, String> {
        let graph = Graph::load_json(json)?;
        Ok(Self::from_graph(graph))
    }

    pub fn fixture_json(&self) -> Result<String, String> {
        self.graph.fixture_json()
    }

    pub fn set_viewport(&mut self, width: u32, height: u32, dpr: f64) {
        self.width = width.max(1);
        self.height = height.max(1);
        self.dpr = dpr.max(1.0);
    }

    pub fn set_camera(&mut self, x: f64, y: f64, zoom: f64) {
        self.graph.camera.x = x;
        self.graph.camera.y = y;
        self.graph.camera.zoom = zoom;
        self.engine.set_camera(x, y, zoom);
    }

    pub fn set_vello_theme_from_json(&mut self, json: &str) -> Result<(), String> {
        self.vello_theme.merge_from_json(json)
    }

    pub fn pointer_down(&mut self, x: f64, y: f64, extend: bool) {
        let world = self.screen_to_world(x, y);
        self.engine.pointer_down(world.x, world.y, extend);
        self.sync_positions_from_engine();
    }

    pub fn pointer_move(&mut self, x: f64, y: f64) {
        let world = self.screen_to_world(x, y);
        self.engine.pointer_move(world.x, world.y);
        self.sync_positions_from_engine();
    }

    pub fn pointer_up(&mut self, x: f64, y: f64) {
        let world = self.screen_to_world(x, y);
        self.engine.pointer_up(world.x, world.y);
        self.sync_positions_from_engine();
    }

    pub fn reorganize(&mut self) {
        for (i, node) in self.graph.nodes.values_mut().enumerate() {
            node.x = i as f64 * 140.0 - 200.0;
            node.y = (i % 3) as f64 * 100.0;
        }
        self.rebuild_engine();
    }

    pub fn run_jack(&mut self, query: &str) -> Result<QueryResult, String> {
        run_jack(&mut self.graph, query)
    }

    pub fn run_jack_json(&mut self, query: &str) -> Result<String, String> {
        run_jack_json(&mut self.graph, query)
    }

    pub fn run_jack_with_fixture_json(&mut self, query: &str) -> Result<String, String> {
        let result = run_jack(&mut self.graph, query)?;
        self.rebuild_engine();
        let fixture_json = self.fixture_json()?;
        let out = JackRunWithFixture { result, fixture_json };
        serde_json::to_string(&out).map_err(|e| e.to_string())
    }

    pub fn tokenize_jack_json(&self, source: &str) -> Result<String, String> {
        let tokens = tokenize_jack(source);
        serde_json::to_string(&tokens).map_err(|e| e.to_string())
    }

    pub fn complete_jack_json(&self, source: &str, cursor: usize) -> Result<String, String> {
        let items = complete_jack(&self.graph, source, cursor);
        serde_json::to_string(&items).map_err(|e| e.to_string())
    }

    pub fn apply_rewrite_json(&mut self, rule_json: &str, bindings_json: &str) -> Result<String, String> {
        let out = apply_rule_json(&mut self.graph, rule_json, bindings_json)?;
        self.rebuild_engine();
        Ok(out)
    }

    pub fn node_overlays_json(&self) -> Result<String, String> {
        Ok("[]".into())
    }

    pub fn draw_lod_label(&self) -> &'static str {
        self.draw_lod_for_frame().label()
    }

    pub fn set_automatic_lod(&mut self, enabled: bool) {
        self.automatic_lod = enabled;
    }

    pub fn set_forced_draw_lod_label(&mut self, label: &str) {
        self.forced_draw_lod = if label.is_empty() { None } else { TrinityDrawLod::from_id(label) };
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
        let mut cam = CavasCamera {
            x: self.graph.camera.x,
            y: self.graph.camera.y,
            zoom: self.graph.camera.zoom,
        };
        wheel_screen(&mut cam, &viewport, sx, sy, delta_y);
        self.set_camera(cam.x, cam.y, cam.zoom);
    }

    pub fn selected_node_ids_json(&self) -> Result<String, String> {
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
        serde_json::to_string(&ids).map_err(|e| e.to_string())
    }

    fn screen_to_world(&self, sx: f64, sy: f64) -> cavas::vello::kurbo::Point {
        use cavas::camera::{screen_to_world, Camera as CavasCamera, Viewport};
        use cavas::vello::kurbo::Point;
        let cam = CavasCamera { x: self.graph.camera.x, y: self.graph.camera.y, zoom: self.graph.camera.zoom };
        let viewport = Viewport { width: self.width, height: self.height, dpr: self.dpr };
        screen_to_world(&cam, &viewport, Point::new(sx, sy))
    }

    fn sync_positions_from_engine(&mut self) {
        for (&nid, widget_id) in &self.node_id_map {
            if let Some(node) = self.engine.nodes.get(&nid) {
                if let Some(entry) = self.graph.nodes.get_mut(widget_id) {
                    entry.x = node.center.x;
                    entry.y = node.center.y;
                }
            }
        }
    }

    fn rebuild_engine(&mut self) {
        self.graph.recompute_derived();
        self.engine = TrinityBoardEngine::new();
        self.engine.enforce_acyclic = true;
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
            let w = if node.width > 0.0 { node.width } else { 88.0 };
            let h = if node.height > 0.0 { node.height } else { 40.0 };
            self.engine.create_rect_node(nid, node.x, node.y, w, h, true);
            let mut in_idx = 0usize;
            let mut out_idx = 0usize;
            for port in &node.ports {
                let hid = next_handle;
                next_handle += 1;
                let angle = match port.direction {
                    PortDirection::In => std::f64::consts::FRAC_PI_2 + (in_idx as f64 * 0.2),
                    PortDirection::Out => -std::f64::consts::FRAC_PI_2 - (out_idx as f64 * 0.2),
                };
                if port.direction == PortDirection::In {
                    in_idx += 1;
                } else {
                    out_idx += 1;
                }
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
    }

    pub fn paint_scene(&self, scene: &mut cavas::vello::Scene, viewport_w: u32, viewport_h: u32, dpr: f64) {
        use cavas::camera::{camera_content_affine, world_to_screen, Camera as CavasCamera, Viewport};
        use cavas::text::append_label;
        use cavas::vello::kurbo::{Circle, Point, Rect, Stroke};
        use cavas::vello::peniko::Fill;

        let theme = &self.vello_theme;
        let cam = CavasCamera { x: self.graph.camera.x, y: self.graph.camera.y, zoom: self.graph.camera.zoom };
        let viewport = Viewport { width: viewport_w.max(1), height: viewport_h.max(1), dpr: dpr.max(1.0) };
        let aff = camera_content_affine(&cam, &viewport);
        let lod = self.draw_lod_for_frame();
        let lod_index = trinity_lod_index(cam.zoom) as i8;
        if self.last_logged_lod.get() != lod_index {
            self.last_logged_lod.set(lod_index);
        }
        let edge_stroke = TRINITY_EDGE_STROKE / cam.zoom.max(0.05);
        let show_handles = lod.handles_visible();
        let show_labels = lod.labels_visible();
        let label_px = (12.0 / cam.zoom.max(0.05)).clamp(8.0, 18.0);
        let snap = self.engine.render_snapshot();
        for (&eid, _) in &self.engine.edges {
            if let Some(curve) = self.engine.edge_curve(eid) {
                scene.stroke(&Stroke::new(edge_stroke), aff, theme.edge_stroke, None, &curve);
            }
        }
        for node in self.graph.nodes.values() {
            let hw = if node.width > 0.0 { node.width } else { 88.0 } * 0.5;
            let hh = if node.height > 0.0 { node.height } else { 40.0 } * 0.5;
            let rect = Rect::new(node.x - hw, node.y - hh, node.x + hw, node.y + hh);
            scene.fill(Fill::NonZero, aff, theme.node_fill, None, &rect);
            scene.stroke(&Stroke::new(edge_stroke), aff, theme.node_stroke, None, &rect);
            if show_labels {
                let label = if lod.full_labels() {
                    node.name.clone()
                } else {
                    trinity_abbreviate_label(&node.name)
                };
                let screen = world_to_screen(&cam, &viewport, Point::new(node.x, node.y - hh + 4.0 / cam.zoom.max(0.05)));
                append_label(scene, &label, screen, label_px, theme.label_fill, theme.label_halo);
            }
        }
        if show_handles {
            for (hid, center, _radius) in &snap.handles {
                let node_id = self.engine.handles.get(hid).map(|h| h.node_id);
                let outward = node_id.and_then(|nid| {
                    self.engine.nodes.get(&nid).and_then(|node| {
                        handle_outward_at_node_rim(*center, node.center, node.shape, node.radius, node.width, node.height)
                    })
                });
                if let Some(out) = outward {
                    scene.fill(Fill::NonZero, aff, theme.handle_fill, None, &handle_exterior_cap_fill_path(*center, out, TRINITY_HANDLE_RADIUS));
                    scene.stroke(&Stroke::new(edge_stroke), aff, theme.handle_stroke, None, &handle_exterior_cap_stroke_path(*center, out, TRINITY_HANDLE_RADIUS));
                } else {
                    let circle = Circle::new(*center, TRINITY_HANDLE_RADIUS);
                    scene.fill(Fill::NonZero, aff, theme.handle_fill, None, &circle);
                    scene.stroke(&Stroke::new(edge_stroke), aff, theme.handle_stroke, None, &circle);
                }
            }
        }
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

// #region 🔖WasmSession
#[cfg(target_arch = "wasm32")]
mod wasm_session {
    use super::*;
    use trinity_ram::GraphFixtureV1;
    use std::cell::RefCell;
    use std::rc::Rc;
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
            let fixture = include_str!("../../fixture/nakagin-capsule-tower.trinity.json");
            let host = TrinityHost::load_fixture_json(fixture).unwrap_or_else(|_| TrinityHost::from_graph(Graph::from_fixture(GraphFixtureV1 { schema: GraphFixtureV1::SCHEMA.into(), name: "empty".into(), manifest_id: Some("nakagin".into()), manifest: Manifest::nakagin_default(), camera: CameraV1::default(), nodes: vec![], edges: vec![], root_node_id: None }).unwrap()));
            Self { state: Rc::new(RefCell::new(TrinitySessionInner { host, gpu: cavas::gpu_session::CanvasGpuSession::default(), width: 1, height: 1, dpr: 1.0 })) }
        }

        #[wasm_bindgen(js_name = loadFixtureJson)]
        pub fn load_fixture_json(&self, json: &str) -> Result<(), JsValue> {
            let host = TrinityHost::load_fixture_json(json).map_err(|e| JsValue::from_str(&e))?;
            self.state.borrow_mut().host = host;
            Ok(())
        }

        #[wasm_bindgen(js_name = fixtureJson)]
        pub fn fixture_json(&self) -> Result<String, JsValue> {
            self.state.borrow().host.fixture_json().map_err(|e| JsValue::from_str(&e))
        }

        #[wasm_bindgen(js_name = nodeOverlaysJson)]
        pub fn node_overlays_json(&self) -> Result<String, JsValue> {
            self.state.borrow().host.node_overlays_json().map_err(|e| JsValue::from_str(&e))
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
            self.state.borrow().host.selected_node_ids_json().map_err(|e| JsValue::from_str(&e))
        }

        #[wasm_bindgen(js_name = reorganize)]
        pub fn reorganize(&self, _options_json: &str) -> Result<(), JsValue> {
            self.state.borrow_mut().host.reorganize();
            Ok(())
        }

        #[wasm_bindgen(js_name = setVelloThemeJson)]
        pub fn set_vello_theme_json(&mut self, json: &str) {
            let _ = self.state.borrow_mut().host.set_vello_theme_from_json(json);
        }

        #[wasm_bindgen(js_name = renderFrame)]
        pub fn render_frame(&self) -> Result<(), JsValue> {
            let mut inner = self.state.borrow_mut();
            let mut scene = cavas::vello::Scene::new();
            let clear = inner.host.vello_theme.raster_clear;
            inner.host.paint_scene(&mut scene, inner.width, inner.height, inner.dpr);
            let scene = cavas::render::scale_scene_for_device_pixel_ratio(scene, inner.dpr);
            inner.gpu.render_frame(&scene, clear)
        }

        #[wasm_bindgen(js_name = runJackJson)]
        pub fn run_jack_json(&self, query: &str) -> Result<String, JsValue> {
            self.state.borrow_mut().host.run_jack_json(query).map_err(|e| JsValue::from_str(&e))
        }

        #[wasm_bindgen(js_name = runJackJsonWithFixture)]
        pub fn run_jack_json_with_fixture(&self, query: &str) -> Result<String, JsValue> {
            self.state
                .borrow_mut()
                .host
                .run_jack_with_fixture_json(query)
                .map_err(|e| JsValue::from_str(&e))
        }

        #[wasm_bindgen(js_name = tokenizeJackJson)]
        pub fn tokenize_jack_json(&self, source: &str) -> Result<String, JsValue> {
            self.state.borrow().host.tokenize_jack_json(source).map_err(|e| JsValue::from_str(&e))
        }

        #[wasm_bindgen(js_name = completeJackJson)]
        pub fn complete_jack_json(&self, source: &str, cursor: usize) -> Result<String, JsValue> {
            self.state
                .borrow()
                .host
                .complete_jack_json(source, cursor)
                .map_err(|e| JsValue::from_str(&e))
        }

        #[wasm_bindgen(js_name = applyRewriteJson)]
        pub fn apply_rewrite_json(&self, rule_json: &str, bindings_json: &str) -> Result<String, JsValue> {
            self.state
                .borrow_mut()
                .host
                .apply_rewrite_json(rule_json, bindings_json)
                .map_err(|e| JsValue::from_str(&e))
        }
    }

    #[wasm_bindgen(js_name = ruleQueryJson)]
    pub fn rule_query_json(rule_json: &str, bindings_json: &str) -> Result<String, JsValue> {
        super::rule_query_json(rule_json, bindings_json).map_err(|e| JsValue::from_str(&e))
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
        let json = include_str!("../../fixture/nakagin-capsule-tower.trinity.json");
        let mut g = Graph::load_json(json).unwrap();
        g.recompute_derived();
        g
    }

    #[test]
    fn nakagin_fixture_loads() {
        let g = nakagin_graph();
        assert_eq!(g.nodes.len(), 6);
        assert_eq!(g.edges.len(), 5);
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
                parameters: vec![ParameterSpec {
                    name: "label".into(),
                    kind: ParameterKind::String,
                    default: PropertyValue::String("nakagin-core".into()),
                }],
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
    fn trinity_host_rebuilds_engine() {
        let host = TrinityHost::from_graph(nakagin_graph());
        assert_eq!(host.engine.nodes.len(), 6);
        assert!(!host.engine.edges.is_empty());
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
}
// #endregion 🔖Tests
