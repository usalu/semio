//! ↩️ `change-placement-yaw` inverse: restores the previous `yaw`.

use crate::change_placement_yaw::ChangePlacementYaw;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangePlacementYaw, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_placement_yaw::ChangePlacementYaw> {
    Ok(match base.placements.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlacementYaw { id: payload.id.clone(), new_yaw: row.yaw })],
        None => Vec::new(),
    })
}
