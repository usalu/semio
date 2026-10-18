//! 🔺️ Sparse declarative delete-support diff; no derived state is evaluated.
use crate::standards::v1::subsets::any::schema::mutations::delete_support::DeleteSupport;
use crate::standards::v1::subsets::any::schema::{diff::{Entry, ModelDiff}, snapshot::ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};
pub fn diff(payload: &DeleteSupport, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {

if !base.supports.contains_key(&payload.id) { return MutationOutcome::refuse(OutcomeCode::TargetMissing, "Structural record does not exist.", [payload.id.clone()]); }

MutationOutcome::new(ModelDiff::supports(payload.id.clone(), Entry::Deleted))
}
