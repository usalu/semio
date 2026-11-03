//! ↩️ `change-impact-action-assumed-force` inverse: restores the previous `assumed_force`.

use crate::change_impact_action_assumed_force::ChangeImpactActionAssumedForce;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeImpactActionAssumedForce, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_impact_action_assumed_force::ChangeImpactActionAssumedForce> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeImpactActionAssumedForce, base: &En1991Snapshot) -> Option<ChangeImpactActionAssumedForce> {
    let r0 = base.impact_actions.iter().find(|row| row.id == payload.id)?;
    Some(ChangeImpactActionAssumedForce { id: payload.id.clone(), new_assumed_force: r0.assumed_force })
}
