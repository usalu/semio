//! 🔺️ Diff constructor for `RemoveSpaceConditions`: the deletion of the conditions record of a space. The space must exist; a space without conditions is a no-op.

use crate::standards::v1::subsets::any::schema::mutations::remove_space_conditions::RemoveSpaceConditions;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Entry;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &RemoveSpaceConditions, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.spaces.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Space \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if !base.space_conditions.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Space \"{}\" has no conditions.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::space_conditions(payload.id.clone(), Entry::Deleted))
}
