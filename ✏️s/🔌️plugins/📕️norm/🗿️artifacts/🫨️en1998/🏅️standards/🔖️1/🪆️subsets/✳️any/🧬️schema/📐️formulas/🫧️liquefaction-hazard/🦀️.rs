//! 🫧 Consequences of liquefaction (EN 1998-5 4.1.4 and 5.4.2): the excess pore pressure ratio r_u of a layer from its factor of safety, the screening of lateral spreading by the ground slope, the liquefied thickness of a soil profile and the loss of lateral support of a pile in liquefied soil (buckling load and bending moment over the unsupported length). The constants are unverified registry rows, the correlations are textbook bases outside EN 1998-5. See [`super`].

use super::part_5::SoilProfile;

/// 🫧 Excess pore pressure ratio r_u = 0.5 + asin(2 F_L^(−1/(b α)) − 1)/π of Seed et al. (1976) with N/N_L = F_L^(−1/b); 1 at F_L ≤ 1 (liquefied, σ'_v = 0).
pub fn excess_pore_pressure_ratio(safety_factor: f64, alpha: f64, cycle_exponent: f64) -> f64 {
    if !(safety_factor > 1.0) {
        return if safety_factor.is_nan() { f64::NAN } else { 1.0 };
    }
    let cycles = safety_factor.powf(-1.0 / (cycle_exponent * alpha));
    0.5 + (2.0 * cycles - 1.0).clamp(-1.0, 1.0).asin() / std::f64::consts::PI
}

/// 📐 Ground slope S [%] = 100 tan β of a slope with the angle β [°].
pub fn slope_percent(angle_deg: f64) -> f64 {
    100.0 * angle_deg.to_radians().tan()
}

/// 🌊 How the ground slope classifies lateral spreading of liquefied ground.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spreading {
    Level,
    Gentle,
    Steep,
}

/// 🌊 Level ground below `min_percent` (oscillation only), gently sloping ground from `min_percent` to `max_percent` (lateral spreading possible), steeper ground beyond (flow failure, slope stability governs).
pub fn spreading_class(slope_percent: f64, min_percent: f64, max_percent: f64) -> Spreading {
    if slope_percent < min_percent { Spreading::Level } else if slope_percent <= max_percent { Spreading::Gentle } else { Spreading::Steep }
}

/// 🪨 One layer of a profile that liquefies: index, top and bottom depth [m] and its smallest factor of safety.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LiquefiedLayer {
    pub index: usize,
    pub top_m: f64,
    pub bottom_m: f64,
    pub safety_factor: f64,
}

/// 🪨 The assessed layers of the profile whose smallest F_L is below the required factor, top to bottom.
pub fn liquefied_layers(profile: &SoilProfile, required_factor: f64) -> Vec<LiquefiedLayer> {
    let bottoms = profile.bottoms();
    (0..profile.layers.len())
        .filter(|i| profile.layers[*i].n1_60 > 0.0)
        .map(|i| (i, profile.layer_governing(i).1))
        .filter(|(_, f_l)| f_l.is_finite() && *f_l < required_factor)
        .map(|(index, safety_factor)| LiquefiedLayer { index, top_m: if index == 0 { 0.0 } else { bottoms[index - 1] }, bottom_m: bottoms[index], safety_factor })
        .collect()
}

/// 🪨 Total thickness of the liquefied layers [m].
pub fn liquefied_thickness_m(layers: &[LiquefiedLayer]) -> f64 {
    layers.iter().map(|l| l.bottom_m - l.top_m).sum()
}

/// 🏗️ Effective length L_eff = K (L_u + c/β) of a pile whose soil support is lost over the length L_u: the liquefied length plus the depth to fixity c/β in the competent soil, with the effective length factor K.
pub fn pile_effective_length_m(unsupported_m: f64, fixity_factor: f64, beta_per_m: f64, length_factor: f64) -> f64 {
    length_factor * (unsupported_m + fixity_factor / beta_per_m)
}

/// 🏗️ Euler load N_cr = π² EI/L_eff² [N] of the pile over its effective length.
pub fn pile_buckling_load_n(ei_nm2: f64, effective_length_m: f64) -> f64 {
    std::f64::consts::PI.powi(2) * ei_nm2 / (effective_length_m * effective_length_m)
}

/// 🏗️ Bending moment V L_eff (head free to rotate) or V L_eff/2 (head fixed against rotation) of the head shear V on the unsupported length.
pub fn pile_unsupported_moment_nm(head_shear_n: f64, effective_length_m: f64, fixed_head: bool) -> f64 {
    head_shear_n.abs() * effective_length_m * if fixed_head { 0.5 } else { 1.0 }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
