//! ↩️ `change-member-ltb-rest-spacing` inverse: restores the previous `ltb_restraint_spacing`.

use crate::change_member_ltb_rest_spacing::ChangeMemberLtbRestSpacing;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberLtbRestSpacing, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_ltb_rest_spacing::ChangeMemberLtbRestSpacing> {
    Ok(match base.members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeMemberLtbRestSpacing { id: payload.id.clone(), new_ltb_restraint_spacing: row.ltb_restraint_spacing })],
        None => Vec::new(),
    })
}
