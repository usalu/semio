//! 🔺️ `change-conn-bolts-material` diff: patches `bolts.material` of the addressed connection row.

use crate::change_conn_bolts_material::ChangeConnBoltsMaterial;
use semio_s_artifact_norm_en1999_diff::{En1999ConnectionsPatch, En1999ConnectionsRows, En1999Diff};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeConnBoltsMaterial, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let Some(r0) = base.connections.iter().find(|row| row.id == payload.connection_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "connection", Vec::<String>::new());
    };
    if r0.bolts.material == payload.new_bolts_material {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The connection already has this value.");
    }
    protocol::MutationOutcome::new(En1999Diff { connections: Some(En1999ConnectionsRows::modification(&payload.connection_id, En1999ConnectionsPatch { bolts_material: Some(payload.new_bolts_material.clone()), ..Default::default() })), ..Default::default() })
}
