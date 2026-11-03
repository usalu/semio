//! 🏗 `change-crane-hoist-release-device` diff: patches `release_device` of the addressed crane hoist row.

use crate::change_crane_hoist_release_device::ChangeCraneHoistReleaseDevice;
use semio_s_artifact_norm_en1991_diff::{En1991CraneHoistDelta, En1991CraneHoistPatch, En1991Diff};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeCraneHoistReleaseDevice, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(r0) = base.crane_hoists.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Crane hoist {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.release_device == payload.new_release_device {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The crane hoist already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { crane_hoists: En1991CraneHoistDelta::modification(&payload.id, En1991CraneHoistPatch { release_device: Some(payload.new_release_device.clone()), ..Default::default() }), ..Default::default() })
}
