//! 🕳️ Control perimeter: the sampled points of the basic control perimeter `u1` at `2d` around a flat-slab column in the panel frame (`x` along the span from the panel start, `y` across with the panel centre line at 0), and the shear around it. See [`super`].

use super::*;
use std::f64::consts::FRAC_PI_2;

const ARC_SEGMENTS: usize = 4;

/// 📍️ A control-perimeter point in the panel frame with its lever arm `t ∈ [-1, 1]` along the span from the column centre.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct PerimeterPoint {
    pub(super) x: f64,
    pub(super) y: f64,
    pub(super) lever: f64,
}

fn clip(ring: &[(f64, f64)], inside: impl Fn((f64, f64)) -> f64) -> Vec<(f64, f64)> {
    let mut out = Vec::new();
    for i in 0..ring.len() {
        let (p, q) = (ring[i], ring[(i + 1) % ring.len()]);
        let (fp, fq) = (inside(p), inside(q));
        if fp >= 0.0 {
            out.push(p);
        }
        if (fp >= 0.0) != (fq >= 0.0) {
            let t = fp / (fp - fq);
            out.push((p.0 + t * (q.0 - p.0), p.1 + t * (q.1 - p.1)));
        }
    }
    out
}

/// 🕳️ The rounded control perimeter at distance `2d` of the column `(c1, c2)` at `position` of a square panel of side `span`, clipped at the free edges `x = span` and `y = span / 2`, without the points between the tangents of the openings.
pub(super) fn control_perimeter(position: semio_s_artifact_norm_en1992_model::ColumnPosition, c1: f64, c2: f64, d: f64, span: f64, wedges: &[Wedge]) -> Vec<PerimeterPoint> {
    let (cx, cy) = match position {
        semio_s_artifact_norm_en1992_model::ColumnPosition::Edge => (span - c1 / 2.0, 0.0),
        semio_s_artifact_norm_en1992_model::ColumnPosition::Corner => (span - c1 / 2.0, span / 2.0 - c2 / 2.0),
        _ => (span / 2.0, 0.0),
    };
    let (hw, hd, k) = (c1 / 2.0, c2 / 2.0, 2.0 * d);
    let corners = [(cx + hw, cy + hd, 0.0), (cx - hw, cy + hd, FRAC_PI_2), (cx - hw, cy - hd, 2.0 * FRAC_PI_2), (cx + hw, cy - hd, 3.0 * FRAC_PI_2)];
    let ring: Vec<(f64, f64)> = corners
        .iter()
        .flat_map(|&(x, y, start)| (0..=ARC_SEGMENTS).map(move |i| {
            let a = start + FRAC_PI_2 * i as f64 / ARC_SEGMENTS as f64;
            (x + k * a.cos(), y + k * a.sin())
        }))
        .collect();
    let ring = match position {
        semio_s_artifact_norm_en1992_model::ColumnPosition::Edge => clip(&ring, |q| span - q.0),
        semio_s_artifact_norm_en1992_model::ColumnPosition::Corner => clip(&clip(&ring, |q| span - q.0), |q| span / 2.0 - q.1),
        _ => ring,
    };
    ring.into_iter().filter(|&(x, y)| !wedges.iter().any(|w| w.contains((x - cx, y - cy)))).map(|(x, y)| PerimeterPoint { x, y, lever: ((x - cx) / (hw + k).max(1e-9)).clamp(-1.0, 1.0) }).collect()
}

/// 📐️ Local shear factor on `u1`: an interior column carries its load increase `β` as a linear shear gradient `2 - β … β` along the span (mean 1, peak `β`), an edge or corner column the uniform `β` of the NDP.
pub(super) fn local_factor(position: semio_s_artifact_norm_en1992_model::ColumnPosition, beta: f64, lever: f64) -> f64 {
    if matches!(position, semio_s_artifact_norm_en1992_model::ColumnPosition::Edge | semio_s_artifact_norm_en1992_model::ColumnPosition::Corner) { beta } else { 1.0 + (beta - 1.0) * lever }
}
