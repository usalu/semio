//! ↩️ `change-ring-x` inverse: restores the previous `x`.

use crate::change_ring_x::ChangeRingX;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeRingX, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_ring_x::ChangeRingX> {
    Ok(match base.ring_stiffeners.iter().find(|row| row.id == payload.ring_stiffener_id) {
        Some(row) => vec![Op::from(ChangeRingX { ring_stiffener_id: payload.ring_stiffener_id.clone(), new_x: row.x })],
        None => Vec::new(),
    })
}
