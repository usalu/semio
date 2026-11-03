//! ↩️ `change-member-ltb-restraint-spacing` inverse: restores the previous `ltb_restraint_spacing`.

use crate::change_member_ltb_restraint_spacing::ChangeMemberLtbRestraintSpacing;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberLtbRestraintSpacing, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_ltb_restraint_spacing::ChangeMemberLtbRestraintSpacing> {
    Ok(match base.members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeMemberLtbRestraintSpacing { id: payload.id.clone(), new_ltb_restraint_spacing: row.ltb_restraint_spacing })],
        None => Vec::new(),
    })
}
