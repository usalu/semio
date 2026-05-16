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
		#[serde(default)]
		pub root: Option<bool>,
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
		pub handle_kind: Option<String>,
		/// CSS `#rgb` / `#rrggbb` / `#rrggbbaa` overriding catalog color for this handle.
		#[serde(default)]
		pub color: Option<String>,
		#[serde(default)]
		pub user_data: Option<serde_json::Value>,
		#[serde(default)]
		pub visible: Option<bool>,
	}

	#[derive(Clone, Debug, Deserialize, Serialize)]
	#[serde(rename_all = "camelCase")]
	pub struct EdgeDescJson {
		pub id: String,
		pub source: String,
		pub target: String,
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

// #region 🕸️ForceGraphLayout
mod force_graph {
	use nalgebra::Vector2;
	use serde::{Deserialize, Serialize};
	use serde_json::Value;
	use std::collections::{HashMap, HashSet};

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
		let mut handle_to_node: HashMap<String, String> = HashMap::new();
		for node in nodes.iter() {
			let Some(obj) = node.as_object() else {
				continue;
			};
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
				if let Some(hid) = ho.get("id").and_then(|v| v.as_str()) {
					handle_to_node.insert(hid.to_string(), nid.to_string());
				}
			}
		}
		let mut id_to_index: HashMap<String, usize> = HashMap::new();
		let mut optional_xy: Vec<Option<(f64, f64)>> = Vec::new();
		let mut positions: Vec<Vector2<f64>> = Vec::new();
		let mut velocities: Vec<Vector2<f64>> = Vec::new();
		let mut radii: Vec<f64> = Vec::new();
		for (idx, node) in nodes.iter().enumerate() {
			let Some(obj) = node.as_object() else {
				return Err("node must be object".into());
			};
			let Some(nid) = obj.get("id").and_then(|v| v.as_str()) else {
				return Err("node id missing".into());
			};
			let x_opt = obj.get("x").and_then(|v| v.as_f64());
			let y_opt = obj.get("y").and_then(|v| v.as_f64());
			let xy = match (x_opt, y_opt) {
				(Some(x), Some(y)) if x.is_finite() && y.is_finite() => Some((x, y)),
				_ => None,
			};
			id_to_index.insert(nid.to_string(), idx);
			optional_xy.push(xy);
			positions.push(Vector2::zeros());
			velocities.push(Vector2::zeros());
			radii.push(node_repulsion_radius(node));
		}
		let n = positions.len();
		let mut sum = Vector2::zeros();
		let mut finite_ct: u32 = 0;
		for xy in &optional_xy {
			if let Some((x, y)) = xy {
				sum += Vector2::new(*x, *y);
				finite_ct += 1;
			}
		}
		let anchor = if finite_ct > 0 {
			sum / (finite_ct as f64)
		} else {
			Vector2::new(opts.center_x.unwrap_or(0.0), opts.center_y.unwrap_or(0.0))
		};
		let mut seed_rng = opts.random_seed;
		for i in 0..n {
			positions[i] = if let Some((x, y)) = optional_xy[i] {
				Vector2::new(x, y)
			} else {
				let t = i as f64;
				let ang = t * 2.39996322972865332;
				let r = 10.0 + t.sqrt() * 22.0;
				let jx = (rand_unit_interval(&mut seed_rng) - 0.5) * 6.0;
				let jy = (rand_unit_interval(&mut seed_rng) - 0.5) * 6.0;
				anchor + Vector2::new(r * ang.cos() + jx, r * ang.sin() + jy)
			};
		}
		let mut edge_pairs: Vec<(usize, usize)> = Vec::new();
		let mut seen: HashSet<(usize, usize)> = HashSet::new();
		for e in &edges {
			let Some(eo) = e.as_object() else {
				continue;
			};
			let Some(src_h) = eo.get("source").and_then(|v| v.as_str()) else {
				continue;
			};
			let Some(tgt_h) = eo.get("target").and_then(|v| v.as_str()) else {
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
		for p in &mut positions {
			if (p.x - gx).abs() < 1e-6 && (p.y - gy).abs() < 1e-6 {
				let jx = (rand_unit_interval(&mut rng) - 0.5) * 12.0;
				let jy = (rand_unit_interval(&mut rng) - 0.5) * 12.0;
				*p += Vector2::new(jx, jy);
			}
		}
		let iters = opts.iterations.max(1);
		for iter in 0..iters {
			let cool = (1.0 - iter as f64 / iters as f64).max(0.08);
			let mut forces = vec![Vector2::zeros(); n];
			for i in 0..n {
				for j in (i + 1)..n {
					let delta = positions[j] - positions[i];
					let dist = delta.norm().max(1e-4);
					let rep = opts.repulsion_strength * cool * (radii[i] * radii[j]).max(1.0) / (dist * dist);
					let dir = delta / dist;
					let f = dir * rep;
					forces[i] -= f;
					forces[j] += f;
				}
			}
			for &(i, j) in &edge_pairs {
				let delta = positions[j] - positions[i];
				let dist = delta.norm().max(1e-4);
				let dir = delta / dist;
				let displacement = dist - k;
				let f = dir * (opts.spring_strength * cool * displacement);
				forces[i] += f;
				forces[j] -= f;
			}
			if opts.gravity > 0.0 {
				let g = opts.gravity * cool;
				for i in 0..n {
					let to_c = Vector2::new(gx - positions[i].x, gy - positions[i].y);
					forces[i] += to_c * g;
				}
			}
			let dt = opts.time_step * cool.sqrt();
			for i in 0..n {
				let mut v = (velocities[i] + forces[i] * dt) * opts.velocity_damping;
				let spd = v.norm();
				if spd > opts.max_speed {
					v *= opts.max_speed / spd;
				}
				velocities[i] = v;
				positions[i] += v * dt;
			}
		}
		for (idx, node) in nodes.iter_mut().enumerate() {
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
				if let Some(hid) = ho.get("id").and_then(|v| v.as_str()) {
					handle_to_node.insert(hid.to_string(), nid.to_string());
				}
			}
		}
		let mut directed: Vec<(String, String)> = Vec::new();
		let mut seen_dir: HashSet<(String, String)> = HashSet::new();
		for e in &edges_json {
			let Some(eo) = e.as_object() else {
				continue;
			};
			let Some(src_h) = eo.get("source").and_then(|v| v.as_str()) else {
				continue;
			};
			let Some(tgt_h) = eo.get("target").and_then(|v| v.as_str()) else {
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
		for (id, (x, y)) in pos {
			let fx = x + dx;
			let fy = y + dy;
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
	use vello::kurbo::Point;

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
			shapes.push(parse_node_shape_snap(no));
			let Some(hs) = no.get("handles").and_then(|v| v.as_array()) else {
				continue;
			};
			for (hi, h) in hs.iter().enumerate() {
				let Some(ho) = h.as_object() else {
					continue;
				};
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
			let Some(src_h) = eo.get("source").and_then(|v| v.as_str()) else {
				continue;
			};
			let Some(tgt_h) = eo.get("target").and_then(|v| v.as_str()) else {
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

	const LOD_MINIMAP_MAX_ZOOM: f64 = 0.15;
	const LOD_DETAIL_MIN_ZOOM: f64 = 0.5;
	const GRID_MAJOR_QUANTUM_WORLD: f64 = 10.0;
	const GRID_MINOR_WORLD: f64 = 1.0;
	const GRID_SCREEN_STEP_MIN_MAJOR_PX: f64 = 18.0;
	const GRID_SCREEN_STEP_MIN_MINOR_PX: f64 = 6.0;
	const WORLD_CLIP_TILE_WORLD: f64 = 256.0;
	const MAX_WORLD_CLIP_TILES: u32 = 768;
	const EDGE_HIT_TOLERANCE_PX: f64 = 8.0;
	const HANDLE_HIT_TOLERANCE_PX: f64 = 10.0;
	const LINK_DRAG_MIN_DISTANCE_PX: f64 = 5.0;
	const LINK_HANDLE_SNAP_EXTRA_PX: f64 = 22.0;
	const SELECTION_LASSO_MIN_POINT_DISTANCE_PX: f64 = 3.0;
	const SELECTION_CLICK_MAX_DISTANCE_PX: f64 = 4.0;
	pub const BOARD_CAMERA_ZOOM_MIN: f64 = 0.05;
	pub const BOARD_CAMERA_ZOOM_MAX: f64 = 32.0;

	#[derive(Clone, Copy, Debug, PartialEq, Eq)]
	enum BoardDrawLod {
		Minimap,
		Overview,
		Detail,
	}

	fn draw_lod(zoom: f64) -> BoardDrawLod {
		if zoom < LOD_MINIMAP_MAX_ZOOM {
			BoardDrawLod::Minimap
		} else if zoom < LOD_DETAIL_MIN_ZOOM {
			BoardDrawLod::Overview
		} else {
			BoardDrawLod::Detail
		}
	}

	fn major_world_step_for_grid(zoom: f64) -> f64 {
		let raw = (GRID_SCREEN_STEP_MIN_MAJOR_PX / zoom.max(1e-9)).max(GRID_MAJOR_QUANTUM_WORLD);
		(raw / GRID_MAJOR_QUANTUM_WORLD).ceil() * GRID_MAJOR_QUANTUM_WORLD
	}

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
		pub root: bool,
		pub style: Option<String>,
		pub text: Option<String>,
	}

	#[derive(Clone, Debug)]
	pub struct HandleKindDef {
		pub name: String,
		pub color: Color,
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
		pub handle_kind: String,
		/// Parsed from descriptor `color` when set (overrides catalog fill).
		pub color_fill: Option<Color>,
	}

	#[derive(Clone, Debug)]
	pub struct EdgeData {
		pub id: String,
		pub source: String,
		pub target: String,
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
		/// Catalog keyed by `handle_kind` id (`{ id, name, color }` from `set_handle_kinds_from_json`).
		pub handle_kinds: BTreeMap<String, HandleKindDef>,
		/// Ordered pairs `(source_handle_kind, target_handle_kind)` allowed for handle-link gestures; empty = unrestricted.
		pub handle_link_compat_pairs: Vec<(String, String)>,
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
		/// Screen-space open polyline (typically two points) while dragging a new handle link.
		pub link_screen_preview: Option<Vec<Point>>,
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
				handle_kinds: BTreeMap::new(),
				handle_link_compat_pairs: Vec::new(),
				selection: BTreeSet::new(),
				selection_options: SelectionOptions {
					method: "rectangle".into(),
					mode: "invertive".into(),
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

		pub fn set_selection_options(
			&mut self,
			method: &str,
			mode: &str,
			select_nodes: bool,
			select_edges: bool,
			select_handles: bool,
		) {
			self.selection_options.method = method.into();
			self.selection_options.mode = mode.into();
			self.selection_options.select_nodes = select_nodes;
			self.selection_options.select_edges = select_edges;
			self.selection_options.select_handles = select_handles;
		}

		/// @emoji 🔗 JSON `[{ "source": "…", "target": "…" }, …]` of allowed directed handle-kind pairs for link gestures; empty clears restrictions.
		pub fn set_handle_link_compat_from_json(&mut self, json: &str) -> Result<(), String> {
			let v: serde_json::Value = serde_json::from_str(json).map_err(|e| e.to_string())?;
			let arr = v
				.as_array()
				.ok_or_else(|| "expected JSON array of {source,target} objects".to_string())?;
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
				next.push((source, target));
			}
			self.handle_link_compat_pairs = next;
			Ok(())
		}

		/// @emoji 🎨 JSON `[{ "id": "…", "name": "…", "color": "#rrggbb" }, …]` catalog for handle-kind fill colors (`name` reserved for UI).
		pub fn set_handle_kinds_from_json(&mut self, json: &str) -> Result<(), String> {
			let v: serde_json::Value = serde_json::from_str(json).map_err(|e| e.to_string())?;
			let arr = v
				.as_array()
				.ok_or_else(|| "expected JSON array of {id,name,color} objects".to_string())?;
			let mut next = BTreeMap::new();
			for row in arr {
				let o = row.as_object().ok_or("handle kind row must be object")?;
				let id = o
					.get("id")
					.and_then(|x| x.as_str())
					.map(str::trim)
					.filter(|s| !s.is_empty())
					.ok_or("handle kind id missing")?;
				let name = o
					.get("name")
					.and_then(|x| x.as_str())
					.unwrap_or("")
					.to_string();
				let color_s = o
					.get("color")
					.and_then(|x| x.as_str())
					.map(str::trim)
					.filter(|s| !s.is_empty())
					.ok_or("handle kind color missing")?;
				let color = Self::parse_css_hex_color(color_s).ok_or_else(|| format!("invalid handle kind color {color_s:?}"))?;
				next.insert(id.to_string(), HandleKindDef { name, color });
			}
			self.handle_kinds = next;
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

		fn resolve_handle_fill_color(&self, h: &HandleData, theme: &VelloThemePalette) -> Color {
			if let Some(c) = h.color_fill {
				return c;
			}
			if let Some(def) = self.handle_kinds.get(&h.handle_kind) {
				return def.color;
			}
			handle_fill(theme, h.selected)
		}

		fn handles_link_compatible_for_drag(&self, source: &HandleData, target: &HandleData) -> bool {
			if self.handle_link_compat_pairs.is_empty() {
				return true;
			}
			let fk = source.handle_kind.as_str();
			let tk = target.handle_kind.as_str();
			self
				.handle_link_compat_pairs
				.iter()
				.any(|(a, b)| a.as_str() == fk && b.as_str() == tk)
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

		/// @emoji 🧿 True while a handle link gesture is active so JS can defer `syncDescriptorJson` the same way as area select.
		pub fn defers_descriptor_sync_from_js(&self) -> bool {
			matches!(
				self.interaction,
				Interaction::LinkAtSourceHandle { .. } | Interaction::LinkDragSnap { .. }
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

		fn handle_world_pos(&self, h: &HandleData) -> Option<Point> {
			let n = self.nodes.get(&h.node_id)?;
			Some(match n.shape {
				NodeShape::Circle => handle_position_on_circle(Point::new(n.x, n.y), n.radius, h.angle),
				NodeShape::Rectangle => handle_position_on_rectangle(Point::new(n.x, n.y), n.width, n.height, h.angle),
			})
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

		pub fn resolve_hit_world(&self, point: Point) -> Option<String> {
			let zoom = self.camera.zoom;
			let o = &self.selection_options;
			if o.select_handles {
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
			}
			if o.select_nodes {
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
			if o.select_edges {
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

		pub fn sync_descriptor(&mut self, desc: &SceneDescriptorJson) -> Result<(), String> {
			self.link_screen_preview = None;
			if matches!(
				self.interaction,
				Interaction::LinkAtSourceHandle { .. } | Interaction::LinkDragSnap { .. }
			) {
				self.interaction = Interaction::None;
			}
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
						root: n.root.unwrap_or(false),
						style: n.style.clone(),
						text: n.text.clone(),
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
						Self::parse_css_hex_color(s)
							.ok_or_else(|| format!("invalid color on handle {}: {s:?}", h.id))?,
					),
				};
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
						handle_kind: kind,
						color_fill,
					},
				);
			}
			for e in &desc.edges {
				let existed = self.edges.contains_key(&e.id);
				self.edges.insert(
					e.id.clone(),
					EdgeData {
						id: e.id.clone(),
						source: e.source.clone(),
						target: e.target.clone(),
						selected: e.selected.unwrap_or(false),
						visible: e.visible.unwrap_or(true),
						style: e.style.clone(),
					},
				);
				if !existed {
					self.push_event(
						"edgeCreate",
						json!({ "id": e.id, "source": e.source, "target": e.target }),
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
			Ok(())
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
					handles.push(HandleDescJson {
						id: hid.into(),
						node_id: id.into(),
						angle,
						radius: None,
						selected: None,
						style: None,
						handle_kind: Some(handle_kind),
						color: handle_color,
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
					let root = obj.get("root").and_then(|v| v.as_bool());
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
						root,
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
					let root = obj.get("root").and_then(|v| v.as_bool());
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
						root,
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
				let Some(source) = e.get("source").and_then(|v| v.as_str()) else {
					return false;
				};
				let Some(target) = e.get("target").and_then(|v| v.as_str()) else {
					return false;
				};
				desc.edges.push(EdgeDescJson {
					id: id.into(),
					source: source.into(),
					target: target.into(),
					selected: None,
					style: None,
					user_data: None,
					visible: None,
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
			scene.stroke(&stroke, Affine::IDENTITY, color, None, &p);
		}

		fn append_nodes_handles_edges(&self, scene: &mut Scene, tile_filter: Option<&WorldBox>, lod: BoardDrawLod) {
			let pad = self.drawable_cull_pad_world();
			let draw_node_stroke = lod != BoardDrawLod::Minimap;
			let draw_handles = lod == BoardDrawLod::Detail;
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
						if draw_node_stroke {
							scene.stroke(&Stroke::new(sw), Affine::IDENTITY, stroke_c, None, &circle);
						}
					}
					NodeShape::Rectangle => {
						let hw = n.width / 2.0;
						let hh = n.height / 2.0;
						let p0 = self.world_to_screen(Point::new(n.x - hw, n.y - hh));
						let p1 = self.world_to_screen(Point::new(n.x + hw, n.y + hh));
						let r = Rect::from_points(p0, p1);
						scene.fill(Fill::NonZero, Affine::IDENTITY, fill, None, &r);
						if draw_node_stroke {
							scene.stroke(&Stroke::new(sw), Affine::IDENTITY, stroke_c, None, &r);
						}
					}
				}
			}
			for h in self.handles.values() {
				if !h.visible || !draw_handles {
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
				let fill = self.resolve_handle_fill_color(h, &self.vello_theme);
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
			let lod = draw_lod(self.camera.zoom);
			let grid_color = self.vello_theme.grid_minor_stroke;
			if lod != BoardDrawLod::Minimap {
				let major_w = major_world_step_for_grid(self.camera.zoom);
				self.stroke_world_step_grid(&mut inner, grid_color, 1.0, major_w, GRID_SCREEN_STEP_MIN_MAJOR_PX);
				if lod == BoardDrawLod::Detail {
					let minor_step = GRID_MINOR_WORLD * self.camera.zoom;
					if minor_step >= GRID_SCREEN_STEP_MIN_MINOR_PX {
						self.stroke_world_step_grid(
							&mut inner,
							grid_color,
							0.55,
							GRID_MINOR_WORLD,
							GRID_SCREEN_STEP_MIN_MINOR_PX,
						);
					}
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
			if let Some(ref pts) = self.link_screen_preview {
				if pts.len() >= 2 {
					let mut path = vello::kurbo::BezPath::new();
					path.move_to(pts[0]);
					for p in pts.iter().skip(1) {
						path.line_to(*p);
					}
					inner.stroke(
						&Stroke::new(2.25),
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
			self.sync_selection_flags_to_objects();
			self.push_select_event();
		}

		fn link_snap_tolerance_world(&self, h: &HandleData) -> f64 {
			let z = self.camera.zoom.max(1e-9);
			(HANDLE_HIT_TOLERANCE_PX + LINK_HANDLE_SNAP_EXTRA_PX) / z + h.radius
		}

		fn nearest_link_snap_handle_world(&self, source_handle_id: &str, world: Point) -> Option<String> {
			let source_handle = self.handles.get(source_handle_id)?;
			let source_node_id = source_handle.node_id.as_str();
			let mut best: Option<(f64, String)> = None;
			for (id, h) in &self.handles {
				if id == source_handle_id || !h.visible {
					continue;
				}
				if h.node_id == source_node_id {
					continue;
				}
				if !self.handles_link_compatible_for_drag(source_handle, h) {
					continue;
				}
				let pw = self.handle_world_pos(h)?;
				let tol = self.link_snap_tolerance_world(h);
				let d = distance_between(world, pw);
				if d <= tol && best.as_ref().map(|(bd, _)| d < *bd).unwrap_or(true) {
					best = Some((d, id.clone()));
				}
			}
			best.map(|(_, id)| id)
		}

		fn sync_link_drag_preview(
			&mut self,
			source_handle_id: &str,
			end_screen: Point,
			world: Point,
			target_handle_id: Option<&str>,
		) {
			let Some(start_w) = self.handles.get(source_handle_id).and_then(|h| self.handle_world_pos(h)) else {
				self.link_screen_preview = None;
				return;
			};
			let start_s = self.world_to_screen(start_w);
			let end_s = if let Some(tid) = target_handle_id {
				self.handles
					.get(tid)
					.and_then(|h| self.handle_world_pos(h))
					.map(|w| self.world_to_screen(w))
					.unwrap_or(end_screen)
			} else {
				end_screen
			};
			self.link_screen_preview = Some(vec![start_s, end_s]);
			if let Some(tid) = target_handle_id {
				self.set_hovered_id(Some(tid.to_string()));
			} else {
				self.update_hover_from_world(world);
			}
		}

		fn try_commit_link_edge(&mut self, source_handle_id: &str, target_handle_id: &str) -> bool {
			if source_handle_id == target_handle_id {
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
			self.edges.insert(
				id.clone(),
				EdgeData {
					id: id.clone(),
					source: source_handle_id.to_string(),
					target: target_handle_id.to_string(),
					selected: false,
					visible: true,
					style: None,
				},
			);
			self.push_event(
				"edgeCreate",
				json!({ "id": id, "source": source_handle_id, "target": target_handle_id }),
			);
			true
		}

		pub fn pointer_down_screen(&mut self, sx: f64, sy: f64, button: u8, shift: bool) {
			self.set_selection_screen_preview(None);
			self.link_screen_preview = None;
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
			if let Some(ref hid) = hit {
				if button == 0 && self.handles.contains_key(hid) {
					let next = Self::merge_pick_into_selection(&self.selection, hid, self.selection_options.mode.as_str());
					let ids: Vec<_> = next.iter().cloned().collect();
					self.set_selection_ids(&ids);
					self.interaction = Interaction::LinkAtSourceHandle {
						source_id: hid.clone(),
						start_screen: screen,
					};
					self.set_hovered_id(Some(hid.clone()));
					return;
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
				Interaction::LinkAtSourceHandle { source_id, start_screen } => {
					if distance_between(screen, start_screen) >= LINK_DRAG_MIN_DISTANCE_PX {
						let optional_target_handle_id = self.nearest_link_snap_handle_world(&source_id, world);
						self.sync_link_drag_preview(&source_id, screen, world, optional_target_handle_id.as_deref());
						self.interaction = Interaction::LinkDragSnap {
							source_id: source_id.clone(),
							target_id: optional_target_handle_id,
						};
					} else {
						self.link_screen_preview = None;
						self.interaction = Interaction::LinkAtSourceHandle { source_id, start_screen };
						self.update_hover_from_world(world);
					}
				}
				Interaction::LinkDragSnap { source_id, .. } => {
					let optional_target_handle_id = self.nearest_link_snap_handle_world(&source_id, world);
					self.sync_link_drag_preview(&source_id, screen, world, optional_target_handle_id.as_deref());
					self.interaction = Interaction::LinkDragSnap {
						source_id: source_id.clone(),
						target_id: optional_target_handle_id,
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
			let grabbed = std::mem::take(&mut self.interaction);
			match grabbed {
				Interaction::LinkDragSnap { source_id, target_id } => {
					self.link_screen_preview = None;
					if let Some(target_handle_id) = target_id {
						self.try_commit_link_edge(&source_id, &target_handle_id);
					}
					self.interaction = Interaction::None;
					self.update_hover_from_world(world);
				}
				Interaction::LinkAtSourceHandle { .. } => {
					self.link_screen_preview = None;
					self.interaction = Interaction::None;
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
					if click_only {
						self.set_selection_ids(&[]);
					} else {
						let next = self.resolve_area_selection_with_initial(&initial_ids, start, &points);
						let ids: Vec<_> = next.iter().cloned().collect();
						self.set_selection_ids(&ids);
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
					if h.visible && self.selection_contains_handle(h, box_, enclosing, polygon) {
						hits.insert(h.id.clone());
					}
				}
			}
			if o.select_edges {
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
	render_ctx: Option<vello::util::RenderContext>,
	renderer: Option<vello::Renderer>,
	surface: Option<vello::util::RenderSurface<'static>>,
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
		let desc: SceneDescriptorJson =
			serde_json::from_str(json).map_err(|e| JsValue::from_str(&e.to_string()))?;
		self.state.borrow_mut().host.sync_descriptor(&desc).map_err(|e| JsValue::from_str(&e))?;
		Ok(())
	}

	#[wasm_bindgen(js_name = setHandleKindsJson)]
	pub fn set_handle_kinds_json(&mut self, json: &str) -> Result<(), JsValue> {
		self.state
			.borrow_mut()
			.host
			.set_handle_kinds_from_json(json)
			.map_err(|e| JsValue::from_str(&e))
	}

	#[wasm_bindgen(js_name = setVelloThemeJson)]
	pub fn set_vello_theme_json(&mut self, json: &str) {
		let _ = self.state.borrow_mut().host.set_vello_theme_from_json(json);
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
	pub fn pointer_down_screen_wasm(&mut self, sx: f64, sy: f64, button: u8, shift: bool) {
		self.state.borrow_mut().host.pointer_down_screen(sx, sy, button, shift);
	}

	#[wasm_bindgen(js_name = pointerMoveScreen)]
	pub fn pointer_move_screen_wasm(&mut self, sx: f64, sy: f64) {
		self.state.borrow_mut().host.pointer_move_screen(sx, sy);
	}

	#[wasm_bindgen(js_name = pointerUpScreen)]
	pub fn pointer_up_screen_wasm(&mut self, sx: f64, sy: f64) {
		self.state.borrow_mut().host.pointer_up_screen(sx, sy);
	}

	#[wasm_bindgen(js_name = pointerLeaveScreen)]
	pub fn pointer_leave_screen_wasm(&mut self) {
		self.state.borrow_mut().host.pointer_leave_screen();
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

	#[wasm_bindgen(js_name = setSelectionIdsJson)]
	pub fn set_selection_ids_json(&mut self, json: &str) -> Result<(), JsValue> {
		let ids: Vec<String> = serde_json::from_str(json).map_err(|err| JsValue::from_str(&err.to_string()))?;
		self.state.borrow_mut().host.set_selection_ids(&ids);
		Ok(())
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
				root: None,
				shape: Some("circle".into()),
				radius: Some(40.0),
				width: None,
				height: None,
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
					user_data: None,
					visible: None,
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
					user_data: None,
					visible: None,
				},
			],
			edges: vec![EdgeDescJson {
				id: "e1".into(),
				source: "a:h0".into(),
				target: "b:h0".into(),
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
			root: None,
			shape: Some("circle".into()),
			radius: Some(40.0),
			width: None,
			height: None,
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
			user_data: None,
			visible: None,
			root: None,
			shape: Some("circle".into()),
			radius: Some(40.0),
			width: None,
			height: None,
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
			user_data: None,
			visible: None,
			root: None,
			shape: Some("circle".into()),
			radius: Some(40.0),
			width: None,
			height: None,
		});
		h.sync_descriptor(&desc).unwrap();
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
			root: None,
			shape: Some("circle".into()),
			radius: Some(40.0),
			width: None,
			height: None,
		});
		h.sync_descriptor(&desc).unwrap();
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
			user_data: None,
			visible: None,
			root: None,
			shape: Some("circle".into()),
			radius: Some(40.0),
			width: None,
			height: None,
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
			user_data: None,
			visible: None,
			root: None,
			shape: Some("circle".into()),
			radius: Some(40.0),
			width: None,
			height: None,
		});
		h.sync_descriptor(&desc).unwrap();
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
			root: None,
			shape: Some("circle".into()),
			radius: Some(40.0),
			width: None,
			height: None,
		});
		h.sync_descriptor(&desc).unwrap();
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
			user_data: None,
			visible: None,
			root: None,
			shape: Some("circle".into()),
			radius: Some(40.0),
			width: None,
			height: None,
		});
		h.sync_descriptor(&desc).unwrap();
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
		assert!(got.contains(&"a:h0".to_string()));
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
					user_data: None,
					visible: None,
					root: None,
					shape: Some("circle".into()),
					radius: Some(40.0),
					width: None,
					height: None,
				},
				NodeDescJson {
					id: "b".into(),
					x: 280.0,
					y: 0.0,
					draggable: Some(true),
					selected: None,
					style: None,
					text: None,
					user_data: None,
					visible: None,
					root: None,
					shape: Some("circle".into()),
					radius: Some(40.0),
					width: None,
					height: None,
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
					user_data: None,
					visible: None,
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
					user_data: None,
					visible: None,
				},
			],
			edges: vec![],
		}
	}

	#[test]
	fn board_host_link_drag_snap_emits_edge_create() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
		let _ = h.drain_events_json();
		let hp_a = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
		let hp_b = handle_position_on_circle(Point::new(280.0, 0.0), 40.0, std::f64::consts::PI);
		let s0 = h.world_to_screen(hp_a);
		h.pointer_down_screen(s0.x, s0.y, 0, false);
		let s_mid = h.world_to_screen(Point::new(140.0, 0.0));
		h.pointer_move_screen(s_mid.x + 20.0, s_mid.y);
		let s1 = h.world_to_screen(hp_b);
		h.pointer_move_screen(s1.x, s1.y);
		h.pointer_up_screen(s1.x, s1.y);
		let ev = h.drain_events_json();
		assert!(ev.contains("edgeCreate"));
		assert!(ev.contains("a:h0"));
		assert!(ev.contains("b:h0"));
		let created: Vec<_> = h.edges.keys().filter(|k| k.starts_with("edge-link-")).cloned().collect();
		assert_eq!(created.len(), 1);
	}

	#[test]
	fn board_host_link_rejects_incompatible_handle_kind_pairs() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		h.set_handle_link_compat_from_json(r#"[{"source":"child","target":"parent"}]"#).unwrap();
		let desc = link_test_scene_no_edge();
		h.sync_descriptor(&desc).unwrap();
		let _ = h.drain_events_json();
		let hp_a = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
		let hp_b = handle_position_on_circle(Point::new(280.0, 0.0), 40.0, std::f64::consts::PI);
		let s0 = h.world_to_screen(hp_a);
		h.pointer_down_screen(s0.x, s0.y, 0, false);
		let s_mid = h.world_to_screen(Point::new(140.0, 0.0));
		h.pointer_move_screen(s_mid.x + 20.0, s_mid.y);
		let s1 = h.world_to_screen(hp_b);
		h.pointer_move_screen(s1.x, s1.y);
		h.pointer_up_screen(s1.x, s1.y);
		let ev = h.drain_events_json();
		assert!(!ev.contains("edgeCreate"));
	}

	#[test]
	fn board_host_link_accepts_matching_handle_kind_pair() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
		let desc = link_test_scene_no_edge();
		h.sync_descriptor(&desc).unwrap();
		let _ = h.drain_events_json();
		let hp_a = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
		let hp_b = handle_position_on_circle(Point::new(280.0, 0.0), 40.0, std::f64::consts::PI);
		let s0 = h.world_to_screen(hp_a);
		h.pointer_down_screen(s0.x, s0.y, 0, false);
		let s_mid = h.world_to_screen(Point::new(140.0, 0.0));
		h.pointer_move_screen(s_mid.x + 20.0, s_mid.y);
		let s1 = h.world_to_screen(hp_b);
		h.pointer_move_screen(s1.x, s1.y);
		h.pointer_up_screen(s1.x, s1.y);
		let ev = h.drain_events_json();
		assert!(ev.contains("edgeCreate"));
	}

	#[test]
	fn board_host_link_short_drag_does_not_emit_edge_create() {
		let mut h = BoardHost::new();
		h.set_size(800, 600, 1.0);
		h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
		let _ = h.drain_events_json();
		let hp_a = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
		let s0 = h.world_to_screen(hp_a);
		h.pointer_down_screen(s0.x, s0.y, 0, false);
		h.pointer_move_screen(s0.x + 2.0, s0.y);
		h.pointer_up_screen(s0.x + 2.0, s0.y);
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
	fn force_graph_rejects_bad_schema() {
		let err = apply_force_graph_layout_to_fixture_v1_json(r#"{"schema":"x","nodes":[],"edges":[]}"#, "{}").unwrap_err();
		assert!(err.contains("schema"));
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
}

// #endregion 🔖Tests
