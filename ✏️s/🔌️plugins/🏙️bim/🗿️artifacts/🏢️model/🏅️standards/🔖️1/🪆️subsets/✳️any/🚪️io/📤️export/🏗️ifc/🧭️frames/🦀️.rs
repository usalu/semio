//! 🧭️ Pure placement arithmetic shared by every IFC family: loop orientation, vertical extents under a top constraint, axis frames and profile measures.

use crate::{Point2, Profile, Vertex};
use semio_framework_geometry::bulge::BulgeSeg;
use semio_framework_geometry::{loops, Point};

/// 📍️ Geometry-kit point of a snapshot point.
pub fn point(value: &Point2) -> Point {
    Point::new(value.x, value.y)
}

fn kit(vertices: &[Vertex]) -> Vec<loops::Vertex> {
    vertices.iter().map(|vertex| loops::Vertex::new(point(&vertex.point), vertex.bulge)).collect()
}

fn pairs(vertices: &[loops::Vertex]) -> Vec<([f64; 2], f64)> {
    vertices.iter().map(|vertex| ([vertex.point.x, vertex.point.y], vertex.bulge)).collect()
}

/// 🔷️ A snapshot loop as counter-clockwise `(point, bulge)` pairs.
pub fn ccw(vertices: &[Vertex]) -> Vec<([f64; 2], f64)> {
    pairs(&loops::ccw(&kit(vertices)))
}

/// 🔷️ A snapshot loop as clockwise `(point, bulge)` pairs (holes).
pub fn cw(vertices: &[Vertex]) -> Vec<([f64; 2], f64)> {
    pairs(&loops::reversed(&loops::ccw(&kit(vertices))))
}

/// 📐️ Exact area of a snapshot loop (arcs included).
pub fn area(vertices: &[Vertex]) -> f64 {
    loops::area(&kit(vertices))
}

/// 📏️ Perimeter of a snapshot loop (arcs included).
pub fn perimeter(vertices: &[Vertex]) -> f64 {
    loops::perimeter(&kit(vertices))
}

/// 📍️ `(point, unit tangent)` of an axis at arc length `s`.
pub fn frame_at(segment: &BulgeSeg, s: f64) -> ([f64; 2], [f64; 2]) {
    let point = segment.point_at_length(s);
    let tangent = segment.tangent_at_length(s);
    ([point.x, point.y], [tangent.x, tangent.y])
}

/// 📏️ Highest point of a profile above its local origin.
pub fn profile_top(profile: &Profile) -> f64 {
    match profile {
        Profile::Rectangle { depth, .. } | Profile::IShape { depth, .. } => depth / 2.0,
        Profile::Circle { diameter } => diameter / 2.0,
        Profile::Custom { outline } => outline.iter().map(|vertex| vertex.point.y).fold(f64::NEG_INFINITY, f64::max),
        Profile::Family { .. } => 0.0,
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
