//! 🔺️ Sparse declarative delete-option-group diff.
use crate::standards::v1::subsets::any::schema::mutations::delete_option_group::DeleteOptionGroup;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Entry;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};
pub fn diff(payload: &DeleteOptionGroup, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.option_groups.contains_key(&payload.id) { return MutationOutcome::refuse(OutcomeCode::TargetMissing, "Record is missing.", [payload.id.clone()]); }
    if base.design_options.values().any(|option| option.group == payload.id) { return MutationOutcome::refuse(OutcomeCode::TargetReferenced, "Option group is in use.", [payload.id.clone()]); }
    MutationOutcome::new(ModelDiff::option_groups(payload.id.clone(), Entry::Deleted))
}
