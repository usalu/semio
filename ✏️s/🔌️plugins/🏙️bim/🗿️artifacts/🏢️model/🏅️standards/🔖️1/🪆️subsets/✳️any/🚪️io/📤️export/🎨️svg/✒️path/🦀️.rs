//! ✒️ Plan geometry as SVG path data: the model's metres become paper millimetres at the drawing scale, the y axis flips (north is up on the sheet), coordinates snap to a micrometre
//! of paper (arc radii keep six decimals: near a half circle the centre moves with the square root of the radius error), and a bulge `tan(sweep / 4)` becomes an elliptical-arc command whose flags follow from its sign and size: the picture is the plan as drawn y-up, so a counter-clockwise
//! (positive) bulge is the counter-clockwise arc of the sheet, which SVG calls sweep-flag 0 because its positive angle direction runs clockwise on a y-down canvas.
//! 📎 https://www.w3.org/TR/SVG11/paths.html#PathDataEllipticalArcCommands

use super::codec::PathCommand;
use crate::standards::v1::subsets::any::schema::inferences::plan_linework::PlanVertex;

/// 📐️ The drawing scale denominator: 1:100.
pub const SCALE: f64 = 100.0;

/// 📏️ Paper millimetres per model metre at the drawing scale.
pub const MM_PER_METRE: f64 = 1000.0 / SCALE;

const EPSILON: f64 = 1e-12;

/// 🖼️ The map from plan metres (y up) to local paper millimetres (y down): the plan's top-left corner `(min_x, max_y)` lands on `(left, top)`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Frame {
    pub min_x: f64,
    pub max_y: f64,
    pub left: f64,
    pub top: f64,
}

/// 🔢️ A coordinate snapped to a micrometre, negative zero normalised.
pub fn snap(value: f64) -> f64 {
    (value * 1000.0).round() / 1000.0 + 0.0
}

impl Frame {
    /// 📍️ The local paper position of a plan point.
    pub fn point(&self, x: f64, y: f64) -> (f64, f64) {
        (snap((x - self.min_x) * MM_PER_METRE + self.left), snap((self.max_y - y) * MM_PER_METRE + self.top))
    }
}

/// 🌀️ Whether the segment leaving a vertex is an arc.
pub fn is_arc(vertex: &PlanVertex) -> bool {
    vertex.bulge.abs() > EPSILON
}

/// 🔢️ The vertices as local paper points.
pub fn points(vertices: &[PlanVertex], frame: &Frame) -> Vec<(f64, f64)> {
    vertices.iter().map(|vertex| frame.point(vertex.x, vertex.y)).collect()
}

fn radius(chord: f64, bulge: f64) -> f64 {
    (chord * (1.0 + bulge * bulge) / (4.0 * bulge.abs()) * 1e6).round() / 1e6
}

fn arc(from: (f64, f64), to: (f64, f64), bulge: f64) -> PathCommand {
    let radius = radius(((to.0 - from.0).powi(2) + (to.1 - from.1).powi(2)).sqrt(), bulge);
    PathCommand::Arc { rx: radius, ry: radius, x_axis_rotation: 0.0, large_arc: bulge.abs() > 1.0, sweep: bulge < 0.0, x: to.0, y: to.1, relative: false }
}

/// ✒️ The path commands of one ring or open path: `M`, then `L` or `A` per segment, `Z` when closed.
pub fn commands(vertices: &[PlanVertex], closed: bool, frame: &Frame) -> Vec<PathCommand> {
    let at = points(vertices, frame);
    let mut out = vec![PathCommand::MoveTo { x: at[0].0, y: at[0].1, relative: false }];
    let segments = if closed { at.len() } else { at.len() - 1 };
    for index in 0..segments {
        let (from, to, bulge) = (at[index], at[(index + 1) % at.len()], vertices[index].bulge);
        if is_arc(&vertices[index]) {
            out.push(arc(from, to, bulge));
        } else if index + 1 < at.len() {
            out.push(PathCommand::LineTo { x: to.0, y: to.1, relative: false });
        }
    }
    if closed {
        out.push(PathCommand::ClosePath);
    }
    out
}

/// 🌀️ Whether any segment drawn is an arc.
pub fn has_arc(vertices: &[PlanVertex], closed: bool) -> bool {
    let drawn = if closed { vertices.len() } else { vertices.len().saturating_sub(1) };
    vertices[..drawn].iter().any(is_arc)
}

/// 📐️ The unsigned area enclosed by a ring of straight segments, in square paper millimetres.
pub fn ring_area(points: &[(f64, f64)]) -> f64 {
    let twice: f64 = (0..points.len()).map(|index| points[index].0 * points[(index + 1) % points.len()].1 - points[(index + 1) % points.len()].0 * points[index].1).sum();
    twice.abs() / 2.0
}

/// 📏️ The length of a path of straight segments, in paper millimetres.
pub fn path_length(points: &[(f64, f64)], closed: bool) -> f64 {
    let segments = if closed { points.len() } else { points.len().saturating_sub(1) };
    (0..segments).map(|index| ((points[(index + 1) % points.len()].0 - points[index].0).powi(2) + (points[(index + 1) % points.len()].1 - points[index].1).powi(2)).sqrt()).sum()
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
