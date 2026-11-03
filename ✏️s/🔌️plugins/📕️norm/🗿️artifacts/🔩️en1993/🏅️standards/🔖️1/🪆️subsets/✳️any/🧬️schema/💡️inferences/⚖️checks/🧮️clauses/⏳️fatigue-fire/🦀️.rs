//! ⏳️ EN 1993-1-9 fatigue and EN 1993-1-2 fire (SI-like: MPa, °C, s, mm). See [EN 1993-1-9](https://eurocodes.jrc.ec.europa.eu/) and [EN 1993-1-2](https://eurocodes.jrc.ec.europa.eu/).

fn sn(key: &str) -> f64 {
    semio_s_plugin_norm_compliance::ndp_common(&format!("ndp.en1993.1-9.sn.{key}"))
}

/// 🧾 Constant amplitude fatigue limit Δσ_D from the corner points N_C, N_D and slope m1 of `ndp.en1993.1-9.sn.*`.
pub fn delta_sigma_d(delta_c: f64) -> f64 {
    delta_c * (sn("n-c") / sn("n-d")).powf(1.0 / sn("m-1"))
}

/// 🧾 Cut-off limit Δσ_L from the corner points N_D, N_L and slope m2.
pub fn delta_sigma_l(delta_c: f64) -> f64 {
    delta_sigma_d(delta_c) * (sn("n-d") / sn("n-l")).powf(1.0 / sn("m-2"))
}

/// 🧾 Endurance N_R for a stress range (already divided by γMf) with the S-N curve of a detail category Δσ_C (m1 above Δσ_D, m2 down to Δσ_L, none below).
pub fn endurance(delta: f64, delta_c: f64) -> Option<f64> {
    let d = delta_sigma_d(delta_c);
    let l = delta_sigma_l(delta_c);
    if delta > d {
        Some(sn("n-c") * (delta_c / delta).powf(sn("m-1")))
    } else if delta > l {
        Some(sn("n-d") * (d / delta).powf(sn("m-2")))
    } else {
        None
    }
}

/// 🧾 Palmgren–Miner damage Σ n_i/N_Ri of a block spectrum with design stress ranges γFf Δσ_i and strength Δσ_C/γMf.
pub fn miner_damage(spectrum: &[(f64, f64)], delta_c: f64, gamma_mf: f64, gamma_ff: f64) -> f64 {
    let dc = delta_c / gamma_mf;
    spectrum
        .iter()
        .filter_map(|&(ds, n)| endurance(ds * gamma_ff, dc).map(|nr| n / nr))
        .sum()
}

/// 🧾 Shear fatigue S-N: Δτ_C with m = 5, cut-off at Δτ_L = 0.457 Δτ_C.
pub fn endurance_shear(delta_tau: f64, delta_tau_c: f64) -> Option<f64> {
    if delta_tau < 0.457 * delta_tau_c { None } else { Some(sn("n-c") * (delta_tau_c / delta_tau).powf(sn("m-2"))) }
}

/// 🧾 Combined normal and shear ranges, Eq. 8.3: (γFf Δσ/(Δσ_C/γMf))³ + (γFf Δτ/(Δτ_C/γMf))⁵.
pub fn combined_normal_shear(d_sigma: f64, d_sigma_c: f64, d_tau: f64, d_tau_c: f64, gamma_ff: f64, gamma_mf: f64) -> f64 {
    (gamma_ff * d_sigma * gamma_mf / d_sigma_c).powi(3) + (gamma_ff * d_tau * gamma_mf / d_tau_c).powi(5)
}

/// 🧾 Damage-equivalent range Δσ_E,2 = λ φ2 Δσ_p (EN 1993-2 §9.5).
pub fn delta_sigma_e2(lambda: f64, phi2: f64, delta_sigma_p: f64) -> f64 {
    lambda * phi2 * delta_sigma_p
}

/// 🧾 Rainflow cycle counting by the ASTM three-point algorithm on a load history; returns (range, count) with half cycles as 0.5.
pub fn rainflow(history: &[f64]) -> Vec<(f64, f64)> {
    let mut pts: Vec<f64> = Vec::new();
    for &x in history {
        if pts.last() != Some(&x) {
            pts.push(x);
        }
    }
    let mut rev: Vec<f64> = Vec::new();
    for i in 0..pts.len() {
        if i == 0 || i + 1 == pts.len() || (pts[i] - pts[i - 1]) * (pts[i + 1] - pts[i]) < 0.0 {
            rev.push(pts[i]);
        }
    }
    let mut stack: Vec<f64> = Vec::new();
    let mut out = Vec::new();
    for &p in &rev {
        stack.push(p);
        while stack.len() >= 3 {
            let n = stack.len();
            let x = (stack[n - 1] - stack[n - 2]).abs();
            let y = (stack[n - 2] - stack[n - 3]).abs();
            if x < y {
                break;
            }
            if n == 3 {
                out.push((y, 0.5));
                stack.remove(0);
            } else {
                out.push((y, 1.0));
                let last = stack.pop().unwrap_or(0.0);
                stack.truncate(n - 3);
                stack.push(last);
            }
        }
    }
    for w in stack.windows(2) {
        out.push(((w[1] - w[0]).abs(), 0.5));
    }
    out
}

const THETAS: [u32; 13] = [20, 100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200];

/// 🔥️ Reduction factor of EN 1993-1-2 Table 3.1 (`col` 1 = k_y,θ, 2 = k_p,θ, else k_E,θ) at the table temperature `index`, read from the registry (`ndp.en1993.1-2.t31.<ky|kp|ke>.<θ>`).
fn table_3_1(index: usize, col: usize) -> f64 {
    let key = match col {
        1 => "ky",
        2 => "kp",
        _ => "ke",
    };
    semio_s_plugin_norm_compliance::ndp_common(&format!("ndp.en1993.1-2.t31.{key}.{}", THETAS[index]))
}

fn interp(theta: f64, col: usize) -> f64 {
    let t = theta.clamp(20.0, 1200.0);
    for index in 0..THETAS.len() - 1 {
        let (t0, t1) = (f64::from(THETAS[index]), f64::from(THETAS[index + 1]));
        if t >= t0 && t <= t1 {
            let f = (t - t0) / (t1 - t0);
            let (a, b) = (table_3_1(index, col), table_3_1(index + 1, col));
            return a + f * (b - a);
        }
    }
    0.0
}

pub fn k_y_theta(theta: f64) -> f64 {
    interp(theta, 1)
}

pub fn k_p_theta(theta: f64) -> f64 {
    interp(theta, 2)
}

pub fn k_e_theta(theta: f64) -> f64 {
    interp(theta, 3)
}

pub fn theta_critical(mu0: f64) -> f64 {
    if mu0 >= 1.0 {
        return 20.0;
    }
    39.19 * (1.0 / (0.9674 * mu0.powf(3.833)) - 1.0).ln() + 482.0
}

pub fn mu0(e_fi_d: f64, r_fi_d_0: f64) -> f64 {
    e_fi_d / r_fi_d_0
}

pub fn gas_iso834(t_min: f64) -> f64 {
    20.0 + 345.0 * (8.0 * t_min + 1.0).log10()
}

pub fn gas_hydrocarbon(t_min: f64) -> f64 {
    20.0 + 1080.0 * (1.0 - 0.325 * (-0.167 * t_min).exp() - 0.675 * (-2.5 * t_min).exp())
}

pub fn gas_external(t_min: f64) -> f64 {
    20.0 + 660.0 * (1.0 - 0.687 * (-0.32 * t_min).exp() - 0.313 * (-3.8 * t_min).exp())
}

pub fn specific_heat_steel(theta: f64) -> f64 {
    if theta < 600.0 {
        425.0 + 0.773 * theta - 1.69e-3 * theta * theta + 2.22e-6 * theta.powi(3)
    } else if theta < 735.0 {
        666.0 + 13002.0 / (738.0 - theta)
    } else if theta < 900.0 {
        545.0 + 17820.0 / (theta - 731.0)
    } else {
        650.0
    }
}

pub fn k_sh_i_section(box_over_am_v: f64) -> f64 {
    0.9 * box_over_am_v
}

/// 🔥 Unprotected steel temperature history sampled each `dt` seconds up to `t_end` s for gas temperature `gas(t_min)`.
pub fn unprotected_steel_temperature(gas: &dyn Fn(f64) -> f64, k_sh: f64, am_over_v: f64, t_end: f64, dt: f64, eps_m: f64, eps_f: f64) -> Vec<(f64, f64)> {
    let sigma = 5.67e-8;
    let alpha_c = 25.0;
    let mut theta_a = 20.0;
    let mut t = 0.0;
    let mut out = vec![(0.0, theta_a)];
    while t < t_end - 1e-9 {
        let tg = gas(t / 60.0);
        let h_net = alpha_c * (tg - theta_a) + eps_m * eps_f * sigma * ((tg + 273.0).powi(4) - (theta_a + 273.0).powi(4));
        let d = k_sh * am_over_v / (specific_heat_steel(theta_a) * 7850.0) * h_net * dt;
        theta_a += d.max(0.0);
        t += dt;
        out.push((t, theta_a));
    }
    out
}

/// 🔥 Protected steel temperature with insulation thickness `dp` (m), λ_p, ρ_p, c_p and Ap/V (1/m).
pub fn protected_steel_temperature(gas: &dyn Fn(f64) -> f64, ap_over_v: f64, dp: f64, lambda_p: f64, rho_p: f64, c_p: f64, t_end: f64, dt: f64) -> Vec<(f64, f64)> {
    let mut theta_a = 20.0;
    let mut t = 0.0;
    let mut out = vec![(0.0, theta_a)];
    while t < t_end - 1e-9 {
        let tg = gas(t / 60.0);
        let tg_next = gas((t + dt) / 60.0);
        let ca = specific_heat_steel(theta_a);
        let phi = c_p * rho_p / (ca * 7850.0) * dp * ap_over_v;
        let d = (lambda_p / dp * ap_over_v) / (ca * 7850.0 * (1.0 + phi / 3.0)) * (tg - theta_a) * dt - ((phi / 10.0).exp() - 1.0) * (tg_next - tg);
        theta_a += d.max(0.0);
        t += dt;
        out.push((t, theta_a));
    }
    out
}

pub fn time_to_temperature(history: &[(f64, f64)], theta_cr: f64) -> Option<f64> {
    history.windows(2).find(|w| w[1].1 >= theta_cr).map(|w| {
        let f = (theta_cr - w[0].1) / (w[1].1 - w[0].1);
        w[0].0 + f * (w[1].0 - w[0].0)
    })
}

/// 🔥 χ_fi (§4.2.3.2) for a member with room-temperature slenderness λ̄ and temperature-dependent θ.
pub fn chi_fi(lambda_bar: f64, theta: f64, fy: f64) -> f64 {
    let lt = lambda_bar * (k_y_theta(theta) / k_e_theta(theta).max(1e-9)).sqrt();
    let alpha = 0.65 * (235.0 / fy).sqrt();
    let phi = 0.5 * (1.0 + alpha * lt + lt * lt);
    1.0 / (phi + (phi * phi - lt * lt).max(0.0).sqrt())
}

pub fn n_b_fi_t_rd(chi_fi: f64, a: f64, ky: f64, fy: f64, gm_fi: f64) -> f64 {
    chi_fi * a * ky * fy / gm_fi
}

/// 🔥 M_fi,θ,Rd = ky,θ (γM0/γM,fi) M_Rd / (κ1 κ2) for class 1–3 (Eq. 4.7).
pub fn m_fi_theta_rd(ky: f64, gm0: f64, gm_fi: f64, m_rd: f64, kappa1: f64, kappa2: f64) -> f64 {
    ky * gm0 / gm_fi * m_rd / (kappa1 * kappa2)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BeamExposure {
    AllSides,
    UnprotectedThreeSidesWithSlab,
    ProtectedThreeSidesWithSlab,
}

pub fn kappa_1(exposure: BeamExposure) -> f64 {
    match exposure {
        BeamExposure::AllSides => 1.0,
        BeamExposure::UnprotectedThreeSidesWithSlab => 0.70,
        BeamExposure::ProtectedThreeSidesWithSlab => 0.85,
    }
}

pub fn kappa_2(statically_indeterminate_support: bool) -> f64 {
    if statically_indeterminate_support { 0.85 } else { 1.0 }
}

/// 🌋 M_b,fi,t,Rd = χ_LT,fi W_pl,y k_y,θ f_y/γ_M,fi (§4.2.3.3, Eq. 4.14) with χ_LT,fi from [`chi_fi`] evaluated on λ̄_LT.
pub fn m_b_fi_t_rd(chi_lt_fi: f64, w_pl_y: f64, ky: f64, fy: f64, gm_fi: f64) -> f64 {
    chi_lt_fi * w_pl_y * ky * fy / gm_fi
}

/// 🌡️ μ_y = (1.2 β_M,y − 3) λ̄_y,θ + 0.44 β_M,y + 0.29 ≤ 0.8 with λ̄_y,θ ≤ 1.1 (§4.2.3.5, Eq. 4.21c).
pub fn mu_y_fire(beta_m: f64, lambda_theta: f64) -> f64 {
    ((1.2 * beta_m - 3.0) * lambda_theta.min(1.1) + 0.44 * beta_m + 0.29).min(0.8)
}

/// 🧯 μ_LT = 0.15 λ̄_z,θ β_M,LT − 0.15 ≤ 0.9 (§4.2.3.5, Eq. 4.21e).
pub fn mu_lt_fire(lambda_z_theta: f64, beta_m_lt: f64) -> f64 {
    (0.15 * lambda_z_theta * beta_m_lt - 0.15).min(0.9)
}

/// ♨️ Interaction factor k = 1 − μ N_fi,Ed/(χ_fi A k_y,θ f_y/γ_M,fi) bounded above by `cap` (3 for k_y, 1 for k_LT).
pub fn k_fire(mu: f64, n_fi: f64, chi_fi: f64, a: f64, ky: f64, fy: f64, gm_fi: f64, cap: f64) -> f64 {
    (1.0 - mu * n_fi / (chi_fi * a * ky * fy / gm_fi)).min(cap)
}

/// 🚒 Utilisations of Eq. 4.21a (in-plane, χ_min,fi and k_y) and 4.21b (out-of-plane, χ_z,fi and k_LT) for strong-axis bending; `m_rd` is W_pl,y k_y,θ f_y/γ_M,fi.
pub fn bending_axial_fire(n_fi: f64, n_rd_min: f64, n_rd_z: f64, m_fi: f64, m_rd_lt: f64, k_y: f64, k_lt: f64) -> (f64, f64) {
    (n_fi / n_rd_min + k_y * m_fi / m_rd_lt, n_fi / n_rd_z + k_lt * m_fi / m_rd_lt)
}

