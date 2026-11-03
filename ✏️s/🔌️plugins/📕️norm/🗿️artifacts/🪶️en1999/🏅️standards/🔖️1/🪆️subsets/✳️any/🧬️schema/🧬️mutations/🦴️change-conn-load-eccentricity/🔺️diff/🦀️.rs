//! 🔺️ `change-conn-load-eccentricity` diff: patches `load_eccentricity` of the addressed connection row.

use crate::change_conn_load_eccentricity::ChangeConnLoadEccentricity;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999ConnectionsPatch, En1999ConnectionsRows};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeConnLoadEccentricity, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_load_eccentricity.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.connections.iter().find(|row| row.id == payload.aluminium_connection_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "aluminium-connection", Vec::<String>::new());
    };
    if r0.load_eccentricity == payload.new_load_eccentricity {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The connection already has this load eccentricity.");
    }
    protocol::MutationOutcome::new(En1999Diff { connections: Some(En1999ConnectionsRows::modification(&payload.aluminium_connection_id, En1999ConnectionsPatch { load_eccentricity: Some(payload.new_load_eccentricity), ..Default::default() })), ..Default::default() })
}
