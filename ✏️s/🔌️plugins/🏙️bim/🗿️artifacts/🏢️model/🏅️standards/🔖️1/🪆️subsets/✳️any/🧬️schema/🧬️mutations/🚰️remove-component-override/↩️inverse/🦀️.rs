//! ↩️ Inverse of `RemoveComponentOverride`: the concrete `SetComponentOverride` carrying the formula of the removed override, none when it was absent.

use crate::standards::v1::subsets::any::schema::mutations::family_rules;
use crate::standards::v1::subsets::any::schema::mutations::set_component_override::SetComponentOverride;
use crate::standards::v1::subsets::any::schema::mutations::remove_component_override::RemoveComponentOverride;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &RemoveComponentOverride, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_component_override::SetComponentOverride>,
{
    match base.component_overrides.get(&family_rules::parameter_key(&payload.component, &payload.name)) {
        Some(record) => vec![Op::from(SetComponentOverride { component: payload.component.clone(), name: payload.name.clone(), value: record.value.clone() })],
        None => Vec::new(),
    }
}
