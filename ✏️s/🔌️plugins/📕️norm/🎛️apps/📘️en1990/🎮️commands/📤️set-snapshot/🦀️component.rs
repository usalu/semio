//! 📤️ En1990 play app command — replace the whole compliance document.
//!
//! 🧩️ The whole-document-replace mutation is banned with no 1:1 replacement (`📓️taxonomy.md`), so the
//! payload decomposes into the closed semantic vocabulary via `En1990Mutation::from_snapshot`
//! (base + target, since `q_k` is a real ordered collection needing full remove/re-insert), bundled
//! into a single atomic edit.

use crate::artifacts::en1990::op::En1990Mutation;
use crate::artifacts::en1990::En1990Snapshot;
use crate::config::{NormConfig, NormConfigMutation};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-snapshot")]
pub struct ReplaceSnapshot {
    #[dsl(block)]
    pub snapshot: En1990Snapshot,
}
//#endregion 🔖️Payload

//#region 🔖️Handler
pub fn handle(payload: &ReplaceSnapshot, doc: &ArtifactView<'_, En1990Snapshot>, _cfg: &ConfigView<'_, NormConfig>) -> Result<Emit<En1990Mutation, NormConfigMutation>, Fault> {
    crate::app_surface::commit_snapshot_fields(En1990Mutation::from_snapshot(doc.snapshot, &payload.snapshot), "setSnapshot")
}
//#endregion 🔖️Handler

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::en1990::op::En1990Mutation;
    use semio_framework_plugin::HistoryView;

    #[test]
    fn handle_commits_the_payload_document_under_its_action_id() {
        let projection = En1990Snapshot::default();
        let config = NormConfig::default();
        let emit = handle(
            &ReplaceSnapshot { snapshot: En1990Snapshot::default() },
            &ArtifactView { snapshot: &projection, history: &HistoryView::empty() },
            &ConfigView { snapshot: &config },
        )
        .expect("handle");
        assert_eq!(emit.artifact_mutations, En1990Mutation::from_snapshot(&En1990Snapshot::default(), &En1990Snapshot::default()));
        assert_eq!(emit.description.as_deref(), Some("setSnapshot"));
        assert!(emit.config_mutations.is_empty());
    }
}
//#endregion 🧪️Tests
