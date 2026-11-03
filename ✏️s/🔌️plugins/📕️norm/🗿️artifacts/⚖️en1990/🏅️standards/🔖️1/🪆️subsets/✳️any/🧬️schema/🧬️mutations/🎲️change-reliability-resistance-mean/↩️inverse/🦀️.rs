//! ↩️ `change-reliability-resistance-mean` inverse: restores the previous `resistance_mean`.

use crate::change_reliability_resistance_mean::ChangeReliabilityResistanceMean;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeReliabilityResistanceMean, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_reliability_resistance_mean::ChangeReliabilityResistanceMean> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeReliabilityResistanceMean, base: &En1990Snapshot) -> Option<ChangeReliabilityResistanceMean> {
    let r0 = base.reliability_analyses.iter().find(|row| row.id == payload.id)?;
    Some(ChangeReliabilityResistanceMean { id: payload.id.clone(), new_resistance_mean: r0.resistance_mean })
}
