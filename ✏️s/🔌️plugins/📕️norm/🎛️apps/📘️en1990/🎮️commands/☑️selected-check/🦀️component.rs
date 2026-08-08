//! ☑️ EN 1990 play app command — point the inspection panel at a different computed check.
//!
//! 📌️ Config-only: it emits `config_mutations`, never document operations — the selected row is view
//! state, not compliance content. Declared as a `view_action`, so the registry's kind discipline
//! actively rejects it if it ever starts emitting document operations.

use crate::artifacts::en1990::op::En1990Mutation;
use crate::artifacts::en1990::En1990Snapshot;
use crate::config::{NormConfig, NormConfigMutation};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
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
pub fn handle(payload: &SetSelectedCheckIndex, _doc: &DocumentView<'_, En1990Snapshot>, _cfg: &ConfigView<'_, NormConfig>) -> Result<Emit<En1990Mutation, NormConfigMutation>, Fault> {
    crate::app_surface::commit_selected_check_index(payload.index)
}
//#endregion 🔖️Handler

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::HistoryView;

    #[test]
    fn handle_emits_only_a_config_operation() {
        let projection = En1990Snapshot::default();
        let config = NormConfig::default();
        let emit = handle(
            &SetSelectedCheckIndex { index: Some(4) },
            &DocumentView { projection: &projection, history: &HistoryView::empty() },
            &ConfigView { projection: &config },
        )
        .expect("handle");
        assert!(emit.document_mutations.is_empty(), "a view action must never emit document operations");
        assert_eq!(emit.config_mutations, vec![NormConfigMutation::SetSelectedCheckIndex { index: Some(4) }]);
    }
}
//#endregion 🧪️Tests
