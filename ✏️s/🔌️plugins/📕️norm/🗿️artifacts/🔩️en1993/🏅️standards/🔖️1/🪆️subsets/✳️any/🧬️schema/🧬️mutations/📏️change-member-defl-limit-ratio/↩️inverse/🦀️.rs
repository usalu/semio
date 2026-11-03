//! ↩️ `change-member-defl-limit-ratio` inverse: restores the previous `deflection_limit_ratio`.

use crate::change_member_defl_limit_ratio::ChangeMemberDeflLimitRatio;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberDeflLimitRatio, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_defl_limit_ratio::ChangeMemberDeflLimitRatio> {
    Ok(match base.members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeMemberDeflLimitRatio { id: payload.id.clone(), new_deflection_limit_ratio: row.deflection_limit_ratio })],
        None => Vec::new(),
    })
}
