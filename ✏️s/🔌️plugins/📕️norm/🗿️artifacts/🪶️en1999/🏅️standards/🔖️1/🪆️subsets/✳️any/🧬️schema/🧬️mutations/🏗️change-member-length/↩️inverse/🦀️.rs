//! ↩️ `change-member-length` inverse: restores the previous `length`.

use crate::change_member_length::ChangeMemberLength;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberLength, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_length::ChangeMemberLength> {
    Ok(match base.members.iter().find(|row| row.id == payload.member_id) {
        Some(row) => vec![Op::from(ChangeMemberLength { member_id: payload.member_id.clone(), new_length: row.length })],
        None => Vec::new(),
    })
}
