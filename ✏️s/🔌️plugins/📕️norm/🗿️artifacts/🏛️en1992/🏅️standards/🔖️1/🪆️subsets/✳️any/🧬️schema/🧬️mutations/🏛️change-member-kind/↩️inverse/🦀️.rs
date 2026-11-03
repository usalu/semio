//! ↩️ `change-member-kind` inverse: restores the previous `kind`.

use crate::change_member_kind::ChangeMemberKind;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberKind, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_kind::ChangeMemberKind> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMemberKind, base: &En1992Snapshot) -> Option<ChangeMemberKind> {
    let r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    Some(ChangeMemberKind { member_id: payload.member_id.clone(), new_kind: r0.kind })
}
