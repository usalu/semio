//! 🔺️ Sparse declarative set-load diff; no derived state is evaluated.
use super::SetLoad;
use crate::{Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};
pub fn diff(payload: &SetLoad, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {

let Some(record) = base.loads.get(&payload.id) else { return MutationOutcome::refuse(OutcomeCode::TargetMissing, "Structural record does not exist.", [payload.id.clone()]); };
let patch = payload.patch().minimal(record); if patch.is_empty() { return MutationOutcome::refuse(OutcomeCode::NoOp, "Structural parameters are unchanged.", [payload.id.clone()]); }
if let Some(problem) = crate::load_problem(base, &patch.write(record)) { return MutationOutcome::refuse(OutcomeCode::Invariant, problem, [payload.id.clone()]); }
MutationOutcome::new(ModelDiff::loads(payload.id.clone(), Entry::Patched(patch)))
}
