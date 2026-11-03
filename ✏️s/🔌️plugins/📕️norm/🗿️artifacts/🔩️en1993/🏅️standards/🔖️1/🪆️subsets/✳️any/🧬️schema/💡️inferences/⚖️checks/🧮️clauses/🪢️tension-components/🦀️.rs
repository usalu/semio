//! 🪢️ EN 1993-1-11 cables, rods and pin joints (mm, N, MPa). See [EN 1993-1-11](https://eurocodes.jrc.ec.europa.eu/).

/// 🪢️ Minimum breaking force of a rope F_min = K d² R_r (EN 12385), K the breaking force factor, d in mm, R_r in MPa, result in N.
pub fn rope_min_breaking(k: f64, d: f64, r_r: f64) -> f64 {
    k * d * d * r_r
}

/// 🪢️ Characteristic breaking force of a strand F_uk = Σ A f_uk reduced by the spinning loss factor.
pub fn strand_breaking(area_metallic: f64, f_uk: f64, spinning_factor: f64) -> f64 {
    area_metallic * f_uk * spinning_factor
}

/// 🪢️ Design resistance F_Rd = F_uk/γR with γR = 1.5 (§6.2).
pub fn cable_f_rd(f_uk: f64, gamma_r: f64) -> f64 {
    f_uk / gamma_r
}

/// 🪢️ Tension utilisation F_Ed/F_Rd.
pub fn cable_utilisation(f_ed: f64, f_uk: f64, gamma_r: f64) -> f64 {
    f_ed / cable_f_rd(f_uk, gamma_r)
}

/// 🪢️ Ernst equivalent modulus E_eff = E/(1 + (γ L)² E/(12 σ³)) with γ the specific weight in N/mm³ and L the horizontal projection.
pub fn ernst_modulus(e: f64, gamma: f64, l: f64, sigma: f64) -> f64 {
    e / (1.0 + (gamma * l).powi(2) * e / (12.0 * sigma.powi(3)))
}

/// 🪢️ Parabolic sag f = w L²/(8 H) for line load w and horizontal force H.
pub fn cable_sag(w: f64, l: f64, h: f64) -> f64 {
    w * l * l / (8.0 * h)
}

/// 🪢️ Resistance of a threaded rod N_t,Rd = k_t A_s f_u/γM2 with k_t = 0.9 (cut thread).
pub fn rod_tension_rd(k_t: f64, a_s: f64, fu: f64, gm2: f64) -> f64 {
    k_t * a_s * fu / gm2
}

/// 🪢️ Pin shear resistance F_v,Rd = 0.6 A f_up/γM2 (EN 1993-1-8 Table 3.10).
pub fn pin_shear_rd(a: f64, fup: f64, gm2: f64) -> f64 {
    0.6 * a * fup / gm2
}

/// 🪢️ Bearing resistance of plate and pin F_b,Rd = 1.5 t d f_y/γM0, serviceability 0.6 t d f_y/γM6,ser.
pub fn pin_bearing_rd(t: f64, d: f64, fy: f64, gm0: f64) -> f64 {
    1.5 * t * d * fy / gm0
}

pub fn pin_bearing_ser(t: f64, d: f64, fy: f64, gm6_ser: f64) -> f64 {
    0.6 * t * d * fy / gm6_ser
}

/// 🪢️ Pin bending moment M_Ed = F_Ed (b + 4c + 2a)/8 and resistance M_Rd = 1.5 W_el f_yp/γM0.
pub fn pin_moment_ed(f_ed: f64, a: f64, b: f64, c: f64) -> f64 {
    f_ed * (b + 4.0 * c + 2.0 * a) / 8.0
}

/// 🪢️ Pin bending moment from the plate layout of a pin joint: F_Ed (b + 4c + 2a)/8 with the middle (inner) plate thickness `b`, the gap `c` between the plates and the outer plate thickness `a` (EN 1993-1-8 Table 3.10: the middle plate loads the pin in the centre, each outer plate carries F_Ed/2 over its thickness).
pub fn pin_moment_of_plates(f_ed: f64, outer: f64, inner: f64, gap: f64) -> f64 {
    pin_moment_ed(f_ed, outer, inner, gap)
}

pub fn pin_moment_rd(w_el: f64, fyp: f64, gm0: f64) -> f64 {
    1.5 * w_el * fyp / gm0
}

/// 🪢️ Combined pin utilisation (M_Ed/M_Rd)² + (F_v,Ed/F_v,Rd)² ≤ 1.
pub fn pin_combined(m_ed: f64, m_rd: f64, fv_ed: f64, fv_rd: f64) -> f64 {
    (m_ed / m_rd).powi(2) + (fv_ed / fv_rd).powi(2)
}
