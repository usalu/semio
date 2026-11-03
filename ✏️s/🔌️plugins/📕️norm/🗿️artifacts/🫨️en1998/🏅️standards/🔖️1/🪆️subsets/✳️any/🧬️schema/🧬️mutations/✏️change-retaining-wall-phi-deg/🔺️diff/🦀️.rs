//! ✏️ `change-retaining-wall-phi-deg` diff: patches `phi_deg` of the addressed retaining wall row.

use crate::change_retaining_wall_phi_deg::ChangeRetainingWallPhiDeg;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998RetainingWallDelta, En1998RetainingWallPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeRetainingWallPhiDeg, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_phi_deg.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.retaining_walls.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("retaining wall {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.phi_deg == payload.new_phi_deg {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The retaining wall already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { retaining_walls: En1998RetainingWallDelta::modification(&payload.id, En1998RetainingWallPatch { phi_deg: Some(payload.new_phi_deg), ..Default::default() }), ..Default::default() })
}
