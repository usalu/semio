//! ↩️ `change-member-length` inverse: restores the previous `length`.

use crate::change_member_length::ChangeMemberLength;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberLength, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_length::ChangeMemberLength> {
    Ok(match base.members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeMemberLength { id: payload.id.clone(), new_length: row.length })],
        None => Vec::new(),
    })
}
