//! 🌀️ S–N curves: EN 1993-1-9 fatigue strength curves of a detail category in log-log space.

use crate::diagram_kit::Xy;

/// 🔩️ Reference cycle count of the detail category.
pub const REFERENCE_CYCLES: f64 = 2.0e6;
/// 🔩️ Constant-amplitude fatigue limit cycle count.
pub const KNEE_CYCLES: f64 = 5.0e6;
/// 🔩️ Cut-off cycle count.
pub const CUTOFF_CYCLES: f64 = 1.0e8;

/// 🔩️ Constant amplitude fatigue limit ΔσD for detail category ΔσC.
pub fn fatigue_limit(delta_sigma_c: f64) -> f64 {
    (2.0f64 / 5.0).powf(1.0 / 3.0) * delta_sigma_c
}

/// 🔩️ Cut-off limit ΔσL for detail category ΔσC.
pub fn cutoff_limit(delta_sigma_c: f64) -> f64 {
    (5.0f64 / 100.0).powf(1.0 / 5.0) * fatigue_limit(delta_sigma_c)
}

/// 🔩️ Fatigue strength ΔσR in the same unit as ΔσC for `cycles`, flat beyond the cut-off.
pub fn strength(delta_sigma_c: f64, cycles: f64) -> f64 {
    if cycles <= KNEE_CYCLES {
        delta_sigma_c * (REFERENCE_CYCLES / cycles).powf(1.0 / 3.0)
    } else if cycles <= CUTOFF_CYCLES {
        fatigue_limit(delta_sigma_c) * (KNEE_CYCLES / cycles).powf(1.0 / 5.0)
    } else {
        cutoff_limit(delta_sigma_c)
    }
}

/// 🌀️ Wöhler line vertices `[N, ΔσR]` from `n_min` to `n_max`; straight in log-log, so the vertices are exact.
pub fn sn_curve(delta_sigma_c: f64, n_min: f64, n_max: f64) -> Vec<Xy> {
    let mut ns = vec![n_min];
    ns.extend([REFERENCE_CYCLES, KNEE_CYCLES, CUTOFF_CYCLES].into_iter().filter(|n| *n > n_min && *n < n_max));
    ns.push(n_max);
    ns.into_iter().map(|n| [n, strength(delta_sigma_c, n)]).collect()
}

/// 🧮️ Allowable cycles for a stress range on the curve; `None` below the cut-off limit.
pub fn allowable_cycles(delta_sigma_c: f64, range: f64) -> Option<f64> {
    if range <= cutoff_limit(delta_sigma_c) {
        return None;
    }
    let d = fatigue_limit(delta_sigma_c);
    Some(if range >= d { REFERENCE_CYCLES * (delta_sigma_c / range).powi(3) } else { KNEE_CYCLES * (d / range).powi(5) })
}
