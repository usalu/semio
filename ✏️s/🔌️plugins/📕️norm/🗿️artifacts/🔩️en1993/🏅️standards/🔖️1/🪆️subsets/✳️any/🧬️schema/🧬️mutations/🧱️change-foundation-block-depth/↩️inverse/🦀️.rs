//! ↩️ `change-foundation-block-depth` inverse: restores the previous `block_depth`.

use crate::change_foundation_block_depth::ChangeFoundationBlockDepth;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFoundationBlockDepth, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_foundation_block_depth::ChangeFoundationBlockDepth> {
    Ok(match base.foundation_blocks.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFoundationBlockDepth { id: payload.id.clone(), new_block_depth: row.block_depth })],
        None => Vec::new(),
    })
}
