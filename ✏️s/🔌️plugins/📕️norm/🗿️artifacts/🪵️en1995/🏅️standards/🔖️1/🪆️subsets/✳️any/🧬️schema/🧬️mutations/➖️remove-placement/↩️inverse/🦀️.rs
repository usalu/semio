use crate::remove_placement::RemovePlacement;
use crate::insert_placement;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &RemovePlacement, base: &En1995Snapshot) -> Option<crate::insert_placement::InsertPlacement> {
    match base.placements.get(payload.index) {
        Some(item) => Some(insert_placement::InsertPlacement { index: payload.index, placement: item.clone() }),
        None => None,
    }
}
