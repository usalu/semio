//! 📐️ Shared validity rules of the horizontal elements (slabs and roofs): closed bulged loops, holes strictly inside their boundary,
//! slope and pitch ranges. Pure checks over authored parameters; each returns the reason a value is refused, none when it is fine.

use crate::{RoofShape, Slope, Vertex};
use semio_framework_geometry::bulge::{intersect, Extent};
use semio_framework_geometry::loops;
use semio_framework_geometry::vector::LENGTH_EPS;
use semio_framework_geometry::Point;
use std::f64::consts::PI;

/// 📐️ Steepest accepted inclination (89 degrees, in radians) of a slab slope or a roof pitch.
pub const MAX_INCLINATION: f64 = 89.0 * PI / 180.0;

fn ring(vertices: &[Vertex]) -> Vec<loops::Vertex> {
    vertices.iter().map(|vertex| loops::Vertex::new(Point::new(vertex.point.x, vertex.point.y), vertex.bulge)).collect()
}

fn crossing(a: &[loops::Vertex], b: &[loops::Vertex]) -> bool {
    let (left, right) = (loops::segments(a), loops::segments(b));
    left.iter().any(|x| right.iter().any(|y| !intersect(x, y, Extent::Bounded).is_empty()))
}

fn enclosed(outer: &[loops::Vertex], inner: &[loops::Vertex]) -> bool {
    loops::contains(outer, inner[0].point)
}

/// 🔷️ Why a loop cannot bound a horizontal element: fewer than three vertices, non-finite or repeated vertices, self-intersection,
/// zero area or clockwise orientation.
pub fn loop_fault(vertices: &[Vertex]) -> Option<&'static str> {
    if vertices.len() < 3 {
        return Some("A loop needs at least three vertices.");
    }
    if !vertices.iter().all(|vertex| vertex.point.x.is_finite() && vertex.point.y.is_finite() && vertex.bulge.is_finite()) {
        return Some("A loop needs finite vertices.");
    }
    let outline = ring(vertices);
    if loops::segments(&outline).iter().any(|segment| segment.chord() <= LENGTH_EPS) {
        return Some("A loop must not repeat a vertex.");
    }
    if !loops::self_intersections(&outline).is_empty() {
        return Some("A loop must not intersect itself.");
    }
    let area = loops::signed_area(&outline);
    if area.abs() <= LENGTH_EPS {
        return Some("A loop must enclose area.");
    }
    if area < 0.0 {
        return Some("A loop must run counter-clockwise.");
    }
    None
}

/// 🕳️ Why the holes cannot pierce the boundary: an invalid hole loop, a hole that touches, crosses or leaves the boundary, or
/// two holes that touch, cross or nest.
pub fn holes_fault(boundary: &[Vertex], holes: &[Vec<Vertex>]) -> Option<String> {
    let outer = ring(boundary);
    let outlines: Vec<Vec<loops::Vertex>> = holes.iter().map(|hole| ring(hole)).collect();
    for (index, hole) in holes.iter().enumerate() {
        if let Some(fault) = loop_fault(hole) {
            return Some(format!("Hole {index}: {fault}"));
        }
        if crossing(&outer, &outlines[index]) || !enclosed(&outer, &outlines[index]) {
            return Some(format!("Hole {index} must lie inside the boundary."));
        }
        for (other, earlier) in outlines.iter().enumerate().take(index) {
            if crossing(earlier, &outlines[index]) || enclosed(earlier, &outlines[index]) || enclosed(&outlines[index], earlier) {
                return Some(format!("Holes {other} and {index} must not overlap."));
            }
        }
    }
    None
}

/// 📐️ Why a slab slope is refused: a non-finite direction or an angle outside [0, 89 degrees).
pub fn slope_fault(slope: &Slope) -> Option<&'static str> {
    let valid = slope.direction.is_finite() && slope.angle.is_finite() && (0.0..MAX_INCLINATION).contains(&slope.angle);
    (!valid).then_some("A slope needs a finite direction and an angle in [0, 89 degrees).")
}

/// 🏠️ Why a roof shape is refused: a pitch outside (0, 89 degrees), a non-finite direction or a non-positive mansard break height.
pub fn shape_fault(shape: &RoofShape) -> Option<&'static str> {
    let pitched = |pitch: f64| pitch > 0.0 && pitch < MAX_INCLINATION;
    let valid = match shape {
        RoofShape::Flat => true,
        RoofShape::Shed { pitch, direction } => pitched(*pitch) && direction.is_finite(),
        RoofShape::Gable { pitch, ridge_direction } => pitched(*pitch) && ridge_direction.is_finite(),
        RoofShape::Hip { pitch } => pitched(*pitch),
        RoofShape::Mansard { lower_pitch, upper_pitch, break_height } => pitched(*lower_pitch) && pitched(*upper_pitch) && break_height.is_finite() && *break_height > 0.0,
    };
    (!valid).then_some("A pitched roof needs pitches in (0, 89 degrees), finite directions and a positive break height.")
}

/// 🏠️ Whether a roof overhang is refused: it must be a finite, non-negative length.
pub fn overhang_fault(overhang: f64) -> Option<&'static str> {
    (!(overhang.is_finite() && overhang >= 0.0)).then_some("A roof overhang must be a finite, non-negative length.")
}
