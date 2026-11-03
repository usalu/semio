//! ✏️ `change-bearing-device` diff: patches `device` of the addressed bearing row.

use crate::change_bearing_device::ChangeBearingDevice;
use semio_s_artifact_norm_en1998_diff::{En1998BearingDelta, En1998BearingPatch, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeBearingDevice, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(r0) = base.bearings.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("bearing {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.device == payload.new_device {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The bearing already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { bearings: En1998BearingDelta::modification(&payload.id, En1998BearingPatch { device: Some(payload.new_device), ..Default::default() }), ..Default::default() })
}
