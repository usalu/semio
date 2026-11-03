//! ↩️ `change-deflection-limit` inverse: restores the previous `deflection_limit`.

use crate::change_deflection_limit::ChangeDeflectionLimit;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeDeflectionLimit, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_deflection_limit::ChangeDeflectionLimit> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeDeflectionLimit, base: &En1996Snapshot) -> Option<ChangeDeflectionLimit> {
    let r0 = base.walls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeDeflectionLimit { id: payload.id.clone(), new_deflection_limit: r0.deflection_limit })
}
