//! ↩️ `change-member-rd-equ-destab` inverse: restores the previous `rd_equ_destab`.

use crate::change_member_rd_equ_destab::ChangeMemberRdEquDestab;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberRdEquDestab, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_rd_equ_destab::ChangeMemberRdEquDestab> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberRdEquDestab, base: &En1990Snapshot) -> Option<ChangeMemberRdEquDestab> {
    let r0 = base.members.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMemberRdEquDestab { id: payload.id.clone(), new_rd_equ_destab: r0.rd_equ_destab })
}
