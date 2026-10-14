//! 🔺️ Sparse declarative delete-load diff; no derived state is evaluated.
use super::DeleteLoad;
use crate::standards::v1::subsets::any::schema::{diff::{Entry, ModelDiff}, snapshot::ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};
pub fn diff(payload: &DeleteLoad, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {

if !base.loads.contains_key(&payload.id) { return MutationOutcome::refuse(OutcomeCode::TargetMissing, "Structural record does not exist.", [payload.id.clone()]); }

MutationOutcome::new(ModelDiff::loads(payload.id.clone(), Entry::Deleted))
}
