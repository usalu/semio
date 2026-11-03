//! 🔺️ `change-anchor-head-depth` diff: patches `head_depth` of the addressed anchor row.

use crate::change_anchor_head_depth::ChangeAnchorHeadDepth;
use semio_s_artifact_norm_en1997_diff::{En1997AnchorsPatch, En1997AnchorsRows, En1997Diff};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &ChangeAnchorHeadDepth, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_head_depth.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.anchors.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("anchor {} missing", payload.id), vec![payload.id.clone()]);
    };
    if r0.head_depth == payload.new_head_depth {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The anchor already has this value.");
    }
    protocol::MutationOutcome::new(En1997Diff { anchors: Some(En1997AnchorsRows::modification(&payload.id, En1997AnchorsPatch { head_depth: Some(payload.new_head_depth), ..Default::default() })), ..Default::default() })
}
