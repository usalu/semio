//! 🔺️ Diff constructor for `DeleteBeamType`: one deleted beam type entry; refused while beams still use it.

use super::DeleteBeamType;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteBeamType, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.beam_types.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Beam type \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if base.beams.values().any(|row| row.beam_type == payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetReferenced, format!("Beam type \"{}\" is still used by beams.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::beam_types(payload.id.clone(), Entry::Deleted))
}
