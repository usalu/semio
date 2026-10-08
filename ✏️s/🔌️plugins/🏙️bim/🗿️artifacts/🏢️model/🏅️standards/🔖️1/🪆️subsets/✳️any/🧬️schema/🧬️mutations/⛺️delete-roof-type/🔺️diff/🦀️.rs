//! 🔺️ Diff constructor for `DeleteRoofType`: one deleted roof type entry; refused while a roof still uses the type.

use super::DeleteRoofType;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteRoofType, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.roof_types.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Roof type \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if base.roofs.values().any(|row| row.roof_type == payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetReferenced, format!("Roof type \"{}\" is still used by roofs.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::roof_types(payload.id.clone(), Entry::Deleted))
}
