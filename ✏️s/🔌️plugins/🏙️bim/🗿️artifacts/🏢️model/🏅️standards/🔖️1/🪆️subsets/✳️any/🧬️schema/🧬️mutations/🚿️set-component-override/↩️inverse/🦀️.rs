//! ↩️ Inverse of `SetComponentOverride`: for an existing override an absolute `SetComponentOverride` restoring its base formula, for a created one the concrete `RemoveComponentOverride`; none when the
//! component is absent or nothing changes.

use crate::standards::v1::subsets::any::schema::mutations::remove_component_override::RemoveComponentOverride;
use crate::standards::v1::subsets::any::schema::mutations::family_rules;
use crate::standards::v1::subsets::any::schema::mutations::set_component_override::SetComponentOverride;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &SetComponentOverride, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::remove_component_override::RemoveComponentOverride> + From<crate::standards::v1::subsets::any::schema::mutations::set_component_override::SetComponentOverride>,
{
    if !base.components.contains_key(&payload.component) {
        return Vec::new();
    }
    match base.component_overrides.get(&family_rules::parameter_key(&payload.component, &payload.name)) {
        None => vec![Op::from(RemoveComponentOverride { component: payload.component.clone(), name: payload.name.clone() })],
        Some(record) if record.value == payload.value => Vec::new(),
        Some(record) => vec![Op::from(SetComponentOverride { component: payload.component.clone(), name: payload.name.clone(), value: record.value.clone() })],
    }
}
