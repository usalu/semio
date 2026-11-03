//! ↩️ `change-falling-drop-height` inverse: restores the previous `drop_height`.

use crate::change_falling_drop_height::ChangeFallingDropHeight;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeFallingDropHeight, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_falling_drop_height::ChangeFallingDropHeight> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeFallingDropHeight, base: &En1991Snapshot) -> Option<ChangeFallingDropHeight> {
    let r0 = base.falling_loads.iter().find(|row| row.id == payload.id)?;
    Some(ChangeFallingDropHeight { id: payload.id.clone(), new_drop_height: r0.drop_height })
}
