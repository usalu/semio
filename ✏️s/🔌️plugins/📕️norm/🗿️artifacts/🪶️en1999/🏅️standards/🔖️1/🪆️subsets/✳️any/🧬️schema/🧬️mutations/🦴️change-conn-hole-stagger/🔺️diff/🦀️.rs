//! 🔺️ `change-conn-hole-stagger` diff: patches `hole_stagger` of the addressed connection row.

use crate::change_conn_hole_stagger::ChangeConnHoleStagger;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999ConnectionsPatch, En1999ConnectionsRows};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeConnHoleStagger, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_hole_stagger.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.connections.iter().find(|row| row.id == payload.aluminium_connection_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "aluminium-connection", Vec::<String>::new());
    };
    if r0.hole_stagger == payload.new_hole_stagger {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The connection already has this hole stagger.");
    }
    protocol::MutationOutcome::new(En1999Diff { connections: Some(En1999ConnectionsRows::modification(&payload.aluminium_connection_id, En1999ConnectionsPatch { hole_stagger: Some(payload.new_hole_stagger), ..Default::default() })), ..Default::default() })
}
