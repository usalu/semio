//! 🔺️ Diff constructor for `SetTypeThermalData`: a sparse patch of the thermal fields of a window type (U-value, g-value, frame fraction) or of a door type (U-value) that differ from the base. Ids are unique across the
//! model, so the id names at most one of them; a door type has no g-value and no frame fraction. The written values must stay in range (see `window_thermal_problem`); providing only equal values is a no-op.

use super::SetTypeThermalData;
use crate::{curtain_thermal_problem, door_thermal_problem, window_thermal_problem, Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetTypeThermalData, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(record) = base.window_types.get(&payload.id) {
        let change = payload.window_patch().minimal(record);
        if change.is_empty() {
            return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Window type \"{}\" already has these thermal values.", payload.id), [payload.id.clone()]);
        }
        if let Some((field, message)) = window_thermal_problem(&change.write(record)) {
            return MutationOutcome::refuse(OutcomeCode::Invariant, message, [field]);
        }
        return MutationOutcome::new(ModelDiff::window_types(payload.id.clone(), Entry::Patched(change)));
    }
    if let Some(record) = base.curtain_wall_types.get(&payload.id) {
        let change = payload.curtain_patch().minimal(record);
        if change.is_empty() {
            return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Curtain wall type \"{}\" already has these thermal values.", payload.id), [payload.id.clone()]);
        }
        if let Some((field, message)) = curtain_thermal_problem(&change.write(record)) {
            return MutationOutcome::refuse(OutcomeCode::Invariant, message, [field]);
        }
        return MutationOutcome::new(ModelDiff::curtain_wall_types(payload.id.clone(), Entry::Patched(change)));
    }
    let Some(record) = base.door_types.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Window, door or curtain wall type \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if payload.g_value.is_some() {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A door type has no g-value.", ["g_value"]);
    }
    if payload.frame_fraction.is_some() {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A door type has no frame fraction.", ["frame_fraction"]);
    }
    let change = payload.door_patch().minimal(record);
    if change.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Door type \"{}\" already has this thermal value.", payload.id), [payload.id.clone()]);
    }
    if let Some((field, message)) = door_thermal_problem(&change.write(record)) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, message, [field]);
    }
    MutationOutcome::new(ModelDiff::door_types(payload.id.clone(), Entry::Patched(change)))
}
