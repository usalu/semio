//! 🏋️ `remove-anchor-action` diff: removes the addressed anchor action row.

use crate::remove_anchor_action::RemoveAnchorAction;
use semio_s_artifact_norm_en1992_diff::{En1992AnchorsActionsRows, En1992AnchorsPatch, En1992AnchorsRows, En1992Diff};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &RemoveAnchorAction, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    let Some(r0) = base.anchors.iter().find(|row| row.id == payload.anchor_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Anchor {} does not exist.", payload.anchor_id), [payload.anchor_id.clone()]);
    };
    let index = payload.index;
    if index >= r0.actions.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "actions index out of range", [index.to_string()]);
    }
    protocol::MutationOutcome::new(En1992Diff { anchors: Some(En1992AnchorsRows::modification(&payload.anchor_id, En1992AnchorsPatch { actions: Some(En1992AnchorsActionsRows::removal(&r0.actions, index)), ..Default::default() })), ..Default::default() })
}
