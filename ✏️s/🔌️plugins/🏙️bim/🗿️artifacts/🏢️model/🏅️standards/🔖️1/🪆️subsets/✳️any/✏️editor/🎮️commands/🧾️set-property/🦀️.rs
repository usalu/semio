//! 🧾️ `setProperty`: sets one typed property of one property set on the given elements, or on the selected ones, through one `set-element-property` mutation each, in one gesture. An
//! existing property keeps its kind (a length stays a length); a new one takes the kind named after a colon (`Pset.Name:length = 3.5`) or the kind its value shows. Without a set and a
//! name the value is the whole entry `Pset.Name = value`, which is what the properties panel's add row sends.

use crate::editor::bim::entities::{property_entry, property_type, property_value};
use crate::editor::bim::kit::fault;
use crate::editor::bim::BimDispatchCtx;
use crate::{ModelMutation, ModelSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "set-property")]
pub struct SetProperty {
    pub ids: Vec<String>,
    pub pset: String,
    pub property: String,
    pub value: String,
}

/// 🎯️ The holders of properties and classifications among the explicit ids, else among the selection: the placed elements and the type records of the library (instances inherit the properties of their type); the other library entries carry none.
pub fn elements_of<'a>(snapshot: &ModelSnapshot, ids: &'a [String], selected: &'a [String]) -> Vec<&'a String> {
    let wanted = if ids.is_empty() { selected } else { ids };
    wanted.iter().filter(|id| crate::mutations::elements::holds_data(snapshot, id)).collect()
}

pub fn handle(payload: &SetProperty, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    let ids = elements_of(snapshot, &payload.ids, &ctx.selected);
    if ids.is_empty() {
        return Err(fault("bim.property.target-missing", "no element to describe among the targets"));
    }
    let (pset, property, kind, text) = if payload.pset.is_empty() || payload.property.is_empty() {
        let entry = property_entry(&payload.value).ok_or_else(|| fault("bim.property.name-invalid", format!("'{}' is not 'Set.Property = value'", payload.value)))?;
        (entry.pset, entry.property, entry.kind, entry.value)
    } else {
        (payload.pset.clone(), payload.property.clone(), None, payload.value.clone())
    };
    let mut mutations = Vec::new();
    for id in ids {
        let existing = snapshot.properties.get(id).and_then(|sets| sets.get(&pset)).and_then(|properties| properties.get(&property));
        let wanted = kind.as_deref().or_else(|| existing.map(property_type));
        let value = property_value(&text, wanted).ok_or_else(|| fault("bim.property.value-invalid", format!("'{text}' does not fit the property '{pset}.{property}'")))?;
        if existing != Some(&value) {
            mutations.push(ModelMutation::SetElementProperty(crate::mutations::set_element_property::SetElementProperty { id: id.clone(), pset: pset.clone(), property: property.clone(), value }));
        }
    }
    Ok(if mutations.is_empty() { Emit::default() } else { Emit::mutations(mutations) })
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
