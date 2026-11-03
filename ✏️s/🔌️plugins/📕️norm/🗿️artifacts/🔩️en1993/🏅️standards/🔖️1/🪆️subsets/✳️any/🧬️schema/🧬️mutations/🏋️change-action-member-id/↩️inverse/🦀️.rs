//! ↩️ `change-action-member-id` inverse: restores the previous `member_id`.

use crate::change_action_member_id::ChangeActionMemberId;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeActionMemberId, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_action_member_id::ChangeActionMemberId> {
    Ok(match base.member_actions.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeActionMemberId { id: payload.id.clone(), new_member_id: row.member_id.clone() })],
        None => Vec::new(),
    })
}
