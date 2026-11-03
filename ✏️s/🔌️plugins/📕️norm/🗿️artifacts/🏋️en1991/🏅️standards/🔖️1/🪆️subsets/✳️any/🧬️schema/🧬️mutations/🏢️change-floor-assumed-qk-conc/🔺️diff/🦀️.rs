//! 🏢 `change-floor-assumed-qk-conc` diff: patches `assumed_qk_concentrated` of the addressed floor area row.

use crate::change_floor_assumed_qk_conc::ChangeFloorAssumedQkConc;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991FloorDelta, En1991FloorPatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeFloorAssumedQkConc, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_assumed_qk_concentrated.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.floors.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Floor area {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.assumed_qk_concentrated == payload.new_assumed_qk_concentrated {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The floor area already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { floors: En1991FloorDelta::modification(&payload.id, En1991FloorPatch { assumed_qk_concentrated: Some(payload.new_assumed_qk_concentrated), ..Default::default() }), ..Default::default() })
}
