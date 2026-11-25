use crate::insert_fastener::InsertFastener;
use crate::remove_fastener;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &InsertFastener, base: &En1995Snapshot) -> Option<crate::remove_fastener::RemoveFastener> {
    let at = payload.index.min(base.fasteners.len());
    Some(remove_fastener::RemoveFastener { index: at })
}
