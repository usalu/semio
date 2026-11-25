use crate::change_placement_yaw::ChangePlacementYaw;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangePlacementYaw, base: &En1995Snapshot) -> Option<crate::change_placement_yaw::ChangePlacementYaw> {
    match base.placements.iter().find(|item| item.id == payload.id) {
        Some(item) => Some(ChangePlacementYaw { id: payload.id.clone(), new_value: item.yaw }),
        None => None,
    }
}
