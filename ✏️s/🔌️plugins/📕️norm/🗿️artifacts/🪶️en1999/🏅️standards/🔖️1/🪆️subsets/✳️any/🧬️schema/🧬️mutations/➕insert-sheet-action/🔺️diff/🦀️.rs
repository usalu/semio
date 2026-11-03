//! 🔺️ `insert-sheet-action` diff.

use crate::insert_sheet_action::InsertSheetAction;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999ColdFormedPatch, En1999ColdFormedRows, En1999ColdFormedActionsRows};

pub fn diff(payload: &InsertSheetAction, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let Some(parent) = base.cold_formed.iter().find(|row| row.id == payload.cold_formed_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Unknown cold-formed sheet id {}", payload.cold_formed_id), vec![payload.cold_formed_id.clone()]);
    };
    if parent.actions.iter().any(|existing| existing.id == payload.action.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Cold-formed sheet action id {} already exists.", payload.action.id), [payload.action.id.clone()]);
    }
    if (payload.index as usize) > parent.actions.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, parent.actions.len()), Vec::<String>::new());
    }
    let nested = En1999ColdFormedActionsRows::insertion(payload.index as usize, payload.action.clone());
    protocol::MutationOutcome::new(En1999Diff { cold_formed: Some(En1999ColdFormedRows::modification(payload.cold_formed_id.clone(), En1999ColdFormedPatch { actions: Some(nested), ..Default::default() })), ..Default::default() })
}
