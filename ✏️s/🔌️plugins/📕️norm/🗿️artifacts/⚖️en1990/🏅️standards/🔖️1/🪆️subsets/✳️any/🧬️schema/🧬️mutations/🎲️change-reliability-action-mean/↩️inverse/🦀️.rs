//! ↩️ `change-reliability-action-mean` inverse: restores the previous `action_mean`.

use crate::change_reliability_action_mean::ChangeReliabilityActionMean;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeReliabilityActionMean, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_reliability_action_mean::ChangeReliabilityActionMean> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeReliabilityActionMean, base: &En1990Snapshot) -> Option<ChangeReliabilityActionMean> {
    let r0 = base.reliability_analyses.iter().find(|row| row.id == payload.id)?;
    Some(ChangeReliabilityActionMean { id: payload.id.clone(), new_action_mean: r0.action_mean })
}
