//! ↩️ `change-member-bearing-length` inverse: restores the previous `bearing_length`.

use crate::change_member_bearing_length::ChangeMemberBearingLength;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberBearingLength, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_bearing_length::ChangeMemberBearingLength> {
    Ok(match base.members.iter().find(|row| row.id == payload.member_id) {
        Some(row) => vec![Op::from(ChangeMemberBearingLength { member_id: payload.member_id.clone(), new_bearing_length: row.bearing_length })],
        None => Vec::new(),
    })
}
