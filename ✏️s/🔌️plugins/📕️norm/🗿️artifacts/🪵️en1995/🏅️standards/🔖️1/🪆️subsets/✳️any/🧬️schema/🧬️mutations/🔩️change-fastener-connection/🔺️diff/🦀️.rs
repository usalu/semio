use crate::change_fastener_connection::ChangeFastenerConnection;
use semio_s_artifact_norm_en1995_diff::En1995Diff;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_artifact_norm_en1995_diff::{En1995FastenerDelta, En1995FastenerPatch};
pub fn diff(payload: &ChangeFastenerConnection, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    let Some(row) = base.fasteners.iter().find(|item| item.id == payload.fastener_id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Unknown fastener id.", vec![payload.fastener_id.clone()]);
    };
    protocol::MutationOutcome::new(En1995Diff { fasteners: En1995FastenerDelta::modification(&row.id, En1995FastenerPatch { connection_id: Some(payload.new_value.clone()), ..Default::default() }), ..Default::default() })
}
