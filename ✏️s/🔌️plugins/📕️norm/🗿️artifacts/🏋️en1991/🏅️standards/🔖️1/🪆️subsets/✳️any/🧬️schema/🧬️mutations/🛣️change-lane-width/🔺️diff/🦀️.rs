//! 🛣️ `change-lane-width` diff: patches `width` of the addressed bridge lane row.

use crate::change_lane_width::ChangeLaneWidth;
use semio_s_artifact_norm_en1991_diff::{En1991BridgeLaneDelta, En1991BridgeLanePatch, En1991Diff};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeLaneWidth, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_width.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.bridge_lanes.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("bridge lane {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.width == payload.new_width {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The bridge lane already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { bridge_lanes: En1991BridgeLaneDelta::modification(&payload.id, En1991BridgeLanePatch { width: Some(payload.new_width), ..Default::default() }), ..Default::default() })
}
