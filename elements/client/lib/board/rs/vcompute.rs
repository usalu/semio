//! 🦀 Board geometry via `vello::kurbo` curves and `vello::Scene` stroke encoding (Vello stack).

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
