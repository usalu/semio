//! ↩️ Inverse of `RemoveComponentOverride`: the concrete `SetComponentOverride` carrying the formula of the removed override, none when it was absent.

use super::super::family_rules;
use super::super::set_component_override::SetComponentOverride;
use super::RemoveComponentOverride;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &RemoveComponentOverride, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.component_overrides.get(&family_rules::parameter_key(&payload.component, &payload.name)) {
        Some(record) => vec![ModelMutation::SetComponentOverride(SetComponentOverride { component: payload.component.clone(), name: payload.name.clone(), value: record.value.clone() })],
        None => Vec::new(),
    }
}
