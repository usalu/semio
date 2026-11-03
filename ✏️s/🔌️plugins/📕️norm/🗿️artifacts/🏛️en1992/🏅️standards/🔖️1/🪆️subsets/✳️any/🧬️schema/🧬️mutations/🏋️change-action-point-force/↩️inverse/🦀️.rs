//! ↩️ `change-action-point-force` inverse: restores the previous `point_force`.

use crate::change_action_point_force::ChangeActionPointForce;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeActionPointForce, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_action_point_force::ChangeActionPointForce> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeActionPointForce, base: &En1992Snapshot) -> Option<ChangeActionPointForce> {
    let r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    let r1 = r0.actions.iter().find(|row| row.id == payload.action_id)?;
    Some(ChangeActionPointForce { member_id: payload.member_id.clone(), action_id: payload.action_id.clone(), new_point_force: r1.point_force })
}
