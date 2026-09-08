//! ✏️ Direct Writer mutation — `EditText` replaces the document's authored text body.
use crate::schema::mutations::WriterMutation;
use crate::WriterDiff;
use crate::WriterSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ✏️ Replaces `WriterSnapshot::text` wholesale with `text` — the taxonomy's `edit` verb covers
/// "an authored content body (text, cell, code)". Diff/inverse delegate to the sibling
/// `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf, dsl::ToValue, dsl::FromValue)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "edit-text")]
pub struct EditText {
    pub text: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn edit_text(text: String) -> WriterMutation {
    WriterMutation::EditText(EditText { text })
}

impl MutationKind<WriterSnapshot, WriterMutation> for EditText {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "edit", entity: "text", kind: "edit-text", record: "EditedText" };

    fn diff(&self, base: &WriterSnapshot) -> protocol::MutationOutcome<WriterDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &WriterSnapshot) -> Vec<WriterMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        "Edit document text".to_string()
    }
}
//#endregion 🔖️Mutation
