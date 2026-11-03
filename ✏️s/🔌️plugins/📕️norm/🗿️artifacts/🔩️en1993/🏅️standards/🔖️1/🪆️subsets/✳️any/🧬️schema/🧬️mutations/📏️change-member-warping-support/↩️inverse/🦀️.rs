//! ↩️ `change-member-warping-support` inverse: restores the previous `warping_support`.

use crate::change_member_warping_support::ChangeMemberWarpingSupport;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberWarpingSupport, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_warping_support::ChangeMemberWarpingSupport> {
    Ok(match base.members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeMemberWarpingSupport { id: payload.id.clone(), new_warping_support: row.warping_support })],
        None => Vec::new(),
    })
}
