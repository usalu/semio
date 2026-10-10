//! 🔍️ `searchClassification`: finds the entries of a classification system whose code or title contains a text, without regard to case, and selects them in the framework `library` domain (granularity `classification-entry`,
//! id `system:code`); the classification browser then lists the matches with their ancestors instead of the whole table. An empty text clears the search. Selection is framework state, so the command writes no mutation; a text
//! that matches nothing is refused instead of silently selecting nothing.

use crate::editor::bim::interaction::{BIM_LIBRARY_DOMAIN, CLASSIFICATION_ENTRY};
use crate::editor::bim::kit::{fault, select_effect};
use crate::editor::bim::BimDispatchCtx;
use crate::{ClassificationSystem, ModelMutation, ModelSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use value_derive::{FromValue, ToValue};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "search-classification")]
pub struct SearchClassification {
    pub system: String,
    pub query: String,
}

/// 🔍️ The codes of the entries of `system` that match `query`, in table order; every entry for an empty query.
pub fn matches(system: &ClassificationSystem, query: &str) -> Vec<String> {
    let needle = query.trim().to_lowercase();
    system.entries.iter().filter(|entry| needle.is_empty() || entry.code.to_lowercase().contains(&needle) || entry.title.to_lowercase().contains(&needle)).map(|entry| entry.code.clone()).collect()
}

/// 🔑️ The id a table entry has in the `library` domain.
pub fn entry_id(system: &str, code: &str) -> String {
    format!("{system}:{code}")
}

pub fn handle(payload: &SearchClassification, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let system = doc.snapshot.classification_systems.get(&payload.system).ok_or_else(|| fault("bim.classification.system-missing", format!("the classification system '{}' does not exist", payload.system)))?;
    let mut emit = Emit::default();
    if payload.query.trim().is_empty() {
        emit.effects.push(select_effect(BIM_LIBRARY_DOMAIN, &[], "replace"));
        return Ok(emit);
    }
    let found = matches(system, &payload.query);
    if found.is_empty() {
        return Err(fault("bim.classification.no-match", format!("no entry of '{}' matches '{}'", system.name, payload.query.trim())));
    }
    let targets: Vec<(String, String)> = found.iter().map(|code| (CLASSIFICATION_ENTRY.to_string(), entry_id(&payload.system, code))).collect();
    emit.effects.push(select_effect(BIM_LIBRARY_DOMAIN, &targets, "replace"));
    Ok(emit)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
