//! ✏️ `change-bearing-bridge-id` diff: patches `bridge_id` of the addressed bearing row.

use crate::change_bearing_bridge_id::ChangeBearingBridgeId;
use semio_s_artifact_norm_en1998_diff::{En1998BearingDelta, En1998BearingPatch, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeBearingBridgeId, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(r0) = base.bearings.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("bearing {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.bridge_id == payload.new_bridge_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The bearing already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { bearings: En1998BearingDelta::modification(&payload.id, En1998BearingPatch { bridge_id: Some(payload.new_bridge_id.clone()), ..Default::default() }), ..Default::default() })
}
