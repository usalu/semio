//! ↩️ `change-action-kind` inverse: restores the previous `kind`.

use crate::change_action_kind::ChangeActionKind;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeActionKind, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_action_kind::ChangeActionKind> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeActionKind, base: &En1992Snapshot) -> Option<ChangeActionKind> {
    let r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    let r1 = r0.actions.iter().find(|row| row.id == payload.action_id)?;
    Some(ChangeActionKind { member_id: payload.member_id.clone(), action_id: payload.action_id.clone(), new_kind: r1.kind })
}
