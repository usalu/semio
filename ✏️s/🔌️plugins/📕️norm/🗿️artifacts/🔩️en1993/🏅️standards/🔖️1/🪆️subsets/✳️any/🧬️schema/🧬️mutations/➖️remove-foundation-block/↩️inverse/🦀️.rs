use crate::remove_foundation_block::RemoveFoundationBlock;
use crate::insert_foundation_block;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &RemoveFoundationBlock, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_foundation_block::InsertFoundationBlock> {
    Ok((|| {
    if payload.index >= base.foundation_blocks.len() { return Vec::new(); }
    vec![Op::from(insert_foundation_block::InsertFoundationBlock { index: Some(payload.index), foundation_block: base.foundation_blocks[payload.index].clone() })]

    })())
}
