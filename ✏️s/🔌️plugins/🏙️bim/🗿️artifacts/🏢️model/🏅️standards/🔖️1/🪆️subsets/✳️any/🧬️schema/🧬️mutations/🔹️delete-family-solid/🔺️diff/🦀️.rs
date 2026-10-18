//! 🔺️ Diff constructor for `DeleteFamilySolid`: the solid leaves; the parameters of its family stay untouched.

use crate::standards::v1::subsets::any::schema::mutations::delete_family_solid::DeleteFamilySolid;
use crate::standards::v1::subsets::any::schema::{diff::{Entry, ModelDiff}, snapshot::ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteFamilySolid, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.family_solids.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Family solid \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::family_solids(payload.id.clone(), Entry::Deleted))
}
