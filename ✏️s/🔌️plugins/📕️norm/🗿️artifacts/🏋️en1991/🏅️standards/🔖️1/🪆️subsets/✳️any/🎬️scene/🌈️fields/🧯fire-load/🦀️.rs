//! 🧯 Fire load colour map: the characteristic and the design fire load density of the compartment as scalar fields of its room volume, so the 3D compartment is coloured by the fire load of the chosen basis (EN 1991-1-2, Annex E).

use super::scene::*;

/// 🆔 Id of the field of the design fire load density q_f,d.
pub const DESIGN: &str = "fire-load-design";

/// 🆔 Id of the field of the characteristic fire load density q_f,k of the chosen basis.
pub const CHARACTERISTIC: &str = "fire-load-characteristic";

fn field(id: &str, j_m2: f64) -> ScalarField {
    ScalarField { id: id.into(), unit: QuantityKind::Energy, samples: FieldSamples::Instance { values: vec![j_m2] } }
}

/// 🌈️ Colour channel of a compartment with characteristic fire load `qf_k` and design fire load `qf_d` (J/m²): both as fields, the design value active.
pub fn channel(qf_k: f64, qf_d: f64) -> ColourChannel {
    ColourChannel::Fields { active: DESIGN.into(), fields: vec![field(DESIGN, qf_d), field(CHARACTERISTIC, qf_k)] }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
