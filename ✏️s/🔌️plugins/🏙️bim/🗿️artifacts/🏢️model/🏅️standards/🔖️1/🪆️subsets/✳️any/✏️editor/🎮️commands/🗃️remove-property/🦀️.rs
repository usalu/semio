//! 🗃️ `removeProperty`: removes one property of one property set from the given elements, or from the selected ones, through one `remove-element-property` mutation each, in one
//! gesture. An element that does not have the property is skipped; when none has it the command is refused instead of doing nothing silently.

use crate::editor::bim::commands::set_property::elements_of;
use crate::editor::bim::kit::fault;
use crate::editor::bim::BimDispatchCtx;
use crate::{ModelMutation, ModelSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use value_derive::{FromValue, ToValue};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "remove-property")]
pub struct RemoveProperty {
    pub ids: Vec<String>,
    pub pset: String,
    pub property: String,
}

pub fn handle(payload: &RemoveProperty, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    let holders: Vec<&String> = elements_of(snapshot, &payload.ids, &ctx.selected).into_iter().filter(|id| snapshot.properties.get(id.as_str()).and_then(|sets| sets.get(&payload.pset)).is_some_and(|properties| properties.contains_key(&payload.property))).collect();
    if holders.is_empty() {
        return Err(fault("bim.property.missing", format!("no target has the property '{}.{}'", payload.pset, payload.property)));
    }
    Ok(Emit::mutations(
        holders.into_iter().map(|id| ModelMutation::RemoveElementProperty(crate::mutations::remove_element_property::RemoveElementProperty { id: id.clone(), pset: payload.pset.clone(), property: payload.property.clone() })).collect(),
    ))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
