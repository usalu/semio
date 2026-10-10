//! 🔺️ Sparse declarative delete-load-case diff; no derived state is evaluated.
use super::DeleteLoadCase;
use crate::{Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};
pub fn diff(payload: &DeleteLoadCase, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {

if !base.load_cases.contains_key(&payload.id) { return MutationOutcome::refuse(OutcomeCode::TargetMissing, "Structural record does not exist.", [payload.id.clone()]); }
if base.loads.values().any(|row| row.load_case == payload.id) { return MutationOutcome::refuse(OutcomeCode::InUse, "Load case is referenced by loads.", [payload.id.clone()]); }
MutationOutcome::new(ModelDiff::load_cases(payload.id.clone(), Entry::Deleted))
}
