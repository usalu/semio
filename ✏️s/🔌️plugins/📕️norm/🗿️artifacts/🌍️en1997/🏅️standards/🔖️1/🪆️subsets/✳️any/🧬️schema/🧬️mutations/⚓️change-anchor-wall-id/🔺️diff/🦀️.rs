//! 🔺️ `change-anchor-wall-id` diff: patches `wall_id` of the addressed anchor row.

use crate::change_anchor_wall_id::ChangeAnchorWallId;
use semio_s_artifact_norm_en1997_diff::{En1997AnchorsPatch, En1997AnchorsRows, En1997Diff};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &ChangeAnchorWallId, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    let Some(r0) = base.anchors.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("anchor {} missing", payload.id), vec![payload.id.clone()]);
    };
    if r0.wall_id == payload.new_wall_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The anchor already has this value.");
    }
    protocol::MutationOutcome::new(En1997Diff { anchors: Some(En1997AnchorsRows::modification(&payload.id, En1997AnchorsPatch { wall_id: Some(payload.new_wall_id.clone()), ..Default::default() })), ..Default::default() })
}
