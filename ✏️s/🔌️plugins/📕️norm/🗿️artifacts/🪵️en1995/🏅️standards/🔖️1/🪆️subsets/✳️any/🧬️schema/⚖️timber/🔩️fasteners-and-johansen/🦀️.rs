//! 🔩️ Fasteners of the EN 1995 timber checks: the characteristic embedment and yield values, the Johansen load-carrying capacities, the effective number and the spacing minima. See [`super`].

use super::*;

pub const K0_7_MM: f64 = 0.007;

pub fn d_char_n(fire_duration_s: f64, beta_n: f64) -> f64 {
    beta_n * fire_duration_s
}

pub fn d_ef(fire_duration_s: f64, beta_n: f64) -> f64 {
    let d_char = d_char_n(fire_duration_s, beta_n);
    let k0 = if fire_duration_s >= 1200.0 {
        1.0
    } else {
        fire_duration_s / 1200.0
    };
    d_char + k0 * K0_7_MM
}

pub fn residual_bh(b_m: f64, h_m: f64, d_ef: f64) -> (f64, f64) {
    ((b_m - 2.0 * d_ef).max(0.0), (h_m - 2.0 * d_ef).max(0.0))
}

pub fn f_h_k(rho_k: f64, d_m: f64, fastener: &str) -> f64 {
    let d_mm = d_m * 1000.0;
    if fastener.to_ascii_lowercase().contains("nail") && d_mm < 8.0 {
        0.082 * rho_k * d_mm.powf(-0.3) * 1e6
    } else {
        0.082 * (1.0 - 0.01 * d_mm) * rho_k * 1e6
    }
}

/// 🔩 Yield moment M_y,Rk = 0.3·f_u,k·d^2.6 (eq. 8.14 / 8.30) — empirical in N·mm, MPa and mm, returned in N·m.
pub fn m_y_k(f_u_k: f64, d_m: f64) -> f64 {
    0.3 * (f_u_k / 1e6) * (d_m * 1000.0).powf(2.6) / 1000.0
}

/// 🔩 Johansen timber–timber single shear (§8.2.2).
pub fn johansen_single_shear_f_v_rk(
    t1: f64,
    t2: f64,
    d: f64,
    f_h1: f64,
    f_h2: f64,
    m_y: f64,
    f_ax_rk: f64,
) -> f64 {
    let beta = if f_h1 > 0.0 { f_h2 / f_h1 } else { 1.0 };
    let f_a = f_h1 * t1 * d;
    let f_b = f_h2 * t2 * d;
    let ratio = if t2 > 0.0 { t1 / t2 } else { 1.0 };
    let f_c = (f_h1 * t1 * d / (1.0 + beta))
        * ((beta + 2.0 * beta.powi(2) * (1.0 + ratio + ratio.powi(2)) + beta.powi(3) * ratio.powi(2))
            .sqrt()
            - beta * (1.0 + ratio));
    let f_d = 1.05
        * (f_h1 * t1 * d / (2.0 + beta))
        * ((2.0 * beta * (1.0 + beta)
            + (4.0 * beta * (2.0 + beta) * m_y) / (f_h1 * d * t1.powi(2)).max(1e-18))
            .sqrt()
            - beta);
    let f_e = 1.05
        * (f_h1 * t2 * d / (1.0 + 2.0 * beta))
        * ((2.0 * beta.powi(2) * (1.0 + beta)
            + (4.0 * beta * (1.0 + 2.0 * beta) * m_y) / (f_h1 * d * t2.powi(2)).max(1e-18))
            .sqrt()
            - beta);
    let f_f = 1.15 * ((2.0 * beta) / (1.0 + beta)).sqrt() * (2.0 * m_y * f_h1 * d).sqrt();
    let modes = [f_a, f_b, f_c.max(0.0), f_d.max(0.0), f_e.max(0.0), f_f.max(0.0)];
    let base = modes.into_iter().fold(f64::INFINITY, f64::min);
    let rope = (f_ax_rk / 4.0).min(base * 0.25);
    base + rope
}

/// 🔩 Johansen timber–timber double shear (§8.2.2).
pub fn johansen_double_shear_f_v_rk(
    t1: f64,
    t2: f64,
    d: f64,
    f_h1: f64,
    f_h2: f64,
    m_y: f64,
    f_ax_rk: f64,
) -> f64 {
    let beta = if f_h1 > 0.0 { f_h2 / f_h1 } else { 1.0 };
    let f_g = f_h1 * t1 * d;
    let f_h = 0.5 * f_h2 * t2 * d;
    let f_j = 1.05
        * (f_h1 * t1 * d / (2.0 + beta))
        * ((2.0 * beta * (1.0 + beta)
            + (4.0 * beta * (2.0 + beta) * m_y) / (f_h1 * d * t1.powi(2)).max(1e-18))
            .sqrt()
            - beta);
    let f_k = 1.15 * ((2.0 * beta) / (1.0 + beta)).sqrt() * (2.0 * m_y * f_h1 * d).sqrt();
    let modes = [f_g, f_h, f_j.max(0.0), f_k.max(0.0)];
    let base = modes.into_iter().fold(f64::INFINITY, f64::min);
    let rope = (f_ax_rk / 4.0).min(base * 0.25);
    base + rope
}

/// 🔩 Johansen steel–timber single shear per shear plane (§8.2.3 eq. 8.9 thin t ≤ 0.5d, eq. 8.10 thick t ≥ d,
/// linear interpolation between).
pub fn johansen_steel_timber_f_v_rk(
    t_timber: f64,
    t_steel: f64,
    d: f64,
    f_h: f64,
    m_y: f64,
    f_ax_rk: f64,
) -> f64 {
    let with_rope = |base: f64| base + (f_ax_rk / 4.0).min(base * 0.25);
    let thin = with_rope((0.4 * f_h * t_timber * d).min(1.15 * (2.0 * m_y * f_h * d).sqrt()).max(0.0));
    let thick = johansen_steel_central_f_v_rk(t_timber, d, f_h, m_y, f_ax_rk);
    let ratio = ((t_steel / d.max(1e-12) - 0.5) / 0.5).clamp(0.0, 1.0);
    thin + ratio * (thick - thin)
}

/// 🔩 Johansen thick steel plate (§8.2.3 eq. 8.10) — also the central steel plate of any thickness in double
/// shear (eq. 8.13), per shear plane.
pub fn johansen_steel_central_f_v_rk(t_timber: f64, d: f64, f_h: f64, m_y: f64, f_ax_rk: f64) -> f64 {
    let bearing = f_h * t_timber * d;
    let one_hinge = bearing * ((2.0 + (4.0 * m_y) / (f_h * d * t_timber.powi(2)).max(1e-18)).sqrt() - 1.0);
    let two_hinges = 2.3 * (m_y * f_h * d).sqrt();
    let base = bearing.min(one_hinge).min(two_hinges).max(0.0);
    base + (f_ax_rk / 4.0).min(base * 0.25)
}

pub fn n_ef(n: u32, a1_m: f64, d_m: f64) -> f64 {
    if n <= 1 {
        return 1.0;
    }
    let a1 = a1_m.max(1e-6);
    let d = d_m.max(1e-6);
    let cand = (n as f64).powf(0.9) * (a1 / (13.0 * d)).powf(0.25);
    (n as f64).min(cand)
}

/// 📏 Table 8.2 / 8.4 / 8.5 minima (α = 0 parallel-to-grain force direction).
pub fn spacing_minima(c: &TimberConnection) -> (f64, f64, f64) {
    let d = c.diameter_m.max(1e-6);
    let ft = c.fastener_type.to_ascii_lowercase();
    let (a1, a3t, a4t) = if ft.contains("nail") {
        (5.0 * d, 10.0 * d, 5.0 * d)
    } else if ft.contains("screw") {
        (5.0 * d, 10.0 * d, 5.0 * d)
    } else {
        (5.0 * d, 7.0 * d, 3.0 * d)
    };
    (a1, a3t, a4t)
}
