//! ↩️ `change-derailment-y` inverse: restores the previous `y`.

use crate::change_derailment_y::ChangeDerailmentY;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeDerailmentY, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_derailment_y::ChangeDerailmentY> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeDerailmentY, base: &En1991Snapshot) -> Option<ChangeDerailmentY> {
    let r0 = base.derailment_cases.iter().find(|row| row.id == payload.id)?;
    Some(ChangeDerailmentY { id: payload.id.clone(), new_y: r0.y })
}
