//! 🔺️ Sparse declarative set-support diff; no derived state is evaluated.
use super::SetSupport;
use crate::{Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};
pub fn diff(payload: &SetSupport, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {

let Some(record) = base.supports.get(&payload.id) else { return MutationOutcome::refuse(OutcomeCode::TargetMissing, "Structural record does not exist.", [payload.id.clone()]); };
let patch = payload.patch().minimal(record); if patch.is_empty() { return MutationOutcome::refuse(OutcomeCode::NoOp, "Structural parameters are unchanged.", [payload.id.clone()]); }
if let Some(problem) = crate::support_problem(base, &patch.write(record)) { return MutationOutcome::refuse(OutcomeCode::Invariant, problem, [payload.id.clone()]); }
MutationOutcome::new(ModelDiff::supports(payload.id.clone(), Entry::Patched(patch)))
}
