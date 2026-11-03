//! ♾️ `change-phi-infinity` diff: patches `phi_infinity` of the addressed wall row.

use crate::change_phi_infinity::ChangePhiInfinity;
use semio_s_artifact_norm_en1996_diff::{En1996Diff, En1996WallDelta, En1996WallPatch};
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn diff(payload: &ChangePhiInfinity, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if !payload.new_phi_infinity.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.walls.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("wall {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.phi_infinity == payload.new_phi_infinity {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The wall already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { walls: En1996WallDelta::modification(&payload.id, En1996WallPatch { phi_infinity: Some(payload.new_phi_infinity), ..Default::default() }), ..Default::default() })
}
