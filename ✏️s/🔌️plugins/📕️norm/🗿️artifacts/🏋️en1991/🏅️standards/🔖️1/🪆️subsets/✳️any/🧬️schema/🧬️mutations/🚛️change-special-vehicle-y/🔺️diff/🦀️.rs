//! 🚛 `change-special-vehicle-y` diff: patches `y` of the addressed special vehicle row.

use crate::change_special_vehicle_y::ChangeSpecialVehicleY;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991SpecialVehicleDelta, En1991SpecialVehiclePatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeSpecialVehicleY, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_y.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.special_vehicles.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Special vehicle {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.y == payload.new_y {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The special vehicle already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { special_vehicles: En1991SpecialVehicleDelta::modification(&payload.id, En1991SpecialVehiclePatch { y: Some(payload.new_y), ..Default::default() }), ..Default::default() })
}
