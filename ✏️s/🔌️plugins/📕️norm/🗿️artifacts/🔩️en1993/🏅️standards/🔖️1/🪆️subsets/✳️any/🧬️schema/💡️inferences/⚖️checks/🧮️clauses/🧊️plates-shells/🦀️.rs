//! 🧊️ EN 1993-1-5 shear lag, reduced stress method and EN 1993-1-6 cylinder buckling (mm, N, MPa). See [EN 1993-1-5 §6](https://eurocodes.jrc.ec.europa.eu/).

/// 🧊️ Shear-lag factor β for sagging bending, κ = α0 b0/L_e (Table 3.1).
pub fn shear_lag_beta(kappa: f64) -> f64 {
    if kappa <= 0.02 { 1.0 } else if kappa <= 0.7 { 1.0 / (1.0 + 6.4 * kappa * kappa) } else { 1.0 / (5.9 * kappa) }
}

/// 🧊️ Reduced stress method utilisation (§10 Eq. 10.5), compression positive.
pub fn reduced_stress_utilisation(sx: f64, sz: f64, tau: f64, rho_x: f64, rho_z: f64, chi_w: f64, fy: f64, gm1: f64) -> f64 {
    let f = fy / gm1;
    let v = if sx > 0.0 && sz > 0.0 { rho_x * rho_z } else { 1.0 };
    (sx / (rho_x * f)).powi(2) + (sz / (rho_z * f)).powi(2) - v * sx * sz / (rho_x * rho_z * f * f) + 3.0 * (tau / (chi_w * f)).powi(2)
}

/// 🛢️ Elastic critical meridional stress of a medium-length cylinder, σ = 0.605 E t/r.
pub fn cylinder_sigma_x_rcr(e: f64, t: f64, r: f64) -> f64 {
    0.605 * e * t / r
}

/// 🛢️ Meridional imperfection factor α_x for fabrication quality parameter Q (Annex D).
pub fn cylinder_alpha_x(r: f64, t: f64, q: f64) -> f64 {
    let dwk = (r / t).sqrt() * t / q;
    0.62 / (1.0 + 1.91 * (dwk / t).powf(1.44)).powf(0.6)
}

/// 🛢️ Meridional buckling reduction χ_x with λ0 = 0.2, β = 0.6, η = 1 (Annex D.1.2.2).
pub fn cylinder_chi_x(lambda: f64, alpha: f64) -> f64 {
    let (l0, beta) = (0.2, 0.6);
    let lp = (alpha / (1.0 - beta)).sqrt();
    if lambda <= l0 { 1.0 } else if lambda < lp { 1.0 - beta * (lambda - l0) / (lp - l0) } else { alpha / (lambda * lambda) }
}

/// 🛢️ Design meridional buckling stress σ_x,Rd = χ fy/γM1 for a cylinder.
pub fn cylinder_sigma_x_rd(e: f64, fy: f64, t: f64, r: f64, q: f64, gm1: f64) -> f64 {
    let lam = (fy / cylinder_sigma_x_rcr(e, t, r)).sqrt();
    cylinder_chi_x(lam, cylinder_alpha_x(r, t, q)) * fy / gm1
}

/// 🛢️ Janssen horizontal pressure p_h = γ K z_0 (1 − e^(−z/z_0)) with z_0 = A/(μ K U).
pub fn janssen_ph(gamma: f64, k: f64, mu: f64, area: f64, perimeter: f64, depth: f64) -> f64 {
    let z0 = area / (mu * k * perimeter);
    gamma * z0 * (1.0 - (-depth / z0).exp()) * k
}

/// 🛢️ Hoop membrane stress σ_θ = p r/t.
pub fn hoop_stress(p: f64, r: f64, t: f64) -> f64 {
    p * r / t
}
