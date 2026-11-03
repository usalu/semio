//! ↩️ `change-reliability-resistance-dist` inverse: restores the previous `resistance_distribution`.

use crate::change_reliability_resistance_dist::ChangeReliabilityResistanceDist;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeReliabilityResistanceDist, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_reliability_resistance_dist::ChangeReliabilityResistanceDist> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeReliabilityResistanceDist, base: &En1990Snapshot) -> Option<ChangeReliabilityResistanceDist> {
    let r0 = base.reliability_analyses.iter().find(|row| row.id == payload.id)?;
    Some(ChangeReliabilityResistanceDist { id: payload.id.clone(), new_resistance_distribution: r0.resistance_distribution })
}
