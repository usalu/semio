//! ↩️ `change-placement-y` inverse: restores the previous `y`.

use crate::change_placement_y::ChangePlacementY;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlacementY, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_placement_y::ChangePlacementY> {
    Ok(match base.placements.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlacementY { id: payload.id.clone(), new_y: row.y })],
        None => Vec::new(),
    })
}
