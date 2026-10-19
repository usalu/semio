//! 🔺️ Diff constructor for `SetComponentOverride`: the override of parameter `name` of a component is one record keyed `component.name`. An existing override gets a one-field patch with the new formula
//! (the same formula is a no-op), an absent one is created. The component and the parameter of its family exist, the formula parses, uses only parameters of the family and closes no circle under the
//! overrides of the component; whether it also computes the kind of the parameter is the business of the inference (a diagnostic), not of the diff. The key is free in every other collection.

use crate::standards::v1::subsets::any::schema::mutations::set_component_override::SetComponentOverride;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{ComponentOverride, ModelSnapshot};
use semio_s_artifact_bim_model_tests_2::standards::v1::subsets::any::schema::diff::patches::library::ComponentOverridePatch;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::{Entry, Patch};
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetComponentOverride, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(fault) = semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::component_rules::override_fault(base, &payload.component, &payload.name, &payload.value) {
        return MutationOutcome::refuse(fault.code, fault.message.clone(), fault.path(None));
    }
    let key = semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::mutations::family_rules::parameter_key(&payload.component, &payload.name);
    if let Some(record) = base.component_overrides.get(&key) {
        let change = ComponentOverridePatch { value: Some(payload.value.clone()), ..Default::default() }.minimal(record);
        if change.is_empty() {
            return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Parameter \"{}\" of component \"{}\" already has this override.", payload.name, payload.component), [key]);
        }
        return MutationOutcome::new(ModelDiff::component_overrides(key, Entry::Patched(change)));
    }
    if let Some(noun) = semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::taken(base, &key) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{key}\" already exists."), [key]);
    }
    MutationOutcome::new(ModelDiff::component_overrides(key, Entry::Created(ComponentOverride { component: payload.component.clone(), name: payload.name.clone(), value: payload.value.clone() })))
}
