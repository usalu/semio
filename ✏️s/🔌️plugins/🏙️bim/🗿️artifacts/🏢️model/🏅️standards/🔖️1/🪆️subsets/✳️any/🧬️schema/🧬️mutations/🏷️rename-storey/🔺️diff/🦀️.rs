//! 🔺️ Diff constructor for `RenameStorey`: a one-field storey patch; renaming to the current name is a `mutation.no-op` warning.

use super::RenameStorey;
use crate::{Entry, ModelDiff, ModelSnapshot, StoreyPatch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &RenameStorey, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(storey) = base.storeys.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Storey \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if storey.name == payload.name {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Storey \"{}\" already has the name \"{}\".", payload.id, payload.name), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::storeys(payload.id.clone(), Entry::Patched(StoreyPatch { name: Some(payload.name.clone()), ..Default::default() })))
}
