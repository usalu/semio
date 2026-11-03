//! ↩️ `change-member-sls-limit-qp` inverse: restores the previous `sls_limit_qp`.

use crate::change_member_sls_limit_qp::ChangeMemberSlsLimitQp;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberSlsLimitQp, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_sls_limit_qp::ChangeMemberSlsLimitQp> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberSlsLimitQp, base: &En1990Snapshot) -> Option<ChangeMemberSlsLimitQp> {
    let r0 = base.members.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMemberSlsLimitQp { id: payload.id.clone(), new_sls_limit_qp: r0.sls_limit_qp })
}
