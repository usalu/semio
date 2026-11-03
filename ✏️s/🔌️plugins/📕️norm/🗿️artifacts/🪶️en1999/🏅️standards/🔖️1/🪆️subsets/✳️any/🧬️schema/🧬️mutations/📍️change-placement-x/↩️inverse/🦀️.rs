//! ↩️ `change-placement-x` inverse: restores the previous `x`.

use crate::change_placement_x::ChangePlacementX;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangePlacementX, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_placement_x::ChangePlacementX> {
    Ok(match base.placements.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlacementX { id: payload.id.clone(), new_x: row.x })],
        None => Vec::new(),
    })
}
