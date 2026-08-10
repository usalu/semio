//! 📤️ Vdi3805 play app command — replace the whole compliance document.

use crate::artifacts::vdi3805::op::Vdi3805Mutation;
use crate::artifacts::vdi3805::Vdi3805Snapshot;
use crate::config::{NormConfig, NormConfigMutation};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-snapshot")]
pub struct SetSnapshot {
    #[dsl(block)]
    pub snapshot: Vdi3805Snapshot,
}
//#endregion 🔖️Payload

//#region 🔖️Handler
pub fn handle(payload: &SetSnapshot, _doc: &ArtifactView<'_, Vdi3805Snapshot>, _cfg: &ConfigView<'_, NormConfig>) -> Result<Emit<Vdi3805Mutation, NormConfigMutation>, Fault> {
    crate::app_surface::commit_snapshot(Vdi3805Mutation::SetSnapshot { snapshot: payload.snapshot.clone() }, "setSnapshot")
}
//#endregion 🔖️Handler

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::vdi3805::op::Vdi3805Mutation;
    use semio_framework_plugin::HistoryView;

    #[test]
    fn handle_commits_the_payload_document_under_its_action_id() {
        let projection = Vdi3805Snapshot::default();
        let config = NormConfig::default();
        let emit = handle(
            &SetSnapshot { snapshot: Vdi3805Snapshot::default() },
            &ArtifactView { snapshot: &projection, history: &HistoryView::empty() },
            &ConfigView { snapshot: &config },
        )
        .expect("handle");
        assert_eq!(emit.artifact_mutations, vec![Vdi3805Mutation::SetSnapshot { snapshot: Vdi3805Snapshot::default() }]);
        assert_eq!(emit.description.as_deref(), Some("setSnapshot"));
        assert!(emit.config_mutations.is_empty());
    }
}
//#endregion 🧪️Tests
