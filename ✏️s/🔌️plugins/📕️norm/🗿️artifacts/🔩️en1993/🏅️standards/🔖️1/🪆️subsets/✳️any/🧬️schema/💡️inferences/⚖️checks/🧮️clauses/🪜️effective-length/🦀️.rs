//! 🪜️ Effective length factor `K = L_cr / L` of a column in a frame from the distribution factors `G = sum(I_c/L_c) / sum(alpha I_b/L_b)` at both ends: the nomogram of Jackson and Moreland solved on its transcendental equations for non-sway and sway frames. See [EN 1993-1-1 6.3.1.3](https://eurocodes.jrc.ec.europa.eu/).

use std::f64::consts::PI;

/// ♾️ Numerical stand-in for an infinite distribution factor (a pinned end); the error of `K` is of the order of its reciprocal.
pub const UNBOUNDED: f64 = 1.0e9;

/// 🧮️ Distribution factor `G` of a node from the stiffness sums of its columns and beams; no beam means a pinned end.
pub fn distribution(column: f64, beam: f64) -> f64 {
    if beam <= 0.0 { UNBOUNDED } else { (column / beam).min(UNBOUNDED) }
}

fn bisect(f: impl Fn(f64) -> f64, mut lo: f64, mut hi: f64) -> f64 {
    let mut f_lo = f(lo);
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        let f_mid = f(mid);
        if f_lo * f_mid <= 0.0 {
            hi = mid;
        } else {
            lo = mid;
            f_lo = f_mid;
        }
    }
    0.5 * (lo + hi)
}

/// 🧮️ Non-sway factor `K` in `[0.5, 1]`: root of `GA GB x^2 sin(x)/4 + (GA+GB)(sin x - x cos x)/2 + 2 (1 - cos x)/x - sin x = 0` with `x = pi / K` (the nomogram equation multiplied by `sin x`, free of poles).
pub fn k_non_sway(ga: f64, gb: f64) -> f64 {
    let h = |x: f64| ga * gb * x * x * x.sin() / 4.0 + (ga + gb) * (x.sin() - x * x.cos()) / 2.0 + 2.0 * (1.0 - x.cos()) / x - x.sin();
    PI / bisect(h, PI, 2.0 * PI)
}

/// 🧮️ Sway factor `K >= 1`: root of `(GA GB x^2 - 36) sin x - 6 (GA+GB) x cos x = 0` with `x = pi / K`; `None` when both ends are pinned (a mechanism).
pub fn k_sway(ga: f64, gb: f64) -> Option<f64> {
    if ga >= UNBOUNDED && gb >= UNBOUNDED {
        return None;
    }
    let h = |x: f64| (ga * gb * x * x - 36.0) * x.sin() - 6.0 * (ga + gb) * x * x.cos();
    Some(PI / bisect(h, 1e-9, PI))
}
