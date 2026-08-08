//! 🧮️ ISO 16757 play app command — recompute the compliance report in place.
//!
//! 📌️ Recommitting the current projection is what forces `NormHost` to re-evaluate: the report is not
//! document state, it is derived on every read, so a no-op whole-document commit is the honest way to
//! record "the user asked for a fresh evaluation" in the command log.

use crate::artifacts::iso16757::op::Iso16757Mutation;
use crate::artifacts::iso16757::Document;
use crate::config::{NormConfig, NormConfigMutation};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
/// 🧮️ Fieldless — this replaces a bare unit variant, whose wire form (`evaluate` / `01 <ord> 00 00`) a
/// fieldless `DslRecord` struct reproduces exactly.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "evaluate")]
pub struct Evaluate {}
//#endregion 🔖️Payload

//#region 🔖️Handler
pub fn handle(_payload: &Evaluate, doc: &DocumentView<'_, Document>, _cfg: &ConfigView<'_, NormConfig>) -> Result<Emit<Iso16757Mutation, NormConfigMutation>, Fault> {
    crate::app_surface::commit_document(doc.projection.clone(), "evaluate")
}
//#endregion 🔖️Handler

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::SetDocumentMutation;
    use semio_framework_plugin::HistoryView;

    #[test]
    fn handle_recommits_the_current_projection_under_its_action_id() {
        let projection = Document::default();
        let config = NormConfig::default();
        let emit = handle(&Evaluate {}, &DocumentView { projection: &projection, history: &HistoryView::empty() }, &ConfigView { projection: &config }).expect("handle");
        assert_eq!(emit.document_mutations, vec![SetDocumentMutation::SetDocument { document: Document::default() }]);
        assert_eq!(emit.description.as_deref(), Some("evaluate"));
    }
}
//#endregion 🧪️Tests
