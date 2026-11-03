//! 🔺️ `change-foundation-anchor-edge-distance` diff: patches `anchor_edge_distance` of the addressed foundation block row.

use crate::change_foundation_anchor_edge_distance::ChangeFoundationAnchorEdgeDistance;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FoundationBlockDelta, En1993FoundationBlockPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeFoundationAnchorEdgeDistance, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_anchor_edge_distance.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.foundation_blocks.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "foundation-block", Vec::<String>::new());
    };
    if row.anchor_edge_distance == payload.new_anchor_edge_distance {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The foundation block already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { foundation_blocks: En1993FoundationBlockDelta::modification(payload.id.clone(), En1993FoundationBlockPatch { anchor_edge_distance: Some(payload.new_anchor_edge_distance), ..Default::default() }), ..Default::default() })
}
