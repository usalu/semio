//! 📐️ Plan geometry of the authoring tools: points, directions, arc bulges and projections onto wall axes, all in model metres. Pure functions over `[x, y]` pairs; the segment
//! kernel (`BulgeSeg`) is the framework's, never re-derived here.

use crate::standards::v1::subsets::any::schema::authored::plan::segment_of;
use crate::{Axis, Point2, Vertex};
use semio_framework_geometry::bulge::BulgeSeg;
use semio_framework_geometry::Point;

/// 📍️ A point of the plan in metres.
pub type P = [f64; 2];

/// 🔬️ Two points closer than this (metres) are the same point.
pub const COINCIDENT: f64 = 1e-6;

pub fn dist(a: P, b: P) -> f64 {
    (b[0] - a[0]).hypot(b[1] - a[1])
}

pub fn mid(a: P, b: P) -> P {
    [(a[0] + b[0]) / 2.0, (a[1] + b[1]) / 2.0]
}

pub fn angle(from: P, to: P) -> f64 {
    (to[1] - from[1]).atan2(to[0] - from[0])
}

pub fn polar(origin: P, direction: f64, length: f64) -> P {
    [origin[0] + length * direction.cos(), origin[1] + length * direction.sin()]
}

pub fn point2(p: P) -> Point2 {
    Point2 { x: p[0], y: p[1] }
}

pub fn from_point2(p: Point2) -> P {
    [p.x, p.y]
}

pub fn pt(p: P) -> Point {
    Point::new(p[0], p[1])
}

pub fn same(a: P, b: P) -> bool {
    dist(a, b) <= COINCIDENT
}

/// 🌙️ The axis through `start` and `end`; a bulge below a micro-radian is a line.
pub fn axis_of(start: P, end: P, bulge: f64) -> Axis {
    if bulge.abs() < 1e-9 {
        Axis::Line { start: point2(start), end: point2(end) }
    } else {
        Axis::Arc { start: point2(start), end: point2(end), bulge }
    }
}

pub fn axis_ends(axis: &Axis) -> (P, P) {
    let (Axis::Line { start, end } | Axis::Arc { start, end, .. }) = axis;
    (from_point2(*start), from_point2(*end))
}

pub fn axis_bulge(axis: &Axis) -> f64 {
    match axis {
        Axis::Line { .. } => 0.0,
        Axis::Arc { bulge, .. } => *bulge,
    }
}

/// 🌙️ The bulge of the arc from `start` to `end` through `through`; none when the three points are collinear.
pub fn bulge_through(start: P, through: P, end: P) -> Option<f64> {
    BulgeSeg::from_three_points(pt(start), pt(through), pt(end)).map(|segment| segment.bulge)
}

/// 🌙️ The flattened points of the arc or line from `start` to `end` within `tolerance` metres.
pub fn flatten(start: P, end: P, bulge: f64, tolerance: f64) -> Vec<P> {
    BulgeSeg::new(pt(start), pt(end), bulge).flatten(tolerance).into_iter().map(|p| [p.x, p.y]).collect()
}

/// 🎯️ Where a point falls on an axis: the arc length from the start to the closest point, that point, its distance and the side (positive left of travel).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OnAxis {
    pub offset: f64,
    pub point: P,
    pub distance: f64,
    pub side: f64,
}

pub fn project(axis: &Axis, p: P) -> OnAxis {
    let segment = segment_of(axis);
    let closest = segment.closest(pt(p));
    let length = segment.length();
    let offset = (closest.t * length).clamp(0.0, length);
    let tangent = segment.tangent_at_length(offset);
    let at = [closest.point.x, closest.point.y];
    let side = tangent.x * (p[1] - at[1]) - tangent.y * (p[0] - at[0]);
    OnAxis { offset, point: at, distance: closest.distance, side }
}

pub fn axis_length(axis: &Axis) -> f64 {
    segment_of(axis).length()
}

pub fn axis_point_at(axis: &Axis, length: f64) -> P {
    let point = segment_of(axis).point_at_length(length);
    [point.x, point.y]
}

pub fn axis_tangent_at(axis: &Axis, length: f64) -> P {
    let tangent = segment_of(axis).tangent_at_length(length);
    [tangent.x, tangent.y]
}

/// 🧭️ Twice the signed area of a ring: positive when counter-clockwise.
pub fn signed_area(ring: &[P]) -> f64 {
    ring.iter().zip(ring.iter().cycle().skip(1)).map(|(a, b)| a[0] * b[1] - b[0] * a[1]).sum::<f64>() / 2.0
}

/// 🧭️ The ring running counter-clockwise, reversed when it ran clockwise.
pub fn counter_clockwise(mut ring: Vec<P>) -> Vec<P> {
    if signed_area(&ring) < 0.0 {
        ring.reverse();
    }
    ring
}

pub fn loop_of(ring: &[P]) -> Vec<Vertex> {
    ring.iter().map(|p| Vertex { point: point2(*p), bulge: 0.0 }).collect()
}

/// 🔲️ The four corners of the rectangle spanned by two opposite corners, counter-clockwise.
pub fn rectangle(a: P, b: P) -> Vec<P> {
    let (x0, x1, y0, y1) = (a[0].min(b[0]), a[0].max(b[0]), a[1].min(b[1]), a[1].max(b[1]));
    vec![[x0, y0], [x1, y0], [x1, y1], [x0, y1]]
}

pub fn rotate_about(p: P, pivot: P, turn: f64) -> P {
    let (sin, cos) = turn.sin_cos();
    let (dx, dy) = (p[0] - pivot[0], p[1] - pivot[1]);
    [pivot[0] + dx * cos - dy * sin, pivot[1] + dx * sin + dy * cos]
}

/// 🧭️ `angle` turned into the nearest multiple of `step` radians.
pub fn quantised(angle: f64, step: f64) -> f64 {
    (angle / step).round() * step
}

pub fn translate(p: P, vector: P) -> P {
    [p[0] + vector[0], p[1] + vector[1]]
}

pub fn bounds(points: impl IntoIterator<Item = P>) -> Option<[f64; 4]> {
    points.into_iter().fold(None, |bounds, p| match bounds {
        None => Some([p[0], p[1], p[0], p[1]]),
        Some([x0, y0, x1, y1]) => Some([x0.min(p[0]), y0.min(p[1]), x1.max(p[0]), y1.max(p[1])]),
    })
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
