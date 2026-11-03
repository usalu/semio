//! ⚓️ `change-anchor-f-yk` diff: patches `f_yk` of the addressed anchor row.

use crate::change_anchor_f_yk::ChangeAnchorFYk;
use semio_s_artifact_norm_en1992_diff::{En1992AnchorsPatch, En1992AnchorsRows, En1992Diff};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &ChangeAnchorFYk, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if !payload.new_f_yk.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.anchors.iter().find(|row| row.id == payload.anchor_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Anchor {} does not exist.", payload.anchor_id), [payload.anchor_id.clone()]);
    };
    if r0.f_yk == payload.new_f_yk {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The anchor already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { anchors: Some(En1992AnchorsRows::modification(&payload.anchor_id, En1992AnchorsPatch { f_yk: Some(payload.new_f_yk), ..Default::default() })), ..Default::default() })
}
