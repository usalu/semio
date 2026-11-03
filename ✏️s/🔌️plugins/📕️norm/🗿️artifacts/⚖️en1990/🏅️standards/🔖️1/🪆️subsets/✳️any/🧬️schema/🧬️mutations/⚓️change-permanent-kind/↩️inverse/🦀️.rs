//! ↩️ `change-permanent-kind` inverse: restores the previous `kind`.

use crate::change_permanent_kind::ChangePermanentKind;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangePermanentKind, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_permanent_kind::ChangePermanentKind> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangePermanentKind, base: &En1990Snapshot) -> Option<ChangePermanentKind> {
    let r0 = base.permanents.iter().find(|row| row.id == payload.id)?;
    Some(ChangePermanentKind { id: payload.id.clone(), new_kind: r0.kind })
}
