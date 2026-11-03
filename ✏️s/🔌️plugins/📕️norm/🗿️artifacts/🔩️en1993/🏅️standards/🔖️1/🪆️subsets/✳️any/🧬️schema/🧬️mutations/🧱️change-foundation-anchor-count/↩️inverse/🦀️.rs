//! ↩️ `change-foundation-anchor-count` inverse: restores the previous `anchor_count`.

use crate::change_foundation_anchor_count::ChangeFoundationAnchorCount;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFoundationAnchorCount, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_foundation_anchor_count::ChangeFoundationAnchorCount> {
    Ok(match base.foundation_blocks.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFoundationAnchorCount { id: payload.id.clone(), new_anchor_count: row.anchor_count })],
        None => Vec::new(),
    })
}
