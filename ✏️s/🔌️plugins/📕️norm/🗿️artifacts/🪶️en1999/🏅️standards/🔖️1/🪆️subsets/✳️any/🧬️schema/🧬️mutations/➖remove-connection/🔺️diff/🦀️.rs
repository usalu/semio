//! 🔺️ `remove-connection` diff.

use crate::remove_connection::RemoveConnection;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999ConnectionsRows};

pub fn diff(payload: &RemoveConnection, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let Some(index) = base.connections.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Unknown connection id {}", payload.id), Vec::<String>::new());
    };
    protocol::MutationOutcome::new(En1999Diff { connections: Some(En1999ConnectionsRows::removal(&base.connections, index)), ..Default::default() })
}
