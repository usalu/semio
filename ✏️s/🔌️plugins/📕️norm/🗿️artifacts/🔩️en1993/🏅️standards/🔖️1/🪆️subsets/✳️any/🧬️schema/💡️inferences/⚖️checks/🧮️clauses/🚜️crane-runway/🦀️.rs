//! 🚜️ EN 1993-6 crane runway girders: wheel loads, local web stresses, eccentric torsion, Hertz contact, fatigue factors (mm, N, MPa). See [EN 1993-6](https://eurocodes.jrc.ec.europa.eu/).

/// 🚜️ Dynamic factor φ2 = φ2,min + β2 v_h (EN 1991-3 Table 2.4).
pub fn phi_2(phi2_min: f64, beta2: f64, vh: f64) -> f64 {
    phi2_min + beta2 * vh
}

/// 🚜️ Design wheel load F_z,Ed = γ_Q φ Q_r,max with the governing dynamic factor.
pub fn wheel_load_design(q_r_max: f64, phi: f64, gamma_q: f64) -> f64 {
    gamma_q * phi * q_r_max
}

/// 🚜️ I_rf = I_r + I_f,eff for a rail fixed to the flange, I_f,eff for a loose rail with I_f,eff = b_eff t_f³/12 (§5.7.1).
pub fn i_rf(i_r: f64, b_eff: f64, tf: f64, rail_fixed: bool) -> f64 {
    (if rail_fixed { i_r } else { 0.0 }) + b_eff * tf.powi(3) / 12.0
}

/// 🚜️ l_eff = 3.25 (I_rf/t_w)^(1/3).
pub fn l_eff(i_rf: f64, tw: f64) -> f64 {
    3.25 * (i_rf / tw).cbrt()
}

/// 🚜️ σ_oz,Ed = F_z,Ed/(l_eff t_w) and τ_oxz,Ed = 0.2 σ_oz,Ed.
pub fn sigma_oz(fz: f64, l_eff: f64, tw: f64) -> f64 {
    fz / (l_eff * tw)
}

pub fn tau_oxz(sigma_oz: f64) -> f64 {
    0.2 * sigma_oz
}

/// 🚜️ Torsion from the eccentric wheel load T = F_z e and the St. Venant shear τ = T t/I_T.
pub fn wheel_torsion(fz: f64, e: f64) -> f64 {
    fz * e
}

pub fn tau_st_venant(t_ed: f64, t: f64, it: f64) -> f64 {
    t_ed * t / it
}

/// 🚜️ Combined web utilisation √(σx² + σz² − σxσz + 3 τ²)/(f_y/γM0) (EN 1993-1-1 Eq. 6.1).
pub fn web_von_mises(sx: f64, sz: f64, tau: f64, fy: f64, gm0: f64) -> f64 {
    (sx * sx + sz * sz - sx * sz + 3.0 * tau * tau).sqrt() / (fy / gm0)
}

/// 🚜️ Hertz line contact stress σ_H = 0.418 √(F E/(b_eff r)).
pub fn hertz_line_contact(fz: f64, e: f64, b_eff: f64, r: f64) -> f64 {
    0.418 * (fz * e / (b_eff * r)).sqrt()
}

/// 🚜️ Damage-equivalence factor λ = (ν k_Q)^(1/m) from the crane class (EN 1991-3 Annex B).
pub fn lambda_crane(nu: f64, k_q: f64, m: f64) -> f64 {
    (nu * k_q).powf(1.0 / m)
}

/// 🚜️ Fatigue utilisation γ_Ff λ Φ_fat Δσ/(Δσ_C/γ_Mf).
pub fn crane_fatigue_utilisation(lambda: f64, phi_fat: f64, delta: f64, delta_c: f64, gamma_ff: f64, gamma_mf: f64) -> f64 {
    gamma_ff * lambda * phi_fat * delta * gamma_mf / delta_c
}

/// 🚜️ Deflection limit span/ratio.
pub fn deflection_limit(span: f64, ratio: f64) -> f64 {
    span / ratio
}
