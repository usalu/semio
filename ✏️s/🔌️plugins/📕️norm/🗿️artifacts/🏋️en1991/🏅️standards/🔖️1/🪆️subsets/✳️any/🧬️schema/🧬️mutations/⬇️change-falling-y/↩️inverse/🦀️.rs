//! ↩️ `change-falling-y` inverse: restores the previous `y`.

use crate::change_falling_y::ChangeFallingY;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeFallingY, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_falling_y::ChangeFallingY> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFallingY, base: &En1991Snapshot) -> Option<ChangeFallingY> {
    let r0 = base.falling_loads.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFallingY { id: payload.id.clone(), new_y: r0.y })
}
