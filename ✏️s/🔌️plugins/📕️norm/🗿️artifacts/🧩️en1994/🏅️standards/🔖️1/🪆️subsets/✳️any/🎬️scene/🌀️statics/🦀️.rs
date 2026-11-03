//! 🌀️ Statics of a uniformly loaded continuous beam from the catalogue solver ([`continuous`]): the normalised deflection line and the contraflexure zones of the hogging regions for simple, end, interior and cantilever spans with the cracked stiffness EI₂ at the internal supports, validated against PyNite.

use super::checks::catalogue::continuous::{solve, Layout, Stiffness};

const POINTS: usize = 160;

/// 🌀️ Result of the unit-load analysis over the whole length mapped to `t` in `[0, 1]`: the deflection normalised to a largest magnitude of 1, and the hogging zones as `t` intervals.
pub struct Line {
    pub deflection: Vec<(f64, f64)>,
    pub hogging: Vec<(f64, f64)>,
}

fn unit_loads(layout: &Layout, own_only: bool) -> Vec<f64> {
    (0..layout.lengths.len()).map(|i| if own_only && i != layout.own { 0.0 } else { 1.0 }).collect()
}

/// 🌀️ Unit uniform load on every span (a cantilever is loaded alone so its tip moves downward): the downward deflection normalised to a largest magnitude of 1 at `t = x / L_total`, and the intervals of hogging moment around the supports.
pub fn uniform_load_line(layout: &Layout, stiffness: Stiffness) -> Option<Line> {
    let total = layout.total();
    let all = solve(layout, &unit_loads(layout, false), stiffness);
    let shape = if layout.cantilever { solve(layout, &unit_loads(layout, true), stiffness) } else { all.clone() };
    let curve = shape.deflection(POINTS);
    let top = curve.iter().map(|p| p.1.abs()).fold(0.0, f64::max);
    if top <= 0.0 || total <= 0.0 {
        return None;
    }
    Some(Line {
        deflection: curve.into_iter().map(|(x, w)| (x / total, w / top)).collect(),
        hogging: all.hogging_zones().into_iter().map(|(a, b)| (a / total, b / total)).collect(),
    })
}

/// 📈️ Piecewise-linear value of an ascending `(t, value)` table at `t` (clamped to the end values).
pub fn interpolate(table: &[(f64, f64)], t: f64) -> f64 {
    let i = table.partition_point(|p| p.0 <= t);
    match (i.checked_sub(1).map(|k| table[k]), table.get(i)) {
        (Some(a), Some(b)) => a.1 + (b.1 - a.1) * (t - a.0) / (b.0 - a.0),
        (Some(a), None) => a.1,
        (None, Some(b)) => b.1,
        (None, None) => 0.0,
    }
}
