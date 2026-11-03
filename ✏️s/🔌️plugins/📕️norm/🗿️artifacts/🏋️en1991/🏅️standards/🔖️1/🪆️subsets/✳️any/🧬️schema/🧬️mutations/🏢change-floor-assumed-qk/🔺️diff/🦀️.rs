//! 🏢 `change-floor-assumed-qk` diff: patches `assumed_qk` of the addressed floor row.

use crate::change_floor_assumed_qk::ChangeFloorAssumedQk;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991FloorDelta, En1991FloorPatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeFloorAssumedQk, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_assumed_qk.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.floors.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("floor {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.assumed_qk == payload.new_assumed_qk {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The floor already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { floors: En1991FloorDelta::modification(&payload.id, En1991FloorPatch { assumed_qk: Some(payload.new_assumed_qk), ..Default::default() }), ..Default::default() })
}
