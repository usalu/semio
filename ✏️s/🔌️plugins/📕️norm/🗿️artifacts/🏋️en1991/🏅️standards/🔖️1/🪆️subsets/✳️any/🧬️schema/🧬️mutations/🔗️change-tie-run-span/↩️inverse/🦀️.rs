//! ↩️ `change-tie-run-span` inverse: restores the previous `span`.

use crate::change_tie_run_span::ChangeTieRunSpan;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeTieRunSpan, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tie_run_span::ChangeTieRunSpan> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTieRunSpan, base: &En1991Snapshot) -> Option<ChangeTieRunSpan> {
    let r0 = base.tie_runs.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTieRunSpan { id: payload.id.clone(), new_span: r0.span })
}
