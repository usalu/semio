//! 🔺️ `change-anchor-min-free-length` diff: patches `min_free_length` of the addressed anchor row.

use crate::change_anchor_min_free_length::ChangeAnchorMinFreeLength;
use semio_s_artifact_norm_en1997_diff::{En1997AnchorsPatch, En1997AnchorsRows, En1997Diff};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &ChangeAnchorMinFreeLength, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_min_free_length.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.anchors.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("anchor {} missing", payload.id), vec![payload.id.clone()]);
    };
    if r0.min_free_length == payload.new_min_free_length {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The anchor already has this value.");
    }
    protocol::MutationOutcome::new(En1997Diff { anchors: Some(En1997AnchorsRows::modification(&payload.id, En1997AnchorsPatch { min_free_length: Some(payload.new_min_free_length), ..Default::default() })), ..Default::default() })
}
