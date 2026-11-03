//! ↩️ `change-member-punching-asw` inverse: restores the previous `punching.asw`.

use crate::change_member_punching_asw::ChangeMemberPunchingAsw;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberPunchingAsw, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_punching_asw::ChangeMemberPunchingAsw> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberPunchingAsw, base: &En1992Snapshot) -> Option<ChangeMemberPunchingAsw> {
    let r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    let object = r0.punching.as_ref()?;
    Some(ChangeMemberPunchingAsw { member_id: payload.member_id.clone(), new_punching_asw: object.asw })
}
