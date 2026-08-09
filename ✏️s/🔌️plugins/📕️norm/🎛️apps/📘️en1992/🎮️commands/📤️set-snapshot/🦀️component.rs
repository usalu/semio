//! 📤️ En1992 play app command — replace the whole compliance document.

use crate::artifacts::en1992::op::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;
use crate::config::{NormConfig, NormConfigMutation};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-snapshot")]
pub struct SetSnapshot {
    #[dsl(block)]
    pub snapshot: En1992Snapshot,
}
//#endregion 🔖️Payload

//#region 🔖️Handler
pub fn handle(payload: &SetSnapshot, _doc: &DocumentView<'_, En1992Snapshot>, _cfg: &ConfigView<'_, NormConfig>) -> Result<Emit<En1992Mutation, NormConfigMutation>, Fault> {
    crate::app_surface::commit_snapshot(En1992Mutation::SetSnapshot { snapshot: payload.snapshot.clone() }, "setSnapshot")
}
//#endregion 🔖️Handler

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::en1992::op::En1992Mutation;
    use semio_framework_plugin::HistoryView;

    #[test]
    fn handle_commits_the_payload_document_under_its_action_id() {
        let projection = En1992Snapshot::default();
        let config = NormConfig::default();
        let emit = handle(
            &SetSnapshot { snapshot: En1992Snapshot::default() },
            &DocumentView { snapshot: &projection, history: &HistoryView::empty() },
            &ConfigView { snapshot: &config },
        )
        .expect("handle");
        assert_eq!(emit.document_mutations, vec![En1992Mutation::SetSnapshot { snapshot: En1992Snapshot::default() }]);
        assert_eq!(emit.description.as_deref(), Some("setSnapshot"));
        assert!(emit.config_mutations.is_empty());
    }
}
//#endregion 🧪️Tests
