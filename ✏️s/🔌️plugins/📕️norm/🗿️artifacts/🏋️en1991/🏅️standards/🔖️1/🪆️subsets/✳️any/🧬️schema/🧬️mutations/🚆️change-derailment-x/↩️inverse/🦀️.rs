//! ↩️ `change-derailment-x` inverse: restores the previous `x`.

use crate::change_derailment_x::ChangeDerailmentX;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeDerailmentX, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_derailment_x::ChangeDerailmentX> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeDerailmentX, base: &En1991Snapshot) -> Option<ChangeDerailmentX> {
    let r0 = base.derailment_cases.iter().find(|row| row.id == payload.id)?;
    Some(ChangeDerailmentX { id: payload.id.clone(), new_x: r0.x })
}
