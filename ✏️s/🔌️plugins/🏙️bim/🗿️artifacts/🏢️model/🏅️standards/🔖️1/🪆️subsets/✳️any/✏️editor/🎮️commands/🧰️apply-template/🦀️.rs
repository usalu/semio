//! 🧰️ `applyTemplate`: applies a property set template to the given holders (elements or types), or to the selected ones, by giving each the properties of the template that have a default and that it does not state itself,
//! through one `set-element-property` mutation per property, in one gesture. Applying it to a type gives every instance of the type the values by inheritance. A holder whose kind the template does not apply to refuses the whole
//! command; a template without defaults, or holders that already state every default, change nothing.

use crate::editor::bim::commands::set_property::elements_of;
use crate::editor::bim::kit::fault;
use crate::editor::bim::BimDispatchCtx;
use crate::standards::v1::subsets::any::schema::inferences::effective_properties::target_of;
use crate::{ModelMutation, ModelSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use value_derive::{FromValue, ToValue};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "apply-template")]
pub struct ApplyTemplate {
    pub ids: Vec<String>,
    pub template: String,
}

pub fn handle(payload: &ApplyTemplate, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    let template = snapshot.property_templates.get(payload.template.trim()).ok_or_else(|| fault("bim.template.target-missing", format!("the property template '{}' does not exist", payload.template)))?;
    let ids = elements_of(snapshot, &payload.ids, &ctx.selected);
    if ids.is_empty() {
        return Err(fault("bim.property.target-missing", "no element to describe among the targets"));
    }
    if let Some(id) = ids.iter().find(|id| !target_of(snapshot, id).is_some_and(|target| template.applies_to.contains(&target))) {
        return Err(fault("bim.template.not-applicable", format!("the template '{}' does not apply to '{id}'", template.name)));
    }
    let mut mutations = Vec::new();
    for id in ids {
        for definition in &template.properties {
            let stated = snapshot.properties.get(id).and_then(|sets| sets.get(&template.name)).is_some_and(|properties| properties.contains_key(&definition.name));
            if let (false, Some(value)) = (stated, &definition.default_value) {
                mutations.push(ModelMutation::SetElementProperty(crate::mutations::set_element_property::SetElementProperty { id: id.clone(), pset: template.name.clone(), property: definition.name.clone(), value: value.clone() }));
            }
        }
    }
    Ok(if mutations.is_empty() { Emit::default() } else { Emit::mutations(mutations) })
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
