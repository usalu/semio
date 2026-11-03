//! 🚛 `change-special-vehicle-designation` diff: patches `designation` of the addressed special vehicle row.

use crate::change_special_vehicle_designation::ChangeSpecialVehicleDesignation;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991SpecialVehicleDelta, En1991SpecialVehiclePatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeSpecialVehicleDesignation, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(r0) = base.special_vehicles.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Special vehicle {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.designation == payload.new_designation {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The special vehicle already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { special_vehicles: En1991SpecialVehicleDelta::modification(&payload.id, En1991SpecialVehiclePatch { designation: Some(payload.new_designation.clone()), ..Default::default() }), ..Default::default() })
}
