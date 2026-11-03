//! 🛢️ Hydrodynamic action on a vertical cylindrical tank (EN 1998-4 Annex A): the impulsive and the convective pressure on the wall by the rigid-tank solutions of Housner and Veletsos, scaled to the impulsive and convective masses of the base shear check, the hydrostatic pressure, the sloshing wave height and the hoop stress. Pure functions of numbers; the checks and the scene call them so picture and report cannot disagree. See [EN 1998-4](https://eur-lex.europa.eu/).

use crate::part_4;

pub const GRAVITY: f64 = 9.81;
pub const SERIES_TERMS: usize = 12;
pub const CONVECTIVE_ROOT: f64 = 1.841_183_781_340_659_3;

/// 🪣 Height of the fluid, the filling ratio of the tank height.
pub fn fluid_height_m(height_m: f64, filling_ratio: f64) -> f64 {
    height_m.max(0.0) * filling_ratio.clamp(0.0, 1.0)
}

/// 💧 Density of the fluid from its mass in the cylinder of radius `R` and height `H`.
pub fn density_kg_m3(fluid_mass_kg: f64, radius_m: f64, fluid_height_m: f64) -> f64 {
    let volume = std::f64::consts::PI * radius_m * radius_m * fluid_height_m;
    if volume > 0.0 { fluid_mass_kg / volume } else { 0.0 }
}

/// 🧮 Modified Bessel function of the first kind `I_n(x)` by its power series (`x ≥ 0`).
pub fn bessel_i(order: u32, x: f64) -> f64 {
    let half = x / 2.0;
    let mut term = (1..=order).fold(1.0, |t, k| t * half / f64::from(k));
    let mut sum = term;
    for k in 0..400_u32 {
        term *= half * half / (f64::from(k + 1) * f64::from(k + 1 + order));
        sum += term;
        if term < 1.0e-17 * sum && f64::from(k) > half {
            break;
        }
    }
    sum
}

/// 🧮 Ratio `I₁(x) / I₁'(x)` with `I₁' = (I₀ + I₂) / 2`.
pub fn bessel_ratio(x: f64) -> f64 {
    2.0 * bessel_i(1, x) / (bessel_i(0, x) + bessel_i(2, x))
}

fn lambda(n: usize) -> f64 {
    (2 * n - 1) as f64 * std::f64::consts::FRAC_PI_2
}

/// 🌊 Impulsive wall pressure of the rigid tank (Veletsos) at the height `z` above the base and the angle `θ` from the direction of the ground motion, `p = 2 ρ H a Σ (−1)^{n−1} / λ_n² · I₁(λ_n R/H) / I₁'(λ_n R/H) · cos(λ_n z/H) cos θ`.
pub fn impulsive_series_pa(z_m: f64, fluid_height_m: f64, radius_m: f64, density: f64, acceleration: f64, theta_rad: f64) -> f64 {
    let sum: f64 = (1..=SERIES_TERMS)
        .map(|n| {
            let l = lambda(n);
            let sign = if n % 2 == 1 { 1.0 } else { -1.0 };
            sign / (l * l) * bessel_ratio(l * radius_m / fluid_height_m) * (l * z_m / fluid_height_m).cos()
        })
        .sum();
    2.0 * density * fluid_height_m * acceleration * sum * theta_rad.cos()
}

/// ⚖️ Impulsive mass share of the series solution, `m_i / m = 2 (H/R) Σ I₁(λ_n R/H) / (λ_n³ I₁'(λ_n R/H))`.
pub fn impulsive_series_mass_ratio(fluid_height_m: f64, radius_m: f64) -> f64 {
    2.0 * fluid_height_m / radius_m * (1..=SERIES_TERMS).map(|n| bessel_ratio(lambda(n) * radius_m / fluid_height_m) / lambda(n).powi(3)).sum::<f64>()
}

/// 🌊 Convective wall pressure of the first sloshing mode at the height `z`, `p = 2 ρ R / (ε² − 1) · cosh(ε z/R) / cosh(ε H/R) · a cos θ` with `ε = 1.841`.
pub fn convective_pa(z_m: f64, fluid_height_m: f64, radius_m: f64, density: f64, acceleration: f64, theta_rad: f64) -> f64 {
    let e = CONVECTIVE_ROOT;
    2.0 * density * radius_m / (e * e - 1.0) * (e * z_m / radius_m).cosh() / (e * fluid_height_m / radius_m).cosh() * acceleration * theta_rad.cos()
}

/// ⚖️ Convective mass share of the first sloshing mode, `m_c / m = 2 R tanh(ε H/R) / (H ε (ε² − 1))`.
pub fn convective_series_mass_ratio(fluid_height_m: f64, radius_m: f64) -> f64 {
    let e = CONVECTIVE_ROOT;
    2.0 * radius_m * (e * fluid_height_m / radius_m).tanh() / (fluid_height_m * e * (e * e - 1.0))
}

/// 💧 Hydrostatic pressure `ρ g (H − z)`.
pub fn hydrostatic_pa(z_m: f64, fluid_height_m: f64, density: f64) -> f64 {
    density * GRAVITY * (fluid_height_m - z_m).max(0.0)
}

/// 📊 The pressure on the wall at one height in the plane of the ground motion.
#[derive(Clone, Debug, PartialEq)]
pub struct WallPressure {
    pub z_m: f64,
    pub hydrostatic_pa: f64,
    pub impulsive_pa: f64,
    pub convective_pa: f64,
    pub dynamic_pa: f64,
    pub total_pa: f64,
}

/// 📊 The wall pressure profile at `points + 1` heights: the impulsive series scaled so that its wall resultant equals the Housner impulsive mass of the base shear check, the convective pressure scaled likewise, both combined by SRSS and added to the hydrostatic pressure.
pub fn wall_profile(fluid_height_m: f64, radius_m: f64, density: f64, impulsive_acceleration: f64, convective_acceleration: f64, points: usize) -> Vec<WallPressure> {
    if fluid_height_m <= 0.0 || radius_m <= 0.0 {
        return Vec::new();
    }
    let ratio = fluid_height_m / radius_m;
    let impulsive_scale = part_4::impulsive_mass_ratio(ratio) / impulsive_series_mass_ratio(fluid_height_m, radius_m).max(1.0e-12);
    let convective_scale = part_4::convective_mass_ratio(ratio) / convective_series_mass_ratio(fluid_height_m, radius_m).max(1.0e-12);
    (0..=points)
        .map(|i| {
            let z = fluid_height_m * i as f64 / points.max(1) as f64;
            let hydrostatic = hydrostatic_pa(z, fluid_height_m, density);
            let impulsive = impulsive_scale * impulsive_series_pa(z, fluid_height_m, radius_m, density, impulsive_acceleration, 0.0);
            let convective = convective_scale * convective_pa(z, fluid_height_m, radius_m, density, convective_acceleration, 0.0);
            let dynamic = part_4::srss(impulsive, convective);
            WallPressure { z_m: z, hydrostatic_pa: hydrostatic, impulsive_pa: impulsive, convective_pa: convective, dynamic_pa: dynamic, total_pa: hydrostatic + dynamic }
        })
        .collect()
}

/// 🔝 The row of the profile with the largest total pressure.
pub fn governing(profile: &[WallPressure]) -> Option<&WallPressure> {
    profile.iter().max_by(|a, b| a.total_pa.total_cmp(&b.total_pa))
}

/// 🌊 Sloshing wave height `d_max = c R S_e(T_c) / g` with the unverified coefficient `c = 0.84` (EN 1998-4 A.3.3.2).
pub fn slosh_height_m(coefficient: f64, radius_m: f64, convective_acceleration: f64) -> f64 {
    coefficient * radius_m * convective_acceleration / GRAVITY
}

/// 🔩 Hoop stress of the thin shell, `σ = p R / t` in MPa.
pub fn hoop_stress_mpa(pressure_pa: f64, radius_m: f64, thickness_m: f64) -> f64 {
    if thickness_m > 0.0 { pressure_pa * radius_m / thickness_m / 1.0e6 } else { f64::INFINITY }
}

/// 📐 Thickness that makes the hoop stress equal the strength.
pub fn required_thickness_m(pressure_pa: f64, radius_m: f64, strength_mpa: f64) -> f64 {
    if strength_mpa > 0.0 { pressure_pa * radius_m / (strength_mpa * 1.0e6) } else { f64::INFINITY }
}

/// 🌊 Free-surface elevation of the first sloshing mode at the angle `θ` and the radius `r`, `η = d_max · J₁(ε r/R) / J₁(ε) · cos θ`, normalised to `d_max` at the wall.
pub fn surface_elevation_m(slosh_m: f64, radius_m: f64, r_m: f64, theta_rad: f64) -> f64 {
    slosh_m * bessel_j1(CONVECTIVE_ROOT * r_m / radius_m) / bessel_j1(CONVECTIVE_ROOT) * theta_rad.cos()
}

/// 🧮 Bessel function `J₁(x)` by its power series (`0 ≤ x ≤ 4`).
pub fn bessel_j1(x: f64) -> f64 {
    let half = x / 2.0;
    let mut term = half;
    let mut sum = term;
    for k in 0..40_u32 {
        term *= -half * half / (f64::from(k + 1) * f64::from(k + 2));
        sum += term;
    }
    sum
}
