//! 🌀️ Plan arcs as cubic Béziers: a segment with bulge `tan(sweep / 4)` becomes at most quarter circles, each approximated by one cubic whose control points sit at `4/3 · tan(sweep / 4)` of the radius along the tangents (the radial error of a quarter
//! circle is below 0.03 %). PDF paths have no arc operator, so this is how a curved wall reaches the page. Pure geometry in the plane of the plan, y up; the caller maps the points to paper.
//! 📎 https://pomax.github.io/bezierinfo/#circles_cubic

use std::f64::consts::FRAC_PI_2;

/// 🔢️ A point of the plan.
pub type Point = (f64, f64);

/// 🌀️ One cubic Bézier segment: the two control points and the end point (the start is the end of the previous segment).
pub type Cubic = (Point, Point, Point);

const EPSILON: f64 = 1e-12;

/// 🌀️ The cubic Béziers that draw the arc from `from` to `to` with the given bulge (positive counter-clockwise in a y-up plan); a bulge of zero or a chord of no length is one straight "curve" with its control points on the chord.
pub fn arc_cubics(from: Point, to: Point, bulge: f64) -> Vec<Cubic> {
    let (dx, dy) = (to.0 - from.0, to.1 - from.1);
    let chord = dx.hypot(dy);
    if bulge.abs() < EPSILON || chord < EPSILON {
        return vec![((from.0 + dx / 3.0, from.1 + dy / 3.0), (from.0 + 2.0 * dx / 3.0, from.1 + 2.0 * dy / 3.0), to)];
    }
    let sweep = 4.0 * bulge.atan();
    let radius = chord / (2.0 * (sweep / 2.0).sin().abs());
    let (middle, normal) = (((from.0 + to.0) / 2.0, (from.1 + to.1) / 2.0), (-dy / chord, dx / chord));
    let offset = chord / 2.0 / (sweep / 2.0).tan();
    let centre = (middle.0 + normal.0 * offset, middle.1 + normal.1 * offset);
    let start = (from.1 - centre.1).atan2(from.0 - centre.0);
    let parts = (sweep.abs() / FRAC_PI_2 - 1e-9).ceil().max(1.0) as usize;
    let step = sweep / parts as f64;
    let reach = 4.0 / 3.0 * (step / 4.0).tan() * radius;
    let at = |angle: f64| (centre.0 + radius * angle.cos(), centre.1 + radius * angle.sin());
    let tangent = |angle: f64| (-angle.sin(), angle.cos());
    (0..parts)
        .map(|index| {
            let (first, last) = (start + step * index as f64, start + step * (index + 1) as f64);
            let (a, b) = (at(first), if index + 1 == parts { to } else { at(last) });
            let (ta, tb) = (tangent(first), tangent(last));
            ((a.0 + reach * ta.0, a.1 + reach * ta.1), (b.0 - reach * tb.0, b.1 - reach * tb.1), b)
        })
        .collect()
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
