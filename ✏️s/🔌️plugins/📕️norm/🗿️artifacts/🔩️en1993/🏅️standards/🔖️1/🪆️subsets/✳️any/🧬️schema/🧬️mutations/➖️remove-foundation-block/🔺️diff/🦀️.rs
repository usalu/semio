//! ➖️ `remove-foundation-block` diff — removes the row at the index.

use crate::remove_foundation_block::RemoveFoundationBlock;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FoundationBlockDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &RemoveFoundationBlock, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.foundation_blocks.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("foundation-block index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1993Diff { foundation_blocks: En1993FoundationBlockDelta::removal(&base.foundation_blocks, payload.index), ..Default::default() })
}
