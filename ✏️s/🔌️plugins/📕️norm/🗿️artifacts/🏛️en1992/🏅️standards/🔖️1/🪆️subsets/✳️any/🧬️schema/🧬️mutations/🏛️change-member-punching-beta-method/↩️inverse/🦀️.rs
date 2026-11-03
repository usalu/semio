//! ↩️ `change-member-punching-beta-method` inverse: restores the previous `punching.beta_method`.

use crate::change_member_punching_beta_method::ChangeMemberPunchingBetaMethod;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberPunchingBetaMethod, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_punching_beta_method::ChangeMemberPunchingBetaMethod> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberPunchingBetaMethod, base: &En1992Snapshot) -> Option<ChangeMemberPunchingBetaMethod> {
    let r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    let object = r0.punching.as_ref()?;
    Some(ChangeMemberPunchingBetaMethod { member_id: payload.member_id.clone(), new_punching_beta_method: object.beta_method })
}
