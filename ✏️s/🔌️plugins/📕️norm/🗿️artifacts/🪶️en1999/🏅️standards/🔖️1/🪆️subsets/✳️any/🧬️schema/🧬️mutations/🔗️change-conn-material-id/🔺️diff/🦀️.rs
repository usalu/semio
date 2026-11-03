//! 🔺️ `change-conn-material-id` diff: patches `material_id` of the addressed connection row.

use crate::change_conn_material_id::ChangeConnMaterialId;
use semio_s_artifact_norm_en1999_diff::{En1999ConnectionsPatch, En1999ConnectionsRows, En1999Diff};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeConnMaterialId, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let Some(r0) = base.connections.iter().find(|row| row.id == payload.connection_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "connection", Vec::<String>::new());
    };
    if r0.material_id == payload.new_material_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The connection already has this value.");
    }
    protocol::MutationOutcome::new(En1999Diff { connections: Some(En1999ConnectionsRows::modification(&payload.connection_id, En1999ConnectionsPatch { material_id: Some(payload.new_material_id.clone()), ..Default::default() })), ..Default::default() })
}
