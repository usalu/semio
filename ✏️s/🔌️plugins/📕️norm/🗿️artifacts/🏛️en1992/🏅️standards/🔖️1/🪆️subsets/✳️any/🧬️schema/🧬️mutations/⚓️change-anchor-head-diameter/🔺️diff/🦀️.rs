//! ⚓️ `change-anchor-head-diameter` diff: patches `head_diameter` of the addressed anchor row.

use crate::change_anchor_head_diameter::ChangeAnchorHeadDiameter;
use semio_s_artifact_norm_en1992_diff::{En1992AnchorsPatch, En1992AnchorsRows, En1992Diff};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &ChangeAnchorHeadDiameter, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if !payload.new_head_diameter.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.anchors.iter().find(|row| row.id == payload.anchor_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Anchor {} does not exist.", payload.anchor_id), [payload.anchor_id.clone()]);
    };
    if r0.head_diameter == payload.new_head_diameter {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The anchor already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { anchors: Some(En1992AnchorsRows::modification(&payload.anchor_id, En1992AnchorsPatch { head_diameter: Some(payload.new_head_diameter), ..Default::default() })), ..Default::default() })
}
