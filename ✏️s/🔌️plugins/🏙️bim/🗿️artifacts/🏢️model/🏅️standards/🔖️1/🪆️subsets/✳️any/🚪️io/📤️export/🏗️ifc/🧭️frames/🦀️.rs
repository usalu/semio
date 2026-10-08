//! 🧭️ Pure placement arithmetic shared by every IFC family: loop orientation, vertical extents under a top constraint, axis frames and profile measures.

use crate::standards::v1::subsets::any::schema::inferences::storey_levels::{top_of, StoreyLevel};
use crate::{Point2, Profile, TopConstraint, Vertex};
use semio_framework_geometry::bulge::BulgeSeg;
use semio_framework_geometry::{loops, Point};
use std::collections::BTreeMap;

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

/// 🧭️ Resolves `(base_z, top_z)` of an element standing on `storey`: building-relative metres.
pub fn vertical(levels: &BTreeMap<String, StoreyLevel>, storey: &str, base_offset: f64, top: &TopConstraint) -> Option<(f64, f64)> {
    let own = levels.get(storey)?;
    let target = match top {
        TopConstraint::Storey { storey: target, .. } => levels.get(target),
        _ => None,
    };
    let base = own.elevation + base_offset;
    Some((base, top_of(top, base, own, target)))
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
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
