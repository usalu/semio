use crate::insert_foundation_block::InsertFoundationBlock;
use crate::remove_foundation_block;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &InsertFoundationBlock, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_foundation_block::RemoveFoundationBlock> {
    Ok((|| {
    let at = payload.index.unwrap_or(usize::MAX).min(base.foundation_blocks.len());
    vec![Op::from(remove_foundation_block::RemoveFoundationBlock { index: at })]

    })())
}
