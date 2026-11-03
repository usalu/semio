//! ↩️ `change-member-defl-limit-ratio` inverse: restores the previous `deflection_limit_ratio`.

use crate::change_member_defl_limit_ratio::ChangeMemberDeflLimitRatio;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberDeflLimitRatio, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_defl_limit_ratio::ChangeMemberDeflLimitRatio> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberDeflLimitRatio, base: &En1990Snapshot) -> Option<ChangeMemberDeflLimitRatio> {
    let r0 = base.members.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMemberDeflLimitRatio { id: payload.id.clone(), new_deflection_limit_ratio: r0.deflection_limit_ratio })
}
