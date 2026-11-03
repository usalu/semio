//! ↩️ `change-member-buckling-length-z` inverse: restores the previous `buckling_length_z`.

use crate::change_member_buckling_length_z::ChangeMemberBucklingLengthZ;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberBucklingLengthZ, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_buckling_length_z::ChangeMemberBucklingLengthZ> {
    Ok(match base.members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeMemberBucklingLengthZ { id: payload.id.clone(), new_buckling_length_z: row.buckling_length_z })],
        None => Vec::new(),
    })
}
