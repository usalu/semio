//! ↩️ `change-weld-member-id` inverse: restores the previous `member_id`.

use crate::change_weld_member_id::ChangeWeldMemberId;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeWeldMemberId, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_weld_member_id::ChangeWeldMemberId> {
    Ok(match base.weld_groups.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeWeldMemberId { id: payload.id.clone(), new_member_id: row.member_id.clone() })],
        None => Vec::new(),
    })
}
