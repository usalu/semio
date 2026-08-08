//! 📤️ EN 1995 play app command — replace the whole compliance document.
//!
//! 📌️ The payload's `#[dsl(keyword)]` MUST equal the `app_commands!` row's `as` literal: a single-field
//! tuple variant delegates its whole `RecordSpec` to the inner type, whose keyword otherwise defaults to
//! `None` and would print with no leading keyword at all.

use crate::artifacts::en1995::op::En1995Mutation;
use crate::artifacts::en1995::Document;
use crate::config::{NormConfig, NormConfigMutation};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-document")]
pub struct SetDocument {
    #[dsl(block)]
    pub document: Document,
}
//#endregion 🔖️Payload

//#region 🔖️Handler
pub fn handle(payload: &SetDocument, _doc: &DocumentView<'_, Document>, _cfg: &ConfigView<'_, NormConfig>) -> Result<Emit<En1995Mutation, NormConfigMutation>, Fault> {
    crate::app_surface::commit_document(payload.document.clone(), "setDocument")
}
//#endregion 🔖️Handler

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::SetDocumentMutation;
    use semio_framework_plugin::HistoryView;

    #[test]
    fn handle_commits_the_payload_document_under_its_action_id() {
        let projection = Document::default();
        let config = NormConfig::default();
        let emit = handle(
            &SetDocument { document: Document::default() },
            &DocumentView { projection: &projection, history: &HistoryView::empty() },
            &ConfigView { projection: &config },
        )
        .expect("handle");
        assert_eq!(emit.document_mutations, vec![SetDocumentMutation::SetDocument { document: Document::default() }]);
        assert_eq!(emit.description.as_deref(), Some("setDocument"));
        assert!(emit.config_mutations.is_empty());
    }
}
//#endregion 🧪️Tests
