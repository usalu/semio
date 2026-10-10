//! ▭️ `profile`: the outline of an authored profile in `(across, depth)` coordinates and its extents, the one definition frames, solids, rooms, bodies, plan and the sweep rules share.

use crate::Profile;
use semio_framework_geometry::loops;
use semio_framework_geometry::Point;

/// 📏️ Sagitta of every tessellated arc, in metres: 0.1 mm keeps the volume error of an arc wall near `2e-5` relative.
pub const CHORD_TOLERANCE: f64 = 1e-4;

/// ▭️ The outline of a profile in `(a, b)` = (across, depth) coordinates centred on the origin, counter-clockwise; circles and custom arcs are flattened within [`CHORD_TOLERANCE`].
pub fn profile_polygon(profile: &Profile) -> Vec<Point> {
    match profile {
        Profile::Rectangle { width, depth } => vec![Point::new(-width / 2.0, -depth / 2.0), Point::new(width / 2.0, -depth / 2.0), Point::new(width / 2.0, depth / 2.0), Point::new(-width / 2.0, depth / 2.0)],
        Profile::Circle { diameter } => {
            let radius = diameter / 2.0;
            let step = if radius > CHORD_TOLERANCE { 2.0 * (1.0 - CHORD_TOLERANCE / radius).acos() } else { std::f64::consts::FRAC_PI_2 };
            let count = ((std::f64::consts::TAU / step).ceil() as usize).clamp(16, 512);
            (0..count).map(|k| k as f64 * std::f64::consts::TAU / count as f64).map(|angle| Point::new(radius * angle.cos(), radius * angle.sin())).collect()
        }
        Profile::IShape { width, depth, web, flange } => {
            let (w, d, t, f) = (width / 2.0, depth / 2.0, web / 2.0, *flange);
            [(-w, -d), (w, -d), (w, -d + f), (t, -d + f), (t, d - f), (w, d - f), (w, d), (-w, d), (-w, d - f), (-t, d - f), (-t, -d + f), (-w, -d + f)].into_iter().map(|(a, b)| Point::new(a, b)).collect()
        }
        Profile::Custom { outline } => loops::flatten(&outline.iter().map(|vertex| loops::Vertex { point: Point::new(vertex.point.x, vertex.point.y), bulge: vertex.bulge }).collect::<Vec<_>>(), CHORD_TOLERANCE),
        Profile::Family { .. } => Vec::new(),
    }
}

/// ▭️ Extents `(across, depth)` of a profile outline.
pub fn profile_extents(outline: &[Point]) -> (f64, f64) {
    let span = |value: fn(&Point) -> f64| outline.iter().map(value).fold(f64::NEG_INFINITY, f64::max) - outline.iter().map(value).fold(f64::INFINITY, f64::min);
    if outline.is_empty() {
        (0.0, 0.0)
    } else {
        (span(|p| p.x), span(|p| p.y))
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
