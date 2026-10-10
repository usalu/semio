//! 🗂️ `setClassification`: classifies the given holders (elements or types), or the selected ones, with a code of a classification system through one `set-element-classification` mutation each, in one gesture. A holder that
//! already carries exactly this code in the system is skipped. A code outside the table of the system is allowed (the diagnostics report it); the classification browser sends this command with the code of a row.

use crate::editor::bim::commands::set_property::elements_of;
use crate::editor::bim::kit::fault;
use crate::editor::bim::BimDispatchCtx;
use crate::{ModelMutation, ModelSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use value_derive::{FromValue, ToValue};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "set-classification")]
pub struct SetClassification {
    pub ids: Vec<String>,
    pub system: String,
    pub code: String,
}

pub fn handle(payload: &SetClassification, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    let ids = elements_of(snapshot, &payload.ids, &ctx.selected);
    if ids.is_empty() {
        return Err(fault("bim.classification.target-missing", "no element to classify among the targets"));
    }
    let (system, code) = (payload.system.trim(), payload.code.trim());
    if system.is_empty() || code.is_empty() {
        return Err(fault("bim.classification.invalid", "a classification needs a system and a code"));
    }
    if !snapshot.classification_systems.contains_key(system) {
        return Err(fault("bim.classification.system-missing", format!("the classification system '{system}' does not exist")));
    }
    let mutations: Vec<ModelMutation> = ids
        .into_iter()
        .filter(|id| snapshot.classifications.get(id.as_str()).and_then(|set| set.get(system)).map(String::as_str) != Some(code))
        .map(|id| ModelMutation::SetElementClassification(crate::mutations::set_element_classification::SetElementClassification { id: id.clone(), system: system.to_string(), code: code.to_string() }))
        .collect();
    Ok(if mutations.is_empty() { Emit::default() } else { Emit::mutations(mutations) })
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
