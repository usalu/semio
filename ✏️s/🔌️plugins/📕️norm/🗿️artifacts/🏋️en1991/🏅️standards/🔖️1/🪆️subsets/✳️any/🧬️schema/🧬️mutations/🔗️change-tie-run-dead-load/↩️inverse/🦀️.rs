//! ↩️ `change-tie-run-dead-load` inverse: restores the previous `dead_load`.

use crate::change_tie_run_dead_load::ChangeTieRunDeadLoad;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeTieRunDeadLoad, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tie_run_dead_load::ChangeTieRunDeadLoad> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTieRunDeadLoad, base: &En1991Snapshot) -> Option<ChangeTieRunDeadLoad> {
    let r0 = base.tie_runs.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTieRunDeadLoad { id: payload.id.clone(), new_dead_load: r0.dead_load })
}
