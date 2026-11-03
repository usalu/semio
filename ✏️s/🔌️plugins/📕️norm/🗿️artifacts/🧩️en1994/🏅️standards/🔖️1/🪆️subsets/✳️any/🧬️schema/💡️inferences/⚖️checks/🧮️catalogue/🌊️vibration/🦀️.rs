//! 🌊️ Floor vibration of composite beams: lowest natural frequency of a simply supported beam from its short-term composite stiffness and the vibrating mass. See [EN 1990 A1.4.4](https://eurocodes.jrc.ec.europa.eu/) and [EN 1994-1-1 7.3](https://eurocodes.jrc.ec.europa.eu/).

use super::constants::E_A;
use std::f64::consts::PI;

/// 🌍 Standard gravity [m/s²] converting the load of the vibrating mass into kilograms.
pub const GRAVITY: f64 = 9.80665;

/// ⚖️ Vibrating mass per metre [kg/m] of the permanent line load `g` and the share `psi` of the variable line load `q` (N/m).
pub fn mass_per_length_kg_m(g_n_m: f64, q_n_m: f64, psi: f64) -> f64 {
    (g_n_m + psi * q_n_m) / GRAVITY
}

/// 🌊️ Lowest natural frequency f₁ = π/(2 L²)·√(E_a I / m) [Hz] of a simply supported beam with the composite second moment `i_m4` (short term, modular ratio n₀) and mass `mass_kg_m` per metre.
pub fn natural_frequency_hz(span_m: f64, i_m4: f64, mass_kg_m: f64) -> f64 {
    PI / (2.0 * span_m * span_m) * (E_A * i_m4 / mass_kg_m.max(1e-12)).sqrt()
}

/// 📏 Utilisation f_min/f₁ of the frequency criterion f₁ ≥ f_min.
pub fn frequency_utilisation(f_min_hz: f64, f1_hz: f64) -> f64 {
    f_min_hz / f1_hz.max(1e-12)
}
