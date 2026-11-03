//! ↩️ `change-imperfection-vertical-load` inverse: restores the previous `vertical_load`.

use crate::change_imperfection_vertical_load::ChangeImperfectionVerticalLoad;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeImperfectionVerticalLoad, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_imperfection_vertical_load::ChangeImperfectionVerticalLoad> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeImperfectionVerticalLoad, base: &En1991Snapshot) -> Option<ChangeImperfectionVerticalLoad> {
    let r0 = base.execution_imperfections.iter().find(|row| row.id == payload.id)?;
    Some(ChangeImperfectionVerticalLoad { id: payload.id.clone(), new_vertical_load: r0.vertical_load })
}
