//! 🧮️ EN 1993-1-1 §6.2 cross-section resistances and interactions (mm, N, MPa, Nmm). See [EN 1993-1-1 §6.2](https://eurocodes.jrc.ec.europa.eu/).

use std::f64::consts::PI;

pub fn n_pl_rd(a: f64, fy: f64, gm0: f64) -> f64 {
    a * fy / gm0
}

pub fn n_u_rd(a_net: f64, fu: f64, gm2: f64) -> f64 {
    0.9 * a_net * fu / gm2
}

pub fn n_t_rd(a: f64, a_net: f64, fy: f64, fu: f64, gm0: f64, gm2: f64) -> f64 {
    n_pl_rd(a, fy, gm0).min(n_u_rd(a_net, fu, gm2))
}

/// 🕳️ Net area with staggered holes Anet = t (b − n d0 + Σ s²/(4p)).
pub fn a_net_staggered(b: f64, t: f64, n_holes: f64, d0: f64, stagger: &[(f64, f64)]) -> f64 {
    t * (b - n_holes * d0 + stagger.iter().map(|(s, p)| s * s / (4.0 * p)).sum::<f64>())
}

pub fn eta_shear(fy: f64) -> f64 {
    if fy <= 460.0 { 1.2 } else { 1.0 }
}

/// ✂️ A_v of a rolled I/H for load parallel to the web (§6.2.6(3)a).
pub fn av_rolled_i(a: f64, b: f64, tf: f64, tw: f64, r: f64, hw: f64, eta: f64) -> f64 {
    (a - 2.0 * b * tf + (tw + 2.0 * r) * tf).max(eta * hw * tw)
}

pub fn av_rolled_u(a: f64, b: f64, tf: f64, tw: f64, r: f64) -> f64 {
    a - 2.0 * b * tf + (tw + r) * tf
}

pub fn av_t(a: f64, b: f64, tf: f64) -> f64 {
    0.9 * (a - b * tf)
}

pub fn av_welded_web(hw: f64, tw: f64, eta: f64) -> f64 {
    eta * hw * tw
}

pub fn av_welded_flanges(a: f64, hw: f64, tw: f64) -> f64 {
    a - hw * tw
}

pub fn av_rhs(a: f64, b: f64, h: f64, parallel_to_depth: bool) -> f64 {
    if parallel_to_depth { a * h / (b + h) } else { a * b / (b + h) }
}

pub fn av_chs(a: f64) -> f64 {
    2.0 * a / PI
}

pub fn v_pl_rd(av: f64, fy: f64, gm0: f64) -> f64 {
    av * fy / 3.0_f64.sqrt() / gm0
}

/// ✂️ ρ = (2 V_Ed/V_Rd − 1)² when V_Ed > 0.5 V_Rd, else 0 (§6.2.8).
pub fn shear_rho(v_ed: f64, v_rd: f64) -> f64 {
    if v_ed <= 0.5 * v_rd { 0.0 } else { (2.0 * v_ed / v_rd - 1.0).powi(2) }
}

/// ✂️ M_y,V,Rd of a doubly symmetric I (§6.2.8(5)), capped at M_y,c,Rd.
pub fn m_y_v_rd_i(w_pl: f64, rho: f64, aw: f64, tw: f64, fy: f64, gm0: f64) -> f64 {
    ((w_pl - rho * aw * aw / (4.0 * tw)) * fy / gm0).min(w_pl * fy / gm0)
}

/// ✂️ V_pl,T,Rd reductions for torsion (§6.2.7(9)). `kind`: 0 = I/H, 1 = U, 2 = hollow.
pub fn v_pl_t_rd(kind: u8, v_pl_rd: f64, tau_t: f64, tau_w: f64, fy: f64, gm0: f64) -> f64 {
    let f = fy / 3.0_f64.sqrt() / gm0;
    match kind {
        0 => (1.0 - tau_t / (1.25 * f)).max(0.0).sqrt() * v_pl_rd,
        1 => ((1.0 - tau_t / (1.25 * f)).max(0.0).sqrt() - tau_w / f) * v_pl_rd,
        _ => (1.0 - tau_t / f) * v_pl_rd,
    }
}

pub fn neglect_axial_effect_y(n_ed: f64, n_pl_rd: f64, hw: f64, tw: f64, fy: f64, gm0: f64) -> bool {
    n_ed <= 0.25 * n_pl_rd && n_ed <= 0.5 * hw * tw * fy / gm0
}

/// 🔗️ M_N,y,Rd of a doubly symmetric I/H (Eq. 6.36).
pub fn m_n_y_rd_i(m_pl_y: f64, n: f64, a_web: f64) -> f64 {
    (m_pl_y * (1.0 - n) / (1.0 - 0.5 * a_web)).min(m_pl_y)
}

pub fn m_n_z_rd_i(m_pl_z: f64, n: f64, a_web: f64) -> f64 {
    if n <= a_web { m_pl_z } else { m_pl_z * (1.0 - ((n - a_web) / (1.0 - a_web)).powi(2)) }
}

pub fn m_n_rd_solid_rect(m_pl: f64, n: f64) -> f64 {
    m_pl * (1.0 - n * n)
}

/// 🔗️ M_N,Rd of a CHS, M_pl(1 − n^1.7) (ECCS form, flagged unverified).
pub fn m_n_rd_chs(m_pl: f64, n: f64) -> f64 {
    m_pl * (1.0 - n.powf(1.7))
}

/// 🔗️ Web area fraction a_w = min((A − 2 b t)/A, 0.5) of a hollow section (Eq. 6.39), `b` and `t` the width and wall thickness.
pub fn a_w_hollow(a: f64, b: f64, t: f64) -> f64 {
    ((a - 2.0 * b * t) / a).min(0.5)
}

/// ✂️ Shear stress τ_Ed = V_Ed/A_w of an I or H section web (Eq. 6.21 simplified), valid for A_f/A_w ≥ 0.6 where the flanges carry the share of the shear that the web area cancels out; `a_f` is returned unused by design.
pub fn tau_ed_i(v_ed: f64, a_w: f64, a_f: f64) -> f64 {
    assert!(a_f >= 0.6 * a_w, "the simplified τ_Ed = V_Ed/A_w needs A_f/A_w >= 0.6");
    v_ed / a_w
}

/// 🔗️ M_N,Rd of a welded box/RHS about the axis whose flange area fraction is `a_f` (a_w about y, a_f about z).
pub fn m_n_rd_rhs(m_pl: f64, n: f64, a_fraction: f64) -> f64 {
    (m_pl * (1.0 - n) / (1.0 - 0.5 * a_fraction)).min(m_pl)
}

/// 🔗️ Exponents (α, β) of Eq. 6.41. `kind`: 0 = I/H, 1 = CHS, 2 = RHS, 3 = other.
pub fn biaxial_exponents(kind: u8, n: f64) -> (f64, f64) {
    match kind {
        0 => (2.0, (5.0 * n).max(1.0)),
        1 => (2.0, 2.0),
        2 => {
            let e = (1.66 / (1.0 - 1.13 * n * n)).min(6.0);
            (e, e)
        }
        _ => (1.0, 1.0),
    }
}

pub fn biaxial_utilisation(my_ed: f64, mz_ed: f64, m_n_y: f64, m_n_z: f64, alpha: f64, beta: f64) -> f64 {
    (my_ed.abs() / m_n_y).powf(alpha) + (mz_ed.abs() / m_n_z).powf(beta)
}

/// 📐️ Class 3 maximum fibre stress ratio σ_x,Ed/(fy/γM0).
pub fn class3_utilisation(n_ed: f64, a: f64, my_ed: f64, w_el_y: f64, mz_ed: f64, w_el_z: f64, fy: f64, gm0: f64) -> f64 {
    (n_ed / a + my_ed / w_el_y + mz_ed / w_el_z).abs() / (fy / gm0)
}

/// 📐️ Class 4 interaction (Eq. 6.44).
pub fn class4_utilisation(n_ed: f64, a_eff: f64, my_ed: f64, e_ny: f64, w_eff_y: f64, mz_ed: f64, e_nz: f64, w_eff_z: f64, fy: f64, gm0: f64) -> f64 {
    let f = fy / gm0;
    (n_ed / (a_eff * f)).abs() + ((my_ed + n_ed * e_ny) / (w_eff_y * f)).abs() + ((mz_ed + n_ed * e_nz) / (w_eff_z * f)).abs()
}

/// 📐️ Von Mises criterion of §6.2.1(5) as a utilisation (squared form, Eq. 6.1).
pub fn von_mises_utilisation(sx: f64, sz: f64, tau: f64, fy: f64, gm0: f64) -> f64 {
    let f = fy / gm0;
    (sx / f).powi(2) + (sz / f).powi(2) - (sx / f) * (sz / f) + 3.0 * (tau / f).powi(2)
}

/// 🌀 St. Venant shear stress of an open thin-walled part, τ = T_t t/I_t.
pub fn tau_t_open(t_t: f64, t: f64, it: f64) -> f64 {
    t_t * t / it
}

/// 🌀 Bredt shear stress of a closed section, τ = T/(2 A_k t).
pub fn tau_t_closed(t_t: f64, a_k: f64, t: f64) -> f64 {
    t_t / (2.0 * a_k * t)
}

pub struct TorsionResult {
    pub theta_max: f64,
    pub t_t_max: f64,
    pub t_w_max: f64,
    pub b_max: f64,
}

/// 🌀 Fork-supported warping torsion by Fourier series: point torque `mt` at `xa` (`udl` false) or uniform torque per length `mt` (`udl` true).
pub fn warping_torsion_fork(l: f64, ei_w: f64, g_it: f64, mt: f64, xa: f64, udl: bool) -> TorsionResult {
    let n = 1999;
    let mut res = TorsionResult { theta_max: 0.0, t_t_max: 0.0, t_w_max: 0.0, b_max: 0.0 };
    let xs: Vec<f64> = (0..=200).map(|i| l * i as f64 / 200.0).collect();
    for &x in &xs {
        let (mut th, mut tt, mut tw, mut bi) = (0.0, 0.0, 0.0, 0.0);
        for k in (1..=n).step_by(1) {
            let kk = k as f64 * PI / l;
            let mk = if udl { 2.0 * mt * (1.0 - (k as f64 * PI).cos()) / (k as f64 * PI) } else { 2.0 / l * mt * (kk * xa).sin() };
            let a = mk / (ei_w * kk.powi(4) + g_it * kk * kk);
            th += a * (kk * x).sin();
            tt += g_it * a * kk * (kk * x).cos();
            tw += ei_w * a * kk.powi(3) * (kk * x).cos();
            bi += ei_w * a * kk * kk * (kk * x).sin();
        }
        res.theta_max = res.theta_max.max(th.abs());
        res.t_t_max = res.t_t_max.max(tt.abs());
        res.t_w_max = res.t_w_max.max(tw.abs());
        res.b_max = res.b_max.max(bi.abs());
    }
    res
}

/// 🛡️ k_τ of an unstiffened-to-stiffened web panel (EN 1993-1-5 Annex A.3) with panel aspect a/hw.
pub fn k_tau(a_over_hw: f64) -> f64 {
    if a_over_hw >= 1.0 { 5.34 + 4.0 / (a_over_hw * a_over_hw) } else { 4.0 + 5.34 / (a_over_hw * a_over_hw) }
}

pub fn shear_buckling_check_required(hw: f64, tw: f64, eps: f64, eta: f64, k_tau: Option<f64>) -> bool {
    match k_tau {
        None => hw / tw > 72.0 * eps / eta,
        Some(k) => hw / tw > 31.0 * eps * k.sqrt() / eta,
    }
}

/// 🛡️ Web slenderness λ̄_w = hw/(37.4 tw ε √k_τ) (Eq. 5.6, stiffened web).
pub fn lambda_w(hw: f64, tw: f64, eps: f64, k_tau: f64) -> f64 {
    hw / (37.4 * tw * eps * k_tau.sqrt())
}

/// 🛡️ χ_w of Table 5.1; `rigid_end_post` selects the rigid or non-rigid row.
pub fn chi_w(lambda_w: f64, eta: f64, rigid_end_post: bool) -> f64 {
    if lambda_w < 0.83 / eta {
        eta
    } else if lambda_w < 1.08 {
        0.83 / lambda_w
    } else if rigid_end_post {
        1.37 / (0.7 + lambda_w)
    } else {
        0.83 / lambda_w
    }
}

/// 🛡️ V_bw,Rd = χ_w fyw hw tw/(√3 γM1).
pub fn v_bw_rd(chi_w: f64, fyw: f64, hw: f64, tw: f64, gm1: f64) -> f64 {
    chi_w * fyw * hw * tw / (3.0_f64.sqrt() * gm1)
}

/// 🛡️ Flange contribution V_bf,Rd (Eq. 5.8), `m_ed` ≤ M_f,Rd.
pub fn v_bf_rd(bf: f64, tf: f64, fyf: f64, c: f64, m_f_rd: f64, m_ed: f64, gm1: f64) -> f64 {
    let m_f = m_f_rd;
    let red = (1.0 - (m_ed / m_f).powi(2)).max(0.0);
    bf * tf * tf * fyf / (c * gm1) * red
}

pub fn v_b_rd_cap(eta: f64, fyw: f64, hw: f64, tw: f64, gm1: f64) -> f64 {
    eta * fyw * hw * tw / (3.0_f64.sqrt() * gm1)
}

/// 🛡️ Interaction of EN 1993-1-5 §7.1, η̄1 + (1 − M_f,Rd/M_pl,Rd)(2 η̄3 − 1)² for η̄1 = M/M_pl, η̄3 = V/V_bw.
pub fn bending_shear_buckling_utilisation(m_ed: f64, m_pl_rd: f64, m_f_rd: f64, v_ed: f64, v_bw_rd: f64) -> f64 {
    let eta1 = m_ed / m_pl_rd;
    let eta3 = v_ed / v_bw_rd;
    if eta3 <= 0.5 { eta1 } else { eta1 + (1.0 - m_f_rd / m_pl_rd) * (2.0 * eta3 - 1.0).powi(2) }
}
