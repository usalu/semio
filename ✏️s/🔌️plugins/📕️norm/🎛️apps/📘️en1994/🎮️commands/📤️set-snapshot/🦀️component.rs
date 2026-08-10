//! 📤️ En1994 play app command — replace the whole compliance document.

use crate::artifacts::en1994::op::En1994Mutation;
use crate::artifacts::en1994::En1994Snapshot;
use crate::config::{NormConfig, NormConfigMutation};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-snapshot")]
pub struct SetSnapshot {
    #[dsl(block)]
    pub snapshot: En1994Snapshot,
}
//#endregion 🔖️Payload

//#region 🔖️Handler
pub fn handle(payload: &SetSnapshot, _doc: &ArtifactView<'_, En1994Snapshot>, _cfg: &ConfigView<'_, NormConfig>) -> Result<Emit<En1994Mutation, NormConfigMutation>, Fault> {
    crate::app_surface::commit_snapshot(En1994Mutation::SetSnapshot { snapshot: payload.snapshot.clone() }, "setSnapshot")
}
//#endregion 🔖️Handler

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::en1994::op::En1994Mutation;
    use semio_framework_plugin::HistoryView;

    #[test]
    fn handle_commits_the_payload_document_under_its_action_id() {
        let projection = En1994Snapshot::default();
        let config = NormConfig::default();
        let emit = handle(
            &SetSnapshot { snapshot: En1994Snapshot::default() },
            &ArtifactView { snapshot: &projection, history: &HistoryView::empty() },
            &ConfigView { snapshot: &config },
        )
        .expect("handle");
        assert_eq!(emit.artifact_mutations, vec![En1994Mutation::SetSnapshot { snapshot: En1994Snapshot::default() }]);
        assert_eq!(emit.description.as_deref(), Some("setSnapshot"));
        assert!(emit.config_mutations.is_empty());
    }
}
//#endregion 🧪️Tests
