use crate::change_placement_z::ChangePlacementZ;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangePlacementZ, base: &En1995Snapshot) -> Option<crate::change_placement_z::ChangePlacementZ> {
    match base.placements.iter().find(|item| item.id == payload.id) {
        Some(item) => Some(ChangePlacementZ { id: payload.id.clone(), new_value: item.z }),
        None => None,
    }
}
