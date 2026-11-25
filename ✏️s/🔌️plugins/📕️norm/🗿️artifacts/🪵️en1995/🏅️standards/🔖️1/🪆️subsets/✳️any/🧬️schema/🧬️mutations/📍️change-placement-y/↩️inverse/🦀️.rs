use crate::change_placement_y::ChangePlacementY;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangePlacementY, base: &En1995Snapshot) -> Option<crate::change_placement_y::ChangePlacementY> {
    match base.placements.iter().find(|item| item.id == payload.id) {
        Some(item) => Some(ChangePlacementY { id: payload.id.clone(), new_value: item.y }),
        None => None,
    }
}
