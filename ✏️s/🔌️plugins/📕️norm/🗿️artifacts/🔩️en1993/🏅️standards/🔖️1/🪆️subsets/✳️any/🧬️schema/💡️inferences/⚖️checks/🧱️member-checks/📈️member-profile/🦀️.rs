//! 📈️ Member profile: relative bending moment and shear along a member from its declared moment diagram, the basis of every station emitted by the member checks. See [`super`] and EN 1993-1-1 6.3.

use super::*;

pub(super) const STATIONS: usize = 10;

/// 📈️ `M(s)/max|M|` at the member coordinate `s` in `[0, 1]`: constant for `uniform`, the simply supported parabola `4s(1-s)` for `parabolic`, the straight line between the end moments `M` and `ψ·M` otherwise.
pub(super) fn moment_shape(member: &SteelMember, s: f64) -> f64 {
    match member.moment_diagram {
        semio_s_artifact_norm_en1993_model::MomentDiagram::Uniform => 1.0,
        semio_s_artifact_norm_en1993_model::MomentDiagram::Parabolic => 4.0 * s * (1.0 - s),
        _ => {
            let psi = member.end_moment_ratio_psi;
            ((1.0 - (1.0 - psi) * s) / psi.abs().max(1.0)).abs()
        }
    }
}

/// 📈️ `V(s)/max|V|`: the antisymmetric `|1-2s|` of a uniformly loaded span for `parabolic`, constant (the slope of the moment line) otherwise.
pub(super) fn shear_shape(member: &SteelMember, s: f64) -> f64 {
    if member.moment_diagram == semio_s_artifact_norm_en1993_model::MomentDiagram::Parabolic { (1.0 - 2.0 * s).abs() } else { 1.0 }
}

/// 🧮️ `demand / capacity`, saturating to infinity (capped by the station) for a vanishing capacity.
pub(super) fn util(demand: f64, capacity: f64) -> f64 {
    if capacity > 1e-12 { demand / capacity } else if demand > 0.0 { f64::INFINITY } else { 0.0 }
}

/// 📐️ `|w(s)| / max|w|` of the elastic deflection line under the declared moment diagram (`EI w'' = -M`, simply supported): `s - 2s³ + s⁴` for `parabolic`, `s(1-s)` for `uniform`, the cubic of the end moments `M` and `ψ M` otherwise; normalised on the sampled stations so the largest station carries the governing deflection.
pub(super) fn deflection_shape(member: &SteelMember, s: f64) -> f64 {
    let c = 1.0 - member.end_moment_ratio_psi;
    let w = |s: f64| {
        match member.moment_diagram {
            semio_s_artifact_norm_en1993_model::MomentDiagram::Uniform => s * (1.0 - s),
            semio_s_artifact_norm_en1993_model::MomentDiagram::Parabolic => s - 2.0 * s.powi(3) + s.powi(4),
            _ => s * (0.5 - c / 6.0) - s * s / 2.0 + c * s.powi(3) / 6.0,
        }
        .abs()
    };
    let peak = (0..=STATIONS).map(|i| w(i as f64 / STATIONS as f64)).fold(0.0, f64::max);
    if peak > 0.0 { w(s) / peak } else { 0.0 }
}
