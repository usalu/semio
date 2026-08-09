//! 📤️ Din4108 play app command — replace the whole compliance document.

use crate::artifacts::din4108::op::Din4108Mutation;
use crate::artifacts::din4108::Din4108Snapshot;
use crate::config::{NormConfig, NormConfigMutation};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-snapshot")]
pub struct SetSnapshot {
    #[dsl(block)]
    pub snapshot: Din4108Snapshot,
}
//#endregion 🔖️Payload

//#region 🔖️Handler
pub fn handle(payload: &SetSnapshot, _doc: &DocumentView<'_, Din4108Snapshot>, _cfg: &ConfigView<'_, NormConfig>) -> Result<Emit<Din4108Mutation, NormConfigMutation>, Fault> {
    crate::app_surface::commit_snapshot(Din4108Mutation::SetSnapshot { snapshot: payload.snapshot.clone() }, "setSnapshot")
}
//#endregion 🔖️Handler

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::din4108::op::Din4108Mutation;
    use semio_framework_plugin::HistoryView;

    #[test]
    fn handle_commits_the_payload_document_under_its_action_id() {
        let projection = Din4108Snapshot::default();
        let config = NormConfig::default();
        let emit = handle(
            &SetSnapshot { snapshot: Din4108Snapshot::default() },
            &DocumentView { snapshot: &projection, history: &HistoryView::empty() },
            &ConfigView { snapshot: &config },
        )
        .expect("handle");
        assert_eq!(emit.document_mutations, vec![Din4108Mutation::SetSnapshot { snapshot: Din4108Snapshot::default() }]);
        assert_eq!(emit.description.as_deref(), Some("setSnapshot"));
        assert!(emit.config_mutations.is_empty());
    }
}
//#endregion 🧪️Tests
