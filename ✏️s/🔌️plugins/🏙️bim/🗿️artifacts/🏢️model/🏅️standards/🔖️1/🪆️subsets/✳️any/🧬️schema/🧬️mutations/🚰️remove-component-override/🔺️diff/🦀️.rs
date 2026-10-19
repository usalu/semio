//! 🔺️ Diff constructor for `RemoveComponentOverride`: the override record leaves and the component evaluates the formula of its family again. Refused when the component or the override is absent.

use crate::standards::v1::subsets::any::schema::mutations::remove_component_override::RemoveComponentOverride;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Entry;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &RemoveComponentOverride, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.components.contains_key(&payload.component) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Component \"{}\" does not exist.", payload.component), ["component"]);
    }
    let key = semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::mutations::family_rules::parameter_key(&payload.component, &payload.name);
    if !base.component_overrides.contains_key(&key) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Component \"{}\" does not override parameter \"{}\".", payload.component, payload.name), [key]);
    }
    MutationOutcome::new(ModelDiff::component_overrides(key, Entry::Deleted))
}
