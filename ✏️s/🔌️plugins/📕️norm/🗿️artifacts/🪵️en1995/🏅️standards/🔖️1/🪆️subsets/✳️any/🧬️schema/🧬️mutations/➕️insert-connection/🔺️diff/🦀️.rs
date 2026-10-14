use crate::insert_connection::InsertConnection;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::En1995ConnectionDelta;
pub fn diff(payload: &InsertConnection, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if base.connections.iter().any(|existing| existing.id == payload.connection.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Connection id {} already exists.", payload.connection.id), [payload.connection.id.clone()]);
    }
    let at = payload.index.min(base.connections.len());
    protocol::MutationOutcome::new(En1995Diff { connections: En1995ConnectionDelta::insertion(at, payload.connection.clone()), ..Default::default() })
}
