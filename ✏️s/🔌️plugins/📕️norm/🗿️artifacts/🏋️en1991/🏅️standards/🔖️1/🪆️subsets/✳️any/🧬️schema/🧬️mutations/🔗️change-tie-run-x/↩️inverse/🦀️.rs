//! ↩️ `change-tie-run-x` inverse: restores the previous `x`.

use crate::change_tie_run_x::ChangeTieRunX;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeTieRunX, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tie_run_x::ChangeTieRunX> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTieRunX, base: &En1991Snapshot) -> Option<ChangeTieRunX> {
    let r0 = base.tie_runs.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTieRunX { id: payload.id.clone(), new_x: r0.x })
}
