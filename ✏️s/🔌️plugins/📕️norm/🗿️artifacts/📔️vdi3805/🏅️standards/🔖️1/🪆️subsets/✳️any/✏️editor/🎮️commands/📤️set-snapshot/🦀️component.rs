//! 📤️ Vdi3805 play app command — replace the whole compliance document.
//!
//! 🧩️ The whole-document-replace mutation is banned with no 1:1 replacement (`📓️taxonomy.md`), so the
//! payload decomposes into the closed semantic vocabulary via `Vdi3805Mutation::from_snapshot`
//! (base + target, since `catalog.products`/`geometry`/`curves` are real id-keyed collections
//! needing full remove/re-insert), bundled into a single atomic edit.

use crate::artifacts::vdi3805::op::Vdi3805Mutation;
use crate::artifacts::vdi3805::Vdi3805Snapshot;
use crate::config::{NormConfig, NormConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-snapshot")]
pub struct ReplaceSnapshot {
    #[dsl(block)]
    pub snapshot: Vdi3805Snapshot,
}
//#endregion 🔖️Payload

//#region 🔖️Handler
pub fn handle(payload: &ReplaceSnapshot, doc: &ArtifactView<'_, Vdi3805Snapshot>, _cfg: &ConfigView<'_, NormConfig>) -> Result<Emit<Vdi3805Mutation, NormConfigMutation>, Fault> {
    crate::app_surface::commit_snapshot_fields(Vdi3805Mutation::from_snapshot(doc.snapshot, &payload.snapshot), "setSnapshot")
}
//#endregion 🔖️Handler

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::vdi3805::op::Vdi3805Mutation;
    use semio_framework_plugin::HistoryView;

    #[semio_framework_async_macros::async_test]
    fn handle_commits_the_payload_document_under_its_action_id() {
        let projection = Vdi3805Snapshot::default();
        let config = NormConfig::default();
        let emit = handle(&ReplaceSnapshot { snapshot: Vdi3805Snapshot::default() }, &ArtifactView::new(&projection, &HistoryView::empty()), &ConfigView { snapshot: &config }).expect("handle");
        assert_eq!(emit.artifact_mutations, Vdi3805Mutation::from_snapshot(&Vdi3805Snapshot::default(), &Vdi3805Snapshot::default()));
        assert_eq!(emit.description.as_deref(), Some("setSnapshot"));
        assert!(emit.config_mutations.is_empty());
    }
}
//#endregion 🧪️Tests
