//! 🧭️ `plan`: the conversions from authored plan values to the geometry crate and the length of an authored axis, the one definition mutations, inferences, the editor and the exports share.

use crate::{Axis, Point2};
use semio_framework_geometry::bulge::BulgeSeg;
use semio_framework_geometry::Point;

/// 📍️ An authored plan point as a geometry point.
pub fn point(p: &Point2) -> Point {
    Point::new(p.x, p.y)
}

/// 📍️ A geometry point as an authored plan point.
pub fn mark(p: Point) -> Point2 {
    Point2 { x: p.x, y: p.y }
}

/// 〰️ The bulged segment of an authored axis (line, or arc by its bulge `tan(sweep / 4)`).
pub fn segment_of(axis: &Axis) -> BulgeSeg {
    match axis {
        Axis::Line { start, end } => BulgeSeg::line(point(start), point(end)),
        Axis::Arc { start, end, bulge } => BulgeSeg::new(point(start), point(end), *bulge),
    }
}

/// 📏️ Arc length of an authored axis: `bulge = tan(sweep / 4)`.
pub fn axis_length(axis: &Axis) -> f64 {
    segment_of(axis).length()
}

/// 📐️ The largest lean of a column from the vertical in radians (60 degrees).
pub const MAX_TILT: f64 = std::f64::consts::FRAC_PI_3;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
