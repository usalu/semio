//! ↩️ `change-member-prestress-force` inverse: restores the previous `prestress.force`.

use crate::change_member_prestress_force::ChangeMemberPrestressForce;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberPrestressForce, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_prestress_force::ChangeMemberPrestressForce> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberPrestressForce, base: &En1992Snapshot) -> Option<ChangeMemberPrestressForce> {
    let r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    let object = r0.prestress.as_ref()?;
    Some(ChangeMemberPrestressForce { member_id: payload.member_id.clone(), new_prestress_force: object.force })
}
