//! ↩️ `change-member-prestress-area` inverse: restores the previous `prestress.area`.

use crate::change_member_prestress_area::ChangeMemberPrestressArea;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberPrestressArea, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_prestress_area::ChangeMemberPrestressArea> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberPrestressArea, base: &En1992Snapshot) -> Option<ChangeMemberPrestressArea> {
    let r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    let object = r0.prestress.as_ref()?;
    Some(ChangeMemberPrestressArea { member_id: payload.member_id.clone(), new_prestress_area: object.area })
}
