//! ❄️ Roof snow profile: the snow load along a roof section from eaves to eaves, the shape coefficient μ₁ raised by the wind-induced drift triangles behind a parapet or an obstruction (EN 1991-1-3, 5.3.6).

use super::checks::part_1_3;
use super::scene::*;

/// 🧱 One drift triangle along the roof section: μ rises (or falls) linearly from the base value to `peak` over `length`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Drift {
    pub start: f64,
    pub length: f64,
    pub peak: f64,
    pub rising: bool,
}

/// 📐 Shape coefficient at plan position `x`: the base value, raised to the largest drift triangle that covers `x`.
pub fn mu_at(base: f64, drifts: &[Drift], x: f64) -> f64 {
    drifts.iter().map(|d| part_1_3::drift_mu_at(base, d.start, d.length, d.peak, d.rising, x)).fold(base, f64::max)
}

/// 📊 `n + 1` profile samples over a section of `depth`, each the load `load(μ)` (Pa) at its plan position.
pub fn samples(depth: f64, n: usize, base: f64, drifts: &[Drift], load: impl Fn(f64) -> f64) -> Vec<ProfileSample> {
    (0..=n).map(|i| ProfileSample { at: i as f64 / n as f64, value: Quantity::new(QuantityKind::Pressure, load(mu_at(base, drifts, depth * i as f64 / n as f64))) }).collect()
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
