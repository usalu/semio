use crate::insert_placement::InsertPlacement;
use crate::remove_placement;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &InsertPlacement, base: &En1995Snapshot) -> Option<crate::remove_placement::RemovePlacement> {
    let at = payload.index.min(base.placements.len());
    Some(remove_placement::RemovePlacement { index: at })
}
