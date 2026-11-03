//! ↩️ `change-buckling-length-member-id` inverse: restores the previous `member_id`.

use crate::change_buckling_length_member_id::ChangeBucklingLengthMemberId;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBucklingLengthMemberId, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_buckling_length_member_id::ChangeBucklingLengthMemberId> {
    Ok(match base.buckling_lengths.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBucklingLengthMemberId { id: payload.id.clone(), new_member_id: row.member_id.clone() })],
        None => Vec::new(),
    })
}
