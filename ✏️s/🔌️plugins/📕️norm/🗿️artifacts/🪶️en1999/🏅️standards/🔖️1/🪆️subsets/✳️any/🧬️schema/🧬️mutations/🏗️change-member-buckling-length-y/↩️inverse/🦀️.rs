//! ↩️ `change-member-buckling-length-y` inverse: restores the previous `buckling_length_y`.

use crate::change_member_buckling_length_y::ChangeMemberBucklingLengthY;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberBucklingLengthY, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_buckling_length_y::ChangeMemberBucklingLengthY> {
    Ok(match base.members.iter().find(|row| row.id == payload.member_id) {
        Some(row) => vec![Op::from(ChangeMemberBucklingLengthY { member_id: payload.member_id.clone(), new_buckling_length_y: row.buckling_length_y })],
        None => Vec::new(),
    })
}
