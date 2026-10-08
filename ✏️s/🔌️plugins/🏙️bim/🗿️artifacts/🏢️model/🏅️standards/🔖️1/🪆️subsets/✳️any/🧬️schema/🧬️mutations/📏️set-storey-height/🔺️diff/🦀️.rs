//! 🔺️ Diff constructor for `SetStoreyHeight`: a one-field storey patch. Nothing downstream is written: every elevation above and
//! every wall resolved by this storey follows by inference.

use super::SetStoreyHeight;
use crate::{Entry, ModelDiff, ModelSnapshot, StoreyPatch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetStoreyHeight, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(storey) = base.storeys.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Storey \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if !(payload.height.is_finite() && payload.height > 0.0) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A storey height must be a positive length.", ["height"]);
    }
    if storey.height == payload.height {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Storey \"{}\" already is {} m high.", payload.id, payload.height), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::storeys(payload.id.clone(), Entry::Patched(StoreyPatch { height: Some(payload.height), ..Default::default() })))
}
