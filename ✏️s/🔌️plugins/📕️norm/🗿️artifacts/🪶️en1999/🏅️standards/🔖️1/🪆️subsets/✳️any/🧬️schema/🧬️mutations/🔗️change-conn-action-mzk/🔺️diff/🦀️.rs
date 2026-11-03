//! 🔺️ `change-conn-action-mzk` diff: patches `m_z_k` of the addressed connection action row.

use crate::change_conn_action_m_z_k::ChangeConnActionMZK;
use semio_s_artifact_norm_en1999_diff::{En1999ConnectionsActionsPatch, En1999ConnectionsActionsRows, En1999ConnectionsPatch, En1999ConnectionsRows, En1999Diff};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeConnActionMZK, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_m_z_k.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.connections.iter().find(|row| row.id == payload.connection_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "connection", Vec::<String>::new());
    };
    let Some(r1) = r0.actions.iter().find(|row| row.id == payload.action_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "action", Vec::<String>::new());
    };
    if r1.m_z_k == payload.new_m_z_k {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The connection action already has this value.");
    }
    protocol::MutationOutcome::new(En1999Diff { connections: Some(En1999ConnectionsRows::modification(&payload.connection_id, En1999ConnectionsPatch { actions: Some(En1999ConnectionsActionsRows::modification(&payload.action_id, En1999ConnectionsActionsPatch { m_z_k: Some(payload.new_m_z_k), ..Default::default() })), ..Default::default() })), ..Default::default() })
}
