//! ↩️ `change-tie-run-kind` inverse: restores the previous `kind`.

use crate::change_tie_run_kind::ChangeTieRunKind;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeTieRunKind, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tie_run_kind::ChangeTieRunKind> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTieRunKind, base: &En1991Snapshot) -> Option<ChangeTieRunKind> {
    let r0 = base.tie_runs.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTieRunKind { id: payload.id.clone(), new_kind: r0.kind })
}
