//! 🌉️ EN 1994-2 bridge girders: shear lag of the steel flanges (EN 1993-1-5, 3.2) and the reinforcement stress range for fatigue from the cracked section (EN 1994-2, 6.8).

use super::{ISection, E_A, E_S};

/// 📐️ Shear-lag parameter κ = α₀·b₀/L_e for a flange outstand `b0_m` with the stiffener parameter `alpha0` (1 for a plain flange).
pub fn kappa(b0_m: f64, le_m: f64, alpha0: f64) -> f64 {
    alpha0 * b0_m / le_m.max(1e-9)
}

/// 📐️ Effective-width factor β₁ for sagging: 1 up to κ = 0.02, 1/(1 + 6.4κ²) up to 0.70, 1/(5.9κ) beyond.
pub fn beta_sagging(kappa: f64) -> f64 {
    if kappa <= 0.02 { 1.0 } else if kappa <= 0.70 { 1.0 / (1.0 + 6.4 * kappa * kappa) } else { 1.0 / (5.9 * kappa) }
}

/// 📐️ Effective-width factor β₂ for hogging: 1 up to κ = 0.02, 1/(1 + 6(κ − 1/(2500κ)) + 1.6κ²) up to 0.70, 1/(8.6κ) beyond.
pub fn beta_hogging(kappa: f64) -> f64 {
    if kappa <= 0.02 { 1.0 } else if kappa <= 0.70 { 1.0 / (1.0 + 6.0 * (kappa - 1.0 / (2500.0 * kappa)) + 1.6 * kappa * kappa) } else { 1.0 / (8.6 * kappa) }
}

/// 📐️ Effective-width factor at an end support β₀ = (0.55 + 0.025/κ)·β₁, never above β₁.
pub fn beta_end_support(kappa: f64) -> f64 {
    ((0.55 + 0.025 / kappa.max(1e-9)) * beta_sagging(kappa)).min(beta_sagging(kappa))
}

/// 📐️ I-section with both flanges reduced to the effective width `tw + 2·β·b₀` of the flange outstands `b₀ = (b − t_w)/2`.
pub fn shear_lag_section(sec: &ISection, beta: f64) -> ISection {
    let b0 = ((sec.b - sec.tw) / 2.0).max(0.0);
    let b_eff = sec.tw + 2.0 * beta.clamp(0.0, 1.0) * b0;
    let lost = (sec.b - b_eff).max(0.0) * sec.tf;
    let arm = (sec.h - sec.tf) / 2.0;
    ISection { b: b_eff, a: (sec.area() - 2.0 * lost).max(1e-9), i_y: sec.i_y - 2.0 * lost * arm * arm - 2.0 * (sec.b - b_eff).max(0.0) * sec.tf.powi(3) / 12.0, ..*sec }
}

/// 🌉️ Reinforcement stresses of the hogging zone under fatigue loading (EN 1994-2, 6.8.5.4) on the cracked section: `sigma_0_pa` without and `sigma_max_f_pa` = σ_s,0 + Δσ_s with the tension-stiffening increment, the equivalent range `delta_equ_pa` = cracked range + Δσ_s (the increment is kept in the range, the larger and safe reading).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RebarFatigue {
    pub sigma_0_pa: f64,
    pub sigma_max_f_pa: f64,
    pub delta_cracked_pa: f64,
    pub delta_equ_pa: f64,
}

/// 🌉️ [`RebarFatigue`] of the maximum fatigue moment `m_max_nm` and the moment range `delta_m_nm` on the cracked section with neutral axis `y_na_m` and second moment `i_cracked_m4`, bars at `y_r_m` (all measured from the slab top), and the increment `delta_ts_pa`.
pub fn rebar_fatigue(m_max_nm: f64, delta_m_nm: f64, y_na_m: f64, y_r_m: f64, i_cracked_m4: f64, delta_ts_pa: f64) -> RebarFatigue {
    let per_nm = E_S / E_A * (y_na_m - y_r_m).abs() / i_cracked_m4.max(1e-12);
    let (sigma_0_pa, delta_cracked_pa, ts) = (m_max_nm.abs() * per_nm, delta_m_nm.abs() * per_nm, delta_ts_pa.max(0.0));
    RebarFatigue { sigma_0_pa, sigma_max_f_pa: sigma_0_pa + ts, delta_cracked_pa, delta_equ_pa: delta_cracked_pa + ts }
}

/// 🔩 C-54 minimum edge distance [m] of a headed stud from the flange edge (EN 1994-2 6.6.5, recalled value, unverified `ndp.en1994.2.stud-edge-distance`).
pub const STUD_EDGE_DISTANCE_MIN_M: f64 = 0.02;

/// 🔩 C-54 minimum transverse centre-to-centre stud spacing [m]: 2.5·d, 4·d in a solid slab (unverified `ndp.en1994.2.stud-cross-spacing`).
pub fn stud_cross_spacing_min_m(d_m: f64, solid_slab: bool) -> f64 {
    (if solid_slab { 4.0 } else { 2.5 }) * d_m
}

/// 🕸️ C-19/C-54 minimum transverse reinforcement [m²/m] of a slab of concrete height `hc_m`: 0.002·h_c (EN 1994-1-1 6.6.6.3, unverified `ndp.en1994.2.min-transverse-reinforcement`).
pub fn min_transverse_reinforcement_m2_per_m(hc_m: f64) -> f64 {
    0.002 * hc_m
}

/// 🧱 Partially loaded area (EN 1992-1-1 6.7, Eq. 6.63): F_Rdu = A_c0·f_cd·√(A_c1/A_c0) ≤ 3·f_cd·A_c0 [N]. The distribution area A_c1 grows 3 times the loaded plate `w_m × h_m`, limited to the slab `slab_b_m × slab_h_m`.
pub fn end_plate_bearing_n(fcd_pa: f64, w_m: f64, h_m: f64, slab_b_m: f64, slab_h_m: f64) -> f64 {
    let a0 = (w_m * h_m).max(1e-12);
    let a1 = ((3.0 * w_m).min(slab_b_m).max(w_m)) * ((3.0 * h_m).min(slab_h_m).max(h_m));
    (a0 * fcd_pa * (a1 / a0).sqrt()).min(3.0 * fcd_pa * a0)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
