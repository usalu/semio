//! 🗼️ EN 1993-3-1 lattice towers and guyed masts and EN 1993-3-2 chimneys (mm, N, MPa, s). See [EN 1993-3-1](https://eurocodes.jrc.ec.europa.eu/).

use std::f64::consts::PI;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LatticeMember {
    Leg,
    Diagonal,
    Redundant,
}

/// 🗼️ Effective slenderness of lattice members (Annex G, simplified, unverified): legs 0.9 λ, diagonals 0.35 + 0.7 λ, redundants 0.7 λ.
pub fn lambda_eff_lattice(kind: LatticeMember, lambda: f64) -> f64 {
    match kind {
        LatticeMember::Leg => 0.9 * lambda,
        LatticeMember::Diagonal => 0.35 + 0.7 * lambda,
        LatticeMember::Redundant => 0.7 * lambda,
    }
}

/// 🗼️ Critical vortex shedding speed v_crit = b n/St.
pub fn v_crit(b: f64, n: f64, st: f64) -> f64 {
    b * n / st
}

/// 🗼️ Scruton number Sc = 2 δ_s m_e/(ρ b²).
pub fn scruton(delta_s: f64, m_e: f64, rho: f64, b: f64) -> f64 {
    2.0 * delta_s * m_e / (rho * b * b)
}

/// 🗼️ Across-wind amplitude y_max/b = (1/St²)(1/Sc) K K_w c_lat (EN 1991-1-4 Annex E).
pub fn y_max_over_b(st: f64, sc: f64, k: f64, kw: f64, c_lat: f64) -> f64 {
    k * kw * c_lat / (st * st * sc)
}

/// 🗼️ Fundamental frequency of a uniform cantilever n1 = (1.875²/(2π L²)) √(E I/m).
pub fn cantilever_frequency(l: f64, ei: f64, m: f64) -> f64 {
    1.875_f64.powi(2) / (2.0 * PI * l * l) * (ei / m).sqrt()
}

/// 🗼️ Number of vortex shedding cycles N = 2 T ν0 (v_crit/v0)² exp(−(v_crit/v0)²) with Weibull-type wind (Annex E.1.5).
pub fn vortex_cycles(t_life: f64, nu0: f64, v_crit: f64, v0: f64) -> f64 {
    let x = (v_crit / v0).powi(2);
    2.0 * t_life * nu0 * x * (-x).exp()
}

/// 🗼️ Ovalling critical wind speed of a shell with ring-bending mode i, v_crit,i = b n_i / St.
pub fn ovalling_v_crit(b: f64, n_i: f64, st: f64) -> f64 {
    b * n_i / st
}

/// 🗼️ Ovalling frequency of a ring segment n_i = (i(i²−1)/√(i²+1)) · (1/(2π r²)) √(E t²/(12 (1 − ν²) ρ)) per unit.
pub fn ovalling_frequency(i: f64, r: f64, t: f64, e: f64, rho_mass: f64) -> f64 {
    i * (i * i - 1.0) / (i * i + 1.0).sqrt() / (2.0 * PI * r * r) * (e * t * t / (12.0 * (1.0 - 0.09) * rho_mass)).sqrt()
}

/// 🗼️ Guyed mast guy stiffness (Ernst) k = E_eff A cos²α / L_guy.
pub fn guy_stiffness(e_eff: f64, a: f64, alpha: f64, l: f64) -> f64 {
    e_eff * a * alpha.cos().powi(2) / l
}
