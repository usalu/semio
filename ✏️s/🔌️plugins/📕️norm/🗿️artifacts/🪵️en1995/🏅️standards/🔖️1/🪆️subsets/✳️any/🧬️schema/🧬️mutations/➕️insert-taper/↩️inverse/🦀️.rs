use crate::insert_taper::InsertTaper;
use crate::remove_taper;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &InsertTaper, base: &En1995Snapshot) -> Option<crate::remove_taper::RemoveTaper> {
    let at = payload.index.min(base.tapers.len());
    Some(remove_taper::RemoveTaper { index: at })
}
