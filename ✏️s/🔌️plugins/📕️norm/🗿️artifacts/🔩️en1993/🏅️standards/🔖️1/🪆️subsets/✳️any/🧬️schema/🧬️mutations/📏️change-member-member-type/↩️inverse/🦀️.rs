//! ↩️ `change-member-member-type` inverse: restores the previous `member_type`.

use crate::change_member_member_type::ChangeMemberMemberType;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberMemberType, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_member_type::ChangeMemberMemberType> {
    Ok(match base.members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeMemberMemberType { id: payload.id.clone(), new_member_type: row.member_type })],
        None => Vec::new(),
    })
}
