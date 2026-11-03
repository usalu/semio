//! ↩️ `change-placement-z` inverse: restores the previous `z`.

use crate::change_placement_z::ChangePlacementZ;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangePlacementZ, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_placement_z::ChangePlacementZ> {
    Ok(match base.placements.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlacementZ { id: payload.id.clone(), new_z: row.z })],
        None => Vec::new(),
    })
}
