//! 📤️ EN 1995 play app command — replace the whole compliance document.
//!
//! 📌️ The payload's `#[dsl(keyword)]` MUST equal the `app_commands!` row's `as` literal: a single-field
//! tuple variant delegates its whole `RecordSpec` to the inner type, whose keyword otherwise defaults to
//! `None` and would print with no leading keyword at all.

use crate::artifacts::en1995::op::En1995Mutation;
use crate::artifacts::en1995::En1995Snapshot;
use crate::config::{NormConfig, NormConfigMutation};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-snapshot")]
pub struct SetSnapshot {
    #[dsl(block)]
    pub snapshot: En1995Snapshot,
}
//#endregion 🔖️Payload

//#region 🔖️Handler
pub fn handle(payload: &SetSnapshot, _doc: &DocumentView<'_, En1995Snapshot>, _cfg: &ConfigView<'_, NormConfig>) -> Result<Emit<En1995Mutation, NormConfigMutation>, Fault> {
    crate::app_surface::commit_snapshot(En1995Mutation::SetSnapshot { snapshot: payload.snapshot.clone() }, "setSnapshot")
}
//#endregion 🔖️Handler

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::en1995::op::En1995Mutation;
    use semio_framework_plugin::HistoryView;

    #[test]
    fn handle_commits_the_payload_document_under_its_action_id() {
        let projection = En1995Snapshot::default();
        let config = NormConfig::default();
        let emit = handle(
            &SetSnapshot { snapshot: En1995Snapshot::default() },
            &DocumentView { snapshot: &projection, history: &HistoryView::empty() },
            &ConfigView { snapshot: &config },
        )
        .expect("handle");
        assert_eq!(emit.document_mutations, vec![En1995Mutation::SetSnapshot { snapshot: En1995Snapshot::default() }]);
        assert_eq!(emit.description.as_deref(), Some("setSnapshot"));
        assert!(emit.config_mutations.is_empty());
    }
}
//#endregion 🧪️Tests
