//! 🚗 `change-accidental-vehicle-mass` diff: patches `vehicle_mass` of the impact block of the addressed accidental case.

use crate::change_accidental_vehicle_mass::ChangeAccidentalVehicleMass;
use semio_s_artifact_norm_en1991_diff::{En1991AccidentalCaseDelta, En1991AccidentalCasePatch, En1991Diff};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeAccidentalVehicleMass, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_vehicle_mass.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.accidental_cases.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Accidental case {} does not exist.", payload.id), [payload.id.clone()]);
    };
    let Some(current) = r0.impact.first() else {
        return protocol::MutationOutcome::error("mutation.target-missing", "The accidental case has no impact block.", [payload.id.clone()]);
    };
    if current.vehicle_mass == payload.new_vehicle_mass {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The accidental case already has this value.");
    }
    let mut impact = r0.impact.clone();
    impact[0].vehicle_mass = payload.new_vehicle_mass;
    protocol::MutationOutcome::new(En1991Diff { accidental_cases: En1991AccidentalCaseDelta::modification(&payload.id, En1991AccidentalCasePatch { impact: Some(impact), ..Default::default() }), ..Default::default() })
}
