//! 🗄️ `removeClassification`: removes the classification of the given holders, or of the selected ones, in one classification system through one `remove-element-classification` mutation each, in one gesture. A holder that
//! has none in that system is skipped; when none has it the command is refused instead of doing nothing silently.

use crate::editor::bim::commands::set_property::elements_of;
use crate::editor::bim::kit::fault;
use crate::editor::bim::BimDispatchCtx;
use crate::{ModelMutation, ModelSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use value_derive::{FromValue, ToValue};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "remove-classification")]
pub struct RemoveClassification {
    pub ids: Vec<String>,
    pub system: String,
}

pub fn handle(payload: &RemoveClassification, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    let system = payload.system.trim();
    let holders: Vec<&String> = elements_of(snapshot, &payload.ids, &ctx.selected).into_iter().filter(|id| snapshot.classifications.get(id.as_str()).is_some_and(|set| set.contains_key(system))).collect();
    if holders.is_empty() {
        return Err(fault("bim.classification.missing", "none of the targets is classified in that system"));
    }
    Ok(Emit::mutations(holders.into_iter().map(|id| ModelMutation::RemoveElementClassification(crate::mutations::remove_element_classification::RemoveElementClassification { id: id.clone(), system: system.to_string() })).collect()))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
