//! 🔺️ Diff for `change-floor-assumed-qk`.
use crate::change_floor_assumed_qk::ChangeFloorAssumedQk;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
use semio_s_artifact_norm_en1991_diff::{En1991FloorDelta, En1991FloorPatch};
pub fn diff(payload: &ChangeFloorAssumedQk, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if payload.index >= base.floors.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "Index out of range.", [payload.index.to_string()]);
    }
    if base.floors[payload.index].assumed_qk == payload.new_assumed_qk {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    let floor = &base.floors[payload.index];
    protocol::MutationOutcome::new(En1991Diff { floors: En1991FloorDelta::modification(&floor.id, En1991FloorPatch { assumed_qk: Some(payload.new_assumed_qk), ..Default::default() }), ..Default::default() })
}
