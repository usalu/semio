//! 🧩️ EN 1993-1-5 stiffened plates, shear lag with plate buckling, patch loading types a to c, interaction and stiffener rules (mm, N, MPa). See [EN 1993-1-5](https://eurocodes.jrc.ec.europa.eu/).

use crate::stability::chi_general;
use std::f64::consts::PI;

/// 🧩️ Reference stress σ_E = π² E t²/(12 (1 − ν²) b²).
pub fn sigma_e(e: f64, t: f64, b: f64) -> f64 {
    PI * PI * e * t * t / (12.0 * (1.0 - 0.09) * b * b)
}

/// 🧩️ Column-like imperfection α_e = α + 0.09/(i/e) (§4.5.3).
pub fn alpha_e(alpha: f64, i: f64, e: f64) -> f64 {
    alpha + 0.09 / (i / e)
}

/// 🧩️ Column buckling reduction χ_c of the stiffened panel for slenderness λ̄_c = √(β_A,c f_y/σ_cr,c).
pub fn chi_column_stiffened(lambda_c: f64, alpha_e: f64) -> f64 {
    if lambda_c <= 0.2 { 1.0 } else { chi_general(lambda_c, alpha_e, 0.2, 1.0) }
}

/// 🧩️ Interpolated reduction ρ_c = (ρ − χ_c) ξ (2 − ξ) + χ_c with ξ = σ_cr,p/σ_cr,c − 1 in [0, 1] (Eq. 4.13).
pub fn rho_c_interpolated(rho: f64, chi_c: f64, sigma_cr_p: f64, sigma_cr_c: f64) -> f64 {
    let xi = (sigma_cr_p / sigma_cr_c - 1.0).clamp(0.0, 1.0);
    (rho - chi_c) * xi * (2.0 - xi) + chi_c
}

/// 🧩️ Shear-lag factor β in hogging (Table 3.1).
pub fn shear_lag_beta_hogging(kappa: f64) -> f64 {
    if kappa <= 0.02 { 1.0 } else if kappa <= 0.7 { 1.0 / (1.0 + 6.0 * (kappa - 1.0 / (2500.0 * kappa)) + 1.6 * kappa * kappa) } else { 1.0 / (8.6 * kappa) }
}

/// 🧩️ κ = α0 b0/L_e with α0 = √(1 + A_sl/(b0 t)).
pub fn kappa_shear_lag(b0: f64, le: f64, a_sl: f64, t: f64) -> f64 {
    (1.0 + a_sl / (b0 * t)).sqrt() * b0 / le
}

/// 🧩️ End-support factor β0 = (0.55 + 0.025/κ) β1 ≤ β1.
pub fn shear_lag_beta_end(kappa: f64, beta1: f64) -> f64 {
    ((0.55 + 0.025 / kappa) * beta1).min(beta1)
}

/// 🧩️ Elastic-plastic ULS factor β_ult = β^κ combined with plate buckling A_eff = A_c,eff,loc β_ult (§3.3).
pub fn a_eff_shear_lag(a_c_eff_loc: f64, beta: f64, kappa: f64) -> f64 {
    a_c_eff_loc * beta.powf(kappa)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PatchType {
    Through,
    OneFlangeNearEnd,
    EndFlange,
}

/// 🧩️ Buckling coefficient k_F of the three load introduction types (Figure 6.1) with a/hw, s_s and c.
pub fn k_f(kind: PatchType, hw: f64, a: f64, ss: f64, c: f64) -> f64 {
    match kind {
        PatchType::Through => 6.0 + 2.0 * (hw / a).powi(2),
        PatchType::OneFlangeNearEnd => 3.5 + 2.0 * (hw / a).powi(2),
        PatchType::EndFlange => (2.0 + 6.0 * (ss + c) / hw).min(6.0),
    }
}

/// 🧩️ k_F extended by a longitudinal web stiffener, 6 + 2 (hw/a)² + (5.44 b1/a − 0.21) √γs with γs = min(10.9 Isl1/(hw tw³), 13 (a/hw)³ + 210 (0.3 − b1/a)).
pub fn k_f_longitudinal(hw: f64, a: f64, tw: f64, b1: f64, i_sl1: f64) -> f64 {
    let gs = (10.9 * i_sl1 / (hw * tw.powi(3))).min(13.0 * (a / hw).powi(3) + 210.0 * (0.3 - b1 / a));
    6.0 + 2.0 * (hw / a).powi(2) + (5.44 * b1 / a - 0.21) * gs.max(0.0).sqrt()
}

pub struct Patch {
    pub k_f: f64,
    pub f_cr: f64,
    pub l_y: f64,
    pub lambda_f: f64,
    pub chi_f: f64,
    pub f_rd: f64,
}

/// 🧩️ Resistance to transverse forces for load types a, b and c (§6.2 to §6.5) with effective loaded length l_y.
pub fn patch_resistance(kind: PatchType, hw: f64, tw: f64, a: f64, tf: f64, bf: f64, fyw: f64, fyf: f64, ss: f64, c: f64, gm1: f64) -> Patch {
    let kf = k_f(kind, hw, a, ss, c);
    let e = crate::stability::e_steel();
    let f_cr = 0.9 * kf * e * tw.powi(3) / hw;
    let m1 = fyf * bf / (fyw * tw);
    let l_e = (kf * e * tw * tw / (2.0 * fyw * hw)).min(ss + c);
    let m2_for = |ly: f64| {
        let lam = (ly * tw * fyw / f_cr).sqrt();
        if lam > 0.5 { 0.02 * (hw / tf).powi(2) } else { 0.0 }
    };
    let l_y = match kind {
        PatchType::Through => {
            let m2 = m2_for(ss + 2.0 * tf * (1.0 + m1.sqrt()));
            (ss + 2.0 * tf * (1.0 + (m1 + m2).sqrt())).min(a)
        }
        PatchType::OneFlangeNearEnd | PatchType::EndFlange => {
            let m2 = m2_for(l_e + tf * (m1 / 2.0 + (l_e / tf).powi(2)).sqrt());
            let v1 = l_e + tf * (m1 / 2.0 + (l_e / tf).powi(2) + m2).sqrt();
            let v2 = l_e + tf * (m1 + m2).sqrt();
            let v = v1.min(v2);
            if kind == PatchType::EndFlange { v.min(ss + c) } else { v }
        }
    };
    let lambda_f = (l_y * tw * fyw / f_cr).sqrt();
    let chi_f = (0.5 / lambda_f).min(1.0);
    Patch { k_f: kf, f_cr, l_y, lambda_f, chi_f, f_rd: fyw * chi_f * l_y * tw / gm1 }
}

/// 🧩️ Interaction of transverse force and bending, (η2 + 0.8 η1)/1.4 (Eq. 7.2).
pub fn patch_bending_utilisation(f_ed: f64, f_rd: f64, m_ed: f64, mc_rd: f64) -> f64 {
    (f_ed / f_rd + 0.8 * m_ed / mc_rd) / 1.4
}

/// 🧩️ Shear coefficient k_τ with longitudinal stiffeners, base value plus k_τ,sl (Annex A.3).
pub fn k_tau_stiffened(a: f64, hw: f64, t: f64, i_sl: f64) -> f64 {
    let base = if a / hw >= 1.0 { 5.34 + 4.0 * (hw / a).powi(2) } else { 4.0 + 5.34 * (hw / a).powi(2) };
    let sl = (9.0 * (hw / a).powi(2) * (i_sl / (t.powi(3) * hw)).powf(0.75)).max(2.1 / t * (i_sl / hw).cbrt());
    base + sl
}

/// 🧩️ Distance `c` of the flange hinges, a (0.25 + 1.6 b_f t_f² f_yf/(t h_w² f_yw)) (Eq. 5.8).
pub fn flange_hinge_distance(a: f64, bf: f64, tf: f64, fyf: f64, t: f64, hw: f64, fyw: f64) -> f64 {
    a * (0.25 + 1.6 * bf * tf * tf * fyf / (t * hw * hw * fyw))
}

/// 🧩️ Shear buckling resistance of a web panel with its three terms (all N, k_τ dimensionless).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PanelShear {
    pub k_tau: f64,
    pub lambda_w: f64,
    pub chi_w: f64,
    pub v_bw_rd: f64,
    pub v_bf_rd: f64,
    pub v_cap: f64,
    pub v_b_rd: f64,
}

/// 🧩️ V_b,Rd = V_bw,Rd + V_bf,Rd ≤ η f_yw h_w t/(√3 γ_M1) (EN 1993-1-5 5.2, Eq. 5.1) of a web `hw` × `t` between transverse stiffeners at distance `a` with the longitudinal stiffener inertia `i_sl` (Annex A.3 k_τ); `flange` = `(b_f, t_f, f_yf, M_f,Rd, M_Ed)` adds the flange term of 5.4 (non-rigid end post, χ_w of Table 5.1).
pub fn panel_shear(a: f64, hw: f64, t: f64, fyw: f64, i_sl: f64, flange: Option<(f64, f64, f64, f64, f64)>, gm1: f64) -> PanelShear {
    let eps = (235.0 / fyw).sqrt();
    let eta = crate::resistance::eta_shear(fyw);
    let k_tau = k_tau_stiffened(a, hw, t, i_sl);
    let lambda_w = crate::resistance::lambda_w(hw, t, eps, k_tau);
    let chi_w = crate::resistance::chi_w(lambda_w, eta, false);
    let v_bw_rd = crate::resistance::v_bw_rd(chi_w, fyw, hw, t, gm1);
    let v_bf_rd = flange.filter(|(bf, tf, _, m_f, _)| *bf > 0.0 && *tf > 0.0 && *m_f > 0.0).map_or(0.0, |(bf, tf, fyf, m_f, m_ed)| crate::resistance::v_bf_rd(bf, tf, fyf, flange_hinge_distance(a, bf, tf, fyf, t, hw, fyw), m_f, m_ed, gm1));
    let v_cap = crate::resistance::v_b_rd_cap(eta, fyw, hw, t, gm1);
    PanelShear { k_tau, lambda_w, chi_w, v_bw_rd, v_bf_rd, v_cap, v_b_rd: (v_bw_rd + v_bf_rd).min(v_cap) }
}

/// 🧩️ Plated M-V interaction, η1 + (1 − M_f,Rd/M_pl,Rd)(2 η3 − 1)² as utilisation (applies for η3 > 0.5 and M_Ed ≥ M_f,Rd, otherwise η1; Eq. 7.1).
pub fn mv_interaction(m_ed: f64, m_pl_rd: f64, m_f_rd: f64, v_ed: f64, v_bw_rd: f64) -> f64 {
    let (eta1, eta3) = (m_ed / m_pl_rd, v_ed / v_bw_rd);
    if eta3 <= 0.5 || m_ed < m_f_rd { eta1.max(eta3) } else { eta1 + (1.0 - m_f_rd / m_pl_rd) * (2.0 * eta3 - 1.0).powi(2) }
}

/// 🧩️ Axial force reductions M_f,N,Rd = M_f,Rd (1 − N/((A_f1 + A_f2) f_yf/γM0)) and M_pl,N,Rd = M_pl,Rd (1 − N/(A f_y/γM0)).
pub fn m_reduced_by_axial(m_rd: f64, n_ed: f64, a_resisting: f64, fy: f64, gm0: f64) -> f64 {
    m_rd * (1.0 - n_ed / (a_resisting * fy / gm0))
}

/// 🧩️ Minimum second moment of area of transverse stiffeners (§9.3.3).
pub fn i_st_min(a: f64, hw: f64, tw: f64) -> f64 {
    if a / hw < 2.0_f64.sqrt() { 1.5 * hw.powi(3) * tw.powi(3) / (a * a) } else { 0.75 * hw * tw.powi(3) }
}

/// 🧩️ Torsional stiffener criterion I_T/I_p ≥ 5.3 f_y/E (§9.2.1(8)) as utilisation.
pub fn stiffener_torsion_utilisation(i_t: f64, i_p: f64, fy: f64, e: f64) -> f64 {
    5.3 * fy / e / (i_t / i_p)
}

/// 🧩️ Flange-induced buckling h_w/t_w ≤ k (E/f_yf) √(A_w/A_fc) as utilisation (§8), k = 0.3 plastic rotation, 0.4 plastic moment, 0.55 elastic.
pub fn flange_induced_buckling_utilisation(hw: f64, tw: f64, k: f64, e: f64, fyf: f64, aw: f64, afc: f64) -> f64 {
    (hw / tw) / (k * e / fyf * (aw / afc).sqrt())
}

/// 🧩️ Load amplifier α_ult,k from 1/α² = (σx/fy)² + (σz/fy)² − σxσz/fy² + 3 (τ/fy)² (§10).
pub fn alpha_ult_k(sx: f64, sz: f64, tau: f64, fy: f64) -> f64 {
    1.0 / ((sx / fy).powi(2) + (sz / fy).powi(2) - sx * sz / (fy * fy) + 3.0 * (tau / fy).powi(2)).sqrt()
}

/// 🧩️ Plate slenderness λ̄_p = √(α_ult,k/α_cr) and ρ for the governing buckling mode (Eq. 4.2 with ψ = 1).
pub fn reduced_stress_rho(alpha_ult: f64, alpha_cr: f64) -> (f64, f64) {
    let lam = (alpha_ult / alpha_cr).sqrt();
    (lam, crate::effective_width::rho_internal(lam, 1.0))
}

/// 🧩️ Simplified reduced stress verification, utilisation γM1/(ρ α_ult,k) (Eq. 10.6 with ρ = min(ρx, ρz, χw)).
pub fn reduced_stress_simple(alpha_ult: f64, rho: f64, gm1: f64) -> f64 {
    gm1 / (rho * alpha_ult)
}
