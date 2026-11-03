//! 🔺️ `change-conn-bolts-pitch` diff: patches `bolts.pitch` of the addressed connection row.

use crate::change_conn_bolts_pitch::ChangeConnBoltsPitch;
use semio_s_artifact_norm_en1999_diff::{En1999ConnectionsPatch, En1999ConnectionsRows, En1999Diff};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeConnBoltsPitch, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_bolts_pitch.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.connections.iter().find(|row| row.id == payload.connection_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "connection", Vec::<String>::new());
    };
    if r0.bolts.pitch == payload.new_bolts_pitch {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The connection already has this value.");
    }
    protocol::MutationOutcome::new(En1999Diff { connections: Some(En1999ConnectionsRows::modification(&payload.connection_id, En1999ConnectionsPatch { bolts_pitch: Some(payload.new_bolts_pitch), ..Default::default() })), ..Default::default() })
}
