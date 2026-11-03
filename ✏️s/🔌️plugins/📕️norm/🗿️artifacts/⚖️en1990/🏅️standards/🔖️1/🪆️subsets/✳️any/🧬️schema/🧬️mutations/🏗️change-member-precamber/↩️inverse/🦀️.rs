//! ↩️ `change-member-precamber` inverse: restores the previous `precamber`.

use crate::change_member_precamber::ChangeMemberPrecamber;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberPrecamber, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_precamber::ChangeMemberPrecamber> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberPrecamber, base: &En1990Snapshot) -> Option<ChangeMemberPrecamber> {
    let r0 = base.members.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMemberPrecamber { id: payload.id.clone(), new_precamber: r0.precamber })
}
