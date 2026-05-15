use std::collections::{BTreeMap, BTreeSet};

// #region 🔖Kinds
/// 🧭 Camera state in world units with a zoom scalar suitable for a WASM host bridge.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera {
	pub x: f64,
	pub y: f64,
	pub zoom: f64,
}

/// 📍 Basic 2D vector used for retained geometry and hit testing.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2 {
	pub x: f32,
	pub y: f32,
}

/// 🧩 Stable node identifier.
pub type NodeId = u64;
/// 🪝 Stable handle identifier.
pub type HandleId = u64;
/// 🪢 Stable edge identifier.
pub type EdgeId = u64;

/// 🟠 Retained node state with world-space center and circular radius.
#[derive(Clone, Debug, PartialEq)]
pub struct Node {
	pub id: NodeId,
	pub center: Vec2,
	pub radius: f32,
	pub draggable: bool,
}

/// 🟣 Tangent handle anchored to a node at a polar angle.
#[derive(Clone, Debug, PartialEq)]
pub struct Handle {
	pub angle: f32,
	pub id: HandleId,
	pub node_id: NodeId,
	pub radius: f32,
}

/// 🪢 Cubic edge connecting two handles.
#[derive(Clone, Debug, PartialEq)]
pub struct Edge {
	pub from_handle: HandleId,
	pub id: EdgeId,
	pub to_handle: HandleId,
}

/// 🌀 Cubic bezier curve whose control arms follow circle normals at the two anchors.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CubicBezier {
	pub p0: Vec2,
	pub p1: Vec2,
	pub p2: Vec2,
	pub p3: Vec2,
}

/// 🎯 Semantic board event emitted after interaction or selection changes.
#[derive(Clone, Debug, PartialEq)]
pub enum BoardEvent {
	HoverChanged { id: Option<u64> },
	NodeMoved { id: NodeId, x: f32, y: f32 },
	SelectionChanged {
		edge_ids: Vec<EdgeId>,
		handle_ids: Vec<HandleId>,
		node_ids: Vec<NodeId>,
	},
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
	pub edges: Vec<CubicBezier>,
	pub handles: Vec<(HandleId, Vec2, f32)>,
	pub nodes: Vec<(NodeId, Vec2, f32)>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum HitObject {
	Edge(EdgeId),
	Handle(HandleId),
	Node(NodeId),
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum InteractionMode {
	DragNode {
		node_id: NodeId,
		offset: Vec2,
	},
	Idle,
}

impl Default for InteractionMode {
	fn default() -> Self {
		Self::Idle
	}
}
// #endregion 🔖Kinds

// #region 🔖Utilities
impl Default for Camera {
	fn default() -> Self {
		Self { x: 0.0, y: 0.0, zoom: 1.0 }
	}
}

impl Vec2 {
	fn dot(self, other: Self) -> f32 {
		(self.x * other.x) + (self.y * other.y)
	}

	fn length(self) -> f32 {
		self.dot(self).sqrt()
	}

	fn normalized(self) -> Self {
		let length = self.length();
		if length <= f32::EPSILON {
			return Self { x: 0.0, y: 0.0 };
		}
		Self { x: self.x / length, y: self.y / length }
	}
}

impl std::ops::Add for Vec2 {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Self { x: self.x + rhs.x, y: self.y + rhs.y }
	}
}

impl std::ops::Sub for Vec2 {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Self { x: self.x - rhs.x, y: self.y - rhs.y }
	}
}

impl std::ops::Mul<f32> for Vec2 {
	type Output = Self;

	fn mul(self, rhs: f32) -> Self::Output {
		Self { x: self.x * rhs, y: self.y * rhs }
	}
}

fn clamp(value: f32, min: f32, max: f32) -> f32 {
	value.max(min).min(max)
}

fn distance(left: Vec2, right: Vec2) -> f32 {
	(left - right).length()
}

fn handle_position(node: &Node, handle: &Handle) -> Vec2 {
	Vec2 {
		x: node.center.x + node.radius * handle.angle.cos(),
		y: node.center.y + node.radius * handle.angle.sin(),
	}
}

fn cubic_point(curve: CubicBezier, step: f32) -> Vec2 {
	let one_minus = 1.0 - step;
	let one_minus_squared = one_minus * one_minus;
	let one_minus_cubed = one_minus_squared * one_minus;
	let step_squared = step * step;
	let step_cubed = step_squared * step;
	Vec2 {
		x: curve.p0.x * one_minus_cubed
			+ 3.0 * curve.p1.x * one_minus_squared * step
			+ 3.0 * curve.p2.x * one_minus * step_squared
			+ curve.p3.x * step_cubed,
		y: curve.p0.y * one_minus_cubed
			+ 3.0 * curve.p1.y * one_minus_squared * step
			+ 3.0 * curve.p2.y * one_minus * step_squared
			+ curve.p3.y * step_cubed,
	}
}

fn distance_to_segment(point: Vec2, start: Vec2, end: Vec2) -> f32 {
	let segment = end - start;
	let segment_len_squared = segment.dot(segment);
	if segment_len_squared <= f32::EPSILON {
		return distance(point, start);
	}
	let projection = clamp((point - start).dot(segment) / segment_len_squared, 0.0, 1.0);
	let closest = start + (segment * projection);
	distance(point, closest)
}

fn distance_to_curve(point: Vec2, curve: CubicBezier, segments: usize) -> f32 {
	let mut smallest_distance = f32::MAX;
	let mut previous = curve.p0;
	for index in 1..=segments {
		let next = cubic_point(curve, index as f32 / segments as f32);
		smallest_distance = smallest_distance.min(distance_to_segment(point, previous, next));
		previous = next;
	}
	smallest_distance
}
// #endregion 🔖Utilities

// #region 🔖Engine
/// ⚙️ Single-file retained board engine prepared for a future WASM export surface.
#[derive(Clone, Debug, Default)]
pub struct BoardEngine {
	camera: Camera,
	edges: BTreeMap<EdgeId, Edge>,
	events: Vec<BoardEvent>,
	handles: BTreeMap<HandleId, Handle>,
	hover: Option<u64>,
	interaction: InteractionMode,
	nodes: BTreeMap<NodeId, Node>,
	selection: Selection,
}

impl BoardEngine {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn set_camera(&mut self, x: f64, y: f64, zoom: f64) {
		self.camera = Camera { x, y, zoom };
	}

	pub fn create_node(&mut self, id: NodeId, x: f32, y: f32, radius: f32, draggable: bool) {
		self.nodes.insert(
			id,
			Node {
				center: Vec2 { x, y },
				draggable,
				id,
				radius,
			},
		);
	}

	pub fn update_node(&mut self, id: NodeId, x: f32, y: f32, radius: f32) {
		if let Some(node) = self.nodes.get_mut(&id) {
			node.center = Vec2 { x, y };
			node.radius = radius;
		}
	}

	pub fn remove_node(&mut self, id: NodeId) {
		self.nodes.remove(&id);
		let removed_handles: Vec<HandleId> = self
			.handles
			.values()
			.filter(|handle| handle.node_id == id)
			.map(|handle| handle.id)
			.collect();
		for handle_id in removed_handles {
			self.remove_handle(handle_id);
		}
		self.selection.node_ids.remove(&id);
		self.push_selection_event();
	}

	pub fn create_handle(&mut self, id: HandleId, node_id: NodeId, angle: f32) {
		self.handles.insert(
			id,
			Handle {
				angle,
				id,
				node_id,
				radius: 8.0,
			},
		);
	}

	pub fn create_edge(&mut self, id: EdgeId, from_handle: HandleId, to_handle: HandleId) {
		self.edges.insert(
			id,
			Edge {
				from_handle,
				id,
				to_handle,
			},
		);
	}

	pub fn pointer_down(&mut self, x: f32, y: f32) {
		let point = Vec2 { x, y };
		match self.hit_test(point) {
			Some(HitObject::Node(node_id)) => {
				self.select_node(node_id);
				if let Some(node) = self.nodes.get(&node_id) {
					if node.draggable {
						self.interaction = InteractionMode::DragNode {
							node_id,
							offset: point - node.center,
						};
					}
				}
				self.update_hover(Some(node_id));
			}
			Some(HitObject::Handle(handle_id)) => {
				self.selection.handle_ids.clear();
				self.selection.edge_ids.clear();
				self.selection.node_ids.clear();
				self.selection.handle_ids.insert(handle_id);
				self.push_selection_event();
				self.update_hover(Some(handle_id));
				self.interaction = InteractionMode::Idle;
			}
			Some(HitObject::Edge(edge_id)) => {
				self.selection.handle_ids.clear();
				self.selection.node_ids.clear();
				self.selection.edge_ids.clear();
				self.selection.edge_ids.insert(edge_id);
				self.push_selection_event();
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

	pub fn pointer_move(&mut self, x: f32, y: f32) {
		let point = Vec2 { x, y };
		match self.interaction {
			InteractionMode::DragNode { node_id, offset } => {
				if let Some(node) = self.nodes.get_mut(&node_id) {
					node.center = point - offset;
					self.events.push(BoardEvent::NodeMoved {
						id: node_id,
						x: node.center.x,
						y: node.center.y,
					});
				}
			}
			InteractionMode::Idle => {
				self.update_hover(self.hit_test(point).map(|hit| match hit {
					HitObject::Edge(id) => id,
					HitObject::Handle(id) => id,
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
		for handle in self.handles.values() {
			if let Some(node) = self.nodes.get(&handle.node_id) {
				snapshot.handles.push((handle.id, handle_position(node, handle), handle.radius));
			}
		}
		for edge in self.edges.values() {
			if let Some(curve) = self.edge_curve(edge.id) {
				snapshot.edges.push(curve);
			}
		}
		snapshot
	}

	pub fn drain_events(&mut self) -> Vec<BoardEvent> {
		std::mem::take(&mut self.events)
	}

	pub fn edge_curve(&self, edge_id: EdgeId) -> Option<CubicBezier> {
		let edge = self.edges.get(&edge_id)?;
		let from_handle = self.handles.get(&edge.from_handle)?;
		let to_handle = self.handles.get(&edge.to_handle)?;
		let from_node = self.nodes.get(&from_handle.node_id)?;
		let to_node = self.nodes.get(&to_handle.node_id)?;
		let from_position = handle_position(from_node, from_handle);
		let to_position = handle_position(to_node, to_handle);
		let chord = distance(from_position, to_position);
		let control_length = clamp(chord * 0.35, 24.0, 240.0);
		let mut from_out = (from_position - from_node.center).normalized();
		if from_out.length() <= f32::EPSILON {
			from_out = (to_position - from_position).normalized();
		}
		let mut to_in = (to_node.center - to_position).normalized();
		if to_in.length() <= f32::EPSILON {
			to_in = (to_position - from_position).normalized();
		}
		Some(CubicBezier {
			p0: from_position,
			p1: from_position + (from_out * control_length),
			p2: to_position + (to_in * control_length),
			p3: to_position,
		})
	}

	fn remove_handle(&mut self, id: HandleId) {
		self.handles.remove(&id);
		let removed_edges: Vec<EdgeId> = self
			.edges
			.values()
			.filter(|edge| edge.from_handle == id || edge.to_handle == id)
			.map(|edge| edge.id)
			.collect();
		for edge_id in removed_edges {
			self.edges.remove(&edge_id);
			self.selection.edge_ids.remove(&edge_id);
		}
		self.selection.handle_ids.remove(&id);
	}

	fn select_node(&mut self, node_id: NodeId) {
		self.selection.edge_ids.clear();
		self.selection.handle_ids.clear();
		self.selection.node_ids.clear();
		self.selection.node_ids.insert(node_id);
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

	fn hit_test(&self, point: Vec2) -> Option<HitObject> {
		for handle in self.handles.values().rev() {
			let node = self.nodes.get(&handle.node_id)?;
			if distance(point, handle_position(node, handle)) <= handle.radius + 6.0 {
				return Some(HitObject::Handle(handle.id));
			}
		}
		for node in self.nodes.values().rev() {
			if distance(point, node.center) <= node.radius {
				return Some(HitObject::Node(node.id));
			}
		}
		for edge in self.edges.values().rev() {
			if let Some(curve) = self.edge_curve(edge.id) {
				if distance_to_curve(point, curve, 18) <= 8.0 {
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
	fn computes_handle_positions_and_edge_curves() {
		let mut engine = BoardEngine::new();
		engine.create_node(1, 0.0, 0.0, 40.0, true);
		engine.create_node(2, 300.0, 0.0, 40.0, true);
		engine.create_handle(10, 1, 0.0);
		engine.create_handle(20, 2, std::f32::consts::PI);
		engine.create_edge(100, 10, 20);

		let curve = engine.edge_curve(100).expect("edge curve should exist");
		assert!((curve.p0.x - 40.0).abs() < 0.001);
		assert!(curve.p0.y.abs() < 0.001);
		assert!((curve.p3.x - 260.0).abs() < 0.001);
		assert!(curve.p3.y.abs() < 0.001);
		let outward = curve.p0 - Vec2 { x: 0.0, y: 0.0 };
		let arm0 = curve.p1 - curve.p0;
		assert!(outward.normalized().dot(arm0.normalized()) > 0.99);
		let inward = Vec2 { x: 300.0, y: 0.0 } - curve.p3;
		let arm1 = curve.p3 - curve.p2;
		assert!(inward.normalized().dot(arm1.normalized()).abs() > 0.99);
	}

	#[test]
	fn drags_nodes_without_rebuilding_the_scene_catalog() {
		let mut engine = BoardEngine::new();
		engine.create_node(1, 0.0, 0.0, 30.0, true);

		engine.pointer_down(0.0, 0.0);
		engine.pointer_move(60.0, 25.0);
		engine.pointer_up();

		let node = engine.nodes.get(&1).expect("node should remain in the engine");
		assert_eq!(node.center, Vec2 { x: 60.0, y: 25.0 });

		let events = engine.drain_events();
		assert!(events.iter().any(|event| matches!(event, BoardEvent::SelectionChanged { node_ids, .. } if node_ids == &vec![1])));
		assert!(events.iter().any(|event| matches!(event, BoardEvent::NodeMoved { id: 1, x, y } if (*x - 60.0).abs() < 0.001 && (*y - 25.0).abs() < 0.001)));
	}

	#[test]
	fn hit_tests_handles_before_nodes_and_edges() {
		let mut engine = BoardEngine::new();
		engine.create_node(1, 0.0, 0.0, 40.0, true);
		engine.create_node(2, 200.0, 0.0, 40.0, true);
		engine.create_handle(10, 1, 0.0);
		engine.create_handle(20, 2, std::f32::consts::PI);
		engine.create_edge(100, 10, 20);

		let handle_point = handle_position(engine.nodes.get(&1).unwrap(), engine.handles.get(&10).unwrap());
		engine.pointer_down(handle_point.x, handle_point.y);

		let events = engine.drain_events();
		assert!(events.iter().any(|event| matches!(event, BoardEvent::SelectionChanged { handle_ids, .. } if handle_ids == &vec![10])));
	}

	#[test]
	fn renders_snapshot_for_nodes_handles_and_edges() {
		let mut engine = BoardEngine::new();
		engine.create_node(1, 10.0, 20.0, 18.0, true);
		engine.create_node(2, 120.0, 20.0, 18.0, true);
		engine.create_handle(10, 1, 0.0);
		engine.create_handle(20, 2, std::f32::consts::PI);
		engine.create_edge(100, 10, 20);

		let snapshot = engine.render_snapshot();
		assert_eq!(snapshot.nodes.len(), 2);
		assert_eq!(snapshot.handles.len(), 2);
		assert_eq!(snapshot.edges.len(), 1);
	}
}
// #endregion 🔖Tests
