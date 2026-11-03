//! ↩️ `change-tie-run-spacing` inverse: restores the previous `spacing`.

use crate::change_tie_run_spacing::ChangeTieRunSpacing;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeTieRunSpacing, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tie_run_spacing::ChangeTieRunSpacing> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTieRunSpacing, base: &En1991Snapshot) -> Option<ChangeTieRunSpacing> {
    let r0 = base.tie_runs.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTieRunSpacing { id: payload.id.clone(), new_spacing: r0.spacing })
}
