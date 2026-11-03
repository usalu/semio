//! ↩️ `change-chase-kind` inverse: restores the previous `kind`.

use crate::change_chase_kind::ChangeChaseKind;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeChaseKind, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_chase_kind::ChangeChaseKind> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeChaseKind, base: &En1996Snapshot) -> Option<ChangeChaseKind> {
    let r0 = base.chases.iter().find(|row| row.id == payload.id)?;
    Some(ChangeChaseKind { id: payload.id.clone(), new_kind: r0.kind })
}
