//! ↩️ `change-member-rd-equ-stab` inverse: restores the previous `rd_equ_stab`.

use crate::change_member_rd_equ_stab::ChangeMemberRdEquStab;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberRdEquStab, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_rd_equ_stab::ChangeMemberRdEquStab> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberRdEquStab, base: &En1990Snapshot) -> Option<ChangeMemberRdEquStab> {
    let r0 = base.members.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMemberRdEquStab { id: payload.id.clone(), new_rd_equ_stab: r0.rd_equ_stab })
}
