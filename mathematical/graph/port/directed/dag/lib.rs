//! 🌳 Directed acyclic port graph: rectangle IO nodes on infinite canvas.

use serde::{Deserialize, Serialize};

pub use infinite_cavas as cavas;
pub use mathematical_graph_port_directed::{
    self as graph, compute_edge_bezier_points, DirectedPortGraphEngine, Edge, EdgeId, GraphExtension, Handle, HandleId, HandleRole, InteractionMode, Node, NodeId, RenderSnapshot, Selection,
};
use graph::BoardEvent;

/// 🌳 DAG board engine alias.
pub type DagBoardEngine = DirectedPortGraphEngine;

// #region 🔖IoNode
/// 📦 Rectangle node with named inputs on the left and outputs on the right.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IoNodeSpec {
    pub id: String,
    pub name: String,
    pub inputs: Vec<IoPortSpec>,
    pub outputs: Vec<IoPortSpec>,
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
    #[serde(default = "default_node_width")]
    pub width: f64,
    #[serde(default = "default_node_height")]
    pub height: f64,
}

fn default_node_width() -> f64 {
    160.0
}

fn default_node_height() -> f64 {
    56.0
}

/// 🪝 Named horizontal port on an IO node edge.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IoPortSpec {
    pub id: String,
    pub label: String,
}

/// 📐 Places input handles on the left and output handles on the right of a rectangle node.
pub fn io_node_handle_angles(input_index: usize, input_count: usize, output_index: usize, output_count: usize) -> (f64, f64) {
    let input_angle = port_angle_on_side(input_index, input_count.max(1), true);
    let output_angle = port_angle_on_side(output_index, output_count.max(1), false);
    (input_angle, output_angle)
}

fn port_angle_on_side(index: usize, count: usize, left: bool) -> f64 {
    let t = (index as f64 + 0.5) / count as f64;
    let y = (t - 0.5) * 0.8;
    if left {
        std::f64::consts::PI + y * std::f64::consts::FRAC_PI_2 * 0.9
    } else {
        y * std::f64::consts::FRAC_PI_2 * 0.9
    }
}

/// 📐 Rectangle-layout port angle (north-zero CCW) aligned with painted IO labels.
pub fn io_node_rect_port_angle(x: f64, y: f64, width: f64, height: f64, index: usize, count: usize, left: bool) -> f64 {
    use cavas::vello::kurbo::Point;
    use graph::rectangle_handle_angle_toward;
    let hw = width * 0.5;
    let hh = height * 0.5;
    let t = (index as f64 + 0.5) / count.max(1) as f64;
    let port_y = y - hh + t * height;
    let port_x = if left { x - hw } else { x + hw };
    rectangle_handle_angle_toward(Point::new(x, y), width, height, Point::new(port_x, port_y))
}
// #endregion 🔖IoNode

// #region 🔖Acyclicity
use std::collections::{HashMap, HashSet};

/// 🚫 Returns true when adding `source -> target` would create a cycle.
pub fn would_create_cycle(existing: &[(String, String)], source: &str, target: &str) -> bool {
    if source == target {
        return true;
    }
    let mut adj: HashMap<String, Vec<String>> = HashMap::new();
    for (u, v) in existing {
        adj.entry(u.clone()).or_default().push(v.clone());
    }
    adj.entry(source.to_string()).or_default().push(target.to_string());
    has_path(&adj, target, source)
}

fn has_path(adj: &HashMap<String, Vec<String>>, from: &str, to: &str) -> bool {
    let mut seen = HashSet::new();
    let mut stack = vec![from.to_string()];
    while let Some(n) = stack.pop() {
        if n == to {
            return true;
        }
        if !seen.insert(n.clone()) {
            continue;
        }
        if let Some(next) = adj.get(&n) {
            for m in next {
                stack.push(m.clone());
            }
        }
    }
    false
}
// #endregion 🔖Acyclicity

// #region 🔖Layout
use mathematical_core::tree_layout::buchheim_positions;
use serde_json::Value;

/// 🌲 Layered DAG layout options for fixture JSON.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DagLayoutOptions {
    #[serde(default = "default_layer_spacing")]
    pub layer_spacing: f64,
    #[serde(default = "default_sibling_gap")]
    pub sibling_gap: f64,
    #[serde(default)]
    pub center_x: Option<f64>,
    #[serde(default)]
    pub center_y: Option<f64>,
}

fn default_layer_spacing() -> f64 {
    120.0
}

fn default_sibling_gap() -> f64 {
    40.0
}

impl Default for DagLayoutOptions {
    fn default() -> Self {
        Self { layer_spacing: default_layer_spacing(), sibling_gap: default_sibling_gap(), center_x: None, center_y: None }
    }
}

/// 🌳 Writes node centers from a layered DAG layout into `dag.fixture/v1`.
pub fn apply_dag_layout_to_fixture_v1_value(fixture: &mut Value, opts: &DagLayoutOptions) -> Result<(), String> {
    let Some(root) = fixture.as_object_mut() else {
        return Err("fixture root must be object".into());
    };
    if root.get("schema").and_then(|v| v.as_str()) != Some("dag.fixture/v1") {
        return Err("schema must be dag.fixture/v1".into());
    }
    let edges_json = root.get("edges").and_then(|v| v.as_array()).cloned().unwrap_or_default();
    let Some(nodes) = root.get_mut("nodes").and_then(|v| v.as_array_mut()) else {
        return Err("nodes array missing".into());
    };
    if nodes.is_empty() {
        return Ok(());
    }
    let mut handle_to_node: HashMap<String, String> = HashMap::new();
    let mut node_ids: HashSet<String> = HashSet::new();
    for node in nodes.iter() {
        let Some(obj) = node.as_object() else {
            continue;
        };
        let Some(nid) = obj.get("id").and_then(|v| v.as_str()) else {
            continue;
        };
        node_ids.insert(nid.to_string());
        if let Some(handles) = obj.get("handles").and_then(|v| v.as_array()) {
            for h in handles {
                if let Some(hid) = h.get("id").and_then(|v| v.as_str()) {
                    handle_to_node.insert(hid.to_string(), nid.to_string());
                }
            }
        }
    }
    let mut directed: Vec<(String, String)> = Vec::new();
    for e in &edges_json {
        let Some(eo) = e.as_object() else {
            continue;
        };
        let src = eo.get("source").and_then(|v| v.as_str()).or_else(|| eo.get("sourceHandle").and_then(|v| v.as_str()));
        let tgt = eo.get("target").and_then(|v| v.as_str()).or_else(|| eo.get("targetHandle").and_then(|v| v.as_str()));
        let (Some(src_h), Some(tgt_h)) = (src, tgt) else {
            continue;
        };
        let u = handle_to_node.get(src_h).cloned().unwrap_or_else(|| src_h.to_string());
        let v = handle_to_node.get(tgt_h).cloned().unwrap_or_else(|| tgt_h.to_string());
        if u != v && node_ids.contains(&u) && node_ids.contains(&v) {
            directed.push((u, v));
        }
    }
    let mut incoming: HashMap<String, u32> = HashMap::new();
    for id in &node_ids {
        incoming.insert(id.clone(), 0);
    }
    for (_, v) in &directed {
        *incoming.entry(v.clone()).or_insert(0) += 1;
    }
    let roots: Vec<String> = node_ids.iter().filter(|id| incoming.get(*id).copied().unwrap_or(0) == 0).cloned().collect();
    let roots = if roots.is_empty() { node_ids.iter().cloned().collect() } else { roots };
    let mut depth: HashMap<String, i32> = HashMap::new();
    for r in &roots {
        depth.insert(r.clone(), 0);
    }
    for _ in 0..directed.len().saturating_add(node_ids.len()).saturating_add(4) {
        let mut changed = false;
        for (u, v) in &directed {
            let Some(&du) = depth.get(u) else {
                continue;
            };
            let nd = du + 1;
            if depth.get(v).copied().unwrap_or(-1) < nd {
                depth.insert(v.clone(), nd);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    let pos = buchheim_positions(&roots, &directed, &depth);
    let mut minx = f64::INFINITY;
    let mut maxx = f64::NEG_INFINITY;
    let mut miny = f64::INFINITY;
    let mut maxy = f64::NEG_INFINITY;
    for (_, (x, y)) in &pos {
        minx = minx.min(*x);
        maxx = maxx.max(*x);
        miny = miny.min(*y);
        maxy = maxy.max(*y);
    }
    let cx = (minx + maxx) * 0.5;
    let cy = (miny + maxy) * 0.5;
    let gx = opts.center_x.unwrap_or(0.0);
    let gy = opts.center_y.unwrap_or(0.0);
    let dx = gx - cx * opts.sibling_gap;
    let dy = gy - cy * opts.layer_spacing;
    for node in nodes.iter_mut() {
        let Some(obj) = node.as_object_mut() else {
            continue;
        };
        let Some(nid) = obj.get("id").and_then(|v| v.as_str()) else {
            continue;
        };
        let Some((bx, by)) = pos.get(nid) else {
            continue;
        };
        obj.insert("x".into(), serde_json::json!(bx * opts.sibling_gap + dx));
        obj.insert("y".into(), serde_json::json!(by * opts.layer_spacing + dy));
    }
    Ok(())
}
// #endregion 🔖Layout

// #region 🔖GraphExtension
/// 🧩 DAG-specific graph extension marker.
pub struct DagExtension;

impl cavas::CanvasExtension for DagExtension {
    fn extension_id(&self) -> &str {
        "dag"
    }
}

impl GraphExtension for DagExtension {}
// #endregion 🔖GraphExtension

fn dag_debug_log(msg: &str) {
    #[cfg(target_arch = "wasm32")]
    web_sys::console::log_1(&msg.into());
    #[cfg(not(target_arch = "wasm32"))]
    eprintln!("{msg}");
}

// #region 🔖DagHost

/// 🌳 Retained DAG host: IO nodes, edges, engine, camera.
pub struct DagHost {
    pub fixture: DagFixtureV1,
    pub engine: DagBoardEngine,
    width: u32,
    height: u32,
    dpr: f64,
    last_screen_x: f64,
    last_screen_y: f64,
    node_id_map: HashMap<NodeId, usize>,
    handle_key_map: HashMap<HandleId, String>,
    edge_id_map: HashMap<EdgeId, String>,
}

/// 📦 `dag.fixture/v1` document.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DagFixtureV1 {
    pub schema: String,
    pub camera: DagCameraV1,
    pub nodes: Vec<IoNodeSpec>,
    pub edges: Vec<DagFixtureEdgeV1>,
}

/// 📷 Fixture camera snapshot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DagCameraV1 {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

/// 🔗 Edge between port handles.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DagFixtureEdgeV1 {
    pub id: String,
    pub source: String,
    pub target: String,
}

impl Default for DagFixtureV1 {
    fn default() -> Self {
        serde_json::from_str(include_str!("fixture/demo.dag.json")).unwrap_or_else(|_| Self {
            schema: "dag.fixture/v1".into(),
            camera: DagCameraV1 { x: 0.0, y: 0.0, zoom: 1.0 },
            nodes: vec![],
            edges: vec![],
        })
    }
}

impl DagHost {
    pub fn default_demo() -> Self {
        Self::from_fixture(DagFixtureV1::default())
    }

    pub fn from_fixture(fixture: DagFixtureV1) -> Self {
        Self::from_fixture_with_layout(fixture, true)
    }

    /// 🌳 Builds a host without running auto-layout (preserves node positions).
    pub fn from_fixture_without_layout(fixture: DagFixtureV1) -> Self {
        Self::from_fixture_with_layout(fixture, false)
    }

    fn from_fixture_with_layout(fixture: DagFixtureV1, apply_layout: bool) -> Self {
        let mut host = Self {
            fixture,
            engine: DagBoardEngine::new(),
            width: 1,
            height: 1,
            dpr: 1.0,
            last_screen_x: 0.0,
            last_screen_y: 0.0,
            node_id_map: HashMap::new(),
            handle_key_map: HashMap::new(),
            edge_id_map: HashMap::new(),
        };
        host.rebuild_engine_with_layout(apply_layout);
        host
    }

    pub fn set_viewport(&mut self, width: u32, height: u32, dpr: f64) {
        self.width = width.max(1);
        self.height = height.max(1);
        self.dpr = dpr.max(1.0);
    }

    pub fn load_fixture_json(json: &str) -> Result<Self, String> {
        let fixture: DagFixtureV1 = serde_json::from_str(json).map_err(|e| e.to_string())?;
        if fixture.schema != "dag.fixture/v1" {
            return Err("schema must be dag.fixture/v1".into());
        }
        Ok(Self::from_fixture(fixture))
    }

    pub fn fixture_json(&self) -> Result<String, String> {
        serde_json::to_string(&self.fixture).map_err(|e| e.to_string())
    }

    fn rebuild_engine(&mut self) {
        self.rebuild_engine_with_layout(true);
    }

    fn rebuild_engine_with_layout(&mut self, apply_layout: bool) {
        self.engine = DagBoardEngine::new();
        self.engine.enforce_acyclic = true;
        self.node_id_map.clear();
        self.handle_key_map.clear();
        self.edge_id_map.clear();
        let (cx, cy, zoom) = (self.fixture.camera.x, self.fixture.camera.y, self.fixture.camera.zoom);
        self.engine.set_camera(cx, cy, zoom);
        if apply_layout {
            let mut fixture_value = serde_json::to_value(&self.fixture).unwrap_or_else(|_| serde_json::json!({}));
            let _ = apply_dag_layout_to_fixture_v1_value(&mut fixture_value, &DagLayoutOptions::default());
            if let Ok(updated) = serde_json::from_value::<DagFixtureV1>(fixture_value.clone()) {
                self.fixture = updated;
            }
        }
        let mut next_node: u64 = 1;
        let mut next_handle: u64 = 10;
        let mut handle_map: HashMap<String, u64> = HashMap::new();
        for (idx, node) in self.fixture.nodes.iter().enumerate() {
            let nid = next_node;
            next_node += 1;
            self.node_id_map.insert(nid, idx);
            self.engine.create_rect_node(nid, node.x, node.y, node.width, node.height, true);
            for (port_idx, port) in node.inputs.iter().enumerate() {
                let in_a = io_node_rect_port_angle(node.x, node.y, node.width, node.height, port_idx, node.inputs.len().max(1), true);
                let hid = next_handle;
                next_handle += 1;
                let key = format!("{}:{}", node.id, port.id);
                handle_map.insert(key.clone(), hid);
                self.handle_key_map.insert(hid, key);
                self.engine.create_handle(hid, nid, in_a);
                self.engine.set_handle_role(hid, HandleRole::Target);
            }
            for (port_idx, port) in node.outputs.iter().enumerate() {
                let out_a = io_node_rect_port_angle(node.x, node.y, node.width, node.height, port_idx, node.outputs.len().max(1), false);
                let hid = next_handle;
                next_handle += 1;
                let key = format!("{}:{}", node.id, port.id);
                handle_map.insert(key.clone(), hid);
                self.handle_key_map.insert(hid, key);
                self.engine.create_handle(hid, nid, out_a);
                self.engine.set_handle_role(hid, HandleRole::Source);
            }
        }
        let existing: Vec<(String, String)> = self
            .fixture
            .edges
            .iter()
            .filter_map(|e| {
                let src = e.source.split(':').next()?.to_string();
                let tgt = e.target.split(':').next()?.to_string();
                Some((src, tgt))
            })
            .collect();
        let mut eid: u64 = 100;
        for edge in &self.fixture.edges {
            if would_create_cycle(&existing, edge.source.split(':').next().unwrap_or(""), edge.target.split(':').next().unwrap_or("")) {
                continue;
            }
            let src = handle_map.get(&edge.source).copied();
            let tgt = handle_map.get(&edge.target).copied();
            if let (Some(s), Some(t)) = (src, tgt) {
                let id = Self::parse_fixture_edge_numeric_id(&edge.id).unwrap_or(eid);
                eid = eid.max(id).saturating_add(1);
                self.engine.create_edge(id, s, t);
                self.edge_id_map.insert(id, edge.id.clone());
            }
        }
        self.engine.set_next_edge_id(eid);
    }

    fn parse_fixture_edge_numeric_id(id: &str) -> Option<u64> {
        id.strip_prefix('e').and_then(|s| s.parse().ok())
    }

    fn screen_to_world_point(&self, sx: f64, sy: f64) -> cavas::vello::kurbo::Point {
        use cavas::camera::{screen_to_world, Camera as CavasCamera, Viewport};
        use cavas::vello::kurbo::Point;
        let cam = CavasCamera { x: self.fixture.camera.x, y: self.fixture.camera.y, zoom: self.fixture.camera.zoom };
        let viewport = Viewport { width: self.width, height: self.height, dpr: self.dpr };
        screen_to_world(&cam, &viewport, Point::new(sx, sy))
    }

    fn sync_node_positions_from_engine(&mut self) {
        for (&nid, &idx) in &self.node_id_map {
            if let Some(node) = self.engine.nodes.get(&nid) {
                self.fixture.nodes[idx].x = node.center.x;
                self.fixture.nodes[idx].y = node.center.y;
            }
        }
    }

    fn sync_edges_from_engine(&mut self) {
        let mut edges = Vec::with_capacity(self.engine.edges.len());
        for (eid, edge) in &self.engine.edges {
            let Some(source) = self.handle_key_map.get(&edge.source).cloned() else {
                continue;
            };
            let Some(target) = self.handle_key_map.get(&edge.target).cloned() else {
                continue;
            };
            let id = self.edge_id_map.get(eid).cloned().unwrap_or_else(|| format!("e{eid}"));
            self.edge_id_map.insert(*eid, id.clone());
            edges.push(DagFixtureEdgeV1 { id, source, target });
        }
        self.fixture.edges = edges;
    }

    fn process_engine_events(&mut self) {
        let events = self.engine.drain_events();
        let mut moved = false;
        let mut wired = false;
        for event in events {
            match event {
                BoardEvent::NodeMoved { id, x, y } => {
                    moved = true;
                    dag_debug_log(&format!("[DEBUG] dag node moved id={id} x={x:.1} y={y:.1}"));
                }
                BoardEvent::EdgeConnected { id, source, target } => {
                    wired = true;
                    dag_debug_log(&format!("[DEBUG] dag edge connected id={id} source={source} target={target}"));
                }
                BoardEvent::EdgeRemoved { id } => {
                    wired = true;
                    dag_debug_log(&format!("[DEBUG] dag edge removed id={id}"));
                }
                _ => {}
            }
        }
        if moved {
            self.sync_node_positions_from_engine();
        }
        if wired {
            self.sync_edges_from_engine();
        }
    }

    pub fn set_camera(&mut self, x: f64, y: f64, zoom: f64) {
        self.fixture.camera = DagCameraV1 { x, y, zoom };
        self.engine.set_camera(x, y, zoom);
    }

    pub fn pointer_down(&mut self, x: f64, y: f64, extend: bool) {
        self.last_screen_x = x;
        self.last_screen_y = y;
        let world = self.screen_to_world_point(x, y);
        self.engine.pointer_down(world.x, world.y, extend);
        self.process_engine_events();
    }

    pub fn pointer_move(&mut self, x: f64, y: f64) {
        self.last_screen_x = x;
        self.last_screen_y = y;
        let world = self.screen_to_world_point(x, y);
        self.engine.pointer_move(world.x, world.y);
        self.process_engine_events();
    }

    pub fn pointer_up(&mut self, x: f64, y: f64) {
        self.last_screen_x = x;
        self.last_screen_y = y;
        let world = self.screen_to_world_point(x, y);
        self.engine.pointer_up(world.x, world.y);
        self.process_engine_events();
    }

    pub fn paint_scene(&self, scene: &mut cavas::vello::Scene, viewport_w: u32, viewport_h: u32, dpr: f64) {
        use cavas::camera::{camera_content_affine, world_to_screen, Camera as CavasCamera, Viewport};
        use cavas::text::append_label;
        use cavas::vello::kurbo::{Affine, Circle, Point, Rect, Stroke};
        use cavas::vello::peniko::{Color, Fill};

        let cam = CavasCamera { x: self.fixture.camera.x, y: self.fixture.camera.y, zoom: self.fixture.camera.zoom };
        let viewport = Viewport { width: viewport_w.max(1), height: viewport_h.max(1), dpr: dpr.max(1.0) };
        let aff = camera_content_affine(&cam, &viewport);
        let snap = self.engine.render_snapshot();
        for curve in &snap.edges {
            scene.stroke(&Stroke::new(2.0), aff, Color::from_rgb8(180, 200, 230), None, curve);
        }
        if let Some((a, b)) = snap.pending_edge {
            let preview = compute_edge_bezier_points(a, b, a, b);
            scene.stroke(&Stroke::new(2.0), aff, Color::from_rgb8(120, 180, 255), None, &preview);
        }
        let node_stroke = Color::from_rgb8(90, 110, 140);
        let node_fill = Color::from_rgba8(40, 48, 62, 230);
        let label_fill = Color::from_rgb8(230, 235, 245);
        let label_halo = Color::from_rgba8(20, 22, 28, 200);
        for node in &self.fixture.nodes {
            let hw = node.width * 0.5;
            let hh = node.height * 0.5;
            let rect = Rect::new(node.x - hw, node.y - hh, node.x + hw, node.y + hh);
            scene.fill(Fill::NonZero, aff, node_fill, None, &rect);
            scene.stroke(&Stroke::new(1.5), aff, node_stroke, None, &rect);
            let px = (10.0 * cam.zoom).clamp(8.0, 18.0);
            let center_screen = world_to_screen(&cam, &viewport, Point::new(node.x, node.y));
            for (i, port) in node.inputs.iter().enumerate() {
                let t = (i as f64 + 0.5) / node.inputs.len().max(1) as f64;
                let world = Point::new(node.x - hw + 8.0 / cam.zoom.max(0.05), node.y - hh + t * hh * 2.0);
                append_label(scene, &port.label, world_to_screen(&cam, &viewport, world), px, label_fill, label_halo);
            }
            for (i, port) in node.outputs.iter().enumerate() {
                let t = (i as f64 + 0.5) / node.outputs.len().max(1) as f64;
                let world = Point::new(node.x + hw - 8.0 / cam.zoom.max(0.05), node.y - hh + t * hh * 2.0);
                append_label(scene, &port.label, world_to_screen(&cam, &viewport, world), px, label_fill, label_halo);
            }
            let name = node.name.trim();
            if !name.is_empty() {
                let mut label_scene = cavas::vello::Scene::new();
                append_label(&mut label_scene, name, Point::new(0.0, 0.0), px * 1.05, label_fill, label_halo);
                let rot = Affine::translate((center_screen.x, center_screen.y)) * Affine::rotate(-std::f64::consts::FRAC_PI_2);
                scene.append(&label_scene, Some(rot));
            }
        }
        for (hid, center, radius) in &snap.handles {
            let fill = match self.engine.handles.get(hid).map(|h| h.role) {
                Some(HandleRole::Source) => Color::from_rgb8(100, 200, 140),
                Some(HandleRole::Target) => Color::from_rgb8(200, 140, 100),
                _ => Color::from_rgb8(180, 200, 230),
            };
            scene.fill(Fill::NonZero, aff, fill, None, &Circle::new(*center, *radius));
        }
    }
}
// #endregion 🔖DagHost

// #region 🔖WasmSession
#[cfg(target_arch = "wasm32")]
mod wasm_session {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_futures::future_to_promise;
    use web_sys::HtmlCanvasElement;

    struct DagSessionInner {
        host: DagHost,
        gpu: cavas::gpu_session::CanvasGpuSession,
        width: u32,
        height: u32,
        dpr: f64,
    }

    #[wasm_bindgen]
    pub struct DagSession {
        state: Rc<RefCell<DagSessionInner>>,
    }

    #[wasm_bindgen]
    impl DagSession {
        #[wasm_bindgen(constructor)]
        pub fn new() -> Self {
            Self { state: Rc::new(RefCell::new(DagSessionInner { host: DagHost::default_demo(), gpu: cavas::gpu_session::CanvasGpuSession::default(), width: 1, height: 1, dpr: 1.0 })) }
        }

        #[wasm_bindgen(js_name = loadFixtureJson)]
        pub fn load_fixture_json(&self, json: &str) -> Result<(), JsValue> {
            let host = DagHost::load_fixture_json(json).map_err(|e| JsValue::from_str(&e))?;
            self.state.borrow_mut().host = host;
            Ok(())
        }

        #[wasm_bindgen(js_name = fixtureJson)]
        pub fn fixture_json(&self) -> Result<String, JsValue> {
            self.state.borrow().host.fixture_json().map_err(|e| JsValue::from_str(&e))
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
                let (render_ctx, renderer, surface) = cavas::gpu_session::CanvasGpuSession::create_canvas_surface(canvas.clone(), pw, ph)
                    .await
                    .map_err(|err| JsValue::from_str(&err))?;
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

        #[wasm_bindgen(js_name = renderFrame)]
        pub fn render_frame(&self) -> Result<(), JsValue> {
            let mut inner = self.state.borrow_mut();
            let mut scene = cavas::vello::Scene::new();
            inner.host.paint_scene(&mut scene, inner.width, inner.height, inner.dpr);
            inner.gpu.render_frame(&scene, cavas::vello::peniko::Color::from_rgba8(20, 22, 28, 255))
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm_session::DagSession;
// #endregion 🔖WasmSession

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn io_node_handle_angles_left_right() {
        let (in_a, out_a) = io_node_handle_angles(0, 2, 0, 1);
        assert!(in_a > std::f64::consts::FRAC_PI_2);
        assert!(out_a.abs() < std::f64::consts::FRAC_PI_2);
    }

    #[test]
    fn cycle_detection_blocks_back_edge() {
        let edges = vec![("a".into(), "b".into()), ("b".into(), "c".into())];
        assert!(would_create_cycle(&edges, "c", "a"));
        assert!(!would_create_cycle(&edges, "a", "c"));
    }

    #[test]
    fn dag_layout_moves_nodes() {
        let mut fixture: Value = serde_json::json!({
            "schema": "dag.fixture/v1",
            "nodes": [
                {"id": "a", "x": 0, "y": 0, "handles": []},
                {"id": "b", "x": 0, "y": 0, "handles": []}
            ],
            "edges": [{"id": "e1", "source": "a", "target": "b"}]
        });
        apply_dag_layout_to_fixture_v1_value(&mut fixture, &DagLayoutOptions::default()).unwrap();
        let a_y = fixture["nodes"][0]["y"].as_f64().unwrap();
        let b_y = fixture["nodes"][1]["y"].as_f64().unwrap();
        assert!((b_y - a_y).abs() > 1.0);
    }

    #[test]
    fn dag_host_loads_demo_fixture() {
        let host = DagHost::default_demo();
        assert_eq!(host.fixture.schema, "dag.fixture/v1");
        assert_eq!(host.fixture.nodes.len(), 6);
        assert_eq!(host.fixture.edges.len(), 6);
        assert!(!host.engine.render_snapshot().edges.is_empty());
    }

    #[test]
    fn dag_host_drags_node_in_world_space() {
        let mut host = DagHost::default_demo();
        host.set_viewport(1280, 800, 1.0);
        let before_x = host.fixture.nodes[0].x;
        let before_y = host.fixture.nodes[0].y;
        let sx = before_x + 640.0;
        let sy = before_y + 400.0;
        host.pointer_down(sx, sy, false);
        host.pointer_move(sx + 40.0, sy + 30.0);
        host.pointer_up(sx + 40.0, sy + 30.0);
        assert!((host.fixture.nodes[0].x - before_x).abs() > 1.0);
        assert!((host.fixture.nodes[0].y - before_y).abs() > 1.0);
    }

    #[test]
    fn dag_host_reconnects_edge_endpoint() {
        let mut host = DagHost::default_demo();
        host.set_viewport(1280, 800, 1.0);
        let combine_b = host.fixture.nodes.iter().position(|n| n.id == "combine").expect("combine");
        let scale = &host.fixture.nodes[host.fixture.nodes.iter().position(|n| n.id == "scale").expect("scale")];
        let combine = &host.fixture.nodes[combine_b];
        let target_sx = combine.x - combine.width * 0.5 + 640.0;
        let target_sy = combine.y + 400.0;
        let source_sx = scale.x + scale.width * 0.5 + 640.0;
        let source_sy = scale.y + 400.0;
        host.pointer_down(target_sx, target_sy, false);
        host.pointer_move(source_sx, source_sy);
        host.pointer_up(source_sx, source_sy);
        let e4 = host.fixture.edges.iter().find(|e| e.id == "e4").expect("e4");
        assert_eq!(e4.source, "scale:out");
    }

    #[test]
    fn io_node_rect_port_angles_on_edges() {
        use cavas::vello::kurbo::Point;
        use graph::handle_position_on_rectangle;
        let left = io_node_rect_port_angle(0.0, 0.0, 160.0, 72.0, 0, 2, true);
        let right = io_node_rect_port_angle(0.0, 0.0, 160.0, 72.0, 0, 1, false);
        let left_pos = handle_position_on_rectangle(Point::new(0.0, 0.0), 160.0, 72.0, left);
        let right_pos = handle_position_on_rectangle(Point::new(0.0, 0.0), 160.0, 72.0, right);
        assert!(left_pos.x < -70.0);
        assert!(right_pos.x > 70.0);
    }
}
// #endregion 🔖Tests
