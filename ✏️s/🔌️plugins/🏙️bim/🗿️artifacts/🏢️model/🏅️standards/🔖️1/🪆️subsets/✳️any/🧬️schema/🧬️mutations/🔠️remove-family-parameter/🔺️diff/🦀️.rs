//! 🔺️ Diff constructor for `RemoveFamilyParameter`: the parameter record leaves. Refused while the formula of another parameter or a solid of the family refers to it (computed from the authored
//! text of the formulas only).

use crate::standards::v1::subsets::any::schema::mutations::remove_family_parameter::RemoveFamilyParameter;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Entry;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &RemoveFamilyParameter, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.families.contains_key(&payload.family) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Family \"{}\" does not exist.", payload.family), ["family"]);
    }
    let key = semio_s_artifact_bim_model_formula::standards::v1::subsets::any::schema::authored::formula::parameter_id(&payload.family, &payload.name);
    if !base.family_parameters.contains_key(&key) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Parameter \"{}\" of family \"{}\" does not exist.", payload.name, payload.family), [key]);
    }
    if let Some(user) = semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::mutations::family_rules::parameter_user(base, &payload.family, &payload.name) {
        return MutationOutcome::refuse(OutcomeCode::TargetReferenced, format!("Parameter \"{}\" is still used by {user}.", payload.name), [key]);
    }
    MutationOutcome::new(ModelDiff::family_parameters(key, Entry::Deleted))
}
