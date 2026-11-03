//! 🏺️ EN 1993-4-1 silos, 4-2 tanks and 4-3 pipelines: wall loads and wall thickness (mm, N, MPa). See [EN 1993-4-1](https://eurocodes.jrc.ec.europa.eu/).

/// 🏺️ Horizontal Janssen pressure p_h = γ K z0 (1 − e^(−z/z0)), z0 = A/(μ K U).
pub fn janssen_ph(gamma: f64, k: f64, mu: f64, a: f64, u: f64, z: f64) -> f64 {
    let z0 = a / (mu * k * u);
    gamma * k * z0 * (1.0 - (-z / z0).exp())
}

/// 🏺️ Wall friction traction p_w = μ p_h.
pub fn janssen_pw(mu: f64, ph: f64) -> f64 {
    mu * ph
}

/// 🏺️ Meridional force per unit circumference n_x = μ ∫0^z p_h dz = μ γ K z0 (z − z0 (1 − e^(−z/z0))).
pub fn janssen_nx(gamma: f64, k: f64, mu: f64, a: f64, u: f64, z: f64) -> f64 {
    let z0 = a / (mu * k * u);
    mu * gamma * k * z0 * (z - z0 * (1.0 - (-z / z0).exp()))
}

/// 🏺️ Vertical stress p_v = γ z0 (1 − e^(−z/z0)).
pub fn janssen_pv(gamma: f64, k: f64, mu: f64, a: f64, u: f64, z: f64) -> f64 {
    let z0 = a / (mu * k * u);
    gamma * z0 * (1.0 - (-z / z0).exp())
}

/// 🏺️ Wall thickness of a tank course from the hydrostatic hoop membrane t = ρ g (H − h) R/(f_y/γM0), liquid density in kg/m³, heights and radius in mm (EN 1993-4-2 §6).
pub fn tank_course_thickness(rho: f64, h_total: f64, h_course: f64, r: f64, fy: f64, gm0: f64) -> f64 {
    rho * 9.81 * 1e-9 * (h_total - h_course) * r / (fy / gm0)
}

/// 🏺️ Hoop membrane stress σ_θ = p r/t.
pub fn hoop_membrane(p: f64, r: f64, t: f64) -> f64 {
    p * r / t
}

/// 🏺️ Pipe wall thickness t = p d/(2 f_y/γM0) (Barlow).
pub fn pipe_wall_thickness(p: f64, d: f64, fy: f64, gm0: f64) -> f64 {
    p * d / (2.0 * fy / gm0)
}

/// 🏺️ Axial stress of a fully restrained pipe σ_x = ν σ_θ − E α ΔT.
pub fn restrained_axial_stress(nu: f64, sigma_theta: f64, e: f64, alpha: f64, dt: f64) -> f64 {
    nu * sigma_theta - e * alpha * dt
}

/// 🏺️ Biaxial von Mises utilisation √(σx² − σxσθ + σθ²)/(f_y/γM0).
pub fn pipe_von_mises(sx: f64, st: f64, fy: f64, gm0: f64) -> f64 {
    (sx * sx - sx * st + st * st).sqrt() / (fy / gm0)
}

/// 🏺️ Elastic collapse pressure of a long pipe under external pressure p_cr = 2E/(1 − ν²) (t/d)³.
pub fn pipe_p_cr(e: f64, nu: f64, t: f64, d: f64) -> f64 {
    2.0 * e / (1.0 - nu * nu) * (t / d).powi(3)
}
