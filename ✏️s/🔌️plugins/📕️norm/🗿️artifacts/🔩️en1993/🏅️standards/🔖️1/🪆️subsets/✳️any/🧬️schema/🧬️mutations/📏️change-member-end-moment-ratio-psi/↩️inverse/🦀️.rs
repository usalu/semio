//! ↩️ `change-member-end-moment-ratio-psi` inverse: restores the previous `end_moment_ratio_psi`.

use crate::change_member_end_moment_ratio_psi::ChangeMemberEndMomentRatioPsi;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberEndMomentRatioPsi, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_end_moment_ratio_psi::ChangeMemberEndMomentRatioPsi> {
    Ok(match base.members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeMemberEndMomentRatioPsi { id: payload.id.clone(), new_end_moment_ratio_psi: row.end_moment_ratio_psi })],
        None => Vec::new(),
    })
}
