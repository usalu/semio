//! 🧰️ EN 1993-1-1 §6.4 built-up members, §7 serviceability, EN 1993-1-10 toughness, EN 1993-4-3 pipes (mm, N, MPa). See [EN 1993-1-1 §6.4](https://eurocodes.jrc.ec.europa.eu/).

use std::f64::consts::PI;

pub struct BuiltUp {
    pub i_eff: f64,
    pub n_cr: f64,
    pub m_ed: f64,
    pub n_ch_ed: f64,
    pub v_ed: f64,
}

/// 🧱️ Laced or battened compression member second-order effects (§6.4.1) with shear stiffness `sv`.
pub fn built_up_member(n_ed: f64, m_ed_first: f64, l: f64, h0: f64, a_ch: f64, i_ch: f64, mu: f64, sv: f64, e: f64) -> BuiltUp {
    let i_eff = 0.5 * h0 * h0 * a_ch + 2.0 * mu * i_ch;
    let n_cr = PI * PI * e * i_eff / (l * l);
    let e0 = l / semio_s_plugin_norm_compliance::ndp_common("ndp.en1993.built-up.bow-divisor");
    let m_ed = (n_ed * e0 + m_ed_first) / (1.0 - n_ed / n_cr - n_ed / sv);
    let n_ch_ed = 0.5 * n_ed + m_ed * h0 * a_ch / (2.0 * i_eff);
    BuiltUp { i_eff, n_cr, m_ed, n_ch_ed, v_ed: PI * m_ed / l }
}

/// 🧱️ Efficiency factor μ of a battened member from λ = L/i0 (§6.4.3.1).
pub fn mu_battened(lambda: f64) -> f64 {
    if lambda >= 150.0 { 0.0 } else if lambda > 75.0 { 2.0 - lambda / 75.0 } else { 1.0 }
}

/// 🧱️ Shear stiffness of a battened member, capped at 2π²E I_ch/a².
pub fn sv_battened(e: f64, i_ch: f64, a: f64, n: f64, i_b: f64, h0: f64) -> f64 {
    let s = 24.0 * e * i_ch / (a * a * (1.0 + 2.0 * i_ch / (n * i_b) * h0 / a));
    s.min(2.0 * PI * PI * e * i_ch / (a * a))
}

/// 🧱️ Shear stiffness of a laced member, single bracing system (Fig. 6.9 a): n E A_d a h0²/(2 d³).
pub fn sv_laced(n: f64, e: f64, a_d: f64, a: f64, h0: f64, d: f64) -> f64 {
    n * e * a_d * a * h0 * h0 / (2.0 * d.powi(3))
}

pub fn batten_forces(v_ed: f64, a: f64, n: f64, h0: f64) -> (f64, f64, f64) {
    (v_ed * a / 4.0, v_ed * a / (n * h0), v_ed * a / (2.0 * n))
}

/// 🧱️ Closely spaced built-up members: maximum chain spacing (§6.4.4, Table 6.9) in units of i_min.
pub fn closely_spaced_max_spacing(back_to_back_angles: bool) -> f64 {
    if back_to_back_angles { 15.0 } else { 70.0 }
}

pub fn deflection_limit(span: f64, ratio: f64) -> f64 {
    span / ratio
}

pub fn max_deflection(d1: f64, d2: f64, d0: f64) -> f64 {
    d1 + d2 - d0
}

/// 〰️ First natural frequency f1 = π/(2L²) √(EI/m) of a simply supported beam (m per length).
pub fn floor_frequency(l: f64, ei: f64, m: f64) -> f64 {
    PI / (2.0 * l * l) * (ei / m).sqrt()
}

/// ❄️ Reference temperature T_Ed = T_md + ΔT_r + ΔT_σ + ΔT_R + ΔT_ε̇ + ΔT_εcf.
pub fn reference_temperature(t_md: f64, dt_r: f64, dt_sigma: f64, dt_r_margin: f64, dt_rate: f64, dt_cf: f64) -> f64 {
    t_md + dt_r + dt_sigma + dt_r_margin + dt_rate + dt_cf
}

/// ❄️ Cold-forming temperature shift, ΔT_εcf = −3 ε_cf[%] with ε_cf = 100 t/(2r).
pub fn cold_forming_shift(t: f64, r: f64) -> f64 {
    -3.0 * 100.0 * t / (2.0 * r)
}

pub struct ToughnessRow {
    pub temperature: f64,
    pub t_max: f64,
}

/// ❄️ Permissible thickness by linear interpolation of a Table 2.1 column ordered by decreasing temperature.
pub fn permissible_thickness(rows: &[ToughnessRow], t_ed: f64) -> f64 {
    if t_ed >= rows[0].temperature {
        return rows[0].t_max;
    }
    for w in rows.windows(2) {
        if t_ed <= w[0].temperature && t_ed >= w[1].temperature {
            let f = (w[0].temperature - t_ed) / (w[0].temperature - w[1].temperature);
            return w[0].t_max + f * (w[1].t_max - w[0].t_max);
        }
    }
    rows[rows.len() - 1].t_max
}

pub fn hoop_thickness_barlow(p: f64, d: f64, fy: f64, gm0: f64) -> f64 {
    p * d / (2.0 * fy / gm0)
}

pub fn pipe_external_pressure_cr(e: f64, nu: f64, t: f64, d: f64) -> f64 {
    2.0 * e / (1.0 - nu * nu) * (t / d).powi(3)
}

pub fn spangler_ovalisation(kb: f64, w: f64, d: f64, e: f64, i_wall: f64, e_soil: f64) -> f64 {
    kb * w * d.powi(3) / (8.0 * e * i_wall + 0.061 * e_soil * d.powi(3))
}

pub fn plate_lateral_pressure_stress(k: f64, p: f64, b: f64, t: f64) -> f64 {
    k * p * b * b / (t * t)
}
