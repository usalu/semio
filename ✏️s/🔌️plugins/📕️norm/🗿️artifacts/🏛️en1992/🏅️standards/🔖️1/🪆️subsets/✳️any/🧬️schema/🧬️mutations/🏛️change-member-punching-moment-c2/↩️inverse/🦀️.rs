//! ↩️ `change-member-punching-moment-c2` inverse: restores the previous `punching.moment_c2`.

use crate::change_member_punching_moment_c2::ChangeMemberPunchingMomentC2;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberPunchingMomentC2, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_punching_moment_c2::ChangeMemberPunchingMomentC2> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberPunchingMomentC2, base: &En1992Snapshot) -> Option<ChangeMemberPunchingMomentC2> {
    let r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    let object = r0.punching.as_ref()?;
    Some(ChangeMemberPunchingMomentC2 { member_id: payload.member_id.clone(), new_punching_moment_c2: object.moment_c2 })
}
