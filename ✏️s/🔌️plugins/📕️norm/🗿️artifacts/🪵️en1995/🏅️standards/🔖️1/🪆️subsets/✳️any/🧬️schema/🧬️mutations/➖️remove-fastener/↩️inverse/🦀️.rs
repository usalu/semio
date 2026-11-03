use crate::remove_fastener::RemoveFastener;
use crate::insert_fastener;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &RemoveFastener, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_fastener::InsertFastener> {
    Ok(match base.fasteners.get(payload.index) {
        Some(item) => vec![Op::from(insert_fastener::InsertFastener { index: payload.index, fastener: item.clone() })],
        None => Vec::new(),
    })
}
