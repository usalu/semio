//! ☑️ EN 1993 play app command — point the inspection panel at a different computed check.
//!
//! 📌️ Config-only: it emits `config_mutations`, never document operations — the selected row is view
//! state, not compliance content. Declared as a `view_action`, so the registry's kind discipline
//! actively rejects it if it ever starts emitting document operations.

use crate::artifacts::en1993::op::En1993Mutation;
use crate::artifacts::en1993::En1993Snapshot;
use crate::config::{NormConfig, NormConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "selected-check")]
pub struct SetSelectedCheckIndex {
    /// 👁️ `None` means "the first check" — the same fallback `crate::app_surface::render_inspection` applies.
    pub index: Option<u32>,
}
//#endregion 🔖️Payload

//#region 🔖️Handler
pub fn handle(payload: &SetSelectedCheckIndex, _doc: &ArtifactView<'_, En1993Snapshot>, _cfg: &ConfigView<'_, NormConfig>) -> Result<Emit<En1993Mutation, NormConfigMutation>, Fault> {
    crate::app_surface::commit_selected_check_index::<En1993Mutation>(payload.index)
}
//#endregion 🔖️Handler

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::HistoryView;

    #[semio_framework_async_macros::async_test]
    fn handle_emits_only_a_config_operation() {
        let projection = En1993Snapshot::default();
        let config = NormConfig::default();
        let emit = handle(&SetSelectedCheckIndex { index: Some(4) }, &ArtifactView::new(&projection, &HistoryView::empty()), &ConfigView { snapshot: &config }).expect("handle");
        assert!(emit.artifact_mutations.is_empty(), "a view action must never emit document operations");
        assert_eq!(emit.config_mutations, vec![NormConfigMutation::SetSelectedCheckIndex { index: Some(4) }]);
    }
}
//#endregion 🧪️Tests
