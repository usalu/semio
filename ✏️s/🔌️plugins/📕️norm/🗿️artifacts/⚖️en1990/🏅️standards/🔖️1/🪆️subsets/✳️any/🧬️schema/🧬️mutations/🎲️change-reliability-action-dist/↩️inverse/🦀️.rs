//! ↩️ `change-reliability-action-dist` inverse: restores the previous `action_distribution`.

use crate::change_reliability_action_dist::ChangeReliabilityActionDist;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeReliabilityActionDist, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_reliability_action_dist::ChangeReliabilityActionDist> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeReliabilityActionDist, base: &En1990Snapshot) -> Option<ChangeReliabilityActionDist> {
    let r0 = base.reliability_analyses.iter().find(|row| row.id == payload.id)?;
    Some(ChangeReliabilityActionDist { id: payload.id.clone(), new_action_distribution: r0.action_distribution })
}
