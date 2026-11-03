use crate::insert_fastener::InsertFastener;
use crate::remove_fastener;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &InsertFastener, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_fastener::RemoveFastener> {
    let at = payload.index.min(base.fasteners.len());
    Ok(vec![Op::from(remove_fastener::RemoveFastener { index: at })])
}
