//! 📤️ EN 1997 play app command — replace the whole compliance document.
//!
//! 📌️ The payload's `#[dsl(keyword)]` MUST equal the `app_commands!` row's `as` literal: a single-field
//! tuple variant delegates its whole `RecordSpec` to the inner type, whose keyword otherwise defaults to
//! `None` and would print with no leading keyword at all.

use crate::artifacts::en1997::op::En1997Mutation;
use crate::artifacts::en1997::En1997Snapshot;
use crate::config::{NormConfig, NormConfigMutation};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-snapshot")]
pub struct SetSnapshot {
    #[dsl(block)]
    pub snapshot: En1997Snapshot,
}
//#endregion 🔖️Payload

//#region 🔖️Handler
pub fn handle(payload: &SetSnapshot, _doc: &ArtifactView<'_, En1997Snapshot>, _cfg: &ConfigView<'_, NormConfig>) -> Result<Emit<En1997Mutation, NormConfigMutation>, Fault> {
    crate::app_surface::commit_snapshot(En1997Mutation::SetSnapshot { snapshot: payload.snapshot.clone() }, "setSnapshot")
}
//#endregion 🔖️Handler

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::en1997::op::En1997Mutation;
    use semio_framework_plugin::HistoryView;

    #[test]
    fn handle_commits_the_payload_document_under_its_action_id() {
        let projection = En1997Snapshot::default();
        let config = NormConfig::default();
        let emit = handle(
            &SetSnapshot { snapshot: En1997Snapshot::default() },
            &ArtifactView { snapshot: &projection, history: &HistoryView::empty() },
            &ConfigView { snapshot: &config },
        )
        .expect("handle");
        assert_eq!(emit.artifact_mutations, vec![En1997Mutation::SetSnapshot { snapshot: En1997Snapshot::default() }]);
        assert_eq!(emit.description.as_deref(), Some("setSnapshot"));
        assert!(emit.config_mutations.is_empty());
    }
}
//#endregion 🧪️Tests
