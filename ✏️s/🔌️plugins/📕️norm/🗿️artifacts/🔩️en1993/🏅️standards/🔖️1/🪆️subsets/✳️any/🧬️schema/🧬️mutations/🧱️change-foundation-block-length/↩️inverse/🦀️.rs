//! ↩️ `change-foundation-block-length` inverse: restores the previous `block_length`.

use crate::change_foundation_block_length::ChangeFoundationBlockLength;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFoundationBlockLength, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_foundation_block_length::ChangeFoundationBlockLength> {
    Ok(match base.foundation_blocks.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFoundationBlockLength { id: payload.id.clone(), new_block_length: row.block_length })],
        None => Vec::new(),
    })
}
