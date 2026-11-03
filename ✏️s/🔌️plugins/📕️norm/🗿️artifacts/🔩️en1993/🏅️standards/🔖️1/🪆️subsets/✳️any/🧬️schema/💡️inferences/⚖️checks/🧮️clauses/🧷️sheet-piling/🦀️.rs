//! 🧷️ EN 1993-5 sheet piles: corrosion allowance, interlock-reduced section properties, shear and moment (mm, N, MPa). See [EN 1993-5](https://eurocodes.jrc.ec.europa.eu/).

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Environment {
    UndisturbedSoil,
    PollutedSoil,
    CompactedFill,
    NonCompactedFill,
}

/// 🧷️ Corrosion loss per face in mm at 5, 25, 50, 75 and 100 years (Table 4-1, unverified, from memory).
pub fn corrosion_loss_table(env: Environment) -> [f64; 5] {
    match env {
        Environment::UndisturbedSoil => [0.00, 0.30, 0.60, 0.90, 1.20],
        Environment::PollutedSoil => [0.15, 0.75, 1.50, 2.25, 3.00],
        Environment::CompactedFill => [0.18, 0.70, 1.20, 1.70, 2.20],
        Environment::NonCompactedFill => [0.50, 1.75, 3.25, 4.75, 6.25],
    }
}

/// 🧷️ Interpolated corrosion loss for a design life in years (linear from the origin up to 5 years).
pub fn corrosion_loss(env: Environment, years: f64) -> f64 {
    let t = corrosion_loss_table(env);
    let y = [5.0, 25.0, 50.0, 75.0, 100.0];
    if years <= y[0] {
        return t[0] * years / y[0];
    }
    for i in 0..4 {
        if years <= y[i + 1] {
            return t[i] + (t[i + 1] - t[i]) * (years - y[i]) / (y[i + 1] - y[i]);
        }
    }
    t[4]
}

/// 🧷️ Section properties reduced for interlock slip, W_eff = β_B W_el and I_eff = β_D I (§5.2.2).
pub fn reduced_properties(beta_b: f64, beta_d: f64, w_el: f64, i: f64) -> (f64, f64) {
    (beta_b * w_el, beta_d * i)
}

/// 🧷️ Elastic moment resistance of a class 3 pile M_c,Rd = β_B W_el f_y/γM0 with the lower-bound interlock factor.
pub fn m_c_rd_pile(beta_b: f64, w_el: f64, fy: f64, gm0: f64) -> f64 {
    beta_b * w_el * fy / gm0
}

/// 🧷️ Plastic shear V_pl,Rd = A_v f_y/(√3 γM0).
pub fn v_pl_rd_pile(av: f64, fy: f64, gm0: f64) -> f64 {
    av * fy / (3.0_f64.sqrt() * gm0)
}

/// 🧷️ Reduced yield strength under high shear, f_y (1 − ρ) with ρ = (2 V_Ed/V_pl,Rd − 1)² for V_Ed > 0.5 V_pl,Rd (§5.2.2(7)).
pub fn fy_reduced_by_shear(v_ed: f64, v_pl_rd: f64, fy: f64) -> f64 {
    if v_ed <= 0.5 * v_pl_rd { fy } else { fy * (1.0 - (2.0 * v_ed / v_pl_rd - 1.0).powi(2)) }
}

/// 🧷️ Interlock shear utilisation V_L,Ed/V_L,Rd with V_L,Rd = β_D f_y t/(√3 γM0) per unit length.
pub fn interlock_shear_utilisation(v_l_ed: f64, beta_d: f64, fy: f64, t: f64, gm0: f64) -> f64 {
    v_l_ed / (beta_d * fy * t / (3.0_f64.sqrt() * gm0))
}

/// 🧷️ Net thickness after corrosion on both faces.
pub fn net_thickness(t: f64, loss_face_1: f64, loss_face_2: f64) -> f64 {
    (t - loss_face_1 - loss_face_2).max(0.0)
}
