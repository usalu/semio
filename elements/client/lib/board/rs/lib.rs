//! 🎛️ Single-source board crate: Vello/kurbo geometry (`vcompute`), selection predicates (`geom_sel`), serde scene JSON (`scene_json`), interactive `BoardHost`, retained `BoardEngine`, and wasm-bindgen facades — all in this file (no sibling `src/` modules).
#![allow(clippy::missing_errors_doc, reason = "Board engine is internal to the elements board bundle.")]

mod vcompute {
	use vello::kurbo::{Affine, CubicBez, ParamCurve, Point, Vec2, Stroke};
	use vello::peniko::Color;
	use vello::Scene;

	#[inline]
	pub fn clamp_f64(value: f64, min: f64, max: f64) -> f64 {
		value.max(min).min(max)
	}

	#[inline]
	pub fn distance_between(left: Point, right: Point) -> f64 {
		(right - left).hypot()
	}

	#[inline]
	pub fn normalize_or_zero(vector: Vec2) -> Vec2 {
		let len = vector.hypot();
		if len <= f64::EPSILON {
			return Vec2::new(0.0, 0.0);
		}
		vector / len
	}

	#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
	pub fn ray_from_origin_to_axis_aligned_rectangle_edge(hw: f64, hh: f64, ux: f64, uy: f64) -> Point {
		let mut t_best = f64::INFINITY;
		if ux.abs() > 1e-12 {
			let tx = ux.signum() * hw / ux;
			let y_at = uy * tx;
			if tx > 0.0 && y_at.abs() <= hh + 1e-9 {
				t_best = t_best.min(tx);
			}
		}
		if uy.abs() > 1e-12 {
			let ty = uy.signum() * hh / uy;
			let x_at = ux * ty;
			if ty > 0.0 && x_at.abs() <= hw + 1e-9 {
				t_best = t_best.min(ty);
			}
		}
		if !t_best.is_finite() || t_best <= 0.0 || t_best == f64::INFINITY {
			return Point::new(hw, 0.0);
		}
		Point::new(ux * t_best, uy * t_best)
	}

	pub fn handle_position_on_circle(center: Point, radius: f64, angle: f64) -> Point {
		let ux = angle.cos();
		let uy = angle.sin();
		center + Vec2::new(ux * radius, uy * radius)
	}

	#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
	pub fn handle_position_on_rectangle(center: Point, width: f64, height: f64, angle: f64) -> Point {
		let hw = width / 2.0;
		let hh = height / 2.0;
		let ux = angle.cos();
		let uy = angle.sin();
		let local = ray_from_origin_to_axis_aligned_rectangle_edge(hw, hh, ux, uy);
		center + Vec2::new(local.x, local.y)
	}

	pub fn compute_edge_bezier_points(from_point: Point, to_point: Point, from_center: Point, to_center: Point) -> CubicBez {
		let mut from_out = normalize_or_zero(from_point - from_center);
		if from_out == Vec2::new(0.0, 0.0) {
			from_out = normalize_or_zero(to_point - from_point);
		}
		let mut to_in = normalize_or_zero(to_center - to_point);
		if to_in == Vec2::new(0.0, 0.0) {
			to_in = normalize_or_zero(to_point - from_point);
		}
		let handle_distance = distance_between(from_point, to_point);
		let control_length = clamp_f64(handle_distance * 0.35, 24.0, 240.0);
		let p1 = from_point + from_out * control_length;
		let p2 = to_point + to_in * control_length;
		CubicBez::new(from_point, p1, p2, to_point)
	}

	pub fn distance_point_to_cubic_bezier(point: Point, curve: CubicBez, segments: usize) -> f64 {
		let mut smallest = f64::INFINITY;
		let mut previous = curve.eval(0.0);
		let n = segments.max(1);
		for index in 1..=n {
			let t = index as f64 / n as f64;
			let next = curve.eval(t);
			smallest = smallest.min(distance_to_segment(point, previous, next));
			previous = next;
		}
		smallest
	}

	fn distance_to_segment(point: Point, start: Point, end: Point) -> f64 {
		let segment = end - start;
		let segment_len_squared = segment.dot(segment);
		if segment_len_squared <= f64::EPSILON {
			return distance_between(point, start);
		}
		let projection = clamp_f64((point - start).dot(segment) / segment_len_squared, 0.0, 1.0);
		let closest = start + segment * projection;
		distance_between(point, closest)
	}

	pub fn encode_board_vello_strokes(curves: &[CubicBez], stroke_width: f64) -> Scene {
		let mut scene = Scene::new();
		let stroke = Stroke::new(stroke_width);
		for curve in curves {
			scene.stroke(&stroke, Affine::IDENTITY, Color::WHITE, None, curve);
		}
		scene
	}
}

mod geom_sel {
	use vello::kurbo::{CubicBez, ParamCurve, Point};

	#[derive(Clone, Copy, Debug)]
	pub struct WorldBox {
		pub min_x: f64,
		pub min_y: f64,
		pub max_x: f64,
		pub max_y: f64,
	}

	pub fn inflate_world_box(b: WorldBox, pad: f64) -> WorldBox {
		WorldBox {
			min_x: b.min_x - pad,
			min_y: b.min_y - pad,
			max_x: b.max_x + pad,
			max_y: b.max_y + pad,
		}
	}

	pub fn world_boxes_overlap(a: WorldBox, b: WorldBox) -> bool {
		a.min_x <= b.max_x && a.max_x >= b.min_x && a.min_y <= b.max_y && a.max_y >= b.min_y
	}

	pub fn world_box_contains_point(b: WorldBox, p: Point) -> bool {
		p.x >= b.min_x && p.x <= b.max_x && p.y >= b.min_y && p.y <= b.max_y
	}

	pub fn world_box_contains_box(outer: WorldBox, inner: WorldBox) -> bool {
		inner.min_x >= outer.min_x && inner.max_x <= outer.max_x && inner.min_y >= outer.min_y && inner.max_y <= outer.max_y
	}

	fn world_box_corners(b: WorldBox) -> [Point; 4] {
		[
			Point::new(b.min_x, b.min_y),
			Point::new(b.max_x, b.min_y),
			Point::new(b.max_x, b.max_y),
			Point::new(b.min_x, b.max_y),
		]
	}

	pub fn world_box_from_points(points: &[Point]) -> Option<WorldBox> {
		if points.is_empty() {
			return None;
		}
		let mut min_x = f64::INFINITY;
		let mut min_y = f64::INFINITY;
		let mut max_x = f64::NEG_INFINITY;
		let mut max_y = f64::NEG_INFINITY;
		for p in points {
			min_x = min_x.min(p.x);
			min_y = min_y.min(p.y);
			max_x = max_x.max(p.x);
			max_y = max_y.max(p.y);
		}
		Some(WorldBox {
			min_x,
			min_y,
			max_x,
			max_y,
		})
	}

	pub fn point_in_polygon(point: Point, polygon: &[Point]) -> bool {
		if polygon.len() < 3 {
			return false;
		}
		let mut inside = false;
		let mut j = polygon.len() - 1;
		for i in 0..polygon.len() {
			let a = polygon[i];
			let b = polygon[j];
			let crosses = (a.y > point.y) != (b.y > point.y);
			if crosses && point.x < (b.x - a.x) * (point.y - a.y) / (b.y - a.y) + a.x {
				inside = !inside;
			}
			j = i;
		}
		inside
	}

	fn point_on_segment(point: Point, start: Point, end: Point) -> bool {
		const EPS: f64 = 1e-9;
		point.x >= start.x.min(end.x) - EPS
			&& point.x <= start.x.max(end.x) + EPS
			&& point.y >= start.y.min(end.y) - EPS
			&& point.y <= start.y.max(end.y) + EPS
			&& ((end.x - start.x) * (point.y - start.y) - (end.y - start.y) * (point.x - start.x)).abs() <= EPS
	}

	fn orientation(a: Point, b: Point, c: Point) -> i8 {
		let v = (b.y - a.y) * (c.x - b.x) - (b.x - a.x) * (c.y - b.y);
		if v > 1e-9 {
			1
		} else if v < -1e-9 {
			-1
		} else {
			0
		}
	}

	fn segments_intersect(a0: Point, a1: Point, b0: Point, b1: Point) -> bool {
		let o1 = orientation(a0, a1, b0);
		let o2 = orientation(a0, a1, b1);
		let o3 = orientation(b0, b1, a0);
		let o4 = orientation(b0, b1, a1);
		if o1 != o2 && o3 != o4 {
			return true;
		}
		point_on_segment(b0, a0, a1)
			|| point_on_segment(b1, a0, a1)
			|| point_on_segment(a0, b0, b1)
			|| point_on_segment(a1, b0, b1)
	}

	fn world_box_edges(box_: WorldBox) -> [(Point, Point); 4] {
		let [a, b, c, d] = world_box_corners(box_);
		[(a, b), (b, c), (c, d), (d, a)]
	}

	pub fn segment_intersects_world_box(start: Point, end: Point, box_: WorldBox) -> bool {
		if world_box_contains_point(box_, start) || world_box_contains_point(box_, end) {
			return true;
		}
		world_box_edges(box_)
			.iter()
			.any(|&(a, b)| segments_intersect(start, end, a, b))
	}

	fn polygon_segments(polygon: &[Point]) -> Vec<(Point, Point)> {
		if polygon.is_empty() {
			return Vec::new();
		}
		let mut out = Vec::with_capacity(polygon.len());
		for i in 0..polygon.len() {
			out.push((polygon[i], polygon[(i + 1) % polygon.len()]));
		}
		out
	}

	pub fn polygon_contains_world_box(polygon: &[Point], box_: WorldBox) -> bool {
		world_box_corners(box_)
			.iter()
			.all(|&p| point_in_polygon(p, polygon))
	}

	pub fn polygon_intersects_world_box(polygon: &[Point], box_: WorldBox) -> bool {
		if world_box_corners(box_).iter().any(|&p| point_in_polygon(p, polygon)) {
			return true;
		}
		if polygon.iter().any(|&p| world_box_contains_point(box_, p)) {
			return true;
		}
		polygon_segments(polygon)
			.iter()
			.any(|&(s, e)| segment_intersects_world_box(s, e, box_))
	}

	pub fn segment_intersects_polygon(start: Point, end: Point, polygon: &[Point]) -> bool {
		if point_in_polygon(start, polygon) || point_in_polygon(end, polygon) {
			return true;
		}
		polygon_segments(polygon)
			.iter()
			.any(|&(a, b)| segments_intersect(start, end, a, b))
	}

	#[allow(dead_code)]
	pub fn cubic_bezier_axis_bounds(c: CubicBez) -> WorldBox {
		let xs = [c.p0.x, c.p1.x, c.p2.x, c.p3.x];
		let ys = [c.p0.y, c.p1.y, c.p2.y, c.p3.y];
		WorldBox {
			min_x: xs.iter().copied().fold(f64::INFINITY, f64::min),
			max_x: xs.iter().copied().fold(f64::NEG_INFINITY, f64::max),
			min_y: ys.iter().copied().fold(f64::INFINITY, f64::min),
			max_y: ys.iter().copied().fold(f64::NEG_INFINITY, f64::max),
		}
	}

	pub fn cubic_bezier_point(c: CubicBez, t: f64) -> Point {
		c.eval(t.clamp(0.0, 1.0))
	}
}

mod scene_json {
	use serde::{Deserialize, Serialize};

	#[derive(Clone, Debug, Deserialize, Serialize)]
	pub struct CameraJson {
		pub x: f64,
		pub y: f64,
		pub zoom: f64,
	}

	#[derive(Clone, Debug, Deserialize, Serialize)]
	#[serde(rename_all = "camelCase")]
	pub struct NodeDescJson {
		pub id: String,
		pub x: f64,
		pub y: f64,
		#[serde(default)]
		pub draggable: Option<bool>,
		#[serde(default)]
		pub selected: Option<bool>,
		#[serde(default)]
		pub style: Option<String>,
		#[serde(default)]
		pub text: Option<String>,
		#[serde(default)]
		pub user_data: Option<serde_json::Value>,
		#[serde(default)]
		pub visible: Option<bool>,
		pub shape: Option<String>,
		#[serde(default)]
		pub radius: Option<f64>,
		#[serde(default)]
		pub width: Option<f64>,
		#[serde(default)]
		pub height: Option<f64>,
	}

	#[derive(Clone, Debug, Deserialize, Serialize)]
	#[serde(rename_all = "camelCase")]
	pub struct HandleDescJson {
		pub id: String,
		pub node_id: String,
		pub angle: f64,
		#[serde(default)]
		pub radius: Option<f64>,
		#[serde(default)]
		pub selected: Option<bool>,
		#[serde(default)]
		pub style: Option<String>,
		#[serde(default)]
		pub user_data: Option<serde_json::Value>,
		#[serde(default)]
		pub visible: Option<bool>,
	}

	#[derive(Clone, Debug, Deserialize, Serialize)]
	#[serde(rename_all = "camelCase")]
	pub struct EdgeDescJson {
		pub id: String,
		pub from: String,
		pub to: String,
		#[serde(default)]
		pub selected: Option<bool>,
		#[serde(default)]
		pub style: Option<String>,
		#[serde(default)]
		pub user_data: Option<serde_json::Value>,
		#[serde(default)]
		pub visible: Option<bool>,
	}

	#[derive(Clone, Debug, Default, Deserialize, Serialize)]
	pub struct SceneDescriptorJson {
		pub nodes: Vec<NodeDescJson>,
		pub handles: Vec<HandleDescJson>,
		pub edges: Vec<EdgeDescJson>,
	}

	#[derive(Clone, Debug, Deserialize, Serialize)]
	pub struct FixtureV1Json {
		pub schema: String,
		pub camera: CameraJson,
		pub nodes: Vec<serde_json::Value>,
		pub edges: Vec<serde_json::Value>,
		#[serde(default)]
		pub meta: Option<serde_json::Value>,
	}
}

pub use scene_json::{CameraJson, EdgeDescJson, FixtureV1Json, HandleDescJson, NodeDescJson, SceneDescriptorJson};

mod board_host {
	use super::scene_json::*;
	use serde_json::json;
	use std::collections::{BTreeMap, BTreeSet};
	use vello::kurbo::{Affine, Circle, CubicBez, Point, Rect, Stroke, Vec2};
	use vello::peniko::{Color, Fill};
	use vello::Scene;

	use super::geom_sel::{
		cubic_bezier_point, inflate_world_box, point_in_polygon, polygon_contains_world_box, polygon_intersects_world_box,
		segment_intersects_polygon, segment_intersects_world_box, world_box_contains_box, world_box_contains_point,
		world_box_from_points, world_boxes_overlap, WorldBox,
	};
	use super::vcompute::{
		compute_edge_bezier_points, distance_between, distance_point_to_cubic_bezier, handle_position_on_circle,
		handle_position_on_rectangle,
	};

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
		/// Screen-space preview polygon (CSS pixels) while area-selecting; cleared when idle.
		pub selection_screen_preview: Option<Vec<Point>>,
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
				selection_screen_preview: None,
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

		pub fn set_selection_screen_preview(&mut self, points: Option<Vec<Point>>) {
			self.selection_screen_preview = points;
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
			let mut inner = Scene::new();
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
				inner.stroke(
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
						inner.fill(Fill::NonZero, Affine::IDENTITY, fill, None, &circle);
						inner.stroke(&Stroke::new(sw), Affine::IDENTITY, stroke_c, None, &circle);
					}
					NodeShape::Rectangle => {
						let hw = n.width / 2.0;
						let hh = n.height / 2.0;
						let p0 = self.world_to_screen(Point::new(n.x - hw, n.y - hh));
						let p1 = self.world_to_screen(Point::new(n.x + hw, n.y + hh));
						let r = Rect::from_points(p0, p1);
						inner.fill(Fill::NonZero, Affine::IDENTITY, fill, None, &r);
						inner.stroke(&Stroke::new(sw), Affine::IDENTITY, stroke_c, None, &r);
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
				inner.fill(Fill::NonZero, Affine::IDENTITY, fill, None, &circle);
				inner.stroke(&Stroke::new(2.0), Affine::IDENTITY, stroke_c, None, &circle);
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
				inner.stroke(
					&Stroke::new(edge_sw),
					Affine::IDENTITY,
					Color::new([0.28, 0.33, 0.41, 1.0]),
					None,
					c,
				);
			}
			if let Some(ref pts) = self.selection_screen_preview {
				if pts.len() >= 2 {
					let mut path = vello::kurbo::BezPath::new();
					path.move_to(pts[0]);
					for p in pts.iter().skip(1) {
						path.line_to(*p);
					}
					path.close_path();
					inner.fill(
						Fill::NonZero,
						Affine::IDENTITY,
						Color::new([0.078, 0.722, 0.651, 0.12]),
						None,
						&path,
					);
					inner.stroke(
						&Stroke::new(1.5),
						Affine::IDENTITY,
						Color::new([0.059, 0.463, 0.431, 0.85]),
						None,
						&path,
					);
				}
			}
			let scale = self.dpr.max(1.0);
			if (scale - 1.0).abs() < f64::EPSILON {
				inner
			} else {
				let mut scene = Scene::new();
				scene.append(&inner, Some(Affine::scale(scale)));
				scene
			}
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
}

pub use board_host::BoardHost;

use std::collections::{BTreeMap, BTreeSet};

pub use vello::kurbo::{CubicBez, Point, Vec2};
use vcompute::{compute_edge_bezier_points, distance_point_to_cubic_bezier, encode_board_vello_strokes};

// #region 🔖Kinds
/// 🧭 Camera state in world units with a zoom scalar suitable for a WASM host bridge.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera {
	pub x: f64,
	pub y: f64,
	pub zoom: f64,
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

/// 🪢 Cubic edge connecting two handles.
#[derive(Clone, Debug, PartialEq)]
pub struct Edge {
	pub from_handle: HandleId,
	pub id: EdgeId,
	pub to_handle: HandleId,
}

/// 🎯 Semantic board event emitted after interaction or selection changes.
#[derive(Clone, Debug, PartialEq)]
pub enum BoardEvent {
	HoverChanged { id: Option<u64> },
	NodeMoved { id: NodeId, x: f64, y: f64 },
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
	pub edges: Vec<CubicBez>,
	pub handles: Vec<(HandleId, Point, f64)>,
	pub nodes: Vec<(NodeId, Point, f64)>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum HitObject {
	Edge(EdgeId),
	Handle(HandleId),
	Node(NodeId),
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum InteractionMode {
	DragNode { node_id: NodeId, offset: Vec2 },
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

fn handle_position(node: &Node, handle: &Handle) -> Point {
	vcompute::handle_position_on_circle(node.center, node.radius, handle.angle)
}

fn distance(left: Point, right: Point) -> f64 {
	vcompute::distance_between(left, right)
}
// #endregion 🔖Utilities

// #region 🔖Engine
/// ⚙️ Single-file retained board engine; geometry uses Vello’s kurbo curves and scene encoding.
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

	pub fn create_node(&mut self, id: NodeId, x: f64, y: f64, radius: f64, draggable: bool) {
		self.nodes.insert(
			id,
			Node {
				center: Point::new(x, y),
				draggable,
				id,
				radius,
			},
		);
	}

	pub fn update_node(&mut self, id: NodeId, x: f64, y: f64, radius: f64) {
		if let Some(node) = self.nodes.get_mut(&id) {
			node.center = Point::new(x, y);
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

	pub fn create_handle(&mut self, id: HandleId, node_id: NodeId, angle: f64) {
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

	pub fn pointer_down(&mut self, x: f64, y: f64) {
		let point = Point::new(x, y);
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

	pub fn pointer_move(&mut self, x: f64, y: f64) {
		let point = Point::new(x, y);
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
		let _vello_scene = encode_board_vello_strokes(&snapshot.edges, 2.0);
		let _ = _vello_scene.encoding().path_tags.len();
		snapshot
	}

	pub fn drain_events(&mut self) -> Vec<BoardEvent> {
		std::mem::take(&mut self.events)
	}

	pub fn edge_curve(&self, edge_id: EdgeId) -> Option<CubicBez> {
		let edge = self.edges.get(&edge_id)?;
		let from_handle = self.handles.get(&edge.from_handle)?;
		let to_handle = self.handles.get(&edge.to_handle)?;
		let from_node = self.nodes.get(&from_handle.node_id)?;
		let to_node = self.nodes.get(&to_handle.node_id)?;
		let from_position = handle_position(from_node, from_handle);
		let to_position = handle_position(to_node, to_handle);
		Some(compute_edge_bezier_points(
			from_position,
			to_position,
			from_node.center,
			to_node.center,
		))
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

	fn hit_test(&self, point: Point) -> Option<HitObject> {
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
				if distance_point_to_cubic_bezier(point, curve, 18) <= 8.0 {
					return Some(HitObject::Edge(edge.id));
				}
			}
		}
		None
	}
}
// #endregion 🔖Engine

// #region 🔖WasmHost
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use web_sys::HtmlCanvasElement;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = boardComputeEdgeBezier)]
pub fn board_compute_edge_bezier(
	from_px: f64,
	from_py: f64,
	from_cx: f64,
	from_cy: f64,
	to_px: f64,
	to_py: f64,
	to_cx: f64,
	to_cy: f64,
) -> Vec<f64> {
	let c = compute_edge_bezier_points(
		Point::new(from_px, from_py),
		Point::new(to_px, to_py),
		Point::new(from_cx, from_cy),
		Point::new(to_cx, to_cy),
	);
	vec![c.p0.x, c.p0.y, c.p1.x, c.p1.y, c.p2.x, c.p2.y, c.p3.x, c.p3.y]
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = boardDistancePointCubic)]
pub fn board_distance_point_cubic(
	px: f64,
	py: f64,
	p0x: f64,
	p0y: f64,
	p1x: f64,
	p1y: f64,
	p2x: f64,
	p2y: f64,
	p3x: f64,
	p3y: f64,
	steps: u32,
) -> f64 {
	let curve = CubicBez::new(
		Point::new(p0x, p0y),
		Point::new(p1x, p1y),
		Point::new(p2x, p2y),
		Point::new(p3x, p3y),
	);
	distance_point_to_cubic_bezier(Point::new(px, py), curve, steps.max(1) as usize)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = boardRayRectEdge)]
pub fn board_ray_rect_edge(hw: f64, hh: f64, ux: f64, uy: f64) -> Vec<f64> {
	let p = vcompute::ray_from_origin_to_axis_aligned_rectangle_edge(hw, hh, ux, uy);
	vec![p.x, p.y]
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = boardHandlePositionCircle)]
pub fn board_handle_position_circle(cx: f64, cy: f64, radius: f64, angle: f64) -> Vec<f64> {
	let p = vcompute::handle_position_on_circle(Point::new(cx, cy), radius, angle);
	vec![p.x, p.y]
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = boardHandlePositionRectangle)]
pub fn board_handle_position_rectangle(cx: f64, cy: f64, width: f64, height: f64, angle: f64) -> Vec<f64> {
	let p = vcompute::handle_position_on_rectangle(Point::new(cx, cy), width, height, angle);
	vec![p.x, p.y]
}

// #region 🔖WasmVelloPresenter
/// 🖥️ `BoardHost` plus WebGPU: encodes the board scene and rasterizes it with Vello, then blits to the canvas surface.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct BoardVelloWasm {
	#[cfg_attr(target_arch = "wasm32", allow(dead_code, reason = "Keeps the canvas alive for the WebGPU surface."))]
	canvas: HtmlCanvasElement,
	host: BoardHost,
	render_ctx: vello::util::RenderContext,
	renderer: Option<vello::Renderer>,
	surface: Option<vello::util::RenderSurface<'static>>,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl BoardVelloWasm {
	/// @emoji 🌊 Creates WebGPU + Vello for `canvas`; `logical_w`/`logical_h` are CSS pixels, `dpr` scales the swapchain to backing-store pixels.
	#[wasm_bindgen(js_name = create)]
	pub async fn create(
		canvas: HtmlCanvasElement,
		logical_w: u32,
		logical_h: u32,
		dpr: f64,
	) -> Result<BoardVelloWasm, JsValue> {
		let lw = logical_w.max(1);
		let lh = logical_h.max(1);
		let dpr = dpr.max(1.0);
		let pw = ((lw as f64 * dpr).round() as u32).max(1);
		let ph = ((lh as f64 * dpr).round() as u32).max(1);
		let mut render_ctx = vello::util::RenderContext::new();
		let surface = render_ctx
			.create_surface(
				vello::wgpu::SurfaceTarget::Canvas(canvas.clone()),
				pw,
				ph,
				vello::wgpu::PresentMode::AutoVsync,
			)
			.await
			.map_err(|err| JsValue::from_str(&format!("{err:?}")))?;
		let dev = &render_ctx.devices[surface.dev_id].device;
		let renderer = vello::Renderer::new(
			dev,
			vello::RendererOptions {
				use_cpu: false,
				antialiasing_support: vello::AaSupport::area_only(),
				num_init_threads: std::num::NonZeroUsize::new(1),
				pipeline_cache: None,
			},
		)
		.map_err(|err| JsValue::from_str(&format!("{err:?}")))?;
		let mut host = BoardHost::new();
		host.set_size(lw, lh, dpr);
		Ok(Self {
			canvas,
			host,
			render_ctx,
			renderer: Some(renderer),
			surface: Some(surface),
		})
	}

	pub fn set_size(&mut self, width: u32, height: u32, dpr: f64) {
		let lw = width.max(1);
		let lh = height.max(1);
		let dpr = dpr.max(1.0);
		let pw = ((lw as f64 * dpr).round() as u32).max(1);
		let ph = ((lh as f64 * dpr).round() as u32).max(1);
		self.host.set_size(lw, lh, dpr);
		if let Some(ref mut surface) = self.surface {
			self.render_ctx.resize_surface(surface, pw, ph);
		}
	}

	#[wasm_bindgen(js_name = setSelectionScreenPreview)]
	pub fn set_selection_screen_preview(&mut self, flat_xy: &[f64]) {
		if flat_xy.len() < 4 || flat_xy.len() % 2 != 0 {
			self.host.set_selection_screen_preview(None);
			return;
		}
		let mut pts = Vec::with_capacity(flat_xy.len() / 2);
		for chunk in flat_xy.chunks_exact(2) {
			pts.push(Point::new(chunk[0], chunk[1]));
		}
		self.host.set_selection_screen_preview(Some(pts));
	}

	#[wasm_bindgen(js_name = clearSelectionScreenPreview)]
	pub fn clear_selection_screen_preview(&mut self) {
		self.host.set_selection_screen_preview(None);
	}

	#[wasm_bindgen(js_name = syncDescriptorJson)]
	pub fn sync_descriptor_json(&mut self, json: &str) -> Result<(), JsValue> {
		let desc: SceneDescriptorJson =
			serde_json::from_str(json).map_err(|e| JsValue::from_str(&e.to_string()))?;
		self.host.sync_descriptor(&desc);
		Ok(())
	}

	#[wasm_bindgen(js_name = parseFixtureJson)]
	pub fn parse_fixture_json(&mut self, json: &str) -> bool {
		let raw: serde_json::Value = match serde_json::from_str(json) {
			Ok(v) => v,
			Err(_) => return false,
		};
		self.host.parse_fixture_v1(&raw)
	}

	#[wasm_bindgen(js_name = setCamera)]
	pub fn set_camera_wasm(&mut self, x: f64, y: f64, zoom: f64) {
		self.host.set_camera(x, y, zoom);
	}

	#[wasm_bindgen(js_name = pointerDownScreen)]
	pub fn pointer_down_screen_wasm(&mut self, sx: f64, sy: f64, button: u8, shift: bool) {
		self.host.pointer_down_screen(sx, sy, button, shift);
	}

	#[wasm_bindgen(js_name = pointerMoveScreen)]
	pub fn pointer_move_screen_wasm(&mut self, sx: f64, sy: f64) {
		self.host.pointer_move_screen(sx, sy);
	}

	#[wasm_bindgen(js_name = pointerUpScreen)]
	pub fn pointer_up_screen_wasm(&mut self, sx: f64, sy: f64) {
		self.host.pointer_up_screen(sx, sy);
	}

	#[wasm_bindgen(js_name = wheelScreen)]
	pub fn wheel_screen_wasm(&mut self, sx: f64, sy: f64, delta_y: f64) {
		self.host.wheel_screen(sx, sy, delta_y);
	}

	#[wasm_bindgen(js_name = deleteSelection)]
	pub fn delete_selection_wasm(&mut self) {
		self.host.delete_selection();
	}

	#[wasm_bindgen(js_name = drainEventsJson)]
	pub fn drain_events_json_wasm(&mut self) -> String {
		self.host.drain_events_json()
	}

	#[wasm_bindgen(js_name = cameraJson)]
	pub fn camera_json(&self) -> String {
		serde_json::json!({
			"x": self.host.camera.x,
			"y": self.host.camera.y,
			"zoom": self.host.camera.zoom,
		})
		.to_string()
	}

	#[wasm_bindgen(js_name = setSelectionOptions)]
	pub fn set_selection_options_wasm(&mut self, method: &str, mode: &str, target: &str) {
		self.host.set_selection_options(method, mode, target);
	}

	#[wasm_bindgen(js_name = encodedSceneHint)]
	pub fn encoded_scene_hint_wasm(&self) -> usize {
		self.host.encoded_scene_hint()
	}

	/// @emoji 🎨 Renders one frame: Vello to the intermediate texture, blit to the swapchain, present.
	pub fn render_frame(&mut self) -> Result<(), JsValue> {
		let surface = self
			.surface
			.as_mut()
			.ok_or_else(|| JsValue::from_str("missing surface"))?;
		let renderer = self
			.renderer
			.as_mut()
			.ok_or_else(|| JsValue::from_str("missing renderer"))?;
		let dh = &self.render_ctx.devices[surface.dev_id];
		let pw = surface.config.width.max(1);
		let ph = surface.config.height.max(1);
		let scene = self.host.build_vello_scene();
		let params = vello::RenderParams {
			base_color: vello::peniko::Color::WHITE,
			width: pw,
			height: ph,
			antialiasing_method: vello::AaConfig::Area,
		};
		renderer
			.render_to_texture(&dh.device, &dh.queue, &scene, &surface.target_view, &params)
			.map_err(|err| JsValue::from_str(&format!("{err:?}")))?;

		let surface_tex = match surface.surface.get_current_texture() {
			vello::wgpu::CurrentSurfaceTexture::Success(t) | vello::wgpu::CurrentSurfaceTexture::Suboptimal(t) => t,
			vello::wgpu::CurrentSurfaceTexture::Outdated => {
				self.render_ctx.configure_surface(surface);
				return Ok(());
			}
			vello::wgpu::CurrentSurfaceTexture::Timeout | vello::wgpu::CurrentSurfaceTexture::Occluded => return Ok(()),
			vello::wgpu::CurrentSurfaceTexture::Lost | vello::wgpu::CurrentSurfaceTexture::Validation => {
				return Err(JsValue::from_str("surface lost or validation error"));
			}
		};
		let view = surface_tex
			.texture
			.create_view(&vello::wgpu::TextureViewDescriptor::default());
		let mut encoder = dh.device.create_command_encoder(&vello::wgpu::CommandEncoderDescriptor {
			label: Some("elements_board_vello_blit"),
		});
		surface
			.blitter
			.copy(&dh.device, &mut encoder, &surface.target_view, &view);
		dh.queue.submit(std::iter::once(encoder.finish()));
		surface_tex.present();
		let _ = dh.device.poll(vello::wgpu::PollType::Poll);
		Ok(())
	}
}
// #endregion 🔖WasmVelloPresenter

/// 🎛️ Thin WASM façade over {@link BoardHost}: JSON descriptor sync, pointer routing, and Vello scene metrics.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct BoardWasmHost {
	inner: BoardHost,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl BoardWasmHost {
	#[wasm_bindgen(constructor)]
	pub fn new() -> Self {
		Self {
			inner: BoardHost::new(),
		}
	}

	pub fn set_size(&mut self, width: u32, height: u32, dpr: f64) {
		self.inner.set_size(width, height, dpr);
	}

	#[wasm_bindgen(js_name = setCamera)]
	pub fn set_camera_wasm(&mut self, x: f64, y: f64, zoom: f64) {
		self.inner.set_camera(x, y, zoom);
	}

	#[wasm_bindgen(js_name = syncDescriptorJson)]
	pub fn sync_descriptor_json(&mut self, json: &str) -> Result<(), JsValue> {
		let desc: SceneDescriptorJson =
			serde_json::from_str(json).map_err(|err| JsValue::from_str(&err.to_string()))?;
		self.inner.sync_descriptor(&desc);
		Ok(())
	}

	#[wasm_bindgen(js_name = parseFixtureJson)]
	pub fn parse_fixture_json(&mut self, json: &str) -> bool {
		let raw: serde_json::Value = match serde_json::from_str(json) {
			Ok(v) => v,
			Err(_) => return false,
		};
		self.inner.parse_fixture_v1(&raw)
	}

	#[wasm_bindgen(js_name = pointerDownScreen)]
	pub fn pointer_down_screen_wasm(&mut self, sx: f64, sy: f64, button: u8, shift: bool) {
		self.inner.pointer_down_screen(sx, sy, button, shift);
	}

	#[wasm_bindgen(js_name = pointerMoveScreen)]
	pub fn pointer_move_screen_wasm(&mut self, sx: f64, sy: f64) {
		self.inner.pointer_move_screen(sx, sy);
	}

	#[wasm_bindgen(js_name = pointerUpScreen)]
	pub fn pointer_up_screen_wasm(&mut self, sx: f64, sy: f64) {
		self.inner.pointer_up_screen(sx, sy);
	}

	#[wasm_bindgen(js_name = wheelScreen)]
	pub fn wheel_screen_wasm(&mut self, sx: f64, sy: f64, delta_y: f64) {
		self.inner.wheel_screen(sx, sy, delta_y);
	}

	#[wasm_bindgen(js_name = deleteSelection)]
	pub fn delete_selection_wasm(&mut self) {
		self.inner.delete_selection();
	}

	#[wasm_bindgen(js_name = drainEventsJson)]
	pub fn drain_events_json_wasm(&mut self) -> String {
		self.inner.drain_events_json()
	}

	#[wasm_bindgen(js_name = cameraJson)]
	pub fn camera_json(&self) -> String {
		serde_json::json!({
			"x": self.inner.camera.x,
			"y": self.inner.camera.y,
			"zoom": self.inner.camera.zoom,
		})
		.to_string()
	}

	#[wasm_bindgen(js_name = setSelectionOptions)]
	pub fn set_selection_options_wasm(&mut self, method: &str, mode: &str, target: &str) {
		self.inner.set_selection_options(method, mode, target);
	}

	#[wasm_bindgen(js_name = encodedSceneHint)]
	pub fn encoded_scene_hint_wasm(&self) -> usize {
		self.inner.encoded_scene_hint()
	}
}
// #endregion 🔖WasmHost

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
		engine.create_handle(20, 2, std::f64::consts::PI);
		engine.create_edge(100, 10, 20);

		let curve = engine.edge_curve(100).expect("edge curve should exist");
		assert!((curve.p0.x - 40.0).abs() < 0.001);
		assert!(curve.p0.y.abs() < 0.001);
		assert!((curve.p3.x - 260.0).abs() < 0.001);
		assert!(curve.p3.y.abs() < 0.001);
		let outward = curve.p0 - Point::ORIGIN;
		let arm0 = curve.p1 - curve.p0;
		let align0 = vcompute::normalize_or_zero(outward).dot(vcompute::normalize_or_zero(arm0));
		let inward = Point::new(300.0, 0.0) - curve.p3;
		let arm1 = curve.p3 - curve.p2;
		let align1 = vcompute::normalize_or_zero(inward).dot(vcompute::normalize_or_zero(arm1)).abs();
		assert!(align0 > 0.99);
		assert!(align1 > 0.99);
	}

	#[test]
	fn drags_nodes_without_rebuilding_the_scene_catalog() {
		let mut engine = BoardEngine::new();
		engine.create_node(1, 0.0, 0.0, 30.0, true);

		engine.pointer_down(0.0, 0.0);
		engine.pointer_move(60.0, 25.0);
		engine.pointer_up();

		let node = engine.nodes.get(&1).expect("node should remain in the engine");
		assert_eq!(node.center, Point::new(60.0, 25.0));

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
		engine.create_handle(20, 2, std::f64::consts::PI);
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
		engine.create_handle(20, 2, std::f64::consts::PI);
		engine.create_edge(100, 10, 20);

		let snapshot = engine.render_snapshot();
		assert_eq!(snapshot.nodes.len(), 2);
		assert_eq!(snapshot.handles.len(), 2);
		assert_eq!(snapshot.edges.len(), 1);
	}
}

#[cfg(test)]
mod host_tests {
	use super::vcompute::handle_position_on_circle;
	use super::{BoardHost, EdgeDescJson, HandleDescJson, NodeDescJson, SceneDescriptorJson};
	use vello::kurbo::Point;

	fn sample_scene() -> SceneDescriptorJson {
		SceneDescriptorJson {
			nodes: vec![NodeDescJson {
				id: "a".into(),
				x: 0.0,
				y: 0.0,
				draggable: Some(true),
				selected: None,
				style: None,
				text: None,
				user_data: None,
				visible: None,
				shape: Some("circle".into()),
				radius: Some(40.0),
				width: None,
				height: None,
			}],
			handles: vec![
				HandleDescJson {
					id: "a.out".into(),
					node_id: "a".into(),
					angle: 0.0,
					radius: None,
					selected: None,
					style: None,
					user_data: None,
					visible: None,
				},
				HandleDescJson {
					id: "b.in".into(),
					node_id: "b".into(),
					angle: std::f64::consts::PI,
					radius: None,
					selected: None,
					style: None,
					user_data: None,
					visible: None,
				},
			],
			edges: vec![EdgeDescJson {
				id: "e1".into(),
				from: "a.out".into(),
				to: "b.in".into(),
				selected: None,
				style: None,
				user_data: None,
				visible: None,
			}],
		}
	}

	#[test]
	fn board_host_syncs_descriptor_and_hit_tests_handle_before_node() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		let mut desc = sample_scene();
		desc.nodes.push(NodeDescJson {
			id: "b".into(),
			x: 300.0,
			y: 0.0,
			draggable: Some(true),
			selected: None,
			style: None,
			text: None,
			user_data: None,
			visible: None,
			shape: Some("circle".into()),
			radius: Some(40.0),
			width: None,
			height: None,
		});
		h.sync_descriptor(&desc);
		let hp = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
		let hit = h.resolve_hit_world(hp);
		assert_eq!(hit.as_deref(), Some("a.out"));
		assert!(h.encoded_scene_hint() > 10);
	}

	#[test]
	fn board_host_drag_emits_node_move() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		let mut desc = sample_scene();
		desc.nodes.push(NodeDescJson {
			id: "b".into(),
			x: 300.0,
			y: 0.0,
			draggable: Some(true),
			selected: None,
			style: None,
			text: None,
			user_data: None,
			visible: None,
			shape: Some("circle".into()),
			radius: Some(40.0),
			width: None,
			height: None,
		});
		h.sync_descriptor(&desc);
		let _ = h.drain_events_json();
		let w = Point::new(0.0, 0.0);
		let s = h.world_to_screen(w);
		h.pointer_down_screen(s.x, s.y, 0, false);
		h.pointer_move_screen(s.x + 50.0, s.y + 30.0);
		h.pointer_up_screen(s.x + 50.0, s.y + 30.0);
		let ev = h.drain_events_json();
		assert!(ev.contains("nodeMove"));
	}
}

// #endregion 🔖Tests
