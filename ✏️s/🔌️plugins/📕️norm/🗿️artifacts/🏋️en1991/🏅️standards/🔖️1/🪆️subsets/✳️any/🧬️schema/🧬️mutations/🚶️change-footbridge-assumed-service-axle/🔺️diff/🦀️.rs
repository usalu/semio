//! 🚶 `change-footbridge-assumed-service-axle` diff: patches `assumed_service_axle` of the addressed footbridge row.

use crate::change_footbridge_assumed_service_axle::ChangeFootbridgeAssumedServiceAxle;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991FootbridgeDelta, En1991FootbridgePatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeFootbridgeAssumedServiceAxle, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_assumed_service_axle.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.footbridges.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Footbridge {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.assumed_service_axle == payload.new_assumed_service_axle {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The footbridge already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { footbridges: En1991FootbridgeDelta::modification(&payload.id, En1991FootbridgePatch { assumed_service_axle: Some(payload.new_assumed_service_axle), ..Default::default() }), ..Default::default() })
}
