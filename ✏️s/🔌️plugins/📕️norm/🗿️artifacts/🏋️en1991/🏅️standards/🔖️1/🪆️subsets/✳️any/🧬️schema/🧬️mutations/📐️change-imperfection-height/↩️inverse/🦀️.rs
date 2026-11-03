//! ↩️ `change-imperfection-height` inverse: restores the previous `height`.

use crate::change_imperfection_height::ChangeImperfectionHeight;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeImperfectionHeight, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_imperfection_height::ChangeImperfectionHeight> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeImperfectionHeight, base: &En1991Snapshot) -> Option<ChangeImperfectionHeight> {
    let r0 = base.execution_imperfections.iter().find(|row| row.id == payload.id)?;
    Some(ChangeImperfectionHeight { id: payload.id.clone(), new_height: r0.height })
}
