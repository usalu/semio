//! 🔺️ `change-emb-wall-delta-active-deg` diff: patches `delta_active_deg` of the addressed embedded wall row.

use crate::change_emb_wall_delta_active_deg::ChangeEmbWallDeltaActiveDeg;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997EmbeddedWallsPatch, En1997EmbeddedWallsRows};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &ChangeEmbWallDeltaActiveDeg, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_delta_active_deg.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.embedded_walls.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("emb-wall {} missing", payload.id), vec![payload.id.clone()]);
    };
    if r0.delta_active_deg == payload.new_delta_active_deg {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The embedded wall already has this value.");
    }
    protocol::MutationOutcome::new(En1997Diff { embedded_walls: Some(En1997EmbeddedWallsRows::modification(&payload.id, En1997EmbeddedWallsPatch { delta_active_deg: Some(payload.new_delta_active_deg), ..Default::default() })), ..Default::default() })
}
