//! 🔺️ Diff constructor for `SetStoreyLevel`: a one-field storey patch; the level index stays unique within the building.

use super::SetStoreyLevel;
use crate::{Entry, ModelDiff, ModelSnapshot, StoreyPatch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetStoreyLevel, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(storey) = base.storeys.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Storey \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if storey.level == payload.level {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Storey \"{}\" already is on level {}.", payload.id, payload.level), [payload.id.clone()]);
    }
    if base.storeys.iter().any(|(other, row)| *other != payload.id && row.building == storey.building && row.level == payload.level) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, format!("Level {} is already taken in building \"{}\".", payload.level, storey.building), ["level"]);
    }
    MutationOutcome::new(ModelDiff::storeys(payload.id.clone(), Entry::Patched(StoreyPatch { level: Some(payload.level), ..Default::default() })))
}
