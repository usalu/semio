//! ↩️ `change-reliability-action-sd` inverse: restores the previous `action_sd`.

use crate::change_reliability_action_sd::ChangeReliabilityActionSd;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeReliabilityActionSd, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_reliability_action_sd::ChangeReliabilityActionSd> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeReliabilityActionSd, base: &En1990Snapshot) -> Option<ChangeReliabilityActionSd> {
    let r0 = base.reliability_analyses.iter().find(|row| row.id == payload.id)?;
    Some(ChangeReliabilityActionSd { id: payload.id.clone(), new_action_sd: r0.action_sd })
}
