//! ↩️ `change-ring-depth` inverse: restores the previous `depth`.

use crate::change_ring_depth::ChangeRingDepth;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeRingDepth, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_ring_depth::ChangeRingDepth> {
    Ok(match base.ring_stiffeners.iter().find(|row| row.id == payload.ring_stiffener_id) {
        Some(row) => vec![Op::from(ChangeRingDepth { ring_stiffener_id: payload.ring_stiffener_id.clone(), new_depth: row.depth })],
        None => Vec::new(),
    })
}
