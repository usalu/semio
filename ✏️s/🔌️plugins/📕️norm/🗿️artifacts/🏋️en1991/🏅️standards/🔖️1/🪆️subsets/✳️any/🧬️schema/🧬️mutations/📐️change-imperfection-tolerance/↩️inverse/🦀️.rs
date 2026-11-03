//! ↩️ `change-imperfection-tolerance` inverse: restores the previous `tolerance`.

use crate::change_imperfection_tolerance::ChangeImperfectionTolerance;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeImperfectionTolerance, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_imperfection_tolerance::ChangeImperfectionTolerance> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeImperfectionTolerance, base: &En1991Snapshot) -> Option<ChangeImperfectionTolerance> {
    let r0 = base.execution_imperfections.iter().find(|row| row.id == payload.id)?;
    Some(ChangeImperfectionTolerance { id: payload.id.clone(), new_tolerance: r0.tolerance })
}
