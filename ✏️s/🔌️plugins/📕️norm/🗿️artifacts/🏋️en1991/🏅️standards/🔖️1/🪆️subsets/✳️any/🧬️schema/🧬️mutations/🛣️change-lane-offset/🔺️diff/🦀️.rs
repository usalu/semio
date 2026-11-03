//! 🛣️ `change-lane-offset` diff: patches `offset` of the addressed bridge lane row.

use crate::change_lane_offset::ChangeLaneOffset;
use semio_s_artifact_norm_en1991_diff::{En1991BridgeLaneDelta, En1991BridgeLanePatch, En1991Diff};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeLaneOffset, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_offset.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.bridge_lanes.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("bridge lane {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.offset == payload.new_offset {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The bridge lane already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { bridge_lanes: En1991BridgeLaneDelta::modification(&payload.id, En1991BridgeLanePatch { offset: Some(payload.new_offset), ..Default::default() }), ..Default::default() })
}
