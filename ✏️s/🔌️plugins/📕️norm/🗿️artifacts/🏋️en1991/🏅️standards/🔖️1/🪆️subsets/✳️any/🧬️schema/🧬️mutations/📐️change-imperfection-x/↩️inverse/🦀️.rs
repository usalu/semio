//! ↩️ `change-imperfection-x` inverse: restores the previous `x`.

use crate::change_imperfection_x::ChangeImperfectionX;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeImperfectionX, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_imperfection_x::ChangeImperfectionX> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeImperfectionX, base: &En1991Snapshot) -> Option<ChangeImperfectionX> {
    let r0 = base.execution_imperfections.iter().find(|row| row.id == payload.id)?;
    Some(ChangeImperfectionX { id: payload.id.clone(), new_x: r0.x })
}
