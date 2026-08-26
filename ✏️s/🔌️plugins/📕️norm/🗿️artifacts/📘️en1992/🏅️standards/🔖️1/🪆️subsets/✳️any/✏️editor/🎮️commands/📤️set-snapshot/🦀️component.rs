//! 📤️ EN 1992 play app command — replace the whole compliance document.
//!
//! 📌️ The payload's `#[dsl(keyword)]` MUST equal the `app_commands!` row's `as` literal: a single-field
//! tuple variant delegates its whole `RecordSpec` to the inner type, whose keyword otherwise defaults to
//! `None` and would print with no leading keyword at all.
//!
//! 🧩️ The whole-document-replace mutation is banned with no 1:1 replacement (`📓️taxonomy.md`), so the
//! payload decomposes into one `change-<field>` mutation per persistent field via
//! `En1992Mutation::from_snapshot`, bundled into a single atomic edit.

use crate::artifacts::en1992::op::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;
use crate::config::{NormConfig, NormConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-snapshot")]
pub struct ReplaceSnapshot {
    #[dsl(block)]
    pub snapshot: En1992Snapshot,
}
//#endregion 🔖️Payload

//#region 🔖️Handler
pub fn handle(payload: &ReplaceSnapshot, _doc: &ArtifactView<'_, En1992Snapshot>, _cfg: &ConfigView<'_, NormConfig>) -> Result<Emit<En1992Mutation, NormConfigMutation>, Fault> {
    crate::app_surface::commit_snapshot_fields(En1992Mutation::from_snapshot(&payload.snapshot), "setSnapshot")
}
//#endregion 🔖️Handler

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::en1992::op::En1992Mutation;
    use semio_framework_plugin::HistoryView;

    #[semio_framework_async_macros::async_test]
    fn handle_commits_the_payload_document_under_its_action_id() {
        let projection = En1992Snapshot::default();
        let config = NormConfig::default();
        let emit = handle(&ReplaceSnapshot { snapshot: En1992Snapshot::default() }, &ArtifactView::new(&projection, &HistoryView::empty()), &ConfigView { snapshot: &config }).expect("handle");
        assert_eq!(emit.artifact_mutations, En1992Mutation::from_snapshot(&En1992Snapshot::default()));
        assert_eq!(emit.description.as_deref(), Some("setSnapshot"));
        assert!(emit.config_mutations.is_empty());
    }
}
//#endregion 🧪️Tests
