//! ↩️ `change-member-deflection-limit-ratio` inverse: restores the previous `deflection_limit_ratio`.

use crate::change_member_deflection_limit_ratio::ChangeMemberDeflectionLimitRatio;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberDeflectionLimitRatio, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_deflection_limit_ratio::ChangeMemberDeflectionLimitRatio> {
    Ok(match base.members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeMemberDeflectionLimitRatio { id: payload.id.clone(), new_deflection_limit_ratio: row.deflection_limit_ratio })],
        None => Vec::new(),
    })
}
