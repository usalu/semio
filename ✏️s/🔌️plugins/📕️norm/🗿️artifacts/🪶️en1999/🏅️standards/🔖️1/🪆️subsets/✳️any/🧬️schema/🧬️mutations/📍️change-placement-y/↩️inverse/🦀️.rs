//! ↩️ `change-placement-y` inverse: restores the previous `y`.

use crate::change_placement_y::ChangePlacementY;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangePlacementY, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_placement_y::ChangePlacementY> {
    Ok(match base.placements.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlacementY { id: payload.id.clone(), new_y: row.y })],
        None => Vec::new(),
    })
}
