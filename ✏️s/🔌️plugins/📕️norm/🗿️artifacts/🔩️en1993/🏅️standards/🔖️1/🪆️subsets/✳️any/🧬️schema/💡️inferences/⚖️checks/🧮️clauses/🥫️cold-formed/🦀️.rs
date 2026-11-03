//! 🥫️ EN 1993-1-3 cold-formed members: edge-stiffened flanges with iterated distortional buckling, shear, web crippling, sheeting restraint (mm, N, MPa). See [EN 1993-1-3](https://eurocodes.jrc.ec.europa.eu/).

use crate::cold_stainless::{chi_distortional, sigma_cr_stiffener, stiffener_spring};
use crate::effective_width::{lambda_p, rho_internal, rho_outstand};

pub const NU_STEEL: f64 = 0.3;

/// 🥫️ k_σ of the edge fold (lip) of a flange, 0.5 for bp,c/bp ≤ 0.35 and 0.5 + 0.83 ∛((bp,c/bp − 0.35)²) up to 0.6 (§5.5.3.2(5)).
pub fn k_sigma_edge_fold(c_over_b: f64) -> f64 {
    if c_over_b <= 0.35 { 0.5 } else { 0.5 + 0.83 * ((c_over_b.min(0.6) - 0.35).powi(2)).cbrt() }
}

pub struct EdgeStiffened {
    pub bp: f64,
    pub cp: f64,
    pub t: f64,
    pub fyb: f64,
    pub hw: f64,
    pub b2: f64,
    pub kf: f64,
    pub e: f64,
}

pub struct Distortional {
    pub be2: f64,
    pub ceff: f64,
    pub a_s: f64,
    pub i_s: f64,
    pub sigma_cr_s: f64,
    pub lambda_d: f64,
    pub chi_d: f64,
    pub iterations: usize,
}

/// 🥫️ Effective stiffener (flange part b_e2 and lip c_eff) at compression stress `sigma`, area and inertia about its own centroid axis parallel to the flange, and its distance `b1` from the web.
fn stiffener_state(g: &EdgeStiffened, sigma: f64) -> (f64, f64, f64, f64, f64) {
    let eps = (235.0 / g.fyb).sqrt();
    let red = (sigma / g.fyb).sqrt();
    let rho_f = rho_internal(lambda_p(g.bp, g.t, eps, 4.0) * red, 1.0);
    let rho_l = rho_outstand(lambda_p(g.cp, g.t, eps, k_sigma_edge_fold(g.cp / g.bp)) * red);
    let be2 = 0.5 * rho_f * g.bp;
    let ceff = rho_l * g.cp;
    let a = (be2 + ceff) * g.t;
    let yc = ceff * ceff / 2.0 / (be2 + ceff);
    let i = be2 * g.t.powi(3) / 12.0 + be2 * g.t * yc * yc + g.t * ceff.powi(3) / 12.0 + g.t * ceff * (ceff / 2.0 - yc).powi(2);
    let b1 = (be2 * (g.bp - be2 / 2.0) + ceff * g.bp) / (be2 + ceff);
    (be2, ceff, a, i, b1)
}

/// 🥫️ Iterated distortional buckling of a single edge-stiffened flange (§5.5.3.2(3) to (10)): σ is updated with χ_d f_yb until χ_d converges.
pub fn distortional_edge_stiffener(g: &EdgeStiffened, gm0: f64) -> Distortional {
    let mut sigma = g.fyb / gm0;
    let mut chi_prev = 0.0;
    let mut out = Distortional { be2: 0.0, ceff: 0.0, a_s: 0.0, i_s: 0.0, sigma_cr_s: 0.0, lambda_d: 0.0, chi_d: 1.0, iterations: 0 };
    for n in 1..=30 {
        let (be2, ceff, a, i, b1) = stiffener_state(g, sigma);
        let k = stiffener_spring(g.e, g.t, b1, g.b2, g.hw, g.kf);
        let scr = sigma_cr_stiffener(k, g.e, i, a);
        let lam = (g.fyb / scr).sqrt();
        let chi = chi_distortional(lam);
        out = Distortional { be2, ceff, a_s: a, i_s: i, sigma_cr_s: scr, lambda_d: lam, chi_d: chi, iterations: n };
        if (chi - chi_prev).abs() < 1e-9 {
            break;
        }
        chi_prev = chi;
        sigma = chi * g.fyb / gm0;
    }
    out
}

/// 🥫️ Reduced stiffener area χ_d A_s and reduced thickness t_red = χ_d t (§5.5.3.2(11)).
pub fn reduced_stiffener(chi_d: f64, a_s: f64, t: f64) -> (f64, f64) {
    (chi_d * a_s, chi_d * t)
}

/// 🥫️ Relative slenderness of the web in shear, λ_w = 0.346 (s_w/t) √(f_yb/E) (§6.1.5).
pub fn lambda_w_cold(sw: f64, t: f64, fyb: f64, e: f64) -> f64 {
    0.346 * sw / t * (fyb / e).sqrt()
}

/// 🥫️ Shear strength f_bv of Table 6.1 (webs without or with stiffened supports).
pub fn f_bv(lambda_w: f64, fyb: f64, stiffened_support: bool) -> f64 {
    if lambda_w <= 0.83 {
        0.58 * fyb
    } else if stiffened_support || lambda_w < 1.40 {
        0.48 * fyb / lambda_w
    } else {
        0.67 * fyb / (lambda_w * lambda_w)
    }
}

/// 🥫️ Web shear resistance V_b,Rd = (h_w/sin φ) t f_bv/γM0.
pub fn v_b_rd_cold(hw: f64, phi_deg: f64, t: f64, f_bv: f64, gm0: f64) -> f64 {
    hw / phi_deg.to_radians().sin() * t * f_bv / gm0
}

/// 🥫️ Web crippling coefficient α and bearing length l_a of category 1 and 2 (Table 6.? values).
pub fn web_crippling_alpha(category: u8, ss: f64) -> (f64, f64) {
    if category == 1 { (semio_s_plugin_norm_compliance::ndp_common("ndp.en1993.1-3.web-crippling.cat1.alpha"), semio_s_plugin_norm_compliance::ndp_common("ndp.en1993.1-3.web-crippling.cat1.la")) } else { (semio_s_plugin_norm_compliance::ndp_common("ndp.en1993.1-3.web-crippling.cat2.alpha"), ss) }
}

/// 🥫️ Web crippling resistance R_w,Rd (§6.1.7.2).
pub fn web_crippling_rd(alpha: f64, t: f64, fyb: f64, e: f64, r: f64, la: f64, phi_deg: f64, gm1: f64) -> f64 {
    alpha * t * t * (fyb * e).sqrt() * (1.0 - 0.1 * (r / t).sqrt()) * (0.5 + (0.02 * la / t).sqrt()) * (2.4 + (phi_deg / 90.0).powi(2)) / gm1
}

/// 🥫️ Combined bending and web crippling, utilisation of M_Ed/M_c,Rd + F_Ed/R_w,Rd ≤ 1.25 (§6.1.11).
pub fn bending_crippling_utilisation(m_ed: f64, mc_rd: f64, f_ed: f64, rw_rd: f64) -> f64 {
    (m_ed / mc_rd + f_ed / rw_rd) / 1.25
}

/// 🥫️ Axial compression with bending about one axis, utilisation N_Ed/N_c,Rd + M_Ed/M_c,Rd ≤ 1 (§6.1.9(1) without the centroid shift ΔM).
pub fn axial_bending_utilisation(n_ed: f64, n_c_rd: f64, m_ed: f64, m_c_rd: f64) -> f64 {
    n_ed / n_c_rd + m_ed / m_c_rd
}

/// 🥫️ Flange curling u = 2 σa² bs⁴/(E² t² z) (EN 1993-1-3 Eq. 5.1, §5.4); σa mean flange stress, bs flange width, z distance of the flange to the neutral axis.
pub fn flange_curling(sigma_a: f64, bs: f64, e: f64, t: f64, z: f64) -> f64 {
    2.0 * sigma_a * sigma_a * bs.powi(4) / (e * e * t * t * z)
}

/// 🥫️ Compression resistance at the effective area and slenderness λ̄ = √(A_eff f_y/N_cr) (§6.2.2).
pub fn n_c_rd_cold(a_eff: f64, fyb: f64, gm0: f64) -> f64 {
    a_eff * fyb / gm0
}

pub fn lambda_eff_cold(a_eff: f64, fyb: f64, ncr: f64) -> f64 {
    (a_eff * fyb / ncr).sqrt()
}

/// 🥫️ Spring stiffness of sheeting and purlin in series, C_D = 1/(1/C_D,C + 1/C_D,B) (§10.1.5).
pub fn c_d_series(c_d_c: f64, c_d_b: f64) -> f64 {
    1.0 / (1.0 / c_d_c + 1.0 / c_d_b)
}

/// 🥫️ Rotational restraint of the sheeting-to-purlin connection plus web bending, C_D,B = E t³ / (4 (1 − ν²) h_d) per unit length.
pub fn c_d_web_bending(e: f64, t: f64, hd: f64) -> f64 {
    e * t.powi(3) / (4.0 * (1.0 - NU_STEEL * NU_STEEL) * hd)
}

/// 🥫️ Average yield strength `f_ya = f_yb + (f_u - f_yb) k n t^2 / A_g` limited to `(f_u + f_yb) / 2` (EN 1993-1-3 3.2.2); any consistent length unit for `t` and `a_g`.
pub fn average_yield(fyb: f64, fu: f64, k: f64, n: f64, t: f64, a_g: f64) -> f64 {
    let gain = if a_g > 0.0 { (fu - fyb) * k * n * t * t / a_g } else { 0.0 };
    (fyb + gain).min((fu + fyb) / 2.0)
}
