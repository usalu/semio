//! 🌐️ Writer mutation — `ChangeLanguage` payload: sets the document's language mode.
use crate::artifacts::writer::schema::mutations::WriterMutation;
use crate::artifacts::writer::WriterDiff;
use crate::artifacts::writer::WriterSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🌐️ Sets `WriterSnapshot::language_id` to `new_language_id`. Diff/inverse delegate to the
/// sibling `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-language")]
pub struct ChangeLanguage {
    pub new_language_id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn change_language(new_language_id: String) -> WriterMutation {
    WriterMutation::ChangeLanguage(ChangeLanguage { new_language_id })
}

impl MutationKind<WriterSnapshot, WriterMutation> for ChangeLanguage {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "language", kind: "change-language", record: "ChangedLanguage" };

    async fn diff(&self, base: &WriterSnapshot) -> protocol::MutationOutcome<WriterDiff> {
        super::diff::diff(self, base)
    }

    async fn inverse(&self, base: &WriterSnapshot) -> Vec<WriterMutation> {
        super::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change document language to \"{}\"", self.new_language_id)
    }
}
//#endregion 🔖️Mutation
