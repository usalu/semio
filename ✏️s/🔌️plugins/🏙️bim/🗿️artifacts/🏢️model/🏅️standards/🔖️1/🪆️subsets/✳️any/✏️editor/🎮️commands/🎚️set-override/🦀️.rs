//! 🎚️ `setOverride`: gives one parameter of the family of a component a formula of its own, or takes it away again. The formula is canonicalised here, so text that does not parse never reaches a mutation; an empty
//! formula, or the formula the family itself gives the parameter, is no override and removes the one the component has. Whether the formula names only parameters of the family and closes no circle is decided by the
//! `set-component-override` mutation, not here. One call is one undoable step.

use crate::editor::bim::kit::fault;
use crate::editor::bim::BimDispatchCtx;
use crate::mutations::remove_component_override::RemoveComponentOverride;
use crate::mutations::set_component_override::SetComponentOverride;
use crate::standards::v1::subsets::any::schema::authored::formula;
use crate::{ModelMutation, ModelSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use value_derive::{FromValue, ToValue};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "set-override")]
pub struct SetOverride {
    pub component: String,
    pub name: String,
    pub value: String,
}

/// 🎚️ The mutation that makes parameter `name` of `component` evaluate `value`, none when nothing changes: the override a component has is removed by an empty formula or the family's own, and an override that
/// already holds the formula is no change.
pub fn plan(snapshot: &ModelSnapshot, payload: &SetOverride) -> Result<Option<ModelMutation>, Fault> {
    let component = snapshot.components.get(&payload.component).ok_or_else(|| fault("bim.override.component-missing", format!("the component '{}' does not exist", payload.component)))?;
    let parameter = snapshot.family_parameters.get(&formula::parameter_id(&component.family, &payload.name)).ok_or_else(|| fault("bim.override.parameter-missing", format!("the family '{}' has no parameter '{}'", component.family, payload.name)))?;
    let current = snapshot.component_overrides.get(&formula::parameter_id(&payload.component, &payload.name));
    let remove = || current.map(|_| ModelMutation::RemoveComponentOverride(RemoveComponentOverride { component: payload.component.clone(), name: payload.name.clone() }));
    let text = payload.value.trim();
    if text.is_empty() {
        return Ok(remove());
    }
    let value = formula::canonical(text).map_err(|error| fault("bim.family.formula-invalid", format!("'{text}' is no formula: {error:?}")))?;
    if value == parameter.value {
        return Ok(remove());
    }
    if current.is_some_and(|row| row.value == value) {
        return Ok(None);
    }
    Ok(Some(ModelMutation::SetComponentOverride(SetComponentOverride { component: payload.component.clone(), name: payload.name.clone(), value })))
}

pub fn handle(payload: &SetOverride, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    Ok(plan(doc.snapshot, payload)?.map_or_else(Emit::default, |mutation| Emit::mutations(vec![mutation])))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
