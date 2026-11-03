//! ↩️ `change-falling-x` inverse: restores the previous `x`.

use crate::change_falling_x::ChangeFallingX;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeFallingX, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_falling_x::ChangeFallingX> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFallingX, base: &En1991Snapshot) -> Option<ChangeFallingX> {
    let r0 = base.falling_loads.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFallingX { id: payload.id.clone(), new_x: r0.x })
}
