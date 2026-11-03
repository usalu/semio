//! 🚛 `insert-special-vehicle` diff: inserts the special vehicle row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_special_vehicle::InsertSpecialVehicle;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991SpecialVehicleDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &InsertSpecialVehicle, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.special_vehicles.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Special vehicle {key} already exists."), [key]);
    }
    if payload.index > base.special_vehicles.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.special_vehicles.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1991Diff { special_vehicles: En1991SpecialVehicleDelta::insertion(payload.index, payload.item.clone()), ..Default::default() })
}
