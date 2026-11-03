//! 🚛 `remove-special-vehicle` diff: removes the addressed special vehicle row.

use crate::remove_special_vehicle::RemoveSpecialVehicle;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991SpecialVehicleDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &RemoveSpecialVehicle, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(index) = base.special_vehicles.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Special vehicle {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1991Diff { special_vehicles: En1991SpecialVehicleDelta::removal(&base.special_vehicles, index), ..Default::default() })
}
