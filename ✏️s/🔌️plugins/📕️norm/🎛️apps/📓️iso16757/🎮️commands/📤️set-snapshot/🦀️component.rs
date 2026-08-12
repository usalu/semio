//! 📤️ Iso16757 play app command — replace the whole compliance document.
//!
//! 🧩️ The whole-document-replace mutation is banned with no 1:1 replacement (`📓️taxonomy.md`), so the
//! payload decomposes into the closed semantic vocabulary via `Iso16757Mutation::from_snapshot`
//! (base + target, since `product_groups`/`products`/`property_definitions`/`subjects` are real
//! id-keyed collections needing full remove/re-insert), bundled into a single atomic edit.

use crate::artifacts::iso16757::op::Iso16757Mutation;
use crate::artifacts::iso16757::Iso16757Snapshot;
use crate::config::{NormConfig, NormConfigMutation};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-snapshot")]
pub struct ReplaceSnapshot {
    #[dsl(block)]
    pub snapshot: Iso16757Snapshot,
}
//#endregion 🔖️Payload

//#region 🔖️Handler
pub fn handle(payload: &ReplaceSnapshot, doc: &ArtifactView<'_, Iso16757Snapshot>, _cfg: &ConfigView<'_, NormConfig>) -> Result<Emit<Iso16757Mutation, NormConfigMutation>, Fault> {
    crate::app_surface::commit_snapshot_fields(Iso16757Mutation::from_snapshot(doc.snapshot, &payload.snapshot), "setSnapshot")
}
//#endregion 🔖️Handler

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::iso16757::op::Iso16757Mutation;
    use semio_framework_plugin::HistoryView;

    #[test]
    fn handle_commits_the_payload_document_under_its_action_id() {
        let projection = Iso16757Snapshot::default();
        let config = NormConfig::default();
        let emit = handle(
            &ReplaceSnapshot { snapshot: Iso16757Snapshot::default() },
            &ArtifactView { snapshot: &projection, history: &HistoryView::empty() },
            &ConfigView { snapshot: &config },
        )
        .expect("handle");
        assert_eq!(emit.artifact_mutations, Iso16757Mutation::from_snapshot(&Iso16757Snapshot::default(), &Iso16757Snapshot::default()));
        assert_eq!(emit.description.as_deref(), Some("setSnapshot"));
        assert!(emit.config_mutations.is_empty());
    }
}
//#endregion 🧪️Tests
