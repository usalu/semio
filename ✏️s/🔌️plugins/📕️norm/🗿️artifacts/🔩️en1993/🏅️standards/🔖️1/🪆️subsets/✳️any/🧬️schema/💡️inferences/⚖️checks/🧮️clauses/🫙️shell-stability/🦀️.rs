//! 🫙️ EN 1993-1-6 shell buckling LS3 and LS1 with fabrication quality classes for unstiffened cylinders (mm, N, MPa). See [EN 1993-1-6](https://eurocodes.jrc.ec.europa.eu/).

fn e_shell() -> f64 {
    semio_s_plugin_norm_compliance::ndp_common("ndp.en1993.1-1.e-modulus")
}

/// 🫙️ Reciprocal of the thin-shell limit t/r < 1/20 that gates the shell checks LS1 to LS4 (spec V-906).
pub const THIN_SHELL_LIMIT_INVERSE: f64 = 20.0;

/// 🫙️ Wall slenderness t/r of a shell.
pub fn thin_shell_ratio(t: f64, r: f64) -> f64 {
    t / r
}

/// 🫙️ Fabrication quality parameter Q of the tolerance classes A (excellent) 40, B (high) 25, C (normal) 16 (Annex D.1.2.2).
pub fn fabrication_q(class: char) -> f64 {
    match class {
        'A' => 40.0,
        'B' => 25.0,
        _ => 16.0,
    }
}

/// 🫙️ Dimensionless length ω = l/√(r t).
pub fn omega(l: f64, r: f64, t: f64) -> f64 {
    l / (r * t).sqrt()
}

/// 🫙️ Length factor C_x for axial compression: short 1.36 − 1.83/ω + 2.07/ω², medium 1, long 1 + 0.2/C_xb (1 − 2ω t/r) ≥ 0.6 (D.1.2.1).
pub fn c_x(omega: f64, r: f64, t: f64, c_xb: f64) -> f64 {
    if omega < 1.7 { 1.36 - 1.83 / omega + 2.07 / (omega * omega) } else if omega <= 0.5 * r / t { 1.0 } else { (1.0 + 0.2 / c_xb * (1.0 - 2.0 * omega * t / r)).max(0.6) }
}

/// 🫙️ σ_x,Rcr = 0.605 E C_x t/r.
pub fn sigma_x_rcr(c_x: f64, t: f64, r: f64) -> f64 {
    0.605 * e_shell() * c_x * t / r
}

/// 🫙️ σ_θ,Rcr = 0.92 E C_θ/ω (t/r) (D.1.3.1).
pub fn sigma_theta_rcr(c_theta: f64, omega: f64, t: f64, r: f64) -> f64 {
    0.92 * e_shell() * c_theta / omega * t / r
}

/// 🫙️ τ_xθ,Rcr = 0.75 E C_τ √(1/ω) (t/r) (D.1.4.1).
pub fn tau_rcr(c_tau: f64, omega: f64, t: f64, r: f64) -> f64 {
    0.75 * e_shell() * c_tau * (1.0 / omega).sqrt() * t / r
}

/// 🫙️ Circumferential imperfection factor α_θ per quality class (unverified: 0.75, 0.65, 0.50).
pub fn alpha_theta(class: char) -> f64 {
    match class {
        'A' => 0.75,
        'B' => 0.65,
        _ => 0.50,
    }
}

/// 🫙️ Shear imperfection factor α_τ (unverified: 0.8).
pub fn alpha_tau() -> f64 {
    0.8
}

/// 🫙️ Meridional factor α_x for fabrication parameter Q, α = 0.62/(1 + 1.91 (Δw_k/t)^1.44)^0.6 with Δw_k = √(r/t) t/Q.
pub fn alpha_x(r: f64, t: f64, q: f64) -> f64 {
    crate::plates_shells::cylinder_alpha_x(r, t, q)
}

/// 🫙️ Reference resistance factor χ(λ̄) = 1, 1 − β ((λ̄ − λ̄0)/(λ̄p − λ̄0))^η, α/λ̄² with λ̄p = √(α/(1 − β)) (§8.5.2).
pub fn chi_shell(lambda: f64, alpha: f64, beta: f64, eta: f64, lambda0: f64) -> f64 {
    let lp = (alpha / (1.0 - beta)).sqrt();
    if lambda <= lambda0 { 1.0 } else if lambda < lp { 1.0 - beta * ((lambda - lambda0) / (lp - lambda0)).powf(eta) } else { alpha / (lambda * lambda) }
}

/// 🫙️ Relative slenderness λ̄ = √(R_pl/R_cr).
pub fn lambda_shell(r_pl: f64, r_cr: f64) -> f64 {
    (r_pl / r_cr).sqrt()
}

pub struct ShellStress {
    pub sigma_cr: f64,
    pub lambda: f64,
    pub chi: f64,
    pub sigma_rd: f64,
}

/// 🫙️ Design buckling stress for axial compression of an unstiffened cylinder with length factor C_x.
pub fn axial_design(l: f64, r: f64, t: f64, fy: f64, class: char, c_xb: f64, gm1: f64) -> ShellStress {
    let om = omega(l, r, t);
    let cr = sigma_x_rcr(c_x(om, r, t, c_xb), t, r);
    let lam = lambda_shell(fy, cr);
    let chi = chi_shell(lam, alpha_x(r, t, fabrication_q(class)), 0.6, 1.0, 0.2);
    ShellStress { sigma_cr: cr, lambda: lam, chi, sigma_rd: chi * fy / gm1 }
}

/// 🫙️ Design buckling stress for circumferential compression (external pressure), λ̄0 = 0.4, β = 0.6, η = 1.
pub fn hoop_design(l: f64, r: f64, t: f64, fy: f64, class: char, c_theta: f64, gm1: f64) -> ShellStress {
    let cr = sigma_theta_rcr(c_theta, omega(l, r, t), t, r);
    let lam = lambda_shell(fy, cr);
    let chi = chi_shell(lam, alpha_theta(class), 0.6, 1.0, 0.4);
    ShellStress { sigma_cr: cr, lambda: lam, chi, sigma_rd: chi * fy / gm1 }
}

/// 🫙️ Design buckling shear stress τ_Rd = χ_τ f_y/(√3 γM1), λ̄0 = 0.4, β = 0.6, η = 1.
pub fn shear_design(l: f64, r: f64, t: f64, fy: f64, c_tau: f64, gm1: f64) -> ShellStress {
    let tau_pl = fy / 3.0_f64.sqrt();
    let cr = tau_rcr(c_tau, omega(l, r, t), t, r);
    let lam = lambda_shell(tau_pl, cr);
    let chi = chi_shell(lam, alpha_tau(), 0.6, 1.0, 0.4);
    ShellStress { sigma_cr: cr, lambda: lam, chi, sigma_rd: chi * tau_pl / gm1 }
}

/// 🫙️ Combined meridional, circumferential and shear utilisation (§8.5.3, Eq. D.?): (σx/σxRd)^kx − ki (σx/σxRd)(σθ/σθRd) + (σθ/σθRd)^kθ + (τ/τRd)^kτ with the exponents from the χ factors.
pub fn interaction_ls3(sx: f64, sx_rd: f64, chi_x: f64, st: f64, st_rd: f64, chi_t: f64, tau: f64, tau_rd: f64, chi_tau: f64) -> f64 {
    let (kx, kt, kta) = (1.25 + 0.75 * chi_x, 1.25 + 0.75 * chi_t, 1.75 + 0.25 * chi_tau);
    let ki = (chi_x * chi_t).powi(2);
    let (a, b, c) = (sx / sx_rd, st / st_rd, tau / tau_rd);
    a.abs().powf(kx) - ki * a * b + b.abs().powf(kt) + c.abs().powf(kta)
}

/// 🫙️ LS1 membrane yield utilisation of a cylinder under hoop and axial stress by von Mises (§8.5 plastic reference).
pub fn ls1_von_mises(sx: f64, st: f64, tau: f64, fy: f64, gm0: f64) -> f64 {
    ((sx * sx - sx * st + st * st + 3.0 * tau * tau).sqrt()) / (fy / gm0)
}
