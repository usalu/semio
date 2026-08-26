//! 🔗️ Writer mutation — `ChangeUri` payload: sets the document's addressable location.
use crate::artifacts::writer::schema::mutations::WriterMutation;
use crate::artifacts::writer::WriterDiff;
use crate::artifacts::writer::WriterSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔗️ Sets `WriterSnapshot::uri` to `new_uri`. Diff/inverse delegate to the sibling
/// `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-uri")]
pub struct ChangeUri {
    pub new_uri: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_uri(new_uri: String) -> WriterMutation {
    WriterMutation::ChangeUri(ChangeUri { new_uri })
}

impl MutationKind<WriterSnapshot, WriterMutation> for ChangeUri {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "uri", kind: "change-uri", record: "ChangedUri" };

    fn diff(&self, base: &WriterSnapshot) -> protocol::MutationOutcome<WriterDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &WriterSnapshot) -> Vec<WriterMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change document URI to \"{}\"", self.new_uri)
    }
}
//#endregion 🔖️Mutation
