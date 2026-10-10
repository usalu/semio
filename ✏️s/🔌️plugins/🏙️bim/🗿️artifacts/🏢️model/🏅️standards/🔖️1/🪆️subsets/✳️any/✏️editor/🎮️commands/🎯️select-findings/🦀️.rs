//! 🎯️ `selectFindings`: selects the elements a finding of the diagnostics names in the framework `elements` domain, which the plan, the 3D view and the outliner share. Selection is framework state, so the command writes no
//! mutation; ids that no collection holds (a finding about a missing reference) are skipped and a finding whose elements are all gone is refused instead of silently selecting nothing.

use crate::editor::bim::entities::kind_holding;
use crate::editor::bim::interaction::BIM_ELEMENT_DOMAIN;
use crate::editor::bim::kit::{fault, select_effect};
use crate::editor::bim::BimDispatchCtx;
use crate::{ModelMutation, ModelSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use value_derive::{FromValue, ToValue};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "select-findings")]
pub struct SelectFindings {
    pub ids: Vec<String>,
}

/// 🎯️ The `(granularity, id)` targets of the placed entities among `ids`, each once, in order.
pub fn targets_of(snapshot: &ModelSnapshot, ids: &[String]) -> Vec<(String, String)> {
    let mut targets: Vec<(String, String)> = Vec::new();
    for id in ids {
        if let Some(row) = kind_holding(snapshot, id).filter(|row| !row.library) {
            if !targets.iter().any(|(_, seen)| seen == id) {
                targets.push((row.kind.to_string(), id.clone()));
            }
        }
    }
    targets
}

pub fn handle(payload: &SelectFindings, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let targets = targets_of(doc.snapshot, &payload.ids);
    if targets.is_empty() {
        return Err(fault("bim.diagnostic.target-missing", "none of the elements the finding names exists"));
    }
    let mut emit = Emit::default();
    emit.effects.push(select_effect(BIM_ELEMENT_DOMAIN, &targets, "replace"));
    Ok(emit)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
