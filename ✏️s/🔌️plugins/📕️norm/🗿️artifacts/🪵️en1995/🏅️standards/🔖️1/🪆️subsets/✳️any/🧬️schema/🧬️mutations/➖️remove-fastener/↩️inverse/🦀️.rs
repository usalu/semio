use crate::remove_fastener::RemoveFastener;
use crate::insert_fastener;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &RemoveFastener, base: &En1995Snapshot) -> Option<crate::insert_fastener::InsertFastener> {
    match base.fasteners.get(payload.index) {
        Some(item) => Some(insert_fastener::InsertFastener { index: payload.index, fastener: item.clone() }),
        None => None,
    }
}
