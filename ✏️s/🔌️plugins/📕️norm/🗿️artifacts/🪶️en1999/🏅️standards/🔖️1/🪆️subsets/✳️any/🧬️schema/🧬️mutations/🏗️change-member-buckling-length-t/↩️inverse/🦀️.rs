//! ↩️ `change-member-buckling-length-t` inverse: restores the previous `buckling_length_t`.

use crate::change_member_buckling_length_t::ChangeMemberBucklingLengthT;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberBucklingLengthT, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_buckling_length_t::ChangeMemberBucklingLengthT> {
    Ok(match base.members.iter().find(|row| row.id == payload.member_id) {
        Some(row) => vec![Op::from(ChangeMemberBucklingLengthT { member_id: payload.member_id.clone(), new_buckling_length_t: row.buckling_length_t })],
        None => Vec::new(),
    })
}
