//! 🧱️ Stress blocks: EN 1992-1-1 3.1.7 rectangular block and parabola-rectangle geometry over a rectangular section.

use crate::diagram_kit::{curves::sample, Xy};

/// 🏛️ Rectangular stress block factors λ and η for the characteristic cylinder strength `fck` in MPa.
pub fn block_factors(fck: f64) -> (f64, f64) {
    if fck <= 50.0 {
        (0.8, 1.0)
    } else {
        (0.8 - (fck - 50.0) / 400.0, 1.0 - (fck - 50.0) / 200.0)
    }
}

/// 🏛️ Ultimate concrete strain εcu3 in per mille.
pub fn ultimate_strain_permille(fck: f64) -> f64 {
    if fck < 50.0 {
        3.5
    } else {
        2.6 + 35.0 * ((90.0 - fck) / 100.0).powi(4)
    }
}

/// 🧱️ Geometry of a rectangular block over a section: depths are measured downwards from the compressed edge.
#[derive(Clone, Debug, PartialEq)]
pub struct StressBlock {
    pub stress_ring: Vec<Xy>,
    pub strain_line: Vec<Xy>,
    pub neutral_axis_depth: f64,
    pub block_depth: f64,
    pub compression_force: f64,
    pub lever_arm: f64,
    pub steel_strain_permille: f64,
}

/// 🧱️ Rectangular stress block for width `b`, effective depth `d`, neutral axis depth `x` and design strength `fcd` (force in the unit of `fcd·b·x`).
pub fn rectangular_block(b: f64, d: f64, x: f64, fck: f64, fcd: f64) -> StressBlock {
    let (lambda, eta) = block_factors(fck);
    let cu = ultimate_strain_permille(fck);
    let depth = lambda * x;
    let stress = eta * fcd;
    let steel = cu * (d - x) / x;
    StressBlock {
        stress_ring: vec![[0.0, 0.0], [stress, 0.0], [stress, depth], [0.0, depth]],
        strain_line: vec![[cu, 0.0], [-steel, d]],
        neutral_axis_depth: x,
        block_depth: depth,
        compression_force: stress * b * depth,
        lever_arm: d - depth / 2.0,
        steel_strain_permille: steel,
    }
}

/// 🏛️ Parabola-rectangle exponent n of EN 1992-1-1 Table 3.1.
pub fn parabola_exponent(fck: f64) -> f64 {
    if fck < 50.0 { 2.0 } else { 1.4 + 23.4 * ((90.0 - fck) / 100.0).powi(4) }
}

/// 🏛️ Strain at the end of the parabola εc2 in per mille.
pub fn parabola_strain_permille(fck: f64) -> f64 {
    if fck < 50.0 { 2.0 } else { 2.0 + 0.085 * (fck - 50.0).powf(0.53) }
}

/// 🏛️ Parabola-rectangle stress σc(ε) with ε in per mille.
pub fn parabola_rectangle_stress(fck: f64, fcd: f64, strain_permille: f64) -> f64 {
    let (c2, cu, n) = (parabola_strain_permille(fck), ultimate_strain_permille(fck), parabola_exponent(fck));
    if strain_permille <= 0.0 || strain_permille > cu {
        0.0
    } else if strain_permille < c2 {
        fcd * (1.0 - (1.0 - strain_permille / c2).powf(n))
    } else {
        fcd
    }
}

/// 🧱️ The σ–ε law of the parabola-rectangle diagram sampled to εcu with εc2 included.
pub fn parabola_rectangle_curve(fck: f64, fcd: f64, n: usize) -> Vec<Xy> {
    let cu = ultimate_strain_permille(fck);
    let mut pts = sample(|e| parabola_rectangle_stress(fck, fcd, e), 0.0, parabola_strain_permille(fck), n);
    pts.push([cu, fcd]);
    pts
}
