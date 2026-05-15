//! 🧮 World-axis boxes and 2D predicates for rectangle/lasso selection (mirrors board JS helpers).

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
