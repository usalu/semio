//! 📏️ Crack width of the hogging zone: the cracked-section reinforcement stress and the tension-stiffening increment of EN 1994-1-1, 7.4.3, and the crack width w_k = s_r,max·(ε_sm − ε_cm) of EN 1992-1-1, 7.3.4 that cl. 7.4.1 refers to. Unverified (recommended values of the EN 1992-1-1 print, not read from it): k_t = 0.4, k_1 = 0.8, k_2 = 0.5, k_3 = 3.4, k_4 = 0.425, the 5(c + φ/2) spacing limit with s_r,max = 1.3·h_c beyond it, and h_c,eff = min(2.5·d_r, h_c/2) with the bar level `d_r` of the slab top.

use super::constants::HOGGING_REBAR_DEPTH_M;
use super::{cracked, delta_sigma_ts, fctm_pa, ISection, E_A, E_S};
use std::f64::consts::PI;

const K_T: f64 = 0.4;
const K_1: f64 = 0.8;
const K_2: f64 = 0.5;
const K_3: f64 = 3.4;
const K_4: f64 = 0.425;

/// 📐️ Cracked-section state of a hogging zone: the bar stress per unit hogging moment and the tension-stiffening increment Δσ_s.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hogging {
    pub sigma_per_nm: f64,
    pub delta_sigma_pa: f64,
}

/// 📐️ The cracked state of the steel section `sec` under a slab of thickness `ys`, concrete height `hc` and effective width `beff` with `as_per_m` bars per metre (EN 1994-1-1, 7.4.3: σ_s,0 = M·(y − d_r)/I_cracked and Δσ_s = 0.4·f_ctm/(α_st·ρ_s)).
pub fn hogging(sec: &ISection, ys: f64, hc: f64, beff: f64, as_per_m: f64, fck_pa: f64) -> Hogging {
    let as_total = as_per_m * beff;
    let cr = cracked(sec, ys, as_total, HOGGING_REBAR_DEPTH_M);
    let act = (beff * hc).max(1e-9);
    Hogging {
        sigma_per_nm: E_S / E_A * (cr.y_na - HOGGING_REBAR_DEPTH_M) / cr.i,
        delta_sigma_pa: delta_sigma_ts(fctm_pa(fck_pa), cr.area, cr.i, sec.area(), sec.i_y, as_total / act),
    }
}

/// 📐️ Bar diameter φ [m] of bars at `spacing_m` that provide `as_per_m` per metre.
pub fn bar_diameter_m(as_per_m: f64, spacing_m: f64) -> f64 {
    (4.0 * as_per_m * spacing_m / PI).max(0.0).sqrt()
}

/// 📐️ The crack width w_k [m] at the bar stress `sigma_s_pa`: ρ_p,eff = A_s/h_c,eff per metre, s_r,max = k_3·c + k_1·k_2·k_4·φ/ρ_p,eff (1.3·h_c for bars wider apart than 5(c + φ/2)), ε_sm − ε_cm = max((σ_s − k_t·f_ct,eff·(1 + α_e·ρ_p,eff)/ρ_p,eff)/E_s, 0.6·σ_s/E_s).
pub fn crack_width_m(sigma_s_pa: f64, as_per_m: f64, spacing_m: f64, hc: f64, fck_pa: f64, ecm_pa: f64) -> f64 {
    let phi = bar_diameter_m(as_per_m, spacing_m);
    let cover = (HOGGING_REBAR_DEPTH_M - phi / 2.0).max(0.0);
    let h_eff = (2.5 * HOGGING_REBAR_DEPTH_M).min(hc / 2.0).max(1e-6);
    let rho = (as_per_m / h_eff).max(1e-9);
    let sigma = sigma_s_pa.max(0.0);
    let eps = ((sigma - K_T * fctm_pa(fck_pa) / rho * (1.0 + E_S / ecm_pa * rho)) / E_S).max(0.6 * sigma / E_S);
    let s_r = if spacing_m <= 5.0 * (cover + phi / 2.0) { K_3 * cover + K_1 * K_2 * K_4 * phi / rho } else { 1.3 * hc };
    s_r * eps
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
