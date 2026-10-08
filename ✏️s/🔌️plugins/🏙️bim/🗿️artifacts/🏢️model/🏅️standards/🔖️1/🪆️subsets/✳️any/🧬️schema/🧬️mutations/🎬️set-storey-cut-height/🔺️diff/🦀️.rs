//! 🔺️ Diff constructor for `SetStoreyCutHeight`: a one-field storey patch assigning (or clearing) the plan cut height. The height must be
//! a positive length; restating the current value is a no-op. Nothing else is written: only the plan view reads it.

use super::SetStoreyCutHeight;
use crate::{cut_height_problem, Assigned, Entry, ModelDiff, ModelSnapshot, StoreyPatch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetStoreyCutHeight, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(storey) = base.storeys.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Storey \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(message) = payload.cut_height.value.and_then(cut_height_problem) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, message, ["cut_height"]);
    }
    if storey.cut_height == payload.cut_height.value {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Storey \"{}\" already has this plan cut height.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::storeys(payload.id.clone(), Entry::Patched(StoreyPatch { cut_height: Some(Assigned::new(payload.cut_height.value)), ..Default::default() })))
}
