//! ⚓️ `change-anchor-c1` diff: patches `c1` of the addressed anchor row.

use crate::change_anchor_c1::ChangeAnchorC1;
use semio_s_artifact_norm_en1992_diff::{En1992AnchorsPatch, En1992AnchorsRows, En1992Diff};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &ChangeAnchorC1, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if !payload.new_c1.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.anchors.iter().find(|row| row.id == payload.anchor_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Anchor {} does not exist.", payload.anchor_id), [payload.anchor_id.clone()]);
    };
    if r0.c1 == payload.new_c1 {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The anchor already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { anchors: Some(En1992AnchorsRows::modification(&payload.anchor_id, En1992AnchorsPatch { c1: Some(payload.new_c1), ..Default::default() })), ..Default::default() })
}
