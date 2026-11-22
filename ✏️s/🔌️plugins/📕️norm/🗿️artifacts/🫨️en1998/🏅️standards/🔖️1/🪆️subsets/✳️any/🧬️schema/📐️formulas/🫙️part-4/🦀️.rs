//! 🫙️ EN 1998-4 formulas for tanks, silos and pipelines.

#[path = "../📏️pipe-strain/🦀️.rs"]
mod pipe_strain;

#[path = "../🌊️pipe-wave/🦀️.rs"]
pub mod pipe_wave;

#[path = "../🌾️silo-pressure/🦀️.rs"]
mod silo_pressure;

pub use self::pipe_strain::{PipeStrainLimits, welded_pipe_strain_limits};
pub use self::pipe_wave::{PipeWaveInput, PipeWaveResponse, bonded_pipe_wave_response};
pub use self::silo_pressure::{SiloPressure, SiloPressureInput, SiloWall, SiloWallSection, granular_silo_pressure};

/// 🫙 Housner impulsive mass ratio m_i/m = tanh(1.7·R/H)/(1.7·R/H) (EN 1998-4 Annex A.3).
pub fn impulsive_mass_ratio(h_over_r: f64) -> f64 {
    let x = 1.7 / h_over_r.max(1e-9);
    x.tanh() / x
}
/// 🫙 Housner convective mass ratio m_c/m = 0.460·(R/H)·tanh(1.84·H/R) = 0.230·(D/H)·tanh(3.68·H/D) (EN 1998-4 Annex A.3, coefficient unverified; the first sloshing mode of the rigid tank gives 2/(ε(ε²−1)) = 0.4548).
pub fn convective_mass_ratio(h_over_r: f64) -> f64 {
    0.460 / h_over_r.max(1e-9) * (1.84 * h_over_r).tanh()
}
/// 🌊 Sloshing period T_c = 2π/ω_c with ω_c² = (1.84·g/R)·tanh(1.84·H/R) (EN 1998-4 Annex A.3).
pub fn convective_period_s(height_m: f64, radius_m: f64) -> f64 {
    let r = radius_m.max(1e-9);
    let omega2 = 1.84 * 9.81 / r * (1.84 * height_m / r).tanh();
    2.0 * std::f64::consts::PI / omega2.max(1e-12).sqrt()
}
/// 🌊 Damping correction of the convective mode, ξ_c = 0.5 % (EN 1998-4 Annex A.3).
pub fn convective_damping_eta() -> f64 { (10.0_f64 / 5.5).sqrt() }
pub fn silo_behaviour_factor(q_nominal: f64) -> f64 { q_nominal.min(1.5).max(1.0) }
pub fn srss(a: f64, b: f64) -> f64 { (a * a + b * b).sqrt() }
/// ⬇️ Axial demand of a silo shell N = m·g·(1 + a_vg/g) with a_vg = 0.9·a_g (EN 1998-1 §3.2.2.3 type 1 ratio, unverified NDP).
pub fn silo_axial_demand_n(mass_kg: f64, a_g: f64) -> f64 {
    mass_kg * (9.81 + semio_s_plugin_norm_compliance::ndp_common("ndp.en1998.vertical-spectrum.avg-ratio") * a_g)
}
