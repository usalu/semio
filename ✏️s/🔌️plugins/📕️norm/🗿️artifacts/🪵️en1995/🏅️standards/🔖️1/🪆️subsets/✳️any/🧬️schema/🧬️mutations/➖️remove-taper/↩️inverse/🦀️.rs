use crate::remove_taper::RemoveTaper;
use crate::insert_taper;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &RemoveTaper, base: &En1995Snapshot) -> Option<crate::insert_taper::InsertTaper> {
    match base.tapers.get(payload.index) {
        Some(item) => Some(insert_taper::InsertTaper { index: payload.index, taper: item.clone() }),
        None => None,
    }
}
