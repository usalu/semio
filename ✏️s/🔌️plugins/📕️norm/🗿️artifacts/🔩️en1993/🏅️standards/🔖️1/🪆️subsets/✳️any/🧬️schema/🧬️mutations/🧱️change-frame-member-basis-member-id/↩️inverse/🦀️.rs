//! ↩️ `change-frame-member-basis-member-id` inverse: restores the previous `member_id`.

use crate::change_frame_member_basis_member_id::ChangeFrameMemberBasisMemberId;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFrameMemberBasisMemberId, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_frame_member_basis_member_id::ChangeFrameMemberBasisMemberId> {
    Ok(match base.frame_member_bases.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFrameMemberBasisMemberId { id: payload.id.clone(), new_member_id: row.member_id.clone() })],
        None => Vec::new(),
    })
}
