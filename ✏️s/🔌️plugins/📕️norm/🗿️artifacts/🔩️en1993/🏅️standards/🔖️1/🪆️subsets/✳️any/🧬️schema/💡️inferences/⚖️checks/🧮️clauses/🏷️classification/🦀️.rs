//! 🏷️ EN 1993-1-1 Table 5.2 classification of compression parts (mm, N, MPa). See [Table 5.2](https://eurocodes.jrc.ec.europa.eu/).

pub fn epsilon(fy: f64) -> f64 {
    (235.0 / fy).sqrt()
}

pub fn epsilon_stainless(fy: f64, e: f64) -> f64 {
    ((235.0 / fy) * (e / crate::stability::e_steel())).sqrt()
}

/// 📏️ Plastic neutral axis fraction α of an internal part under N_Ed and bending, clipped to [0,1].
pub fn alpha_internal_web(n_ed: f64, c: f64, t: f64, fy: f64) -> f64 {
    (0.5 * (1.0 + n_ed / (c * t * fy))).clamp(0.0, 1.0)
}

/// 🏷️ Class of an internal compression part by stress ratio ψ and plastic fraction α (sheet 1).
pub fn internal_part_class(c_over_t: f64, eps: f64, psi: f64, alpha: f64) -> u8 {
    let (l1, l2) = if alpha >= 1.0 - 1e-12 {
        (33.0 * eps, 38.0 * eps)
    } else if alpha > 0.5 {
        (396.0 * eps / (13.0 * alpha - 1.0), 456.0 * eps / (13.0 * alpha - 1.0))
    } else {
        (36.0 * eps / alpha.max(1e-9), 41.5 * eps / alpha.max(1e-9))
    };
    let l3 = if psi > -1.0 { 42.0 * eps / (0.67 + 0.33 * psi) } else { 62.0 * eps * (1.0 - psi) * (-psi).sqrt() };
    if c_over_t <= l1 {
        1
    } else if c_over_t <= l2 {
        2
    } else if c_over_t <= l3 {
        3
    } else {
        4
    }
}

/// 🏷️ k_σ of EN 1993-1-5 Table 4.2 for an outstand with its tip in compression, ψ in (-3,1].
pub fn k_sigma_outstand_tip_compression(psi: f64) -> f64 {
    if psi >= 1.0 {
        0.43
    } else if psi > 0.0 {
        0.578 / (psi + 0.34)
    } else if psi == 0.0 {
        1.70
    } else if psi > -1.0 {
        1.7 - 5.0 * psi + 17.1 * psi * psi
    } else {
        23.8
    }
}

/// 🏷️ Class of an outstand flange (sheet 2): `tip_compression` selects the stress-distribution rows.
pub fn outstand_class(c_over_t: f64, eps: f64, alpha: f64, psi: f64, tip_compression: bool) -> u8 {
    let a = alpha.clamp(1e-9, 1.0);
    let (l1, l2) = if tip_compression {
        (9.0 * eps / a, 10.0 * eps / a)
    } else {
        (9.0 * eps / (a * a.sqrt()), 10.0 * eps / (a * a.sqrt()))
    };
    let k = if tip_compression { k_sigma_outstand_tip_compression(psi) } else { 0.57 - 0.21 * psi + 0.07 * psi * psi };
    let l3 = if (alpha - 1.0).abs() < 1e-12 && (psi - 1.0).abs() < 1e-12 {
        14.0 * eps
    } else {
        21.0 * eps * k.sqrt()
    };
    if c_over_t <= l1 {
        1
    } else if c_over_t <= l2 {
        2
    } else if c_over_t <= l3 {
        3
    } else {
        4
    }
}

/// 🏷️ Class of a circular hollow section by d/t (sheet 3).
pub fn chs_class(d_over_t: f64, eps: f64) -> u8 {
    let e2 = eps * eps;
    if d_over_t <= 50.0 * e2 {
        1
    } else if d_over_t <= 70.0 * e2 {
        2
    } else if d_over_t <= 90.0 * e2 {
        3
    } else {
        4
    }
}

/// 🏷️ Class 3 test of an angle in compression (sheet 3): h/t and (b+h)/2t limits; otherwise class 4.
pub fn angle_class(h: f64, b: f64, t: f64, eps: f64) -> u8 {
    if h / t <= 15.0 * eps && (b + h) / (2.0 * t) <= 11.5 * eps {
        3
    } else {
        4
    }
}

pub struct Geometry {
    pub h: f64,
    pub b: f64,
    pub tw: f64,
    pub tf: f64,
    pub r: f64,
    pub weld_throat: f64,
}

pub struct PartClasses {
    pub flange: u8,
    pub web: u8,
    pub section: u8,
    pub c_flange: f64,
    pub c_web: f64,
    pub alpha_web: f64,
}

/// 🏷️ Classification of an I/H section under N_Ed and about-y bending (`bending_psi` = web ψ, 1 = pure compression stress).
pub fn classify_i_section(g: &Geometry, fy: f64, n_ed: f64, web_psi: f64, flange_tip_compression: bool) -> PartClasses {
    let eps = epsilon(fy);
    let welded = g.weld_throat > 0.0;
    let (c_f, c_w) = if welded {
        ((g.b - g.tw - 2.0 * std::f64::consts::SQRT_2 * g.weld_throat) / 2.0, g.h - 2.0 * g.tf - 2.0 * std::f64::consts::SQRT_2 * g.weld_throat)
    } else {
        ((g.b - g.tw - 2.0 * g.r) / 2.0, g.h - 2.0 * g.tf - 2.0 * g.r)
    };
    let alpha = if n_ed.abs() < 1e-9 && web_psi <= -1.0 + 1e-12 { 0.5 } else { alpha_internal_web(n_ed, c_w, g.tw, fy) };
    let web = internal_part_class(c_w / g.tw, eps, web_psi, if n_ed >= c_w * g.tw * fy - 1e-9 { 1.0 } else { alpha });
    let flange = outstand_class(c_f / g.tf, eps, 1.0, 1.0, flange_tip_compression);
    PartClasses { flange, web, section: flange.max(web), c_flange: c_f, c_web: c_w, alpha_web: alpha }
}

/// 🏷️ Class of a rectangular hollow section (internal parts c = b − 3t, h − 3t): uniform compression of the flanges, web ψ and α from N and bending.
pub fn classify_rhs(h: f64, b: f64, t: f64, fy: f64, web_psi: f64, web_alpha: f64) -> u8 {
    let eps = epsilon(fy);
    let flange = internal_part_class((b - 3.0 * t) / t, eps, 1.0, 1.0);
    let web = internal_part_class((h - 3.0 * t) / t, eps, web_psi, web_alpha);
    flange.max(web)
}
