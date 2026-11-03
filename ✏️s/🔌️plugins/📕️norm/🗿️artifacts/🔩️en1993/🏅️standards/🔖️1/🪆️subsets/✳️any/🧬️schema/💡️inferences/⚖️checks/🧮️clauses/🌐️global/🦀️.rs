//! 🌐️ EN 1993-1-1 §5 global analysis: α_cr, second-order amplification, imperfections (mm, N). See [EN 1993-1-1 §5.2–5.3](https://eurocodes.jrc.ec.europa.eu/).

pub fn alpha_cr_storey(h_ed: f64, v_ed: f64, h: f64, delta_h_ed: f64) -> Option<f64> {
    if v_ed <= 0.0 || delta_h_ed <= 0.0 { None } else { Some((h_ed / v_ed) * (h / delta_h_ed)) }
}

fn stability_functions(k: f64) -> (f64, f64) {
    let (sin, cos) = k.sin_cos();
    (k * (sin - k * cos) / (2.0 - 2.0 * cos - k * sin), (k - sin) / (sin - k * cos))
}

/// 🌐️ α_cr of a single-bay portal with fixed bases as the lowest sway buckling eigenvalue (EN 1993-1-1 5.2.1(3)): columns of inertia `ic` and height `h` carry `p_col` each, the beam of inertia `ib` and span `span` restrains the column heads with 6EI_b/L (antisymmetric sway). Exact stability functions: s(1+c)[s(1+c)/(s+κ) − 2] + k² = 0, κ = 6 I_b h/(I_c L), α_cr = k² E I_c/(h² P). Same units for all lengths and E.
pub fn alpha_cr_portal_fixed(e: f64, ic: f64, ib: f64, h: f64, span: f64, p_col: f64) -> Option<f64> {
    if e <= 0.0 || ic <= 0.0 || ib <= 0.0 || h <= 0.0 || span <= 0.0 || p_col <= 0.0 {
        return None;
    }
    let kappa = 6.0 * ib * h / (ic * span);
    let f = |k: f64| {
        let (s, c) = stability_functions(k);
        s * (1.0 + c) * (s * (1.0 + c) / (s + kappa) - 2.0) + k * k
    };
    let steps = 4000;
    let (mut a, mut fa) = (1e-4, f(1e-4));
    for i in 1..steps {
        let b = 1e-4 + (std::f64::consts::PI - 1e-4) * i as f64 / steps as f64;
        let fb = f(b);
        if fa < 0.0 && fb >= 0.0 {
            let (mut lo, mut hi) = (a, b);
            for _ in 0..80 {
                let mid = 0.5 * (lo + hi);
                if f(mid) < 0.0 { lo = mid } else { hi = mid }
            }
            let k = 0.5 * (lo + hi);
            return Some(k * k * e * ic / (h * h * p_col));
        }
        (a, fa) = (b, fb);
    }
    None
}

/// 🌐️ Relative excess of the Horne approximation (Eq. 5.2) over the eigenvalue; Eq. 5.2 is only accepted while it stays within [`horne_tolerance`] of the eigenvalue.
pub fn horne_excess(horne: f64, eigen: f64) -> f64 {
    horne / eigen - 1.0
}

/// 🌐️ Accepted relative excess of Eq. 5.2 over the eigenvalue, read from the registry (`ndp.en1993.v-026.2`).
pub fn horne_tolerance() -> f64 {
    semio_s_plugin_norm_compliance::ndp_common("ndp.en1993.v-026.2")
}

pub fn alpha_cr_limit(plastic: bool) -> f64 {
    if plastic { 15.0 } else { 10.0 }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnalysisMode {
    FirstOrder,
    AmplifiedFirstOrder,
    SecondOrder,
}

pub fn analysis_mode(alpha_cr: f64, plastic: bool) -> AnalysisMode {
    if alpha_cr >= alpha_cr_limit(plastic) {
        AnalysisMode::FirstOrder
    } else if alpha_cr >= 3.0 {
        AnalysisMode::AmplifiedFirstOrder
    } else {
        AnalysisMode::SecondOrder
    }
}

pub fn amplification(alpha_cr: f64) -> Option<f64> {
    if alpha_cr >= 3.0 { Some(1.0 / (1.0 - 1.0 / alpha_cr)) } else { None }
}

/// 📐️ Sway imperfection φ = φ0 α_h α_m (Eq. 5.5) with φ0 from `ndp.en1993.1-1.phi0` (recommended 1/200, unverified); `h` in m.
pub fn sway_phi(h_m: f64, m: f64) -> f64 {
    semio_s_plugin_norm_compliance::ndp_common("ndp.en1993.1-1.phi0") * sway_alpha_h(h_m) * sway_alpha_m(m)
}

/// 📐️ Height reduction α_h = 2/√h limited to 2/3 ≤ α_h ≤ 1 (Eq. 5.5); `h` in m.
pub fn sway_alpha_h(h_m: f64) -> f64 {
    (2.0 / h_m.sqrt()).clamp(2.0 / 3.0, 1.0)
}

/// 📐️ Column count reduction α_m = √(0.5 (1 + 1/m)) (Eq. 5.5).
pub fn sway_alpha_m(m: f64) -> f64 {
    (0.5 * (1.0 + 1.0 / m)).sqrt()
}

pub fn sway_equivalent_force(phi: f64, n_ed: f64) -> f64 {
    phi * n_ed
}

pub fn sway_may_be_neglected(h_ed: f64, v_ed: f64) -> bool {
    h_ed >= 0.15 * v_ed
}

/// 📐️ Bow imperfection e0/L of Table 5.1; `curve` index 0 = a0, 1 = a, 2 = b, 3 = c, 4 = d.
pub fn bow_ratio(curve: usize, plastic: bool) -> f64 {
    const CURVES: [&str; 5] = ["a0", "a", "b", "c", "d"];
    1.0 / semio_s_plugin_norm_compliance::ndp_common(&format!("ndp.en1993.1-1.t51.{}.{}", CURVES[curve], if plastic { "plastic" } else { "elastic" }))
}

/// 📐️ Equivalent uniform transverse load q = 8 N_Ed e0/L² of a bow imperfection e0 on a member of length L under N_Ed (formula unverified against the print: the spec text gives only the eq. 5.12-5.13 bracing form).
pub fn bow_equivalent_load(n_ed: f64, e0: f64, length: f64) -> f64 {
    8.0 * n_ed * e0 / (length * length)
}

pub fn bow_must_be_included(lambda_bar: f64, a: f64, fy: f64, n_ed: f64) -> bool {
    lambda_bar > 0.5 * (a * fy / n_ed).sqrt()
}

/// 📐️ Bracing system: equivalent stabilising force q_d = Σ N_Ed · 8 (e0 + δ_q)/L² with e0 = α_m L/divisor, divisor 500 from `ndp.en1993.bracing.imperfection-divisor`.
pub fn bracing_qd(sum_n_ed: f64, m: f64, l: f64, delta_q: f64) -> f64 {
    let alpha_m = (0.5 * (1.0 + 1.0 / m)).sqrt();
    let e0 = alpha_m * l / semio_s_plugin_norm_compliance::ndp_common("ndp.en1993.bracing.imperfection-divisor");
    sum_n_ed * 8.0 * (e0 + delta_q) / (l * l)
}
