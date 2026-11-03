//! ↩️ `change-tie-run-y` inverse: restores the previous `y`.

use crate::change_tie_run_y::ChangeTieRunY;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeTieRunY, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tie_run_y::ChangeTieRunY> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTieRunY, base: &En1991Snapshot) -> Option<ChangeTieRunY> {
    let r0 = base.tie_runs.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTieRunY { id: payload.id.clone(), new_y: r0.y })
}
