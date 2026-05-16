//! 🎛️ Single-source board crate: vector geometry (`vcompute`), selection predicates (`geom_sel`), serde scene JSON (`scene_json`), interactive `BoardHost`, retained `BoardEngine`, and wasm-bindgen facades — all in this file (no sibling `src/` modules).
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

	/// 🧭 Rectangle handle `angle` is **0 at top edge center (north)**, increasing **counter‑clockwise** in board space (`y` down): `π/4` NW corner, `π/2` west midpoint, `π` south, `3π/2` east; circles keep **east‑zero** `atan2(dy,dx)` convention.
	#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
	pub fn handle_position_on_rectangle(center: Point, width: f64, height: f64, angle: f64) -> Point {
		let hw = width / 2.0;
		let hh = height / 2.0;
		let ux = -angle.sin();
		let uy = -angle.cos();
		let local = ray_from_origin_to_axis_aligned_rectangle_edge(hw, hh, ux, uy);
		center + Vec2::new(local.x, local.y)
	}

	pub fn compute_edge_bezier_points(from_point: Point, to_point: Point, from_center: Point, to_center: Point) -> CubicBez {
		let mut from_out = normalize_or_zero(from_point - from_center);
		if from_out == Vec2::new(0.0, 0.0) {
			from_out = normalize_or_zero(to_point - from_point);
		}
		let mut to_out = normalize_or_zero(to_point - to_center);
		if to_out == Vec2::new(0.0, 0.0) {
			to_out = normalize_or_zero(to_point - from_point);
		}
		let handle_distance = distance_between(from_point, to_point);
		let control_length = clamp_f64(handle_distance * 0.35, 24.0, 240.0);
		let p1 = from_point + from_out * control_length;
		let p2 = to_point + to_out * control_length;
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

	pub fn encode_board_stroke_scene(curves: &[CubicBez], stroke_width: f64) -> Scene {
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

mod elements_board_palette {
	use vello::peniko::Color;
	include!(concat!(env!("OUT_DIR"), "/elements_styling_board.rs"));
}

mod board_host {
	use super::elements_board_palette as board_palette;
	use super::scene_json::*;
	use serde_json::json;
	use std::collections::{BTreeMap, BTreeSet};
	use vello::kurbo::{Affine, Circle, CubicBez, Point, Rect, Stroke, Vec2};
	use vello::peniko::{Color, Fill};
	use vello::Scene;

	use super::geom_sel::{
		cubic_bezier_axis_bounds, cubic_bezier_point, inflate_world_box, point_in_polygon, polygon_contains_world_box,
		polygon_intersects_world_box, segment_intersects_polygon, segment_intersects_world_box, world_box_contains_box,
		world_box_contains_point, world_box_from_points, world_boxes_overlap, WorldBox,
	};
	use super::vcompute::{
		compute_edge_bezier_points, distance_between, distance_point_to_cubic_bezier, handle_position_on_circle,
		handle_position_on_rectangle,
	};

	const GRID_WORLD_STEP: f64 = 96.0;
	const WORLD_CLIP_TILE_WORLD: f64 = 256.0;
	const MAX_WORLD_CLIP_TILES: u32 = 768;
	const EDGE_HIT_TOLERANCE_PX: f64 = 8.0;
	const HANDLE_HIT_TOLERANCE_PX: f64 = 10.0;
	const HANDLE_DRAW_MIN_ZOOM: f64 = 0.45;
	const SELECTION_LASSO_MIN_POINT_DISTANCE_PX: f64 = 3.0;
	const SELECTION_CLICK_MAX_DISTANCE_PX: f64 = 4.0;
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
		DragNodes {
			offset: Vec2,
			primary_id: String,
			start_positions: BTreeMap<String, (f64, f64)>,
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

	#[derive(Clone, Copy, Debug)]
	pub struct VelloThemePalette {
		pub raster_clear: Color,
		pub grid_minor_stroke: Color,
		pub edge_stroke: Color,
		pub edge_stroke_selected: Color,
		pub node_fill: Color,
		pub node_stroke: Color,
		pub node_fill_selected: Color,
		pub node_stroke_selected: Color,
		pub handle_fill: Color,
		pub handle_stroke: Color,
		pub handle_fill_selected: Color,
		pub handle_stroke_selected: Color,
		pub selection_preview_fill: Color,
		pub selection_preview_stroke: Color,
	}

	impl Default for VelloThemePalette {
		fn default() -> Self {
			Self {
				raster_clear: board_palette::RASTER_CLEAR,
				grid_minor_stroke: board_palette::GRID_MINOR_STROKE,
				edge_stroke: board_palette::EDGE_STROKE,
				edge_stroke_selected: board_palette::EDGE_STROKE_SELECTED,
				node_fill: board_palette::NODE_FILL,
				node_stroke: board_palette::NODE_STROKE,
				node_fill_selected: board_palette::NODE_FILL_SELECTED,
				node_stroke_selected: board_palette::NODE_STROKE_SELECTED,
				handle_fill: board_palette::HANDLE_FILL,
				handle_stroke: board_palette::HANDLE_STROKE,
				handle_fill_selected: board_palette::HANDLE_FILL_SELECTED,
				handle_stroke_selected: board_palette::HANDLE_STROKE_SELECTED,
				selection_preview_fill: board_palette::SELECTION_PREVIEW_FILL,
				selection_preview_stroke: board_palette::SELECTION_PREVIEW_STROKE,
			}
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
		pub vello_theme: VelloThemePalette,
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
				world_raster_tiling: "world-clip".into(),
				events: Vec::new(),
				selection_screen_preview: None,
				vello_theme: VelloThemePalette::default(),
			}
		}
	}

	impl BoardHost {
		fn color_from_json_rgba8(arr: &[serde_json::Value]) -> Option<Color> {
			let r = u8::try_from(arr.get(0)?.as_u64().unwrap_or(0).min(255)).ok()?;
			let g = u8::try_from(arr.get(1)?.as_u64().unwrap_or(0).min(255)).ok()?;
			let b = u8::try_from(arr.get(2)?.as_u64().unwrap_or(0).min(255)).ok()?;
			let a = u8::try_from(arr.get(3).and_then(|x| x.as_u64()).unwrap_or(255).min(255)).ok()?;
			Some(Color::from_rgba8(r, g, b, a))
		}

		pub fn new() -> Self {
			Self::default()
		}

		pub fn set_size(&mut self, width: u32, height: u32, dpr: f64) {
			self.width = width.max(1);
			self.height = height.max(1);
			self.dpr = dpr.max(1.0);
		}

		pub fn set_camera(&mut self, x: f64, y: f64, zoom: f64) {
			let zoom = zoom.clamp(BOARD_CAMERA_ZOOM_MIN, BOARD_CAMERA_ZOOM_MAX);
			if (self.camera.x - x).abs() < 1e-9
				&& (self.camera.y - y).abs() < 1e-9
				&& (self.camera.zoom - zoom).abs() < 1e-9
			{
				return;
			}
			self.camera.x = x;
			self.camera.y = y;
			self.camera.zoom = zoom;
			self.push_event("camera", json!({ "x": self.camera.x, "y": self.camera.y, "zoom": self.camera.zoom }));
		}

		pub fn set_selection_options(&mut self, method: &str, mode: &str, target: &str) {
			self.selection_options.method = method.into();
			self.selection_options.mode = mode.into();
			self.selection_options.target = target.into();
		}

		/// @emoji 🧩 Selects world-space clip tiling for Vello scene construction (`none` | `world-clip`).
		pub fn set_world_raster_tiling(&mut self, mode: &str) {
			let next = if mode == "world-clip" { "world-clip".into() } else { "none".into() };
			if self.world_raster_tiling == next {
				return;
			}
			self.world_raster_tiling = next;
		}

		pub fn set_selection_screen_preview(&mut self, points: Option<Vec<Point>>) {
			self.selection_screen_preview = points;
		}

		pub fn set_vello_theme_from_json(&mut self, json: &str) -> Result<(), String> {
			let v: serde_json::Value = serde_json::from_str(json).map_err(|e| e.to_string())?;
			let mut next = self.vello_theme;
			if let Some(arr) = v.get("rasterClear").and_then(|x| x.as_array()) {
				if let Some(c) = Self::color_from_json_rgba8(arr) {
					next.raster_clear = c;
				}
			}
			if let Some(arr) = v.get("gridMinorStroke").and_then(|x| x.as_array()) {
				if let Some(c) = Self::color_from_json_rgba8(arr) {
					next.grid_minor_stroke = c;
				}
			}
			if let Some(arr) = v.get("edgeStroke").and_then(|x| x.as_array()) {
				if let Some(c) = Self::color_from_json_rgba8(arr) {
					next.edge_stroke = c;
				}
			}
			if let Some(arr) = v.get("edgeStrokeSelected").and_then(|x| x.as_array()) {
				if let Some(c) = Self::color_from_json_rgba8(arr) {
					next.edge_stroke_selected = c;
				}
			}
			if let Some(arr) = v.get("nodeFill").and_then(|x| x.as_array()) {
				if let Some(c) = Self::color_from_json_rgba8(arr) {
					next.node_fill = c;
				}
			}
			if let Some(arr) = v.get("nodeStroke").and_then(|x| x.as_array()) {
				if let Some(c) = Self::color_from_json_rgba8(arr) {
					next.node_stroke = c;
				}
			}
			if let Some(arr) = v.get("nodeFillSelected").and_then(|x| x.as_array()) {
				if let Some(c) = Self::color_from_json_rgba8(arr) {
					next.node_fill_selected = c;
				}
			}
			if let Some(arr) = v.get("nodeStrokeSelected").and_then(|x| x.as_array()) {
				if let Some(c) = Self::color_from_json_rgba8(arr) {
					next.node_stroke_selected = c;
				}
			}
			if let Some(arr) = v.get("handleFill").and_then(|x| x.as_array()) {
				if let Some(c) = Self::color_from_json_rgba8(arr) {
					next.handle_fill = c;
				}
			}
			if let Some(arr) = v.get("handleStroke").and_then(|x| x.as_array()) {
				if let Some(c) = Self::color_from_json_rgba8(arr) {
					next.handle_stroke = c;
				}
			}
			if let Some(arr) = v.get("handleFillSelected").and_then(|x| x.as_array()) {
				if let Some(c) = Self::color_from_json_rgba8(arr) {
					next.handle_fill_selected = c;
				}
			}
			if let Some(arr) = v.get("handleStrokeSelected").and_then(|x| x.as_array()) {
				if let Some(c) = Self::color_from_json_rgba8(arr) {
					next.handle_stroke_selected = c;
				}
			}
			if let Some(arr) = v.get("selectionPreviewFill").and_then(|x| x.as_array()) {
				if let Some(c) = Self::color_from_json_rgba8(arr) {
					next.selection_preview_fill = c;
				}
			}
			if let Some(arr) = v.get("selectionPreviewStroke").and_then(|x| x.as_array()) {
				if let Some(c) = Self::color_from_json_rgba8(arr) {
					next.selection_preview_stroke = c;
				}
			}
			self.vello_theme = next;
			Ok(())
		}

		fn sync_selection_screen_overlay(&mut self, start_screen: Point, screen_points: &[Point]) {
			if screen_points.len() < 2 {
				self.selection_screen_preview = None;
				return;
			}
			self.selection_screen_preview = Some(if self.selection_options.method == "lasso" {
				screen_points.to_vec()
			} else {
				let last = *screen_points.last().unwrap_or(&start_screen);
				vec![
					start_screen,
					Point::new(last.x, start_screen.y),
					last,
					Point::new(start_screen.x, last.y),
				]
			});
		}

		fn push_event(&mut self, name: &str, payload: serde_json::Value) {
			self.events.push(json!({ "name": name, "payload": payload }));
		}

		pub fn drain_events_json(&mut self) -> String {
			let out = serde_json::to_string(&self.events).unwrap_or_else(|_| "[]".into());
			self.events.clear();
			out
		}

		fn sync_selection_flags_to_objects(&mut self) {
			for n in self.nodes.values_mut() {
				n.selected = self.selection.contains(&n.id);
			}
			for h in self.handles.values_mut() {
				h.selected = self.selection.contains(&h.id);
			}
			for e in self.edges.values_mut() {
				e.selected = self.selection.contains(&e.id);
			}
		}

		fn push_select_event(&mut self) {
			let mut sorted: Vec<_> = self.selection.iter().cloned().collect();
			sorted.sort();
			self.push_event("select", json!({ "ids": sorted }));
		}

		pub fn set_selection_ids(&mut self, ids: &[String]) {
			let next: BTreeSet<String> = ids.iter().cloned().collect();
			if next == self.selection {
				return;
			}
			self.selection = next;
			self.sync_selection_flags_to_objects();
			self.push_select_event();
		}

		/// @emoji 🧿 True during left‑button rectangle/lasso drag so callers can avoid descriptor round‑trips that fight the live marquee state.
		pub fn is_dragging_area_select(&self) -> bool {
			matches!(&self.interaction, Interaction::Selection { .. })
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
			let t = self.selection_options.target.as_str();
			if t == "nodes" || t == "nodes&edges" {
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
			}
			if t == "edges" || t == "nodes&edges" {
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
			}
			None
		}

		fn merge_pick_into_selection(initial: &BTreeSet<String>, hit_id: &str, mode: &str) -> BTreeSet<String> {
			let mut next = initial.clone();
			match mode {
				"additive" => {
					next.insert(hit_id.to_string());
				}
				"subtractive" => {
					next.remove(hit_id);
				}
				_ => {
					if next.contains(hit_id) {
						next.remove(hit_id);
					} else {
						next.insert(hit_id.to_string());
					}
				}
			}
			next
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

		fn drawable_cull_pad_world(&self) -> f64 {
			16.0 / self.camera.zoom.max(1e-9)
		}

		fn visible_world_box(&self, pad_world: f64) -> WorldBox {
			let corners = [
				self.screen_to_world(Point::new(0.0, 0.0)),
				self.screen_to_world(Point::new(self.width as f64, 0.0)),
				self.screen_to_world(Point::new(self.width as f64, self.height as f64)),
				self.screen_to_world(Point::new(0.0, self.height as f64)),
			];
			let base = world_box_from_points(&corners).unwrap_or(WorldBox {
				min_x: self.camera.x - 1.0,
				min_y: self.camera.y - 1.0,
				max_x: self.camera.x + 1.0,
				max_y: self.camera.y + 1.0,
			});
			inflate_world_box(base, pad_world)
		}

		fn world_tile_screen_clip_rect(&self, ix: i32, iy: i32, tile: f64) -> Rect {
			let wx0 = ix as f64 * tile;
			let wy0 = iy as f64 * tile;
			let wx1 = wx0 + tile;
			let wy1 = wy0 + tile;
			let ps = [
				self.world_to_screen(Point::new(wx0, wy0)),
				self.world_to_screen(Point::new(wx1, wy0)),
				self.world_to_screen(Point::new(wx1, wy1)),
				self.world_to_screen(Point::new(wx0, wy1)),
			];
			let mut min_x = f64::INFINITY;
			let mut min_y = f64::INFINITY;
			let mut max_x = f64::NEG_INFINITY;
			let mut max_y = f64::NEG_INFINITY;
			for p in ps {
				min_x = min_x.min(p.x);
				min_y = min_y.min(p.y);
				max_x = max_x.max(p.x);
				max_y = max_y.max(p.y);
			}
			Rect::from_points(Point::new(min_x, min_y), Point::new(max_x, max_y)).inflate(1.0, 1.0)
		}

		fn handle_world_bounds_cull(&self, h: &HandleData) -> Option<WorldBox> {
			let pos = self.handle_world_pos(h)?;
			let pad = self.drawable_cull_pad_world() + h.radius.max(1.0);
			Some(inflate_world_box(
				WorldBox {
					min_x: pos.x,
					min_y: pos.y,
					max_x: pos.x,
					max_y: pos.y,
				},
				pad,
			))
		}

		fn edge_world_bounds_for_cull(&self, e: &EdgeData) -> Option<WorldBox> {
			let c = self.edge_curve(e)?;
			let axis = cubic_bezier_axis_bounds(c);
			let half_w_world = self.camera.zoom.max(0.75) / self.camera.zoom.max(1e-9);
			Some(inflate_world_box(axis, half_w_world + self.drawable_cull_pad_world()))
		}

		fn append_nodes_handles_edges(&self, scene: &mut Scene, tile_filter: Option<&WorldBox>) {
			let pad = self.drawable_cull_pad_world();
			for n in self.nodes.values() {
				if !n.visible {
					continue;
				}
				if let Some(tb) = tile_filter {
					if !world_boxes_overlap(*tb, self.node_world_bounds(n, pad)) {
						continue;
					}
				}
				let fill = node_fill_color(&self.vello_theme, n.selected);
				let stroke_c = node_stroke_color(&self.vello_theme, n.selected);
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
				if let Some(tb) = tile_filter {
					let Some(hb) = self.handle_world_bounds_cull(h) else { continue };
					if !world_boxes_overlap(*tb, hb) {
						continue;
					}
				}
				let Some(wp) = self.handle_world_pos(h) else { continue };
				let c = self.world_to_screen(wp);
				let r = (h.radius * self.camera.zoom).max(1.0);
				let circle = Circle::new(c, r);
				let fill = handle_fill(&self.vello_theme, h.selected);
				let stroke_c = handle_stroke(&self.vello_theme, h.selected);
				scene.fill(Fill::NonZero, Affine::IDENTITY, fill, None, &circle);
				scene.stroke(&Stroke::new(2.0), Affine::IDENTITY, stroke_c, None, &circle);
			}
			let edge_sw = 2.0 * self.camera.zoom.max(0.75);
			let edge_stroke = Stroke::new(edge_sw);
			for e in self.edges.values() {
				if !e.visible {
					continue;
				}
				if let Some(tb) = tile_filter {
					let Some(eb) = self.edge_world_bounds_for_cull(e) else { continue };
					if !world_boxes_overlap(*tb, eb) {
						continue;
					}
				}
				if let Some(c) = self.edge_curve(e) {
					let p0 = self.world_to_screen(c.p0);
					let p1 = self.world_to_screen(c.p1);
					let p2 = self.world_to_screen(c.p2);
					let p3 = self.world_to_screen(c.p3);
					let curve = CubicBez::new(p0, p1, p2, p3);
					let stroke_color = if e.selected {
						self.vello_theme.edge_stroke_selected
					} else {
						self.vello_theme.edge_stroke
					};
					scene.stroke(&edge_stroke, Affine::IDENTITY, stroke_color, None, &curve);
				}
			}
		}

		pub fn build_vector_scene(&self) -> Scene {
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
					self.vello_theme.grid_minor_stroke,
					None,
					&p,
				);
			}
			let use_tiles = self.world_raster_tiling == "world-clip";
			if use_tiles {
				let pad = self.drawable_cull_pad_world();
				let vis = self.visible_world_box(pad);
				let t = WORLD_CLIP_TILE_WORLD;
				let ix0 = (vis.min_x / t).floor() as i32;
				let iy0 = (vis.min_y / t).floor() as i32;
				let ix1 = (vis.max_x / t).floor() as i32;
				let iy1 = (vis.max_y / t).floor() as i32;
				let nx = (ix1 - ix0 + 1).max(0) as u32;
				let ny = (iy1 - iy0 + 1).max(0) as u32;
				let n_tiles = nx.saturating_mul(ny);
				if n_tiles == 0 || n_tiles > MAX_WORLD_CLIP_TILES {
					self.append_nodes_handles_edges(&mut inner, None);
				} else {
					for iy in iy0..=iy1 {
						for ix in ix0..=ix1 {
							let tile_box = WorldBox {
								min_x: ix as f64 * t,
								min_y: iy as f64 * t,
								max_x: (ix as f64 + 1.0) * t,
								max_y: (iy as f64 + 1.0) * t,
							};
							let clip = self.world_tile_screen_clip_rect(ix, iy, t);
							inner.push_clip_layer(Fill::NonZero, Affine::IDENTITY, &clip);
							self.append_nodes_handles_edges(&mut inner, Some(&tile_box));
							inner.pop_layer();
						}
					}
				}
			} else {
				self.append_nodes_handles_edges(&mut inner, None);
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
						self.vello_theme.selection_preview_fill,
						None,
						&path,
					);
					inner.stroke(
						&Stroke::new(1.5),
						Affine::IDENTITY,
						self.vello_theme.selection_preview_stroke,
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
			let s = self.build_vector_scene();
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
			let mut node_ids: BTreeSet<String> = self.selection.iter().filter(|id| self.nodes.contains_key(*id)).cloned().collect();
			for id in self.selection.iter() {
				if let Some(handle) = self.handles.get(id) {
					node_ids.insert(handle.node_id.clone());
				}
			}
			let node_ids: Vec<_> = node_ids.into_iter().collect();
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
			self.sync_selection_flags_to_objects();
			self.push_select_event();
		}

		pub fn pointer_down_screen(&mut self, sx: f64, sy: f64, button: u8, shift: bool) {
			self.set_selection_screen_preview(None);
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
						let members: Vec<String> = self
							.selection
							.iter()
							.filter(|id| self.nodes.get(*id).is_some_and(|n| n.draggable))
							.cloned()
							.collect();
						let drag_group = members.contains(&nid) && members.len() > 1;
						if !drag_group {
							let next = Self::merge_pick_into_selection(&self.selection, &nid, self.selection_options.mode.as_str());
							let ids: Vec<_> = next.iter().cloned().collect();
							self.set_selection_ids(&ids);
						}
						let mut start_positions = BTreeMap::new();
						for id in if drag_group { members.as_slice() } else { std::slice::from_ref(&nid) } {
							if let Some(n) = self.nodes.get(id) {
								start_positions.insert(id.clone(), (n.x, n.y));
							}
						}
						self.interaction = Interaction::DragNodes {
							primary_id: nid,
							offset: world - Point::new(nx, ny),
							start_positions,
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
				let next = Self::merge_pick_into_selection(&self.selection, &id, self.selection_options.mode.as_str());
				let ids: Vec<_> = next.iter().cloned().collect();
				self.set_selection_ids(&ids);
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
				Interaction::DragNodes {
					primary_id,
					offset,
					start_positions,
				} => {
					let primary_id = primary_id.clone();
					let offset = offset;
					let start_positions_cloned = start_positions.clone();
					let (px0, py0) = start_positions.get(&primary_id).copied().unwrap_or((0.0, 0.0));
					let nx = world.x - offset.x;
					let ny = world.y - offset.y;
					let dx = nx - px0;
					let dy = ny - py0;
					for (id, (ox0, oy0)) in &start_positions {
						if let Some(n) = self.nodes.get_mut(id) {
							let mx = ox0 + dx;
							let my = oy0 + dy;
							n.x = mx;
							n.y = my;
							self.push_event("nodeMove", json!({ "id": id, "x": mx, "y": my }));
						}
					}
					self.interaction = Interaction::DragNodes {
						primary_id,
						offset,
						start_positions: start_positions_cloned,
					};
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
					self.sync_selection_screen_overlay(start_screen, &screen_points);
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
				start_screen,
			} = std::mem::take(&mut self.interaction)
			{
				points.push(world);
				screen_points.push(screen);
				let end_screen = screen_points.last().copied().unwrap_or(start_screen);
				let click_only = distance_between(start_screen, end_screen) < SELECTION_CLICK_MAX_DISTANCE_PX;
				if click_only {
					self.set_selection_ids(&[]);
				} else {
					let next = self.resolve_area_selection_with_initial(&initial_ids, start, &points);
					let ids: Vec<_> = next.iter().cloned().collect();
					self.set_selection_ids(&ids);
				}
				self.set_selection_screen_preview(None);
				return;
			}
			self.interaction = Interaction::None;
		}

		pub fn pointer_leave_screen(&mut self) {
			if matches!(self.interaction, Interaction::None) {
				self.set_hovered_id(None);
			}
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

		fn selection_contains_handle(&self, h: &HandleData, box_: WorldBox, enclosing: bool, polygon: &[Point]) -> bool {
			let Some(pos) = self.handle_world_pos(h) else {
				return false;
			};
			let pad = h.radius.max(1.0);
			let bounds = WorldBox {
				min_x: pos.x - pad,
				min_y: pos.y - pad,
				max_x: pos.x + pad,
				max_y: pos.y + pad,
			};
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
			if t == "nodes&edges" {
				for h in self.handles.values() {
					if h.visible && self.selection_contains_handle(h, box_, enclosing, polygon) {
						hits.insert(h.id.clone());
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

	fn node_fill_color(theme: &VelloThemePalette, sel: bool) -> Color {
		if sel {
			theme.node_fill_selected
		} else {
			theme.node_fill
		}
	}

	fn node_stroke_color(theme: &VelloThemePalette, sel: bool) -> Color {
		if sel {
			theme.node_stroke_selected
		} else {
			theme.node_stroke
		}
	}

	fn handle_fill(theme: &VelloThemePalette, sel: bool) -> Color {
		if sel {
			theme.handle_fill_selected
		} else {
			theme.handle_fill
		}
	}

	fn handle_stroke(theme: &VelloThemePalette, sel: bool) -> Color {
		if sel {
			theme.handle_stroke_selected
		} else {
			theme.handle_stroke
		}
	}
}

pub use board_host::BoardHost;

use std::collections::{BTreeMap, BTreeSet};

pub use vello::kurbo::{CubicBez, Point, Vec2};
use vcompute::{compute_edge_bezier_points, distance_point_to_cubic_bezier, encode_board_stroke_scene};

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
/// ⚙️ Single-file retained board engine; geometry uses cubic curves and vector scene encoding.
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

	pub fn pointer_down(&mut self, x: f64, y: f64, extend_selection: bool) {
		let point = Point::new(x, y);
		match self.hit_test(point) {
			Some(HitObject::Node(node_id)) => {
				self.apply_pick_selection(HitObject::Node(node_id), extend_selection);
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
				self.apply_pick_selection(HitObject::Handle(handle_id), extend_selection);
				self.update_hover(Some(handle_id));
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
		let _stroke_scene = encode_board_stroke_scene(&snapshot.edges, 2.0);
		let _ = _stroke_scene.encoding().path_tags.len();
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

	fn apply_pick_selection(&mut self, hit: HitObject, extend_selection: bool) {
		if !extend_selection {
			self.selection = Selection::default();
		}
		match hit {
			HitObject::Node(id) => {
				self.selection.node_ids.insert(id);
			}
			HitObject::Handle(id) => {
				self.selection.handle_ids.insert(id);
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

// #region 🔖WasmSession
/// 🖥️ Single WASM entry: one {@link BoardHost}, optional WebGPU surface bound via {@link BoardSession::attach_canvas}.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct BoardSession {
	host: BoardHost,
	#[cfg_attr(target_arch = "wasm32", allow(dead_code, reason = "Retains canvas for the WebGPU surface lifetime."))]
	canvas: Option<HtmlCanvasElement>,
	render_ctx: Option<vello::util::RenderContext>,
	renderer: Option<vello::Renderer>,
	surface: Option<vello::util::RenderSurface<'static>>,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl BoardSession {
	#[wasm_bindgen(constructor)]
	pub fn new() -> Self {
		Self {
			host: BoardHost::new(),
			canvas: None,
			render_ctx: None,
			renderer: None,
			surface: None,
		}
	}

	#[wasm_bindgen(js_name = gpuReady)]
	pub fn gpu_ready(&self) -> bool {
		self.surface.is_some()
	}

	#[wasm_bindgen(js_name = isDraggingAreaSelect)]
	pub fn is_dragging_area_select(&self) -> bool {
		self.host.is_dragging_area_select()
	}

	/// @emoji 🌊 Binds WebGPU presentation to `canvas` once; `logical_w`/`logical_h` are CSS pixels, `dpr` scales the swapchain backing store.
	pub async fn attach_canvas(
		&mut self,
		canvas: HtmlCanvasElement,
		logical_w: u32,
		logical_h: u32,
		dpr: f64,
	) -> Result<(), JsValue> {
		if self.surface.is_some() {
			return Err(JsValue::from_str("canvas surface already attached"));
		}
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
		self.host.set_size(lw, lh, dpr);
		self.canvas = Some(canvas);
		self.render_ctx = Some(render_ctx);
		self.renderer = Some(renderer);
		self.surface = Some(surface);
		Ok(())
	}

	#[wasm_bindgen(js_name = setSize)]
	pub fn set_size(&mut self, width: u32, height: u32, dpr: f64) {
		let lw = width.max(1);
		let lh = height.max(1);
		let dpr = dpr.max(1.0);
		let pw = ((lw as f64 * dpr).round() as u32).max(1);
		let ph = ((lh as f64 * dpr).round() as u32).max(1);
		self.host.set_size(lw, lh, dpr);
		if let (Some(ref mut surface), Some(ref mut render_ctx)) = (self.surface.as_mut(), self.render_ctx.as_mut()) {
			let cur_w = surface.config.width;
			let cur_h = surface.config.height;
			if cur_w != pw || cur_h != ph {
				render_ctx.resize_surface(surface, pw, ph);
			}
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

	#[wasm_bindgen(js_name = setVelloThemeJson)]
	pub fn set_vello_theme_json(&mut self, json: &str) {
		let _ = self.host.set_vello_theme_from_json(json);
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

	#[wasm_bindgen(js_name = pointerLeaveScreen)]
	pub fn pointer_leave_screen_wasm(&mut self) {
		self.host.pointer_leave_screen();
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

		#[wasm_bindgen(js_name = setWorldRasterTiling)]
		pub fn set_world_raster_tiling_wasm(&mut self, mode: &str) {
			self.host.set_world_raster_tiling(mode);
		}

		#[wasm_bindgen(js_name = setSelectionIdsJson)]
	pub fn set_selection_ids_json(&mut self, json: &str) -> Result<(), JsValue> {
		let ids: Vec<String> = serde_json::from_str(json).map_err(|err| JsValue::from_str(&err.to_string()))?;
		self.host.set_selection_ids(&ids);
		Ok(())
	}

	#[wasm_bindgen(js_name = encodedSceneHint)]
	pub fn encoded_scene_hint_wasm(&self) -> usize {
		self.host.encoded_scene_hint()
	}

	/// @emoji 🎨 Presents one frame when a GPU surface is attached; otherwise no-op `Ok`.
	#[wasm_bindgen(js_name = renderFrame)]
	pub fn render_frame(&mut self) -> Result<(), JsValue> {
		for _attempt in 0..3u8 {
			let scene = self.host.build_vector_scene();
			let Some(surface) = self.surface.as_mut() else {
				return Ok(());
			};
			let Some(renderer) = self.renderer.as_mut() else {
				return Ok(());
			};
			let Some(render_ctx) = self.render_ctx.as_mut() else {
				return Ok(());
			};
			let dh = &render_ctx.devices[surface.dev_id];
			let pw = surface.config.width.max(1);
			let ph = surface.config.height.max(1);
			let params = vello::RenderParams {
				base_color: self.host.vello_theme.raster_clear,
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
					render_ctx.configure_surface(surface);
					continue;
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
				label: Some("elements_board_surface_blit"),
			});
			surface
				.blitter
				.copy(&dh.device, &mut encoder, &surface.target_view, &view);
			dh.queue.submit(std::iter::once(encoder.finish()));
			surface_tex.present();
			let _ = dh.device.poll(vello::wgpu::PollType::Poll);
			return Ok(());
		}
		Ok(())
	}
}
// #endregion 🔖WasmSession

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
		let align1 = vcompute::normalize_or_zero(inward).dot(vcompute::normalize_or_zero(arm1));
		assert!(align0 > 0.99);
		assert!(align1 > 0.99);
	}

	#[test]
	fn drags_nodes_without_rebuilding_the_scene_catalog() {
		let mut engine = BoardEngine::new();
		engine.create_node(1, 0.0, 0.0, 30.0, true);

		engine.pointer_down(0.0, 0.0, false);
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
		engine.pointer_down(handle_point.x, handle_point.y, false);

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

	#[test]
	fn engine_extend_pick_keeps_node_when_adding_handle() {
		let mut engine = BoardEngine::new();
		engine.create_node(1, 0.0, 0.0, 40.0, true);
		engine.create_node(2, 300.0, 0.0, 40.0, true);
		engine.create_handle(10, 1, 0.0);
		engine.create_handle(20, 2, std::f64::consts::PI);
		engine.create_edge(100, 10, 20);

		engine.pointer_down(0.0, 0.0, false);
		let _ = engine.drain_events();
		let hp = handle_position(engine.nodes.get(&1).unwrap(), engine.handles.get(&10).unwrap());
		engine.pointer_down(hp.x, hp.y, true);
		let events = engine.drain_events();
		let last = events.iter().rev().find_map(|event| match event {
			BoardEvent::SelectionChanged {
				node_ids,
				handle_ids,
				edge_ids,
			} => Some((node_ids.clone(), handle_ids.clone(), edge_ids.clone())),
			_ => None,
		});
		let Some((node_ids, handle_ids, edge_ids)) = last else {
			panic!("expected SelectionChanged");
		};
		assert!(node_ids.contains(&1));
		assert!(handle_ids.contains(&10));
		assert!(edge_ids.is_empty());
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

	#[test]
	fn board_host_multi_select_drag_moves_each_selected_node() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		let mut desc = sample_scene();
		desc.nodes.push(NodeDescJson {
			id: "b".into(),
			x: 100.0,
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
		h.set_selection_ids(&["a".into(), "b".into()]);
		let _ = h.drain_events_json();
		let w = Point::new(0.0, 0.0);
		let s = h.world_to_screen(w);
		h.pointer_down_screen(s.x, s.y, 0, false);
		h.pointer_move_screen(s.x + 10.0, s.y + 5.0);
		h.pointer_up_screen(s.x + 10.0, s.y + 5.0);
		let a = h.nodes.get("a").expect("node a");
		let b = h.nodes.get("b").expect("node b");
		assert!((a.x - 10.0).abs() < 1e-6);
		assert!((a.y - 5.0).abs() < 1e-6);
		assert!((b.x - 110.0).abs() < 1e-6);
		assert!((b.y - 5.0).abs() < 1e-6);
		let sorted: Vec<_> = h.selection.iter().cloned().collect();
		assert_eq!(sorted, vec!["a".to_string(), "b".to_string()]);
	}

	#[test]
	fn board_host_selection_target_edges_skips_node_geometry() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		h.set_selection_options("rectangle", "invertive", "edges");
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
		let inside_node_a = Point::new(0.0, 0.0);
		assert!(h.resolve_hit_world(inside_node_a).is_none());
		let on_edge = Point::new(150.0, 0.0);
		assert_eq!(h.resolve_hit_world(on_edge).as_deref(), Some("e1"));
	}

	#[test]
	fn board_host_additive_click_merges_edge_into_existing_selection() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		h.set_selection_options("rectangle", "additive", "nodes&edges");
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
		h.set_selection_ids(&["a".into()]);
		let _ = h.drain_events_json();
		let on_edge = Point::new(150.0, 0.0);
		let s = h.world_to_screen(on_edge);
		h.pointer_down_screen(s.x, s.y, 0, false);
		let mut got: Vec<_> = h.selection.iter().cloned().collect();
		got.sort();
		assert_eq!(got, vec!["a".to_string(), "e1".to_string()]);
	}

	#[test]
	fn board_host_background_click_without_drag_clears_selection() {
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
		h.set_selection_ids(&["a".into(), "e1".into()]);
		let away = Point::new(5000.0, 5000.0);
		let s = h.world_to_screen(away);
		h.pointer_down_screen(s.x, s.y, 0, false);
		h.pointer_up_screen(s.x, s.y);
		assert!(h.selection.is_empty());
	}

	#[test]
	fn board_host_rectangle_area_select_includes_handles_with_nodes() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		h.set_selection_options("rectangle", "invertive", "nodes&edges");
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
		let w0 = Point::new(-90.0, -70.0);
		let w1 = Point::new(90.0, 90.0);
		let s0 = h.world_to_screen(w0);
		let s1 = h.world_to_screen(w1);
		h.pointer_down_screen(s0.x, s0.y, 0, false);
		h.pointer_move_screen(s1.x, s1.y);
		h.pointer_up_screen(s1.x, s1.y);
		let mut got: Vec<_> = h.selection.iter().cloned().collect();
		got.sort();
		assert!(got.contains(&"a".to_string()));
		assert!(got.contains(&"a.out".to_string()));
	}
}

// #endregion 🔖Tests
