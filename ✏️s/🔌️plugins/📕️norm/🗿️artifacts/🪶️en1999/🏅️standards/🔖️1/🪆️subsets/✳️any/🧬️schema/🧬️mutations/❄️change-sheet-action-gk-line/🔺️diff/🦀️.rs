//! 🔺️ `change-sheet-action-gk-line` diff: patches `g_k_line` of the addressed cold-formed sheet action row.

use crate::change_sheet_action_g_k_line::ChangeSheetActionGKLine;
use semio_s_artifact_norm_en1999_diff::{En1999ColdFormedActionsPatch, En1999ColdFormedActionsRows, En1999ColdFormedPatch, En1999ColdFormedRows, En1999Diff};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeSheetActionGKLine, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_g_k_line.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.cold_formed.iter().find(|row| row.id == payload.cold_formed_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "cold-formed", Vec::<String>::new());
    };
    let Some(r1) = r0.actions.iter().find(|row| row.id == payload.action_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "action", Vec::<String>::new());
    };
    if r1.g_k_line == payload.new_g_k_line {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The cold-formed sheet action already has this value.");
    }
    protocol::MutationOutcome::new(En1999Diff { cold_formed: Some(En1999ColdFormedRows::modification(&payload.cold_formed_id, En1999ColdFormedPatch { actions: Some(En1999ColdFormedActionsRows::modification(&payload.action_id, En1999ColdFormedActionsPatch { g_k_line: Some(payload.new_g_k_line), ..Default::default() })), ..Default::default() })), ..Default::default() })
}
