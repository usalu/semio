//! ↩️ `change-member-buckling-length-y` inverse: restores the previous `buckling_length_y`.

use crate::change_member_buckling_length_y::ChangeMemberBucklingLengthY;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberBucklingLengthY, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_buckling_length_y::ChangeMemberBucklingLengthY> {
    Ok(match base.members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeMemberBucklingLengthY { id: payload.id.clone(), new_buckling_length_y: row.buckling_length_y })],
        None => Vec::new(),
    })
}
