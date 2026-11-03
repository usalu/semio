//! ✏️ `change-bridge-importance-class` diff: patches `importance_class` of the addressed bridge row.

use crate::change_bridge_importance_class::ChangeBridgeImportanceClass;
use semio_s_artifact_norm_en1998_diff::{En1998BridgeDelta, En1998BridgePatch, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeBridgeImportanceClass, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_importance_class.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.bridges.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("bridge {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.importance_class == payload.new_importance_class {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The bridge already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { bridges: En1998BridgeDelta::modification(&payload.id, En1998BridgePatch { importance_class: Some(payload.new_importance_class), ..Default::default() }), ..Default::default() })
}
