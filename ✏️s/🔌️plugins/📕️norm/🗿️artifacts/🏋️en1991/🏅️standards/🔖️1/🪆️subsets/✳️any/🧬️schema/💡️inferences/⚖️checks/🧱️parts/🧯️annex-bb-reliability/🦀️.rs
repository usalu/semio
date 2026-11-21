//! 🧯 Design reliability for compartment fires according to DIN EN 1991-1-2/NA:2015-09 Annex BB.5.

use semio_s_plugin_norm_compliance::normal_distribution::{phi_cdf, phi_inv};

/// 📊 Model inputs for the annual probability of an initial fire in Table BB.3.
pub fn ignition_model(occupancy: &str) -> Option<(f64, f64, f64)> {
    match occupancy {
        "residential" => Some((4.8e-5, 0.90, 0.003)),
        "office" => Some((5.9e-5, 0.90, 0.0062)),
        "hospital" => Some((7.0e-4, 0.75, 0.300)),
        "hotel" => Some((8.0e-5, 1.00, 0.037)),
        "school" => Some((2.0e-4, 0.75, 0.040)),
        "retail" => Some((6.6e-5, 1.00, 0.0084)),
        "theatre" => Some((9.7e-5, 0.75, 0.020)),
        "assembly" => Some((9.7e-5, 1.00, 0.120)),
        _ => None,
    }
}

/// 🚒 Failure probability from the applicable response row of Table BB.4.
pub fn brigade_failure(brigade: &str, minutes: f64, project: f64) -> Option<f64> {
    match brigade {
        "public" if minutes.is_finite() && minutes > 0.0 =>
            Some(0.2 + 0.3 * ((minutes - 15.0) / 5.0).clamp(0.0, 1.0)),
        "onsite_four" if minutes.is_finite() && minutes > 0.0 && minutes < 10.0 => Some(0.02),
        "onsite_two" if minutes.is_finite() && minutes > 0.0 && minutes < 10.0 => Some(0.05),
        "project" if project > 0.0 && project <= 1.0 => Some(project),
        _ => None,
    }
}

/// 💧 Automatic suppression failure probability from Table BB.4.
pub fn suppression_failure(system: &str) -> Option<f64> {
    match system {
        "absent" => Some(1.0),
        "standard" => Some(0.02),
        "other_sprinkler" => Some(0.05),
        "water" | "gas" => Some(0.1),
        _ => None,
    }
}

/// 🏛 Annual target failure probability for the use and consequence classes of Table BB.5.
pub fn annual_failure_target(group: &str, impact: &str, project: f64) -> Option<f64> {
    match (group, impact) {
        ("office", "high") | ("other", "medium") => Some(1.3e-6),
        ("office" | "agricultural", "medium") | ("other", "low") => Some(1.3e-5),
        ("office" | "agricultural", "low") => Some(1.1e-4),
        ("other", "high") => Some(1.0e-7),
        ("project", _) if project > 0.0 && project < 1.0 => Some(project),
        _ => None,
    }
}

/// 🧮 Annual ignition probability according to Eq. (BB.10).
pub fn ignition_probability(area: f64, coefficient: f64, exponent: f64) -> f64 {
    -(-coefficient * area.powf(exponent)).exp_m1()
}

/// 📈 Conditional reliability and partial factors from Eqs. (BB.13)–(BB.15).
#[derive(Debug, Clone, Copy)]
pub struct CompartmentReliability {
    pub damaging_fire: f64,
    pub conditional_failure: f64,
    pub beta: f64,
    pub classified_fire_load_factor: f64,
    pub surveyed_fire_load_factor: f64,
}

impl CompartmentReliability {
    /// ✅ Evaluate the annual and conditional probabilities using verified project input.
    pub fn evaluate(ignition: f64, manual: f64, automatic: f64, target: f64) -> Option<Self> {
        let damaging_fire = ignition * manual * automatic;
        let conditional_failure = target / damaging_fire;
        if !(conditional_failure > 0.0 && conditional_failure < 1.0) {
            return None;
        }
        let beta = -phi_inv(conditional_failure);
        let gamma = |variation: f64| -> f64 {
            let fractile = |log_probability: f64| 1.0 - variation * 0.78 * (0.5772 + (-log_probability).ln());
            let log_probability = if beta >= 0.0 { (-phi_cdf(-0.6 * beta)).ln_1p() } else { phi_cdf(0.6 * beta).ln() };
            fractile(log_probability) / fractile(0.9_f64.ln())
        };
        let classified_fire_load_factor = gamma(0.3);
        let surveyed_fire_load_factor = gamma(0.2);
        if !classified_fire_load_factor.is_finite() || classified_fire_load_factor <= 0.0
            || !surveyed_fire_load_factor.is_finite() || surveyed_fire_load_factor <= 0.0 {
            return None;
        }
        Some(Self { damaging_fire, conditional_failure, beta, classified_fire_load_factor, surveyed_fire_load_factor })
    }
}
