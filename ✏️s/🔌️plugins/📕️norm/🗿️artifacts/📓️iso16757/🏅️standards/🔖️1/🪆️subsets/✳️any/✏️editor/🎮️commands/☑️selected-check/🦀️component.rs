//! ☑️ ISO 16757 play app command — point the inspection panel at a different computed check.
//!
//! 📌️ Config-only: it emits `config_mutations`, never document operations — the selected row is view
//! state, not compliance content. Declared as a `view_action`, so the registry's kind discipline
//! actively rejects it if it ever starts emitting document operations.

use crate::artifacts::iso16757::op::Iso16757Mutation;
use crate::artifacts::iso16757::Iso16757Snapshot;
use crate::config::{NormConfig, NormConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "selected-check")]
pub struct SetSelectedCheckIndex {
    /// 👁️ `None` means "the first check" — the same fallback `crate::app_surface::render_inspection` applies.
    pub index: Option<u32>,
}
//#endregion 🔖️Payload

//#region 🔖️Handler
pub fn handle(payload: &SetSelectedCheckIndex, _doc: &ArtifactView<'_, Iso16757Snapshot>, _cfg: &ConfigView<'_, NormConfig>) -> Result<Emit<Iso16757Mutation, NormConfigMutation>, Fault> {
    crate::app_surface::commit_selected_check_index::<Iso16757Mutation>(payload.index)
}
//#endregion 🔖️Handler

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::HistoryView;

    #[test]
    fn handle_emits_only_a_config_operation() {
        let projection = Iso16757Snapshot::default();
        let config = NormConfig::default();
        let emit = handle(&SetSelectedCheckIndex { index: Some(4) }, &ArtifactView::new(&projection, &HistoryView::empty()), &ConfigView { snapshot: &config }).expect("handle");
        assert!(emit.artifact_mutations.is_empty(), "a view action must never emit document operations");
        assert_eq!(emit.config_mutations, vec![NormConfigMutation::SetSelectedCheckIndex { index: Some(4) }]);
    }
}
//#endregion 🧪️Tests
