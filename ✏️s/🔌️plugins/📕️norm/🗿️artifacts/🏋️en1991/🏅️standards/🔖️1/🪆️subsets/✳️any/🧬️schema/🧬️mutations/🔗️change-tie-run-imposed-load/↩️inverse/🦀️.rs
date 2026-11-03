//! ↩️ `change-tie-run-imposed-load` inverse: restores the previous `imposed_load`.

use crate::change_tie_run_imposed_load::ChangeTieRunImposedLoad;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeTieRunImposedLoad, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tie_run_imposed_load::ChangeTieRunImposedLoad> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTieRunImposedLoad, base: &En1991Snapshot) -> Option<ChangeTieRunImposedLoad> {
    let r0 = base.tie_runs.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTieRunImposedLoad { id: payload.id.clone(), new_imposed_load: r0.imposed_load })
}
