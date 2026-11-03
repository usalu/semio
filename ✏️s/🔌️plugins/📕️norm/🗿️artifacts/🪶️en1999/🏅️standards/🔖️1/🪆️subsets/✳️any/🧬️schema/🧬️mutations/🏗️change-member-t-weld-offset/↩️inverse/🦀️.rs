//! ↩️ `change-member-t-weld-offset` inverse: restores the previous `transverse_weld_offset`.

use crate::change_member_t_weld_offset::ChangeMemberTWeldOffset;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberTWeldOffset, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_t_weld_offset::ChangeMemberTWeldOffset> {
    Ok(match base.members.iter().find(|row| row.id == payload.member_id) {
        Some(row) => vec![Op::from(ChangeMemberTWeldOffset { member_id: payload.member_id.clone(), new_transverse_weld_offset: row.transverse_weld_offset })],
        None => Vec::new(),
    })
}
