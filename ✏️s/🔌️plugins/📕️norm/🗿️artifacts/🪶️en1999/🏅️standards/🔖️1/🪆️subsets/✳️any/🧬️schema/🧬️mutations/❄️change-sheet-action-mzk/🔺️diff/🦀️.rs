//! 🔺️ `change-sheet-action-mzk` diff: patches `m_z_k` of the addressed cold-formed sheet action row.

use crate::change_sheet_action_m_z_k::ChangeSheetActionMZK;
use semio_s_artifact_norm_en1999_diff::{En1999ColdFormedActionsPatch, En1999ColdFormedActionsRows, En1999ColdFormedPatch, En1999ColdFormedRows, En1999Diff};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeSheetActionMZK, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_m_z_k.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.cold_formed.iter().find(|row| row.id == payload.cold_formed_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "cold-formed", Vec::<String>::new());
    };
    let Some(r1) = r0.actions.iter().find(|row| row.id == payload.action_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "action", Vec::<String>::new());
    };
    if r1.m_z_k == payload.new_m_z_k {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The cold-formed sheet action already has this value.");
    }
    protocol::MutationOutcome::new(En1999Diff { cold_formed: Some(En1999ColdFormedRows::modification(&payload.cold_formed_id, En1999ColdFormedPatch { actions: Some(En1999ColdFormedActionsRows::modification(&payload.action_id, En1999ColdFormedActionsPatch { m_z_k: Some(payload.new_m_z_k), ..Default::default() })), ..Default::default() })), ..Default::default() })
}
