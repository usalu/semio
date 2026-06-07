//! 🕸️ Generic property graph engine on infinite canvas; specialize via quadrant crates.

pub mod geometry;
pub mod scene_json;

pub use geometry::{
    circle_handle_angle_toward, clamp_f64, compute_edge_bezier_points, distance_between, distance_point_to_cubic_bezier, encode_board_stroke_scene, handle_position_on_circle,
    handle_position_on_rectangle, normalize_or_zero, ray_from_origin_to_axis_aligned_rectangle_edge, rectangle_handle_angle_toward,
};
pub use scene_json::{board_json_visible_option, board_json_visible_or_true, CameraJson, NodeDescJson};

pub use infinite_cavas as cavas;
pub use mathematical_core::{self as core, Directed, Directedness, Edge as CoreEdge, EdgeId, HandleId, NodeId, Normal, PortModel, Ported, Undirected};

pub use core::orient_endpoints;

// #region 🔖GraphExtension
/// 🧩 Extension hook for domain-specific graph behavior.
pub trait GraphExtension: cavas::CanvasExtension {}
// #endregion 🔖GraphExtension

// #region 🔖Kinds
use std::collections::{BTreeMap, BTreeSet};

use cavas::vello::kurbo::{CubicBez, Point, Vec2};

/// 🧭 Camera state in world units with a zoom scalar suitable for a WASM host bridge.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

/// 🟠 Retained node state with world-space center and circular radius.
#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub id: NodeId,
    pub center: Point,
    pub radius: f64,
    pub draggable: bool,
}

/// 🟣 Tangent handle anchored to a node at a polar angle.
#[derive(Clone, Debug, PartialEq)]
pub struct Handle {
    pub angle: f64,
    pub id: HandleId,
    pub node_id: NodeId,
    pub radius: f64,
}

/// 🪢 Retained edge with typed endpoints.
pub type GraphEdge<E> = CoreEdge<E>;

/// 🎯 Semantic board event emitted after interaction or selection changes.
#[derive(Clone, Debug, PartialEq)]
pub enum BoardEvent {
    HoverChanged { id: Option<u64> },
    NodeMoved { id: NodeId, x: f64, y: f64 },
    SelectionChanged { edge_ids: Vec<EdgeId>, handle_ids: Vec<HandleId>, node_ids: Vec<NodeId> },
}

/// ✅ Selection snapshot maintained by the engine hot path.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Selection {
    pub edge_ids: BTreeSet<EdgeId>,
    pub handle_ids: BTreeSet<HandleId>,
    pub node_ids: BTreeSet<NodeId>,
}

/// 🖼️ Minimal render snapshot suitable for a host-side drawing layer or tests.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RenderSnapshot {
    pub edges: Vec<CubicBez>,
    pub handles: Vec<(HandleId, Point, f64)>,
    pub nodes: Vec<(NodeId, Point, f64)>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum HitObject<E> {
    Edge(EdgeId),
    Endpoint(E),
    Node(NodeId),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InteractionMode {
    DragNode { node_id: NodeId, offset: Vec2 },
    Idle,
}

impl Default for InteractionMode {
    fn default() -> Self {
        Self::Idle
    }
}

pub fn handle_position(node: &Node, handle: &Handle) -> Point {
    geometry::handle_position_on_circle(node.center, node.radius, handle.angle)
}

fn distance(left: Point, right: Point) -> f64 {
    geometry::distance_between(left, right)
}

// #region 🔖GraphPortModel
/// 🔌 Port model with graph selection semantics.
pub trait GraphPortModel: PortModel {
    fn select_endpoint(selection: &mut Selection, endpoint: Self::Endpoint);
}

impl GraphPortModel for Normal {
    fn select_endpoint(selection: &mut Selection, endpoint: Self::Endpoint) {
        selection.node_ids.insert(endpoint);
    }
}

impl GraphPortModel for Ported {
    fn select_endpoint(selection: &mut Selection, endpoint: Self::Endpoint) {
        selection.handle_ids.insert(endpoint);
    }
}
// #endregion 🔖GraphPortModel
// #endregion 🔖Kinds

// #region 🔖Engine

/// ⚙️ Retained graph engine parameterized by port model and directedness.
#[derive(Clone, Debug)]
pub struct GraphEngine<P: GraphPortModel, D: Directedness> {
    pub camera: Camera,
    pub edges: BTreeMap<EdgeId, GraphEdge<P::Endpoint>>,
    pub events: Vec<BoardEvent>,
    pub handles: BTreeMap<HandleId, Handle>,
    pub hover: Option<u64>,
    pub interaction: InteractionMode,
    pub nodes: BTreeMap<NodeId, Node>,
    pub selection: Selection,
    _directedness: std::marker::PhantomData<D>,
    _port: std::marker::PhantomData<P>,
}

impl<P: GraphPortModel, D: Directedness> Default for GraphEngine<P, D> {
    fn default() -> Self {
        Self {
            camera: Camera::default(),
            edges: BTreeMap::new(),
            events: Vec::new(),
            handles: BTreeMap::new(),
            hover: None,
            interaction: InteractionMode::default(),
            nodes: BTreeMap::new(),
            selection: Selection::default(),
            _directedness: std::marker::PhantomData,
            _port: std::marker::PhantomData,
        }
    }
}

impl<P: GraphPortModel, D: Directedness> GraphEngine<P, D> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_camera(&mut self, x: f64, y: f64, zoom: f64) {
        self.camera = Camera { x, y, zoom };
    }

    pub fn create_node(&mut self, id: NodeId, x: f64, y: f64, radius: f64, draggable: bool) {
        self.nodes.insert(id, Node { center: Point::new(x, y), draggable, id, radius });
    }

    pub fn update_node(&mut self, id: NodeId, x: f64, y: f64, radius: f64) {
        if let Some(node) = self.nodes.get_mut(&id) {
            node.center = Point::new(x, y);
            node.radius = radius;
        }
    }

    pub fn remove_node(&mut self, id: NodeId) {
        self.nodes.remove(&id);
        if P::HAS_PORTS {
            let removed_handles: Vec<HandleId> = self.handles.values().filter(|handle| handle.node_id == id).map(|handle| handle.id).collect();
            for handle_id in removed_handles {
                self.remove_handle(handle_id);
            }
        } else {
            let removed_edges: Vec<EdgeId> = self
                .edges
                .values()
                .filter(|edge| P::endpoint_as_u64(edge.source) == id || P::endpoint_as_u64(edge.target) == id)
                .map(|edge| edge.id)
                .collect();
            for edge_id in removed_edges {
                self.edges.remove(&edge_id);
                self.selection.edge_ids.remove(&edge_id);
            }
        }
        self.selection.node_ids.remove(&id);
        self.push_selection_event();
    }

    pub fn create_handle(&mut self, id: HandleId, node_id: NodeId, angle: f64) {
        if P::HAS_PORTS {
            self.handles.insert(id, Handle { angle, id, node_id, radius: 8.0 });
        }
    }

    pub fn create_edge(&mut self, id: EdgeId, source: P::Endpoint, target: P::Endpoint) {
        let (source, target) = orient_endpoints::<P::Endpoint, D>(source, target);
        self.edges.insert(id, GraphEdge { id, source, target });
    }

    pub fn pointer_down(&mut self, x: f64, y: f64, extend_selection: bool) {
        let point = Point::new(x, y);
        match self.hit_test(point) {
            Some(HitObject::Node(node_id)) => {
                self.apply_pick_selection(HitObject::Node(node_id), extend_selection);
                if let Some(node) = self.nodes.get(&node_id) {
                    if node.draggable {
                        self.interaction = InteractionMode::DragNode { node_id, offset: point - node.center };
                    }
                }
                self.update_hover(Some(node_id));
            }
            Some(HitObject::Endpoint(ep)) => {
                self.apply_pick_selection(HitObject::Endpoint(ep), extend_selection);
                self.update_hover(Some(P::endpoint_as_u64(ep)));
                self.interaction = InteractionMode::Idle;
            }
            Some(HitObject::Edge(edge_id)) => {
                self.apply_pick_selection(HitObject::Edge(edge_id), extend_selection);
                self.update_hover(Some(edge_id));
                self.interaction = InteractionMode::Idle;
            }
            None => {
                self.selection = Selection::default();
                self.push_selection_event();
                self.update_hover(None);
                self.interaction = InteractionMode::Idle;
            }
        }
    }

    pub fn pointer_move(&mut self, x: f64, y: f64) {
        let point = Point::new(x, y);
        match self.interaction {
            InteractionMode::DragNode { node_id, offset } => {
                if let Some(node) = self.nodes.get_mut(&node_id) {
                    node.center = point - offset;
                    self.events.push(BoardEvent::NodeMoved { id: node_id, x: node.center.x, y: node.center.y });
                }
            }
            InteractionMode::Idle => {
                self.update_hover(self.hit_test(point).map(|hit| match hit {
                    HitObject::Edge(id) => id,
                    HitObject::Endpoint(ep) => P::endpoint_as_u64(ep),
                    HitObject::Node(id) => id,
                }));
            }
        }
    }

    pub fn pointer_up(&mut self) {
        self.interaction = InteractionMode::Idle;
    }

    pub fn render_snapshot(&self) -> RenderSnapshot {
        let mut snapshot = RenderSnapshot::default();
        for node in self.nodes.values() {
            snapshot.nodes.push((node.id, node.center, node.radius));
        }
        if P::HAS_PORTS {
            for handle in self.handles.values() {
                if let Some(node) = self.nodes.get(&handle.node_id) {
                    snapshot.handles.push((handle.id, handle_position(node, handle), handle.radius));
                }
            }
        }
        for edge in self.edges.values() {
            if let Some(curve) = self.edge_curve(edge.id) {
                snapshot.edges.push(curve);
            }
        }
        let _stroke_scene = encode_board_stroke_scene(&snapshot.edges, 2.0);
        let _ = _stroke_scene.encoding().path_tags.len();
        snapshot
    }

    pub fn drain_events(&mut self) -> Vec<BoardEvent> {
        std::mem::take(&mut self.events)
    }

    pub fn edge_curve(&self, edge_id: EdgeId) -> Option<CubicBez> {
        let edge = self.edges.get(&edge_id)?;
        let (source_position, target_position, source_center, target_center) = self.endpoint_positions(edge)?;
        Some(compute_edge_bezier_points(source_position, target_position, source_center, target_center))
    }

    fn endpoint_positions(&self, edge: &GraphEdge<P::Endpoint>) -> Option<(Point, Point, Point, Point)> {
        if P::HAS_PORTS {
            let source_handle = self.handles.get(&P::endpoint_as_handle(edge.source)?)?;
            let target_handle = self.handles.get(&P::endpoint_as_handle(edge.target)?)?;
            let source_node = self.nodes.get(&source_handle.node_id)?;
            let target_node = self.nodes.get(&target_handle.node_id)?;
            let source_position = handle_position(source_node, source_handle);
            let target_position = handle_position(target_node, target_handle);
            return Some((source_position, target_position, source_node.center, target_node.center));
        }
        let source_node = self.nodes.get(&P::endpoint_as_u64(edge.source))?;
        let target_node = self.nodes.get(&P::endpoint_as_u64(edge.target))?;
        Some((source_node.center, target_node.center, source_node.center, target_node.center))
    }

    fn remove_handle(&mut self, id: HandleId) {
        self.handles.remove(&id);
        let removed_edges: Vec<EdgeId> = self
            .edges
            .values()
            .filter(|edge| P::endpoint_as_u64(edge.source) == id || P::endpoint_as_u64(edge.target) == id)
            .map(|edge| edge.id)
            .collect();
        for edge_id in removed_edges {
            self.edges.remove(&edge_id);
            self.selection.edge_ids.remove(&edge_id);
        }
        self.selection.handle_ids.remove(&id);
    }

    fn apply_pick_selection(&mut self, hit: HitObject<P::Endpoint>, extend_selection: bool) {
        if !extend_selection {
            self.selection = Selection::default();
        }
        match hit {
            HitObject::Node(id) => {
                self.selection.node_ids.insert(id);
            }
            HitObject::Endpoint(ep) => {
                P::select_endpoint(&mut self.selection, ep);
            }
            HitObject::Edge(id) => {
                self.selection.edge_ids.insert(id);
            }
        }
        self.push_selection_event();
    }

    fn update_hover(&mut self, hover: Option<u64>) {
        if self.hover == hover {
            return;
        }
        self.hover = hover;
        self.events.push(BoardEvent::HoverChanged { id: hover });
    }

    fn push_selection_event(&mut self) {
        self.events.push(BoardEvent::SelectionChanged {
            edge_ids: self.selection.edge_ids.iter().copied().collect(),
            handle_ids: self.selection.handle_ids.iter().copied().collect(),
            node_ids: self.selection.node_ids.iter().copied().collect(),
        });
    }

    fn hit_test(&self, point: Point) -> Option<HitObject<P::Endpoint>> {
        if P::HAS_PORTS {
            for handle in self.handles.values().rev() {
                let node = self.nodes.get(&handle.node_id)?;
                if distance(point, handle_position(node, handle)) <= handle.radius + 6.0 {
                    if let Some(ep) = P::try_handle_endpoint(handle.id) {
                        return Some(HitObject::Endpoint(ep));
                    }
                }
            }
        }
        for node in self.nodes.values().rev() {
            if distance(point, node.center) <= node.radius {
                return Some(HitObject::Node(node.id));
            }
        }
        for edge in self.edges.values().rev() {
            if let Some(curve) = self.edge_curve(edge.id) {
                if distance_point_to_cubic_bezier(point, curve, 18) <= 8.0 {
                    return Some(HitObject::Edge(edge.id));
                }
            }
        }
        None
    }
}
// #endregion 🔖Engine

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn port_directed_engine_round_trip() {
        let mut engine = GraphEngine::<Ported, Directed>::new();
        engine.create_node(1, 0.0, 0.0, 40.0, true);
        engine.create_handle(10, 1, 0.0);
        engine.create_handle(11, 1, std::f64::consts::PI);
        engine.create_edge(100, 10, 11);
        let snap = engine.render_snapshot();
        assert_eq!(snap.nodes.len(), 1);
        assert_eq!(snap.handles.len(), 2);
        assert_eq!(snap.edges.len(), 1);
    }

    #[test]
    fn normal_directed_node_edges() {
        let mut engine = GraphEngine::<Normal, Directed>::new();
        engine.create_node(1, 0.0, 0.0, 40.0, true);
        engine.create_node(2, 120.0, 0.0, 40.0, true);
        engine.create_edge(100, 1, 2);
        let snap = engine.render_snapshot();
        assert_eq!(snap.nodes.len(), 2);
        assert!(snap.handles.is_empty());
        assert_eq!(snap.edges.len(), 1);
    }

    #[test]
    fn undirected_normalizes_endpoints() {
        let mut engine = GraphEngine::<Normal, Undirected>::new();
        engine.create_node(1, 0.0, 0.0, 40.0, true);
        engine.create_node(2, 120.0, 0.0, 40.0, true);
        engine.create_edge(100, 2, 1);
        let edge = engine.edges.get(&100).unwrap();
        assert_eq!(edge.source, 1);
        assert_eq!(edge.target, 2);
    }
}
// #endregion 🔖Tests
