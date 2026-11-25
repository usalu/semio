use crate::remove_bracing::RemoveBracing;
use crate::insert_bracing;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &RemoveBracing, base: &En1995Snapshot) -> Option<crate::insert_bracing::InsertBracing> {
    match base.bracings.get(payload.index) {
        Some(item) => Some(insert_bracing::InsertBracing { index: payload.index, bracing: item.clone() }),
        None => None,
    }
}
