//! 🏷️ Direct Writer mutation — `RenameWriter` renames the document's identity `id`.
use crate::schema::mutations::WriterMutation;
use crate::WriterDiff;
use crate::WriterSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🏷️ Renames `WriterSnapshot::id` — the document's identity (derived from the last path segment
/// of `uri` when a file is opened, per `open_document`'s app-level handler) — to `new_id`. Diff/
/// inverse delegate to the sibling `🔺️diff`/`↩️inverse` leaves.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf, dsl::ToValue, dsl::FromValue)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
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

    /// 🎯️ The writer's identity. Concurrent writers of the same field conflict; writers of different fields never do.
    fn target(&self) -> Vec<String> {
        vec!["id".to_string()]
    }

    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Rename document to \"{}\"", self.new_id), &format!("Dokument in \"{}\" umbenennen", self.new_id))
    }
}
//#endregion 🔖️Mutation
