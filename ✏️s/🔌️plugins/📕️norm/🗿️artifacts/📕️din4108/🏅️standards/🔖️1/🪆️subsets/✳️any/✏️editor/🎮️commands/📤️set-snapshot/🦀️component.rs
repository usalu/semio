//! 📤️ Din4108 play app command — replace the whole compliance document.
//!
//! 🧩️ The whole-document-replace mutation is banned with no 1:1 replacement (`📓️taxonomy.md`), so the
//! payload decomposes into the closed semantic vocabulary via `Din4108Mutation::from_snapshot`
//! (base + target, since `layers` is a real ordered collection needing full remove/re-insert),
//! bundled into a single atomic edit.

use crate::artifacts::din4108::op::Din4108Mutation;
use crate::artifacts::din4108::Din4108Snapshot;
use crate::config::{NormConfig, NormConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-snapshot")]
pub struct ReplaceSnapshot {
    #[dsl(block)]
    pub snapshot: Din4108Snapshot,
}
//#endregion 🔖️Payload

//#region 🔖️Handler
pub async fn handle(payload: &ReplaceSnapshot, doc: &ArtifactView<'_, Din4108Snapshot>, _cfg: &ConfigView<'_, NormConfig>) -> Result<Emit<Din4108Mutation, NormConfigMutation>, Fault> {
    crate::app_surface::commit_snapshot_fields(Din4108Mutation::from_snapshot(doc.snapshot, &payload.snapshot), "setSnapshot")
}
//#endregion 🔖️Handler

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::din4108::op::Din4108Mutation;
    use semio_framework_plugin::HistoryView;

    #[semio_framework_async_macros::async_test]
    async fn handle_commits_the_payload_document_under_its_action_id() {
        let projection = Din4108Snapshot::default();
        let config = NormConfig::default();
        let emit = handle(&ReplaceSnapshot { snapshot: Din4108Snapshot::default() }, &ArtifactView::new(&projection, &HistoryView::empty()), &ConfigView { snapshot: &config }).expect("handle");
        assert_eq!(emit.artifact_mutations, Din4108Mutation::from_snapshot(&Din4108Snapshot::default(), &Din4108Snapshot::default()));
        assert_eq!(emit.description.as_deref(), Some("setSnapshot"));
        assert!(emit.config_mutations.is_empty());
    }
}
//#endregion 🧪️Tests
