//! ↩️ `change-member-ltb-length` inverse: restores the previous `ltb_length`.

use crate::change_member_ltb_length::ChangeMemberLtbLength;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberLtbLength, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_ltb_length::ChangeMemberLtbLength> {
    Ok(match base.members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeMemberLtbLength { id: payload.id.clone(), new_ltb_length: row.ltb_length })],
        None => Vec::new(),
    })
}
