//! ➕️ `insert-foundation-block` diff — inserts the row at its position, clamped to the end of the collection.

use crate::insert_foundation_block::InsertFoundationBlock;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FoundationBlockDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &InsertFoundationBlock, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.foundation_blocks.iter().any(|existing| existing.id == payload.foundation_block.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Foundation block id {} already exists.", payload.foundation_block.id), [payload.foundation_block.id.clone()]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.foundation_blocks.len());
    protocol::MutationOutcome::new(En1993Diff { foundation_blocks: En1993FoundationBlockDelta::insertion(index, payload.foundation_block.clone()), ..Default::default() })
}
