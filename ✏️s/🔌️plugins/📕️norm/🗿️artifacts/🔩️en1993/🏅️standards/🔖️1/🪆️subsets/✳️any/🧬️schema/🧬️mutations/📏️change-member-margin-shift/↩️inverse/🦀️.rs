//! ↩️ `change-member-margin-shift` inverse: restores the previous `margin_shift`.

use crate::change_member_margin_shift::ChangeMemberMarginShift;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberMarginShift, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_margin_shift::ChangeMemberMarginShift> {
    Ok(match base.members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeMemberMarginShift { id: payload.id.clone(), new_margin_shift: row.margin_shift })],
        None => Vec::new(),
    })
}
