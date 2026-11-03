//! ↩️ `change-member-span` inverse: restores the previous `span`.

use crate::change_member_span::ChangeMemberSpan;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberSpan, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_span::ChangeMemberSpan> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberSpan, base: &En1990Snapshot) -> Option<ChangeMemberSpan> {
    let r0 = base.members.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMemberSpan { id: payload.id.clone(), new_span: r0.span })
}
