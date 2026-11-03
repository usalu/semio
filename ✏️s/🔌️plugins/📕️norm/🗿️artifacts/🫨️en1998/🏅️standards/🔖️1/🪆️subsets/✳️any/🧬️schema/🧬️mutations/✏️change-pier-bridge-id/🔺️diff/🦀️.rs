//! ✏️ `change-pier-bridge-id` diff: patches `bridge_id` of the addressed pier row.

use crate::change_pier_bridge_id::ChangePierBridgeId;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998PierDelta, En1998PierPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangePierBridgeId, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(r0) = base.piers.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("pier {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.bridge_id == payload.new_bridge_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The pier already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { piers: En1998PierDelta::modification(&payload.id, En1998PierPatch { bridge_id: Some(payload.new_bridge_id.clone()), ..Default::default() }), ..Default::default() })
}
