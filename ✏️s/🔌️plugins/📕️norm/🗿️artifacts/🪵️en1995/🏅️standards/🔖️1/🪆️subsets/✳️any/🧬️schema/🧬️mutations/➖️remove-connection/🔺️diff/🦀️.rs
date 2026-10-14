use crate::remove_connection::RemoveConnection;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::En1995ConnectionDelta;
pub fn diff(payload: &RemoveConnection, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if payload.index >= base.connections.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Connection index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1995Diff { connections: En1995ConnectionDelta::removal(&base.connections, payload.index), ..Default::default() })
}
