//! 🏷️ Direct Writer mutation — `RenameWriter` renames the document's identity `id`.
use crate::artifacts::writer::schema::mutations::WriterMutation;
use crate::artifacts::writer::WriterDiff;
use crate::artifacts::writer::WriterSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🏷️ Renames `WriterSnapshot::id` — the document's identity (derived from the last path segment
/// of `uri` when a file is opened, per `open_document`'s app-level handler) — to `new_id`. Diff/
/// inverse delegate to the sibling `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "rename-writer")]
pub struct RenameWriter {
    pub new_id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn rename_writer(new_id: String) -> WriterMutation {
    WriterMutation::RenameWriter(RenameWriter { new_id })
}

impl MutationKind<WriterSnapshot, WriterMutation> for RenameWriter {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "writer", kind: "rename-writer", record: "RenamedWriter" };

    fn diff(&self, base: &WriterSnapshot) -> protocol::MutationOutcome<WriterDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &WriterSnapshot) -> Vec<WriterMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Rename document to \"{}\"", self.new_id)
    }
}
//#endregion 🔖️Mutation
