//! 🔺️ Sparse declarative delete-support diff; no derived state is evaluated.
use super::DeleteSupport;
use crate::{Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};
pub fn diff(payload: &DeleteSupport, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {

if !base.supports.contains_key(&payload.id) { return MutationOutcome::refuse(OutcomeCode::TargetMissing, "Structural record does not exist.", [payload.id.clone()]); }

MutationOutcome::new(ModelDiff::supports(payload.id.clone(), Entry::Deleted))
}
