//! ↩️ `change-member-sls-limit-freq` inverse: restores the previous `sls_limit_freq`.

use crate::change_member_sls_limit_freq::ChangeMemberSlsLimitFreq;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberSlsLimitFreq, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_sls_limit_freq::ChangeMemberSlsLimitFreq> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberSlsLimitFreq, base: &En1990Snapshot) -> Option<ChangeMemberSlsLimitFreq> {
    let r0 = base.members.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMemberSlsLimitFreq { id: payload.id.clone(), new_sls_limit_freq: r0.sls_limit_freq })
}
