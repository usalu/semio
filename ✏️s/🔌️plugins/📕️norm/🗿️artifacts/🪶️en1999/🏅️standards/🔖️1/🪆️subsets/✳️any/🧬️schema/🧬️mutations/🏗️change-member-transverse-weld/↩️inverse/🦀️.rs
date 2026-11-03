//! ↩️ `change-member-transverse-weld` inverse: restores the previous `transverse_weld`.

use crate::change_member_transverse_weld::ChangeMemberTransverseWeld;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberTransverseWeld, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_transverse_weld::ChangeMemberTransverseWeld> {
    Ok(match base.members.iter().find(|row| row.id == payload.member_id) {
        Some(row) => vec![Op::from(ChangeMemberTransverseWeld { member_id: payload.member_id.clone(), new_transverse_weld: row.transverse_weld })],
        None => Vec::new(),
    })
}
