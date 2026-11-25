use crate::change_placement_x::ChangePlacementX;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangePlacementX, base: &En1995Snapshot) -> Option<crate::change_placement_x::ChangePlacementX> {
    match base.placements.iter().find(|item| item.id == payload.id) {
        Some(item) => Some(ChangePlacementX { id: payload.id.clone(), new_value: item.x }),
        None => None,
    }
}
