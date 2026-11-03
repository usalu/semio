//! ↩️ `change-tie-run-psi` inverse: restores the previous `psi`.

use crate::change_tie_run_psi::ChangeTieRunPsi;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeTieRunPsi, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tie_run_psi::ChangeTieRunPsi> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTieRunPsi, base: &En1991Snapshot) -> Option<ChangeTieRunPsi> {
    let r0 = base.tie_runs.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTieRunPsi { id: payload.id.clone(), new_psi: r0.psi })
}
