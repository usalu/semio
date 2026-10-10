//! 🔺️ Diff constructor for `RemoveComponentOverride`: the override record leaves and the component evaluates the formula of its family again. Refused when the component or the override is absent.

use super::super::family_rules;
use super::RemoveComponentOverride;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &RemoveComponentOverride, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.components.contains_key(&payload.component) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Component \"{}\" does not exist.", payload.component), ["component"]);
    }
    let key = family_rules::parameter_key(&payload.component, &payload.name);
    if !base.component_overrides.contains_key(&key) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Component \"{}\" does not override parameter \"{}\".", payload.component, payload.name), [key]);
    }
    MutationOutcome::new(ModelDiff::component_overrides(key, Entry::Deleted))
}
