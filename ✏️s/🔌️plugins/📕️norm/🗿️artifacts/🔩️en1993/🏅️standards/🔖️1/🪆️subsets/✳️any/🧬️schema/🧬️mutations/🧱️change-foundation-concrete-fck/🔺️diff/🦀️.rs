//! 🔺️ `change-foundation-concrete-fck` diff: patches `concrete_fck` of the addressed foundation block row.

use crate::change_foundation_concrete_fck::ChangeFoundationConcreteFck;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FoundationBlockDelta, En1993FoundationBlockPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeFoundationConcreteFck, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_concrete_fck.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.foundation_blocks.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "foundation-block", Vec::<String>::new());
    };
    if row.concrete_fck == payload.new_concrete_fck {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The foundation block already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { foundation_blocks: En1993FoundationBlockDelta::modification(payload.id.clone(), En1993FoundationBlockPatch { concrete_fck: Some(payload.new_concrete_fck), ..Default::default() }), ..Default::default() })
}
