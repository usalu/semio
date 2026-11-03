//! 🏋️ `insert-anchor-action` diff: inserts the anchor action row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_anchor_action::InsertAnchorAction;
use semio_s_artifact_norm_en1992_diff::{En1992AnchorsActionsRows, En1992AnchorsPatch, En1992AnchorsRows, En1992Diff};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &InsertAnchorAction, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    let Some(r0) = base.anchors.iter().find(|row| row.id == payload.anchor_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Anchor {} does not exist.", payload.anchor_id), [payload.anchor_id.clone()]);
    };
    if r0.actions.iter().any(|existing| existing.id == payload.anchor_action.id) {
        let key = payload.anchor_action.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Anchor action {key} already exists."), [key]);
    }
    if payload.index > r0.actions.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, r0.actions.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1992Diff { anchors: Some(En1992AnchorsRows::modification(&payload.anchor_id, En1992AnchorsPatch { actions: Some(En1992AnchorsActionsRows::insertion(payload.index, payload.anchor_action.clone())), ..Default::default() })), ..Default::default() })
}
