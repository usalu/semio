//! 🪟️ EN 1993-1-5 §4.4 effective widths of class 4 plate elements (mm, MPa). See [EN 1993-1-5 Table 4.1/4.2](https://eurocodes.jrc.ec.europa.eu/).

pub fn k_sigma_internal(psi: f64) -> f64 {
    if psi >= 1.0 {
        4.0
    } else if psi > 0.0 {
        8.2 / (1.05 + psi)
    } else if psi == 0.0 {
        7.81
    } else if psi > -1.0 {
        7.81 - 6.29 * psi + 9.78 * psi * psi
    } else if psi == -1.0 {
        23.9
    } else {
        5.98 * (1.0 - psi) * (1.0 - psi)
    }
}

pub fn k_sigma_outstand(psi: f64, tip_compression: bool) -> f64 {
    if tip_compression {
        crate::classification::k_sigma_outstand_tip_compression(psi)
    } else {
        0.57 - 0.21 * psi + 0.07 * psi * psi
    }
}

/// 🪟️ Plate slenderness λ̄_p = (b̄/t)/(28.4 ε √k_σ).
pub fn lambda_p(b_bar: f64, t: f64, eps: f64, k_sigma: f64) -> f64 {
    (b_bar / t) / (28.4 * eps * k_sigma.sqrt())
}

/// 🪟️ ρ for an internal element including the ψ term (Eq. 4.2).
pub fn rho_internal(lambda_p: f64, psi: f64) -> f64 {
    if lambda_p <= 0.5 + (0.085 - 0.055 * psi).max(0.0).sqrt() {
        1.0
    } else {
        ((lambda_p - 0.055 * (3.0 + psi)) / (lambda_p * lambda_p)).min(1.0)
    }
}

/// 🪟️ ρ for an outstand element (Eq. 4.3).
pub fn rho_outstand(lambda_p: f64) -> f64 {
    if lambda_p <= 0.748 {
        1.0
    } else {
        ((lambda_p - 0.188) / (lambda_p * lambda_p)).min(1.0)
    }
}

pub struct EffectiveInternal {
    pub rho: f64,
    pub lambda_p: f64,
    pub k_sigma: f64,
    pub b_eff: f64,
    pub b_e1: f64,
    pub b_e2: f64,
}

/// 🪟️ Effective width of an internal compression element (Table 4.1).
pub fn effective_internal(b_bar: f64, t: f64, fy: f64, psi: f64) -> EffectiveInternal {
    let eps = (235.0 / fy).sqrt();
    let k = k_sigma_internal(psi);
    let lp = lambda_p(b_bar, t, eps, k);
    let rho = rho_internal(lp, psi);
    let (b_eff, e1, e2) = if psi >= 1.0 {
        let be = rho * b_bar;
        (be, 0.5 * be, 0.5 * be)
    } else if psi >= 0.0 {
        let be = rho * b_bar;
        let e1 = 2.0 / (5.0 - psi) * be;
        (be, e1, be - e1)
    } else {
        let bc = b_bar / (1.0 - psi);
        let be = rho * bc;
        (be, 0.4 * be, 0.6 * be)
    };
    EffectiveInternal { rho, lambda_p: lp, k_sigma: k, b_eff, b_e1: e1, b_e2: e2 }
}

/// 🪟️ Effective width of an outstand element.
pub fn effective_outstand(c: f64, t: f64, fy: f64, psi: f64, tip_compression: bool) -> (f64, f64) {
    let eps = (235.0 / fy).sqrt();
    let lp = lambda_p(c, t, eps, k_sigma_outstand(psi, tip_compression));
    let rho = rho_outstand(lp);
    (rho, rho * c)
}

/// 🪟️ ρ with stress-dependent slenderness λ̄_p,red = λ̄_p √(σ_com,Ed/(fy/γM0)) (EN 1993-1-5 §4.4(4)).
pub fn lambda_p_reduced(lambda_p: f64, sigma_com_ed: f64, fy: f64, gamma_m0: f64) -> f64 {
    lambda_p * (sigma_com_ed / (fy / gamma_m0)).max(0.0).sqrt()
}

/// 🪟️ Shift e_N of the effective centroid under uniform compression from the lost strips `(ΔA, z)` measured from the gross centroid: e_N = −Σ ΔA z/A_eff (EN 1993-1-5 §4.3(3)).
pub fn compression_centroid_shift(a_gross: f64, lost: &[(f64, f64)]) -> f64 {
    let lost_area: f64 = lost.iter().map(|(a, _)| a).sum();
    -lost.iter().map(|(a, z)| a * z).sum::<f64>() / (a_gross - lost_area)
}

/// 🪟️ e_N of an I section in uniform compression: outstand flanges ρ_f and the internal web ρ_w lose their strips symmetrically, so the shift vanishes unless the flanges differ.
pub fn i_section_compression_shift(h: f64, b_top: f64, b_bottom: f64, tw: f64, tf: f64, r: f64, fy: f64) -> f64 {
    let eps = (235.0 / fy).sqrt();
    let flange_loss = |b: f64| {
        let c = ((b - tw - 2.0 * r) / 2.0).max(1e-6);
        2.0 * c * tf * (1.0 - rho_outstand(lambda_p(c, tf, eps, 0.43)))
    };
    let c_w = (h - 2.0 * tf - 2.0 * r).max(1e-6);
    let web_loss = c_w * tw * (1.0 - rho_internal(lambda_p(c_w, tw, eps, 4.0), 1.0).max(0.0));
    let z_f = (h - tf) / 2.0;
    let area = (b_top + b_bottom) * tf + (h - 2.0 * tf) * tw;
    let z_g = (b_top - b_bottom) * tf * z_f / area;
    compression_centroid_shift(area, &[(flange_loss(b_top), z_f - z_g), (flange_loss(b_bottom), -z_f - z_g), (web_loss, -z_g)])
}

pub struct IEffective {
    pub a_eff: f64,
    pub w_eff_y: f64,
    pub e_n: f64,
    pub web_rho: f64,
    pub flange_rho: f64,
}

/// 🪟️ Effective properties of a doubly symmetric welded I under pure bending about y (web ψ from the gross section), iterating the stress ratio.
pub fn i_section_bending(h: f64, b: f64, tw: f64, tf: f64, fy: f64) -> IEffective {
    let hw = h - 2.0 * tf;
    let c = (b - tw) / 2.0;
    let eps = (235.0 / fy).sqrt();
    let flange = rho_outstand(lambda_p(c, tf, eps, k_sigma_outstand(1.0, true)));
    let web = effective_internal(hw, tw, fy, -1.0);
    let bc = hw / 2.0;
    let web_eff_top = web.b_e1;
    let web_eff_bot_comp = web.b_e2;
    let a_flange_t = (tw + 2.0 * flange * c) * tf;
    let a_flange_b = b * tf;
    let a_web_t = web_eff_top * tw;
    let a_web_c = web_eff_bot_comp * tw;
    let a_web_tension = bc * tw;
    let z_top = h / 2.0 - tf / 2.0;
    let z_wt = bc - web_eff_top / 2.0;
    let z_wc = web_eff_bot_comp / 2.0;
    let z_tension = -bc / 2.0;
    let a_eff = a_flange_t + a_flange_b + a_web_t + a_web_c + a_web_tension;
    let s = a_flange_t * z_top - a_flange_b * z_top + a_web_t * z_wt + a_web_c * z_wc + a_web_tension * z_tension;
    let e_n = s / a_eff;
    let i_loc = |a: f64, z: f64, own: f64| own + a * (z - e_n) * (z - e_n);
    let iy = i_loc(a_flange_t, z_top, (tw + 2.0 * flange * c) * tf.powi(3) / 12.0)
        + i_loc(a_flange_b, -z_top, b * tf.powi(3) / 12.0)
        + i_loc(a_web_t, z_wt, tw * web_eff_top.powi(3) / 12.0)
        + i_loc(a_web_c, z_wc, tw * web_eff_bot_comp.powi(3) / 12.0)
        + i_loc(a_web_tension, z_tension, tw * bc.powi(3) / 12.0);
    let z_max = (h / 2.0 - e_n).max(h / 2.0 + e_n);
    IEffective { a_eff, w_eff_y: iy / z_max, e_n, web_rho: web.rho, flange_rho: flange }
}
