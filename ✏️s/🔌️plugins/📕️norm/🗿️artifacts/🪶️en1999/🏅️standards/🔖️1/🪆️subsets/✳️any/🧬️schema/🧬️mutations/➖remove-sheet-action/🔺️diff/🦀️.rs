//! 🔺️ `remove-sheet-action` diff.

use crate::remove_sheet_action::RemoveSheetAction;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999ColdFormedPatch, En1999ColdFormedRows, En1999ColdFormedActionsRows};

pub fn diff(payload: &RemoveSheetAction, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let Some(parent) = base.cold_formed.iter().find(|row| row.id == payload.cold_formed_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Unknown cold-formed sheet id {}", payload.cold_formed_id), vec![payload.cold_formed_id.clone()]);
    };
    let Some(index) = parent.actions.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Unknown cold-formed sheet action id {}", payload.id), Vec::<String>::new());
    };
    let nested = En1999ColdFormedActionsRows::removal(&parent.actions, index);
    protocol::MutationOutcome::new(En1999Diff { cold_formed: Some(En1999ColdFormedRows::modification(payload.cold_formed_id.clone(), En1999ColdFormedPatch { actions: Some(nested), ..Default::default() })), ..Default::default() })
}
