//! ↩️ `change-foundation-block-width` inverse: restores the previous `block_width`.

use crate::change_foundation_block_width::ChangeFoundationBlockWidth;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFoundationBlockWidth, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_foundation_block_width::ChangeFoundationBlockWidth> {
    Ok(match base.foundation_blocks.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFoundationBlockWidth { id: payload.id.clone(), new_block_width: row.block_width })],
        None => Vec::new(),
    })
}
