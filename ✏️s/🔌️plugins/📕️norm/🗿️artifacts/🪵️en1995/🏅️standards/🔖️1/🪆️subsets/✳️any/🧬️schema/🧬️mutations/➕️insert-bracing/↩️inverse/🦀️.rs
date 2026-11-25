use crate::insert_bracing::InsertBracing;
use crate::remove_bracing;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &InsertBracing, base: &En1995Snapshot) -> Option<crate::remove_bracing::RemoveBracing> {
    let at = payload.index.min(base.bracings.len());
    Some(remove_bracing::RemoveBracing { index: at })
}
