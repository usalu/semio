//! 🎛️ Single-source board crate: vector geometry (`vcompute`), selection predicates (`geom_sel`), serde scene JSON (`scene_json`), interactive `BoardHost`, retained `BoardEngine`, and wasm-bindgen facades — all in this file (nested `mod` blocks only; no extra `.rs` files).
#![allow(clippy::missing_errors_doc, reason = "Board engine is internal to the elements board bundle.")]

pub use vello_svg::usvg;
pub use vello_svg::vello;

// #region 🏷️BoardIconAssets

mod board_icon_assets {
	//! @emoji 📎 Static bytes for board icon rendering; `include_bytes!` paths are relative to this `lib.rs` file.

	pub static NOTO_COLOR_EMOJI_SUBSET_TTF: &[u8] = include_bytes!("assets/NotoColorEmoji-subset.ttf");
}

// #endregion 🏷️BoardIconAssets

mod vcompute {
	use crate::vello::kurbo::{Affine, CubicBez, ParamCurve, Point, Vec2, Stroke};
	use crate::vello::peniko::Color;
	use crate::vello::Scene;

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

	/// 🧭 East-zero polar angle for a circle handle that meets the ray from `center` toward `toward` on the rim.
	pub fn circle_handle_angle_toward(center: Point, toward: Point) -> f64 {
		let d = toward - center;
		f64::atan2(d.y, d.x)
	}

	/// 🧭 North-zero rectangle handle angle so the rim point lies on the ray from `center` toward `toward`.
	pub fn rectangle_handle_angle_toward(center: Point, _width: f64, _height: f64, toward: Point) -> f64 {
		let u = normalize_or_zero(toward - center);
		f64::atan2(-u.x, -u.y)
	}

	pub fn compute_edge_bezier_points(
		source_point: Point,
		target_point: Point,
		source_center: Point,
		target_center: Point,
	) -> CubicBez {
		let mut source_radial = normalize_or_zero(source_point - source_center);
		if source_radial == Vec2::new(0.0, 0.0) {
			source_radial = normalize_or_zero(target_point - source_point);
		}
		let mut target_radial = normalize_or_zero(target_point - target_center);
		if target_radial == Vec2::new(0.0, 0.0) {
			target_radial = normalize_or_zero(target_point - source_point);
		}
		let handle_distance = distance_between(source_point, target_point);
		let control_length = clamp_f64(handle_distance * 0.35, 24.0, 240.0);
		let p1 = source_point + source_radial * control_length;
		let p2 = target_point + target_radial * control_length;
		CubicBez::new(source_point, p1, p2, target_point)
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
	use crate::vello::kurbo::{CubicBez, ParamCurve, Point};

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
		/// @emoji 🏷️ Runtime host encoding: catalog id from the baked icon table or inline SVG (`<?xml` / `<svg` …) parsed at detail LOD.
		#[serde(default)]
		pub icon_kind: Option<String>,
		/// @emoji 🧩 Semantic node-kind id for compatibility rows at `node` specificity.
		#[serde(default)]
		pub node_kind: Option<String>,
		#[serde(default)]
		pub user_data: Option<serde_json::Value>,
		#[serde(default)]
		pub visible: Option<bool>,
		#[serde(default)]
		pub root: Option<bool>,
		pub shape: Option<String>,
		#[serde(default)]
		pub radius: Option<f64>,
		#[serde(default)]
		pub width: Option<f64>,
		#[serde(default)]
		pub height: Option<f64>,
		#[serde(default)]
		pub scale: Option<f64>,
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
		pub handle_kind: Option<String>,
		/// CSS `#rgb` / `#rrggbb` / `#rrggbbaa` overriding catalog color for this handle.
		#[serde(default)]
		pub color: Option<String>,
		/// @emoji 🏷️ Runtime host encoding: `typst:`, `emoji:`, `image:data:…`, catalog id, or inline SVG for detail LOD.
		#[serde(default)]
		pub icon_kind: Option<String>,
		#[serde(default)]
		pub user_data: Option<serde_json::Value>,
		#[serde(default)]
		pub visible: Option<bool>,
		#[serde(default)]
		pub scale: Option<f64>,
	}

	#[derive(Clone, Debug, Deserialize, Serialize)]
	#[serde(rename_all = "camelCase")]
	pub struct EdgeDescJson {
		pub id: String,
		pub source: String,
		pub target: String,
		/// @emoji 🧩 Semantic edge-kind id for compatibility at `edge` specificity.
		#[serde(default)]
		pub edge_kind: Option<String>,
		#[serde(default)]
		pub selected: Option<bool>,
		#[serde(default)]
		pub style: Option<String>,
		#[serde(default)]
		pub user_data: Option<serde_json::Value>,
		#[serde(default)]
		pub visible: Option<bool>,
	}

	/// @emoji 🧵 Transient cubic link from a handle to another handle or a free world point (descriptor + link gesture).
	#[derive(Clone, Debug, Deserialize, Serialize)]
	#[serde(rename_all = "camelCase")]
	pub struct WireDescJson {
		pub id: String,
		pub source: String,
		/// @emoji 🧩 Semantic wire-kind id (defaults from catalog when omitted in fixtures).
		#[serde(default)]
		pub wire_kind: Option<String>,
		#[serde(default)]
		pub target: Option<String>,
		#[serde(default)]
		pub end_x: Option<f64>,
		#[serde(default)]
		pub end_y: Option<f64>,
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
	#[serde(rename_all = "camelCase")]
	pub struct SceneDescriptorJson {
		pub nodes: Vec<NodeDescJson>,
		pub handles: Vec<HandleDescJson>,
		pub edges: Vec<EdgeDescJson>,
		#[serde(default)]
		pub wires: Vec<WireDescJson>,
		/// @emoji 💠 JS‑authored ids to paint with secondary “left selection” chrome (not in current `selected` flags).
		#[serde(default)]
		pub selection_exit_highlight_ids: Vec<String>,
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

	/// 🧾 Reads fixture edge endpoint handle ids from `source` and `target` string fields only.
	pub fn fixture_edge_handle_ids_from_object(
		eo: &serde_json::Map<String, serde_json::Value>,
	) -> Option<(&str, &str)> {
		let source = eo.get("source").and_then(|v| v.as_str())?;
		let target = eo.get("target").and_then(|v| v.as_str())?;
		Some((source, target))
	}
}

pub use scene_json::{
	CameraJson, EdgeDescJson, FixtureV1Json, HandleDescJson, NodeDescJson, SceneDescriptorJson, WireDescJson,
	fixture_edge_handle_ids_from_object,
};

fn board_json_hidden_flag(obj: &serde_json::Map<String, serde_json::Value>) -> Option<bool> {
	obj.get("hidden").and_then(|v| v.as_bool())
}

fn board_json_visible_option(obj: &serde_json::Map<String, serde_json::Value>) -> Option<bool> {
	match board_json_hidden_flag(obj) {
		Some(hidden) => Some(!hidden),
		None => obj.get("visible").and_then(|v| v.as_bool()),
	}
}

fn board_json_visible_or_true(obj: &serde_json::Map<String, serde_json::Value>) -> bool {
	board_json_visible_option(obj).unwrap_or(true)
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn normalize_board_descriptor_hidden_to_visible(value: &mut serde_json::Value) {
	let Some(root) = value.as_object_mut() else {
		return;
	};
	for key in ["nodes", "handles", "edges", "wires"] {
		let Some(rows) = root.get_mut(key).and_then(|v| v.as_array_mut()) else {
			continue;
		};
		for row in rows {
			let Some(obj) = row.as_object_mut() else {
				continue;
			};
			if let Some(visible) = board_json_visible_option(obj) {
				obj.insert("visible".into(), serde_json::json!(visible));
			}
		}
	}
}

// #region 🕸️ForceGraphLayout
mod force_graph {
	use serde::{Deserialize, Serialize};
	use serde_json::Value;
	use std::collections::{HashMap, HashSet};
	use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign};

	use super::{board_json_visible_or_true, fixture_edge_handle_ids_from_object};

	// #region 🔖Vec2
	/// 📐 Tiny 2-vector for force layout only (no external linear-algebra crate).
	#[derive(Clone, Copy, Debug)]
	struct Vec2 {
		x: f64,
		y: f64,
	}

	impl Vec2 {
		const ZERO: Self = Self { x: 0.0, y: 0.0 };
		#[inline]
		fn new(x: f64, y: f64) -> Self {
			Self { x, y }
		}
		#[inline]
		fn norm(self) -> f64 {
			(self.x * self.x + self.y * self.y).sqrt()
		}
	}

	impl Add for Vec2 {
		type Output = Self;
		#[inline]
		fn add(self, rhs: Self) -> Self {
			Self::new(self.x + rhs.x, self.y + rhs.y)
		}
	}

	impl AddAssign for Vec2 {
		#[inline]
		fn add_assign(&mut self, rhs: Self) {
			self.x += rhs.x;
			self.y += rhs.y;
		}
	}

	impl Sub for Vec2 {
		type Output = Self;
		#[inline]
		fn sub(self, rhs: Self) -> Self {
			Self::new(self.x - rhs.x, self.y - rhs.y)
		}
	}

	impl SubAssign for Vec2 {
		#[inline]
		fn sub_assign(&mut self, rhs: Self) {
			self.x -= rhs.x;
			self.y -= rhs.y;
		}
	}

	impl Mul<f64> for Vec2 {
		type Output = Self;
		#[inline]
		fn mul(self, s: f64) -> Self {
			Self::new(self.x * s, self.y * s)
		}
	}

	impl Mul<Vec2> for f64 {
		type Output = Vec2;
		#[inline]
		fn mul(self, v: Vec2) -> Vec2 {
			v * self
		}
	}

	impl MulAssign<f64> for Vec2 {
		#[inline]
		fn mul_assign(&mut self, s: f64) {
			self.x *= s;
			self.y *= s;
		}
	}

	impl Div<f64> for Vec2 {
		type Output = Self;
		#[inline]
		fn div(self, s: f64) -> Self {
			Self::new(self.x / s, self.y / s)
		}
	}
	// #endregion

	#[derive(Clone, Debug, Deserialize, Serialize)]
	#[serde(rename_all = "camelCase")]
	pub struct ForceGraphLayoutOptions {
		#[serde(default = "default_iterations")]
		pub iterations: u32,
		#[serde(default = "default_ideal_edge_length")]
		pub ideal_edge_length: f64,
		#[serde(default = "default_repulsion_strength")]
		pub repulsion_strength: f64,
		#[serde(default = "default_spring_strength")]
		pub spring_strength: f64,
		#[serde(default = "default_gravity")]
		pub gravity: f64,
		#[serde(default)]
		pub center_x: Option<f64>,
		#[serde(default)]
		pub center_y: Option<f64>,
		#[serde(default = "default_time_step")]
		pub time_step: f64,
		#[serde(default = "default_velocity_damping")]
		pub velocity_damping: f64,
		#[serde(default = "default_max_speed")]
		pub max_speed: f64,
		#[serde(default = "default_random_seed")]
		pub random_seed: u64,
		/// Barnes–Hut opening angle θ (`width / distance`); smaller is more accurate, larger is faster.
		#[serde(default = "default_barnes_hut_theta")]
		pub barnes_hut_theta: f64,
		/// Use exact O(n²) repulsion when the visible body count is at most this (tiny graphs / tests).
		#[serde(default = "default_pairwise_repulsion_max_bodies")]
		pub pairwise_repulsion_max_bodies: u32,
		/// Node ids whose `x`/`y` stay fixed for this layout pass (pinned bodies still participate in repulsion and springs).
		#[serde(default)]
		pub locked_node_ids: Vec<String>,
	}

	fn default_iterations() -> u32 {
		420
	}
	fn default_ideal_edge_length() -> f64 {
		140.0
	}
	fn default_repulsion_strength() -> f64 {
		6500.0
	}
	fn default_spring_strength() -> f64 {
		0.028
	}
	fn default_gravity() -> f64 {
		0.018
	}
	fn default_time_step() -> f64 {
		0.85
	}
	fn default_velocity_damping() -> f64 {
		0.88
	}
	fn default_max_speed() -> f64 {
		48.0
	}
	fn default_random_seed() -> u64 {
		0x5eedfaced0u64
	}
	fn default_barnes_hut_theta() -> f64 {
		0.78
	}
	fn default_pairwise_repulsion_max_bodies() -> u32 {
		56
	}

	impl Default for ForceGraphLayoutOptions {
		fn default() -> Self {
			Self {
				iterations: default_iterations(),
				ideal_edge_length: default_ideal_edge_length(),
				repulsion_strength: default_repulsion_strength(),
				spring_strength: default_spring_strength(),
				gravity: default_gravity(),
				center_x: None,
				center_y: None,
				time_step: default_time_step(),
				velocity_damping: default_velocity_damping(),
				max_speed: default_max_speed(),
				random_seed: default_random_seed(),
				barnes_hut_theta: default_barnes_hut_theta(),
				pairwise_repulsion_max_bodies: default_pairwise_repulsion_max_bodies(),
				locked_node_ids: Vec::new(),
			}
		}
	}

	fn split_mix64(mut z: u64) -> u64 {
		z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
		z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
		z ^ (z >> 31)
	}

	fn rand_unit_interval(seed: &mut u64) -> f64 {
		*seed = split_mix64(*seed);
		(*seed as f64) / (u64::MAX as f64)
	}

	fn node_repulsion_radius(node: &Value) -> f64 {
		let Some(obj) = node.as_object() else {
			return 32.0;
		};
		if obj.get("shape").and_then(|v| v.as_str()) == Some("rectangle") {
			let w = obj.get("width").and_then(|v| v.as_f64()).unwrap_or(40.0);
			let h = obj.get("height").and_then(|v| v.as_f64()).unwrap_or(40.0);
			return ((w * w + h * h).sqrt() * 0.5).max(8.0);
		}
		obj.get("radius").and_then(|v| v.as_f64()).filter(|r| r.is_finite() && *r > 0.0).unwrap_or(32.0)
	}

	// #region 🔖ForceGraphRepulsion
	/// 📐 Repulsive acceleration on body `i` from body `j` (shared by pairwise sweep and Barnes–Hut leaves).
	#[inline]
	fn pairwise_repulsion_on_i_from_j(
		i: usize,
		j: usize,
		positions: &[Vec2],
		radii: &[f64],
		cool: f64,
		k_rep: f64,
	) -> Vec2 {
		let delta = positions[j] - positions[i];
		let dist = delta.norm().max(1e-4);
		let rep = k_rep * cool * (radii[i] * radii[j]).max(1.0) / (dist * dist);
		(delta / dist) * (-rep)
	}

	mod barnes_hut {
		use super::{pairwise_repulsion_on_i_from_j, Vec2};

		const NO_CHILD: u32 = u32::MAX;

		/// 🌌 Quadtree cell: empty leaf, occupied leaf, or internal node with four children.
		#[derive(Clone, Debug)]
		struct Cell {
			min_x: f64,
			min_y: f64,
			max_x: f64,
			max_y: f64,
			body: Option<usize>,
			children: [u32; 4],
			com: Vec2,
			mass: f64,
			max_r: f64,
		}

		/// 🌲 Point quadtree for one repulsion pass over a fixed body set.
		pub(super) struct QuadTree {
			cells: Vec<Cell>,
		}

		#[inline]
		fn is_internal(c: &Cell) -> bool {
			c.children[0] != NO_CHILD
		}

		#[inline]
		fn cell_width(c: &Cell) -> f64 {
			(c.max_x - c.min_x).max(c.max_y - c.min_y)
		}

		#[inline]
		fn point_in_cell(px: f64, py: f64, c: &Cell) -> bool {
			px >= c.min_x && px <= c.max_x && py >= c.min_y && py <= c.max_y
		}

		fn quadrant_bounds(min_x: f64, min_y: f64, max_x: f64, max_y: f64, q: usize) -> (f64, f64, f64, f64) {
			let mx = (min_x + max_x) * 0.5;
			let my = (min_y + max_y) * 0.5;
			match q {
				0 => (min_x, min_y, mx, my),
				1 => (mx, min_y, max_x, my),
				2 => (min_x, my, mx, max_y),
				3 => (mx, my, max_x, max_y),
				_ => (min_x, min_y, max_x, max_y),
			}
		}

		#[inline]
		fn quadrant_index(px: f64, py: f64, mx: f64, my: f64) -> usize {
			let east = if px >= mx { 1usize } else { 0 };
			let north = if py >= my { 2usize } else { 0 };
			east + north
		}

		fn empty_leaf(min_x: f64, min_y: f64, max_x: f64, max_y: f64) -> Cell {
			Cell {
				min_x,
				min_y,
				max_x,
				max_y,
				body: None,
				children: [NO_CHILD; 4],
				com: Vec2::ZERO,
				mass: 0.0,
				max_r: 0.0,
			}
		}

		fn subdivide_leaf(tree: &mut Vec<Cell>, ni: usize) {
			let (min_x, min_y, max_x, max_y) = (tree[ni].min_x, tree[ni].min_y, tree[ni].max_x, tree[ni].max_y);
			let mut ch = [NO_CHILD; 4];
			for q in 0..4usize {
				let (a, b, c, d) = quadrant_bounds(min_x, min_y, max_x, max_y, q);
				let idx = tree.len();
				tree.push(empty_leaf(a, b, c, d));
				ch[q] = idx as u32;
			}
			tree[ni].body = None;
			tree[ni].children = ch;
			tree[ni].mass = 0.0;
			tree[ni].com = Vec2::ZERO;
			tree[ni].max_r = 0.0;
		}

		fn aggregate(tree: &mut Vec<Cell>, ni: usize) {
			if !is_internal(&tree[ni]) {
				return;
			}
			let mut m = 0.0f64;
			let mut sx = 0.0f64;
			let mut sy = 0.0f64;
			let mut mxr = 0.0f64;
			let ch = tree[ni].children;
			for q in 0..4usize {
				let chn = &tree[ch[q] as usize];
				m += chn.mass;
				sx += chn.com.x * chn.mass;
				sy += chn.com.y * chn.mass;
				mxr = mxr.max(chn.max_r);
			}
			if m > 0.0 {
				sx /= m;
				sy /= m;
			}
			tree[ni].mass = m;
			tree[ni].com = Vec2::new(sx, sy);
			tree[ni].max_r = mxr;
		}

		fn insert(tree: &mut Vec<Cell>, ni: usize, idx: usize, pos: Vec2, r: f64, positions: &[Vec2], radii: &[f64]) {
			if is_internal(&tree[ni]) {
				let mx = (tree[ni].min_x + tree[ni].max_x) * 0.5;
				let my = (tree[ni].min_y + tree[ni].max_y) * 0.5;
				let q = quadrant_index(pos.x, pos.y, mx, my);
				let ci = tree[ni].children[q] as usize;
				insert(tree, ci, idx, pos, r, positions, radii);
				aggregate(tree, ni);
				return;
			}
			if tree[ni].mass <= 0.0 && tree[ni].body.is_none() {
				tree[ni].body = Some(idx);
				tree[ni].com = pos;
				tree[ni].mass = 1.0;
				tree[ni].max_r = r;
				return;
			}
			if let Some(ex) = tree[ni].body {
				if ex == idx {
					return;
				}
				let p_ex = positions[ex];
				let r_ex = radii[ex];
				subdivide_leaf(tree, ni);
				insert(tree, ni, ex, p_ex, r_ex, positions, radii);
				insert(tree, ni, idx, pos, r, positions, radii);
			}
		}

		fn square_bounds(positions: &[Vec2]) -> (f64, f64, f64, f64) {
			let mut min_x = f64::INFINITY;
			let mut min_y = f64::INFINITY;
			let mut max_x = f64::NEG_INFINITY;
			let mut max_y = f64::NEG_INFINITY;
			for p in positions {
				min_x = min_x.min(p.x);
				min_y = min_y.min(p.y);
				max_x = max_x.max(p.x);
				max_y = max_y.max(p.y);
			}
			if !min_x.is_finite() || !max_x.is_finite() {
				return (-1.0, -1.0, 1.0, 1.0);
			}
			let pad = 1e-3f64;
			let w = ((max_x - min_x).max(max_y - min_y) + pad * 2.0).max(1e-6);
			let cx = (min_x + max_x) * 0.5;
			let cy = (min_y + max_y) * 0.5;
			let h = w * 0.5;
			(cx - h, cy - h, cx + h, cy + h)
		}

		fn repulsion_rec(
			cells: &[Cell],
			ni: usize,
			i: usize,
			p_i: Vec2,
			r_i: f64,
			theta: f64,
			cool: f64,
			k_rep: f64,
			positions: &[Vec2],
			radii: &[f64],
		) -> Vec2 {
			let node = &cells[ni];
			if node.mass <= 0.0 {
				return Vec2::ZERO;
			}
			if !is_internal(node) {
				if let Some(j) = node.body {
					if j == i {
						return Vec2::ZERO;
					}
					return pairwise_repulsion_on_i_from_j(i, j, positions, radii, cool, k_rep);
				}
				return Vec2::ZERO;
			}
			let width = cell_width(node);
			let delta_c = node.com - p_i;
			let d = delta_c.norm().max(1e-6);
			if point_in_cell(p_i.x, p_i.y, node) {
				let mut acc = Vec2::ZERO;
				for q in 0..4usize {
					let c = node.children[q];
					if c != NO_CHILD {
						acc += repulsion_rec(cells, c as usize, i, p_i, r_i, theta, cool, k_rep, positions, radii);
					}
				}
				return acc;
			}
			if width / d < theta {
				let rep = k_rep * cool * (r_i * node.max_r).max(1.0) / (d * d);
				return (p_i - node.com) / d * rep;
			}
			let mut acc = Vec2::ZERO;
			for q in 0..4usize {
				let c = node.children[q];
				if c != NO_CHILD {
					acc += repulsion_rec(cells, c as usize, i, p_i, r_i, theta, cool, k_rep, positions, radii);
				}
			}
			acc
		}

		impl QuadTree {
			pub(super) fn build(positions: &[Vec2], radii: &[f64]) -> Self {
				let (a, b, c, d) = square_bounds(positions);
				let mut cells = vec![empty_leaf(a, b, c, d)];
				for i in 0..positions.len() {
					insert(&mut cells, 0, i, positions[i], radii[i], positions, radii);
				}
				Self { cells }
			}

			pub(super) fn repulsion_on_body(
				&self,
				i: usize,
				positions: &[Vec2],
				radii: &[f64],
				theta: f64,
				cool: f64,
				k_rep: f64,
			) -> Vec2 {
				repulsion_rec(
					&self.cells,
					0,
					i,
					positions[i],
					radii[i],
					theta,
					cool,
					k_rep,
					positions,
					radii,
				)
			}
		}
	}

	fn add_repulsion_forces(
		forces: &mut [Vec2],
		positions: &[Vec2],
		radii: &[f64],
		n: usize,
		cool: f64,
		k_rep: f64,
		theta: f64,
		pair_cap: usize,
	) {
		if n <= pair_cap {
			for i in 0..n {
				for j in (i + 1)..n {
					let f = pairwise_repulsion_on_i_from_j(i, j, positions, radii, cool, k_rep);
					forces[i] += f;
					forces[j] -= f;
				}
			}
		} else {
			let tree = barnes_hut::QuadTree::build(positions, radii);
			for i in 0..n {
				forces[i] += tree.repulsion_on_body(i, positions, radii, theta, cool, k_rep);
			}
		}
	}
	// #endregion

	// #region 🔖ForceGraphIntegration
	fn add_spring_forces(
		forces: &mut [Vec2],
		positions: &[Vec2],
		edge_pairs: &[(usize, usize)],
		ideal_len: f64,
		spring_k: f64,
		cool: f64,
	) {
		for &(i, j) in edge_pairs {
			let delta = positions[j] - positions[i];
			let dist = delta.norm().max(1e-4);
			let dir = delta / dist;
			let displacement = dist - ideal_len;
			let f = dir * (spring_k * cool * displacement);
			forces[i] += f;
			forces[j] -= f;
		}
	}

	fn add_gravity_toward(forces: &mut [Vec2], positions: &[Vec2], gx: f64, gy: f64, gamma: f64, cool: f64) {
		if gamma <= 0.0 {
			return;
		}
		let g = gamma * cool;
		for i in 0..forces.len() {
			let to_c = Vec2::new(gx - positions[i].x, gy - positions[i].y);
			forces[i] += to_c * g;
		}
	}

	fn integrate_velocity_and_positions(
		positions: &mut [Vec2],
		velocities: &mut [Vec2],
		forces: &[Vec2],
		dt_base: f64,
		cool: f64,
		damping: f64,
		v_max: f64,
	) {
		let dt = dt_base * cool.sqrt();
		for i in 0..positions.len() {
			let mut v = (velocities[i] + forces[i] * dt) * damping;
			let spd = v.norm();
			if spd > v_max {
				v *= v_max / spd;
			}
			velocities[i] = v;
			positions[i] += v * dt;
		}
	}

	fn zero_forces_on_pinned(forces: &mut [Vec2], pin: &[Option<Vec2>]) {
		for i in 0..forces.len() {
			if pin[i].is_some() {
				forces[i] = Vec2::ZERO;
			}
		}
	}

	fn enforce_pin_constraints(positions: &mut [Vec2], velocities: &mut [Vec2], pin: &[Option<Vec2>]) {
		for i in 0..positions.len() {
			if let Some(p) = pin[i] {
				positions[i] = p;
				velocities[i] = Vec2::ZERO;
			}
		}
	}
	// #endregion

	pub fn apply_force_graph_layout_to_fixture_v1_value(fixture: &mut Value, opts: &ForceGraphLayoutOptions) -> Result<(), String> {
		let Some(root) = fixture.as_object_mut() else {
			return Err("fixture root must be object".into());
		};
		if root.get("schema").and_then(|v| v.as_str()) != Some("elements.board.fixture/v1") {
			return Err("schema must be elements.board.fixture/v1".into());
		}
		let edges = root.get("edges").and_then(|v| v.as_array()).cloned().unwrap_or_default();
		let Some(nodes) = root.get_mut("nodes").and_then(|v| v.as_array_mut()) else {
			return Err("nodes array missing".into());
		};
		if nodes.is_empty() {
			return Ok(());
		}
		let locked_ids: HashSet<String> = opts.locked_node_ids.iter().cloned().collect();
		let mut handle_to_node: HashMap<String, String> = HashMap::new();
		for node in nodes.iter() {
			let Some(obj) = node.as_object() else {
				continue;
			};
			if !board_json_visible_or_true(obj) {
				continue;
			}
			let Some(nid) = obj.get("id").and_then(|v| v.as_str()) else {
				continue;
			};
			let Some(handles) = obj.get("handles").and_then(|v| v.as_array()) else {
				continue;
			};
			for h in handles {
				let Some(ho) = h.as_object() else {
					continue;
				};
				if !board_json_visible_or_true(ho) {
					continue;
				}
				if let Some(hid) = ho.get("id").and_then(|v| v.as_str()) {
					handle_to_node.insert(hid.to_string(), nid.to_string());
				}
			}
		}
		let mut id_to_index: HashMap<String, usize> = HashMap::new();
		let mut visible_node_indices: Vec<usize> = Vec::new();
		let mut optional_xy: Vec<Option<(f64, f64)>> = Vec::new();
		let mut is_locked: Vec<bool> = Vec::new();
		let mut positions: Vec<Vec2> = Vec::new();
		let mut velocities: Vec<Vec2> = Vec::new();
		let mut radii: Vec<f64> = Vec::new();
		for (raw_idx, node) in nodes.iter().enumerate() {
			let Some(obj) = node.as_object() else {
				return Err("node must be object".into());
			};
			if !board_json_visible_or_true(obj) {
				continue;
			}
			let Some(nid) = obj.get("id").and_then(|v| v.as_str()) else {
				return Err("node id missing".into());
			};
			let x_opt = obj.get("x").and_then(|v| v.as_f64());
			let y_opt = obj.get("y").and_then(|v| v.as_f64());
			let xy = match (x_opt, y_opt) {
				(Some(x), Some(y)) if x.is_finite() && y.is_finite() => Some((x, y)),
				_ => None,
			};
			id_to_index.insert(nid.to_string(), positions.len());
			visible_node_indices.push(raw_idx);
			optional_xy.push(xy);
			is_locked.push(locked_ids.contains(nid));
			positions.push(Vec2::ZERO);
			velocities.push(Vec2::ZERO);
			radii.push(node_repulsion_radius(node));
		}
		let n = positions.len();
		if n == 0 {
			return Ok(());
		}
		let mut sum = Vec2::ZERO;
		let mut finite_ct: u32 = 0;
		for xy in &optional_xy {
			if let Some((x, y)) = xy {
				sum += Vec2::new(*x, *y);
				finite_ct += 1;
			}
		}
		let anchor = if finite_ct > 0 {
			sum / (finite_ct as f64)
		} else {
			Vec2::new(opts.center_x.unwrap_or(0.0), opts.center_y.unwrap_or(0.0))
		};
		let mut seed_rng = opts.random_seed;
		for i in 0..n {
			positions[i] = if let Some((x, y)) = optional_xy[i] {
				Vec2::new(x, y)
			} else {
				let t = i as f64;
				let ang = t * 2.39996322972865332;
				let r = 10.0 + t.sqrt() * 22.0;
				let jx = (rand_unit_interval(&mut seed_rng) - 0.5) * 6.0;
				let jy = (rand_unit_interval(&mut seed_rng) - 0.5) * 6.0;
				anchor + Vec2::new(r * ang.cos() + jx, r * ang.sin() + jy)
			};
		}
		let pin: Vec<Option<Vec2>> = (0..n).map(|i| if is_locked[i] { Some(positions[i]) } else { None }).collect();
		let mut edge_pairs: Vec<(usize, usize)> = Vec::new();
		let mut seen: HashSet<(usize, usize)> = HashSet::new();
		for e in &edges {
			let Some(eo) = e.as_object() else {
				continue;
			};
			if !board_json_visible_or_true(eo) {
				continue;
			}
			let Some((src_h, tgt_h)) = fixture_edge_handle_ids_from_object(eo) else {
				continue;
			};
			let Some(a) = handle_to_node.get(src_h) else {
				continue;
			};
			let Some(b) = handle_to_node.get(tgt_h) else {
				continue;
			};
			if a == b {
				continue;
			}
			let Some(&ia) = id_to_index.get(a) else {
				continue;
			};
			let Some(&ib) = id_to_index.get(b) else {
				continue;
			};
			let lo = ia.min(ib);
			let hi = ia.max(ib);
			if seen.insert((lo, hi)) {
				edge_pairs.push((lo, hi));
			}
		}
		let mut cx = 0.0f64;
		let mut cy = 0.0f64;
		for p in &positions {
			cx += p.x;
			cy += p.y;
		}
		cx /= n as f64;
		cy /= n as f64;
		let gx = opts.center_x.unwrap_or(cx);
		let gy = opts.center_y.unwrap_or(cy);
		let k = opts.ideal_edge_length.max(1e-6);
		let mut rng = opts.random_seed;
		for i in 0..n {
			if pin[i].is_some() {
				continue;
			}
			if (positions[i].x - gx).abs() < 1e-6 && (positions[i].y - gy).abs() < 1e-6 {
				let jx = (rand_unit_interval(&mut rng) - 0.5) * 12.0;
				let jy = (rand_unit_interval(&mut rng) - 0.5) * 12.0;
				positions[i] += Vec2::new(jx, jy);
			}
		}
		let iters = opts.iterations.max(1);
		for iter in 0..iters {
			let cool = (1.0 - iter as f64 / iters as f64).max(0.08);
			let mut forces = vec![Vec2::ZERO; n];
			let theta = opts.barnes_hut_theta.clamp(0.2, 1.35);
			let pair_cap = opts.pairwise_repulsion_max_bodies.max(4) as usize;
			add_repulsion_forces(
				&mut forces,
				&positions,
				&radii,
				n,
				cool,
				opts.repulsion_strength,
				theta,
				pair_cap,
			);
			add_spring_forces(&mut forces, &positions, &edge_pairs, k, opts.spring_strength, cool);
			add_gravity_toward(&mut forces, &positions, gx, gy, opts.gravity, cool);
			zero_forces_on_pinned(&mut forces, &pin);
			integrate_velocity_and_positions(
				&mut positions,
				&mut velocities,
				&forces,
				opts.time_step,
				cool,
				opts.velocity_damping,
				opts.max_speed,
			);
			enforce_pin_constraints(&mut positions, &mut velocities, &pin);
		}
		for (idx, raw_idx) in visible_node_indices.into_iter().enumerate() {
			let Some(node) = nodes.get_mut(raw_idx) else {
				continue;
			};
			let Some(obj) = node.as_object_mut() else {
				continue;
			};
			obj.insert("x".into(), serde_json::json!(positions[idx].x));
			obj.insert("y".into(), serde_json::json!(positions[idx].y));
		}
		Ok(())
	}

	pub fn apply_force_graph_layout_to_fixture_v1_json(fixture_json: &str, options_json: &str) -> Result<String, String> {
		let mut fixture: Value = serde_json::from_str(fixture_json).map_err(|e| e.to_string())?;
		let opts: ForceGraphLayoutOptions = if options_json.trim().is_empty() {
			ForceGraphLayoutOptions::default()
		} else {
			serde_json::from_str(options_json).map_err(|e| e.to_string())?
		};
		apply_force_graph_layout_to_fixture_v1_value(&mut fixture, &opts)?;
		serde_json::to_string(&fixture).map_err(|e| e.to_string())
	}
}
// #endregion 🕸️ForceGraphLayout

// #region 🌳HierarchicalTreeLayout
mod hierarchical_tree {
	use serde::Deserialize;
	use serde_json::Value;
	use std::collections::{HashMap, HashSet};

	use super::{board_json_visible_or_true, fixture_edge_handle_ids_from_object};

	/// 🌳 Buchheim tidy-tree knobs: rank gap, sibling breadth, growth-axis string, optional world anchor for the laid subtree.
	#[derive(Clone, Debug, Deserialize)]
	#[serde(rename_all = "camelCase")]
	pub struct HierarchicalTreeLayoutOptions {
		#[serde(default = "default_layer_spacing")]
		pub layer_spacing: f64,
		#[serde(default = "default_sibling_gap")]
		pub sibling_gap: f64,
		#[serde(default = "default_direction")]
		pub direction: String,
		#[serde(default)]
		pub center_x: Option<f64>,
		#[serde(default)]
		pub center_y: Option<f64>,
		/// 📌 Node ids whose incoming fixture centers are kept; Buchheim still runs for placement of unlocked nodes.
		#[serde(default)]
		pub locked_node_ids: Vec<String>,
	}

	fn default_layer_spacing() -> f64 {
		120.0
	}
	fn default_sibling_gap() -> f64 {
		28.0
	}
	fn default_direction() -> String {
		"downwards".into()
	}

	impl Default for HierarchicalTreeLayoutOptions {
		fn default() -> Self {
			Self {
				layer_spacing: default_layer_spacing(),
				sibling_gap: default_sibling_gap(),
				direction: default_direction(),
				center_x: None,
				center_y: None,
				locked_node_ids: Vec::new(),
			}
		}
	}

	#[derive(Clone, Copy, Debug, PartialEq, Eq)]
	enum TreeDirection {
		Downwards,
		Upwards,
		Right,
		Left,
	}

	impl TreeDirection {
		fn parse(s: &str) -> Result<Self, String> {
			match s.trim().to_ascii_lowercase().as_str() {
				"down" | "downwards" => Ok(Self::Downwards),
				"up" | "upwards" => Ok(Self::Upwards),
				"right" => Ok(Self::Right),
				"left" => Ok(Self::Left),
				_ => Err(format!("unknown hierarchical tree direction: {s}")),
			}
		}
	}

	fn half_extent(node: &Value) -> f64 {
		let Some(obj) = node.as_object() else {
			return 24.0;
		};
		if obj.get("shape").and_then(|v| v.as_str()) == Some("rectangle") {
			let w = obj.get("width").and_then(|v| v.as_f64()).unwrap_or(40.0);
			let h = obj.get("height").and_then(|v| v.as_f64()).unwrap_or(40.0);
			return (w.max(h) * 0.5).max(8.0);
		}
		obj.get("radius").and_then(|v| v.as_f64()).filter(|r| r.is_finite() && *r > 0.0).unwrap_or(24.0)
	}

	const TREE_SUPER_ID: &str = "__tree_super__";

	/** @emoji 🌲 Buchheim et al. (GD 2002) tidy tree: O(n) Reingold–Tilford with even sibling spacing (after pymag-trees listing 12). */
	#[derive(Debug)]
	struct BuchheimNode {
		id: String,
		parent: Option<usize>,
		children: Vec<usize>,
		x: f64,
		y: f64,
		mod_: f64,
		thread: Option<usize>,
		ancestor: usize,
		change: f64,
		shift: f64,
		number: i32,
		synthetic: bool,
	}

	fn buchheim_left_brother(nodes: &[BuchheimNode], i: usize) -> Option<usize> {
		let p = nodes[i].parent?;
		let ch = &nodes[p].children;
		let pos = ch.iter().position(|&c| c == i)?;
		if pos == 0 {
			return None;
		}
		Some(ch[pos - 1])
	}

	fn buchheim_leftmost_sibling(nodes: &[BuchheimNode], i: usize) -> Option<usize> {
		let p = nodes[i].parent?;
		let ch = &nodes[p].children;
		if ch.first() == Some(&i) {
			return None;
		}
		ch.first().copied()
	}

	fn buchheim_next_right(nodes: &[BuchheimNode], i: usize) -> Option<usize> {
		if let Some(t) = nodes[i].thread {
			return Some(t);
		}
		nodes[i].children.last().copied()
	}

	fn buchheim_next_left(nodes: &[BuchheimNode], i: usize) -> Option<usize> {
		if let Some(t) = nodes[i].thread {
			return Some(t);
		}
		nodes[i].children.first().copied()
	}

	fn buchheim_ancestor(nodes: &[BuchheimNode], vil: usize, v: usize, default_ancestor: usize) -> usize {
		let par = nodes[v].parent.expect("buchheim ancestor needs parent");
		let pa = nodes[vil].ancestor;
		if nodes[par].children.iter().any(|&c| c == pa) {
			pa
		} else {
			default_ancestor
		}
	}

	fn buchheim_move_subtree(nodes: &mut [BuchheimNode], wl: usize, wr: usize, shift: f64) {
		let subtrees = (nodes[wr].number - nodes[wl].number) as f64;
		if subtrees <= 0.0 {
			return;
		}
		nodes[wr].change -= shift / subtrees;
		nodes[wr].shift += shift;
		nodes[wl].change += shift / subtrees;
		nodes[wr].x += shift;
		nodes[wr].mod_ += shift;
	}

	fn buchheim_execute_shifts(nodes: &mut [BuchheimNode], v: usize) {
		let mut shift = 0.0f64;
		let mut change = 0.0f64;
		for &w in nodes[v].children.iter().rev() {
			nodes[w].x += shift;
			nodes[w].mod_ += shift;
			change += nodes[w].change;
			shift += nodes[w].shift + change;
		}
	}

	fn buchheim_apportion(nodes: &mut [BuchheimNode], v: usize, default_ancestor: usize, distance: f64) -> usize {
		let mut default_ancestor = default_ancestor;
		let w = match buchheim_left_brother(nodes, v) {
			Some(w) => w,
			None => return default_ancestor,
		};
		let mut vir = v;
		let mut vor = v;
		let mut vil = w;
		let mut vol = match buchheim_leftmost_sibling(nodes, v) {
			Some(s) => s,
			None => return default_ancestor,
		};
		let mut sir = nodes[v].mod_;
		let mut sor = nodes[v].mod_;
		let mut sil = nodes[vil].mod_;
		let mut sol = nodes[vol].mod_;
		loop {
			let vil_r = buchheim_next_right(nodes, vil);
			let vir_l = buchheim_next_left(nodes, vir);
			if vil_r.is_none() || vir_l.is_none() {
				break;
			}
			vil = vil_r.unwrap();
			vir = vir_l.unwrap();
			let vol_l = buchheim_next_left(nodes, vol);
			let vor_r = buchheim_next_right(nodes, vor);
			if vol_l.is_none() || vor_r.is_none() {
				break;
			}
			vol = vol_l.unwrap();
			vor = vor_r.unwrap();
			nodes[vor].ancestor = v;
			let shift = (nodes[vil].x + sil) - (nodes[vir].x + sir) + distance;
			if shift > 0.0 {
				let a = buchheim_ancestor(nodes, vil, v, default_ancestor);
				buchheim_move_subtree(nodes, a, v, shift);
				sir += shift;
				sor += shift;
			}
			sil += nodes[vil].mod_;
			sir += nodes[vir].mod_;
			sol += nodes[vol].mod_;
			sor += nodes[vor].mod_;
		}
		if let Some(vil_r) = buchheim_next_right(nodes, vil) {
			if buchheim_next_right(nodes, vor).is_none() {
				nodes[vor].thread = Some(vil_r);
				nodes[vor].mod_ += sil - sor;
			}
		} else if buchheim_next_left(nodes, vir).is_some() && buchheim_next_left(nodes, vol).is_none() {
			if let Some(vir_l) = buchheim_next_left(nodes, vir) {
				nodes[vol].thread = Some(vir_l);
				nodes[vol].mod_ += sir - sol;
			}
			default_ancestor = v;
		}
		default_ancestor
	}

	fn buchheim_first_walk(nodes: &mut [BuchheimNode], v: usize, distance: f64) -> usize {
		if nodes[v].children.is_empty() {
			if buchheim_leftmost_sibling(nodes, v).is_some() {
				let lb = buchheim_left_brother(nodes, v).expect("leaf with leftmost sibling has left brother");
				nodes[v].x = nodes[lb].x + distance;
			} else {
				nodes[v].x = 0.0;
			}
			return v;
		}
		let mut default_ancestor = nodes[v].children[0];
		for &w in &nodes[v].children.clone() {
			buchheim_first_walk(nodes, w, distance);
			default_ancestor = buchheim_apportion(nodes, w, default_ancestor, distance);
		}
		buchheim_execute_shifts(nodes, v);
		let c0 = nodes[v].children[0];
		let c1 = *nodes[v].children.last().expect("internal node has children");
		let mid = (nodes[c0].x + nodes[c1].x) * 0.5;
		if let Some(w) = buchheim_left_brother(nodes, v) {
			nodes[v].x = nodes[w].x + distance;
			nodes[v].mod_ = nodes[v].x - mid;
		} else {
			nodes[v].x = mid;
		}
		v
	}

	fn buchheim_second_walk(nodes: &mut [BuchheimNode], v: usize, m: f64, depth: i32, min_x: f64) -> f64 {
		nodes[v].x += m;
		nodes[v].y = depth as f64;
		let mut min_x = min_x.min(nodes[v].x);
		for &w in &nodes[v].children.clone() {
			min_x = buchheim_second_walk(nodes, w, m + nodes[v].mod_, depth + 1, min_x);
		}
		min_x
	}

	fn buchheim_third_walk(nodes: &mut [BuchheimNode], v: usize, n: f64) {
		nodes[v].x += n;
		for &c in &nodes[v].children.clone() {
			buchheim_third_walk(nodes, c, n);
		}
	}

	fn run_buchheim_layout(
		id_to_node: &HashMap<String, Value>,
		roots: &[String],
		directed: &[(String, String)],
		depth: &HashMap<String, i32>,
	) -> Result<HashMap<String, (f64, f64)>, String> {
		let roots_set: HashSet<String> = roots.iter().cloned().collect();
		let mut incoming: HashMap<String, Vec<String>> = HashMap::new();
		for (u, v) in directed {
			incoming.entry(v.clone()).or_default().push(u.clone());
		}
		for v in incoming.values_mut() {
			v.sort();
			v.dedup();
		}
		let mut chosen_parent: HashMap<String, String> = HashMap::new();
		for id in id_to_node.keys() {
			if roots_set.contains(id) {
				continue;
			}
			let ps = incoming.get(id).cloned().unwrap_or_default();
			if ps.is_empty() {
				continue;
			}
			let best = ps
				.iter()
				.min_by_key(|p| {
					let dp = depth.get(*p).copied().unwrap_or(0);
					(dp, (*p).clone())
				})
				.expect("non-empty ps")
				.clone();
			chosen_parent.insert(id.clone(), best);
		}
		let mut ordered_ids: Vec<String> = id_to_node.keys().cloned().collect();
		ordered_ids.sort();
		let id_to_idx: HashMap<String, usize> = ordered_ids.iter().enumerate().map(|(i, s)| (s.clone(), i)).collect();
		let super_idx = ordered_ids.len();
		let mut nodes: Vec<BuchheimNode> = ordered_ids
			.iter()
			.map(|id| BuchheimNode {
				ancestor: 0,
				change: 0.0,
				children: vec![],
				id: id.clone(),
				mod_: 0.0,
				number: 0,
				parent: None,
				shift: 0.0,
				synthetic: false,
				thread: None,
				x: -1.0,
				y: 0.0,
			})
			.collect();
		nodes.push(BuchheimNode {
			ancestor: super_idx,
			change: 0.0,
			children: vec![],
			id: TREE_SUPER_ID.to_string(),
			mod_: 0.0,
			number: 0,
			parent: None,
			shift: 0.0,
			synthetic: true,
			thread: None,
			x: -1.0,
			y: 0.0,
		});
		for (i, oid) in ordered_ids.iter().enumerate() {
			let pidx = if roots_set.contains(oid) {
				super_idx
			} else {
				match chosen_parent.get(oid) {
					Some(p) => *id_to_idx.get(p).ok_or_else(|| format!("missing parent index for {p}"))?,
					None => super_idx,
				}
			};
			nodes[i].parent = Some(pidx);
		}
		for p in 0..=super_idx {
			nodes[p].children.clear();
		}
		for i in 0..super_idx {
			let pi = nodes[i].parent.ok_or_else(|| "tree node missing parent".to_string())?;
			nodes[pi].children.push(i);
		}
		for p in 0..=super_idx {
			let mut ch: Vec<usize> = nodes[p].children.clone();
			ch.sort_by_key(|&c| nodes[c].id.clone());
			nodes[p].children = ch;
		}
		for p in 0..=super_idx {
			if nodes[p].children.is_empty() {
				continue;
			}
			let ch = nodes[p].children.clone();
			for (k, &c) in ch.iter().enumerate() {
				nodes[c].number = (k + 1) as i32;
				nodes[c].ancestor = c;
			}
		}
		let dist = 1.0f64;
		buchheim_first_walk(&mut nodes, super_idx, dist);
		let min_x = buchheim_second_walk(&mut nodes, super_idx, 0.0, 0, f64::INFINITY);
		if min_x.is_finite() && min_x < 0.0 {
			buchheim_third_walk(&mut nodes, super_idx, -min_x);
		}
		let mut out: HashMap<String, (f64, f64)> = HashMap::new();
		for (i, n) in nodes.iter().enumerate() {
			if i == super_idx || n.synthetic {
				continue;
			}
			out.insert(n.id.clone(), (n.x, n.y));
		}
		Ok(out)
	}

	/// 🌳 Writes node centers: Buchheim tidy-tree on a spanning forest (min-depth parent tie-break id), synthetic multi-root; super-root not serialized.
	pub fn apply_hierarchical_tree_layout_to_fixture_v1_value(fixture: &mut Value, opts: &HierarchicalTreeLayoutOptions) -> Result<(), String> {
		let dir = TreeDirection::parse(&opts.direction)?;
		let Some(root) = fixture.as_object_mut() else {
			return Err("fixture root must be object".into());
		};
		if root.get("schema").and_then(|v| v.as_str()) != Some("elements.board.fixture/v1") {
			return Err("schema must be elements.board.fixture/v1".into());
		}
		let edges_json = root.get("edges").and_then(|v| v.as_array()).cloned().unwrap_or_default();
		let Some(nodes) = root.get_mut("nodes").and_then(|v| v.as_array_mut()) else {
			return Err("nodes array missing".into());
		};
		if nodes.is_empty() {
			return Ok(());
		}
		let mut handle_to_node: HashMap<String, String> = HashMap::new();
		let mut id_to_node: HashMap<String, Value> = HashMap::new();
		for node in nodes.iter() {
			let Some(obj) = node.as_object() else {
				continue;
			};
			if !board_json_visible_or_true(obj) {
				continue;
			}
			let Some(nid) = obj.get("id").and_then(|v| v.as_str()) else {
				continue;
			};
			id_to_node.insert(nid.to_string(), node.clone());
			let Some(handles) = obj.get("handles").and_then(|v| v.as_array()) else {
				continue;
			};
			for h in handles {
				let Some(ho) = h.as_object() else {
					continue;
				};
				if !board_json_visible_or_true(ho) {
					continue;
				}
				if let Some(hid) = ho.get("id").and_then(|v| v.as_str()) {
					handle_to_node.insert(hid.to_string(), nid.to_string());
				}
			}
		}
		if id_to_node.is_empty() {
			return Ok(());
		}
		let mut directed: Vec<(String, String)> = Vec::new();
		let mut seen_dir: HashSet<(String, String)> = HashSet::new();
		for e in &edges_json {
			let Some(eo) = e.as_object() else {
				continue;
			};
			if !board_json_visible_or_true(eo) {
				continue;
			}
			let Some((src_h, tgt_h)) = fixture_edge_handle_ids_from_object(eo) else {
				continue;
			};
			let Some(source_node_id) = handle_to_node.get(src_h) else {
				continue;
			};
			let Some(target_node_id) = handle_to_node.get(tgt_h) else {
				continue;
			};
			if source_node_id == target_node_id {
				continue;
			}
			if seen_dir.insert((source_node_id.clone(), target_node_id.clone())) {
				directed.push((source_node_id.clone(), target_node_id.clone()));
			}
		}
		let mut incoming_edge_count_by_node: HashMap<String, u32> = HashMap::new();
		for id in id_to_node.keys() {
			incoming_edge_count_by_node.insert(id.clone(), 0);
		}
		for (_source_nid, target_nid) in &directed {
			*incoming_edge_count_by_node.entry(target_nid.clone()).or_insert(0) += 1;
		}
		let mut roots: Vec<String> = Vec::new();
		for node in nodes.iter() {
			let Some(obj) = node.as_object() else {
				continue;
			};
			if !board_json_visible_or_true(obj) {
				continue;
			}
			if obj.get("root").and_then(|v| v.as_bool()) == Some(true) {
				if let Some(nid) = obj.get("id").and_then(|v| v.as_str()) {
					roots.push(nid.to_string());
				}
			}
		}
		roots.sort();
		roots.dedup();
		if roots.is_empty() {
			for (id, &d) in &incoming_edge_count_by_node {
				if d == 0 {
					roots.push(id.clone());
				}
			}
			roots.sort();
		}
		if roots.is_empty() {
			roots = id_to_node.keys().cloned().collect();
			roots.sort();
		}
		let mut depth: HashMap<String, i32> = HashMap::new();
		for r in &roots {
			depth.insert(r.clone(), 0);
		}
		let cap = directed.len().saturating_mul(3).saturating_add(nodes.len()).saturating_add(8);
		for _ in 0..cap {
			let mut changed = false;
			for (source_nid, target_nid) in &directed {
				let Some(&dp) = depth.get(source_nid) else {
					continue;
				};
				let nd = dp + 1;
				let cur = *depth.get(target_nid).unwrap_or(&-1);
				if nd > cur {
					depth.insert(target_nid.clone(), nd);
					changed = true;
				}
			}
			if !changed {
				break;
			}
		}
		let max_depth = depth.values().copied().max().unwrap_or(0);
		for id in id_to_node.keys() {
			depth.entry(id.clone()).or_insert(max_depth + 1);
		}
		let raw = run_buchheim_layout(&id_to_node, &roots, &directed, &depth)?;
		let mean_half: f64 = id_to_node.values().map(|nv| half_extent(nv)).sum::<f64>() / id_to_node.len().max(1) as f64;
		let along_scale = (opts.sibling_gap + 2.0 * mean_half).max(8.0);
		let mut pos: HashMap<String, (f64, f64)> = HashMap::new();
		for (id, (bx, by)) in raw {
			let along = bx * along_scale;
			let orth = by * opts.layer_spacing;
			let (lx, ly) = match dir {
				TreeDirection::Downwards => (along, orth),
				TreeDirection::Upwards => (along, -orth),
				TreeDirection::Right => (orth, along),
				TreeDirection::Left => (-orth, along),
			};
			pos.insert(id, (lx, ly));
		}
		let mut minx = f64::INFINITY;
		let mut maxx = f64::NEG_INFINITY;
		let mut miny = f64::INFINITY;
		let mut maxy = f64::NEG_INFINITY;
		for (id, (x, y)) in &pos {
			let h = half_extent(id_to_node.get(id).unwrap());
			minx = minx.min(x - h);
			maxx = maxx.max(x + h);
			miny = miny.min(y - h);
			maxy = maxy.max(y + h);
		}
		if !minx.is_finite() {
			minx = 0.0;
			maxx = 1.0;
			miny = 0.0;
			maxy = 1.0;
		}
		let cx = (minx + maxx) * 0.5;
		let cy = (miny + maxy) * 0.5;
		let gx = opts.center_x.unwrap_or(0.0);
		let gy = opts.center_y.unwrap_or(0.0);
		let dx = gx - cx;
		let dy = gy - cy;
		let locked_set: HashSet<String> = opts.locked_node_ids.iter().cloned().collect();
		let mut pinned_world: HashMap<String, (f64, f64)> = HashMap::new();
		if !locked_set.is_empty() {
			for node in nodes.iter() {
				let Some(obj) = node.as_object() else {
					continue;
				};
				if !board_json_visible_or_true(obj) {
					continue;
				}
				let Some(nid) = obj.get("id").and_then(|v| v.as_str()) else {
					continue;
				};
				if !locked_set.contains(nid) {
					continue;
				}
				if !id_to_node.contains_key(nid) {
					continue;
				}
				let px = obj.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0);
				let py = obj.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0);
				pinned_world.insert(nid.to_string(), (px, py));
			}
		}
		for (id, (x, y)) in pos {
			let (fx, fy) = if let Some(&(px, py)) = pinned_world.get(&id) {
				(px, py)
			} else {
				(x + dx, y + dy)
			};
			let idx = nodes
				.iter()
				.position(|n| n.get("id").and_then(|v| v.as_str()) == Some(id.as_str()))
				.ok_or_else(|| format!("node index {id}"))?;
			let Some(obj) = nodes[idx].as_object_mut() else {
				continue;
			};
			obj.insert("x".into(), serde_json::json!(fx));
			obj.insert("y".into(), serde_json::json!(fy));
		}
		Ok(())
	}
}
// #endregion 🌳HierarchicalTreeLayout

// #region 🔁RedrawLayout
mod redraw_layout {
	use serde::Deserialize;
	use serde_json::Value;
	use std::collections::HashMap;
	use crate::vello::kurbo::Point;

	use super::{board_json_visible_or_true, fixture_edge_handle_ids_from_object};
	use super::force_graph::{apply_force_graph_layout_to_fixture_v1_value, ForceGraphLayoutOptions};
	use super::hierarchical_tree::{apply_hierarchical_tree_layout_to_fixture_v1_value, HierarchicalTreeLayoutOptions};
	use super::vcompute::{circle_handle_angle_toward, distance_between, rectangle_handle_angle_toward};

	#[derive(Debug, Clone, Copy)]
	enum NodeShapeSnap {
		Circle { cx: f64, cy: f64 },
		Rect { cx: f64, cy: f64, w: f64, h: f64 },
	}

	impl NodeShapeSnap {
		fn center(self) -> Point {
			match self {
				NodeShapeSnap::Circle { cx, cy, .. } | NodeShapeSnap::Rect { cx, cy, .. } => Point::new(cx, cy),
			}
		}

		fn handle_angle_toward(self, toward: Point) -> Option<f64> {
			let c = self.center();
			if distance_between(c, toward) <= 1e-9 {
				return None;
			}
			Some(match self {
				NodeShapeSnap::Circle { cx, cy, .. } => circle_handle_angle_toward(Point::new(cx, cy), toward),
				NodeShapeSnap::Rect { cx, cy, w, h } => rectangle_handle_angle_toward(Point::new(cx, cy), w, h, toward),
			})
		}
	}

	fn parse_node_shape_snap(node: &serde_json::Map<String, Value>) -> Option<NodeShapeSnap> {
		let cx = node.get("x").and_then(|v| v.as_f64())?;
		let cy = node.get("y").and_then(|v| v.as_f64())?;
		if node.get("shape").and_then(|v| v.as_str()) == Some("rectangle") {
			let w = node.get("width").and_then(|v| v.as_f64())?;
			let h = node.get("height").and_then(|v| v.as_f64())?;
			Some(NodeShapeSnap::Rect { cx, cy, w, h })
		} else {
			node.get("radius").and_then(|v| v.as_f64())?;
			Some(NodeShapeSnap::Circle { cx, cy })
		}
	}

	/// 🔗 Sets each edge endpoint handle `angle` so the chord follows node centers; last edge wins on shared handles.
	pub fn apply_edge_handle_snap_to_fixture_v1_value(fixture: &mut Value) -> Result<(), String> {
		let Some(root) = fixture.as_object_mut() else {
			return Err("fixture root must be object".into());
		};
		if root.get("schema").and_then(|v| v.as_str()) != Some("elements.board.fixture/v1") {
			return Err("schema must be elements.board.fixture/v1".into());
		}
		let edges_json = root.get("edges").and_then(|v| v.as_array()).cloned().unwrap_or_default();
		let Some(nodes) = root.get_mut("nodes").and_then(|v| v.as_array_mut()) else {
			return Err("nodes array missing".into());
		};
		let mut shapes: Vec<Option<NodeShapeSnap>> = Vec::with_capacity(nodes.len());
		let mut handle_loc: HashMap<String, (usize, usize)> = HashMap::new();
		for (ni, node_val) in nodes.iter().enumerate() {
			let Some(no) = node_val.as_object() else {
				shapes.push(None);
				continue;
			};
			if !board_json_visible_or_true(no) {
				shapes.push(None);
				continue;
			}
			shapes.push(parse_node_shape_snap(no));
			let Some(hs) = no.get("handles").and_then(|v| v.as_array()) else {
				continue;
			};
			for (hi, h) in hs.iter().enumerate() {
				let Some(ho) = h.as_object() else {
					continue;
				};
				if !board_json_visible_or_true(ho) {
					continue;
				}
				if let Some(hid) = ho.get("id").and_then(|v| v.as_str()) {
					handle_loc.insert(hid.to_string(), (ni, hi));
				}
			}
		}
		let mut angle_by_loc: HashMap<(usize, usize), f64> = HashMap::new();
		for e in &edges_json {
			let Some(eo) = e.as_object() else {
				continue;
			};
			if !board_json_visible_or_true(eo) {
				continue;
			}
			let Some((src_h, tgt_h)) = fixture_edge_handle_ids_from_object(eo) else {
				continue;
			};
			let Some(&(ni_a, hi_a)) = handle_loc.get(src_h) else {
				continue;
			};
			let Some(&(ni_b, hi_b)) = handle_loc.get(tgt_h) else {
				continue;
			};
			let Some(sa) = shapes.get(ni_a).copied().flatten() else {
				continue;
			};
			let Some(sb) = shapes.get(ni_b).copied().flatten() else {
				continue;
			};
			if let Some(ang_a) = sa.handle_angle_toward(sb.center()) {
				angle_by_loc.insert((ni_a, hi_a), ang_a);
			}
			if let Some(ang_b) = sb.handle_angle_toward(sa.center()) {
				angle_by_loc.insert((ni_b, hi_b), ang_b);
			}
		}
		for ((ni, hi), ang) in angle_by_loc {
			let Some(node_val) = nodes.get_mut(ni) else {
				continue;
			};
			let Some(no) = node_val.as_object_mut() else {
				continue;
			};
			let Some(hs) = no.get_mut("handles").and_then(|v| v.as_array_mut()) else {
				continue;
			};
			let Some(h) = hs.get_mut(hi) else {
				continue;
			};
			let Some(ho) = h.as_object_mut() else {
				continue;
			};
			ho.insert("angle".into(), serde_json::json!(ang));
		}
		Ok(())
	}

	pub fn apply_edge_handle_snap_to_fixture_v1_json(fixture_json: &str) -> Result<String, String> {
		let mut fixture: Value = serde_json::from_str(fixture_json).map_err(|e| e.to_string())?;
		apply_edge_handle_snap_to_fixture_v1_value(&mut fixture)?;
		serde_json::to_string(&fixture).map_err(|e| e.to_string())
	}

	#[derive(Debug, Deserialize)]
	#[serde(rename_all = "camelCase")]
	struct RedrawFixtureOptions {
		mode: String,
		#[serde(default)]
		center_x: Option<f64>,
		#[serde(default)]
		center_y: Option<f64>,
		#[serde(default)]
		random_seed: Option<u64>,
		#[serde(default)]
		redraw_handles_after: bool,
		#[serde(default)]
		locked_node_ids: Vec<String>,
		#[serde(default)]
		force_graph: Option<ForceGraphLayoutOptions>,
		#[serde(default)]
		hierarchical_tree: Option<HierarchicalTreeLayoutOptions>,
	}

	pub fn apply_redraw_layout_to_fixture_v1_json(fixture_json: &str, options_json: &str) -> Result<String, String> {
		let opts: RedrawFixtureOptions = serde_json::from_str(options_json).map_err(|e| e.to_string())?;
		let mut fixture: Value = serde_json::from_str(fixture_json).map_err(|e| e.to_string())?;
		match opts.mode.as_str() {
			"force-graph" => {
				let mut fo = opts.force_graph.clone().unwrap_or_default();
				if opts.center_x.is_some() {
					fo.center_x = opts.center_x;
				}
				if opts.center_y.is_some() {
					fo.center_y = opts.center_y;
				}
				if let Some(s) = opts.random_seed {
					fo.random_seed = s;
				}
				for id in &opts.locked_node_ids {
					if !fo.locked_node_ids.contains(id) {
						fo.locked_node_ids.push(id.clone());
					}
				}
				apply_force_graph_layout_to_fixture_v1_value(&mut fixture, &fo)?;
			}
			"hierarchical-tree" => {
				let mut hierarchical_opts = opts.hierarchical_tree.clone().unwrap_or_default();
				if opts.center_x.is_some() {
					hierarchical_opts.center_x = opts.center_x;
				}
				if opts.center_y.is_some() {
					hierarchical_opts.center_y = opts.center_y;
				}
				for id in &opts.locked_node_ids {
					if !hierarchical_opts.locked_node_ids.contains(id) {
						hierarchical_opts.locked_node_ids.push(id.clone());
					}
				}
				apply_hierarchical_tree_layout_to_fixture_v1_value(&mut fixture, &hierarchical_opts)?;
			}
			other => return Err(format!("unknown redraw mode: {other}")),
		}
		if opts.redraw_handles_after {
			apply_edge_handle_snap_to_fixture_v1_value(&mut fixture)?;
		}
		serde_json::to_string(&fixture).map_err(|e| e.to_string())
	}
}
// #endregion 🔁RedrawLayout

pub use force_graph::{apply_force_graph_layout_to_fixture_v1_json, apply_force_graph_layout_to_fixture_v1_value, ForceGraphLayoutOptions};
pub use redraw_layout::{apply_edge_handle_snap_to_fixture_v1_json, apply_redraw_layout_to_fixture_v1_json};

mod elements_board_palette {
	use crate::vello::peniko::Color;
	include!(concat!(env!("OUT_DIR"), "/elements_styling_board.rs"));
}

mod board_metabolism_icons {
	include!(concat!(env!("OUT_DIR"), "/board_metabolism_icon_match.rs"));
}

fn resolve_node_icon_svg_from_encoding(encoded: &str) -> Option<String> {
	let t = encoded.trim();
	if t.is_empty() {
		return None;
	}
	if let Some(s) = board_metabolism_icons::board_metabolism_icon_svg(t) {
		return Some(s.to_string());
	}
	let lower = t.to_ascii_lowercase();
	if lower.starts_with("<?xml") || lower.contains("<svg") {
		return Some(t.to_string());
	}
	None
}

mod board_icon_codec {
	use base64::Engine as _;
	use std::path::PathBuf;
	use std::sync::{Arc, OnceLock};
	use typst::foundations::{Bytes, Datetime};
	use typst::layout::{Abs, PagedDocument};
	use typst::text::Font;
	use typst::Library;
	use typst::LibraryExt;
	use typst::World;
	use typst::syntax::{FileId, Source, VirtualPath};
	use typst::utils::LazyHash;

	#[derive(Debug)]
	pub enum BoardResolvedIcon {
		None,
		SvgThemed(String),
		SvgPlain(String),
		RasterRgba8 {
			rgba: Arc<[u8]>,
			w: u32,
			h: u32,
		},
	}

	struct RgbaImage {
		data: Arc<[u8]>,
		w: u32,
		h: u32,
	}

	fn decode_raster_icon_bytes(t: &str) -> Option<RgbaImage> {
		let s = t.trim().strip_prefix("image:").unwrap_or(t.trim()).trim();
		let rest = s
			.strip_prefix("data:image/png;base64,")
			.or_else(|| s.strip_prefix("data:image/jpeg;base64,"))
			.or_else(|| s.strip_prefix("data:image/jpg;base64,"))?;
		let raw = base64::engine::general_purpose::STANDARD.decode(rest.trim()).ok()?;
		let img = image::load_from_memory(&raw).ok()?;
		let rgba = img.to_rgba8();
		let (w, h) = rgba.dimensions();
		if w == 0 || h == 0 {
			return None;
		}
		Some(RgbaImage {
			data: Arc::from(rgba.into_raw().into_boxed_slice()),
			w,
			h,
		})
	}

	fn typst_asset_font_list() -> Vec<Font> {
		let mut out = Vec::new();
		for bytes in typst_assets::fonts() {
			let blob = Bytes::new(bytes);
			let mut idx = 0u32;
			loop {
				if let Some(f) = Font::new(blob.clone(), idx) {
					out.push(f);
					idx = idx.saturating_add(1);
				} else {
					break;
				}
			}
		}
		out
	}

	fn typst_asset_font_list_plus_noto_color_emoji() -> Vec<Font> {
		let mut out = typst_asset_font_list();
		let emoji_blob = Bytes::new(super::board_icon_assets::NOTO_COLOR_EMOJI_SUBSET_TTF);
		let mut idx = 0u32;
		loop {
			if let Some(f) = Font::new(emoji_blob.clone(), idx) {
				out.push(f);
				idx = idx.saturating_add(1);
			} else {
				break;
			}
		}
		out
	}

	fn board_typst_compile_markup_to_svg(
		markup: &str,
		fonts: &'static [Font],
		book: &'static LazyHash<typst::text::FontBook>,
	) -> Option<String> {
		static LIB: OnceLock<LazyHash<Library>> = OnceLock::new();
		static MAIN: OnceLock<FileId> = OnceLock::new();
		let library = LIB.get_or_init(|| LazyHash::new(Library::default()));
		let main = *MAIN.get_or_init(|| FileId::new(None, VirtualPath::new("/board.typ")));
		let source = Source::new(main, markup.to_string());
		struct BoardTypstWorld<'a> {
			library: &'static LazyHash<Library>,
			book: &'static LazyHash<typst::text::FontBook>,
			main: FileId,
			source: Source,
			fonts: &'a [Font],
		}
		impl World for BoardTypstWorld<'_> {
			fn library(&self) -> &LazyHash<Library> {
				self.library
			}
			fn book(&self) -> &LazyHash<typst::text::FontBook> {
				self.book
			}
			fn main(&self) -> FileId {
				self.main
			}
			fn source(&self, id: FileId) -> typst::diag::FileResult<Source> {
				if id == self.main {
					Ok(self.source.clone())
				} else {
					Err(typst::diag::FileError::NotFound(PathBuf::from("board.typ")))
				}
			}
			fn file(&self, _id: FileId) -> typst::diag::FileResult<Bytes> {
				Err(typst::diag::FileError::NotFound(PathBuf::from("board.bin")))
			}
			fn font(&self, index: usize) -> Option<Font> {
				self.fonts.get(index).cloned()
			}
			fn today(&self, _offset: Option<i64>) -> Option<Datetime> {
				None
			}
		}
		let w = BoardTypstWorld {
			library,
			book,
			main,
			source,
			fonts,
		};
		let warned = typst::compile::<PagedDocument>(&w);
		let doc = warned.output.ok()?;
		if doc.pages.is_empty() {
			return None;
		}
		Some(typst_svg::svg_merged(&doc, Abs::pt(3.0)))
	}

	static TYPST_ASSET_FONTS: OnceLock<Vec<Font>> = OnceLock::new();
	static TYPST_ASSET_BOOK: OnceLock<LazyHash<typst::text::FontBook>> = OnceLock::new();
	// Noto Color Emoji (COLR) in the same `FontBook` as math broke `typst:` icon compiles; keep a second pool for `emoji:` only.
	static TYPST_ICON_EMOJI_FONTS: OnceLock<Vec<Font>> = OnceLock::new();
	static TYPST_ICON_EMOJI_BOOK: OnceLock<LazyHash<typst::text::FontBook>> = OnceLock::new();

	pub fn board_typst_markup_to_svg(markup: &str) -> Option<String> {
		let fonts = TYPST_ASSET_FONTS.get_or_init(typst_asset_font_list);
		let book = TYPST_ASSET_BOOK.get_or_init(|| LazyHash::new(typst::text::FontBook::from_fonts(fonts.iter())));
		board_typst_compile_markup_to_svg(markup, fonts.as_slice(), book)
	}

	fn board_typst_markup_to_svg_for_icon_emoji(markup: &str) -> Option<String> {
		let fonts = TYPST_ICON_EMOJI_FONTS.get_or_init(typst_asset_font_list_plus_noto_color_emoji);
		let book =
			TYPST_ICON_EMOJI_BOOK.get_or_init(|| LazyHash::new(typst::text::FontBook::from_fonts(fonts.iter())));
		board_typst_compile_markup_to_svg(markup, fonts.as_slice(), book)
	}

	pub fn board_resolve_icon_kind(encoded: &str) -> BoardResolvedIcon {
		let t = encoded.trim();
		if t.is_empty() {
			return BoardResolvedIcon::None;
		}
		if let Some(src) = t.strip_prefix("typst:") {
			let src = src.trim();
			if src.is_empty() {
				return BoardResolvedIcon::None;
			}
			let wrapped = format!(
				"#set page(width: 96pt, height: 96pt, margin: 3pt, fill: none)\n{src}"
			);
			return match board_typst_markup_to_svg(&wrapped) {
				Some(s) => BoardResolvedIcon::SvgPlain(s),
				None => BoardResolvedIcon::None,
			};
		}
		if let Some(em) = t.strip_prefix("emoji:") {
			let em = em.trim();
			if em.is_empty() {
				return BoardResolvedIcon::None;
			}
			let wrapped = format!(
				"#set page(width: 88pt, height: 88pt, margin: 2pt, fill: none)\n#set align(center + horizon)\n#set text(size: 44pt, font: \"Noto Color Emoji\")\n{em}"
			);
			return match board_typst_markup_to_svg_for_icon_emoji(&wrapped) {
				Some(s) => BoardResolvedIcon::SvgPlain(s),
				None => BoardResolvedIcon::None,
			};
		}
		if let Some(img) = decode_raster_icon_bytes(t) {
			return BoardResolvedIcon::RasterRgba8 {
				rgba: img.data,
				w: img.w,
				h: img.h,
			};
		}
		if let Some(svg) = super::resolve_node_icon_svg_from_encoding(t) {
			if super::board_metabolism_icons::board_metabolism_icon_svg(t).is_some() {
				return BoardResolvedIcon::SvgThemed(svg);
			}
			return BoardResolvedIcon::SvgPlain(svg);
		}
		BoardResolvedIcon::None
	}
}

/// @emoji 🖼️ Parses SVG via `usvg` into Vello paths; maps near-black / near-white fills and strokes to caller `fg` / `bg` (multiply with paint opacity). Each path uses `path.abs_transform()` only (usvg already stores document-absolute transforms; do not compose parent × abs when walking groups).
mod svg_icon_vello09 {
	use std::sync::{Arc, OnceLock};

	use crate::vello::kurbo::{Affine, BezPath, Point, Stroke};
	use crate::vello::peniko::{Color, Fill};
	use crate::vello::Scene;
	use crate::usvg;

	// #region 🔖BoardIconUsvgParseOptions

	static BOARD_ICON_USVG_OPTIONS: OnceLock<usvg::Options<'static>> = OnceLock::new();

	/// @emoji 🔤 Shared `usvg` parse options with bundled Noto Color Emoji so `<text>` in Typst `emoji:` SVG matches the Typst font book; avoids system fallback glyphs.
	pub fn usvg_options_board_icons() -> &'static usvg::Options<'static> {
		BOARD_ICON_USVG_OPTIONS.get_or_init(|| {
			let mut db = fontdb::Database::new();
			db.load_font_data(super::board_icon_assets::NOTO_COLOR_EMOJI_SUBSET_TTF.to_vec());
			let mut o = usvg::Options::default();
			o.fontdb = Arc::new(db);
			o.font_family = "Noto Color Emoji".into();
			o
		})
	}

	// #endregion 🔖BoardIconUsvgParseOptions

	fn to_affine(ts: &usvg::Transform) -> Affine {
		let usvg::Transform { sx, kx, ky, sy, tx, ty } = *ts;
		Affine::new([sx, ky, kx, sy, tx, ty].map(f64::from))
	}

	fn to_bez_path(path: &usvg::Path) -> BezPath {
		let mut local_path = BezPath::new();
		let mut just_closed = false;
		let mut most_recent_initial = (0_f64, 0_f64);
		for elt in path.data().segments() {
			match elt {
				usvg::tiny_skia_path::PathSegment::MoveTo(p) => {
					if std::mem::take(&mut just_closed) {
						local_path.move_to(most_recent_initial);
					}
					most_recent_initial = (p.x.into(), p.y.into());
					local_path.move_to(most_recent_initial);
				}
				usvg::tiny_skia_path::PathSegment::LineTo(p) => {
					if std::mem::take(&mut just_closed) {
						local_path.move_to(most_recent_initial);
					}
					local_path.line_to(Point::new(p.x as f64, p.y as f64));
				}
				usvg::tiny_skia_path::PathSegment::QuadTo(p1, p2) => {
					if std::mem::take(&mut just_closed) {
						local_path.move_to(most_recent_initial);
					}
					local_path.quad_to(
						Point::new(p1.x as f64, p1.y as f64),
						Point::new(p2.x as f64, p2.y as f64),
					);
				}
				usvg::tiny_skia_path::PathSegment::CubicTo(p1, p2, p3) => {
					if std::mem::take(&mut just_closed) {
						local_path.move_to(most_recent_initial);
					}
					local_path.curve_to(
						Point::new(p1.x as f64, p1.y as f64),
						Point::new(p2.x as f64, p2.y as f64),
						Point::new(p3.x as f64, p3.y as f64),
					);
				}
				usvg::tiny_skia_path::PathSegment::Close => {
					just_closed = true;
					local_path.close_path();
				}
			}
		}
		local_path
	}

	fn map_solid_icon_paint(paint: &usvg::Paint, opacity: usvg::Opacity, fg: Color, bg: Color) -> Option<Color> {
		let usvg::Paint::Color(c) = paint else {
			return None;
		};
		let a = opacity.get();
		if c.red < 22 && c.green < 22 && c.blue < 22 {
			return Some(fg.multiply_alpha(a));
		}
		if c.red > 233 && c.green > 233 && c.blue > 233 {
			return Some(bg.multiply_alpha(a));
		}
		Some(Color::from_rgba8(c.red, c.green, c.blue, opacity.to_u8()))
	}

	fn render_group(scene: &mut Scene, group: &usvg::Group, fg: Color, bg: Color) {
		for node in group.children() {
			match node {
				usvg::Node::Group(g) => render_group(scene, g, fg, bg),
				usvg::Node::Path(path) => {
					if !path.is_visible() {
						continue;
					}
					let transform = to_affine(&path.abs_transform());
					let local_path = to_bez_path(path);
					if let Some(fill) = path.fill() {
						if let Some(color) = map_solid_icon_paint(fill.paint(), fill.opacity(), fg, bg) {
							scene.fill(
								match fill.rule() {
									usvg::FillRule::NonZero => Fill::NonZero,
									usvg::FillRule::EvenOdd => Fill::EvenOdd,
								},
								transform,
								color,
								None,
								&local_path,
							);
						}
					}
					if let Some(stroke) = path.stroke() {
						if let Some(color) = map_solid_icon_paint(stroke.paint(), stroke.opacity(), fg, bg) {
							let conv = Stroke::new(f64::from(stroke.width().get()));
							scene.stroke(&conv, transform, color, None, &local_path);
						}
					}
				}
				_ => {}
			}
		}
	}

	fn icon_rect_xywh(r: usvg::Rect) -> Option<(f64, f64, f64, f64)> {
		let w = f64::from(r.width());
		let h = f64::from(r.height());
		if !(w > 1e-6 && h > 1e-6 && w.is_finite() && h.is_finite()) {
			return None;
		}
		Some((f64::from(r.x()), f64::from(r.y()), w, h))
	}

	fn icon_rect_nonzero(r: usvg::tiny_skia_path::NonZeroRect) -> (f64, f64, f64, f64) {
		(
			f64::from(r.x()),
			f64::from(r.y()),
			f64::from(r.width()),
			f64::from(r.height()),
		)
	}

	fn icon_union_xywh(a: (f64, f64, f64, f64), b: (f64, f64, f64, f64)) -> (f64, f64, f64, f64) {
		let ax1 = a.0 + a.2;
		let ay1 = a.1 + a.3;
		let bx1 = b.0 + b.2;
		let by1 = b.1 + b.3;
		let x0 = a.0.min(b.0);
		let y0 = a.1.min(b.1);
		let x1 = ax1.max(bx1);
		let y1 = ay1.max(by1);
		(x0, y0, x1 - x0, y1 - y0)
	}

	fn icon_union_rects_into(acc: &mut Option<(f64, f64, f64, f64)>, r: usvg::Rect) {
		if let Some(xy) = icon_rect_xywh(r) {
			*acc = Some(match acc.take() {
				None => xy,
				Some(a) => icon_union_xywh(a, xy),
			});
		}
	}

	fn icon_visit_node_bounds(node: &usvg::Node, acc: &mut Option<(f64, f64, f64, f64)>) {
		match node {
			usvg::Node::Group(g) => {
				for c in g.children() {
					icon_visit_node_bounds(c, acc);
				}
			}
			usvg::Node::Path(p) => {
				if !p.is_visible() {
					return;
				}
				icon_union_rects_into(acc, p.abs_bounding_box());
				icon_union_rects_into(acc, p.abs_stroke_bounding_box());
			}
			usvg::Node::Image(img) => {
				if !img.is_visible() {
					return;
				}
				icon_union_rects_into(acc, img.abs_bounding_box());
			}
			usvg::Node::Text(t) => {
				icon_union_rects_into(acc, t.abs_bounding_box());
				icon_union_rects_into(acc, t.abs_stroke_bounding_box());
			}
		}
	}

	/// @emoji 📐 Union of visible paint bounds (paths, raster images, text) in absolute SVG space for uniform scale-and-center fits.
	pub fn svg_icon_content_bounds(tree: &usvg::Tree) -> (f64, f64, f64, f64) {
		let mut acc = None::<(f64, f64, f64, f64)>;
		for c in tree.root().children() {
			icon_visit_node_bounds(c, &mut acc);
		}
		if let Some(u) = acc {
			let (_, _, bw, bh) = u;
			if bw > 1e-6 && bh > 1e-6 {
				return u;
			}
		}
		let root = tree.root();
		let mut u = icon_rect_nonzero(root.abs_layer_bounding_box());
		if let Some(r) = icon_rect_xywh(root.abs_stroke_bounding_box()) {
			u = icon_union_xywh(u, r);
		}
		if let Some(r) = icon_rect_xywh(root.abs_bounding_box()) {
			u = icon_union_xywh(u, r);
		}
		let (_, _, bw, bh) = u;
		if bw > 1e-6 && bh > 1e-6 {
			return u;
		}
		let w = f64::from(tree.size().width());
		let h = f64::from(tree.size().height());
		(0.0, 0.0, w.max(1.0), h.max(1.0))
	}

	pub fn render_svg_tree_themed(scene: &mut Scene, tree: &usvg::Tree, fg: Color, bg: Color) {
		render_group(scene, tree.root(), fg, bg);
	}

	#[allow(dead_code)]
	pub fn append_svg_str_themed(scene: &mut Scene, svg: &str, fg: Color, bg: Color) -> Result<(), String> {
		let tree = usvg::Tree::from_str(svg, usvg_options_board_icons()).map_err(|e| e.to_string())?;
		render_svg_tree_themed(scene, &tree, fg, bg);
		Ok(())
	}

	#[allow(dead_code)]
	pub fn append_svg_str(scene: &mut Scene, svg: &str) -> Result<(), String> {
		append_svg_str_themed(scene, svg, Color::BLACK, Color::WHITE)
	}
}

mod board_host {
	use super::board_json_visible_option;
	use super::elements_board_palette as board_palette;
	use super::scene_json::*;
	use serde_json::json;
	use std::collections::{BTreeMap, BTreeSet};
	use crate::vello::kurbo::{Affine, Circle, CubicBez, Point, Rect, Stroke, Vec2};
	use crate::vello::peniko::{Blob, Color, Fill, ImageAlphaType, ImageBrush, ImageData, ImageFormat};
	use crate::vello::Scene;
	use crate::usvg;

	use super::geom_sel::{
		cubic_bezier_axis_bounds, cubic_bezier_point, inflate_world_box, point_in_polygon, polygon_contains_world_box,
		polygon_intersects_world_box, segment_intersects_polygon, segment_intersects_world_box, world_box_contains_box,
		world_box_contains_point, world_box_from_points, world_boxes_overlap, WorldBox,
	};
	use super::vcompute::{
		compute_edge_bezier_points, distance_between, distance_point_to_cubic_bezier, handle_position_on_circle,
		handle_position_on_rectangle,
	};

	use std::cell::RefCell;
	use std::collections::HashMap;
	use std::hash::{Hash, Hasher};
	use std::sync::Arc;

	const LOD_MINIMAP_MAX_ZOOM_DEFAULT: f64 = 0.15;
	const LOD_OVERVIEW_MAX_ZOOM_DEFAULT: f64 = 0.35;
	const LOD_COMPACT_MAX_ZOOM_DEFAULT: f64 = 0.55;
	const LOD_NORMAL_MAX_ZOOM_DEFAULT: f64 = 1.25;
	const LOD_DETAIL_MAX_ZOOM_DEFAULT: f64 = 2.5;
	const GRID_WORLD_LARGE: f64 = 10.0;
	const GRID_WORLD_MEDIUM: f64 = 2.5;
	const GRID_WORLD_SMALL: f64 = 0.5;
	const GRID_WORLD_MICRO: f64 = 0.1;
	const GRID_FACTOR_DEFAULT: f64 = 10.0;
	const WORLD_CLIP_TILE_WORLD: f64 = 256.0;
	const MAX_WORLD_CLIP_TILES: u32 = 768;
	const EDGE_HIT_TOLERANCE_PX: f64 = 8.0;
	const HANDLE_HIT_TOLERANCE_PX: f64 = 10.0;
	const INDIRECT_HANDLE_MARKER_NODE_SCALE: f64 = 0.8;
	/// Radial offset from node rim to indirect-handle center, as a fraction of node half-extent (circle radius or half the shorter rectangle side).
	const INDIRECT_HANDLE_RING_GAP_NODE_SCALE: f64 = 0.7;
	const LINK_DRAG_MIN_DISTANCE_PX: f64 = 5.0;
	const LINK_HANDLE_SNAP_EXTRA_PX: f64 = 22.0;
	const LINK_COMMIT_SNAP_TIGHT_PX: f64 = 2.0;
	const SELECTION_LASSO_MIN_POINT_DISTANCE_PX: f64 = 3.0;
	const SELECTION_CLICK_MAX_DISTANCE_PX: f64 = 4.0;
	const BOUNDED_DRAG_HIT_PAD_PX: f64 = 8.0;
	pub const BOARD_CAMERA_ZOOM_MIN: f64 = 0.05;
	pub const BOARD_CAMERA_ZOOM_MAX: f64 = 32.0;
	const BOARD_DEFAULT_WIRE_KIND_ID: &str = "board.wire.link";

	#[derive(Clone, Copy, Debug, PartialEq, Eq)]
	enum BoardDrawLod {
		Minimap,
		Overview,
		Compact,
		Normal,
		Detail,
		Micro,
	}

	#[derive(Clone)]
	enum CachedIconBody {
		Vector(Scene),
		Raster(Arc<ImageData>),
	}

	#[derive(Clone)]
	struct CachedIconPaint {
		bx: f64,
		by: f64,
		bw: f64,
		bh: f64,
		body: CachedIconBody,
	}

	#[derive(Clone, Copy, Debug, PartialEq, Eq)]
	pub enum NodeShape {
		Circle,
		Rectangle,
	}

	#[derive(Clone, Copy, Debug, PartialEq, Eq)]
	enum BoardElementStyleKind {
		Original,
		Neutral,
		Hovered,
		Selected,
		Highlighted,
		Disabled,
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
		pub scale: f64,
		pub draggable: bool,
		pub selected: bool,
		pub visible: bool,
		pub root: bool,
		pub style: Option<String>,
		pub text: Option<String>,
		/// @emoji 🏷️ Runtime host encoding: catalog id from the baked icon table or inline SVG (`<?xml` / `<svg` …) parsed at detail LOD.
		pub icon_kind: Option<String>,
		pub node_kind: String,
	}

	#[derive(Clone, Debug)]
	pub struct HandleKindDef {
		pub name: String,
		pub color: Color,
		pub default_wire_kind: Option<String>,
		pub scale: f64,
	}

	#[derive(Clone, Debug)]
	pub struct WireKindDef {
		pub name: String,
		pub default_edge_kind: Option<String>,
	}

	#[derive(Clone, Debug)]
	pub struct NodeKindDef {
		pub name: String,
		pub scale: f64,
	}

	#[derive(Clone, Debug)]
	pub struct EdgeKindDef {
		pub name: String,
	}

	#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
	pub enum CompatSpecificity {
		General = 0,
		Node = 1,
		Edge = 2,
		Handle = 3,
		Wire = 4,
	}

	#[derive(Clone, Debug)]
	pub struct LinkCompatRule {
		pub source: String,
		pub target: String,
		pub bidirectional: bool,
		pub important: bool,
		pub specificity: CompatSpecificity,
	}

	#[derive(Clone, Debug)]
	pub struct HandleData {
		pub id: String,
		pub node_id: String,
		pub angle: f64,
		pub radius: f64,
		pub scale: f64,
		pub selected: bool,
		pub visible: bool,
		pub style: Option<String>,
		pub handle_kind: String,
		/// Parsed from descriptor `color` when set (overrides catalog fill).
		pub color_fill: Option<Color>,
		/// @emoji 🏷️ Runtime host encoding: `typst:`, `emoji:`, `image:data:…`, catalog id, or inline SVG for detail LOD.
		pub icon_kind: Option<String>,
	}

	#[derive(Clone, Debug)]
	pub struct EdgeData {
		pub id: String,
		pub source: String,
		pub target: String,
		pub selected: bool,
		pub visible: bool,
		pub style: Option<String>,
		pub edge_kind: String,
	}

	#[derive(Clone, Debug)]
	pub struct WireData {
		pub id: String,
		pub source: String,
		pub target: Option<String>,
		pub end_x: Option<f64>,
		pub end_y: Option<f64>,
		pub selected: bool,
		pub visible: bool,
		pub style: Option<String>,
		pub wire_kind: String,
	}

	#[derive(Clone, Debug)]
	pub struct Camera {
		pub x: f64,
		pub y: f64,
		pub zoom: f64,
	}

	#[derive(Clone, Debug, PartialEq, Eq)]
	pub struct SelectionOptions {
		pub method: String,
		pub mode: String,
		pub select_nodes: bool,
		pub select_edges: bool,
		pub select_handles: bool,
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
			/// @emoji 🧲 Preview/commit pair while an unconnected node overlaps a compatible target (`proximityConnect` on pointer-up).
			proximity_pair: Option<(String, String)>,
		},
		/// @emoji 🖱️ Background down before drag threshold; click-up deselects without preselect or exit chrome.
		SelectionPending {
			initial_ids: BTreeSet<String>,
			start: Point,
			start_screen: Point,
		},
		Selection {
			initial_ids: BTreeSet<String>,
			points: Vec<Point>,
			screen_points: Vec<Point>,
			start: Point,
			start_screen: Point,
		},
		LinkAtSourceHandle {
			source_id: String,
			start_screen: Point,
		},
		LinkDragSnap {
			source_id: String,
			target_id: Option<String>,
			end_world: Point,
		},
		LinkTargetNode {
			source_id: String,
			target_node_id: String,
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
		pub edge_stroke_hovered: Color,
		pub edge_stroke_selected: Color,
		pub edge_stroke_selection_exit: Color,
		pub edge_stroke_disabled: Color,
		pub node_fill: Color,
		pub node_stroke: Color,
		pub node_fill_hovered: Color,
		pub node_stroke_hovered: Color,
		pub node_fill_selected: Color,
		pub node_stroke_selected: Color,
		pub node_fill_selection_exit: Color,
		pub node_stroke_selection_exit: Color,
		pub node_fill_disabled: Color,
		pub node_stroke_disabled: Color,
		pub indirect_handle_fill: Color,
		pub indirect_handle_stroke: Color,
		pub handle_fill: Color,
		pub handle_stroke: Color,
		pub handle_fill_hovered: Color,
		pub handle_stroke_hovered: Color,
		pub handle_fill_selected: Color,
		pub handle_stroke_selected: Color,
		pub handle_fill_selection_exit: Color,
		pub handle_stroke_selection_exit: Color,
		pub handle_fill_disabled: Color,
		pub handle_stroke_disabled: Color,
		pub wire_stroke: Color,
		pub wire_stroke_hovered: Color,
		pub wire_stroke_selected: Color,
		pub wire_stroke_highlighted: Color,
		pub wire_stroke_disabled: Color,
		pub selection_preview_fill: Color,
		pub selection_preview_stroke: Color,
	}

	impl Default for VelloThemePalette {
		fn default() -> Self {
			Self {
				raster_clear: board_palette::RASTER_CLEAR,
				grid_minor_stroke: board_palette::GRID_MINOR_STROKE,
				edge_stroke: board_palette::EDGE_STROKE,
				edge_stroke_hovered: board_palette::NODE_STROKE,
				edge_stroke_selected: board_palette::EDGE_STROKE_SELECTED,
				edge_stroke_selection_exit: board_palette::INDIRECT_HANDLE_STROKE,
				edge_stroke_disabled: board_palette::GRID_MINOR_STROKE,
				node_fill: board_palette::NODE_FILL,
				node_stroke: board_palette::NODE_STROKE,
				node_fill_hovered: board_palette::NODE_FILL,
				node_stroke_hovered: board_palette::NODE_STROKE,
				node_fill_selected: board_palette::NODE_FILL_SELECTED,
				node_stroke_selected: board_palette::NODE_STROKE_SELECTED,
				node_fill_selection_exit: board_palette::INDIRECT_HANDLE_FILL,
				node_stroke_selection_exit: board_palette::INDIRECT_HANDLE_STROKE,
				node_fill_disabled: board_palette::NODE_FILL,
				node_stroke_disabled: board_palette::GRID_MINOR_STROKE,
				indirect_handle_fill: board_palette::INDIRECT_HANDLE_FILL,
				indirect_handle_stroke: board_palette::INDIRECT_HANDLE_STROKE,
				handle_fill: board_palette::HANDLE_FILL,
				handle_stroke: board_palette::HANDLE_STROKE,
				handle_fill_hovered: board_palette::HANDLE_FILL,
				handle_stroke_hovered: board_palette::HANDLE_STROKE,
				handle_fill_selected: board_palette::HANDLE_FILL_SELECTED,
				handle_stroke_selected: board_palette::HANDLE_STROKE_SELECTED,
				handle_fill_selection_exit: board_palette::INDIRECT_HANDLE_FILL,
				handle_stroke_selection_exit: board_palette::INDIRECT_HANDLE_STROKE,
				handle_fill_disabled: board_palette::HANDLE_FILL,
				handle_stroke_disabled: board_palette::GRID_MINOR_STROKE,
				wire_stroke: board_palette::EDGE_STROKE,
				wire_stroke_hovered: board_palette::NODE_STROKE,
				wire_stroke_selected: board_palette::EDGE_STROKE_SELECTED,
				wire_stroke_highlighted: board_palette::INDIRECT_HANDLE_STROKE,
				wire_stroke_disabled: board_palette::GRID_MINOR_STROKE,
				selection_preview_fill: board_palette::SELECTION_PREVIEW_FILL,
				selection_preview_stroke: board_palette::SELECTION_PREVIEW_STROKE,
			}
		}
	}

	#[derive(Clone)]
	pub struct BoardHost {
		pub camera: Camera,
		pub nodes: BTreeMap<String, NodeData>,
		pub handles: BTreeMap<String, HandleData>,
		pub edges: BTreeMap<String, EdgeData>,
		pub wires: BTreeMap<String, WireData>,
		/// Catalog keyed by `handle_kind` id (see `set_board_kind_catalogs_from_json`).
		pub handle_kinds: BTreeMap<String, HandleKindDef>,
		pub wire_kinds: BTreeMap<String, WireKindDef>,
		pub node_kinds: BTreeMap<String, NodeKindDef>,
		pub edge_kinds: BTreeMap<String, EdgeKindDef>,
		/// @emoji 🔗 Kind-compatibility rules for link gestures; empty = unrestricted.
		pub link_compat_rules: Vec<LinkCompatRule>,
		pub selection: BTreeSet<String>,
		/// @emoji 👁️ Live rectangle/lasso preview ids (committed selection stays in `selection` until pointer-up).
		pub preselect: BTreeSet<String>,
		/// @emoji 💠 During preselect: anchor selection \\ `preselect` (secondary chrome while dragging).
		pub preselect_removed: BTreeSet<String>,
		/// @emoji 💠 After commit: ids dropped in the last `select` transition only.
		pub selection_exit_highlight: BTreeSet<String>,
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
		/// Screen-space polyline preview (CSS px) while dragging a handle link before drop.
		pub link_screen_preview: Option<Vec<Point>>,
		pub vello_theme: VelloThemePalette,
		/// @emoji 📶 Upper bounds for zoom bands: `zoom < minimap`, then overview, compact, normal, detail, else micro.
		pub lod_minimap_max_zoom: f64,
		pub lod_overview_max_zoom: f64,
		pub lod_compact_max_zoom: f64,
		pub lod_normal_max_zoom: f64,
		pub lod_detail_max_zoom: f64,
		/// @emoji 📐 Positive multiplier for LOD world grid steps (`10` / `5` / `1` base world units per band).
		pub grid_factor: f64,
		/// @emoji 🧲 When true, node drags snap to the finest visible LOD grid (step scales with `grid_factor`).
		pub grid_snap_enabled: bool,
		pub preserve_original_element_style: bool,
		/// @emoji 📶 When true (default), camera zoom selects draw LOD; when false, optional `forced_draw_lod` pins the tier when set.
		pub automatic_lod: bool,
		forced_draw_lod: Option<BoardDrawLod>,
		icon_vector_cache: RefCell<HashMap<String, CachedIconPaint>>,
		/// @emoji 📡 Dedupes {@code linkCompatibleNodes} emissions while a link wire is active.
		link_compat_nodes_emit_key: Option<String>,
		/// @emoji 📡 Dedupes {@code linkTargetRing} emissions while a link wire is active.
		link_target_ring_emit_key: Option<String>,
		/// @emoji 📡 Dedupes `select` emissions when ids are unchanged but modifier merge mode changes mid‑gesture.
		last_select_emit_sig: Option<(Vec<String>, Option<String>)>,
		/// @emoji 📡 Dedupes `preselect` emissions during area-select drag.
		last_preselect_emit_sig: Option<(Vec<String>, Vec<String>, Option<String>)>,
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
				wires: BTreeMap::new(),
				handle_kinds: BTreeMap::new(),
				wire_kinds: BTreeMap::new(),
				node_kinds: BTreeMap::new(),
				edge_kinds: BTreeMap::new(),
				link_compat_rules: Vec::new(),
				selection: BTreeSet::new(),
				preselect: BTreeSet::new(),
				preselect_removed: BTreeSet::new(),
				selection_exit_highlight: BTreeSet::new(),
				selection_options: SelectionOptions {
					method: "rectangle".into(),
					mode: "replace".into(),
					select_nodes: true,
					select_edges: true,
					select_handles: true,
				},
				hovered_id: None,
				interaction: Interaction::None,
				width: 1,
				height: 1,
				dpr: 1.0,
				world_raster_tiling: "world-clip".into(),
				events: Vec::new(),
				selection_screen_preview: None,
				link_screen_preview: None,
				vello_theme: VelloThemePalette::default(),
				lod_minimap_max_zoom: LOD_MINIMAP_MAX_ZOOM_DEFAULT,
				lod_overview_max_zoom: LOD_OVERVIEW_MAX_ZOOM_DEFAULT,
				lod_compact_max_zoom: LOD_COMPACT_MAX_ZOOM_DEFAULT,
				lod_normal_max_zoom: LOD_NORMAL_MAX_ZOOM_DEFAULT,
				lod_detail_max_zoom: LOD_DETAIL_MAX_ZOOM_DEFAULT,
				grid_factor: GRID_FACTOR_DEFAULT,
				grid_snap_enabled: false,
				preserve_original_element_style: false,
				automatic_lod: true,
				forced_draw_lod: None,
				icon_vector_cache: RefCell::new(HashMap::new()),
				link_compat_nodes_emit_key: None,
				link_target_ring_emit_key: None,
				last_select_emit_sig: None,
				last_preselect_emit_sig: None,
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

		fn grid_step_large_world(&self) -> f64 {
			GRID_WORLD_LARGE * self.grid_factor
		}
		fn grid_step_medium_world(&self) -> f64 {
			GRID_WORLD_MEDIUM * self.grid_factor
		}
		fn grid_step_small_world(&self) -> f64 {
			GRID_WORLD_SMALL * self.grid_factor
		}
		fn grid_step_micro_world(&self) -> f64 {
			GRID_WORLD_MICRO * self.grid_factor
		}

		pub fn new() -> Self {
			Self::default()
		}

		fn current_draw_lod(&self) -> BoardDrawLod {
			if !self.automatic_lod {
				if let Some(lod) = self.forced_draw_lod {
					return lod;
				}
			}
			let z = self.camera.zoom;
			if z < self.lod_minimap_max_zoom {
				BoardDrawLod::Minimap
			} else if z < self.lod_overview_max_zoom {
				BoardDrawLod::Overview
			} else if z < self.lod_compact_max_zoom {
				BoardDrawLod::Compact
			} else if z < self.lod_normal_max_zoom {
				BoardDrawLod::Normal
			} else if z < self.lod_detail_max_zoom {
				BoardDrawLod::Detail
			} else {
				BoardDrawLod::Micro
			}
		}

		fn lod_visible_grid_snap_step_world(&self) -> Option<f64> {
			match self.current_draw_lod() {
				BoardDrawLod::Minimap => None,
				BoardDrawLod::Overview | BoardDrawLod::Compact => Some(self.grid_step_large_world()),
				BoardDrawLod::Normal => Some(self.grid_step_medium_world()),
				BoardDrawLod::Detail => Some(self.grid_step_small_world()),
				BoardDrawLod::Micro => Some(self.grid_step_micro_world()),
			}
		}

		fn snap_world_scalar(&self, v: f64) -> f64 {
			if !self.grid_snap_enabled {
				return v;
			}
			let Some(step) = self.lod_visible_grid_snap_step_world() else {
				return v;
			};
			(v / step).round() * step
		}

		fn snap_world_pair(&self, x: f64, y: f64) -> (f64, f64) {
			(self.snap_world_scalar(x), self.snap_world_scalar(y))
		}

		/// @emoji 📶 JSON `{ "minimapMaxZoom", "overviewMaxZoom", "compactMaxZoom", "normalMaxZoom", "detailMaxZoom" }` strictly increasing CSS-scale zoom values.
		pub fn set_lod_zoom_thresholds_from_json(&mut self, json: &str) -> Result<(), String> {
			let v: serde_json::Value = serde_json::from_str(json).map_err(|e| e.to_string())?;
			let a = v.get("minimapMaxZoom").and_then(|x| x.as_f64()).ok_or_else(|| "minimapMaxZoom".to_string())?;
			let b = v.get("overviewMaxZoom").and_then(|x| x.as_f64()).ok_or_else(|| "overviewMaxZoom".to_string())?;
			let c = v.get("compactMaxZoom").and_then(|x| x.as_f64()).ok_or_else(|| "compactMaxZoom".to_string())?;
			let d = v.get("normalMaxZoom").and_then(|x| x.as_f64()).ok_or_else(|| "normalMaxZoom".to_string())?;
			let e = v.get("detailMaxZoom").and_then(|x| x.as_f64()).ok_or_else(|| "detailMaxZoom".to_string())?;
			if !(BOARD_CAMERA_ZOOM_MIN..=BOARD_CAMERA_ZOOM_MAX).contains(&a)
				|| !(BOARD_CAMERA_ZOOM_MIN..=BOARD_CAMERA_ZOOM_MAX).contains(&b)
				|| !(BOARD_CAMERA_ZOOM_MIN..=BOARD_CAMERA_ZOOM_MAX).contains(&c)
				|| !(BOARD_CAMERA_ZOOM_MIN..=BOARD_CAMERA_ZOOM_MAX).contains(&d)
				|| !(BOARD_CAMERA_ZOOM_MIN..=BOARD_CAMERA_ZOOM_MAX).contains(&e)
			{
				return Err("lod zoom thresholds must lie within camera zoom bounds".into());
			}
			if !(a < b && b < c && c < d && d < e) {
				return Err("lod zoom thresholds must satisfy minimap < overview < compact < normal < detail".into());
			}
			self.lod_minimap_max_zoom = a;
			self.lod_overview_max_zoom = b;
			self.lod_compact_max_zoom = c;
			self.lod_normal_max_zoom = d;
			self.lod_detail_max_zoom = e;
			Ok(())
		}

		pub fn set_grid_snap_enabled(&mut self, enabled: bool) {
			self.grid_snap_enabled = enabled;
		}

		pub fn set_automatic_lod(&mut self, enabled: bool) {
			self.automatic_lod = enabled;
			if enabled {
				self.forced_draw_lod = None;
			}
		}

		pub fn set_forced_draw_lod_label(&mut self, label: &str) {
			let t = label.trim();
			if t.is_empty() {
				self.forced_draw_lod = None;
				return;
			}
			self.forced_draw_lod = Some(match t {
				"minimap" => BoardDrawLod::Minimap,
				"overview" => BoardDrawLod::Overview,
				"compact" => BoardDrawLod::Compact,
				"normal" => BoardDrawLod::Normal,
				"detail" => BoardDrawLod::Detail,
				"micro" => BoardDrawLod::Micro,
				_ => {
					self.forced_draw_lod = None;
					return;
				}
			});
		}

		pub fn set_grid_factor(&mut self, v: f64) -> Result<(), String> {
			if !v.is_finite() || v <= 0.0 || v > 1_000_000.0 {
				return Err("gridFactor must be finite and in (0, 1e6]".into());
			}
			self.grid_factor = v;
			Ok(())
		}

		fn get_or_build_icon_paint(
			&self,
			encoded: &str,
			fg: Color,
			bg: Color,
			preserve_original_style: bool,
		) -> Option<(f64, f64, f64, f64, CachedIconBody)> {
			let resolved = super::board_icon_codec::board_resolve_icon_kind(encoded);
			let key = match &resolved {
				super::board_icon_codec::BoardResolvedIcon::None => return None,
				super::board_icon_codec::BoardResolvedIcon::SvgThemed(s)
				| super::board_icon_codec::BoardResolvedIcon::SvgPlain(s) => {
					Self::icon_vector_cache_key(if preserve_original_style { "p" } else { "t" }, s.as_str(), fg, bg)
				}
				super::board_icon_codec::BoardResolvedIcon::RasterRgba8 { rgba, w, h } => {
					Self::icon_raster_cache_key(rgba, *w, *h)
				}
			};
			{
				let g = self.icon_vector_cache.borrow();
				if let Some(c) = g.get(&key) {
					return Some((c.bx, c.by, c.bw, c.bh, c.body.clone()));
				}
			}
			let (bx, by, bw, bh, body) = match resolved {
				super::board_icon_codec::BoardResolvedIcon::None => return None,
				super::board_icon_codec::BoardResolvedIcon::SvgThemed(s) => {
					let tree = usvg::Tree::from_str(s.trim(), super::svg_icon_vello09::usvg_options_board_icons()).ok()?;
					let (bx, by, bw, bh) = super::svg_icon_vello09::svg_icon_content_bounds(&tree);
					if !(bw > 0.0 && bh > 0.0 && bw.is_finite() && bh.is_finite()) {
						return None;
					}
					let mut s = Scene::new();
					if preserve_original_style {
						let _ = vello_svg::append_tree(&mut s, &tree);
					} else {
						super::svg_icon_vello09::render_svg_tree_themed(&mut s, &tree, fg, bg);
					}
					(bx, by, bw, bh, CachedIconBody::Vector(s))
				}
				super::board_icon_codec::BoardResolvedIcon::SvgPlain(s) => {
					let svg_t = s.trim();
					let tree = usvg::Tree::from_str(svg_t, super::svg_icon_vello09::usvg_options_board_icons()).ok()?;
					let (bx, by, bw, bh) = super::svg_icon_vello09::svg_icon_content_bounds(&tree);
					if !(bw > 0.0 && bh > 0.0 && bw.is_finite() && bh.is_finite()) {
						return None;
					}
					let mut s = Scene::new();
					if preserve_original_style {
						let _ = vello_svg::append_tree(&mut s, &tree);
					} else {
						super::svg_icon_vello09::render_svg_tree_themed(&mut s, &tree, fg, bg);
					}
					(bx, by, bw, bh, CachedIconBody::Vector(s))
				}
				super::board_icon_codec::BoardResolvedIcon::RasterRgba8 { rgba, w, h } => {
					let bx = 0.0_f64;
					let by = 0.0_f64;
					let bw = f64::from(w);
					let bh = f64::from(h);
					let img = ImageData {
						data: Blob::new(Arc::new(rgba.as_ref().to_vec())),
						format: ImageFormat::Rgba8,
						alpha_type: ImageAlphaType::Alpha,
						width: w,
						height: h,
					};
					(bx, by, bw, bh, CachedIconBody::Raster(Arc::new(img)))
				}
			};
			let cached = CachedIconPaint {
				bx,
				by,
				bw,
				bh,
				body: body.clone(),
			};
			self.icon_vector_cache.borrow_mut().insert(key, cached);
			Some((bx, by, bw, bh, body))
		}

		pub fn clear_icon_vector_cache(&mut self) {
			self.icon_vector_cache.borrow_mut().clear();
		}

		fn icon_vector_cache_key(tag: &str, svg: &str, fg: Color, bg: Color) -> String {
			let mut hasher = std::collections::hash_map::DefaultHasher::new();
			svg.hash(&mut hasher);
			let hx = hasher.finish();
			let f = fg.to_rgba8();
			let b = bg.to_rgba8();
			format!(
				"v8|{tag}|{hx:x}|{}|{:02x}{:02x}{:02x}{:02x}|{:02x}{:02x}{:02x}{:02x}",
				svg.len(),
				f.r, f.g, f.b, f.a, b.r, b.g, b.b, b.a
			)
		}

		fn icon_raster_cache_key(rgba: &Arc<[u8]>, w: u32, h: u32) -> String {
			let mut hasher = std::collections::hash_map::DefaultHasher::new();
			rgba.as_ref().hash(&mut hasher);
			let hx = hasher.finish();
			format!("v8|r|{w}x{h}|{hx:x}|{}", rgba.len())
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

		pub fn set_selection_options(
			&mut self,
			method: &str,
			mode: &str,
			select_nodes: bool,
			select_edges: bool,
			select_handles: bool,
		) {
			self.selection_options.method = method.into();
			self.selection_options.mode = if mode == "default" { "replace" } else { mode }.into();
			self.selection_options.select_nodes = select_nodes;
			self.selection_options.select_edges = select_edges;
			self.selection_options.select_handles = select_handles;
		}

		/// @emoji 🔗 JSON `[{ "source","target","bidirectional"?,"important"?,"specificity"? },…]` gates link gestures; empty clears restrictions.
		pub fn set_handle_link_compat_from_json(&mut self, json: &str) -> Result<(), String> {
			let v: serde_json::Value = serde_json::from_str(json).map_err(|e| e.to_string())?;
			let arr = v
				.as_array()
				.ok_or_else(|| "expected JSON array of compatibility objects".to_string())?;
			let mut next = Vec::new();
			for row in arr {
				let o = row.as_object().ok_or("compat row must be object")?;
				let source = o
					.get("source")
					.and_then(|x| x.as_str())
					.ok_or_else(|| "compat row missing string source".to_string())?
					.trim()
					.to_string();
				let target = o
					.get("target")
					.and_then(|x| x.as_str())
					.ok_or_else(|| "compat row missing string target".to_string())?
					.trim()
					.to_string();
				let bidirectional = o.get("bidirectional").and_then(|x| x.as_bool()).unwrap_or(false);
				let important = o.get("important").and_then(|x| x.as_bool()).unwrap_or(false);
				let spec_s = o
					.get("specificity")
					.and_then(|x| x.as_str())
					.unwrap_or("handle");
				let specificity = Self::parse_compat_specificity(spec_s)?;
				next.push(LinkCompatRule {
					source,
					target,
					bidirectional,
					important,
					specificity,
				});
			}
			self.link_compat_rules = next;
			Ok(())
		}

		fn parse_compat_specificity(raw: &str) -> Result<CompatSpecificity, String> {
			match raw.trim().to_ascii_lowercase().as_str() {
				"general" => Ok(CompatSpecificity::General),
				"node" => Ok(CompatSpecificity::Node),
				"edge" => Ok(CompatSpecificity::Edge),
				"handle" => Ok(CompatSpecificity::Handle),
				"wire" => Ok(CompatSpecificity::Wire),
				_ => Err(format!("compat specificity must be general|node|edge|handle|wire, got {raw:?}")),
			}
		}

		/// @emoji 🧩 JSON object `{ handleKinds?, wireKinds?, nodeKinds?, edgeKinds? }` replacing prior catalogs (omit arrays to clear that slice).
		pub fn set_board_kind_catalogs_from_json(&mut self, json: &str) -> Result<(), String> {
			let v: serde_json::Value = serde_json::from_str(json).map_err(|e| e.to_string())?;
			let o = v.as_object().ok_or("kind catalogs root must be object")?;
			if let Some(arr) = o.get("handleKinds").and_then(|x| x.as_array()) {
				let mut next = BTreeMap::new();
				for row in arr {
					let ho = row.as_object().ok_or("handle kind row must be object")?;
					let id = ho
						.get("id")
						.and_then(|x| x.as_str())
						.map(str::trim)
						.filter(|s| !s.is_empty())
						.ok_or("handle kind id missing")?;
					let name = ho
						.get("label")
						.or_else(|| ho.get("name"))
						.and_then(|x| x.as_str())
						.unwrap_or("")
						.to_string();
					let color_s = ho
						.get("color")
						.and_then(|x| x.as_str())
						.map(str::trim)
						.filter(|s| !s.is_empty())
						.ok_or("handle kind color missing")?;
					let color = Self::parse_css_color(color_s).ok_or_else(|| format!("invalid handle kind color {color_s:?}"))?;
					let default_wire_kind = ho
						.get("defaultWireKind")
						.or_else(|| ho.get("default_wire_kind"))
						.and_then(|x| x.as_str())
						.map(str::trim)
						.filter(|s| !s.is_empty())
						.map(|s| s.to_string());
					let scale = ho.get("scale").and_then(|x| x.as_f64()).filter(|x| x.is_finite() && *x > 0.0).unwrap_or(1.0);
					next.insert(
						id.to_string(),
						HandleKindDef {
							name,
							color,
							default_wire_kind,
							scale,
						},
					);
				}
				self.handle_kinds = next;
			}
			if let Some(arr) = o.get("wireKinds").and_then(|x| x.as_array()) {
				let mut next = BTreeMap::new();
				for row in arr {
					let wo = row.as_object().ok_or("wire kind row must be object")?;
					let id = wo
						.get("id")
						.and_then(|x| x.as_str())
						.map(str::trim)
						.filter(|s| !s.is_empty())
						.ok_or("wire kind id missing")?;
					let name = wo
						.get("label")
						.or_else(|| wo.get("name"))
						.and_then(|x| x.as_str())
						.unwrap_or("")
						.to_string();
					let default_edge_kind = wo
						.get("defaultEdgeKind")
						.or_else(|| wo.get("default_edge_kind"))
						.and_then(|x| x.as_str())
						.map(str::trim)
						.filter(|s| !s.is_empty())
						.map(|s| s.to_string());
					next.insert(id.to_string(), WireKindDef { name, default_edge_kind });
				}
				self.wire_kinds = next;
			}
			if let Some(arr) = o.get("nodeKinds").and_then(|x| x.as_array()) {
				let mut next = BTreeMap::new();
				for row in arr {
					let no = row.as_object().ok_or("node kind row must be object")?;
					let id = no
						.get("id")
						.and_then(|x| x.as_str())
						.map(str::trim)
						.filter(|s| !s.is_empty())
						.ok_or("node kind id missing")?;
					let name = no
						.get("label")
						.or_else(|| no.get("name"))
						.and_then(|x| x.as_str())
						.unwrap_or("")
						.to_string();
					let scale = no.get("scale").and_then(|x| x.as_f64()).filter(|x| x.is_finite() && *x > 0.0).unwrap_or(1.0);
					next.insert(id.to_string(), NodeKindDef { name, scale });
				}
				self.node_kinds = next;
			}
			if let Some(arr) = o.get("edgeKinds").and_then(|x| x.as_array()) {
				let mut next = BTreeMap::new();
				for row in arr {
					let eo = row.as_object().ok_or("edge kind row must be object")?;
					let id = eo
						.get("id")
						.and_then(|x| x.as_str())
						.map(str::trim)
						.filter(|s| !s.is_empty())
						.ok_or("edge kind id missing")?;
					let name = eo
						.get("label")
						.or_else(|| eo.get("name"))
						.and_then(|x| x.as_str())
						.unwrap_or("")
						.to_string();
					next.insert(id.to_string(), EdgeKindDef { name });
				}
				self.edge_kinds = next;
			}
			Ok(())
		}

		fn parse_css_hex_color(s: &str) -> Option<Color> {
			let s = s.trim();
			let hex = s.strip_prefix('#')?;
			match hex.len() {
				3 => {
					let mut full = String::new();
					for ch in hex.chars() {
						full.push(ch);
						full.push(ch);
					}
					let v = u32::from_str_radix(&full, 16).ok()?;
					let r = ((v >> 16) & 0xff) as u8;
					let g = ((v >> 8) & 0xff) as u8;
					let b = (v & 0xff) as u8;
					Some(Color::from_rgba8(r, g, b, 255))
				}
				6 => {
					let v = u32::from_str_radix(hex, 16).ok()?;
					let r = ((v >> 16) & 0xff) as u8;
					let g = ((v >> 8) & 0xff) as u8;
					let b = (v & 0xff) as u8;
					Some(Color::from_rgba8(r, g, b, 255))
				}
				8 => {
					let v = u32::from_str_radix(hex, 16).ok()?;
					let r = ((v >> 24) & 0xff) as u8;
					let g = ((v >> 16) & 0xff) as u8;
					let b = ((v >> 8) & 0xff) as u8;
					let a = (v & 0xff) as u8;
					Some(Color::from_rgba8(r, g, b, a))
				}
				_ => None,
			}
		}

		/// @emoji 🎨 Accepts `#rgb`/`#rrggbb`/`#rrggbbaa` or CSS `hsl()` / `hsla()` (comma or space syntax, optional `/` alpha).
		fn parse_css_color(s: &str) -> Option<Color> {
			if let Some(c) = Self::parse_css_hex_color(s) {
				return Some(c);
			}
			Self::parse_css_hsl_color(s)
		}

		fn parse_css_hsl_color(s: &str) -> Option<Color> {
			let low = s.trim().to_ascii_lowercase();
			let (legacy_alpha_form, inner) =
				if let Some(inner) = low.strip_prefix("hsla(").and_then(|x| x.strip_suffix(')')) {
					(true, inner)
				} else if let Some(inner) = low.strip_prefix("hsl(").and_then(|x| x.strip_suffix(')')) {
					(false, inner)
				} else {
					return None;
				};
			let inner = inner.trim();
			let (main, alpha_slash) = inner
				.split_once('/')
				.map(|(a, b)| (a.trim(), Some(b.trim())))
				.unwrap_or((inner, None));
			let normalized = main.replace(',', " ");
			let parts: Vec<&str> = normalized.split_whitespace().collect();
			if parts.len() < 3 {
				return None;
			}
			let h = Self::parse_css_hsl_hue(parts[0])?;
			let sat = Self::parse_css_hsl_sl(parts[1])?;
			let light = Self::parse_css_hsl_sl(parts[2])?;
			let alpha = if let Some(a) = alpha_slash {
				Self::parse_css_alpha_channel(a)?
			} else if legacy_alpha_form && parts.len() >= 4 {
				Self::parse_css_alpha_channel(parts[3])?
			} else {
				1.0
			};
			let (r, g, b) = Self::hsl_to_rgb_bytes(h, sat, light);
			let a = (alpha * 255.0).round().clamp(0.0, 255.0) as u8;
			Some(Color::from_rgba8(r, g, b, a))
		}

		fn parse_css_hsl_hue(tok: &str) -> Option<f64> {
			let t = tok.trim();
			let n = t.strip_suffix("deg").map(str::trim).unwrap_or(t);
			let v: f64 = n.parse().ok()?;
			v.is_finite().then_some(v)
		}

		fn parse_css_hsl_sl(tok: &str) -> Option<f64> {
			let t = tok.trim();
			if let Some(p) = t.strip_suffix('%') {
				let v: f64 = p.trim().parse().ok()?;
				Some((v / 100.0).clamp(0.0, 1.0))
			} else {
				let v: f64 = t.parse().ok()?;
				Some(v.clamp(0.0, 1.0))
			}
		}

		fn parse_css_alpha_channel(tok: &str) -> Option<f64> {
			Self::parse_css_hsl_sl(tok)
		}

		fn board_hsl_hue_to_rgb_component(p: f64, q: f64, mut t: f64) -> f64 {
			if t < 0.0 {
				t += 1.0;
			}
			if t > 1.0 {
				t -= 1.0;
			}
			if t < 1.0 / 6.0 {
				p + (q - p) * 6.0 * t
			} else if t < 0.5 {
				q
			} else if t < 2.0 / 3.0 {
				p + (q - p) * (2.0 / 3.0 - t) * 6.0
			} else {
				p
			}
		}

		fn hsl_to_rgb_bytes(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
			let h_norm = ((h % 360.0 + 360.0) % 360.0) / 360.0;
			let s = s.clamp(0.0, 1.0);
			let l = l.clamp(0.0, 1.0);
			if s <= f64::EPSILON {
				let v = (l * 255.0).round().clamp(0.0, 255.0) as u8;
				return (v, v, v);
			}
			let q = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s };
			let p = 2.0 * l - q;
			let r = Self::board_hsl_hue_to_rgb_component(p, q, h_norm + 1.0 / 3.0);
			let g = Self::board_hsl_hue_to_rgb_component(p, q, h_norm);
			let b = Self::board_hsl_hue_to_rgb_component(p, q, h_norm - 1.0 / 3.0);
			(
				(r * 255.0).round().clamp(0.0, 255.0) as u8,
				(g * 255.0).round().clamp(0.0, 255.0) as u8,
				(b * 255.0).round().clamp(0.0, 255.0) as u8,
			)
		}

		fn explicit_style_kind(style: Option<&str>) -> Option<BoardElementStyleKind> {
			match style.map(str::trim).filter(|s| !s.is_empty()) {
				Some("original") => Some(BoardElementStyleKind::Original),
				Some("neutral") => Some(BoardElementStyleKind::Neutral),
				Some("hovered") => Some(BoardElementStyleKind::Hovered),
				Some("selected") => Some(BoardElementStyleKind::Selected),
				Some("highlighted") => Some(BoardElementStyleKind::Highlighted),
				Some("disabled") => Some(BoardElementStyleKind::Disabled),
				_ => None,
			}
		}

		fn hovered_style_kind(&self, id: &str) -> Option<BoardElementStyleKind> {
			if self.is_preselect_active() {
				return None;
			}
			if self.selection.contains(id) {
				return None;
			}
			(self.hovered_id.as_deref() == Some(id)).then_some(BoardElementStyleKind::Hovered)
		}

		fn is_preselect_active(&self) -> bool {
			self.is_preselecting() || !self.preselect.is_empty()
		}

		/// @emoji 🎨 During area-select: preselect → Selected; anchor∖preselect → Highlighted; idle selection → Selected.
		fn resolve_interaction_style_kind(&self, id: &str) -> BoardElementStyleKind {
			if self.is_preselect_active() {
				if self.preselect.contains(id) {
					return BoardElementStyleKind::Selected;
				}
				if self.selection.contains(id) {
					return BoardElementStyleKind::Highlighted;
				}
				return BoardElementStyleKind::Neutral;
			}
			if self.selection.contains(id) {
				return BoardElementStyleKind::Selected;
			}
			BoardElementStyleKind::Neutral
		}

		fn resolve_node_style_kind(&self, n: &NodeData) -> BoardElementStyleKind {
			if let Some(kind) = Self::explicit_style_kind(n.style.as_deref()) {
				return kind;
			}
			if let Some(kind) = self.hovered_style_kind(n.id.as_str()) {
				return kind;
			}
			self.resolve_interaction_style_kind(n.id.as_str())
		}

		fn resolve_handle_style_kind(&self, h: &HandleData) -> BoardElementStyleKind {
			if let Some(kind) = Self::explicit_style_kind(h.style.as_deref()) {
				return kind;
			}
			if let Some(kind) = self.hovered_style_kind(h.id.as_str()) {
				return kind;
			}
			self.resolve_interaction_style_kind(h.id.as_str())
		}

		fn resolve_edge_style_kind(&self, e: &EdgeData) -> BoardElementStyleKind {
			if let Some(kind) = Self::explicit_style_kind(e.style.as_deref()) {
				return kind;
			}
			if let Some(kind) = self.hovered_style_kind(e.id.as_str()) {
				return kind;
			}
			self.resolve_interaction_style_kind(e.id.as_str())
		}

		fn resolve_wire_style_kind(&self, w: &WireData) -> BoardElementStyleKind {
			if let Some(kind) = Self::explicit_style_kind(w.style.as_deref()) {
				return kind;
			}
			if let Some(kind) = self.hovered_style_kind(w.id.as_str()) {
				return kind;
			}
			self.resolve_interaction_style_kind(w.id.as_str())
		}

		fn node_fill_for_style(theme: &VelloThemePalette, kind: BoardElementStyleKind) -> Color {
			match kind {
				BoardElementStyleKind::Hovered => theme.node_fill_hovered,
				BoardElementStyleKind::Selected => theme.node_fill_selected,
				BoardElementStyleKind::Highlighted => theme.node_fill_selection_exit,
				BoardElementStyleKind::Disabled => theme.node_fill_disabled,
				BoardElementStyleKind::Original | BoardElementStyleKind::Neutral => theme.node_fill,
			}
		}

		fn node_stroke_for_style(theme: &VelloThemePalette, kind: BoardElementStyleKind) -> Color {
			match kind {
				BoardElementStyleKind::Hovered => theme.node_stroke_hovered,
				BoardElementStyleKind::Selected => theme.node_stroke_selected,
				BoardElementStyleKind::Highlighted => theme.node_stroke_selection_exit,
				BoardElementStyleKind::Disabled => theme.node_stroke_disabled,
				BoardElementStyleKind::Original | BoardElementStyleKind::Neutral => theme.node_stroke,
			}
		}

		fn resolve_handle_fill_color(&self, h: &HandleData, theme: &VelloThemePalette, kind: BoardElementStyleKind) -> Color {
			match kind {
				BoardElementStyleKind::Hovered => theme.handle_fill_hovered,
				BoardElementStyleKind::Selected => theme.handle_fill_selected,
				BoardElementStyleKind::Highlighted => theme.handle_fill_selection_exit,
				BoardElementStyleKind::Disabled => theme.handle_fill_disabled,
				BoardElementStyleKind::Original | BoardElementStyleKind::Neutral => {
					if let Some(c) = h.color_fill {
						return c;
					}
					if let Some(def) = self.handle_kinds.get(&h.handle_kind) {
						return def.color;
					}
					theme.handle_fill
				}
			}
		}

		fn resolve_handle_stroke_color(&self, _h: &HandleData, theme: &VelloThemePalette, kind: BoardElementStyleKind) -> Color {
			match kind {
				BoardElementStyleKind::Hovered => theme.handle_stroke_hovered,
				BoardElementStyleKind::Selected => theme.handle_stroke_selected,
				BoardElementStyleKind::Highlighted => theme.handle_stroke_selection_exit,
				BoardElementStyleKind::Disabled => theme.handle_stroke_disabled,
				BoardElementStyleKind::Original | BoardElementStyleKind::Neutral => theme.handle_stroke,
			}
		}

		fn edge_stroke_for_style(theme: &VelloThemePalette, kind: BoardElementStyleKind) -> Color {
			match kind {
				BoardElementStyleKind::Hovered => theme.edge_stroke_hovered,
				BoardElementStyleKind::Selected => theme.edge_stroke_selected,
				BoardElementStyleKind::Highlighted => theme.edge_stroke_selection_exit,
				BoardElementStyleKind::Disabled => theme.edge_stroke_disabled,
				BoardElementStyleKind::Original | BoardElementStyleKind::Neutral => theme.edge_stroke,
			}
		}

		fn wire_stroke_for_style(theme: &VelloThemePalette, kind: BoardElementStyleKind) -> Color {
			match kind {
				BoardElementStyleKind::Hovered => theme.wire_stroke_hovered,
				BoardElementStyleKind::Selected => theme.wire_stroke_selected,
				BoardElementStyleKind::Highlighted => theme.wire_stroke_highlighted,
				BoardElementStyleKind::Disabled => theme.wire_stroke_disabled,
				BoardElementStyleKind::Original | BoardElementStyleKind::Neutral => theme.wire_stroke,
			}
		}

		fn handles_link_compatible_for_drag(&self, source: &HandleData, target: &HandleData) -> bool {
			if self.link_compat_rules.is_empty() {
				return true;
			}
			let mut matched: Vec<&LinkCompatRule> = self
				.link_compat_rules
				.iter()
				.filter(|rule| self.link_gesture_rule_applies(rule, source, target))
				.collect();
			if matched.is_empty() {
				return false;
			}
			if matched.iter().any(|r| r.important) {
				matched.retain(|r| r.important);
			} else {
				let max_rank = matched
					.iter()
					.map(|r| r.specificity as i32)
					.max()
					.unwrap_or(0);
				matched.retain(|r| (r.specificity as i32) == max_rank);
			}
			!matched.is_empty()
		}

		fn compat_pair_matches(rule: &LinkCompatRule, a: &str, b: &str) -> bool {
			if rule.source == a && rule.target == b {
				return true;
			}
			if rule.bidirectional && rule.source == b && rule.target == a {
				return true;
			}
			false
		}

		fn resolve_default_wire_kind_for_handle(&self, h: &HandleData) -> String {
			self.handle_kinds
				.get(&h.handle_kind)
				.and_then(|d| d.default_wire_kind.as_ref())
				.map(|s| s.trim().to_string())
				.filter(|s| !s.is_empty())
				.unwrap_or_else(|| BOARD_DEFAULT_WIRE_KIND_ID.to_string())
		}

		fn resolve_default_edge_kind_for_wire_kind(&self, wire_kind: &str) -> String {
			self.wire_kinds
				.get(wire_kind)
				.and_then(|d| d.default_edge_kind.as_ref())
				.map(|s| s.trim().to_string())
				.filter(|s| !s.is_empty())
				.unwrap_or_default()
		}

		fn link_gesture_rule_applies(&self, rule: &LinkCompatRule, source: &HandleData, target: &HandleData) -> bool {
			let w_src = self.resolve_default_wire_kind_for_handle(source);
			let w_tgt = self.resolve_default_wire_kind_for_handle(target);
			let e_src = self.resolve_default_edge_kind_for_wire_kind(&w_src);
			let e_tgt = self.resolve_default_edge_kind_for_wire_kind(&w_tgt);
			let sn = self
				.nodes
				.get(&source.node_id)
				.map(|n| n.node_kind.as_str())
				.unwrap_or("");
			let tn = self
				.nodes
				.get(&target.node_id)
				.map(|n| n.node_kind.as_str())
				.unwrap_or("");
			let sh = source.handle_kind.as_str();
			let th = target.handle_kind.as_str();
			match rule.specificity {
				CompatSpecificity::General => Self::compat_pair_matches(rule, sh, th),
				CompatSpecificity::Node => Self::compat_pair_matches(rule, sn, tn),
				CompatSpecificity::Edge => Self::compat_pair_matches(rule, e_src.as_str(), e_tgt.as_str()),
				CompatSpecificity::Handle => Self::compat_pair_matches(rule, sh, th),
				CompatSpecificity::Wire => Self::compat_pair_matches(rule, w_src.as_str(), th),
			}
		}

		fn default_edge_kind_for_created_link(&self, source: &HandleData, _target: &HandleData) -> String {
			let wk = self.resolve_default_wire_kind_for_handle(source);
			self.resolve_default_edge_kind_for_wire_kind(&wk)
		}

		/// @emoji 🧩 Selects world-space clip tiling for Vello scene construction (`none` | `world-clip`).
		pub fn set_world_raster_tiling(&mut self, mode: &str) {
			let next = if mode == "world-clip" { "world-clip".into() } else { "none".into() };
			if self.world_raster_tiling == next {
				return;
			}
			self.world_raster_tiling = next;
		}

		pub fn set_original_element_style(&mut self, enabled: bool) {
			if self.preserve_original_element_style == enabled {
				return;
			}
			self.preserve_original_element_style = enabled;
			self.icon_vector_cache.borrow_mut().clear();
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
			if let Some(arr) = v.get("edgeStrokeHovered").and_then(|x| x.as_array()) {
				if let Some(c) = Self::color_from_json_rgba8(arr) {
					next.edge_stroke_hovered = c;
				}
			}
			if let Some(arr) = v.get("edgeStrokeSelected").and_then(|x| x.as_array()) {
				if let Some(c) = Self::color_from_json_rgba8(arr) {
					next.edge_stroke_selected = c;
				}
			}
			if let Some(arr) = v.get("edgeStrokeSelectionExit").and_then(|x| x.as_array()) {
				if let Some(c) = Self::color_from_json_rgba8(arr) {
					next.edge_stroke_selection_exit = c;
				}
			}
			if let Some(arr) = v.get("edgeStrokeDisabled").and_then(|x| x.as_array()) {
				if let Some(c) = Self::color_from_json_rgba8(arr) {
					next.edge_stroke_disabled = c;
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
			if let Some(arr) = v.get("nodeFillHovered").and_then(|x| x.as_array()) {
				if let Some(c) = Self::color_from_json_rgba8(arr) {
					next.node_fill_hovered = c;
				}
			}
			if let Some(arr) = v.get("nodeStrokeHovered").and_then(|x| x.as_array()) {
				if let Some(c) = Self::color_from_json_rgba8(arr) {
					next.node_stroke_hovered = c;
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
			if let Some(arr) = v.get("nodeFillSelectionExit").and_then(|x| x.as_array()) {
				if let Some(c) = Self::color_from_json_rgba8(arr) {
					next.node_fill_selection_exit = c;
				}
			}
			if let Some(arr) = v.get("nodeStrokeSelectionExit").and_then(|x| x.as_array()) {
				if let Some(c) = Self::color_from_json_rgba8(arr) {
					next.node_stroke_selection_exit = c;
				}
			}
			if let Some(arr) = v.get("nodeFillDisabled").and_then(|x| x.as_array()) {
				if let Some(c) = Self::color_from_json_rgba8(arr) {
					next.node_fill_disabled = c;
				}
			}
			if let Some(arr) = v.get("nodeStrokeDisabled").and_then(|x| x.as_array()) {
				if let Some(c) = Self::color_from_json_rgba8(arr) {
					next.node_stroke_disabled = c;
				}
			}
			if let Some(arr) = v.get("indirectHandleFill").and_then(|x| x.as_array()) {
				if let Some(c) = Self::color_from_json_rgba8(arr) {
					next.indirect_handle_fill = c;
				}
			}
			if let Some(arr) = v.get("indirectHandleStroke").and_then(|x| x.as_array()) {
				if let Some(c) = Self::color_from_json_rgba8(arr) {
					next.indirect_handle_stroke = c;
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
			if let Some(arr) = v.get("handleFillHovered").and_then(|x| x.as_array()) {
				if let Some(c) = Self::color_from_json_rgba8(arr) {
					next.handle_fill_hovered = c;
				}
			}
			if let Some(arr) = v.get("handleStrokeHovered").and_then(|x| x.as_array()) {
				if let Some(c) = Self::color_from_json_rgba8(arr) {
					next.handle_stroke_hovered = c;
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
			if let Some(arr) = v.get("handleFillSelectionExit").and_then(|x| x.as_array()) {
				if let Some(c) = Self::color_from_json_rgba8(arr) {
					next.handle_fill_selection_exit = c;
				}
			}
			if let Some(arr) = v.get("handleStrokeSelectionExit").and_then(|x| x.as_array()) {
				if let Some(c) = Self::color_from_json_rgba8(arr) {
					next.handle_stroke_selection_exit = c;
				}
			}
			if let Some(arr) = v.get("handleFillDisabled").and_then(|x| x.as_array()) {
				if let Some(c) = Self::color_from_json_rgba8(arr) {
					next.handle_fill_disabled = c;
				}
			}
			if let Some(arr) = v.get("handleStrokeDisabled").and_then(|x| x.as_array()) {
				if let Some(c) = Self::color_from_json_rgba8(arr) {
					next.handle_stroke_disabled = c;
				}
			}
			if let Some(arr) = v.get("wireStroke").and_then(|x| x.as_array()) {
				if let Some(c) = Self::color_from_json_rgba8(arr) {
					next.wire_stroke = c;
				}
			}
			if let Some(arr) = v.get("wireStrokeHovered").and_then(|x| x.as_array()) {
				if let Some(c) = Self::color_from_json_rgba8(arr) {
					next.wire_stroke_hovered = c;
				}
			}
			if let Some(arr) = v.get("wireStrokeSelected").and_then(|x| x.as_array()) {
				if let Some(c) = Self::color_from_json_rgba8(arr) {
					next.wire_stroke_selected = c;
				}
			}
			if let Some(arr) = v.get("wireStrokeHighlighted").and_then(|x| x.as_array()) {
				if let Some(c) = Self::color_from_json_rgba8(arr) {
					next.wire_stroke_highlighted = c;
				}
			}
			if let Some(arr) = v.get("wireStrokeDisabled").and_then(|x| x.as_array()) {
				if let Some(c) = Self::color_from_json_rgba8(arr) {
					next.wire_stroke_disabled = c;
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
			self.icon_vector_cache.borrow_mut().clear();
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

		fn is_preselecting(&self) -> bool {
			matches!(&self.interaction, Interaction::Selection { .. })
		}

		/// @emoji 💠 Live area-select preview ids, or committed selection when not preselecting.
		fn selection_chrome_ids(&self) -> BTreeSet<String> {
			if self.is_preselecting() || !self.preselect.is_empty() {
				self.preselect.clone()
			} else {
				self.selection.clone()
			}
		}

		/// @emoji 🖱️ Empty selection on background click without exit/highlight chrome or preselect.
		fn clear_selection_on_background_click(&mut self) {
			if self.selection.is_empty() {
				return;
			}
			self.preselect.clear();
			self.preselect_removed.clear();
			self.last_preselect_emit_sig = None;
			self.last_select_emit_sig = None;
			self.selection_exit_highlight.clear();
			self.selection.clear();
			self.sync_selection_flags_to_objects();
			self.push_event("select", json!({ "ids": [], "exitHighlightIds": [] }));
		}

		fn sync_selection_flags_to_objects(&mut self) {
			let chrome = self.selection_chrome_ids();
			for n in self.nodes.values_mut() {
				n.selected = chrome.contains(&n.id);
			}
			for h in self.handles.values_mut() {
				h.selected = chrome.contains(&h.id);
			}
			for e in self.edges.values_mut() {
				e.selected = chrome.contains(&e.id);
			}
			for w in self.wires.values_mut() {
				w.selected = chrome.contains(&w.id);
			}
		}

		fn push_select_event(&mut self) {
			self.last_select_emit_sig = None;
			let mut sorted: Vec<_> = self.selection.iter().cloned().collect();
			sorted.sort();
			self.push_event("select", json!({ "ids": sorted, "exitHighlightIds": [] }));
		}

		pub fn set_selection_ids(&mut self, ids: &[String]) {
			let next: BTreeSet<String> = ids.iter().cloned().collect();
			if next == self.selection {
				return;
			}
			self.preselect.clear();
			self.preselect_removed.clear();
			self.last_preselect_emit_sig = None;
			self.selection_exit_highlight.clear();
			self.selection = next;
			self.sync_selection_flags_to_objects();
			self.push_select_event();
		}

		/// @emoji 🔇 Updates committed selection without emitting `select` (controlled React sync).
		pub fn set_selection_ids_silent(&mut self, ids: &[String]) {
			let next: BTreeSet<String> = ids.iter().cloned().collect();
			if next == self.selection {
				return;
			}
			self.preselect.clear();
			self.preselect_removed.clear();
			self.last_preselect_emit_sig = None;
			self.selection_exit_highlight.clear();
			self.selection = next;
			self.sync_selection_flags_to_objects();
		}

		/// @emoji 🔇 Mirrors area-select preview chrome without emitting `preselect` (shared multi-view sync).
		pub fn set_preselect_state_silent(&mut self, ids: &[String], removed_ids: &[String]) {
			let next: BTreeSet<String> = ids.iter().cloned().collect();
			let removed: BTreeSet<String> = removed_ids.iter().cloned().collect();
			if self.preselect == next && self.preselect_removed == removed {
				return;
			}
			self.preselect = next;
			self.preselect_removed = removed;
			self.sync_selection_flags_to_objects();
		}

		fn set_selection_ids_gestured(&mut self, ids: &[String], gesture: Option<&str>) {
			let next: BTreeSet<String> = ids.iter().cloned().collect();
			let mut sorted: Vec<_> = next.iter().cloned().collect();
			sorted.sort();
			let gesture_owned = gesture.map(std::borrow::ToOwned::to_owned);
			let sig = (sorted.clone(), gesture_owned.clone());
			if next == self.selection && self.last_select_emit_sig.as_ref() == Some(&sig) {
				return;
			}
			self.last_select_emit_sig = Some(sig);
			self.preselect.clear();
			self.preselect_removed.clear();
			self.last_preselect_emit_sig = None;
			if next != self.selection {
				self.selection_exit_highlight.clear();
				self.selection = next;
				self.sync_selection_flags_to_objects();
			}
			let mut payload = json!({ "ids": sorted, "exitHighlightIds": [] });
			if let Some(ref g) = gesture_owned {
				payload["gestureMergeMode"] = json!(g);
			}
			self.push_event("select", payload);
		}

		/// @emoji 👁️ Rectangle/lasso drag preview: `preselect` + `preselect_removed` (anchor \\ preselect); emits `preselect` only.
		fn apply_area_preselect(&mut self, anchor_ids: &BTreeSet<String>, ids: &[String], gesture: Option<&str>) {
			let next: BTreeSet<String> = ids.iter().cloned().collect();
			let sorted = Self::sorted_selection_ids(&next);
			let removed = Self::sorted_selection_ids(&anchor_ids.difference(&next).cloned().collect());
			let gesture_owned = gesture.map(std::borrow::ToOwned::to_owned);
			let sig = (sorted.clone(), removed.clone(), gesture_owned.clone());
			if self.preselect == next && self.last_preselect_emit_sig.as_ref() == Some(&sig) {
				return;
			}
			self.last_preselect_emit_sig = Some(sig);
			self.preselect = next;
			self.preselect_removed = anchor_ids.difference(&self.preselect).cloned().collect();
			self.set_hovered_id_silent(None);
			self.sync_selection_flags_to_objects();
			let mut payload = json!({ "ids": sorted, "removedIds": removed });
			if let Some(ref g) = gesture_owned {
				payload["gestureMergeMode"] = json!(g);
			}
			self.push_event("preselect", payload);
		}

		fn sorted_selection_ids(set: &BTreeSet<String>) -> Vec<String> {
			let mut v: Vec<_> = set.iter().cloned().collect();
			v.sort();
			v
		}

		/// @emoji 🧿 Ends a rectangle/lasso cycle: commits `selection`, clears preselect (highlight only lives in preselect).
		fn commit_area_select_from_initial(&mut self, initial_ids: &BTreeSet<String>, ids: &[String], gesture: Option<&str>) {
			let next: BTreeSet<String> = ids.iter().cloned().collect();
			let sorted = Self::sorted_selection_ids(&next);
			let anchor = Self::sorted_selection_ids(initial_ids);
			let gesture_owned = gesture.map(std::borrow::ToOwned::to_owned);
			self.last_select_emit_sig = None;
			self.last_preselect_emit_sig = None;
			self.preselect.clear();
			self.preselect_removed.clear();
			self.selection_exit_highlight.clear();
			self.selection = next;
			self.sync_selection_flags_to_objects();
			let mut payload = json!({ "ids": sorted, "anchorIds": anchor, "exitHighlightIds": [] });
			if let Some(ref g) = gesture_owned {
				payload["gestureMergeMode"] = json!(g);
			}
			self.push_event("select", payload);
		}

		/// @emoji 🧿 True during left‑button rectangle/lasso drag so callers can avoid descriptor round‑trips that fight the live marquee state.
		pub fn is_dragging_area_select(&self) -> bool {
			matches!(&self.interaction, Interaction::Selection { .. })
		}

		/// @emoji 🧿 True while a handle link gesture is active so JS can defer `syncDescriptorJson` the same way as area select.
		pub fn defers_descriptor_sync_from_js(&self) -> bool {
			matches!(
				self.interaction,
				Interaction::LinkAtSourceHandle { .. } | Interaction::LinkDragSnap { .. } | Interaction::LinkTargetNode { .. }
			)
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

		fn node_kind_scale(&self, node_kind: &str) -> f64 {
			self.node_kinds.get(node_kind).map(|k| k.scale).unwrap_or(1.0)
		}

		fn handle_kind_scale(&self, handle_kind: &str) -> f64 {
			self.handle_kinds.get(handle_kind).map(|k| k.scale).unwrap_or(1.0)
		}

		fn effective_node_scale(&self, n: &NodeData) -> f64 {
			(n.scale * self.node_kind_scale(n.node_kind.as_str())).max(1e-9)
		}

		fn scaled_node_radius(&self, n: &NodeData) -> f64 {
			n.radius * self.effective_node_scale(n)
		}

		fn scaled_node_width(&self, n: &NodeData) -> f64 {
			n.width * self.effective_node_scale(n)
		}

		fn scaled_node_height(&self, n: &NodeData) -> f64 {
			n.height * self.effective_node_scale(n)
		}

		fn effective_handle_scale(&self, h: &HandleData) -> f64 {
			let node_scale = self
				.nodes
				.get(h.node_id.as_str())
				.map(|n| self.effective_node_scale(n))
				.unwrap_or(1.0);
			(node_scale * h.scale * self.handle_kind_scale(h.handle_kind.as_str())).max(1e-9)
		}

		pub(crate) fn effective_handle_radius(&self, h: &HandleData) -> f64 {
			h.radius * self.effective_handle_scale(h)
		}

		pub(crate) fn handle_world_pos(&self, h: &HandleData) -> Option<Point> {
			let n = self.nodes.get(&h.node_id)?;
			Some(match n.shape {
				NodeShape::Circle => handle_position_on_circle(Point::new(n.x, n.y), self.scaled_node_radius(n), h.angle),
				NodeShape::Rectangle => {
					handle_position_on_rectangle(Point::new(n.x, n.y), self.scaled_node_width(n), self.scaled_node_height(n), h.angle)
				}
			})
		}

		/// @emoji 📐 Node half-extent for indirect ring layout: circle radius or half the shorter rectangle side.
		fn indirect_node_half_extent(&self, n: &NodeData) -> f64 {
			match n.shape {
				NodeShape::Circle => self.scaled_node_radius(n),
				NodeShape::Rectangle => self.scaled_node_width(n).min(self.scaled_node_height(n)) * 0.5,
			}
		}

		/// @emoji 📐 Radial world offset from node rim to indirect-handle center (`INDIRECT_HANDLE_RING_GAP_NODE_SCALE`× half-extent) so ring–node proportions stay fixed when zooming.
		fn indirect_handle_ring_offset_world(&self, n: &NodeData) -> f64 {
			(self.indirect_node_half_extent(n) * INDIRECT_HANDLE_RING_GAP_NODE_SCALE).max(1e-9)
		}

		/// @emoji 📐 Ghost link handles sit on a rim offset by `INDIRECT_HANDLE_RING_GAP_NODE_SCALE`× node half-extent from the node body so ring spacing scales with the node at every zoom.
		pub(crate) fn indirect_handle_world_pos(&self, h: &HandleData) -> Option<Point> {
			let n = self.nodes.get(&h.node_id)?;
			let offset = self.indirect_handle_ring_offset_world(n);
			Some(match n.shape {
				NodeShape::Circle => handle_position_on_circle(Point::new(n.x, n.y), self.scaled_node_radius(n) + offset, h.angle),
				NodeShape::Rectangle => handle_position_on_rectangle(
					Point::new(n.x, n.y),
					self.scaled_node_width(n) + 2.0 * offset,
					self.scaled_node_height(n) + 2.0 * offset,
					h.angle,
				),
			})
		}

		/// @emoji 📐 Indirect-connect marker radius in world units: `INDIRECT_HANDLE_MARKER_NODE_SCALE`× circle radius or × half the shorter rectangle side.
		pub(crate) fn indirect_handle_marker_radius_world(&self, h: &HandleData) -> f64 {
			let Some(n) = self.nodes.get(&h.node_id) else {
				return (self.effective_handle_radius(h) * INDIRECT_HANDLE_MARKER_NODE_SCALE).max(1e-9);
			};
			let handle_local_scale = (h.scale * self.handle_kind_scale(h.handle_kind.as_str())).max(1e-9);
			(self.indirect_node_half_extent(n) * INDIRECT_HANDLE_MARKER_NODE_SCALE * handle_local_scale).max(1e-9)
		}

		/// @emoji 🧭 Source handle id while a link wire is drawn (`LinkDragSnap` / `LinkTargetNode`).
		fn active_link_source_handle_id(&self) -> Option<&str> {
			match &self.interaction {
				Interaction::LinkDragSnap { source_id, .. } | Interaction::LinkTargetNode { source_id, .. } => {
					Some(source_id.as_str())
				}
				_ => None,
			}
		}

		/// @emoji 🧭 Visible target node ids that expose at least one free handle compatible with `source_handle_id`.
		fn link_drag_compatible_target_node_ids(&self, source_handle_id: &str) -> Vec<String> {
			let Some(source) = self.handles.get(source_handle_id) else {
				return Vec::new();
			};
			let source_node_id = source.node_id.as_str();
			let mut out = Vec::new();
			let mut seen = std::collections::BTreeSet::new();
			for (hid, h) in &self.handles {
				if h.node_id == source_node_id || !self.handle_effectively_visible(hid.as_str()) {
					continue;
				}
				if self.handle_has_incident_edge(hid.as_str()) {
					continue;
				}
				if !self.handles_link_compatible_for_drag(source, h) {
					continue;
				}
				if !self.nodes.get(&h.node_id).is_some_and(|n| n.visible) {
					continue;
				}
				if seen.insert(h.node_id.clone()) {
					out.push(h.node_id.clone());
				}
			}
			out.sort();
			out
		}

		/// @emoji 🧭 Count of visible free handles on `node_id` compatible with `source_handle_id`.
		fn link_compatible_handle_count_on_node(&self, source_handle_id: &str, node_id: &str) -> usize {
			let Some(source) = self.handles.get(source_handle_id) else {
				return 0;
			};
			if source.node_id == node_id {
				return 0;
			}
			self.handles
				.iter()
				.filter(|(id, h)| {
					h.node_id == node_id
						&& self.handle_eligible_link_target_ring(id.as_str(), source_handle_id)
						&& self.handles_link_compatible_for_drag(source, h)
				})
				.count()
		}

		/// @emoji 🧭 Free compatible handle ids on `node_id` for an active link from `source_handle_id`.
		fn link_compatible_handle_ids_on_node(&self, source_handle_id: &str, node_id: &str) -> Vec<String> {
			let Some(source) = self.handles.get(source_handle_id) else {
				return Vec::new();
			};
			let mut out: Vec<String> = self
				.handles
				.iter()
				.filter_map(|(id, h)| {
					if h.node_id != node_id {
						return None;
					}
					if !self.handle_eligible_link_target_ring(id.as_str(), source_handle_id) {
						return None;
					}
					self.handles_link_compatible_for_drag(source, h).then(|| id.clone())
				})
				.collect();
			out.sort();
			out
		}

		/// @emoji 🧭 Compatible target node under `world` while a link wire is active (node body hit).
		fn link_drag_ring_target_node_id(&self, source_handle_id: &str, world: Point) -> Option<String> {
			let nid = self.resolve_node_hit_world(world)?;
			if self.handles.get(source_handle_id)?.node_id == nid {
				return None;
			}
			self.node_has_any_free_link_compatible_handle(source_handle_id, nid.as_str())
				.then_some(nid)
		}

		/// @emoji 🧭 Resolves which single node draws the overview/normal indirect handle ring when that node has **more than one** eligible free handles (otherwise the sole handle is implicit).
		fn indirect_ring_node_id(&self, lod: BoardDrawLod) -> Option<String> {
			if !matches!(lod, BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Normal) {
				return None;
			}
			if let Interaction::LinkTargetNode {
				source_id,
				target_node_id,
			} = &self.interaction
			{
				if self.link_compatible_handle_count_on_node(source_id, target_node_id) > 1 {
					return self.nodes.get(target_node_id).filter(|n| n.visible).map(|n| n.id.clone());
				}
				return None;
			}
			if let Interaction::LinkDragSnap {
				source_id,
				end_world,
				..
			} = &self.interaction
			{
				let ring_nid = self.link_drag_ring_target_node_id(source_id, *end_world)?;
				if self.link_compatible_handle_count_on_node(source_id, ring_nid.as_str()) > 1 {
					return Some(ring_nid);
				}
				return None;
			}
			if self.active_link_source_handle_id().is_some() {
				return None;
			}
			let ring_nid = if self.selection.len() == 1 {
				self.selection.iter().next()?.clone()
			} else {
				return None;
			};
			let n = self.nodes.get(&ring_nid).filter(|n| n.visible)?;
			if self.eligible_indirect_handle_count_on_node(n.id.as_str()) > 1 {
				Some(ring_nid)
			} else {
				None
			}
		}

		fn eligible_indirect_handle_count_on_node(&self, node_id: &str) -> usize {
			self.handles
				.iter()
				.filter(|(id, h)| {
					h.node_id == node_id && self.handle_effectively_visible(id.as_str()) && self.handle_eligible_indirect_connect_ring(id.as_str())
				})
				.count()
		}

		/// @emoji 🧭 Returns the handle id when `node_id` has exactly one visible free indirect-eligible handle.
		fn sole_eligible_indirect_handle_on_node(&self, node_id: &str) -> Option<String> {
			let mut found: Option<String> = None;
			for (id, h) in &self.handles {
				if h.node_id != node_id || !self.handle_effectively_visible(id.as_str()) || !self.handle_eligible_indirect_connect_ring(id.as_str()) {
					continue;
				}
				if found.is_some() {
					return None;
				}
				found = Some(id.clone());
			}
			found
		}

		/// @emoji 🧭 When the drop target has exactly one free handle compatible with `source_handle_id`, returns that handle id (otherwise `None`).
		fn node_sole_free_link_compatible_handle(&self, source_handle_id: &str, target_node_id: &str) -> Option<String> {
			let source = self.handles.get(source_handle_id)?;
			if source.node_id == target_node_id {
				return None;
			}
			let mut found: Option<String> = None;
			for (id, h) in &self.handles {
				if h.node_id != target_node_id || !self.handle_effectively_visible(id.as_str()) {
					continue;
				}
				if self.handle_has_incident_edge(id.as_str()) {
					continue;
				}
				if !self.handles_link_compatible_for_drag(source, h) {
					continue;
				}
				if found.is_some() {
					return None;
				}
				found = Some(id.clone());
			}
			found
		}

		fn point_in_node_world(&self, n: &NodeData, point: Point) -> bool {
			match n.shape {
				NodeShape::Rectangle => {
					let hw = self.scaled_node_width(n) / 2.0;
					let hh = self.scaled_node_height(n) / 2.0;
					(point.x - n.x).abs() <= hw && (point.y - n.y).abs() <= hh
				}
				NodeShape::Circle => distance_between(point, Point::new(n.x, n.y)) <= self.scaled_node_radius(n),
			}
		}

		fn sole_indirect_handle_hit_link_target(&self, point: Point) -> Option<String> {
			let Interaction::LinkTargetNode {
				source_id,
				target_node_id,
			} = &self.interaction
			else {
				return None;
			};
			let th = self.node_sole_free_link_compatible_handle(source_id, target_node_id)?;
			let n = self.nodes.get(target_node_id)?;
			if !n.visible {
				return None;
			}
			if !self.point_in_node_world(n, point) {
				return None;
			}
			Some(th)
		}

		fn sole_indirect_handle_hit_idle_selected_node(&self, point: Point) -> Option<String> {
			if !matches!(self.interaction, Interaction::None) {
				return None;
			}
			if self.selection.len() != 1 {
				return None;
			}
			let nid = self.selection.iter().next()?;
			if !self.nodes.contains_key(nid) {
				return None;
			}
			let sole = self.sole_eligible_indirect_handle_on_node(nid)?;
			let n = self.nodes.get(nid)?;
			if !n.visible {
				return None;
			}
			if !self.point_in_node_world(n, point) {
				return None;
			}
			Some(sole)
		}

		/// @emoji 🧭 True when `target_node_id` hosts at least one visible free handle that can pair with `source_handle_id` under link-compat rules.
		fn node_has_any_free_link_compatible_handle(&self, source_handle_id: &str, target_node_id: &str) -> bool {
			let Some(source) = self.handles.get(source_handle_id) else {
				return false;
			};
			if source.node_id == target_node_id {
				return false;
			}
			for (hid, h) in &self.handles {
				if h.node_id != target_node_id || !self.handle_effectively_visible(hid.as_str()) {
					continue;
				}
				if self.handle_has_incident_edge(hid.as_str()) {
					continue;
				}
				if self.handles_link_compatible_for_drag(source, h) {
					return true;
				}
			}
			false
		}

		/// @emoji 💫 True when the handle may appear on a link-target ghost ring (`overview`/`normal` LOD).
		fn handle_eligible_link_target_ring(&self, handle_id: &str, source_handle_id: &str) -> bool {
			if !self.handle_effectively_visible(handle_id) || self.handle_has_incident_edge(handle_id) {
				return false;
			}
			let Some(source) = self.handles.get(source_handle_id) else {
				return false;
			};
			let Some(target) = self.handles.get(handle_id) else {
				return false;
			};
			if source.node_id == target.node_id {
				return false;
			}
			self.handles_link_compatible_for_drag(source, target)
		}

		fn indirect_ring_handle_eligible(&self, handle_id: &str, ring_node_id: &str) -> bool {
			if self.handles.get(handle_id).is_none_or(|h| h.node_id != ring_node_id) {
				return false;
			}
			if let Some(source_id) = self.active_link_source_handle_id() {
				self.handle_eligible_link_target_ring(handle_id, source_id)
			} else {
				self.handle_eligible_indirect_connect_ring(handle_id)
			}
		}

		fn link_drag_target_ring_hit(&self, source_id: &str, point: Point) -> Option<String> {
			if !matches!(
				self.current_draw_lod(),
				BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Normal
			) {
				return None;
			}
			let node_id = self.link_drag_ring_target_node_id(source_id, point)?;
			if self.link_compatible_handle_count_on_node(source_id, node_id.as_str()) <= 1 {
				return None;
			}
			let zoom = self.camera.zoom;
			for h in self.handles.values().rev() {
				if h.node_id != node_id || !self.handle_eligible_link_target_ring(h.id.as_str(), source_id) {
					continue;
				}
				let Some(pos) = self.indirect_handle_world_pos(h) else { continue };
				let tol = (HANDLE_HIT_TOLERANCE_PX / zoom) + self.indirect_handle_marker_radius_world(h);
				if distance_between(point, pos) <= tol {
					return Some(h.id.clone());
				}
			}
			None
		}

		fn link_target_ring_snapshot(&self, source_handle_id: &str) -> (Option<String>, Vec<String>) {
			let node_id = match &self.interaction {
				Interaction::LinkTargetNode { target_node_id, .. } => Some(target_node_id.clone()),
				Interaction::LinkDragSnap { end_world, .. } => {
					self.link_drag_ring_target_node_id(source_handle_id, *end_world)
				}
				_ => None,
			};
			let Some(nid) = node_id else {
				return (None, Vec::new());
			};
			if self.link_compatible_handle_count_on_node(source_handle_id, nid.as_str()) <= 1 {
				return (None, Vec::new());
			}
			(
				Some(nid.clone()),
				self.link_compatible_handle_ids_on_node(source_handle_id, nid.as_str()),
			)
		}

		fn sync_link_gesture_events(&mut self) {
			let Some(source) = self.active_link_source_handle_id().map(str::to_string) else {
				self.clear_link_gesture_events();
				return;
			};
			let node_ids = self.link_drag_compatible_target_node_ids(&source);
			let compat_key = format!("{}|{}", source, node_ids.join(","));
			if self.link_compat_nodes_emit_key.as_deref() != Some(compat_key.as_str()) {
				self.link_compat_nodes_emit_key = Some(compat_key);
				self.push_event(
					"linkCompatibleNodes",
					json!({ "source": source, "nodeIds": node_ids }),
				);
			}
			let (ring_node_id, ring_handle_ids) = self.link_target_ring_snapshot(&source);
			let ring_key = format!(
				"{}|{}|{}",
				source,
				ring_node_id.as_deref().unwrap_or(""),
				ring_handle_ids.join(",")
			);
			if self.link_target_ring_emit_key.as_deref() != Some(ring_key.as_str()) {
				self.link_target_ring_emit_key = Some(ring_key);
				self.push_event(
					"linkTargetRing",
					json!({
						"source": source,
						"nodeId": ring_node_id,
						"handleIds": ring_handle_ids,
					}),
				);
			}
		}

		fn clear_link_gesture_events(&mut self) {
			if self.link_compat_nodes_emit_key.take().is_some() {
				self.push_event("linkCompatibleNodes", json!({ "source": "", "nodeIds": [] }));
			}
			if self.link_target_ring_emit_key.take().is_some() {
				self.push_event(
					"linkTargetRing",
					json!({ "source": "", "nodeId": null, "handleIds": [] }),
				);
			}
		}

		fn node_center_world(&self, node_id: &str) -> Option<Point> {
			let n = self.nodes.get(node_id)?;
			Some(Point::new(n.x, n.y))
		}

		fn edge_curve(&self, e: &EdgeData) -> Option<CubicBez> {
			let source_handle = self.handles.get(&e.source)?;
			let target_handle = self.handles.get(&e.target)?;
			let source_node = self.nodes.get(&source_handle.node_id)?;
			let target_node = self.nodes.get(&target_handle.node_id)?;
			let source_pos = self.handle_world_pos(source_handle)?;
			let target_pos = self.handle_world_pos(target_handle)?;
			Some(compute_edge_bezier_points(
				source_pos,
				target_pos,
				Point::new(source_node.x, source_node.y),
				Point::new(target_node.x, target_node.y),
			))
		}

		fn link_drag_wire_curve_world(&self, source_id: &str, target_id: Option<&str>, end_world: Point) -> Option<CubicBez> {
			let source_handle = self.handles.get(source_id)?;
			let source_node = self.nodes.get(&source_handle.node_id)?;
			let source_pos = self.handle_world_pos(source_handle)?;
			let source_center = Point::new(source_node.x, source_node.y);
			let (target_pos, target_center) = if let Some(tid) = target_id {
				let th = self.handles.get(tid)?;
				let tn = self.nodes.get(&th.node_id)?;
				(
					self.handle_world_pos(th)?,
					Point::new(tn.x, tn.y),
				)
			} else {
				(end_world, end_world)
			};
			Some(compute_edge_bezier_points(
				source_pos,
				target_pos,
				source_center,
				target_center,
			))
		}

		fn active_link_wire_curve(&self) -> Option<CubicBez> {
			match &self.interaction {
				Interaction::LinkDragSnap {
					source_id,
					target_id,
					end_world,
				} => self.link_drag_wire_curve_world(source_id.as_str(), target_id.as_deref(), *end_world),
				Interaction::LinkTargetNode {
					source_id,
					target_node_id,
				} => self.link_drag_wire_curve_world(source_id.as_str(), None, self.node_center_world(target_node_id)?),
				Interaction::DragNodes {
					proximity_pair: Some((src, tgt)),
					..
				} => self.link_drag_wire_curve_world(src.as_str(), Some(tgt.as_str()), Point::ZERO),
				_ => None,
			}
		}

		fn wire_curve(&self, w: &WireData) -> Option<CubicBez> {
			let end_world = match (&w.target, w.end_x, w.end_y) {
				(None, Some(x), Some(y)) if x.is_finite() && y.is_finite() => Point::new(x, y),
				(Some(tid), _, _) => {
					self.handles.get(tid)?;
					return self.edge_curve(&EdgeData {
						id: w.id.clone(),
						source: w.source.clone(),
						target: tid.clone(),
						selected: w.selected,
						visible: w.visible,
						style: w.style.clone(),
						edge_kind: String::new(),
					});
				}
				_ => return None,
			};
			self.link_drag_wire_curve_world(w.source.as_str(), None, end_world)
		}

		fn apply_link_drag_snap_hover(&mut self, _source_handle_id: &str, world: Point, target_handle_id: Option<&str>) {
			if let Some(tid) = target_handle_id {
				self.set_hovered_id(Some(tid.to_string()));
			} else {
				self.update_hover_from_world(world);
			}
		}

		/// @emoji 🧭 Minimap/overview LOD: group selection and bounded drag only — no per-node/edge/handle picks.
		fn lod_disables_discrete_pick(&self) -> bool {
			matches!(self.current_draw_lod(), BoardDrawLod::Minimap | BoardDrawLod::Overview)
		}

		/// @emoji 🔗 Overview LOD: tight world-radius hit on a free handle so link drag can start without enabling broad `resolve_hit_world` handle picks.
		fn resolve_overview_free_link_handle_pointer_world(&self, point: Point) -> Option<String> {
			if !matches!(self.current_draw_lod(), BoardDrawLod::Overview) {
				return None;
			}
			if !self.selection_options.select_handles {
				return None;
			}
			const MAX_D_WORLD: f64 = 2.25;
			let mut best: Option<(f64, String)> = None;
			for h in self.handles.values() {
				if !self.handle_effectively_visible(h.id.as_str()) || self.handle_has_incident_edge(h.id.as_str()) {
					continue;
				}
				let Some(pos) = self.handle_world_pos(h) else {
					continue;
				};
				let d = distance_between(point, pos);
				if d <= MAX_D_WORLD && best.as_ref().map(|(bd, _)| d < *bd).unwrap_or(true) {
					best = Some((d, h.id.clone()));
				}
			}
			best.map(|(_, id)| id)
		}

		/// @emoji 🧭 Minimap/overview LOD: pointer-down inside the selection AABB moves the group without a discrete hit.
		fn lod_uses_bounded_drag(&self) -> bool {
			matches!(self.current_draw_lod(), BoardDrawLod::Minimap | BoardDrawLod::Overview)
		}

		fn resolve_hover_world(&self, point: Point) -> Option<String> {
			let lod = self.current_draw_lod();
			let zoom = self.camera.zoom;
			if !matches!(lod, BoardDrawLod::Minimap) {
				if matches!(lod, BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Normal) {
					if let Some(hid) = self.sole_indirect_handle_hit_link_target(point) {
						return Some(hid);
					}
					if let Interaction::LinkDragSnap { source_id, .. } = &self.interaction {
						if let Some(hid) = self.link_drag_target_ring_hit(source_id, point) {
							return Some(hid);
						}
					}
				}
				if let Some(ring_node_id) = self.indirect_ring_node_id(lod) {
					for h in self.handles.values().rev() {
						if h.node_id != ring_node_id || !self.handle_effectively_visible(h.id.as_str()) {
							continue;
						}
						if !self.indirect_ring_handle_eligible(h.id.as_str(), ring_node_id.as_str()) {
							continue;
						}
						let Some(pos) = self.indirect_handle_world_pos(h) else { continue };
						let tol = (HANDLE_HIT_TOLERANCE_PX / zoom) + self.indirect_handle_marker_radius_world(h);
						if distance_between(point, pos) <= tol {
							return Some(h.id.clone());
						}
					}
				}
				if matches!(lod, BoardDrawLod::Normal | BoardDrawLod::Detail | BoardDrawLod::Micro) {
					for h in self.handles.values().rev() {
						if !self.handle_effectively_visible(h.id.as_str()) {
							continue;
						}
						let Some(pos) = self.handle_world_pos(h) else { continue };
						let tol = (HANDLE_HIT_TOLERANCE_PX / zoom) + self.effective_handle_radius(h);
						if distance_between(point, pos) <= tol {
							return Some(h.id.clone());
						}
					}
				}
				if matches!(lod, BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Normal) {
					if let Some(hid) = self.sole_indirect_handle_hit_idle_selected_node(point) {
						return Some(hid);
					}
				}
			}
			for n in self.nodes.values().rev() {
				if !n.visible {
					continue;
				}
				match n.shape {
					NodeShape::Rectangle => {
						let hw = self.scaled_node_width(n) / 2.0;
						let hh = self.scaled_node_height(n) / 2.0;
						if (point.x - n.x).abs() <= hw && (point.y - n.y).abs() <= hh {
							return Some(n.id.clone());
						}
					}
					NodeShape::Circle => {
						if distance_between(point, Point::new(n.x, n.y)) <= self.scaled_node_radius(n) {
							return Some(n.id.clone());
						}
					}
				}
			}
			for w in self.wires.values().rev() {
				if !self.wire_effectively_visible(w) {
					continue;
				}
				if let Some(c) = self.wire_curve(w) {
					if distance_point_to_cubic_bezier(point, c, 18) <= EDGE_HIT_TOLERANCE_PX / zoom {
						return Some(w.id.clone());
					}
				}
			}
			for e in self.edges.values().rev() {
				if !self.edge_effectively_visible(e) {
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

		pub fn resolve_hit_world(&self, point: Point) -> Option<String> {
			if self.lod_disables_discrete_pick() {
				return None;
			}
			let zoom = self.camera.zoom;
			let o = &self.selection_options;
			if o.select_handles {
				if matches!(
					self.current_draw_lod(),
					BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Normal
				) {
					if let Some(hid) = self.sole_indirect_handle_hit_link_target(point) {
						return Some(hid);
					}
					if let Interaction::LinkDragSnap { source_id, .. } = &self.interaction {
						if let Some(hid) = self.link_drag_target_ring_hit(source_id, point) {
							return Some(hid);
						}
					}
				}
				if let Some(ring_node_id) = self.indirect_ring_node_id(self.current_draw_lod()) {
					for h in self.handles.values().rev() {
						if h.node_id != ring_node_id || !self.handle_effectively_visible(h.id.as_str()) {
							continue;
						}
						if !self.indirect_ring_handle_eligible(h.id.as_str(), ring_node_id.as_str()) {
							continue;
						}
						let Some(pos) = self.indirect_handle_world_pos(h) else { continue };
						let tol = (HANDLE_HIT_TOLERANCE_PX / zoom) + self.indirect_handle_marker_radius_world(h);
						if distance_between(point, pos) <= tol {
							return Some(h.id.clone());
						}
					}
				}
				if matches!(
					self.current_draw_lod(),
					BoardDrawLod::Normal | BoardDrawLod::Detail | BoardDrawLod::Micro
				) {
					for h in self.handles.values().rev() {
						if !self.handle_effectively_visible(h.id.as_str()) {
							continue;
						}
						let Some(pos) = self.handle_world_pos(h) else { continue };
						let tol = (HANDLE_HIT_TOLERANCE_PX / zoom) + self.effective_handle_radius(h);
						if distance_between(point, pos) <= tol {
							return Some(h.id.clone());
						}
					}
				}
				if matches!(
					self.current_draw_lod(),
					BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Normal
				) {
					if let Some(hid) = self.sole_indirect_handle_hit_idle_selected_node(point) {
						return Some(hid);
					}
				}
			}
			if o.select_nodes {
				for n in self.nodes.values().rev() {
					if !n.visible {
						continue;
					}
					match n.shape {
						NodeShape::Rectangle => {
							let hw = self.scaled_node_width(n) / 2.0;
							let hh = self.scaled_node_height(n) / 2.0;
							if (point.x - n.x).abs() <= hw && (point.y - n.y).abs() <= hh {
								return Some(n.id.clone());
							}
						}
						NodeShape::Circle => {
							if distance_between(point, Point::new(n.x, n.y)) <= self.scaled_node_radius(n) {
								return Some(n.id.clone());
							}
						}
					}
				}
			}
			if o.select_edges {
				for e in self.edges.values().rev() {
					if !self.edge_effectively_visible(e) {
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

		fn resolve_node_hit_world(&self, point: Point) -> Option<String> {
			for n in self.nodes.values().rev() {
				if !n.visible {
					continue;
				}
				match n.shape {
					NodeShape::Rectangle => {
						let hw = self.scaled_node_width(n) / 2.0;
						let hh = self.scaled_node_height(n) / 2.0;
						if (point.x - n.x).abs() <= hw && (point.y - n.y).abs() <= hh {
							return Some(n.id.clone());
						}
					}
					NodeShape::Circle => {
						if distance_between(point, Point::new(n.x, n.y)) <= self.scaled_node_radius(n) {
							return Some(n.id.clone());
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
				"replace" => {
					next.clear();
					next.insert(hit_id.to_string());
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

		fn pick_merge_mode_for_modifiers(ctrl_or_meta: bool, shift: bool, option_mode: &str) -> String {
			if ctrl_or_meta && shift {
				return "invertive".into();
			}
			if ctrl_or_meta {
				return "subtractive".into();
			}
			if shift {
				return "additive".into();
			}
			option_mode.to_string()
		}

		pub fn sync_descriptor(&mut self, desc: &SceneDescriptorJson) -> Result<(), String> {
			if matches!(
				self.interaction,
				Interaction::LinkAtSourceHandle { .. } | Interaction::LinkDragSnap { .. } | Interaction::LinkTargetNode { .. }
			) {
				self.interaction = Interaction::None;
				self.clear_link_gesture_events();
			}
			let want_nodes: BTreeSet<_> = desc.nodes.iter().map(|n| n.id.clone()).collect();
			let want_handles: BTreeSet<_> = desc.handles.iter().map(|h| h.id.clone()).collect();
			let want_edges: BTreeSet<_> = desc.edges.iter().map(|e| e.id.clone()).collect();
			let want_wires: BTreeSet<_> = desc.wires.iter().map(|w| w.id.clone()).collect();
			self.edges.retain(|id, _| want_edges.contains(id));
			self.wires.retain(|id, _| want_wires.contains(id));
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
				let node_kind = n
					.node_kind
					.as_ref()
					.map(|s| s.trim().to_string())
					.filter(|s| !s.is_empty())
					.unwrap_or_default();
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
						scale: n.scale.filter(|v| v.is_finite() && *v > 0.0).unwrap_or(1.0),
						draggable: n.draggable.unwrap_or(true),
						selected: n.selected.unwrap_or(false),
						visible: n.visible.unwrap_or(true),
						root: n.root.unwrap_or(false),
						style: n.style.clone(),
						text: n.text.clone(),
						icon_kind: n.icon_kind.clone(),
						node_kind,
					},
				);
			}
			for h in &desc.handles {
				let kind = h
					.handle_kind
					.as_deref()
					.unwrap_or("")
					.trim()
					.to_string();
				let color_fill = match h.color.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty()) {
					None => None,
					Some(s) => Some(
						Self::parse_css_color(s)
							.ok_or_else(|| format!("invalid color on handle {}: {s:?}", h.id))?,
					),
				};
				let icon_kind = h
					.icon_kind
					.as_ref()
					.map(|s| s.trim())
					.filter(|s| !s.is_empty())
					.map(|s| s.to_string());
				self.handles.insert(
					h.id.clone(),
					HandleData {
						id: h.id.clone(),
						node_id: h.node_id.clone(),
						angle: h.angle,
						radius: h.radius.unwrap_or(8.0),
						scale: h.scale.filter(|v| v.is_finite() && *v > 0.0).unwrap_or(1.0),
						selected: h.selected.unwrap_or(false),
						visible: h.visible.unwrap_or(true),
						style: h.style.clone(),
						handle_kind: kind,
						color_fill,
						icon_kind,
					},
				);
			}
			for e in &desc.edges {
				let existed = self.edges.contains_key(&e.id);
				let edge_kind = e
					.edge_kind
					.as_ref()
					.map(|s| s.trim().to_string())
					.filter(|s| !s.is_empty())
					.unwrap_or_default();
				self.edges.insert(
					e.id.clone(),
					EdgeData {
						id: e.id.clone(),
						source: e.source.clone(),
						target: e.target.clone(),
						selected: e.selected.unwrap_or(false),
						visible: e.visible.unwrap_or(true),
						style: e.style.clone(),
						edge_kind,
					},
				);
				if !existed {
					self.push_event(
						"edgeCreate",
						json!({ "id": e.id, "source": e.source, "target": e.target }),
					);
				}
			}
			for w in &desc.wires {
				let target = w
					.target
					.as_ref()
					.map(|s| s.trim().to_string())
					.filter(|s| !s.is_empty());
				let (end_x, end_y) = match &target {
					Some(_) => (None, None),
					None => {
						let x = match w.end_x {
							Some(v) if v.is_finite() => Some(v),
							_ => None,
						};
						let y = match w.end_y {
							Some(v) if v.is_finite() => Some(v),
							_ => None,
						};
						if x.is_none() || y.is_none() {
							continue;
						}
						(x, y)
					}
				};
				let wire_kind = w
					.wire_kind
					.as_ref()
					.map(|s| s.trim().to_string())
					.filter(|s| !s.is_empty())
					.or_else(|| {
						self.handles
							.get(w.source.as_str())
							.map(|h| self.resolve_default_wire_kind_for_handle(h))
					})
					.unwrap_or_else(|| BOARD_DEFAULT_WIRE_KIND_ID.to_string());
				self.wires.insert(
					w.id.clone(),
					WireData {
						id: w.id.clone(),
						source: w.source.clone(),
						target,
						end_x,
						end_y,
						selected: w.selected.unwrap_or(false),
						visible: w.visible.unwrap_or(true),
						style: w.style.clone(),
						wire_kind,
					},
				);
			}
			if !self.is_preselect_active() {
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
				for w in &desc.wires {
					if w.selected == Some(true) {
						new_selection.insert(w.id.clone());
					}
				}
				let prev_sel = self.selection.clone();
				if prev_sel != new_selection {
					self.selection_exit_highlight.clear();
				}
				self.selection = new_selection;
				if prev_sel != self.selection {
					self.push_select_event();
				}
			}
			self.sync_selection_flags_to_objects();
			Ok(())
		}

		pub fn clear_scene(&mut self) {
			self.edges.clear();
			self.wires.clear();
			self.handles.clear();
			self.nodes.clear();
			self.selection.clear();
			self.preselect.clear();
			self.preselect_removed.clear();
			self.selection_exit_highlight.clear();
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
					.map(str::trim)
					.filter(|s| !s.is_empty())
					.map(String::from);
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
					let handle_kind = ho
						.get("handleKind")
						.or_else(|| ho.get("handle_kind"))
						.and_then(|v| v.as_str())
						.map(str::trim)
						.filter(|s| !s.is_empty())
						.map(String::from)
						.unwrap_or_else(|| "board.port".into());
					let handle_color = ho
						.get("color")
						.and_then(|v| v.as_str())
						.map(str::trim)
						.filter(|s| !s.is_empty())
						.map(String::from);
					let handle_icon_kind = ho
						.get("iconKind")
						.and_then(|v| v.as_str())
						.map(str::trim)
						.filter(|s| !s.is_empty())
						.map(|s| s.to_string());
					let handle_scale = ho
						.get("scale")
						.and_then(|v| v.as_f64())
						.filter(|v| v.is_finite() && *v > 0.0);
					handles.push(HandleDescJson {
						id: hid.into(),
						node_id: id.into(),
						angle,
						radius: None,
						scale: handle_scale,
						selected: None,
						style: None,
						handle_kind: Some(handle_kind),
						color: handle_color,
						icon_kind: handle_icon_kind,
						user_data: None,
						visible: board_json_visible_option(ho),
					});
				}
				let shape_str = obj.get("shape").and_then(|v| v.as_str());
				let fixture_node_kind = obj
					.get("nodeKind")
					.or_else(|| obj.get("node_kind"))
					.and_then(|v| v.as_str())
					.map(str::trim)
					.filter(|s| !s.is_empty())
					.map(|s| s.to_string());
				let fixture_node_scale = obj
					.get("scale")
					.and_then(|v| v.as_f64())
					.filter(|v| v.is_finite() && *v > 0.0);
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
					let root = obj.get("root").and_then(|v| v.as_bool());
					let icon_kind = obj
						.get("iconKind")
						.and_then(|v| v.as_str())
						.map(str::trim)
						.filter(|s| !s.is_empty())
						.map(|s| s.to_string());
					desc.nodes.push(NodeDescJson {
						id: id.into(),
						x,
						y,
						draggable: None,
						selected: None,
						style: None,
						text,
						icon_kind,
						node_kind: fixture_node_kind.clone(),
						user_data: None,
						visible: board_json_visible_option(obj),
						root,
						shape: Some("rectangle".into()),
						radius: None,
						width: Some(width),
						height: Some(height),
						scale: fixture_node_scale,
					});
				} else {
					let Some(radius) = obj.get("radius").and_then(|v| v.as_f64()) else {
						return false;
					};
					if radius <= 0.0 {
						return false;
					}
					let root = obj.get("root").and_then(|v| v.as_bool());
					let icon_kind = obj
						.get("iconKind")
						.and_then(|v| v.as_str())
						.map(str::trim)
						.filter(|s| !s.is_empty())
						.map(|s| s.to_string());
					desc.nodes.push(NodeDescJson {
						id: id.into(),
						x,
						y,
						draggable: None,
						selected: None,
						style: None,
						text,
						icon_kind,
						node_kind: fixture_node_kind.clone(),
						user_data: None,
						visible: board_json_visible_option(obj),
						root,
						shape: Some("circle".into()),
						radius: Some(radius),
						width: None,
						height: None,
						scale: fixture_node_scale,
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
				let Some((source, target)) = fixture_edge_handle_ids_from_object(e) else {
					return false;
				};
				let edge_kind = e
					.get("edgeKind")
					.or_else(|| e.get("edge_kind"))
					.and_then(|v| v.as_str())
					.map(str::trim)
					.filter(|s| !s.is_empty())
					.map(|s| s.to_string());
				desc.edges.push(EdgeDescJson {
					id: id.into(),
					source: source.into(),
					target: target.into(),
					edge_kind,
					selected: None,
					style: None,
					user_data: None,
					visible: board_json_visible_option(e),
				});
			}
			if self.sync_descriptor(&desc).is_err() {
				return false;
			}
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
			let pad = self.drawable_cull_pad_world() + self.effective_handle_radius(h).max(1.0);
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

		fn indirect_handle_world_bounds_cull(&self, h: &HandleData) -> Option<WorldBox> {
			let pos = self.indirect_handle_world_pos(h)?;
			let pad = self.drawable_cull_pad_world() + self.indirect_handle_marker_radius_world(h).max(1.0);
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

		fn stroke_world_step_grid(
			&self,
			scene: &mut Scene,
			color: Color,
			stroke_px: f64,
			world_step: f64,
			min_step_screen: f64,
		) {
			let step = world_step * self.camera.zoom;
			if step < min_step_screen {
				return;
			}
			let stroke = Stroke::new(stroke_px);
			let w = self.width as f64;
			let h = self.height as f64;
			let origin = self.world_to_screen(Point::new(0.0, 0.0));
			let x_off = ((origin.x % step) + step) % step;
			let y_off = ((origin.y % step) + step) % step;
			let mut p = crate::vello::kurbo::BezPath::new();
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
			scene.stroke(&stroke, Affine::IDENTITY, color, None, &p);
		}

		fn append_handle_marker(
			&self,
			scene: &mut Scene,
			h: &HandleData,
			center: Point,
			radius_world: f64,
			draw_icon: bool,
			style_kind: BoardElementStyleKind,
			paint_override: Option<(Color, Color, f64)>,
		) {
			let c = self.world_to_screen(center);
			let r = (radius_world * self.camera.zoom).max(1.0);
			let circle = Circle::new(c, r);
			let (fill, stroke_c, stroke_px) = if let Some((f, s, sw)) = paint_override {
				(f, s, sw)
			} else {
				(
					self.resolve_handle_fill_color(h, &self.vello_theme, style_kind),
					self.resolve_handle_stroke_color(h, &self.vello_theme, style_kind),
					2.0_f64,
				)
			};
			scene.fill(Fill::NonZero, Affine::IDENTITY, fill, None, &circle);
			scene.stroke(&Stroke::new(stroke_px), Affine::IDENTITY, stroke_c, None, &circle);
			if draw_icon {
				if let Some(k) = h.icon_kind.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty()) {
					let preserve_original_style = self.preserve_original_element_style || style_kind == BoardElementStyleKind::Original;
					if let Some((bx, by, bw, bh, body)) = self.get_or_build_icon_paint(k, stroke_c, fill, preserve_original_style) {
						let fit_inset = 0.62;
						let s = radius_world * self.camera.zoom * fit_inset;
						let cx = bx + bw * 0.5;
						let cy = by + bh * 0.5;
						let avail = 2.0 * s;
						let scale = (avail / bw).min(avail / bh);
						let aff = Affine::translate((c.x - scale * cx, c.y - scale * cy)) * Affine::scale(scale);
						let r_clip = (radius_world * self.camera.zoom * 0.82).max(1.0);
						let disc = Circle::new(c, r_clip);
						scene.push_clip_layer(Fill::NonZero, Affine::IDENTITY, &disc);
						match &body {
							CachedIconBody::Vector(icon_scene) => {
								scene.append(icon_scene, Some(aff));
							}
							CachedIconBody::Raster(img) => {
								scene.draw_image(&ImageBrush::new((**img).clone()), aff);
							}
						}
						scene.pop_layer();
					}
				}
			}
		}

		fn append_indirect_handle_ring(&self, scene: &mut Scene, tile_filter: Option<&WorldBox>, node_id: &str) {
			for h in self.handles.values() {
				if h.node_id != node_id || !self.handle_effectively_visible(h.id.as_str()) {
					continue;
				}
				if !self.indirect_ring_handle_eligible(h.id.as_str(), node_id) {
					continue;
				}
				if let Some(tb) = tile_filter {
					let Some(hb) = self.indirect_handle_world_bounds_cull(h) else { continue };
					if !world_boxes_overlap(*tb, hb) {
						continue;
					}
				}
				let Some(wp) = self.indirect_handle_world_pos(h) else { continue };
				let style_kind = self.resolve_handle_style_kind(h);
				let stroke_px = 2.0_f64;
				let paint_override = if matches!(style_kind, BoardElementStyleKind::Original | BoardElementStyleKind::Neutral) {
					Some((
						self.vello_theme.indirect_handle_fill,
						self.vello_theme.indirect_handle_stroke,
						stroke_px,
					))
				} else {
					None
				};
				self.append_handle_marker(
					scene,
					h,
					wp,
					self.indirect_handle_marker_radius_world(h),
					false,
					style_kind,
					paint_override,
				);
			}
		}

		fn append_nodes_handles_edges(&self, scene: &mut Scene, tile_filter: Option<&WorldBox>, lod: BoardDrawLod) {
			let pad = self.drawable_cull_pad_world();
			let draw_handles = matches!(lod, BoardDrawLod::Normal | BoardDrawLod::Detail | BoardDrawLod::Micro);
			let draw_node_icons = matches!(lod, BoardDrawLod::Detail | BoardDrawLod::Micro);
			let draw_handle_icons = lod == BoardDrawLod::Micro;
			let indirect_ring_node_id = self.indirect_ring_node_id(lod);
			let link_source = self.active_link_source_handle_id().map(str::to_string);
			let link_compat_nodes: std::collections::BTreeSet<String> = link_source
				.as_ref()
				.map(|s| {
					self.link_drag_compatible_target_node_ids(s)
						.into_iter()
						.collect()
				})
				.unwrap_or_default();
			for n in self.nodes.values() {
				if !n.visible {
					continue;
				}
				if let Some(tb) = tile_filter {
					if !world_boxes_overlap(*tb, self.node_world_bounds(n, pad)) {
						continue;
					}
				}
				let link_compat = link_compat_nodes.contains(&n.id);
				let resolved_style_kind = self.resolve_node_style_kind(n);
				let style_kind = if link_compat
					&& matches!(resolved_style_kind, BoardElementStyleKind::Original | BoardElementStyleKind::Neutral)
				{
					BoardElementStyleKind::Highlighted
				} else {
					resolved_style_kind
				};
				let draw_node_stroke = lod != BoardDrawLod::Minimap
					|| !matches!(style_kind, BoardElementStyleKind::Original | BoardElementStyleKind::Neutral);
				let stroke_c = Self::node_stroke_for_style(&self.vello_theme, style_kind);
				let fill = if lod == BoardDrawLod::Minimap
					&& matches!(style_kind, BoardElementStyleKind::Original | BoardElementStyleKind::Neutral)
				{
					stroke_c
				} else {
					Self::node_fill_for_style(&self.vello_theme, style_kind)
				};
				let sw = 2.0_f64;
				match n.shape {
					NodeShape::Circle => {
						let c = self.world_to_screen(Point::new(n.x, n.y));
						let r = (self.scaled_node_radius(n) * self.camera.zoom).max(1.0);
						let circle = Circle::new(c, r);
						scene.fill(Fill::NonZero, Affine::IDENTITY, fill, None, &circle);
						if draw_node_stroke {
							scene.stroke(&Stroke::new(sw), Affine::IDENTITY, stroke_c, None, &circle);
						}
					}
					NodeShape::Rectangle => {
						let hw = self.scaled_node_width(n) / 2.0;
						let hh = self.scaled_node_height(n) / 2.0;
						let p0 = self.world_to_screen(Point::new(n.x - hw, n.y - hh));
						let p1 = self.world_to_screen(Point::new(n.x + hw, n.y + hh));
						let r = Rect::from_points(p0, p1);
						scene.fill(Fill::NonZero, Affine::IDENTITY, fill, None, &r);
						if draw_node_stroke {
							scene.stroke(&Stroke::new(sw), Affine::IDENTITY, stroke_c, None, &r);
						}
					}
				}
				if draw_node_icons {
					if let Some(k) = n.icon_kind.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty()) {
						let preserve_original_style = self.preserve_original_element_style || style_kind == BoardElementStyleKind::Original;
						if let Some((bx, by, bw, bh, body)) = self.get_or_build_icon_paint(k, stroke_c, fill, preserve_original_style) {
							let clip_inset = 0.88;
							let fit_inset = 0.76;
							let (sx_half, sy_half) = match n.shape {
								NodeShape::Circle => {
									let s = self.scaled_node_radius(n) * self.camera.zoom * fit_inset;
									(s, s)
								}
								NodeShape::Rectangle => (
									self.scaled_node_width(n) * self.camera.zoom * fit_inset * 0.5,
									self.scaled_node_height(n) * self.camera.zoom * fit_inset * 0.5,
								),
							};
							let center = self.world_to_screen(Point::new(n.x, n.y));
							let cx = bx + bw * 0.5;
							let cy = by + bh * 0.5;
							let avail_w = 2.0 * sx_half;
							let avail_h = 2.0 * sy_half;
							let scale = (avail_w / bw).min(avail_h / bh);
							let aff = Affine::translate((center.x - scale * cx, center.y - scale * cy))
								* Affine::scale(scale);
							match n.shape {
								NodeShape::Circle => {
									let r_clip = (self.scaled_node_radius(n) * self.camera.zoom * clip_inset).max(1.0);
									let disc = Circle::new(center, r_clip);
									scene.push_clip_layer(Fill::NonZero, Affine::IDENTITY, &disc);
									match &body {
										CachedIconBody::Vector(icon_scene) => {
											scene.append(icon_scene, Some(aff));
										}
										CachedIconBody::Raster(img) => {
											scene.draw_image(&ImageBrush::new((**img).clone()), aff);
										}
									}
									scene.pop_layer();
								}
								NodeShape::Rectangle => {
									let hw = self.scaled_node_width(n) * self.camera.zoom * clip_inset * 0.5;
									let hh = self.scaled_node_height(n) * self.camera.zoom * clip_inset * 0.5;
									let clip_r = Rect::from_points(
										Point::new(center.x - hw, center.y - hh),
										Point::new(center.x + hw, center.y + hh),
									);
									scene.push_clip_layer(Fill::NonZero, Affine::IDENTITY, &clip_r);
									match &body {
										CachedIconBody::Vector(icon_scene) => {
											scene.append(icon_scene, Some(aff));
										}
										CachedIconBody::Raster(img) => {
											scene.draw_image(&ImageBrush::new((**img).clone()), aff);
										}
									}
									scene.pop_layer();
								}
							}
						}
					}
				}
			}
			for h in self.handles.values() {
				if !draw_handles || !self.handle_effectively_visible(h.id.as_str()) {
					continue;
				}
				if let Some(tb) = tile_filter {
					let Some(hb) = self.handle_world_bounds_cull(h) else { continue };
					if !world_boxes_overlap(*tb, hb) {
						continue;
					}
				}
				let Some(wp) = self.handle_world_pos(h) else { continue };
				let style_kind = self.resolve_handle_style_kind(h);
				self.append_handle_marker(scene, h, wp, self.effective_handle_radius(h), draw_handle_icons, style_kind, None);
			}
			let edge_sw = if lod == BoardDrawLod::Minimap {
				1.12_f64
			} else {
				2.0 * self.camera.zoom.max(0.75)
			};
			let edge_stroke = Stroke::new(edge_sw);
			for e in self.edges.values() {
				if !self.edge_effectively_visible(e) {
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
					let stroke_color = Self::edge_stroke_for_style(&self.vello_theme, self.resolve_edge_style_kind(e));
					scene.stroke(&edge_stroke, Affine::IDENTITY, stroke_color, None, &curve);
				}
			}
			let wire_sw = 2.25_f64;
			let wire_stroke = Stroke::new(wire_sw);
			for w in self.wires.values() {
				if !self.wire_effectively_visible(w) {
					continue;
				}
				if let Some(c) = self.wire_curve(w) {
					let p0 = self.world_to_screen(c.p0);
					let p1 = self.world_to_screen(c.p1);
					let p2 = self.world_to_screen(c.p2);
					let p3 = self.world_to_screen(c.p3);
					let curve = CubicBez::new(p0, p1, p2, p3);
					let wc = Self::wire_stroke_for_style(&self.vello_theme, self.resolve_wire_style_kind(w));
					scene.stroke(&wire_stroke, Affine::IDENTITY, wc, None, &curve);
				}
			}
			if let Some(node_id) = indirect_ring_node_id {
				self.append_indirect_handle_ring(scene, tile_filter, &node_id);
			}
			let link_wire_sw = 2.85_f64;
			let link_wire_stroke = Stroke::new(link_wire_sw);
			let link_wire_color = self.vello_theme.node_stroke;
			if let Some(c) = self.active_link_wire_curve() {
				let p0 = self.world_to_screen(c.p0);
				let p1 = self.world_to_screen(c.p1);
				let p2 = self.world_to_screen(c.p2);
				let p3 = self.world_to_screen(c.p3);
				let curve = CubicBez::new(p0, p1, p2, p3);
				scene.stroke(&link_wire_stroke, Affine::IDENTITY, link_wire_color, None, &curve);
			}
		}

		pub fn build_vector_scene(&self) -> Scene {
			let mut inner = Scene::new();
			let lod = self.current_draw_lod();
			let grid_color = self.vello_theme.grid_minor_stroke;
			if lod != BoardDrawLod::Minimap {
				self.stroke_world_step_grid(&mut inner, grid_color, 1.0, self.grid_step_large_world(), 0.0);
				match lod {
					BoardDrawLod::Normal | BoardDrawLod::Detail | BoardDrawLod::Micro => {
						self.stroke_world_step_grid(&mut inner, grid_color, 0.72, self.grid_step_medium_world(), 0.0);
					}
					BoardDrawLod::Minimap | BoardDrawLod::Overview | BoardDrawLod::Compact => {}
				}
				if matches!(lod, BoardDrawLod::Detail | BoardDrawLod::Micro) {
					self.stroke_world_step_grid(&mut inner, grid_color, 0.48, self.grid_step_small_world(), 0.0);
				}
				if lod == BoardDrawLod::Micro {
					self.stroke_world_step_grid(&mut inner, grid_color, 0.32, self.grid_step_micro_world(), 0.0);
				}
			}
			if let Some(ref pts) = self.selection_screen_preview {
				if pts.len() >= 2 {
					let mut path = crate::vello::kurbo::BezPath::new();
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
					self.append_nodes_handles_edges(&mut inner, None, lod);
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
							self.append_nodes_handles_edges(&mut inner, Some(&tile_box), lod);
							inner.pop_layer();
						}
					}
				}
			} else {
				self.append_nodes_handles_edges(&mut inner, None, lod);
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
			let next = self.resolve_hover_world(world);
			self.set_hovered_id(next);
		}

		pub fn set_hovered_id(&mut self, id: Option<String>) {
			if self.hovered_id == id {
				return;
			}
			self.hovered_id = id.clone();
			self.push_event("hover", json!({ "id": id }));
		}

		/// @emoji 🔇 Updates hover chrome without emitting `hover` (controlled React sync).
		pub fn set_hovered_id_silent(&mut self, id: Option<String>) {
			if self.hovered_id == id {
				return;
			}
			self.hovered_id = id;
		}

		pub fn wheel_screen(&mut self, sx: f64, sy: f64, delta_y: f64) {
			let zoom_factor = if delta_y < 0.0 { 1.1 } else { 0.9 };
			let next_zoom = (self.camera.zoom * zoom_factor).clamp(BOARD_CAMERA_ZOOM_MIN, BOARD_CAMERA_ZOOM_MAX);
			let screen = Point::new(sx, sy);
			let world_before = self.screen_to_world(screen);
			let nx = world_before.x - (sx - self.width as f64 / 2.0) / next_zoom;
			let ny = world_before.y - (sy - self.height as f64 / 2.0) / next_zoom;
			self.set_camera(nx, ny, next_zoom);
			if matches!(self.interaction, Interaction::None) {
				let world = self.screen_to_world(screen);
				self.update_hover_from_world(world);
			}
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
					let wids: Vec<_> = self
						.wires
						.iter()
						.filter(|(_, w)| w.source == *hid || w.target.as_ref() == Some(&hid))
						.map(|(k, _)| k.clone())
						.collect();
					for wid in &wids {
						self.wires.remove(wid);
						self.selection.remove(wid);
					}
					let eids: Vec<_> = self
						.edges
						.iter()
						.filter(|(_, e)| e.source == hid || e.target == hid)
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
			self.selection_exit_highlight.clear();
			self.sync_selection_flags_to_objects();
			self.push_select_event();
		}

		fn link_snap_drag_tolerance_screen(&self, h: &HandleData) -> f64 {
			let z = self.camera.zoom.max(1e-9);
			HANDLE_HIT_TOLERANCE_PX + LINK_HANDLE_SNAP_EXTRA_PX + self.effective_handle_radius(h) * z
		}

		fn link_snap_commit_proximity_ok(&self, target_handle_id: &str, world: Point) -> bool {
			let Some(h) = self.handles.get(target_handle_id) else {
				return false;
			};
			if !self.handle_effectively_visible(target_handle_id) {
				return false;
			}
			let Some(pw) = self.handle_world_pos(h) else {
				return false;
			};
			let z = self.camera.zoom.max(1e-9);
			let d_screen = distance_between(self.world_to_screen(world), self.world_to_screen(pw));
			let tol_commit = HANDLE_HIT_TOLERANCE_PX + LINK_COMMIT_SNAP_TIGHT_PX + self.effective_handle_radius(h) * z;
			d_screen <= tol_commit
		}

		/// @emoji 🔗 True when any edge uses this handle as `source` or `target` (handle already participates in a link).
		fn handle_has_incident_edge(&self, handle_id: &str) -> bool {
			self.edges.values().any(|e| e.source == handle_id || e.target == handle_id)
		}

		fn node_has_any_incident_edge(&self, node_id: &str) -> bool {
			self.handles
				.values()
				.filter(|h| h.node_id == node_id)
				.any(|h| self.handle_has_incident_edge(h.id.as_str()))
		}

		fn lod_allows_node_proximity_connect(&self) -> bool {
			matches!(
				self.current_draw_lod(),
				BoardDrawLod::Normal | BoardDrawLod::Detail | BoardDrawLod::Micro
			)
		}

		/// @emoji 🧲 While dragging a node with no incident edges, overlapping bounds pick the nearest compatible free handle pair.
		fn node_drag_proximity_handle_pair(&self, moving_node_id: &str) -> Option<(String, String)> {
			if !self.lod_allows_node_proximity_connect() {
				return None;
			}
			if !self.node_effectively_visible(moving_node_id) {
				return None;
			}
			if self.node_has_any_incident_edge(moving_node_id) {
				return None;
			}
			let moving = self.nodes.get(moving_node_id)?;
			let moving_bounds = self.node_world_bounds(moving, 0.0);
			let mut best: Option<(f64, String, String)> = None;
			for (target_id, target) in &self.nodes {
				if target_id == moving_node_id || !self.node_effectively_visible(target_id.as_str()) {
					continue;
				}
				let target_bounds = self.node_world_bounds(target, 0.0);
				if !world_boxes_overlap(moving_bounds, target_bounds) {
					continue;
				}
				for (src_id, src_h) in &self.handles {
					if src_h.node_id != moving_node_id
						|| !self.handle_effectively_visible(src_id.as_str())
						|| self.handle_has_incident_edge(src_id.as_str())
					{
						continue;
					}
					let Some(src_pos) = self.handle_world_pos(src_h) else {
						continue;
					};
					for (tgt_id, tgt_h) in &self.handles {
						if tgt_h.node_id != target_id.as_str()
							|| !self.handle_effectively_visible(tgt_id.as_str())
							|| self.handle_has_incident_edge(tgt_id.as_str())
						{
							continue;
						}
						if !self.handles_link_compatible_for_drag(src_h, tgt_h) {
							continue;
						}
						let Some(tgt_pos) = self.handle_world_pos(tgt_h) else {
							continue;
						};
						let d = distance_between(src_pos, tgt_pos);
						if best.as_ref().map(|(bd, _, _)| d < *bd).unwrap_or(true) {
							best = Some((d, src_id.clone(), tgt_id.clone()));
						}
					}
				}
			}
			best.map(|(_, s, t)| (s, t))
		}

		fn node_effectively_visible(&self, node_id: &str) -> bool {
			self.nodes.get(node_id).is_some_and(|n| n.visible)
		}

		fn handle_effectively_visible(&self, handle_id: &str) -> bool {
			self.handles
				.get(handle_id)
				.is_some_and(|h| h.visible && self.node_effectively_visible(h.node_id.as_str()))
		}

		fn edge_effectively_visible(&self, edge: &EdgeData) -> bool {
			edge.visible
				&& self.handle_effectively_visible(edge.source.as_str())
				&& self.handle_effectively_visible(edge.target.as_str())
		}

		fn wire_effectively_visible(&self, wire: &WireData) -> bool {
			wire.visible
				&& self.handle_effectively_visible(wire.source.as_str())
				&& wire.target.as_ref().map(|id| self.handle_effectively_visible(id.as_str())).unwrap_or(true)
		}

		/// @emoji 💫 True when the handle may be drawn or hit-tested on the indirect-connect ghost ring (`overview`/`normal` LOD).
		fn handle_eligible_indirect_connect_ring(&self, handle_id: &str) -> bool {
			self.handle_effectively_visible(handle_id) && !self.handle_has_incident_edge(handle_id)
		}

		/// @emoji 📍 Drag-phase link snap tests **screen px** to the handle anchor so detail/micro zoom keeps a stable hit halo; pointer-up re-checks with `link_snap_commit_proximity_ok` before `proximityConnect`.
		fn nearest_link_snap_handle_world(&self, source_handle_id: &str, world: Point) -> Option<String> {
			if matches!(self.current_draw_lod(), BoardDrawLod::Minimap) {
				return None;
			}
			let source_handle = self.handles.get(source_handle_id)?;
			if !self.handle_effectively_visible(source_handle_id) {
				return None;
			}
			let source_node_id = source_handle.node_id.as_str();
			let p_scr = self.world_to_screen(world);
			let mut best: Option<(f64, String)> = None;
			for (id, h) in &self.handles {
				if id == source_handle_id || !self.handle_effectively_visible(id.as_str()) {
					continue;
				}
				if self.handle_has_incident_edge(id.as_str()) {
					continue;
				}
				if h.node_id == source_node_id {
					continue;
				}
				if !self.handles_link_compatible_for_drag(source_handle, h) {
					continue;
				}
				let pw = self.handle_world_pos(h)?;
				let h_scr = self.world_to_screen(pw);
				let d_screen = distance_between(p_scr, h_scr);
				let tol_screen = self.link_snap_drag_tolerance_screen(h);
				if d_screen <= tol_screen && best.as_ref().map(|(bd, _)| d_screen < *bd).unwrap_or(true) {
					best = Some((d_screen, id.clone()));
				}
			}
			best.map(|(_, id)| id)
		}

		fn try_commit_link_edge(&mut self, source_handle_id: &str, target_handle_id: &str, also_emit: Option<&'static str>) -> bool {
			if source_handle_id == target_handle_id {
				return false;
			}
			if !self.handle_effectively_visible(source_handle_id) || !self.handle_effectively_visible(target_handle_id) {
				return false;
			}
			let Some(source_row) = self.handles.get(source_handle_id) else {
				return false;
			};
			let Some(target_row) = self.handles.get(target_handle_id) else {
				return false;
			};
			if source_row.node_id == target_row.node_id {
				return false;
			}
			if !self.handles_link_compatible_for_drag(source_row, target_row) {
				return false;
			}
			if self.handle_has_incident_edge(source_handle_id) || self.handle_has_incident_edge(target_handle_id) {
				return false;
			}
			for e in self.edges.values() {
				if e.source == source_handle_id && e.target == target_handle_id {
					return false;
				}
			}
			let mut n = self.edges.len().saturating_add(1);
			let id = loop {
				let candidate = format!("edge-link-{n}");
				if !self.edges.contains_key(&candidate) {
					break candidate;
				}
				n = n.saturating_add(1);
			};
			let edge_kind = self.default_edge_kind_for_created_link(source_row, target_row);
			self.edges.insert(
				id.clone(),
				EdgeData {
					id: id.clone(),
					source: source_handle_id.to_string(),
					target: target_handle_id.to_string(),
					selected: false,
					visible: true,
					style: None,
					edge_kind,
				},
			);
			self.push_event(
				"edgeCreate",
				json!({ "id": id, "source": source_handle_id, "target": target_handle_id }),
			);
			if let Some(name) = also_emit {
				self.push_event(
					name,
					json!({ "id": id, "source": source_handle_id, "target": target_handle_id }),
				);
			}
			true
		}

		pub fn pointer_down_screen(&mut self, sx: f64, sy: f64, button: u8, shift: bool, ctrl_or_meta: bool) {
			self.set_selection_screen_preview(None);
			let screen = Point::new(sx, sy);
			let world = self.screen_to_world(screen);
			let hit = self
				.resolve_hit_world(world)
				.or_else(|| self.resolve_overview_free_link_handle_pointer_world(world));
			if let Interaction::LinkTargetNode {
				source_id,
				target_node_id,
			} = self.interaction.clone()
			{
				self.interaction = Interaction::None;
				self.clear_link_gesture_events();
				if button == 0 {
					if let Some(th) = self.node_sole_free_link_compatible_handle(&source_id, &target_node_id) {
						if hit.as_deref() == Some(target_node_id.as_str()) || hit.as_deref() == Some(th.as_str()) {
							self.try_commit_link_edge(&source_id, &th, Some("indirectConnect"));
							self.update_hover_from_world(world);
							return;
						}
					}
					if let Some(hid) = hit.as_ref().filter(|id| {
						self.handles
							.get(*id)
							.is_some_and(|h| h.node_id == target_node_id)
							&& self.handle_eligible_link_target_ring(id.as_str(), source_id.as_str())
					}) {
						self.try_commit_link_edge(&source_id, hid, Some("indirectConnect"));
						self.update_hover_from_world(world);
						return;
					}
				}
				self.update_hover_from_world(world);
				return;
			}
			let merge_from_modifiers = ctrl_or_meta || shift;
			let pick_mode = Self::pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.selection_options.mode.as_str());
			if button == 0 && !merge_from_modifiers && self.try_begin_bounded_selection_drag_at(world) {
				return;
			}
			if button == 1 {
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
						let members_before: Vec<String> = self
							.selection
							.iter()
							.filter(|id| self.nodes.get(*id).is_some_and(|n| n.draggable))
							.cloned()
							.collect();
						let drag_group_before = members_before.contains(&nid) && members_before.len() > 1;
						let force_pick_merge = (pick_mode == "replace" && !drag_group_before)
							|| pick_mode == "subtractive"
							|| (pick_mode == "invertive" && merge_from_modifiers);
						if !drag_group_before || force_pick_merge {
							let next = Self::merge_pick_into_selection(&self.selection, &nid, pick_mode.as_str());
							let ids: Vec<_> = next.iter().cloned().collect();
							let gesture = merge_from_modifiers.then_some(pick_mode.as_str());
							self.set_selection_ids_gestured(&ids, gesture);
						}
						let members: Vec<String> = self
							.selection
							.iter()
							.filter(|id| self.nodes.get(*id).is_some_and(|n| n.draggable))
							.cloned()
							.collect();
						let drag_group = members.contains(&nid) && members.len() > 1;
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
							proximity_pair: None,
						};
						self.set_hovered_id(hit);
						return;
					}
				}
			}
			if let Some(ref hid) = hit {
				if button == 0 && self.handles.contains_key(hid) && !self.handle_has_incident_edge(hid.as_str()) {
					let next = Self::merge_pick_into_selection(&self.selection, hid, pick_mode.as_str());
					let ids: Vec<_> = next.iter().cloned().collect();
					let gesture = merge_from_modifiers.then_some(pick_mode.as_str());
					self.set_selection_ids_gestured(&ids, gesture);
					self.interaction = Interaction::LinkAtSourceHandle {
						source_id: hid.clone(),
						start_screen: screen,
					};
					self.set_hovered_id(Some(hid.clone()));
					return;
				}
			}
			if hit.is_none() && button == 0 {
				self.interaction = Interaction::SelectionPending {
					initial_ids: self.selection.clone(),
					start: world,
					start_screen: screen,
				};
				self.set_hovered_id(None);
				return;
			}
			self.interaction = Interaction::None;
			if let Some(id) = hit {
				let next = Self::merge_pick_into_selection(&self.selection, &id, pick_mode.as_str());
				let ids: Vec<_> = next.iter().cloned().collect();
				let gesture = merge_from_modifiers.then_some(pick_mode.as_str());
				self.set_selection_ids_gestured(&ids, gesture);
				self.set_hovered_id(Some(id));
			} else {
				let gesture = merge_from_modifiers.then_some(pick_mode.as_str());
				self.set_selection_ids_gestured(&[], gesture);
				self.set_hovered_id(None);
			}
		}

		pub fn pointer_move_screen(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool) {
			let screen = Point::new(sx, sy);
			let world = self.screen_to_world(screen);
			match std::mem::replace(&mut self.interaction, Interaction::None) {
				Interaction::DragNodes {
					primary_id,
					offset,
					start_positions,
					..
				} => {
					let primary_id = primary_id.clone();
					let offset = offset;
					let start_positions_cloned = start_positions.clone();
					let (px0, py0) = start_positions.get(&primary_id).copied().unwrap_or((0.0, 0.0));
					let nx = world.x - offset.x;
					let ny = world.y - offset.y;
					let mut dx = nx - px0;
					let mut dy = ny - py0;
					if self.grid_snap_enabled {
						let (snx, sny) = self.snap_world_pair(nx, ny);
						dx = snx - px0;
						dy = sny - py0;
					}
					for (id, (ox0, oy0)) in &start_positions {
						if let Some(n) = self.nodes.get_mut(id) {
							let mx = ox0 + dx;
							let my = oy0 + dy;
							n.x = mx;
							n.y = my;
							self.push_event("nodeMove", json!({ "id": id, "x": mx, "y": my }));
						}
					}
					let proximity_pair = if start_positions.len() == 1 {
						self.node_drag_proximity_handle_pair(primary_id.as_str())
					} else {
						None
					};
					self.interaction = Interaction::DragNodes {
						primary_id,
						offset,
						start_positions: start_positions_cloned,
						proximity_pair,
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
				Interaction::SelectionPending {
					initial_ids,
					start,
					start_screen,
				} => {
					if distance_between(start_screen, screen) < SELECTION_CLICK_MAX_DISTANCE_PX {
						self.interaction = Interaction::SelectionPending {
							initial_ids,
							start,
							start_screen,
						};
					} else {
						let points = vec![start, world];
						let screen_points = vec![start_screen, screen];
						let merge_mode =
							Self::pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.selection_options.mode.as_str());
						let next = self.resolve_area_selection_with_initial(&initial_ids, start, &points, merge_mode.as_str());
						let ids: Vec<_> = next.iter().cloned().collect();
						let merge_from_modifiers = ctrl_or_meta || shift;
						let gesture = merge_from_modifiers.then_some(merge_mode.as_str());
						self.apply_area_preselect(&initial_ids, &ids, gesture);
						self.sync_selection_screen_overlay(start_screen, &screen_points);
						self.interaction = Interaction::Selection {
							initial_ids,
							points,
							screen_points,
							start,
							start_screen,
						};
					}
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
					let merge_mode =
						Self::pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.selection_options.mode.as_str());
					let next = self.resolve_area_selection_with_initial(&initial, start, &pts, merge_mode.as_str());
					let ids: Vec<_> = next.iter().cloned().collect();
					let merge_from_modifiers = ctrl_or_meta || shift;
					let gesture = merge_from_modifiers.then_some(merge_mode.as_str());
					self.apply_area_preselect(&initial, &ids, gesture);
					self.sync_selection_screen_overlay(start_screen, &screen_points);
					self.interaction = Interaction::Selection {
						initial_ids,
						points,
						screen_points,
						start,
						start_screen,
					};
				}
				Interaction::LinkAtSourceHandle { source_id, start_screen } => {
					if distance_between(screen, start_screen) >= LINK_DRAG_MIN_DISTANCE_PX {
						let optional_target_handle_id = self.nearest_link_snap_handle_world(&source_id, world);
						self.apply_link_drag_snap_hover(&source_id, world, optional_target_handle_id.as_deref());
						self.interaction = Interaction::LinkDragSnap {
							source_id: source_id.clone(),
							target_id: optional_target_handle_id,
							end_world: world,
						};
						self.sync_link_gesture_events();
					} else {
						self.interaction = Interaction::LinkAtSourceHandle { source_id, start_screen };
						self.update_hover_from_world(world);
					}
				}
				Interaction::LinkDragSnap { source_id, .. } => {
					let optional_target_handle_id = self.nearest_link_snap_handle_world(&source_id, world);
					self.apply_link_drag_snap_hover(&source_id, world, optional_target_handle_id.as_deref());
					self.interaction = Interaction::LinkDragSnap {
						source_id: source_id.clone(),
						target_id: optional_target_handle_id,
						end_world: world,
					};
					self.sync_link_gesture_events();
				}
				Interaction::LinkTargetNode {
					source_id,
					target_node_id,
				} => {
					self.interaction = Interaction::LinkTargetNode {
						source_id,
						target_node_id,
					};
					self.update_hover_from_world(world);
				}
				Interaction::None => {
					self.interaction = Interaction::None;
					self.update_hover_from_world(world);
				}
			}
		}

		pub fn pointer_up_screen(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool) {
			let screen = Point::new(sx, sy);
			let world = self.screen_to_world(screen);
			let grabbed = std::mem::take(&mut self.interaction);
			match grabbed {
				Interaction::LinkDragSnap { source_id, target_id, .. } => {
					if let Some(ref target_handle_id) = target_id {
						if self.link_snap_commit_proximity_ok(target_handle_id, world)
							&& self.try_commit_link_edge(&source_id, target_handle_id, Some("proximityConnect"))
						{
							self.interaction = Interaction::None;
							self.clear_link_gesture_events();
							self.update_hover_from_world(world);
							return;
						}
					}
					if let Some(target_node_id) = self.resolve_node_hit_world(world) {
						let source_node_id = self.handles.get(&source_id).map(|h| h.node_id.clone());
						if source_node_id.as_deref() != Some(target_node_id.as_str()) {
							if let Some(sole_target) =
								self.node_sole_free_link_compatible_handle(source_id.as_str(), target_node_id.as_str())
							{
								self.try_commit_link_edge(&source_id, &sole_target, Some("indirectConnect"));
								self.clear_link_gesture_events();
							} else {
								self.interaction = Interaction::LinkTargetNode {
									source_id,
									target_node_id: target_node_id.clone(),
								};
								self.set_hovered_id(Some(target_node_id));
								self.sync_link_gesture_events();
							}
							self.update_hover_from_world(world);
							return;
						}
					}
					self.interaction = Interaction::None;
					self.clear_link_gesture_events();
					self.update_hover_from_world(world);
				}
				Interaction::LinkAtSourceHandle { .. } => {
					self.interaction = Interaction::None;
					self.clear_link_gesture_events();
					self.update_hover_from_world(world);
				}
				Interaction::DragNodes {
					proximity_pair: Some((src, tgt)),
					..
				} => {
					let _ = self.try_commit_link_edge(&src, &tgt, Some("proximityConnect"));
					self.interaction = Interaction::None;
					self.update_hover_from_world(world);
				}
				Interaction::DragNodes { .. } => {
					self.interaction = Interaction::None;
					self.update_hover_from_world(world);
				}
				Interaction::SelectionPending {
					initial_ids,
					start,
					start_screen,
				} => {
					let _ = (start, start_screen);
					let merge_from_modifiers = ctrl_or_meta || shift;
					if !merge_from_modifiers {
						self.clear_selection_on_background_click();
					} else {
						let merge_mode =
							Self::pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.selection_options.mode.as_str());
						let gesture = Some(merge_mode.as_str());
						let next =
							self.resolve_area_selection_with_initial(&initial_ids, start, &[start], merge_mode.as_str());
						let ids: Vec<_> = next.iter().cloned().collect();
						self.set_selection_ids_gestured(&ids, gesture);
					}
					self.set_selection_screen_preview(None);
					self.update_hover_from_world(world);
				}
				Interaction::Selection {
					mut points,
					mut screen_points,
					start,
					initial_ids,
					start_screen,
				} => {
					points.push(world);
					screen_points.push(screen);
					let end_screen = screen_points.last().copied().unwrap_or(start_screen);
					let click_only = distance_between(start_screen, end_screen) < SELECTION_CLICK_MAX_DISTANCE_PX;
					let merge_from_modifiers = ctrl_or_meta || shift;
					let merge_mode =
						Self::pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.selection_options.mode.as_str());
					let gesture = merge_from_modifiers.then(|| merge_mode.as_str());
					if click_only {
						self.commit_area_select_from_initial(&initial_ids, &[], gesture);
					} else {
						let next =
							self.resolve_area_selection_with_initial(&initial_ids, start, &points, merge_mode.as_str());
						let ids: Vec<_> = next.iter().cloned().collect();
						self.commit_area_select_from_initial(&initial_ids, &ids, gesture);
					}
					self.set_selection_screen_preview(None);
					self.update_hover_from_world(world);
				}
				_ => {
					self.interaction = Interaction::None;
					self.update_hover_from_world(world);
				}
			}
		}

		pub fn pointer_leave_screen(&mut self) {
			if matches!(self.interaction, Interaction::None) {
				self.set_hovered_id(None);
			}
		}

		/// @emoji ↩️ Aborts an in‑flight rectangle/lasso drag and restores the selection snapshot from when the gesture began.
		pub fn cancel_area_select(&mut self) -> bool {
			let prev = std::mem::replace(&mut self.interaction, Interaction::None);
			match prev {
				Interaction::SelectionPending { .. } => {
					self.set_selection_screen_preview(None);
					true
				}
				Interaction::Selection { initial_ids, .. } => {
					self.set_selection_screen_preview(None);
					self.preselect.clear();
					self.preselect_removed.clear();
					self.last_preselect_emit_sig = None;
					self.selection = initial_ids.clone();
					self.sync_selection_flags_to_objects();
					self.last_select_emit_sig = None;
					let sorted = Self::sorted_selection_ids(&self.selection);
					self.push_event("preselectCancel", json!({ "ids": sorted }));
					true
				}
				other => {
					self.interaction = other;
					false
				}
			}
		}

		fn node_world_bounds(&self, n: &NodeData, pad: f64) -> WorldBox {
			let raw = match n.shape {
				NodeShape::Rectangle => {
					let hw = self.scaled_node_width(n) / 2.0;
					let hh = self.scaled_node_height(n) / 2.0;
					WorldBox {
						min_x: n.x - hw,
						min_y: n.y - hh,
						max_x: n.x + hw,
						max_y: n.y + hh,
					}
				}
				NodeShape::Circle => WorldBox {
					min_x: n.x - self.scaled_node_radius(n),
					min_y: n.y - self.scaled_node_radius(n),
					max_x: n.x + self.scaled_node_radius(n),
					max_y: n.y + self.scaled_node_radius(n),
				},
			};
			inflate_world_box(raw, pad)
		}

		fn selection_draggable_node_members(&self) -> Vec<String> {
			self.selection
				.iter()
				.filter(|id| self.nodes.get(*id).is_some_and(|n| n.draggable))
				.cloned()
				.collect()
		}

		fn selection_union_bounds_world(&self) -> Option<WorldBox> {
			let mut corners: Vec<Point> = Vec::new();
			for id in &self.selection {
				let Some(n) = self.nodes.get(id) else {
					continue;
				};
				let b = self.node_world_bounds(n, 0.0);
				corners.push(Point::new(b.min_x, b.min_y));
				corners.push(Point::new(b.max_x, b.max_y));
			}
			world_box_from_points(&corners)
		}

		/// @emoji 📦 Starts a group drag when `world` lies inside the padded union bounds of the current selection (minimap/overview LOD).
		fn try_begin_bounded_selection_drag_at(&mut self, world: Point) -> bool {
			if !self.lod_uses_bounded_drag() {
				return false;
			}
			let members = self.selection_draggable_node_members();
			if members.is_empty() {
				return false;
			}
			let Some(bounds) = self.selection_union_bounds_world() else {
				return false;
			};
			let pad = BOUNDED_DRAG_HIT_PAD_PX / self.camera.zoom.max(1e-9);
			if !world_box_contains_point(inflate_world_box(bounds, pad), world) {
				return false;
			}
			let primary_id = members
				.iter()
				.min_by(|a, b| {
					let da = self
						.nodes
						.get(*a)
						.map(|n| distance_between(world, Point::new(n.x, n.y)))
						.unwrap_or(f64::INFINITY);
					let db = self
						.nodes
						.get(*b)
						.map(|n| distance_between(world, Point::new(n.x, n.y)))
						.unwrap_or(f64::INFINITY);
					da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
				})
				.cloned()
				.unwrap_or_else(|| members[0].clone());
			let (px0, py0) = self
				.nodes
				.get(&primary_id)
				.map(|n| (n.x, n.y))
				.unwrap_or((0.0, 0.0));
			let mut start_positions = BTreeMap::new();
			for id in &members {
				if let Some(n) = self.nodes.get(id) {
					start_positions.insert(id.clone(), (n.x, n.y));
				}
			}
			self.interaction = Interaction::DragNodes {
				primary_id,
				offset: world - Point::new(px0, py0),
				start_positions,
				proximity_pair: None,
			};
			self.set_hovered_id(None);
			true
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
			let pad = self.effective_handle_radius(h).max(1.0);
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

		fn resolve_area_selection_with_initial(
			&self,
			initial: &BTreeSet<String>,
			start: Point,
			points: &[Point],
			merge_mode: &str,
		) -> BTreeSet<String> {
			let Some((box_, enclosing, ref polygon)) = self.selection_drag_shape_world(start, points) else {
				return initial.clone();
			};
			let mut hits = BTreeSet::new();
			let o = &self.selection_options;
			if o.select_nodes {
				for n in self.nodes.values() {
					if n.visible && self.selection_contains_node(n, box_, enclosing, polygon) {
						hits.insert(n.id.clone());
					}
				}
			}
			if o.select_handles {
				for h in self.handles.values() {
					if self.handle_effectively_visible(h.id.as_str()) && self.selection_contains_handle(h, box_, enclosing, polygon) {
						hits.insert(h.id.clone());
					}
				}
			}
			if o.select_edges {
				for e in self.edges.values() {
					if !self.edge_effectively_visible(e) {
						continue;
					}
					if let Some(c) = self.edge_curve(e) {
						if self.selection_contains_edge(c, box_, enclosing, polygon) {
							hits.insert(e.id.clone());
						}
					}
				}
			}
			if merge_mode == "replace" {
				return hits;
			}
			let mut next = initial.clone();
			for id in &hits {
				match merge_mode {
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

}

pub use board_host::BoardHost;

use std::collections::{BTreeMap, BTreeSet};

pub use crate::vello::kurbo::{CubicBez, Point, Vec2};
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
	pub id: EdgeId,
	pub source_handle: HandleId,
	pub target_handle: HandleId,
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

	pub fn create_edge(&mut self, id: EdgeId, source_handle: HandleId, target_handle: HandleId) {
		self.edges.insert(
			id,
			Edge {
				id,
				source_handle,
				target_handle,
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
		let source_handle = self.handles.get(&edge.source_handle)?;
		let target_handle = self.handles.get(&edge.target_handle)?;
		let source_node = self.nodes.get(&source_handle.node_id)?;
		let target_node = self.nodes.get(&target_handle.node_id)?;
		let source_position = handle_position(source_node, source_handle);
		let target_position = handle_position(target_node, target_handle);
		Some(compute_edge_bezier_points(
			source_position,
			target_position,
			source_node.center,
			target_node.center,
		))
	}

	fn remove_handle(&mut self, id: HandleId) {
		self.handles.remove(&id);
		let removed_edges: Vec<EdgeId> = self
			.edges
			.values()
			.filter(|edge| edge.source_handle == id || edge.target_handle == id)
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
use std::cell::RefCell;
#[cfg(target_arch = "wasm32")]
use std::rc::Rc;
#[cfg(target_arch = "wasm32")]
use js_sys::Promise;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::future_to_promise;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = boardComputeEdgeBezier)]
pub fn board_compute_edge_bezier(
	source_px: f64,
	source_py: f64,
	source_cx: f64,
	source_cy: f64,
	target_px: f64,
	target_py: f64,
	target_cx: f64,
	target_cy: f64,
) -> Vec<f64> {
	let c = compute_edge_bezier_points(
		Point::new(source_px, source_py),
		Point::new(target_px, target_py),
		Point::new(source_cx, source_cy),
		Point::new(target_cx, target_cy),
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

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = boardRedrawLayoutFixtureJson)]
pub fn board_redraw_layout_fixture_json(fixture_json: &str, options_json: &str) -> Result<String, JsValue> {
	apply_redraw_layout_to_fixture_v1_json(fixture_json, options_json).map_err(|e| JsValue::from_str(&e))
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = boardRedrawHandlesFixtureJson)]
pub fn board_redraw_handles_fixture_json(fixture_json: &str) -> Result<String, JsValue> {
	apply_edge_handle_snap_to_fixture_v1_json(fixture_json).map_err(|e| JsValue::from_str(&e))
}

// #region 🔖WasmSession
/// 🖥️ Single WASM entry: one {@link BoardHost}, optional WebGPU surface bound via {@link BoardSession::attach_canvas}.
#[cfg(target_arch = "wasm32")]
struct BoardSessionInner {
	host: BoardHost,
	#[allow(dead_code, reason = "Retains canvas for the WebGPU surface lifetime.")]
	canvas: Option<HtmlCanvasElement>,
	render_ctx: Option<crate::vello::util::RenderContext>,
	renderer: Option<crate::vello::Renderer>,
	surface: Option<crate::vello::util::RenderSurface<'static>>,
}

#[cfg(target_arch = "wasm32")]
impl BoardSessionInner {
	fn set_logical_size_and_maybe_resize_surface(&mut self, lw: u32, lh: u32, dpr: f64, pw: u32, ph: u32) {
		self.host.set_size(lw, lh, dpr);
		if let (Some(surface), Some(render_ctx)) = (self.surface.as_mut(), self.render_ctx.as_mut()) {
			let cur_w = surface.config.width;
			let cur_h = surface.config.height;
			if cur_w != pw || cur_h != ph {
				render_ctx.resize_surface(surface, pw, ph);
			}
		}
	}

	fn render_frame_gpu(&mut self) -> Result<(), JsValue> {
		for _attempt in 0..3u8 {
			let scene = self.host.build_vector_scene();
			let (surface, renderer, render_ctx) = match (
				self.surface.as_mut(),
				self.renderer.as_mut(),
				self.render_ctx.as_mut(),
			) {
				(Some(s), Some(r), Some(rc)) => (s, r, rc),
				_ => return Ok(()),
			};
			let dh = &render_ctx.devices[surface.dev_id];
			let pw = surface.config.width.max(1);
			let ph = surface.config.height.max(1);
			let params = crate::vello::RenderParams {
				base_color: self.host.vello_theme.raster_clear,
				width: pw,
				height: ph,
				antialiasing_method: crate::vello::AaConfig::Area,
			};
			renderer
				.render_to_texture(&dh.device, &dh.queue, &scene, &surface.target_view, &params)
				.map_err(|err| JsValue::from_str(&format!("{err:?}")))?;

			let surface_tex = match surface.surface.get_current_texture() {
				Ok(t) => t,
				Err(crate::vello::wgpu::SurfaceError::Outdated) => {
					surface.surface.configure(&dh.device, &surface.config);
					continue;
				}
				Err(crate::vello::wgpu::SurfaceError::Timeout) | Err(crate::vello::wgpu::SurfaceError::Other) => return Ok(()),
				Err(crate::vello::wgpu::SurfaceError::Lost)
				| Err(crate::vello::wgpu::SurfaceError::OutOfMemory) => {
					return Err(JsValue::from_str("surface lost or validation error"));
				}
			};
			let view = surface_tex
				.texture
				.create_view(&crate::vello::wgpu::TextureViewDescriptor::default());
			let mut encoder = dh.device.create_command_encoder(&crate::vello::wgpu::CommandEncoderDescriptor {
				label: Some("elements_board_surface_blit"),
			});
			surface
				.blitter
				.copy(&dh.device, &mut encoder, &surface.target_view, &view);
			dh.queue.submit(std::iter::once(encoder.finish()));
			surface_tex.present();
			let _ = dh.device.poll(crate::vello::wgpu::PollType::Poll).ok();
			return Ok(());
		}
		Ok(())
	}
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct BoardSession {
	state: Rc<RefCell<BoardSessionInner>>,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl BoardSession {
	#[wasm_bindgen(constructor)]
	pub fn new() -> Self {
		Self {
			state: Rc::new(RefCell::new(BoardSessionInner {
				host: BoardHost::new(),
				canvas: None,
				render_ctx: None,
				renderer: None,
				surface: None,
			})),
		}
	}

	#[wasm_bindgen(js_name = gpuReady)]
	pub fn gpu_ready(&self) -> bool {
		self.state.borrow().surface.is_some()
	}

	#[wasm_bindgen(js_name = isDraggingAreaSelect)]
	pub fn is_dragging_area_select(&self) -> bool {
		self.state.borrow().host.is_dragging_area_select()
	}

	#[wasm_bindgen(js_name = defersDescriptorSyncFromJs)]
	pub fn defers_descriptor_sync_from_js(&self) -> bool {
		self.state.borrow().host.defers_descriptor_sync_from_js()
	}

	/// @emoji 🌊 Binds WebGPU presentation to `canvas` once; `logical_w`/`logical_h` are CSS pixels, `dpr` scales the swapchain backing store; uses `future_to_promise` so wasm-bindgen does not hold `&mut BoardSession` across `await` (avoids `borrow_fail` vs `setSize` during GPU setup).
	#[wasm_bindgen(js_name = attach_canvas)]
	pub fn attach_canvas(
		&mut self,
		canvas: HtmlCanvasElement,
		logical_w: u32,
		logical_h: u32,
		dpr: f64,
	) -> Promise {
		let inner = self.state.clone();
		if inner.borrow().surface.is_some() {
			return future_to_promise(async move { Err(JsValue::from_str("canvas surface already attached")) });
		}
		let lw = logical_w.max(1);
		let lh = logical_h.max(1);
		let dpr = dpr.max(1.0);
		let pw = ((lw as f64 * dpr).round() as u32).max(1);
		let ph = ((lh as f64 * dpr).round() as u32).max(1);
		let canvas = canvas.clone();
		future_to_promise(async move {
			let mut render_ctx = crate::vello::util::RenderContext::new();
			let surface = render_ctx
				.create_surface(
					crate::vello::wgpu::SurfaceTarget::Canvas(canvas.clone()),
					pw,
					ph,
					crate::vello::wgpu::PresentMode::AutoVsync,
				)
				.await
				.map_err(|err| JsValue::from_str(&format!("{err:?}")))?;
			let dev = &render_ctx.devices[surface.dev_id].device;
			let renderer = crate::vello::Renderer::new(
				dev,
				crate::vello::RendererOptions {
					use_cpu: false,
					antialiasing_support: crate::vello::AaSupport::area_only(),
					num_init_threads: std::num::NonZeroUsize::new(1),
					pipeline_cache: None,
				},
			)
			.map_err(|err| JsValue::from_str(&format!("{err:?}")))?;
			let mut g = inner.borrow_mut();
			if g.surface.is_some() {
				return Err(JsValue::from_str("canvas surface already attached"));
			}
			g.host.set_size(lw, lh, dpr);
			g.canvas = Some(canvas);
			g.render_ctx = Some(render_ctx);
			g.renderer = Some(renderer);
			g.surface = Some(surface);
			Ok(JsValue::UNDEFINED)
		})
	}

	#[wasm_bindgen(js_name = setSize)]
	pub fn set_size(&mut self, width: u32, height: u32, dpr: f64) {
		let lw = width.max(1);
		let lh = height.max(1);
		let dpr = dpr.max(1.0);
		let pw = ((lw as f64 * dpr).round() as u32).max(1);
		let ph = ((lh as f64 * dpr).round() as u32).max(1);
		let mut inner = self.state.borrow_mut();
		inner.set_logical_size_and_maybe_resize_surface(lw, lh, dpr, pw, ph);
	}

	#[wasm_bindgen(js_name = setSelectionScreenPreview)]
	pub fn set_selection_screen_preview(&mut self, flat_xy: &[f64]) {
		let mut inner = self.state.borrow_mut();
		if flat_xy.len() < 4 || flat_xy.len() % 2 != 0 {
			inner.host.set_selection_screen_preview(None);
			return;
		}
		let mut pts = Vec::with_capacity(flat_xy.len() / 2);
		for chunk in flat_xy.chunks_exact(2) {
			pts.push(Point::new(chunk[0], chunk[1]));
		}
		inner.host.set_selection_screen_preview(Some(pts));
	}

	#[wasm_bindgen(js_name = clearSelectionScreenPreview)]
	pub fn clear_selection_screen_preview(&mut self) {
		self.state.borrow_mut().host.set_selection_screen_preview(None);
	}

	#[wasm_bindgen(js_name = syncDescriptorJson)]
	pub fn sync_descriptor_json(&mut self, json: &str) -> Result<(), JsValue> {
		let mut raw: serde_json::Value = serde_json::from_str(json).map_err(|e| JsValue::from_str(&e.to_string()))?;
		normalize_board_descriptor_hidden_to_visible(&mut raw);
		let desc: SceneDescriptorJson = serde_json::from_value(raw).map_err(|e| JsValue::from_str(&e.to_string()))?;
		self.state.borrow_mut().host.sync_descriptor(&desc).map_err(|e| JsValue::from_str(&e))?;
		Ok(())
	}

	#[wasm_bindgen(js_name = setBoardKindCatalogsJson)]
	pub fn set_board_kind_catalogs_json(&mut self, json: &str) -> Result<(), JsValue> {
		self.state
			.borrow_mut()
			.host
			.set_board_kind_catalogs_from_json(json)
			.map_err(|e| JsValue::from_str(&e))
	}

	#[wasm_bindgen(js_name = setVelloThemeJson)]
	pub fn set_vello_theme_json(&mut self, json: &str) {
		let _ = self.state.borrow_mut().host.set_vello_theme_from_json(json);
	}

	#[wasm_bindgen(js_name = clearIconVectorCache)]
	pub fn clear_icon_vector_cache_wasm(&mut self) {
		self.state.borrow_mut().host.clear_icon_vector_cache();
	}

	#[wasm_bindgen(js_name = parseFixtureJson)]
	pub fn parse_fixture_json(&mut self, json: &str) -> bool {
		let raw: serde_json::Value = match serde_json::from_str(json) {
			Ok(v) => v,
			Err(_) => return false,
		};
		self.state.borrow_mut().host.parse_fixture_v1(&raw)
	}

	#[wasm_bindgen(js_name = setCamera)]
	pub fn set_camera_wasm(&mut self, x: f64, y: f64, zoom: f64) {
		self.state.borrow_mut().host.set_camera(x, y, zoom);
	}

	#[wasm_bindgen(js_name = pointerDownScreen)]
	pub fn pointer_down_screen_wasm(&mut self, sx: f64, sy: f64, button: u8, shift: bool, ctrl_or_meta: bool) {
		self.state.borrow_mut().host.pointer_down_screen(sx, sy, button, shift, ctrl_or_meta);
	}

	#[wasm_bindgen(js_name = pointerMoveScreen)]
	pub fn pointer_move_screen_wasm(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool) {
		self.state.borrow_mut().host.pointer_move_screen(sx, sy, shift, ctrl_or_meta);
	}

	#[wasm_bindgen(js_name = pointerUpScreen)]
	pub fn pointer_up_screen_wasm(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool) {
		self.state.borrow_mut().host.pointer_up_screen(sx, sy, shift, ctrl_or_meta);
	}

	#[wasm_bindgen(js_name = pointerLeaveScreen)]
	pub fn pointer_leave_screen_wasm(&mut self) {
		self.state.borrow_mut().host.pointer_leave_screen();
	}

	#[wasm_bindgen(js_name = cancelAreaSelect)]
	pub fn cancel_area_select_wasm(&mut self) -> bool {
		self.state.borrow_mut().host.cancel_area_select()
	}

	#[wasm_bindgen(js_name = wheelScreen)]
	pub fn wheel_screen_wasm(&mut self, sx: f64, sy: f64, delta_y: f64) {
		self.state.borrow_mut().host.wheel_screen(sx, sy, delta_y);
	}

	#[wasm_bindgen(js_name = deleteSelection)]
	pub fn delete_selection_wasm(&mut self) {
		self.state.borrow_mut().host.delete_selection();
	}

	#[wasm_bindgen(js_name = drainEventsJson)]
	pub fn drain_events_json_wasm(&mut self) -> String {
		self.state.borrow_mut().host.drain_events_json()
	}

	#[wasm_bindgen(js_name = cameraJson)]
	pub fn camera_json(&self) -> String {
		let inner = self.state.borrow();
		serde_json::json!({
			"x": inner.host.camera.x,
			"y": inner.host.camera.y,
			"zoom": inner.host.camera.zoom,
		})
		.to_string()
	}

	#[wasm_bindgen(js_name = setSelectionOptions)]
	pub fn set_selection_options_wasm(
		&mut self,
		method: &str,
		mode: &str,
		select_nodes: bool,
		select_edges: bool,
		select_handles: bool,
	) {
		self.state
			.borrow_mut()
			.host
			.set_selection_options(method, mode, select_nodes, select_edges, select_handles);
	}

	#[wasm_bindgen(js_name = setHandleLinkCompatJson)]
	pub fn set_handle_link_compat_json(&mut self, json: &str) -> Result<(), JsValue> {
		self.state
			.borrow_mut()
			.host
			.set_handle_link_compat_from_json(json)
			.map_err(|e| JsValue::from_str(&e))
	}

	#[wasm_bindgen(js_name = setWorldRasterTiling)]
	pub fn set_world_raster_tiling_wasm(&mut self, mode: &str) {
		self.state.borrow_mut().host.set_world_raster_tiling(mode);
	}

	#[wasm_bindgen(js_name = setLodZoomThresholdsJson)]
	pub fn set_lod_zoom_thresholds_json_wasm(&mut self, json: &str) -> Result<(), JsValue> {
		self.state
			.borrow_mut()
			.host
			.set_lod_zoom_thresholds_from_json(json)
			.map_err(|e| JsValue::from_str(&e))
	}

	#[wasm_bindgen(js_name = setGridSnapEnabled)]
	pub fn set_grid_snap_enabled_wasm(&mut self, enabled: bool) {
		self.state.borrow_mut().host.set_grid_snap_enabled(enabled);
	}

	#[wasm_bindgen(js_name = setGridFactor)]
	pub fn set_grid_factor_wasm(&mut self, v: f64) -> Result<(), JsValue> {
		self.state
			.borrow_mut()
			.host
			.set_grid_factor(v)
			.map_err(|e| JsValue::from_str(&e))
	}

	#[wasm_bindgen(js_name = setOriginalElementStyle)]
	pub fn set_original_element_style_wasm(&mut self, enabled: bool) {
		self.state.borrow_mut().host.set_original_element_style(enabled);
	}

	#[wasm_bindgen(js_name = setAutomaticLod)]
	pub fn set_automatic_lod_wasm(&mut self, enabled: bool) {
		self.state.borrow_mut().host.set_automatic_lod(enabled);
	}

	#[wasm_bindgen(js_name = setForcedDrawLodLabel)]
	pub fn set_forced_draw_lod_label_wasm(&mut self, label: &str) {
		self.state.borrow_mut().host.set_forced_draw_lod_label(label);
	}

	#[wasm_bindgen(js_name = setSelectionIdsJson)]
	pub fn set_selection_ids_json(&mut self, json: &str) -> Result<(), JsValue> {
		let ids: Vec<String> = serde_json::from_str(json).map_err(|err| JsValue::from_str(&err.to_string()))?;
		self.state.borrow_mut().host.set_selection_ids(&ids);
		Ok(())
	}

	#[wasm_bindgen(js_name = setSelectionIdsJsonSilent)]
	pub fn set_selection_ids_json_silent(&mut self, json: &str) -> Result<(), JsValue> {
		let ids: Vec<String> = serde_json::from_str(json).map_err(|err| JsValue::from_str(&err.to_string()))?;
		self.state.borrow_mut().host.set_selection_ids_silent(&ids);
		Ok(())
	}

	#[wasm_bindgen(js_name = setPreselectStateJsonSilent)]
	pub fn set_preselect_state_json_silent(&mut self, json: &str) -> Result<(), JsValue> {
		#[derive(serde::Deserialize)]
		struct PreselectSync {
			ids: Vec<String>,
			#[serde(default, rename = "removedIds")]
			removed_ids: Vec<String>,
		}
		let body: PreselectSync = serde_json::from_str(json).map_err(|err| JsValue::from_str(&err.to_string()))?;
		self.state
			.borrow_mut()
			.host
			.set_preselect_state_silent(&body.ids, &body.removed_ids);
		Ok(())
	}

	#[wasm_bindgen(js_name = setHoveredIdSilent)]
	pub fn set_hovered_id_silent_wasm(&mut self, id: Option<String>) {
		self.state.borrow_mut().host.set_hovered_id_silent(id);
	}

	#[wasm_bindgen(js_name = encodedSceneHint)]
	pub fn encoded_scene_hint_wasm(&self) -> usize {
		self.state.borrow().host.encoded_scene_hint()
	}

	/// @emoji 🎨 Presents one frame when a GPU surface is attached; otherwise no-op `Ok`.
	#[wasm_bindgen(js_name = renderFrame)]
	pub fn render_frame(&mut self) -> Result<(), JsValue> {
		self.state.borrow_mut().render_frame_gpu()
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
		let source_radial = curve.p0 - Point::ORIGIN;
		let arm0 = curve.p1 - curve.p0;
		let align0 = vcompute::normalize_or_zero(source_radial).dot(vcompute::normalize_or_zero(arm0));
		let target_approach = Point::new(300.0, 0.0) - curve.p3;
		let arm1 = curve.p3 - curve.p2;
		let align1 = vcompute::normalize_or_zero(target_approach).dot(vcompute::normalize_or_zero(arm1));
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
	use crate::board_host::Interaction;
	use crate::geom_sel::cubic_bezier_point;
	use super::vcompute::distance_between;
	use super::vcompute::compute_edge_bezier_points;
	use super::vcompute::handle_position_on_circle;
	use super::vcompute::handle_position_on_rectangle;
	use super::{BoardHost, EdgeDescJson, HandleDescJson, NodeDescJson, SceneDescriptorJson, WireDescJson};
	use crate::vello::kurbo::Point;
	use serde_json::json;

	fn set_detail_lod(h: &mut BoardHost) {
		h.set_camera(0.0, 0.0, 2.0);
	}

	fn set_micro_lod(h: &mut BoardHost) {
		h.set_camera(0.0, 60.0, 4.5);
	}

	fn set_overview_lod(h: &mut BoardHost) {
		h.set_camera(0.0, 0.0, 0.25);
	}

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
				icon_kind: None,
				node_kind: None,
				user_data: None,
				visible: None,
				root: None,
				shape: Some("circle".into()),
				radius: Some(40.0),
				width: None,
				height: None,
				scale: None,
			}],
			handles: vec![
				HandleDescJson {
					id: "a:h0".into(),
					node_id: "a".into(),
					angle: 0.0,
					radius: None,
					selected: None,
					style: None,
					handle_kind: Some("port".into()),
					color: None,
					icon_kind: None,
					user_data: None,
					visible: None,
					scale: None,
				},
				HandleDescJson {
					id: "b:h0".into(),
					node_id: "b".into(),
					angle: std::f64::consts::PI,
					radius: None,
					selected: None,
					style: None,
					handle_kind: Some("port".into()),
					color: None,
					icon_kind: None,
					user_data: None,
					visible: None,
					scale: None,
				},
			],
			edges: vec![EdgeDescJson {
				id: "e1".into(),
				source: "a:h0".into(),
				target: "b:h0".into(),
				edge_kind: None,
				selected: None,
				style: None,
				user_data: None,
				visible: None,
			}],
			wires: vec![],
			selection_exit_highlight_ids: vec![],
		}
	}

	#[test]
	fn board_host_manual_lod_follow_zoom_still_encodes_graph() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		h.sync_descriptor(&sample_scene()).unwrap();
		let with_automatic = h.encoded_scene_hint();
		assert!(with_automatic > 0, "sample scene should encode vector paths");
		h.set_automatic_lod(false);
		h.set_forced_draw_lod_label("");
		let manual_follow_zoom = h.encoded_scene_hint();
		assert!(
			manual_follow_zoom > 0,
			"manual follow-zoom LOD must still draw nodes/edges (hint={manual_follow_zoom})"
		);
		h.set_forced_draw_lod_label("overview");
		let pinned_overview = h.encoded_scene_hint();
		assert!(pinned_overview > 0, "pinned overview LOD must still draw graph");
		h.set_automatic_lod(true);
		let automatic_restored = h.encoded_scene_hint();
		assert_eq!(with_automatic, automatic_restored);
	}

	#[test]
	fn board_host_pick_selection_never_sets_exit_highlight() {
		let mut h = BoardHost::new();
		h.set_size(400, 300, 1.0);
		let mut d = sample_scene();
		d.selection_exit_highlight_ids = vec!["a".into(), "ghost".into()];
		h.sync_descriptor(&d).unwrap();
		let _ = h.drain_events_json();
		assert!(h.selection_exit_highlight.is_empty());
		h.set_selection_ids(&["a".into(), "e1".into()]);
		let ev = h.drain_events_json();
		assert!(h.selection_exit_highlight.is_empty());
		assert!(ev.contains("\"exitHighlightIds\":[]"));
		h.set_selection_ids(&["e1".into()]);
		let ev2 = h.drain_events_json();
		assert!(h.selection_exit_highlight.is_empty());
		assert!(ev2.contains("\"exitHighlightIds\":[]"));
	}

	#[test]
	fn board_host_vello_theme_keeps_explicit_element_state_colors() {
		let mut h = BoardHost::new();
		h.set_vello_theme_from_json(
			r#"{
				"nodeStrokeHovered": [1, 2, 3, 255],
				"edgeStrokeHovered": [4, 5, 6, 255],
				"handleStrokeHovered": [7, 8, 9, 255],
				"wireStrokeHovered": [10, 11, 12, 255]
			}"#,
		)
		.unwrap();
		assert_eq!(h.vello_theme.node_stroke_hovered.to_rgba8(), crate::vello::peniko::Color::from_rgba8(1, 2, 3, 255).to_rgba8());
		assert_eq!(h.vello_theme.edge_stroke_hovered.to_rgba8(), crate::vello::peniko::Color::from_rgba8(4, 5, 6, 255).to_rgba8());
		assert_eq!(h.vello_theme.handle_stroke_hovered.to_rgba8(), crate::vello::peniko::Color::from_rgba8(7, 8, 9, 255).to_rgba8());
		assert_eq!(h.vello_theme.wire_stroke_hovered.to_rgba8(), crate::vello::peniko::Color::from_rgba8(10, 11, 12, 255).to_rgba8());
	}

	#[test]
	fn board_host_cancel_area_select_restores_initial_selection() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		set_detail_lod(&mut h);
		h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
		let _ = h.drain_events_json();
		h.set_selection_ids(&["a".into(), "b".into()]);
		let _ = h.drain_events_json();
		h.pointer_down_screen(5.0, 5.0, 0, false, false);
		assert!(!h.is_dragging_area_select());
		h.pointer_move_screen(20.0, 5.0, false, false);
		assert!(h.is_dragging_area_select());
		let _ = h.drain_events_json();
		assert!(h.cancel_area_select());
		assert!(!h.is_dragging_area_select());
		let ev = h.drain_events_json();
		assert!(ev.contains("preselectCancel"));
		assert!(!ev.contains("\"select\""));
		assert_eq!(h.selection.len(), 2);
		assert!(h.selection.contains("a") && h.selection.contains("b"));
		assert!(h.preselect.is_empty());
	}

	#[test]
	fn board_host_syncs_descriptor_and_hit_tests_handle_before_node() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		set_detail_lod(&mut h);
		let mut desc = sample_scene();
		desc.nodes.push(NodeDescJson {
			id: "b".into(),
			x: 300.0,
			y: 0.0,
			draggable: Some(true),
			selected: None,
			style: None,
			text: None,
			icon_kind: None,
			node_kind: None,
			user_data: None,
			visible: None,
			root: None,
			shape: Some("circle".into()),
			radius: Some(40.0),
			width: None,
			height: None,
			scale: None,
		});
		h.sync_descriptor(&desc).unwrap();
		let hp = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
		let hit = h.resolve_hit_world(hp);
		assert_eq!(hit.as_deref(), Some("a:h0"));
		assert!(h.encoded_scene_hint() > 10);
	}

	#[test]
	fn board_host_world_clip_changes_vector_encoding() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		let mut desc = sample_scene();
		desc.nodes.push(NodeDescJson {
			id: "b".into(),
			x: 600.0,
			y: 400.0,
			draggable: Some(true),
			selected: None,
			style: None,
			text: None,
			icon_kind: None,
			node_kind: None,
			user_data: None,
			visible: None,
			root: None,
			shape: Some("circle".into()),
			radius: Some(40.0),
			width: None,
			height: None,
			scale: None,
		});
		h.sync_descriptor(&desc).unwrap();
		h.set_world_raster_tiling("none");
		let monolithic = h.encoded_scene_hint();
		h.set_world_raster_tiling("world-clip");
		let tiled = h.encoded_scene_hint();
		assert!(tiled >= monolithic);
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
			icon_kind: None,
			node_kind: None,
			user_data: None,
			visible: None,
			root: None,
			shape: Some("circle".into()),
			radius: Some(40.0),
			width: None,
			height: None,
			scale: None,
		});
		h.sync_descriptor(&desc).unwrap();
		let _ = h.drain_events_json();
		let w = Point::new(0.0, 0.0);
		let s = h.world_to_screen(w);
		h.pointer_down_screen(s.x, s.y, 0, false, false);
		h.pointer_move_screen(s.x + 50.0, s.y + 30.0, false, false);
		h.pointer_up_screen(s.x + 50.0, s.y + 30.0, false, false);
		let ev = h.drain_events_json();
		assert!(ev.contains("nodeMove"));
	}

	#[test]
	fn board_host_compact_discrete_hit_selects_and_drags_node() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		h.set_camera(0.0, 0.0, 0.5);
		let mut desc = sample_scene();
		desc.handles.clear();
		desc.edges.clear();
		desc.nodes.push(NodeDescJson {
			id: "b".into(),
			x: 300.0,
			y: 0.0,
			draggable: Some(true),
			selected: None,
			style: None,
			text: None,
			icon_kind: None,
			node_kind: None,
			user_data: None,
			visible: None,
			root: None,
			shape: Some("circle".into()),
			radius: Some(40.0),
			width: None,
			height: None,
			scale: None,
		});
		h.sync_descriptor(&desc).unwrap();
		let _ = h.drain_events_json();
		assert_eq!(h.resolve_hit_world(Point::new(0.0, 0.0)).as_deref(), Some("a"));
		assert!(h.resolve_hit_world(Point::new(150.0, 0.0)).is_none());
		let s = h.world_to_screen(Point::new(0.0, 0.0));
		h.pointer_down_screen(s.x, s.y, 0, false, false);
		h.pointer_move_screen(s.x + 50.0, s.y + 30.0, false, false);
		h.pointer_up_screen(s.x + 50.0, s.y + 30.0, false, false);
		let ev = h.drain_events_json();
		assert!(ev.contains("nodeMove"), "compact discrete node hit should drag, got: {ev}");
	}

	#[test]
	fn board_host_minimap_bounded_drag_moves_selection_inside_union_bounds() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		h.set_automatic_lod(false);
		h.set_forced_draw_lod_label("minimap");
		h.set_camera(0.0, 0.0, 0.1);
		let mut desc = sample_scene();
		desc.handles.clear();
		desc.edges.clear();
		desc.nodes.push(NodeDescJson {
			id: "b".into(),
			x: 300.0,
			y: 0.0,
			draggable: Some(true),
			selected: None,
			style: None,
			text: None,
			icon_kind: None,
			node_kind: None,
			user_data: None,
			visible: None,
			root: None,
			shape: Some("circle".into()),
			radius: Some(40.0),
			width: None,
			height: None,
			scale: None,
		});
		h.sync_descriptor(&desc).unwrap();
		h.set_selection_ids(&["a".into(), "b".into()]);
		let _ = h.drain_events_json();
		let gap = Point::new(150.0, 0.0);
		assert!(h.resolve_hit_world(gap).is_none());
		let s = h.world_to_screen(gap);
		h.pointer_down_screen(s.x, s.y, 0, false, false);
		h.pointer_move_screen(s.x + 50.0, s.y + 30.0, false, false);
		h.pointer_up_screen(s.x + 50.0, s.y + 30.0, false, false);
		let ev = h.drain_events_json();
		assert!(ev.contains("nodeMove"), "expected bounded drag nodeMove, got: {ev}");
		let zoom = 0.1;
		let dx = 50.0 / zoom;
		let dy = 30.0 / zoom;
		let a = h.nodes.get("a").unwrap();
		let b = h.nodes.get("b").unwrap();
		assert!((a.x - dx).abs() < 1e-3 && (a.y - dy).abs() < 1e-3);
		assert!((b.x - (300.0 + dx)).abs() < 1e-3 && (b.y - dy).abs() < 1e-3);
	}

	#[test]
	fn board_host_overview_bounded_drag_moves_selection_inside_union_bounds() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		h.set_automatic_lod(false);
		h.set_forced_draw_lod_label("overview");
		set_overview_lod(&mut h);
		let mut desc = sample_scene();
		desc.handles.clear();
		desc.edges.clear();
		desc.nodes.push(NodeDescJson {
			id: "b".into(),
			x: 300.0,
			y: 0.0,
			draggable: Some(true),
			selected: None,
			style: None,
			text: None,
			icon_kind: None,
			node_kind: None,
			user_data: None,
			visible: None,
			root: None,
			shape: Some("circle".into()),
			radius: Some(40.0),
			width: None,
			height: None,
			scale: None,
		});
		h.sync_descriptor(&desc).unwrap();
		h.set_selection_ids(&["a".into(), "b".into()]);
		let _ = h.drain_events_json();
		let gap = Point::new(150.0, 0.0);
		assert!(h.resolve_hit_world(gap).is_none());
		let s = h.world_to_screen(gap);
		h.pointer_down_screen(s.x, s.y, 0, false, false);
		h.pointer_move_screen(s.x + 40.0, s.y + 20.0, false, false);
		h.pointer_up_screen(s.x + 40.0, s.y + 20.0, false, false);
		let ev = h.drain_events_json();
		assert!(ev.contains("nodeMove"), "expected overview bounded drag, got: {ev}");
	}

	#[test]
	fn board_host_detail_lod_resolves_direct_handle_hit() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		set_detail_lod(&mut h);
		let mut desc = sample_scene();
		desc.nodes.push(NodeDescJson {
			id: "b".into(),
			x: 300.0,
			y: 0.0,
			draggable: Some(true),
			selected: None,
			style: None,
			text: None,
			icon_kind: None,
			node_kind: None,
			user_data: None,
			visible: None,
			root: None,
			shape: Some("circle".into()),
			radius: Some(40.0),
			width: None,
			height: None,
			scale: None,
		});
		h.sync_descriptor(&desc).unwrap();
		let hp = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
		let probe = Point::new(hp.x + 2.0, hp.y);
		assert_eq!(h.resolve_hit_world(probe).as_deref(), Some("a:h0"));
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
			icon_kind: None,
			node_kind: None,
			user_data: None,
			visible: None,
			root: None,
			shape: Some("circle".into()),
			radius: Some(40.0),
			width: None,
			height: None,
			scale: None,
		});
		h.sync_descriptor(&desc).unwrap();
		h.set_selection_options("rectangle", "additive", true, true, true);
		h.set_selection_ids(&["a".into(), "b".into()]);
		let _ = h.drain_events_json();
		let w = Point::new(0.0, 0.0);
		let s = h.world_to_screen(w);
		h.pointer_down_screen(s.x, s.y, 0, false, false);
		h.pointer_move_screen(s.x + 10.0, s.y + 5.0, false, false);
		h.pointer_up_screen(s.x + 10.0, s.y + 5.0, false, false);
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
		h.set_selection_options("rectangle", "invertive", false, true, false);
		let mut desc = sample_scene();
		desc.nodes.push(NodeDescJson {
			id: "b".into(),
			x: 300.0,
			y: 0.0,
			draggable: Some(true),
			selected: None,
			style: None,
			text: None,
			icon_kind: None,
			node_kind: None,
			user_data: None,
			visible: None,
			root: None,
			shape: Some("circle".into()),
			radius: Some(40.0),
			width: None,
			height: None,
			scale: None,
		});
		h.sync_descriptor(&desc).unwrap();
		let inside_node_a = Point::new(0.0, 0.0);
		assert!(h.resolve_hit_world(inside_node_a).is_none());
		let on_edge = Point::new(150.0, 0.0);
		assert_eq!(h.resolve_hit_world(on_edge).as_deref(), Some("e1"));
	}

	#[test]
	fn board_host_additive_click_merges_edge_into_existing_selection() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		h.set_selection_options("rectangle", "additive", true, true, true);
		let mut desc = sample_scene();
		desc.nodes.push(NodeDescJson {
			id: "b".into(),
			x: 300.0,
			y: 0.0,
			draggable: Some(true),
			selected: None,
			style: None,
			text: None,
			icon_kind: None,
			node_kind: None,
			user_data: None,
			visible: None,
			root: None,
			shape: Some("circle".into()),
			radius: Some(40.0),
			width: None,
			height: None,
			scale: None,
		});
		h.sync_descriptor(&desc).unwrap();
		h.set_selection_ids(&["a".into()]);
		let _ = h.drain_events_json();
		let on_edge = Point::new(150.0, 0.0);
		let s = h.world_to_screen(on_edge);
		h.pointer_down_screen(s.x, s.y, 0, false, false);
		let mut got: Vec<_> = h.selection.iter().cloned().collect();
		got.sort();
		assert_eq!(got, vec!["a".to_string(), "e1".to_string()]);
	}

	#[test]
	fn board_host_background_click_deselect_skips_preselect_events() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		let desc = sample_scene();
		h.sync_descriptor(&desc).unwrap();
		h.set_selection_ids(&["a".into(), "e1".into()]);
		let _ = h.drain_events_json();
		let away = Point::new(5000.0, 5000.0);
		let s = h.world_to_screen(away);
		h.pointer_down_screen(s.x, s.y, 0, false, false);
		assert!(!h.is_dragging_area_select());
		h.pointer_move_screen(s.x + 1.0, s.y, false, false);
		let mid = h.drain_events_json();
		assert!(!mid.contains("preselect"), "background click path must not emit preselect");
		assert!(h.preselect_removed.is_empty());
		assert!(h.selection_exit_highlight.is_empty());
		assert!(h.selection.contains("a"));
		h.pointer_up_screen(s.x, s.y, false, false);
		assert!(h.selection.is_empty());
		assert!(h.selection_exit_highlight.is_empty());
		let fin = h.drain_events_json();
		assert!(fin.contains("select"));
		assert!(!fin.contains("preselect"));
		assert!(fin.contains("\"exitHighlightIds\":[]"));
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
			icon_kind: None,
			node_kind: None,
			user_data: None,
			visible: None,
			root: None,
			shape: Some("circle".into()),
			radius: Some(40.0),
			width: None,
			height: None,
			scale: None,
		});
		h.sync_descriptor(&desc).unwrap();
		h.set_selection_ids(&["a".into(), "e1".into()]);
		let away = Point::new(5000.0, 5000.0);
		let s = h.world_to_screen(away);
		h.pointer_down_screen(s.x, s.y, 0, false, false);
		h.pointer_up_screen(s.x, s.y, false, false);
		assert!(h.selection.is_empty());
	}

	#[test]
	fn board_host_rectangle_area_select_includes_handles_with_nodes() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		h.set_selection_options("rectangle", "invertive", true, true, true);
		let mut desc = sample_scene();
		desc.nodes.push(NodeDescJson {
			id: "b".into(),
			x: 300.0,
			y: 0.0,
			draggable: Some(true),
			selected: None,
			style: None,
			text: None,
			icon_kind: None,
			node_kind: None,
			user_data: None,
			visible: None,
			root: None,
			shape: Some("circle".into()),
			radius: Some(40.0),
			width: None,
			height: None,
			scale: None,
		});
		h.sync_descriptor(&desc).unwrap();
		let _ = h.drain_events_json();
		let w0 = Point::new(-90.0, -70.0);
		let w1 = Point::new(90.0, 90.0);
		let s0 = h.world_to_screen(w0);
		let s1 = h.world_to_screen(w1);
		h.pointer_down_screen(s0.x, s0.y, 0, false, false);
		h.pointer_move_screen(s1.x, s1.y, false, false);
		h.pointer_up_screen(s1.x, s1.y, false, false);
		let mut got: Vec<_> = h.selection.iter().cloned().collect();
		got.sort();
		assert!(got.contains(&"a".to_string()));
		assert!(got.contains(&"a:h0".to_string()));
	}

	#[test]
	fn board_host_area_select_preselect_matches_selected_chrome() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		set_detail_lod(&mut h);
		let mut desc = sample_scene();
		desc.nodes.push(NodeDescJson {
			id: "b".into(),
			x: 300.0,
			y: 0.0,
			draggable: Some(true),
			selected: None,
			style: None,
			text: None,
			icon_kind: None,
			node_kind: None,
			user_data: None,
			visible: None,
			root: None,
			shape: Some("circle".into()),
			radius: Some(40.0),
			width: None,
			height: None,
			scale: None,
		});
		h.sync_descriptor(&desc).unwrap();
		let _ = h.drain_events_json();
		h.set_selection_ids(&["a".into()]);
		let _ = h.drain_events_json();
		assert!(h.preselect_removed.is_empty());
		assert!(h.selection_exit_highlight.is_empty());
		let w_down = Point::new(350.0, -50.0);
		let w_mid = Point::new(270.0, 50.0);
		let w_end = Point::new(265.0, 48.0);
		let s_down = h.world_to_screen(w_down);
		h.pointer_down_screen(s_down.x, s_down.y, 0, false, false);
		assert!(!h.is_dragging_area_select());
		let _ = h.drain_events_json();
		let s_mid = h.world_to_screen(w_mid);
		let s_end = h.world_to_screen(w_end);
		h.pointer_move_screen(s_mid.x, s_mid.y, false, false);
		assert!(h.is_dragging_area_select());
		let _ = h.drain_events_json();
		assert!(h.preselect.contains("b"), "preview should include node b");
		assert!(h.preselect_removed.contains("a"));
		assert!(h.selection_exit_highlight.is_empty());
		assert!(!h.selection.contains("b"), "committed selection unchanged during preselect");
		let frozen = h.preselect_removed.clone();
		h.pointer_move_screen(s_end.x, s_end.y, false, false);
		let _ = h.drain_events_json();
		assert_eq!(frozen, h.preselect_removed);
		h.pointer_up_screen(s_end.x, s_end.y, false, false);
		let _ = h.drain_events_json();
		assert!(h.selection.contains("b"));
		assert!(!h.selection.contains("a"));
		assert!(h.preselect_removed.is_empty());
		assert!(h.selection_exit_highlight.is_empty());
	}

	#[test]
	fn board_host_area_select_from_empty_keeps_selection_until_commit() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		set_detail_lod(&mut h);
		let mut desc = sample_scene();
		desc.nodes.push(NodeDescJson {
			id: "b".into(),
			x: 300.0,
			y: 0.0,
			draggable: Some(true),
			selected: None,
			style: None,
			text: None,
			icon_kind: None,
			node_kind: None,
			user_data: None,
			visible: None,
			root: None,
			shape: Some("circle".into()),
			radius: Some(40.0),
			width: None,
			height: None,
			scale: None,
		});
		h.sync_descriptor(&desc).unwrap();
		h.set_selection_ids(&[]);
		let _ = h.drain_events_json();
		let w_down = Point::new(350.0, -50.0);
		let w_mid = Point::new(270.0, 50.0);
		let s_down = h.world_to_screen(w_down);
		let s_mid = h.world_to_screen(w_mid);
		h.pointer_down_screen(s_down.x, s_down.y, 0, false, false);
		h.pointer_move_screen(s_mid.x, s_mid.y, false, false);
		let _ = h.drain_events_json();
		assert!(h.is_dragging_area_select());
		assert!(h.preselect.contains("b"));
		assert!(h.preselect_removed.is_empty());
		assert!(h.selection.is_empty());
		h.pointer_up_screen(s_mid.x, s_mid.y, false, false);
		let _ = h.drain_events_json();
		assert!(h.selection.contains("b"));
		assert!(h.preselect.is_empty());
	}

	#[test]
	fn board_host_minimap_preselect_matches_selected_chrome() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		h.set_camera(0.0, 0.0, 0.1);
		let mut desc = sample_scene();
		desc.nodes.push(NodeDescJson {
			id: "b".into(),
			x: 300.0,
			y: 0.0,
			draggable: Some(true),
			selected: None,
			style: None,
			text: None,
			icon_kind: None,
			node_kind: None,
			user_data: None,
			visible: None,
			root: None,
			shape: Some("circle".into()),
			radius: Some(40.0),
			width: None,
			height: None,
			scale: None,
		});
		h.sync_descriptor(&desc).unwrap();
		let neutral_hint = h.encoded_scene_hint();
		h.set_selection_ids(&["b".into()]);
		let _ = h.drain_events_json();
		let selected_hint = h.encoded_scene_hint();
		assert!(
			selected_hint > neutral_hint,
			"minimap selected chrome should add visible vector encoding over neutral state"
		);
		h.set_selection_ids(&["a".into()]);
		let _ = h.drain_events_json();
		let w_down = Point::new(350.0, -50.0);
		let w_end = Point::new(265.0, 48.0);
		let s_down = h.world_to_screen(w_down);
		let s_end = h.world_to_screen(w_end);
		h.pointer_down_screen(s_down.x, s_down.y, 0, false, false);
		h.pointer_move_screen(s_end.x, s_end.y, false, false);
		assert!(h.is_dragging_area_select());
		assert!(h.preselect.contains("b"));
		h.set_selection_screen_preview(None);
		let preselect_hint = h.encoded_scene_hint();
		assert!(
			preselect_hint > neutral_hint,
			"minimap preselect should add visible selected chrome over neutral minimap rendering"
		);
	}

	#[test]
	fn board_host_silent_preselect_applies_selected_chrome_without_area_drag() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		h.set_camera(0.0, 0.0, 0.1);
		let mut desc = sample_scene();
		desc.nodes.push(NodeDescJson {
			id: "b".into(),
			x: 300.0,
			y: 0.0,
			draggable: Some(true),
			selected: None,
			style: None,
			text: None,
			icon_kind: None,
			node_kind: None,
			user_data: None,
			visible: None,
			root: None,
			shape: Some("circle".into()),
			radius: Some(40.0),
			width: None,
			height: None,
			scale: None,
		});
		h.sync_descriptor(&desc).unwrap();
		let neutral_hint = h.encoded_scene_hint();
		assert!(!matches!(h.interaction, Interaction::Selection { .. }));
		h.set_preselect_state_silent(&["b".into()], &[]);
		assert!(h.nodes.get("b").is_some_and(|n| n.selected));
		assert!(h.nodes.get("a").is_some_and(|n| !n.selected));
		let preselect_hint = h.encoded_scene_hint();
		assert!(
			preselect_hint > neutral_hint,
			"silent minimap preselect should paint selected chrome without an active area-select interaction"
		);
	}

	#[test]
	fn board_host_hover_tracks_visible_wires() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		set_detail_lod(&mut h);
		let mut desc = sample_scene();
		desc.edges.clear();
		desc.wires.push(WireDescJson {
			id: "w1".into(),
			source: "a:h0".into(),
			target: None,
			end_x: Some(220.0),
			end_y: Some(0.0),
			selected: None,
			style: None,
			wire_kind: None,
			user_data: None,
			visible: None,
		});
		h.sync_descriptor(&desc).unwrap();
		let source = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
		let curve = compute_edge_bezier_points(source, Point::new(220.0, 0.0), Point::new(0.0, 0.0), Point::new(220.0, 0.0));
		let probe = cubic_bezier_point(curve, 0.5);
		h.update_hover_from_world(probe);
		assert_eq!(h.hovered_id.as_deref(), Some("w1"));
	}

	fn link_test_scene_no_edge() -> SceneDescriptorJson {
		SceneDescriptorJson {
			nodes: vec![
				NodeDescJson {
					id: "a".into(),
					x: 0.0,
					y: 0.0,
					draggable: Some(true),
					selected: None,
					style: None,
					text: None,
					icon_kind: None,
					node_kind: None,
					user_data: None,
					visible: None,
					root: None,
					shape: Some("circle".into()),
					radius: Some(40.0),
					width: None,
					height: None,
					scale: None,
				},
				NodeDescJson {
					id: "b".into(),
					x: 280.0,
					y: 0.0,
					draggable: Some(true),
					selected: None,
					style: None,
					text: None,
					icon_kind: None,
					node_kind: None,
					user_data: None,
					visible: None,
					root: None,
					shape: Some("circle".into()),
					radius: Some(40.0),
					width: None,
					height: None,
					scale: None,
				},
			],
			handles: vec![
				HandleDescJson {
					id: "a:h0".into(),
					node_id: "a".into(),
					angle: 0.0,
					radius: None,
					selected: None,
					style: None,
					handle_kind: Some("parent".into()),
					color: None,
					icon_kind: None,
					user_data: None,
					visible: None,
					scale: None,
				},
				HandleDescJson {
					id: "b:h0".into(),
					node_id: "b".into(),
					angle: std::f64::consts::PI,
					radius: None,
					selected: None,
					style: None,
					handle_kind: Some("child".into()),
					color: None,
					icon_kind: None,
					user_data: None,
					visible: None,
					scale: None,
				},
			],
			edges: vec![],
			wires: vec![],
			selection_exit_highlight_ids: vec![],
		}
	}

	fn link_test_scene_no_edge_non_draggable_nodes() -> SceneDescriptorJson {
		let mut s = link_test_scene_no_edge();
		for n in &mut s.nodes {
			n.draggable = Some(false);
		}
		s
	}

	fn link_test_scene_node_a_two_free_handles() -> SceneDescriptorJson {
		let mut s = link_test_scene_no_edge();
		s.handles.push(HandleDescJson {
			id: "a:h1".into(),
			node_id: "a".into(),
			angle: std::f64::consts::FRAC_PI_2,
			radius: None,
			selected: None,
			style: None,
			handle_kind: Some("parent".into()),
			color: None,
			icon_kind: None,
			user_data: None,
			visible: None,
			scale: None,
		});
		s
	}

	fn link_test_scene_b_two_free_child_handles() -> SceneDescriptorJson {
		let mut s = link_test_scene_no_edge();
		s.handles.push(HandleDescJson {
			id: "b:h1".into(),
			node_id: "b".into(),
			angle: 0.0,
			radius: None,
			selected: None,
			style: None,
			handle_kind: Some("child".into()),
			color: None,
			icon_kind: None,
			user_data: None,
			visible: None,
			scale: None,
		});
		s
	}

	fn link_test_scene_target_b_handle_busy() -> SceneDescriptorJson {
		let mut s = link_test_scene_no_edge();
		s.nodes.push(NodeDescJson {
			id: "c".into(),
			x: 560.0,
			y: 0.0,
			draggable: Some(true),
			selected: None,
			style: None,
			text: None,
			icon_kind: None,
			node_kind: None,
			user_data: None,
			visible: None,
			root: None,
			shape: Some("circle".into()),
			radius: Some(40.0),
			width: None,
			height: None,
			scale: None,
		});
		s.handles.push(HandleDescJson {
			id: "c:h0".into(),
			node_id: "c".into(),
			angle: std::f64::consts::PI,
			radius: None,
			selected: None,
			style: None,
			handle_kind: Some("child".into()),
			color: None,
			icon_kind: None,
			user_data: None,
			visible: None,
			scale: None,
		});
		s.edges.push(EdgeDescJson {
			id: "e-bc".into(),
			source: "b:h0".into(),
			target: "c:h0".into(),
			edge_kind: None,
			selected: None,
			style: None,
			user_data: None,
			visible: None,
		});
		s
	}

	fn link_test_scene_a_to_b_linked() -> SceneDescriptorJson {
		let mut s = link_test_scene_no_edge();
		s.edges.push(EdgeDescJson {
			id: "e-ab".into(),
			source: "a:h0".into(),
			target: "b:h0".into(),
			edge_kind: None,
			selected: None,
			style: None,
			user_data: None,
			visible: None,
		});
		s
	}

	fn link_test_scene_node_a_two_handles_one_busy() -> SceneDescriptorJson {
		let mut s = link_test_scene_a_to_b_linked();
		s.handles.push(HandleDescJson {
			id: "a:h1".into(),
			node_id: "a".into(),
			angle: std::f64::consts::FRAC_PI_2,
			radius: None,
			selected: None,
			style: None,
			handle_kind: Some("parent".into()),
			color: None,
			icon_kind: None,
			user_data: None,
			visible: None,
			scale: None,
		});
		s
	}

	#[test]
	fn board_host_node_drag_proximity_connect_overlapping_compatible_handles() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		set_detail_lod(&mut h);
		h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#)
			.unwrap();
		h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
		let _ = h.drain_events_json();
		let center_b = h.world_to_screen(Point::new(280.0, 0.0));
		h.pointer_down_screen(center_b.x, center_b.y, 0, false, false);
		let overlap = h.world_to_screen(Point::new(60.0, 0.0));
		h.pointer_move_screen(overlap.x, overlap.y, false, false);
		assert!(
			matches!(
				h.interaction,
				Interaction::DragNodes {
					proximity_pair: Some(_),
					..
				}
			),
			"expected proximity preview wire while overlapping compatible nodes"
		);
		h.pointer_up_screen(overlap.x, overlap.y, false, false);
		let ev = h.drain_events_json();
		assert!(ev.contains("edgeCreate"), "expected edgeCreate, got: {ev}");
		assert!(ev.contains("proximityConnect"), "expected proximityConnect, got: {ev}");
		assert!(ev.contains("b:h0"));
		assert!(ev.contains("a:h0"));
	}

	#[test]
	fn board_host_node_drag_skips_proximity_when_moving_node_has_incident_edge() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		set_detail_lod(&mut h);
		h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#)
			.unwrap();
		h.sync_descriptor(&link_test_scene_a_to_b_linked()).unwrap();
		let _ = h.drain_events_json();
		let center_b = h.world_to_screen(Point::new(280.0, 0.0));
		h.pointer_down_screen(center_b.x, center_b.y, 0, false, false);
		let overlap = h.world_to_screen(Point::new(60.0, 0.0));
		h.pointer_move_screen(overlap.x, overlap.y, false, false);
		assert!(
			matches!(
				h.interaction,
				Interaction::DragNodes {
					proximity_pair: None,
					..
				}
			),
			"connected moving node must not preview node-drag proximity"
		);
		h.pointer_up_screen(overlap.x, overlap.y, false, false);
		let ev = h.drain_events_json();
		assert!(!ev.contains("proximityConnect"), "expected no proximityConnect, got: {ev}");
	}

	#[test]
	fn board_host_link_drag_snap_emits_edge_create() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		set_detail_lod(&mut h);
		h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
		let _ = h.drain_events_json();
		let hp_a = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
		let hp_b = handle_position_on_circle(Point::new(280.0, 0.0), 40.0, std::f64::consts::PI);
		let s0 = h.world_to_screen(hp_a);
		h.pointer_down_screen(s0.x, s0.y, 0, false, false);
		let s_mid = h.world_to_screen(Point::new(140.0, 0.0));
		h.pointer_move_screen(s_mid.x + 20.0, s_mid.y, false, false);
		let s1 = h.world_to_screen(hp_b);
		h.pointer_move_screen(s1.x, s1.y, false, false);
		h.pointer_up_screen(s1.x, s1.y, false, false);
		let ev = h.drain_events_json();
		assert!(ev.contains("edgeCreate"));
		assert!(ev.contains("proximityConnect"));
		assert!(ev.contains("a:h0"));
		assert!(ev.contains("b:h0"));
		let created: Vec<_> = h.edges.keys().filter(|k| k.starts_with("edge-link-")).cloned().collect();
		assert_eq!(created.len(), 1);
	}

	#[test]
	fn board_host_link_drag_snap_micro_zoom_rectangle_compatible_handles() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		set_micro_lod(&mut h);
		h.set_board_kind_catalogs_from_json(
			&serde_json::json!({
				"handleKinds": [
					{"id":"core.rect.bottom","label":"B","color":"#112233","defaultWireKind":"link.w"},
					{"id":"core.rect.top","label":"T","color":"#112233","defaultWireKind":"link.w"}
				],
				"wireKinds": [{"id":"link.w","label":"W","defaultEdgeKind":"link.e"}],
			})
			.to_string(),
		)
		.unwrap();
		h.set_handle_link_compat_from_json(
			r#"[{"source":"core.rect.bottom","target":"core.rect.top","specificity":"handle"}]"#,
		)
		.unwrap();
		let desc = SceneDescriptorJson {
			nodes: vec![
				NodeDescJson {
					id: "a".into(),
					x: 0.0,
					y: 100.0,
					draggable: Some(true),
					selected: None,
					style: None,
					text: None,
					icon_kind: None,
					node_kind: None,
					user_data: None,
					visible: None,
					root: None,
					shape: Some("rectangle".into()),
					radius: None,
					width: Some(100.0),
					height: Some(56.0),
					scale: None,
				},
				NodeDescJson {
					id: "b".into(),
					x: 0.0,
					y: 20.0,
					draggable: Some(true),
					selected: None,
					style: None,
					text: None,
					icon_kind: None,
					node_kind: None,
					user_data: None,
					visible: None,
					root: None,
					shape: Some("rectangle".into()),
					radius: None,
					width: Some(100.0),
					height: Some(56.0),
					scale: None,
				},
			],
			handles: vec![
				HandleDescJson {
					id: "a:h0".into(),
					node_id: "a".into(),
					angle: std::f64::consts::PI,
					radius: None,
					selected: None,
					style: None,
					handle_kind: Some("core.rect.bottom".into()),
					color: None,
					icon_kind: None,
					user_data: None,
					visible: None,
					scale: None,
				},
				HandleDescJson {
					id: "b:h0".into(),
					node_id: "b".into(),
					angle: 0.0,
					radius: None,
					selected: None,
					style: None,
					handle_kind: Some("core.rect.top".into()),
					color: None,
					icon_kind: None,
					user_data: None,
					visible: None,
					scale: None,
				},
			],
			edges: vec![],
			wires: vec![],
			selection_exit_highlight_ids: vec![],
		};
		h.sync_descriptor(&desc).unwrap();
		let _ = h.drain_events_json();
		let pa = handle_position_on_rectangle(Point::new(0.0, 100.0), 100.0, 56.0, std::f64::consts::PI);
		let pb = handle_position_on_rectangle(Point::new(0.0, 20.0), 100.0, 56.0, 0.0);
		let s0 = h.world_to_screen(pa);
		h.pointer_down_screen(s0.x, s0.y, 0, false, false);
		let mid = Point::new(0.0, 60.0);
		let s_mid = h.world_to_screen(mid);
		h.pointer_move_screen(s_mid.x, s_mid.y, false, false);
		let s1 = h.world_to_screen(pb);
		h.pointer_move_screen(s1.x, s1.y, false, false);
		assert!(
			matches!(h.interaction, Interaction::LinkDragSnap { ref target_id, .. } if target_id.as_deref() == Some("b:h0")),
			"expected drag snap onto b:h0 at micro zoom"
		);
		h.pointer_up_screen(s1.x, s1.y, false, false);
		let ev = h.drain_events_json();
		assert!(ev.contains("edgeCreate"), "expected edgeCreate, got: {ev}");
		assert!(ev.contains("proximityConnect"), "expected proximityConnect, got: {ev}");
	}

	#[test]
	fn board_host_link_drag_snap_proximity_connect_in_overview_lod() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		set_overview_lod(&mut h);
		h.sync_descriptor(&link_test_scene_no_edge_non_draggable_nodes()).unwrap();
		let _ = h.drain_events_json();
		let center_a = h.world_to_screen(Point::new(0.0, 0.0));
		h.pointer_down_screen(center_a.x, center_a.y, 0, false, false);
		h.pointer_up_screen(center_a.x, center_a.y, false, false);
		let _ = h.drain_events_json();
		let hp_a = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
		let hp_b = handle_position_on_circle(Point::new(280.0, 0.0), 40.0, std::f64::consts::PI);
		let s0 = h.world_to_screen(hp_a);
		h.pointer_down_screen(s0.x, s0.y, 0, false, false);
		let s_mid = h.world_to_screen(Point::new(140.0, 0.0));
		h.pointer_move_screen(s_mid.x + 20.0, s_mid.y, false, false);
		let s1 = h.world_to_screen(hp_b);
		h.pointer_move_screen(s1.x, s1.y, false, false);
		h.pointer_up_screen(s1.x, s1.y, false, false);
		let ev = h.drain_events_json();
		assert!(ev.contains("edgeCreate"), "expected edgeCreate at overview LOD, got: {ev}");
		assert!(
			ev.contains("proximityConnect") || ev.contains("indirectConnect"),
			"expected proximityConnect or indirectConnect, got: {ev}"
		);
	}

	#[test]
	fn board_host_hidden_handle_blocks_proximity_connect() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		set_detail_lod(&mut h);
		let fixture = json!({
			"schema": "elements.board.fixture/v1",
			"camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
			"nodes": [
				{
					"id": "a",
					"x": 0.0,
					"y": 0.0,
					"radius": 40.0,
					"handles": [{ "id": "a:h0", "angle": 0.0, "handleKind": "board.port" }]
				},
				{
					"id": "b",
					"x": 280.0,
					"y": 0.0,
					"radius": 40.0,
					"handles": [{ "id": "b:h0", "angle": 3.14159, "handleKind": "board.port", "hidden": true }]
				}
			],
			"edges": []
		});
		assert!(h.parse_fixture_v1(&fixture));
		let _ = h.drain_events_json();
		let hp_a = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
		let hp_b = handle_position_on_circle(Point::new(280.0, 0.0), 40.0, std::f64::consts::PI);
		let s0 = h.world_to_screen(hp_a);
		h.pointer_down_screen(s0.x, s0.y, 0, false, false);
		let s_mid = h.world_to_screen(Point::new(140.0, 0.0));
		h.pointer_move_screen(s_mid.x + 20.0, s_mid.y, false, false);
		let s1 = h.world_to_screen(hp_b);
		h.pointer_move_screen(s1.x, s1.y, false, false);
		h.pointer_up_screen(s1.x, s1.y, false, false);
		let ev = h.drain_events_json();
		assert!(!ev.contains("edgeCreate"), "hidden handle should block connect, got: {ev}");
		assert!(h.edges.is_empty());
	}

	#[test]
	fn board_host_hidden_node_blocks_indirect_connect() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		h.set_camera(0.0, 0.0, 1.0);
		h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
		let fixture = json!({
			"schema": "elements.board.fixture/v1",
			"camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
			"nodes": [
				{
					"id": "a",
					"x": 0.0,
					"y": 0.0,
					"radius": 40.0,
					"handles": [{ "id": "a:h0", "angle": 0.0, "handleKind": "parent" }]
				},
				{
					"id": "b",
					"x": 280.0,
					"y": 0.0,
					"radius": 40.0,
					"hidden": true,
					"handles": [{ "id": "b:h0", "angle": 3.14159, "handleKind": "child" }]
				}
			],
			"edges": []
		});
		assert!(h.parse_fixture_v1(&fixture));
		let _ = h.drain_events_json();
		h.set_selection_ids(&["a".into()]);
		let inside_a = h.world_to_screen(Point::new(0.0, 0.0));
		h.pointer_down_screen(inside_a.x, inside_a.y, 0, false, false);
		let inside_b = h.world_to_screen(Point::new(280.0, 0.0));
		h.pointer_move_screen(inside_b.x, inside_b.y, false, false);
		h.pointer_up_screen(inside_b.x, inside_b.y, false, false);
		let ev = h.drain_events_json();
		assert!(!ev.contains("edgeCreate"), "hidden node should block indirect connect, got: {ev}");
		assert!(matches!(h.interaction, Interaction::None));
		assert!(h.edges.is_empty());
	}

	#[test]
	fn board_host_overview_lod_omits_direct_handle_resolve_hit() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		set_overview_lod(&mut h);
		h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
		let hp = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
		let probe = Point::new(hp.x + 3.0, hp.y);
		assert_ne!(h.resolve_hit_world(probe).as_deref(), Some("a:h0"));
	}

	#[test]
	fn board_host_link_rejects_incompatible_handle_kind_pairs() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		set_detail_lod(&mut h);
		h.set_handle_link_compat_from_json(r#"[{"source":"child","target":"parent"}]"#).unwrap();
		let desc = link_test_scene_no_edge();
		h.sync_descriptor(&desc).unwrap();
		let _ = h.drain_events_json();
		let hp_a = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
		let hp_b = handle_position_on_circle(Point::new(280.0, 0.0), 40.0, std::f64::consts::PI);
		let s0 = h.world_to_screen(hp_a);
		h.pointer_down_screen(s0.x, s0.y, 0, false, false);
		let s_mid = h.world_to_screen(Point::new(140.0, 0.0));
		h.pointer_move_screen(s_mid.x + 20.0, s_mid.y, false, false);
		let s1 = h.world_to_screen(hp_b);
		h.pointer_move_screen(s1.x, s1.y, false, false);
		h.pointer_up_screen(s1.x, s1.y, false, false);
		let ev = h.drain_events_json();
		assert!(!ev.contains("edgeCreate"));
	}

	#[test]
	fn board_host_link_accepts_matching_handle_kind_pair() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		set_detail_lod(&mut h);
		h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
		let desc = link_test_scene_no_edge();
		h.sync_descriptor(&desc).unwrap();
		let _ = h.drain_events_json();
		let hp_a = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
		let hp_b = handle_position_on_circle(Point::new(280.0, 0.0), 40.0, std::f64::consts::PI);
		let s0 = h.world_to_screen(hp_a);
		h.pointer_down_screen(s0.x, s0.y, 0, false, false);
		let s_mid = h.world_to_screen(Point::new(140.0, 0.0));
		h.pointer_move_screen(s_mid.x + 20.0, s_mid.y, false, false);
		let s1 = h.world_to_screen(hp_b);
		h.pointer_move_screen(s1.x, s1.y, false, false);
		h.pointer_up_screen(s1.x, s1.y, false, false);
		let ev = h.drain_events_json();
		assert!(ev.contains("edgeCreate"));
		assert!(ev.contains("proximityConnect"));
	}

	#[test]
	fn board_host_normal_lod_prefers_node_at_center_and_handle_off_rim() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
		assert_eq!(h.resolve_hit_world(Point::new(0.0, 0.0)).as_deref(), Some("a"));
		let hp = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
		let probe = Point::new(hp.x + 2.0, hp.y);
		assert_eq!(h.resolve_hit_world(probe).as_deref(), Some("a:h0"));
	}

	#[test]
	fn board_host_indirect_ring_resolve_skips_connected_handles() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		h.set_camera(0.0, 0.0, 1.0);
		h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
		h.sync_descriptor(&link_test_scene_node_a_two_handles_one_busy()).unwrap();
		let _ = h.drain_events_json();
		h.set_selection_ids(&["a".into()]);
		let ha0 = h.handles.get("a:h0").unwrap();
		let ring_busy = h.indirect_handle_world_pos(ha0).unwrap();
		assert_ne!(h.resolve_hit_world(ring_busy).as_deref(), Some("a:h0"));
		assert_eq!(h.resolve_hit_world(Point::new(0.0, 0.0)).as_deref(), Some("a:h1"));
	}

	#[test]
	fn board_host_indirect_sole_compatible_drop_creates_edge_immediately() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		h.set_camera(0.0, 0.0, 1.0);
		h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
		h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
		let _ = h.drain_events_json();
		h.set_selection_ids(&["a".into()]);
		let inside_a = h.world_to_screen(Point::new(0.0, 0.0));
		h.pointer_down_screen(inside_a.x, inside_a.y, 0, false, false);
		assert!(matches!(
			h.interaction,
			Interaction::LinkAtSourceHandle { ref source_id, .. } if source_id == "a:h0"
		));
		let inside_b = h.world_to_screen(Point::new(280.0, 0.0));
		h.pointer_move_screen(inside_b.x, inside_b.y, false, false);
		assert!(matches!(h.interaction, Interaction::LinkDragSnap { .. }));
		h.pointer_up_screen(inside_b.x, inside_b.y, false, false);
		assert!(matches!(h.interaction, Interaction::None));
		let ev = h.drain_events_json();
		assert!(ev.contains("edgeCreate"));
		assert!(ev.contains("indirectConnect"));
		assert!(ev.contains("a:h0"));
		assert!(ev.contains("b:h0"));
	}

	#[test]
	fn board_host_indirect_two_compatible_child_handles_on_target_require_ring_pick() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		h.set_camera(0.0, 0.0, 1.0);
		h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
		h.sync_descriptor(&link_test_scene_b_two_free_child_handles()).unwrap();
		let _ = h.drain_events_json();
		h.set_selection_ids(&["a".into()]);
		let sa = h.world_to_screen(Point::new(0.0, 0.0));
		h.pointer_down_screen(sa.x, sa.y, 0, false, false);
		let sb = h.world_to_screen(Point::new(280.0, 0.0));
		h.pointer_move_screen(sb.x, sb.y, false, false);
		h.pointer_up_screen(sb.x, sb.y, false, false);
		assert!(matches!(
			h.interaction,
			Interaction::LinkTargetNode { ref target_node_id, .. } if target_node_id == "b"
		));
		let b0 = h.handles.get("b:h0").unwrap();
		let ring0 = h.indirect_handle_world_pos(b0).unwrap();
		let s1 = h.world_to_screen(ring0);
		h.pointer_down_screen(s1.x, s1.y, 0, false, false);
		let ev = h.drain_events_json();
		assert!(ev.contains("edgeCreate"));
		assert!(ev.contains("indirectConnect"));
		assert!(ev.contains("a:h0"));
		assert!(ev.contains("b:h0"));
	}

	#[test]
	fn board_host_indirect_target_click_elsewhere_stops_wire() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		h.set_camera(0.0, 0.0, 1.0);
		h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
		h.sync_descriptor(&link_test_scene_b_two_free_child_handles()).unwrap();
		h.set_selection_ids(&["a".into()]);
		let sa = h.world_to_screen(Point::new(0.0, 0.0));
		h.pointer_down_screen(sa.x, sa.y, 0, false, false);
		let target_center = h.world_to_screen(Point::new(280.0, 0.0));
		h.pointer_move_screen(target_center.x, target_center.y, false, false);
		h.pointer_up_screen(target_center.x, target_center.y, false, false);
		assert!(matches!(h.interaction, Interaction::LinkTargetNode { .. }));
		h.pointer_down_screen(20.0, 20.0, 0, false, false);
		assert!(matches!(h.interaction, Interaction::None));
		assert!(h.edges.is_empty());
	}

	#[test]
	fn board_host_indirect_ring_shown_when_node_has_two_free_handles() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		h.set_camera(0.0, 0.0, 1.0);
		h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
		h.sync_descriptor(&link_test_scene_node_a_two_free_handles()).unwrap();
		let _ = h.drain_events_json();
		h.set_selection_ids(&["a".into()]);
		let ha0 = h.handles.get("a:h0").unwrap();
		let ring = h.indirect_handle_world_pos(ha0).unwrap();
		assert_eq!(h.resolve_hit_world(ring).as_deref(), Some("a:h0"));
	}

	#[test]
	fn board_host_link_drag_emits_compatible_nodes_and_target_ring_events() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		h.set_camera(0.0, 0.0, 1.0);
		h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
		h.sync_descriptor(&link_test_scene_b_two_free_child_handles()).unwrap();
		let _ = h.drain_events_json();
		h.set_selection_ids(&["a".into()]);
		let sa = h.world_to_screen(Point::new(0.0, 0.0));
		h.pointer_down_screen(sa.x, sa.y, 0, false, false);
		let sb = h.world_to_screen(Point::new(280.0, 0.0));
		h.pointer_move_screen(sb.x, sb.y, false, false);
		let ev = h.drain_events_json();
		assert!(ev.contains("linkCompatibleNodes"), "got: {ev}");
		assert!(ev.contains(r#""nodeIds":["b"]"#) || ev.contains(r#""nodeIds": ["b"]"#), "got: {ev}");
		assert!(ev.contains("linkTargetRing"), "got: {ev}");
		assert!(ev.contains("b:h0") && ev.contains("b:h1"), "got: {ev}");
		let ring = h.indirect_handle_world_pos(h.handles.get("b:h1").unwrap()).unwrap();
		assert_eq!(h.resolve_hit_world(ring).as_deref(), Some("b:h1"));
		h.pointer_up_screen(20.0, 20.0, false, false);
		let ev_end = h.drain_events_json();
		assert!(ev_end.contains("linkCompatibleNodes"));
		assert!(ev_end.contains(r#""nodeIds":[]"#) || ev_end.contains(r#""nodeIds": []"#));
		assert!(ev_end.contains("linkTargetRing"));
	}

	#[test]
	fn board_host_indirect_ring_gap_scales_with_node_across_zoom() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
		let ha = h.handles.get("a:h0").unwrap().clone();
		let node_r = 40.0_f64;
		let body = || handle_position_on_circle(Point::new(0.0, 0.0), node_r, 0.0);
		let gap_ratio = |host: &BoardHost| {
			let ring = host.indirect_handle_world_pos(&ha).unwrap();
			let gap_px = distance_between(host.world_to_screen(ring), host.world_to_screen(body()));
			gap_px / (node_r * host.camera.zoom)
		};
		h.set_camera(0.0, 0.0, 1.0);
		let ratio_z1 = gap_ratio(&h);
		let gap_px_z1 = node_r * ratio_z1;
		h.set_camera(0.0, 0.0, 4.25);
		let ratio_z2 = gap_ratio(&h);
		let gap_px_z2 = node_r * 4.25 * ratio_z2;
		assert!(
			(ratio_z1 - ratio_z2).abs() < 1e-6,
			"rim-to-ring ratios differ: {ratio_z1} vs {ratio_z2}"
		);
		assert!((ratio_z1 - 0.7).abs() < 1e-6);
		assert!((gap_px_z2 - gap_px_z1 * 4.25).abs() < 0.6, "screen gap should scale with zoom: {gap_px_z1} vs {gap_px_z2}");
	}

	#[test]
	fn board_host_indirect_handle_marker_radius_scales_with_node_extent() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
		let ha = h.handles.get("a:h0").unwrap();
		assert!((h.indirect_handle_marker_radius_world(ha) - 32.0).abs() < 1e-6);
	}

	#[test]
	fn board_host_handle_scale_combines_node_and_kind_scales() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		h.set_board_kind_catalogs_from_json(
			&serde_json::json!({
				"handleKinds": [{"id":"slot-a","label":"Slot A","color":"#112233","scale":2.0}],
				"nodeKinds": [{"id":"kind-a","label":"Kind A","scale":1.5}],
			})
			.to_string(),
		)
		.unwrap();
		let mut desc = link_test_scene_no_edge();
		desc.nodes[0].node_kind = Some("kind-a".into());
		desc.nodes[0].scale = Some(2.0);
		desc.handles[0].handle_kind = Some("slot-a".into());
		desc.handles[0].scale = Some(0.5);
		h.sync_descriptor(&desc).unwrap();
		let ha = h.handles.get("a:h0").unwrap();
		assert_eq!(h.resolve_hit_world(Point::new(120.0, 0.0)).as_deref(), Some("a:h0"));
		assert!((h.indirect_handle_marker_radius_world(ha) - 96.0).abs() < 1e-6);
	}

	#[test]
	fn board_host_link_wire_specificity_allows_when_handle_row_absent() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		set_detail_lod(&mut h);
		h.set_board_kind_catalogs_from_json(
			&serde_json::json!({
				"handleKinds": [{"id":"parent","label":"P","color":"#112233","defaultWireKind":"flow.wire"}],
				"wireKinds": [{"id":"flow.wire","label":"W","defaultEdgeKind":"flow.edge"}],
			})
			.to_string(),
		)
		.unwrap();
		h.set_handle_link_compat_from_json(r#"[{"source":"flow.wire","target":"child","specificity":"wire"}]"#)
			.unwrap();
		let desc = link_test_scene_no_edge();
		h.sync_descriptor(&desc).unwrap();
		let _ = h.drain_events_json();
		let hp_a = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
		let hp_b = handle_position_on_circle(Point::new(280.0, 0.0), 40.0, std::f64::consts::PI);
		let s0 = h.world_to_screen(hp_a);
		h.pointer_down_screen(s0.x, s0.y, 0, false, false);
		let s_mid = h.world_to_screen(Point::new(140.0, 0.0));
		h.pointer_move_screen(s_mid.x + 20.0, s_mid.y, false, false);
		let s1 = h.world_to_screen(hp_b);
		h.pointer_move_screen(s1.x, s1.y, false, false);
		h.pointer_up_screen(s1.x, s1.y, false, false);
		let ev = h.drain_events_json();
		assert!(ev.contains("edgeCreate"));
		assert!(ev.contains("proximityConnect"));
	}

	#[test]
	fn board_host_kind_catalog_accepts_modern_hsl_handle_colors() {
		let mut h = BoardHost::new();
		h.set_board_kind_catalogs_from_json(
			&serde_json::json!({
				"handleKinds": [
					{"id":"space","label":"S","color":"hsl(206 52% 48%)"},
					{"id":"comma","label":"C","color":"hsl(206, 52%, 48%)"},
					{"id":"slash","label":"Sl","color":"hsl(206 52% 48% / 0.5)"},
				],
			})
			.to_string(),
		)
		.unwrap();
		let c_space = h.handle_kinds.get("space").expect("space").color;
		let c_comma = h.handle_kinds.get("comma").expect("comma").color;
		let c_slash = h.handle_kinds.get("slash").expect("slash").color;
		assert_eq!(c_space, c_comma);
		assert_ne!(c_space, c_slash);
	}

	#[test]
	fn board_host_link_important_pair_overrides_lower_specificity_filter() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		set_detail_lod(&mut h);
		h.set_board_kind_catalogs_from_json(
			&serde_json::json!({
				"handleKinds": [{"id":"parent","label":"P","color":"#112233","defaultWireKind":"flow.wire"}],
				"wireKinds": [{"id":"flow.wire","label":"W"}],
			})
			.to_string(),
		)
		.unwrap();
		h.set_handle_link_compat_from_json(
			r#"[
				{"source":"flow.wire","target":"nope","specificity":"wire"},
				{"source":"parent","target":"child","specificity":"general","important":true}
			]"#,
		)
		.unwrap();
		let desc = link_test_scene_no_edge();
		h.sync_descriptor(&desc).unwrap();
		let _ = h.drain_events_json();
		let hp_a = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
		let hp_b = handle_position_on_circle(Point::new(280.0, 0.0), 40.0, std::f64::consts::PI);
		let s0 = h.world_to_screen(hp_a);
		h.pointer_down_screen(s0.x, s0.y, 0, false, false);
		let s_mid = h.world_to_screen(Point::new(140.0, 0.0));
		h.pointer_move_screen(s_mid.x + 20.0, s_mid.y, false, false);
		let s1 = h.world_to_screen(hp_b);
		h.pointer_move_screen(s1.x, s1.y, false, false);
		h.pointer_up_screen(s1.x, s1.y, false, false);
		let ev = h.drain_events_json();
		assert!(ev.contains("edgeCreate"));
		assert!(ev.contains("proximityConnect"));
	}

	#[test]
	fn board_host_link_drag_does_not_snap_when_target_handle_busy() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		set_detail_lod(&mut h);
		h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
		h.sync_descriptor(&link_test_scene_target_b_handle_busy()).unwrap();
		let _ = h.drain_events_json();
		let hp_a = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
		let hp_b = handle_position_on_circle(Point::new(280.0, 0.0), 40.0, std::f64::consts::PI);
		let s0 = h.world_to_screen(hp_a);
		h.pointer_down_screen(s0.x, s0.y, 0, false, false);
		let s_mid = h.world_to_screen(Point::new(140.0, 0.0));
		h.pointer_move_screen(s_mid.x + 20.0, s_mid.y, false, false);
		let s1 = h.world_to_screen(hp_b);
		h.pointer_move_screen(s1.x, s1.y, false, false);
		h.pointer_up_screen(s1.x, s1.y, false, false);
		let ev = h.drain_events_json();
		assert!(!ev.contains("edgeCreate"));
		assert_eq!(h.edges.len(), 1);
		assert!(h.edges.contains_key("e-bc"));
	}

	#[test]
	fn board_host_link_does_not_start_from_busy_source_handle() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		set_detail_lod(&mut h);
		h.sync_descriptor(&link_test_scene_a_to_b_linked()).unwrap();
		let _ = h.drain_events_json();
		let hp_a = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
		let s0 = h.world_to_screen(hp_a);
		h.pointer_down_screen(s0.x, s0.y, 0, false, false);
		assert!(matches!(h.interaction, Interaction::None));
		assert!(!h.drain_events_json().contains("edgeCreate"));
	}

	#[test]
	fn board_host_indirect_does_not_commit_on_busy_target_handle() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		h.set_camera(0.0, 0.0, 1.0);
		h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
		h.sync_descriptor(&link_test_scene_target_b_handle_busy()).unwrap();
		let _ = h.drain_events_json();
		h.set_selection_ids(&["a".into()]);
		let sa = h.world_to_screen(Point::new(0.0, 0.0));
		h.pointer_down_screen(sa.x, sa.y, 0, false, false);
		let target_center = h.world_to_screen(Point::new(280.0, 0.0));
		h.pointer_move_screen(target_center.x, target_center.y, false, false);
		h.pointer_up_screen(target_center.x, target_center.y, false, false);
		assert!(matches!(
			h.interaction,
			Interaction::LinkTargetNode {
				ref source_id,
				ref target_node_id
			} if source_id == "a:h0" && target_node_id == "b"
		));
		let _ = h.drain_events_json();
		let sb = h.world_to_screen(Point::new(280.0, 0.0));
		h.pointer_down_screen(sb.x, sb.y, 0, false, false);
		let ev = h.drain_events_json();
		assert!(!ev.contains("edgeCreate"));
		assert_eq!(h.edges.len(), 1);
		assert!(matches!(h.interaction, Interaction::None));
	}

	#[test]
	fn board_host_link_short_drag_does_not_emit_edge_create() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		set_detail_lod(&mut h);
		h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
		let _ = h.drain_events_json();
		let hp_a = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
		let s0 = h.world_to_screen(hp_a);
		h.pointer_down_screen(s0.x, s0.y, 0, false, false);
		h.pointer_move_screen(s0.x + 2.0, s0.y, false, false);
		h.pointer_up_screen(s0.x + 2.0, s0.y, false, false);
		let ev = h.drain_events_json();
		assert!(!ev.contains("edgeCreate"));
	}
}

#[cfg(test)]
mod force_graph_tests {
	use super::{
		apply_edge_handle_snap_to_fixture_v1_json, apply_force_graph_layout_to_fixture_v1_json, apply_redraw_layout_to_fixture_v1_json,
	};
	use serde_json::json;
	use std::collections::HashMap;

	#[test]
	fn force_graph_spreads_two_linked_circles_along_x() {
		let fixture = json!({
			"schema": "elements.board.fixture/v1",
			"camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
			"nodes": [
				{
					"id": "a",
					"x": 0.0,
					"y": 0.0,
					"radius": 40.0,
					"handles": [{ "id": "a:h0", "angle": 0.0, "handleKind": "board.port" }]
				},
				{
					"id": "b",
					"x": 1.0,
					"y": 0.0,
					"radius": 40.0,
					"handles": [{ "id": "b:h0", "angle": 3.14159, "handleKind": "board.port" }]
				}
			],
			"edges": [{ "id": "e1", "source": "a:h0", "target": "b:h0" }]
		});
		let opts = json!({
			"iterations": 200,
			"idealEdgeLength": 180.0,
			"repulsionStrength": 8000.0,
			"springStrength": 0.04,
			"gravity": 0.0,
			"randomSeed": 7
		});
		let out = apply_force_graph_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
		let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
		let nodes = parsed["nodes"].as_array().unwrap();
		let ax = nodes[0]["x"].as_f64().unwrap();
		let bx = nodes[1]["x"].as_f64().unwrap();
		assert!((bx - ax).abs() > 80.0, "expected horizontal separation, got a={ax} b={bx}");
	}

	#[test]
	fn force_graph_pins_locked_node_positions() {
		let fixture = json!({
			"schema": "elements.board.fixture/v1",
			"camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
			"nodes": [
				{
					"id": "a",
					"x": 0.0,
					"y": 0.0,
					"radius": 35.0,
					"handles": [{ "id": "a:h0", "angle": 0.0, "handleKind": "board.port" }]
				},
				{
					"id": "b",
					"x": 40.0,
					"y": 0.0,
					"radius": 35.0,
					"handles": [{ "id": "b:h0", "angle": 3.14159, "handleKind": "board.port" }]
				}
			],
			"edges": [{ "id": "e1", "source": "a:h0", "target": "b:h0" }]
		});
		let opts = json!({
			"iterations": 180,
			"idealEdgeLength": 160.0,
			"repulsionStrength": 7500.0,
			"springStrength": 0.045,
			"gravity": 0.0,
			"randomSeed": 101,
			"lockedNodeIds": ["a"]
		});
		let out = apply_force_graph_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
		let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
		let nodes = parsed["nodes"].as_array().unwrap();
		let ax = nodes[0]["x"].as_f64().unwrap();
		let ay = nodes[0]["y"].as_f64().unwrap();
		assert!((ax - 0.0).abs() < 1e-9 && (ay - 0.0).abs() < 1e-9);
		let bx = nodes[1]["x"].as_f64().unwrap();
		assert!((bx - 40.0).abs() > 25.0, "expected free node to move, bx={bx}");
	}

	#[test]
	fn redraw_force_graph_top_level_locked_node_ids_pins() {
		let fixture = json!({
			"schema": "elements.board.fixture/v1",
			"camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
			"nodes": [
				{
					"id": "a",
					"x": 0.0,
					"y": 0.0,
					"radius": 35.0,
					"handles": [{ "id": "a:h0", "angle": 0.0, "handleKind": "board.port" }]
				},
				{
					"id": "b",
					"x": 40.0,
					"y": 0.0,
					"radius": 35.0,
					"handles": [{ "id": "b:h0", "angle": 3.14159, "handleKind": "board.port" }]
				}
			],
			"edges": [{ "id": "e1", "source": "a:h0", "target": "b:h0" }]
		});
		let opts = json!({
			"mode": "force-graph",
			"lockedNodeIds": ["a"],
			"randomSeed": 101,
			"redrawHandlesAfter": false,
			"forceGraph": {
				"iterations": 180,
				"idealEdgeLength": 160.0,
				"repulsionStrength": 7500.0,
				"springStrength": 0.045,
				"gravity": 0.0
			}
		});
		let out = apply_redraw_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
		let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
		let nodes = parsed["nodes"].as_array().unwrap();
		assert!((nodes[0]["x"].as_f64().unwrap() - 0.0).abs() < 1e-9);
		assert!((nodes[0]["y"].as_f64().unwrap() - 0.0).abs() < 1e-9);
	}

	#[test]
	fn force_graph_rejects_bad_schema() {
		let err = apply_force_graph_layout_to_fixture_v1_json(r#"{"schema":"x","nodes":[],"edges":[]}"#, "{}").unwrap_err();
		assert!(err.contains("schema"));
	}

	#[test]
	fn force_graph_barnes_hut_many_bodies_yields_finite_coordinates() {
		let mut nodes = Vec::new();
		let mut edges = Vec::new();
		for k in 0..64 {
			let id = format!("n{k}");
			nodes.push(json!({
				"id": id,
				"x": (k % 8) as f64 * 12.0,
				"y": (k / 8) as f64 * 12.0,
				"radius": 8.0,
				"handles": [{ "id": format!("{id}:h0"), "angle": 0.0, "handleKind": "board.port" }]
			}));
			if k > 0 {
				let prev = format!("n{}", k - 1);
				edges.push(json!({
					"id": format!("e{k}"),
					"source": format!("{prev}:h0"),
					"target": format!("{id}:h0")
				}));
			}
		}
		let fixture = json!({
			"schema": "elements.board.fixture/v1",
			"camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
			"nodes": nodes,
			"edges": edges
		});
		let opts = json!({
			"iterations": 180,
			"idealEdgeLength": 90.0,
			"repulsionStrength": 6000.0,
			"springStrength": 0.05,
			"gravity": 0.01,
			"randomSeed": 91,
			"barnesHutTheta": 0.72,
			"pairwiseRepulsionMaxBodies": 12
		});
		let out = apply_force_graph_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
		let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
		for row in parsed["nodes"].as_array().unwrap() {
			let x = row["x"].as_f64().unwrap();
			let y = row["y"].as_f64().unwrap();
			assert!(x.is_finite() && y.is_finite());
		}
		let xs: Vec<f64> = parsed["nodes"].as_array().unwrap().iter().map(|r| r["x"].as_f64().unwrap()).collect();
		let ys: Vec<f64> = parsed["nodes"].as_array().unwrap().iter().map(|r| r["y"].as_f64().unwrap()).collect();
		let x_span = xs.iter().copied().fold(f64::NEG_INFINITY, f64::max) - xs.iter().copied().fold(f64::INFINITY, f64::min);
		let y_span = ys.iter().copied().fold(f64::NEG_INFINITY, f64::max) - ys.iter().copied().fold(f64::INFINITY, f64::min);
		assert!(x_span > 40.0 && y_span > 35.0, "expected BH layout to spread graph, x_span={x_span} y_span={y_span}");
	}

	#[test]
	fn force_graph_bh_layout_is_deterministic_for_fixed_seed() {
		let mut nodes = Vec::new();
		let mut edges = Vec::new();
		for k in 0..36 {
			let id = format!("n{k}");
			nodes.push(json!({
				"id": id,
				"x": (k % 6) as f64 * 9.0,
				"y": (k / 6) as f64 * 9.0,
				"radius": 6.5,
				"handles": [{ "id": format!("{id}:h0"), "angle": 0.0, "handleKind": "board.port" }]
			}));
			if k > 0 {
				let prev = format!("n{}", k - 1);
				edges.push(json!({
					"id": format!("e{k}"),
					"source": format!("{prev}:h0"),
					"target": format!("{id}:h0")
				}));
			}
		}
		let fixture = json!({
			"schema": "elements.board.fixture/v1",
			"camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
			"nodes": nodes,
			"edges": edges
		});
		let opts = json!({
			"iterations": 120,
			"idealEdgeLength": 88.0,
			"repulsionStrength": 5400.0,
			"springStrength": 0.047,
			"gravity": 0.013,
			"randomSeed": 4041,
			"barnesHutTheta": 0.55,
			"pairwiseRepulsionMaxBodies": 8
		});
		let s = fixture.to_string();
		let o = opts.to_string();
		let out_a = apply_force_graph_layout_to_fixture_v1_json(&s, &o).unwrap();
		let out_b = apply_force_graph_layout_to_fixture_v1_json(&s, &o).unwrap();
		assert_eq!(out_a, out_b, "BH path must be bitwise reproducible for identical inputs");
	}

	#[test]
	fn force_graph_pairwise_layout_is_deterministic_for_fixed_seed() {
		let fixture = json!({
			"schema": "elements.board.fixture/v1",
			"camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
			"nodes": [
				{ "id": "a", "x": 0.0, "y": 0.0, "radius": 30.0, "handles": [{ "id": "a:h0", "angle": 0.0, "handleKind": "board.port" }] },
				{ "id": "b", "x": 3.0, "y": 1.0, "radius": 30.0, "handles": [{ "id": "b:h0", "angle": 3.14, "handleKind": "board.port" }] },
				{ "id": "c", "x": -2.0, "y": 4.0, "radius": 28.0, "handles": [{ "id": "c:h0", "angle": 1.0, "handleKind": "board.port" }] }
			],
			"edges": [
				{ "id": "e1", "source": "a:h0", "target": "b:h0" },
				{ "id": "e2", "source": "b:h0", "target": "c:h0" }
			]
		});
		let opts = json!({
			"iterations": 90,
			"idealEdgeLength": 110.0,
			"repulsionStrength": 6200.0,
			"springStrength": 0.042,
			"gravity": 0.011,
			"randomSeed": 909,
			"pairwiseRepulsionMaxBodies": 80
		});
		let s = fixture.to_string();
		let o = opts.to_string();
		let out_a = apply_force_graph_layout_to_fixture_v1_json(&s, &o).unwrap();
		let out_b = apply_force_graph_layout_to_fixture_v1_json(&s, &o).unwrap();
		assert_eq!(out_a, out_b);
	}

	#[test]
	fn force_graph_clamped_barnes_hut_theta_runs_without_error() {
		let fixture = json!({
			"schema": "elements.board.fixture/v1",
			"camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
			"nodes": [
				{ "id": "a", "x": 0.0, "y": 0.0, "radius": 20.0, "handles": [{ "id": "a:h0", "angle": 0.0, "handleKind": "board.port" }] },
				{ "id": "b", "x": 5.0, "y": 0.0, "radius": 20.0, "handles": [{ "id": "b:h0", "angle": 3.14, "handleKind": "board.port" }] },
				{ "id": "c", "x": 2.0, "y": 8.0, "radius": 18.0, "handles": [{ "id": "c:h0", "angle": 0.0, "handleKind": "board.port" }] }
			],
			"edges": [
				{ "id": "e1", "source": "a:h0", "target": "b:h0" },
				{ "id": "e2", "source": "b:h0", "target": "c:h0" }
			]
		});
		let opts = json!({
			"iterations": 40,
			"idealEdgeLength": 100.0,
			"repulsionStrength": 5000.0,
			"springStrength": 0.05,
			"gravity": 0.01,
			"randomSeed": 3,
			"barnesHutTheta": 500.0,
			"pairwiseRepulsionMaxBodies": 2
		});
		let out = apply_force_graph_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
		let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
		for row in parsed["nodes"].as_array().unwrap() {
			assert!(row["x"].as_f64().unwrap().is_finite());
			assert!(row["y"].as_f64().unwrap().is_finite());
		}
	}

	#[test]
	fn redraw_force_graph_wraps_flat_options() {
		let fixture = json!({
			"schema": "elements.board.fixture/v1",
			"camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
			"nodes": [
				{
					"id": "a",
					"x": 0.0,
					"y": 0.0,
					"radius": 40.0,
					"handles": [{ "id": "a:h0", "angle": 0.0, "handleKind": "board.port" }]
				},
				{
					"id": "b",
					"x": 1.0,
					"y": 0.0,
					"radius": 40.0,
					"handles": [{ "id": "b:h0", "angle": 3.14159, "handleKind": "board.port" }]
				}
			],
			"edges": [{ "id": "e1", "source": "a:h0", "target": "b:h0" }]
		});
		let opts = json!({
			"mode": "force-graph",
			"randomSeed": 7,
			"forceGraph": {
				"iterations": 200,
				"idealEdgeLength": 180.0,
				"repulsionStrength": 8000.0,
				"springStrength": 0.04,
				"gravity": 0.0
			}
		});
		let out = apply_redraw_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
		let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
		let nodes = parsed["nodes"].as_array().unwrap();
		let ax = nodes[0]["x"].as_f64().unwrap();
		let bx = nodes[1]["x"].as_f64().unwrap();
		assert!((bx - ax).abs() > 80.0);
	}

	#[test]
	fn edge_handle_snap_sets_circle_handle_angles_on_center_line() {
		let fixture = json!({
			"schema": "elements.board.fixture/v1",
			"camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
			"nodes": [
				{
					"id": "a",
					"x": 0.0,
					"y": 0.0,
					"radius": 40.0,
					"handles": [{ "id": "a:h0", "angle": 1.57, "handleKind": "board.port" }]
				},
				{
					"id": "b",
					"x": 200.0,
					"y": 0.0,
					"radius": 40.0,
					"handles": [{ "id": "b:h0", "angle": 0.0, "handleKind": "board.port" }]
				}
			],
			"edges": [{ "id": "e1", "source": "a:h0", "target": "b:h0" }]
		});
		let out = apply_edge_handle_snap_to_fixture_v1_json(&fixture.to_string()).unwrap();
		let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
		let nodes = parsed["nodes"].as_array().unwrap();
		let ang_a = nodes[0]["handles"][0]["angle"].as_f64().unwrap();
		let ang_b = nodes[1]["handles"][0]["angle"].as_f64().unwrap();
		assert!((ang_a - 0.0).abs() < 1e-6, "expected east on a, got {ang_a}");
		assert!((ang_b - std::f64::consts::PI).abs() < 1e-6, "expected west on b, got {ang_b}");
	}

	#[test]
	fn redraw_force_graph_with_snap_sets_handle_angles() {
		let fixture = json!({
			"schema": "elements.board.fixture/v1",
			"camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
			"nodes": [
				{
					"id": "a",
					"x": 0.0,
					"y": 0.0,
					"radius": 40.0,
					"handles": [{ "id": "a:h0", "angle": 1.57, "handleKind": "board.port" }]
				},
				{
					"id": "b",
					"x": 200.0,
					"y": 0.0,
					"radius": 40.0,
					"handles": [{ "id": "b:h0", "angle": 0.0, "handleKind": "board.port" }]
				}
			],
			"edges": [{ "id": "e1", "source": "a:h0", "target": "b:h0" }]
		});
		let opts = json!({
			"mode": "force-graph",
			"redrawHandlesAfter": true,
			"randomSeed": 7,
			"forceGraph": {
				"iterations": 200,
				"idealEdgeLength": 180.0,
				"repulsionStrength": 8000.0,
				"springStrength": 0.04,
				"gravity": 0.0
			}
		});
		let out = apply_redraw_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
		let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
		let nodes = parsed["nodes"].as_array().unwrap();
		let ang_a = nodes[0]["handles"][0]["angle"].as_f64().unwrap();
		let ang_b = nodes[1]["handles"][0]["angle"].as_f64().unwrap();
		let ax = nodes[0]["x"].as_f64().unwrap();
		let bx = nodes[1]["x"].as_f64().unwrap();
		let ay = nodes[0]["y"].as_f64().unwrap();
		let by = nodes[1]["y"].as_f64().unwrap();
		let exp_a = f64::atan2(by - ay, bx - ax);
		let exp_b = f64::atan2(ay - by, ax - bx);
		let wrap_diff = |a: f64, b: f64| {
			let mut d = (a - b).rem_euclid(std::f64::consts::TAU);
			if d > std::f64::consts::PI {
				d -= std::f64::consts::TAU;
			}
			d.abs()
		};
		assert!(wrap_diff(ang_a, exp_a) < 0.03, "a angle {ang_a} vs exp {exp_a}");
		assert!(wrap_diff(ang_b, exp_b) < 0.03, "b angle {ang_b} vs exp {exp_b}");
	}

	#[test]
	fn force_graph_accepts_logical_nodes_without_xy() {
		let fixture = json!({
			"schema": "elements.board.fixture/v1",
			"camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
			"nodes": [
				{
					"id": "a",
					"radius": 40.0,
					"handles": [{ "id": "a:h0", "angle": 0.0, "handleKind": "board.port" }]
				},
				{
					"id": "b",
					"radius": 40.0,
					"handles": [{ "id": "b:h0", "angle": 3.14159, "handleKind": "board.port" }]
				}
			],
			"edges": [{ "id": "e1", "source": "a:h0", "target": "b:h0" }]
		});
		let opts = json!({
			"mode": "force-graph",
			"centerX": 0.0,
			"centerY": 0.0,
			"randomSeed": 3,
			"forceGraph": { "iterations": 120, "idealEdgeLength": 160.0, "gravity": 0.0 }
		});
		let out = apply_redraw_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
		let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
		for n in parsed["nodes"].as_array().unwrap() {
			assert!(n["x"].as_f64().unwrap().is_finite());
			assert!(n["y"].as_f64().unwrap().is_finite());
		}
	}

	#[test]
	fn hierarchical_tree_stacks_by_depth() {
		let fixture = json!({
			"schema": "elements.board.fixture/v1",
			"camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
			"nodes": [
				{
					"id": "r",
					"root": true,
					"radius": 18.0,
					"handles": [{ "id": "r:h", "angle": 0.0, "handleKind": "board.port" }]
				},
				{
					"id": "c1",
					"radius": 18.0,
					"handles": [{ "id": "c1:h", "angle": 0.0, "handleKind": "board.port" }]
				},
				{
					"id": "c2",
					"radius": 18.0,
					"handles": [{ "id": "c2:h", "angle": 0.0, "handleKind": "board.port" }]
				}
			],
			"edges": [
				{ "id": "e1", "source": "r:h", "target": "c1:h" },
				{ "id": "e2", "source": "r:h", "target": "c2:h" }
			]
		});
		let opts = json!({
			"mode": "hierarchical-tree",
			"centerX": 0.0,
			"centerY": 0.0,
			"hierarchicalTree": { "direction": "downwards", "layerSpacing": 90.0, "siblingGap": 12.0 }
		});
		let out = apply_redraw_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
		let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
		let mut ys: HashMap<String, f64> = HashMap::new();
		for n in parsed["nodes"].as_array().unwrap() {
			let id = n["id"].as_str().unwrap().to_string();
			ys.insert(id, n["y"].as_f64().unwrap());
		}
		let ry = *ys.get("r").unwrap();
		let c1y = *ys.get("c1").unwrap();
		let c2y = *ys.get("c2").unwrap();
		assert!((c1y - ry).abs() > 40.0, "expected child below root");
		assert!((c2y - ry).abs() > 40.0);
		assert!((c1y - c2y).abs() < 1e-3, "siblings share row");
	}

	#[test]
	fn hierarchical_tree_pins_locked_root_coordinates() {
		let fixture = json!({
			"schema": "elements.board.fixture/v1",
			"camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
			"nodes": [
				{
					"id": "r",
					"x": 120.0,
					"y": -33.0,
					"root": true,
					"radius": 18.0,
					"handles": [{ "id": "r:h", "angle": 0.0, "handleKind": "board.port" }]
				},
				{
					"id": "c1",
					"x": 0.0,
					"y": 0.0,
					"radius": 18.0,
					"handles": [{ "id": "c1:h", "angle": 0.0, "handleKind": "board.port" }]
				},
				{
					"id": "c2",
					"x": 5.0,
					"y": 0.0,
					"radius": 18.0,
					"handles": [{ "id": "c2:h", "angle": 0.0, "handleKind": "board.port" }]
				}
			],
			"edges": [
				{ "id": "e1", "source": "r:h", "target": "c1:h" },
				{ "id": "e2", "source": "r:h", "target": "c2:h" }
			]
		});
		let opts = json!({
			"mode": "hierarchical-tree",
			"centerX": 0.0,
			"centerY": 0.0,
			"lockedNodeIds": ["r"],
			"hierarchicalTree": { "direction": "downwards", "layerSpacing": 90.0, "siblingGap": 12.0 }
		});
		let out = apply_redraw_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
		let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
		let mut by_id: HashMap<String, (f64, f64)> = HashMap::new();
		for n in parsed["nodes"].as_array().unwrap() {
			let id = n["id"].as_str().unwrap().to_string();
			by_id.insert(id, (n["x"].as_f64().unwrap(), n["y"].as_f64().unwrap()));
		}
		let (rx, ry) = *by_id.get("r").unwrap();
		assert!((rx - 120.0).abs() < 1e-3 && (ry + 33.0).abs() < 1e-3, "locked root moved: {rx},{ry}");
		let (_c1x, c1y) = *by_id.get("c1").unwrap();
		let (_c2x, c2y) = *by_id.get("c2").unwrap();
		assert!((c1y - c2y).abs() < 1e-3, "siblings share row");
		assert!((c1y - ry).abs() > 40.0, "children laid relative to tree, root stayed pinned");
	}

	#[test]
	fn redraw_hierarchical_tree_nested_locked_node_ids_pins() {
		let fixture = json!({
			"schema": "elements.board.fixture/v1",
			"camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
			"nodes": [
				{
					"id": "r",
					"x": 77.0,
					"y": 12.0,
					"root": true,
					"radius": 18.0,
					"handles": [{ "id": "r:h", "angle": 0.0, "handleKind": "board.port" }]
				},
				{
					"id": "c1",
					"x": 0.0,
					"y": 0.0,
					"radius": 18.0,
					"handles": [{ "id": "c1:h", "angle": 0.0, "handleKind": "board.port" }]
				}
			],
			"edges": [{ "id": "e1", "source": "r:h", "target": "c1:h" }]
		});
		let opts = json!({
			"mode": "hierarchical-tree",
			"centerX": 0.0,
			"centerY": 0.0,
			"hierarchicalTree": {
				"direction": "downwards",
				"layerSpacing": 90.0,
				"siblingGap": 12.0,
				"lockedNodeIds": ["r"]
			}
		});
		let out = apply_redraw_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
		let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
		let mut by_id: HashMap<String, (f64, f64)> = HashMap::new();
		for n in parsed["nodes"].as_array().unwrap() {
			let id = n["id"].as_str().unwrap().to_string();
			by_id.insert(id, (n["x"].as_f64().unwrap(), n["y"].as_f64().unwrap()));
		}
		let (rx, ry) = *by_id.get("r").unwrap();
		assert!((rx - 77.0).abs() < 1e-3 && (ry - 12.0).abs() < 1e-3, "nested locked list ignored: {rx},{ry}");
	}

	#[test]
	fn hierarchical_tree_right_places_children_larger_x_than_root() {
		let fixture = json!({
			"schema": "elements.board.fixture/v1",
			"camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
			"nodes": [
				{
					"id": "r",
					"root": true,
					"radius": 18.0,
					"handles": [{ "id": "r:h", "angle": 0.0, "handleKind": "board.port" }]
				},
				{
					"id": "c1",
					"radius": 18.0,
					"handles": [{ "id": "c1:h", "angle": 0.0, "handleKind": "board.port" }]
				}
			],
			"edges": [{ "id": "e1", "source": "r:h", "target": "c1:h" }]
		});
		let opts = json!({
			"mode": "hierarchical-tree",
			"centerX": 0.0,
			"centerY": 0.0,
			"hierarchicalTree": { "direction": "right", "layerSpacing": 90.0, "siblingGap": 12.0 }
		});
		let out = apply_redraw_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
		let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
		let mut xs: HashMap<String, f64> = HashMap::new();
		for n in parsed["nodes"].as_array().unwrap() {
			let id = n["id"].as_str().unwrap().to_string();
			xs.insert(id, n["x"].as_f64().unwrap());
		}
		let rx = *xs.get("r").unwrap();
		let c1x = *xs.get("c1").unwrap();
		assert!(c1x > rx + 40.0, "expected child to the right of root");
	}

	#[test]
	fn hierarchical_tree_upwards_places_children_smaller_y_than_root() {
		let fixture = json!({
			"schema": "elements.board.fixture/v1",
			"camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
			"nodes": [
				{
					"id": "r",
					"root": true,
					"radius": 18.0,
					"handles": [{ "id": "r:h", "angle": 0.0, "handleKind": "board.port" }]
				},
				{
					"id": "c1",
					"radius": 18.0,
					"handles": [{ "id": "c1:h", "angle": 0.0, "handleKind": "board.port" }]
				}
			],
			"edges": [{ "id": "e1", "source": "r:h", "target": "c1:h" }]
		});
		let opts = json!({
			"mode": "hierarchical-tree",
			"centerX": 0.0,
			"centerY": 0.0,
			"hierarchicalTree": { "direction": "upwards", "layerSpacing": 90.0, "siblingGap": 12.0 }
		});
		let out = apply_redraw_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
		let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
		let mut ys: HashMap<String, f64> = HashMap::new();
		for n in parsed["nodes"].as_array().unwrap() {
			let id = n["id"].as_str().unwrap().to_string();
			ys.insert(id, n["y"].as_f64().unwrap());
		}
		let ry = *ys.get("r").unwrap();
		let c1y = *ys.get("c1").unwrap();
		assert!(c1y < ry - 40.0, "expected child above root (smaller y)");
	}

	#[test]
	fn hierarchical_tree_rejects_unknown_direction() {
		let fixture = json!({
			"schema": "elements.board.fixture/v1",
			"camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
			"nodes": [
				{
					"id": "r",
					"root": true,
					"radius": 18.0,
					"handles": [{ "id": "r:h", "angle": 0.0, "handleKind": "board.port" }]
				}
			],
			"edges": []
		});
		let opts = json!({
			"mode": "hierarchical-tree",
			"hierarchicalTree": { "direction": "sideways" }
		});
		let err = apply_redraw_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap_err();
		assert!(err.contains("unknown hierarchical tree direction"));
	}

	#[test]
	fn redraw_rejects_unknown_mode() {
		let fixture = json!({
			"schema": "elements.board.fixture/v1",
			"camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
			"nodes": [],
			"edges": []
		});
		let err = apply_redraw_layout_to_fixture_v1_json(&fixture.to_string(), r#"{"mode":"nope"}"#).unwrap_err();
		assert!(err.contains("unknown redraw mode"));
	}

	#[test]
	fn svg_icon_vello09_append_smoke() {
		let mut scene = crate::vello::Scene::new();
		let svg = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10"><rect width="10" height="10" fill="#ffffff"/><path d="M0 0 L10 10" stroke="#000000" stroke-width="1"/></svg>"##;
		super::svg_icon_vello09::append_svg_str(&mut scene, svg).expect("parse svg");
		let fg = crate::vello::peniko::Color::from_rgba8(200, 10, 10, 255);
		let bg = crate::vello::peniko::Color::from_rgba8(10, 200, 10, 255);
		let mut scene2 = crate::vello::Scene::new();
		super::svg_icon_vello09::append_svg_str_themed(&mut scene2, svg, fg, bg).expect("parse themed");
	}

	#[test]
	fn board_icon_codec_resolves_typst_math_to_svg_plain() {
		let r = super::board_icon_codec::board_resolve_icon_kind("typst:$x^2$");
		match r {
			super::board_icon_codec::BoardResolvedIcon::SvgPlain(s) => {
				assert!(s.contains("<svg"), "{}", &s[..s.len().min(240)]);
			}
			other => panic!("unexpected resolution: {other:?}"),
		}
	}

	#[test]
	fn board_icon_codec_resolves_emoji_prefix_without_tofu() {
		let r = super::board_icon_codec::board_resolve_icon_kind("emoji:☺");
		match r {
			super::board_icon_codec::BoardResolvedIcon::SvgPlain(s) => {
				assert!(s.contains("<svg"), "{}", &s[..s.len().min(240)]);
				assert!(
					!s.contains('\u{fffd}'),
					"expected no U+FFFD replacement in emoji SVG, got {}",
					&s[..s.len().min(400)]
				);
			}
			other => panic!("unexpected resolution: {other:?}"),
		}
	}

	#[test]
	fn svg_icon_content_bounds_follows_nested_group_translate() {
		let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="200" height="200" viewBox="0 0 200 200"><g transform="translate(72 88)"><rect width="12" height="12" fill="rgb(8,8,8)"/></g></svg>"#;
		let tree = crate::usvg::Tree::from_str(svg, &crate::usvg::Options::default()).expect("parse");
		let (x, y, w, h) = super::svg_icon_vello09::svg_icon_content_bounds(&tree);
		assert!(x >= 70.0 && x <= 74.0, "expected translated art near x≈72, got {x}");
		assert!(y >= 86.0 && y <= 90.0, "expected translated art near y≈88, got {y}");
		assert!(w > 10.0 && w < 14.0 && h > 10.0 && h < 14.0, "expected ~12×12 bbox, got {w}×{h}");
	}

	#[test]
	fn svg_icon_content_bounds_includes_visible_image_abs_box() {
		let svg = r##"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100"><image href="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg==" x="30" y="40" width="50" height="50"/></svg>"##;
		let tree = crate::usvg::Tree::from_str(svg, super::svg_icon_vello09::usvg_options_board_icons()).expect("parse");
		let (x, y, w, h) = super::svg_icon_vello09::svg_icon_content_bounds(&tree);
		assert!((x - 30.0).abs() < 2.0, "expected image bbox near x=30, got {x}");
		assert!((y - 40.0).abs() < 2.0, "expected image bbox near y=40, got {y}");
		assert!((w - 50.0).abs() < 2.0 && (h - 50.0).abs() < 2.0, "expected ~50×50 bbox, got {w}×{h}");
	}
}

// #endregion 🔖Tests
