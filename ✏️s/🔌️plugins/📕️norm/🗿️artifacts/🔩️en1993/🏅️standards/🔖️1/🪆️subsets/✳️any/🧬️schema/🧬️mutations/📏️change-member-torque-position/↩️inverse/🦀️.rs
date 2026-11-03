//! ↩️ `change-member-torque-position` inverse: restores the previous `torque_position`.

use crate::change_member_torque_position::ChangeMemberTorquePosition;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberTorquePosition, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_torque_position::ChangeMemberTorquePosition> {
    Ok(match base.members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeMemberTorquePosition { id: payload.id.clone(), new_torque_position: row.torque_position })],
        None => Vec::new(),
    })
}
