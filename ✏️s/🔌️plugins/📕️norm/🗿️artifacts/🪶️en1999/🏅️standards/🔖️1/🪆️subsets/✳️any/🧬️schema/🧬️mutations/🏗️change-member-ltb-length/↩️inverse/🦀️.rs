//! ↩️ `change-member-ltb-length` inverse: restores the previous `ltb_length`.

use crate::change_member_ltb_length::ChangeMemberLtbLength;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberLtbLength, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_ltb_length::ChangeMemberLtbLength> {
    Ok(match base.members.iter().find(|row| row.id == payload.member_id) {
        Some(row) => vec![Op::from(ChangeMemberLtbLength { member_id: payload.member_id.clone(), new_ltb_length: row.ltb_length })],
        None => Vec::new(),
    })
}
