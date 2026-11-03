//! ↩️ `change-imperfection-y` inverse: restores the previous `y`.

use crate::change_imperfection_y::ChangeImperfectionY;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeImperfectionY, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_imperfection_y::ChangeImperfectionY> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeImperfectionY, base: &En1991Snapshot) -> Option<ChangeImperfectionY> {
    let r0 = base.execution_imperfections.iter().find(|row| row.id == payload.id)?;
    Some(ChangeImperfectionY { id: payload.id.clone(), new_y: r0.y })
}
