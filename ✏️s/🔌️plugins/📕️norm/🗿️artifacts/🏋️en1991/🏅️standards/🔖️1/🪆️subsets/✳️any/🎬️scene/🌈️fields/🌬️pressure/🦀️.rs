//! 🌬️ Wind pressure colour map: the net pressure of a c_pe zone for both internal-pressure cases (EN 1991-1-4, 7.2.9) and the external pressure alone as scalar fields of the zone entity, so the 3D building faces are coloured by pressure and suction.

use super::checks::part_1_4;
use super::scene::*;

/// 🆔 Id of the field of the external pressure q_p c_pe,10 alone.
pub const EXTERNAL: &str = "wind-external";

/// 🆔 Id of the field of the net pressure for the internal pressure coefficient `c_pi`.
pub fn net_id(c_pi: f64) -> String {
    format!("wind-net{c_pi:+.1}")
}

fn field(id: String, pa: f64) -> ScalarField {
    ScalarField { id, unit: QuantityKind::Pressure, samples: FieldSamples::Instance { values: vec![pa] } }
}

/// 🌈️ Colour channel of a zone with peak velocity pressure `qp` (Pa), external coefficient `c_pe` and structural factor `factor`: one field per internal-pressure case plus the external one, the larger magnitude active.
pub fn channel(qp: f64, c_pe: f64, factor: f64) -> ColourChannel {
    let nets: Vec<(String, f64)> = part_1_4::c_pi_cases().iter().map(|c_pi| (net_id(*c_pi), part_1_4::wind_pressure_pa(qp, c_pe, *c_pi, 1.0, 1.0) * factor)).collect();
    let active = nets.iter().max_by(|a, b| a.1.abs().total_cmp(&b.1.abs())).map_or_else(|| EXTERNAL.to_string(), |n| n.0.clone());
    let mut fields: Vec<ScalarField> = nets.into_iter().map(|(id, pa)| field(id, pa)).collect();
    fields.push(field(EXTERNAL.into(), qp * c_pe * factor));
    ColourChannel::Fields { active, fields }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
