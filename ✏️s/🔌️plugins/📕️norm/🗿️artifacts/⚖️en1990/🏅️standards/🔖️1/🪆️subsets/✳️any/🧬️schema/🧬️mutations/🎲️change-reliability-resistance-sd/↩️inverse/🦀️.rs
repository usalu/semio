//! ↩️ `change-reliability-resistance-sd` inverse: restores the previous `resistance_sd`.

use crate::change_reliability_resistance_sd::ChangeReliabilityResistanceSd;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeReliabilityResistanceSd, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_reliability_resistance_sd::ChangeReliabilityResistanceSd> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeReliabilityResistanceSd, base: &En1990Snapshot) -> Option<ChangeReliabilityResistanceSd> {
    let r0 = base.reliability_analyses.iter().find(|row| row.id == payload.id)?;
    Some(ChangeReliabilityResistanceSd { id: payload.id.clone(), new_resistance_sd: r0.resistance_sd })
}
