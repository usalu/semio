//! ↩️ Inverse of `SetComponentOverride`: for an existing override an absolute `SetComponentOverride` restoring its base formula, for a created one the concrete `RemoveComponentOverride`; none when the
//! component is absent or nothing changes.

use super::super::remove_component_override::RemoveComponentOverride;
use super::super::family_rules;
use super::SetComponentOverride;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &SetComponentOverride, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if !base.components.contains_key(&payload.component) {
        return Vec::new();
    }
    match base.component_overrides.get(&family_rules::parameter_key(&payload.component, &payload.name)) {
        None => vec![ModelMutation::RemoveComponentOverride(RemoveComponentOverride { component: payload.component.clone(), name: payload.name.clone() })],
        Some(record) if record.value == payload.value => Vec::new(),
        Some(record) => vec![ModelMutation::SetComponentOverride(SetComponentOverride { component: payload.component.clone(), name: payload.name.clone(), value: record.value.clone() })],
    }
}
