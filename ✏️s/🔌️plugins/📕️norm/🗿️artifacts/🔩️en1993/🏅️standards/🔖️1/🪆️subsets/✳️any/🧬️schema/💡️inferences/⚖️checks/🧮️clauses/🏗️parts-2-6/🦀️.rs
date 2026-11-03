//! 🏗️ EN 1993-2 bridge fatigue, 1-11 cables, 3 towers, 5 piling, 6 crane runways, 1-8 base plates (mm, N, MPa). See [EN 1993-6](https://eurocodes.jrc.ec.europa.eu/).

/// 🌉 λ = λ1 λ2 λ3 λ4 ≤ λ_max for the damage-equivalence factor (EN 1993-2 §9.5.2).
pub fn lambda_equivalence(l1: f64, l2: f64, l3: f64, l4: f64, lambda_max: f64) -> f64 {
    (l1 * l2 * l3 * l4).min(lambda_max)
}

/// 🌉 Fatigue verification γFf Δσ_E,2 ≤ Δσ_C/γMf as a utilisation.
pub fn bridge_fatigue_utilisation(delta_e2: f64, delta_c: f64, gamma_ff: f64, gamma_mf: f64) -> f64 {
    gamma_ff * delta_e2 / (delta_c / gamma_mf)
}

/// 🪢 Cable design tensile resistance F_Rd = min(F_uk/γR, F_k) in the EN 1993-1-11 form (γR recommended 1.5).
pub fn cable_resistance(f_uk: f64, f_k: f64, gamma_r: f64) -> f64 {
    (f_uk / gamma_r).min(f_k)
}

/// 🗼 EN 1993-3-1 cylindrical shaft: wind-amplified compression utilisation N_Ed(1 + c_f c_d q-related factor)/(χ A fy/γM1) given the amplification factor.
pub fn tower_leg_utilisation(n_ed: f64, amplification: f64, chi: f64, area: f64, fy: f64, gm1: f64) -> f64 {
    n_ed * amplification / (chi * area * fy / gm1)
}

/// 🪵 EN 1993-5 corroded section: net thickness after loss over the design life, t_net = t − Σ c.
pub fn piling_net_thickness(t: f64, loss_per_face: f64, faces: f64) -> f64 {
    (t - loss_per_face * faces).max(0.0)
}

/// 🪵 EN 1993-5 §5.2.2 class-3 sheet pile bending resistance with β_B reduction for the interlock shear transfer.
pub fn sheet_pile_bending_rd(beta_b: f64, w_el: f64, fy: f64, gm0: f64) -> f64 {
    beta_b * w_el * fy / gm0
}

/// 🏗️ Crane runway: effective loaded length l_eff = 3.25 (I_rf/t_w)^(1/3) (EN 1993-6 §5.7.1).
pub fn crane_l_eff(i_rf: f64, tw: f64) -> f64 {
    3.25 * (i_rf / tw).cbrt()
}

/// 🏗️ Local vertical stress σ_oz,Ed = F_z,Ed/(l_eff t_w).
pub fn crane_sigma_oz(fz_ed: f64, l_eff: f64, tw: f64) -> f64 {
    fz_ed / (l_eff * tw)
}

/// 🏗️ Base plate (EN 1993-1-8 §6.2.5) bearing strength f_jd = β_j k_j f_cd with k_j = 1 for the simplified foundation.
pub fn base_plate_fjd(beta_j: f64, kj: f64, fcd: f64) -> f64 {
    beta_j * kj * fcd
}

/// 🏗️ Additional bearing width c = t √(fy/(3 f_jd γM0)).
pub fn base_plate_c(t: f64, fy: f64, fjd: f64, gm0: f64) -> f64 {
    t * (fy / (3.0 * fjd * gm0)).sqrt()
}

/// 🏗️ Base plate compression resistance of one T-stub, F_c,Rd = f_jd b_eff l_eff.
pub fn base_plate_fc_rd(fjd: f64, b_eff: f64, l_eff: f64) -> f64 {
    fjd * b_eff * l_eff
}
