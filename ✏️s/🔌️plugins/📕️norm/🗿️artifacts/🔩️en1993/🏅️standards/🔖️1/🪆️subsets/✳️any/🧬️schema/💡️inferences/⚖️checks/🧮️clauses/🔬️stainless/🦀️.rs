//! 🔬️ EN 1993-1-4 stainless steel: plate reduction, member buckling, interaction factors, fire reduction, secant modulus (mm, N, MPa). See [EN 1993-1-4](https://eurocodes.jrc.ec.europa.eu/).

use crate::stability::chi_general;

/// 🔬️ Larger root of x² − a x + b = 0, the slenderness where ρ = a/λ − b/λ² reaches 1.
fn rho_threshold(a: f64, b: f64) -> f64 {
    0.5 * (a + (a * a - 4.0 * b).sqrt())
}

/// 🔬️ ρ of an internal stainless element: cold-formed 0.772/λ − 0.125/λ², welded 0.772/λ − 0.079/λ² (Table 5.3).
pub fn rho_internal_stainless(lambda_p: f64, welded: bool) -> f64 {
    let b = semio_s_plugin_norm_compliance::ndp_common(if welded { "ndp.en1993.1-4.rho-internal.b.welded" } else { "ndp.en1993.1-4.rho-internal.b.cold" });
    let a = semio_s_plugin_norm_compliance::ndp_common("ndp.en1993.1-4.rho-internal.a");
    if lambda_p <= rho_threshold(a, b) { 1.0 } else { (a / lambda_p - b / (lambda_p * lambda_p)).min(1.0) }
}

/// 🔬️ ρ of an outstand stainless element: cold-formed 1/λ − 0.231/λ², welded 1/λ − 0.242/λ² (Table 5.3).
pub fn rho_outstand_stainless(lambda_p: f64, welded: bool) -> f64 {
    let b = semio_s_plugin_norm_compliance::ndp_common(if welded { "ndp.en1993.1-4.rho-outstand.b.welded" } else { "ndp.en1993.1-4.rho-outstand.b.cold" });
    let a = semio_s_plugin_norm_compliance::ndp_common("ndp.en1993.1-4.rho-outstand.a");
    if lambda_p <= rho_threshold(a, b) { 1.0 } else { (a / lambda_p - b / (lambda_p * lambda_p)).min(1.0) }
}

/// 🔬️ Cross-section resistances N_c,Rd = A fy/γM0 and M_c,Rd = W fy/γM0 (class 1 to 3 with W_pl or W_el, class 4 with W_eff).
pub fn section_rd(a_or_w: f64, fy: f64, gm0: f64) -> f64 {
    a_or_w * fy / gm0
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StainlessMember {
    ColdFormedOpen,
    WeldedOpen,
    Hollow,
}

fn pair(table: &str, kind: &str) -> (f64, f64) {
    (semio_s_plugin_norm_compliance::ndp_common(&format!("ndp.en1993.1-4.{table}.{kind}.alpha")), semio_s_plugin_norm_compliance::ndp_common(&format!("ndp.en1993.1-4.{table}.{kind}.lambda0")))
}

/// 🔬️ (α, λ̄0) for flexural buckling (Table 5.4), read from `ndp.en1993.1-4.flexural.<welded-open|other>`.
pub fn flexural_params(kind: StainlessMember) -> (f64, f64) {
    pair("flexural", if kind == StainlessMember::WeldedOpen { "welded-open" } else { "other" })
}

/// 🔬️ (α_LT, λ̄LT,0) for lateral-torsional buckling, read from `ndp.en1993.1-4.ltb.<welded-open|other>`.
pub fn ltb_params(kind: StainlessMember) -> (f64, f64) {
    pair("ltb", if kind == StainlessMember::WeldedOpen { "welded-open" } else { "other" })
}

/// 🔬️ (α, λ̄0) for torsional and torsional-flexural buckling, read from `ndp.en1993.1-4.torsional.all`.
pub fn torsional_params() -> (f64, f64) {
    pair("torsional", "all")
}

pub fn chi_flexural_stainless(lambda: f64, kind: StainlessMember) -> f64 {
    let (a, l0) = flexural_params(kind);
    if lambda <= l0 { 1.0 } else { chi_general(lambda, a, l0, 1.0) }
}

pub fn chi_lt_stainless(lambda_lt: f64, kind: StainlessMember) -> f64 {
    let (a, l0) = ltb_params(kind);
    if lambda_lt <= l0 { 1.0 } else { chi_general(lambda_lt, a, l0, 1.0) }
}

pub fn chi_torsional_stainless(lambda_t: f64) -> f64 {
    let (a, l0) = torsional_params();
    if lambda_t <= l0 { 1.0 } else { chi_general(lambda_t, a, l0, 1.0) }
}

/// 🔬️ Interaction factor k = 1 + 2 (λ̄ − 0.5) n, bounded by 1.2 ≤ k ≤ 1.2 + 2n with n = N_Ed/(χ N_Rk/γM1).
pub fn k_interaction(lambda: f64, n: f64) -> f64 {
    (1.0 + 2.0 * (lambda - 0.5) * n).clamp(1.2, 1.2 + 2.0 * n)
}

/// 🔬️ Interaction factor for lateral-torsional buckling, k_LT = 1 − 0.1 λ̄z n_z/(C_mLT − 0.25) ≥ 1 − 0.1 n_z/(C_mLT − 0.25).
pub fn k_lt_interaction(lambda_z: f64, n_z: f64, cm_lt: f64) -> f64 {
    (1.0 - 0.1 * lambda_z * n_z / (cm_lt - 0.25)).max(1.0 - 0.1 * n_z / (cm_lt - 0.25))
}

/// 🔬️ Member interaction N/(χ_y N_Rk/γ) + k_y M_y/(χ_LT M_y,Rk/γ) + k_z M_z/(M_z,Rk/γ).
pub fn interaction_stainless(n: f64, chi_y: f64, n_rk: f64, ky: f64, my: f64, chi_lt: f64, my_rk: f64, kz: f64, mz: f64, mz_rk: f64, gm1: f64) -> f64 {
    n / (chi_y * n_rk / gm1) + ky * my / (chi_lt * my_rk / gm1) + kz * mz / (mz_rk / gm1)
}

/// 🔬️ Secant modulus E_s = σ/ε of the Ramberg-Osgood law.
pub fn secant_modulus(sigma: f64, e: f64, fy: f64, n: f64) -> f64 {
    e / (1.0 + 0.002 * e / sigma * (sigma / fy).powf(n))
}

/// 🔬️ Deflection scaling δ_s = δ_E E/E_s of a member under stress σ.
pub fn deflection_secant_factor(sigma: f64, e: f64, fy: f64, n: f64) -> f64 {
    e / secant_modulus(sigma, e, fy, n)
}

/// 🔬️ Steel temperatures [°C] of the stainless fire reduction table (EN 1993-1-2 Table C.1, `ndp.en1993.1-2.stainless-fire.1-4301.<k02p|ke|ku>.t<θ>`, unverified, copied from memory).
pub const FIRE_TEMPERATURES: [f64; 13] = [20.0, 100.0, 200.0, 300.0, 400.0, 500.0, 600.0, 700.0, 800.0, 900.0, 1000.0, 1100.0, 1200.0];

/// 🔬️ Linear interpolation of the stainless fire reduction factors (k_0.2p,θ, k_E,θ, k_u,θ) at steel temperature θ, read from the registry.
pub fn stainless_fire_factors(grade: &str, theta: f64) -> Option<(f64, f64, f64)> {
    if grade != "1.4301" {
        return None;
    }
    let axis: Vec<(String, f64)> = FIRE_TEMPERATURES.iter().map(|t| (format!("t{}", *t as i64), *t)).collect();
    let axis: Vec<(&str, f64)> = axis.iter().map(|(label, t)| (label.as_str(), *t)).collect();
    let read = |column: &str| semio_s_plugin_norm_compliance::ndp_interpolate("ndp.en1993.1-2.stainless-fire", &["1-4301", column], &axis, theta, semio_s_plugin_norm_compliance::AnnexChoice::En);
    Some((read("k02p"), read("ke"), read("ku")))
}
