//! 🎛️ Authoritative board runtime: scene graph, camera, pointer/selection, and Vello scene encoding.

mod geom;
mod types;

pub use types::*;

use crate::vcompute::{
	compute_edge_bezier_points, distance_between, distance_point_to_cubic_bezier, handle_position_on_circle,
	handle_position_on_rectangle,
};
use geom::{
	point_in_polygon, polygon_contains_world_box, polygon_intersects_world_box, segment_intersects_polygon,
	segment_intersects_world_box, world_box_contains_box, world_box_contains_point, world_box_from_points,
	world_boxes_overlap, inflate_world_box, cubic_bezier_point, WorldBox,
};
use serde_json::json;
use std::collections::{BTreeMap, BTreeSet};
use vello::kurbo::{Affine, Circle, CubicBez, Point, Rect, Stroke, Vec2};
use vello::peniko::{Color, Fill};
use vello::Scene;

const GRID_WORLD_STEP: f64 = 96.0;
const EDGE_HIT_TOLERANCE_PX: f64 = 8.0;
const HANDLE_HIT_TOLERANCE_PX: f64 = 10.0;
const HANDLE_DRAW_MIN_ZOOM: f64 = 0.45;
const SELECTION_LASSO_MIN_POINT_DISTANCE_PX: f64 = 3.0;
pub const BOARD_CAMERA_ZOOM_MIN: f64 = 0.05;
pub const BOARD_CAMERA_ZOOM_MAX: f64 = 32.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeShape {
	Circle,
	Rectangle,
}

#[derive(Clone, Debug)]
pub struct NodeData {
	pub id: String,
	pub x: f64,
	pub y: f64,
	pub shape: NodeShape,
	pub radius: f64,
	pub width: f64,
	pub height: f64,
	pub draggable: bool,
	pub selected: bool,
	pub visible: bool,
	pub style: Option<String>,
	pub text: Option<String>,
}

#[derive(Clone, Debug)]
pub struct HandleData {
	pub id: String,
	pub node_id: String,
	pub angle: f64,
	pub radius: f64,
	pub selected: bool,
	pub visible: bool,
	pub style: Option<String>,
}

#[derive(Clone, Debug)]
pub struct EdgeData {
	pub id: String,
	pub from: String,
	pub to: String,
	pub selected: bool,
	pub visible: bool,
	pub style: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Camera {
	pub x: f64,
	pub y: f64,
	pub zoom: f64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SelectionOptions {
	pub method: String,
	pub mode: String,
	pub target: String,
}

#[derive(Clone, Debug)]
pub enum Interaction {
	None,
	Pan {
		origin: Camera,
		start_screen: Point,
	},
	DragNode {
		node_id: String,
		offset: Vec2,
	},
	Selection {
		initial_ids: BTreeSet<String>,
		points: Vec<Point>,
		screen_points: Vec<Point>,
		start: Point,
		start_screen: Point,
	},
}

impl Default for Interaction {
	fn default() -> Self {
		Self::None
	}
}

#[derive(Clone, Debug)]
pub struct BoardHost {
	pub camera: Camera,
	pub nodes: BTreeMap<String, NodeData>,
	pub handles: BTreeMap<String, HandleData>,
	pub edges: BTreeMap<String, EdgeData>,
	pub selection: BTreeSet<String>,
	pub selection_options: SelectionOptions,
	pub hovered_id: Option<String>,
	pub interaction: Interaction,
	pub width: u32,
	pub height: u32,
	pub dpr: f64,
	pub world_raster_tiling: String,
	pub events: Vec<serde_json::Value>,
}

impl Default for Camera {
	fn default() -> Self {
		Self { x: 0.0, y: 0.0, zoom: 1.0 }
	}
}

impl Default for BoardHost {
	fn default() -> Self {
		Self {
			camera: Camera::default(),
			nodes: BTreeMap::new(),
			handles: BTreeMap::new(),
			edges: BTreeMap::new(),
			selection: BTreeSet::new(),
			selection_options: SelectionOptions {
				method: "rectangle".into(),
				mode: "invertive".into(),
				target: "nodes&edges".into(),
			},
			hovered_id: None,
			interaction: Interaction::None,
			width: 1,
			height: 1,
			dpr: 1.0,
			world_raster_tiling: "none".into(),
			events: Vec::new(),
		}
	}
}

impl BoardHost {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn set_size(&mut self, width: u32, height: u32, dpr: f64) {
		self.width = width.max(1);
		self.height = height.max(1);
		self.dpr = dpr.max(1.0);
	}

	pub fn set_camera(&mut self, x: f64, y: f64, zoom: f64) {
		self.camera.x = x;
		self.camera.y = y;
		self.camera.zoom = zoom.clamp(BOARD_CAMERA_ZOOM_MIN, BOARD_CAMERA_ZOOM_MAX);
		self.push_event("camera", json!({ "x": self.camera.x, "y": self.camera.y, "zoom": self.camera.zoom }));
	}

	pub fn set_selection_options(&mut self, method: &str, mode: &str, target: &str) {
		self.selection_options.method = method.into();
		self.selection_options.mode = mode.into();
		self.selection_options.target = target.into();
	}

	fn push_event(&mut self, name: &str, payload: serde_json::Value) {
		self.events.push(json!({ "name": name, "payload": payload }));
	}

	pub fn drain_events_json(&mut self) -> String {
		let out = serde_json::to_string(&self.events).unwrap_or_else(|_| "[]".into());
		self.events.clear();
		out
	}

	pub fn set_selection_ids(&mut self, ids: &[String]) {
		self.selection = ids.iter().cloned().collect();
		for n in self.nodes.values_mut() {
			n.selected = self.selection.contains(&n.id);
		}
		for h in self.handles.values_mut() {
			h.selected = self.selection.contains(&h.id);
		}
		for e in self.edges.values_mut() {
			e.selected = self.selection.contains(&e.id);
		}
		let mut sorted: Vec<_> = self.selection.iter().cloned().collect();
		sorted.sort();
		self.push_event("select", json!({ "ids": sorted }));
	}

	pub fn world_to_screen(&self, p: Point) -> Point {
		Point::new(
			(p.x - self.camera.x) * self.camera.zoom + self.width as f64 / 2.0,
			(p.y - self.camera.y) * self.camera.zoom + self.height as f64 / 2.0,
		)
	}

	pub fn screen_to_world(&self, p: Point) -> Point {
		Point::new(
			(p.x - self.width as f64 / 2.0) / self.camera.zoom + self.camera.x,
			(p.y - self.height as f64 / 2.0) / self.camera.zoom + self.camera.y,
		)
	}

	fn handle_world_pos(&self, h: &HandleData) -> Option<Point> {
		let n = self.nodes.get(&h.node_id)?;
		Some(match n.shape {
			NodeShape::Circle => handle_position_on_circle(Point::new(n.x, n.y), n.radius, h.angle),
			NodeShape::Rectangle => handle_position_on_rectangle(Point::new(n.x, n.y), n.width, n.height, h.angle),
		})
	}

	fn edge_curve(&self, e: &EdgeData) -> Option<CubicBez> {
		let fh = self.handles.get(&e.from)?;
		let th = self.handles.get(&e.to)?;
		let fn_ = self.nodes.get(&fh.node_id)?;
		let tn = self.nodes.get(&th.node_id)?;
		let fp = self.handle_world_pos(fh)?;
		let tp = self.handle_world_pos(th)?;
		Some(compute_edge_bezier_points(
			fp,
			tp,
			Point::new(fn_.x, fn_.y),
			Point::new(tn.x, tn.y),
		))
	}

	pub fn resolve_hit_world(&self, point: Point) -> Option<String> {
		let zoom = self.camera.zoom;
		for h in self.handles.values().rev() {
			if !h.visible {
				continue;
			}
			let pos = self.handle_world_pos(h)?;
			let tol = (HANDLE_HIT_TOLERANCE_PX / zoom) + h.radius;
			if distance_between(point, pos) <= tol {
				return Some(h.id.clone());
			}
		}
		for n in self.nodes.values().rev() {
			if !n.visible {
				continue;
			}
			match n.shape {
				NodeShape::Rectangle => {
					let hw = n.width / 2.0;
					let hh = n.height / 2.0;
					if (point.x - n.x).abs() <= hw && (point.y - n.y).abs() <= hh {
						return Some(n.id.clone());
					}
				}
				NodeShape::Circle => {
					if distance_between(point, Point::new(n.x, n.y)) <= n.radius {
						return Some(n.id.clone());
					}
				}
			}
		}
		for e in self.edges.values().rev() {
			if !e.visible {
				continue;
			}
			if let Some(c) = self.edge_curve(e) {
				if distance_point_to_cubic_bezier(point, c, 18) <= EDGE_HIT_TOLERANCE_PX / zoom {
					return Some(e.id.clone());
				}
			}
		}
		None
	}

	pub fn sync_descriptor(&mut self, desc: &SceneDescriptorJson) {
		let want_nodes: BTreeSet<_> = desc.nodes.iter().map(|n| n.id.clone()).collect();
		let want_handles: BTreeSet<_> = desc.handles.iter().map(|h| h.id.clone()).collect();
		let want_edges: BTreeSet<_> = desc.edges.iter().map(|e| e.id.clone()).collect();
		self.edges.retain(|id, _| want_edges.contains(id));
		self.handles.retain(|id, _| want_handles.contains(id));
		self.nodes.retain(|id, _| want_nodes.contains(id));
		for n in &desc.nodes {
			let shape = if n.shape.as_deref() == Some("rectangle") {
				NodeShape::Rectangle
			} else {
				NodeShape::Circle
			};
			let (radius, width, height) = match shape {
				NodeShape::Circle => (n.radius.unwrap_or(0.0), 0.0, 0.0),
				NodeShape::Rectangle => (0.0, n.width.unwrap_or(0.0), n.height.unwrap_or(0.0)),
			};
			self.nodes.insert(
				n.id.clone(),
				NodeData {
					id: n.id.clone(),
					x: n.x,
					y: n.y,
					shape,
					radius,
					width,
					height,
					draggable: n.draggable.unwrap_or(true),
					selected: n.selected.unwrap_or(false),
					visible: n.visible.unwrap_or(true),
					style: n.style.clone(),
					text: n.text.clone(),
				},
			);
		}
		for h in &desc.handles {
			self.handles.insert(
				h.id.clone(),
				HandleData {
					id: h.id.clone(),
					node_id: h.node_id.clone(),
					angle: h.angle,
					radius: h.radius.unwrap_or(8.0),
					selected: h.selected.unwrap_or(false),
					visible: h.visible.unwrap_or(true),
					style: h.style.clone(),
				},
			);
		}
		for e in &desc.edges {
			let existed = self.edges.contains_key(&e.id);
			self.edges.insert(
				e.id.clone(),
				EdgeData {
					id: e.id.clone(),
					from: e.from.clone(),
					to: e.to.clone(),
					selected: e.selected.unwrap_or(false),
					visible: e.visible.unwrap_or(true),
					style: e.style.clone(),
				},
			);
			if !existed {
				self.push_event(
					"edgeCreate",
					json!({ "id": e.id, "from": e.from, "to": e.to }),
				);
			}
		}
		let mut new_selection = BTreeSet::new();
		for n in &desc.nodes {
			if n.selected == Some(true) {
				new_selection.insert(n.id.clone());
			}
		}
		for h in &desc.handles {
			if h.selected == Some(true) {
				new_selection.insert(h.id.clone());
			}
		}
		for e in &desc.edges {
			if e.selected == Some(true) {
				new_selection.insert(e.id.clone());
			}
		}
		let prev_sel = self.selection.clone();
		self.selection = new_selection;
		for n in self.nodes.values_mut() {
			n.selected = self.selection.contains(&n.id);
		}
		for h in self.handles.values_mut() {
			h.selected = self.selection.contains(&h.id);
		}
		for e in self.edges.values_mut() {
			e.selected = self.selection.contains(&e.id);
		}
		if prev_sel != self.selection {
			let mut sorted: Vec<_> = self.selection.iter().cloned().collect();
			sorted.sort();
			self.push_event("select", json!({ "ids": sorted }));
		}
	}

	pub fn clear_scene(&mut self) {
		self.edges.clear();
		self.handles.clear();
		self.nodes.clear();
		self.selection.clear();
	}

	pub fn parse_fixture_v1(&mut self, raw: &serde_json::Value) -> bool {
		let f: FixtureV1Json = match serde_json::from_value(raw.clone()) {
			Ok(v) => v,
			Err(_) => return false,
		};
		if f.schema != "elements.board.fixture/v1" {
			return false;
		}
		self.set_camera(f.camera.x, f.camera.y, f.camera.zoom);
		self.clear_scene();
		let mut desc = SceneDescriptorJson::default();
		for entry in f.nodes {
			let Some(obj) = entry.as_object() else {
				return false;
			};
			let Some(id) = obj.get("id").and_then(|v| v.as_str()) else {
				return false;
			};
			let Some(x) = obj.get("x").and_then(|v| v.as_f64()) else {
				return false;
			};
			let Some(y) = obj.get("y").and_then(|v| v.as_f64()) else {
				return false;
			};
			if !x.is_finite() || !y.is_finite() {
				return false;
			}
			let text = obj
				.get("text")
				.and_then(|v| v.as_str())
				.map(String::from)
				.or_else(|| obj.get("label").and_then(|v| v.as_str()).map(String::from));
			let Some(handles_arr) = obj.get("handles").and_then(|v| v.as_array()) else {
				return false;
			};
			let mut handles: Vec<HandleDescJson> = Vec::new();
			for h in handles_arr {
				let Some(ho) = h.as_object() else {
					return false;
				};
				let Some(hid) = ho.get("id").and_then(|v| v.as_str()) else {
					return false;
				};
				let Some(angle) = ho.get("angle").and_then(|v| v.as_f64()) else {
					return false;
				};
				if !angle.is_finite() {
					return false;
				}
				handles.push(HandleDescJson {
					id: hid.into(),
					node_id: id.into(),
					angle,
					radius: None,
					selected: None,
					style: None,
					user_data: None,
					visible: None,
				});
			}
			let shape_str = obj.get("shape").and_then(|v| v.as_str());
			if shape_str == Some("rectangle") {
				let Some(width) = obj.get("width").and_then(|v| v.as_f64()) else {
					return false;
				};
				let Some(height) = obj.get("height").and_then(|v| v.as_f64()) else {
					return false;
				};
				if width <= 0.0 || height <= 0.0 {
					return false;
				}
				desc.nodes.push(NodeDescJson {
					id: id.into(),
					x,
					y,
					draggable: None,
					selected: None,
					style: None,
					text,
					user_data: None,
					visible: None,
					shape: Some("rectangle".into()),
					radius: None,
					width: Some(width),
					height: Some(height),
				});
			} else {
				let Some(radius) = obj.get("radius").and_then(|v| v.as_f64()) else {
					return false;
				};
				if radius <= 0.0 {
					return false;
				}
				desc.nodes.push(NodeDescJson {
					id: id.into(),
					x,
					y,
					draggable: None,
					selected: None,
					style: None,
					text,
					user_data: None,
					visible: None,
					shape: Some("circle".into()),
					radius: Some(radius),
					width: None,
					height: None,
				});
			}
			desc.handles.extend(handles);
		}
		for entry in f.edges {
			let Some(e) = entry.as_object() else {
				return false;
			};
			let Some(id) = e.get("id").and_then(|v| v.as_str()) else {
				return false;
			};
			let Some(from) = e.get("from").and_then(|v| v.as_str()) else {
				return false;
			};
			let Some(to) = e.get("to").and_then(|v| v.as_str()) else {
				return false;
			};
			desc.edges.push(EdgeDescJson {
				id: id.into(),
				from: from.into(),
				to: to.into(),
				selected: None,
				style: None,
				user_data: None,
				visible: None,
			});
		}
		self.sync_descriptor(&desc);
		true
	}

	pub fn build_vello_scene(&self) -> Scene {
		let mut scene = Scene::new();
		let stroke_grid = Stroke::new(1.0);
		let step = GRID_WORLD_STEP * self.camera.zoom;
		if step >= 18.0 {
			let origin = self.world_to_screen(Point::new(0.0, 0.0));
			let x_off = ((origin.x % step) + step) % step;
			let y_off = ((origin.y % step) + step) % step;
			let w = self.width as f64;
			let h = self.height as f64;
			let mut p = vello::kurbo::BezPath::new();
			let mut x = x_off;
			while x <= w {
				p.move_to(Point::new(x, 0.0));
				p.line_to(Point::new(x, h));
				x += step;
			}
			let mut y = y_off;
			while y <= h {
				p.move_to(Point::new(0.0, y));
				p.line_to(Point::new(w, y));
				y += step;
			}
			scene.stroke(
				&stroke_grid,
				Affine::IDENTITY,
				Color::new([0.58, 0.64, 0.72, 0.18]),
				None,
				&p,
			);
		}
		for n in self.nodes.values() {
			if !n.visible {
				continue;
			}
			let fill = node_fill_color(n.selected);
			let stroke_c = node_stroke_color(n.selected);
			let sw = 2.0_f64;
			match n.shape {
				NodeShape::Circle => {
					let c = self.world_to_screen(Point::new(n.x, n.y));
					let r = (n.radius * self.camera.zoom).max(1.0);
					let circle = Circle::new(c, r);
					scene.fill(Fill::NonZero, Affine::IDENTITY, fill, None, &circle);
					scene.stroke(&Stroke::new(sw), Affine::IDENTITY, stroke_c, None, &circle);
				}
				NodeShape::Rectangle => {
					let hw = n.width / 2.0;
					let hh = n.height / 2.0;
					let p0 = self.world_to_screen(Point::new(n.x - hw, n.y - hh));
					let p1 = self.world_to_screen(Point::new(n.x + hw, n.y + hh));
					let r = Rect::from_points(p0, p1);
					scene.fill(Fill::NonZero, Affine::IDENTITY, fill, None, &r);
					scene.stroke(&Stroke::new(sw), Affine::IDENTITY, stroke_c, None, &r);
				}
			}
		}
		for h in self.handles.values() {
			if !h.visible || self.camera.zoom < HANDLE_DRAW_MIN_ZOOM {
				continue;
			}
			let Some(wp) = self.handle_world_pos(h) else { continue };
			let c = self.world_to_screen(wp);
			let r = (h.radius * self.camera.zoom).max(1.0);
			let circle = Circle::new(c, r);
			let fill = handle_fill(h.selected);
			let stroke_c = handle_stroke(h.selected);
			scene.fill(Fill::NonZero, Affine::IDENTITY, fill, None, &circle);
			scene.stroke(&Stroke::new(2.0), Affine::IDENTITY, stroke_c, None, &circle);
		}
		let mut curves: Vec<CubicBez> = Vec::new();
		for e in self.edges.values() {
			if !e.visible {
				continue;
			}
			if let Some(c) = self.edge_curve(e) {
				let p0 = self.world_to_screen(c.p0);
				let p1 = self.world_to_screen(c.p1);
				let p2 = self.world_to_screen(c.p2);
				let p3 = self.world_to_screen(c.p3);
				curves.push(CubicBez::new(p0, p1, p2, p3));
			}
		}
		let edge_sw = 2.0 * self.camera.zoom.max(0.75);
		for c in &curves {
			scene.stroke(
				&Stroke::new(edge_sw),
				Affine::IDENTITY,
				Color::new([0.28, 0.33, 0.41, 1.0]),
				None,
				c,
			);
		}
		scene
	}

	pub fn encoded_scene_hint(&self) -> usize {
		let s = self.build_vello_scene();
		s.encoding().path_tags.len()
	}

	pub fn update_hover_from_world(&mut self, world: Point) {
		let next = self.resolve_hit_world(world);
		self.set_hovered_id(next);
	}

	pub fn set_hovered_id(&mut self, id: Option<String>) {
		if self.hovered_id == id {
			return;
		}
		self.hovered_id = id.clone();
		self.push_event("hover", json!({ "id": id }));
	}

	pub fn wheel_screen(&mut self, sx: f64, sy: f64, delta_y: f64) {
		let zoom_factor = if delta_y < 0.0 { 1.1 } else { 0.9 };
		let next_zoom = (self.camera.zoom * zoom_factor).clamp(BOARD_CAMERA_ZOOM_MIN, BOARD_CAMERA_ZOOM_MAX);
		let screen = Point::new(sx, sy);
		let world_before = self.screen_to_world(screen);
		let nx = world_before.x - (sx - self.width as f64 / 2.0) / next_zoom;
		let ny = world_before.y - (sy - self.height as f64 / 2.0) / next_zoom;
		self.set_camera(nx, ny, next_zoom);
	}

	pub fn delete_selection(&mut self) {
		let edge_ids: Vec<_> = self.selection.iter().filter(|id| self.edges.contains_key(*id)).cloned().collect();
		for id in &edge_ids {
			self.edges.remove(id);
			self.push_event("edgeDelete", json!({ "id": id }));
		}
		let node_ids: Vec<_> = self.selection.iter().filter(|id| self.nodes.contains_key(*id)).cloned().collect();
		for nid in &node_ids {
			let handle_ids: Vec<_> = self
				.handles
				.iter()
				.filter(|(_, h)| &h.node_id == nid)
				.map(|(k, _)| k.clone())
				.collect();
			for hid in handle_ids {
				let eids: Vec<_> = self
					.edges
					.iter()
					.filter(|(_, e)| e.from == hid || e.to == hid)
					.map(|(k, _)| k.clone())
					.collect();
				for eid in eids {
					self.edges.remove(&eid);
					self.selection.remove(&eid);
					self.push_event("edgeDelete", json!({ "id": eid }));
				}
				self.handles.remove(&hid);
				self.selection.remove(&hid);
			}
			self.nodes.remove(nid);
			self.push_event("nodeDelete", json!({ "id": nid }));
		}
		for id in edge_ids {
			self.selection.remove(&id);
		}
		for id in node_ids {
			self.selection.remove(&id);
		}
		let ids: Vec<_> = self.selection.iter().cloned().collect();
		self.set_selection_ids(&ids);
	}

	pub fn pointer_down_screen(&mut self, sx: f64, sy: f64, button: u8, shift: bool) {
		let screen = Point::new(sx, sy);
		let world = self.screen_to_world(screen);
		let hit = self.resolve_hit_world(world);
		if button == 1 || (hit.is_none() && shift) {
			self.interaction = Interaction::Pan {
				origin: self.camera.clone(),
				start_screen: screen,
			};
			return;
		}
		if let Some(ref hid) = hit {
			if let Some(node) = self.nodes.get(hid) {
				if node.draggable {
					let nid = hid.clone();
					let nx = node.x;
					let ny = node.y;
					self.set_selection_ids(&[nid.clone()]);
					self.interaction = Interaction::DragNode {
						node_id: nid,
						offset: world - Point::new(nx, ny),
					};
					self.set_hovered_id(hit);
					return;
				}
			}
		}
		if hit.is_none() && button == 0 {
			self.interaction = Interaction::Selection {
				initial_ids: self.selection.clone(),
				points: vec![world],
				screen_points: vec![screen],
				start: world,
				start_screen: screen,
			};
			self.set_hovered_id(None);
			return;
		}
		self.interaction = Interaction::None;
		if let Some(id) = hit {
			self.set_selection_ids(&[id.clone()]);
			self.set_hovered_id(Some(id));
		} else {
			self.set_selection_ids(&[]);
			self.set_hovered_id(None);
		}
	}

	pub fn pointer_move_screen(&mut self, sx: f64, sy: f64) {
		let screen = Point::new(sx, sy);
		let world = self.screen_to_world(screen);
		match std::mem::replace(&mut self.interaction, Interaction::None) {
			Interaction::DragNode { node_id, offset } => {
				let nid = node_id.clone();
				let off = offset;
				if let Some(n) = self.nodes.get_mut(&nid) {
					let nx = world.x - off.x;
					let ny = world.y - off.y;
					n.x = nx;
					n.y = ny;
					self.push_event("nodeMove", json!({ "id": nid, "x": nx, "y": ny }));
				}
				self.interaction = Interaction::DragNode { node_id, offset };
			}
			Interaction::Pan { origin, start_screen } => {
				let delta = screen - start_screen;
				let nx = origin.x - delta.x / origin.zoom;
				let ny = origin.y - delta.y / origin.zoom;
				self.set_camera(nx, ny, origin.zoom);
				self.interaction = Interaction::Pan {
					origin,
					start_screen,
				};
			}
			Interaction::Selection {
				mut points,
				mut screen_points,
				start,
				initial_ids,
				start_screen,
			} => {
				let last_screen = screen_points.last().copied().unwrap_or(start_screen);
				let add_point = self.selection_options.method == "lasso"
					|| distance_between(screen, last_screen) >= SELECTION_LASSO_MIN_POINT_DISTANCE_PX;
				if add_point {
					points.push(world);
					screen_points.push(screen);
				} else if !points.is_empty() {
					let last = points.len() - 1;
					points[last] = world;
					let ls = screen_points.len() - 1;
					screen_points[ls] = screen;
				}
				let initial = initial_ids.clone();
				let pts = points.clone();
				let next = self.resolve_area_selection_with_initial(&initial, start, &pts);
				let ids: Vec<_> = next.iter().cloned().collect();
				self.set_selection_ids(&ids);
				self.interaction = Interaction::Selection {
					initial_ids,
					points,
					screen_points,
					start,
					start_screen,
				};
			}
			Interaction::None => {
				self.interaction = Interaction::None;
				self.update_hover_from_world(world);
			}
		}
	}

	pub fn pointer_up_screen(&mut self, sx: f64, sy: f64) {
		let screen = Point::new(sx, sy);
		let world = self.screen_to_world(screen);
		if let Interaction::Selection {
			mut points,
			mut screen_points,
			start,
			initial_ids,
			..
		} = std::mem::take(&mut self.interaction)
		{
			points.push(world);
			screen_points.push(screen);
			let next = self.resolve_area_selection_with_initial(&initial_ids, start, &points);
			let ids: Vec<_> = next.iter().cloned().collect();
			self.set_selection_ids(&ids);
			let _ = screen_points;
			return;
		}
		self.interaction = Interaction::None;
	}

	fn node_world_bounds(&self, n: &NodeData, pad: f64) -> WorldBox {
		let raw = match n.shape {
			NodeShape::Rectangle => {
				let hw = n.width / 2.0;
				let hh = n.height / 2.0;
				WorldBox {
					min_x: n.x - hw,
					min_y: n.y - hh,
					max_x: n.x + hw,
					max_y: n.y + hh,
				}
			}
			NodeShape::Circle => WorldBox {
				min_x: n.x - n.radius,
				min_y: n.y - n.radius,
				max_x: n.x + n.radius,
				max_y: n.y + n.radius,
			},
		};
		inflate_world_box(raw, pad)
	}

	fn selection_drag_shape_world(&self, start: Point, points: &[Point]) -> Option<(WorldBox, bool, Vec<Point>)> {
		let last = points.last().copied().unwrap_or(start);
		let enclosing = last.x >= start.x;
		if self.selection_options.method == "lasso" && points.len() >= 3 {
			let poly: Vec<Point> = points.to_vec();
			let b = world_box_from_points(&poly)?;
			return Some((b, enclosing, poly));
		}
		let b = world_box_from_points(&[start, last])?;
		let poly = vec![
			Point::new(b.min_x, b.min_y),
			Point::new(b.max_x, b.min_y),
			Point::new(b.max_x, b.max_y),
			Point::new(b.min_x, b.max_y),
		];
		Some((b, enclosing, poly))
	}

	fn selection_contains_node(&self, n: &NodeData, box_: WorldBox, enclosing: bool, polygon: &[Point]) -> bool {
		let bounds = self.node_world_bounds(n, 0.0);
		let lasso = self.selection_options.method == "lasso";
		if enclosing {
			if lasso {
				polygon_contains_world_box(polygon, bounds)
			} else {
				world_box_contains_box(box_, bounds)
			}
		} else if lasso {
			polygon_intersects_world_box(polygon, bounds)
		} else {
			world_boxes_overlap(box_, bounds)
		}
	}

	fn selection_contains_edge(&self, c: CubicBez, box_: WorldBox, enclosing: bool, polygon: &[Point]) -> bool {
		const STEPS: usize = 24;
		let mut samples = Vec::with_capacity(STEPS + 1);
		for i in 0..=STEPS {
			let t = i as f64 / STEPS as f64;
			samples.push(cubic_bezier_point(c, t));
		}
		let lasso = self.selection_options.method == "lasso";
		if enclosing {
			if lasso {
				samples.iter().all(|&p| point_in_polygon(p, polygon))
			} else {
				samples.iter().all(|&p| world_box_contains_point(box_, p))
			}
		} else if lasso {
			(1..samples.len()).any(|i| segment_intersects_polygon(samples[i - 1], samples[i], polygon))
		} else {
			(1..samples.len()).any(|i| segment_intersects_world_box(samples[i - 1], samples[i], box_))
		}
	}

	fn resolve_area_selection_with_initial(&self, initial: &BTreeSet<String>, start: Point, points: &[Point]) -> BTreeSet<String> {
		let Some((box_, enclosing, ref polygon)) = self.selection_drag_shape_world(start, points) else {
			return initial.clone();
		};
		let mut hits = BTreeSet::new();
		let t = self.selection_options.target.as_str();
		if t == "nodes" || t == "nodes&edges" {
			for n in self.nodes.values() {
				if n.visible && self.selection_contains_node(n, box_, enclosing, polygon) {
					hits.insert(n.id.clone());
				}
			}
		}
		if t == "edges" || t == "nodes&edges" {
			for e in self.edges.values() {
				if !e.visible {
					continue;
				}
				if let Some(c) = self.edge_curve(e) {
					if self.selection_contains_edge(c, box_, enclosing, polygon) {
						hits.insert(e.id.clone());
					}
				}
			}
		}
		let mut next = initial.clone();
		for id in &hits {
			match self.selection_options.mode.as_str() {
				"additive" => {
					next.insert(id.clone());
				}
				"subtractive" => {
					next.remove(id);
				}
				_ => {
					if next.contains(id) {
						next.remove(id);
					} else {
						next.insert(id.clone());
					}
				}
			}
		}
		next
	}
}

fn node_fill_color(sel: bool) -> Color {
	if sel {
		Color::new([0.6, 0.96, 0.9, 1.0])
	} else {
		Color::new([0.89, 0.91, 0.94, 1.0])
	}
}

fn node_stroke_color(sel: bool) -> Color {
	if sel {
		Color::new([0.06, 0.46, 0.43, 1.0])
	} else {
		Color::new([0.06, 0.09, 0.16, 1.0])
	}
}

fn handle_fill(sel: bool) -> Color {
	if sel {
		Color::new([0.08, 0.73, 0.65, 1.0])
	} else {
		Color::WHITE
	}
}

fn handle_stroke(sel: bool) -> Color {
	if sel {
		Color::new([0.06, 0.46, 0.43, 1.0])
	} else {
		Color::new([0.06, 0.09, 0.16, 1.0])
	}
}
