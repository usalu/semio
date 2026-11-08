//! ✏️ `change-isolation-system-bridge-id` diff: patches `bridge_id` of the addressed isolation_system row.

use crate::change_isolation_system_bridge_id::ChangeIsolationSystemBridgeId;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998IsolationSystemDelta, En1998IsolationSystemPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeIsolationSystemBridgeId, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(r0) = base.isolation_systems.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("isolation_system {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.bridge_id == payload.new_bridge_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The isolation_system already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { isolation_systems: En1998IsolationSystemDelta::modification(&payload.id, En1998IsolationSystemPatch { bridge_id: Some(payload.new_bridge_id.clone()), ..Default::default() }), ..Default::default() })
}
