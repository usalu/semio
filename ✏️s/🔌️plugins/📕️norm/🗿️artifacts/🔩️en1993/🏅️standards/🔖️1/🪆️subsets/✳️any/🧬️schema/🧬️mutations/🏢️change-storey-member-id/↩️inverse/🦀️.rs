//! ↩️ `change-storey-member-id` inverse: restores the previous `member_id`.

use crate::change_storey_member_id::ChangeStoreyMemberId;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeStoreyMemberId, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_storey_member_id::ChangeStoreyMemberId> {
    Ok(match base.storeys.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeStoreyMemberId { id: payload.id.clone(), new_member_id: row.member_id.clone() })],
        None => Vec::new(),
    })
}
