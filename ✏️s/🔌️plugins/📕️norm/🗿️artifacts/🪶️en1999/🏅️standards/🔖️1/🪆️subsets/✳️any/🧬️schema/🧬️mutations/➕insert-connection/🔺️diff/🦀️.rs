//! 🔺️ `insert-connection` diff.

use crate::insert_connection::InsertConnection;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999ConnectionsRows};

pub fn diff(payload: &InsertConnection, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if base.connections.iter().any(|existing| existing.id == payload.connection.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Connection id {} already exists.", payload.connection.id), [payload.connection.id.clone()]);
    }
    if (payload.index as usize) > base.connections.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", (payload.index as usize), base.connections.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1999Diff { connections: Some(En1999ConnectionsRows::insertion(payload.index as usize, payload.connection.clone())), ..Default::default() })
}
