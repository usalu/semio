//! 🦠️ ProgramSnapshot mutation — `documents` leaf: create/delete/rename/replace document rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `ArtifactRecord` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::ArtifactRecord;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateDocument
/// 🌱️ Brings a new document row into existence in `program.artifacts`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDocument {
    pub document: ArtifactRecord,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateDocument {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "document", kind: "create-document", record: "CreatedDocument" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create document \"{}\"", self.document.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.document.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateDocument

//#region 🔖️DeleteDocument
/// 🗑️ Removes a document row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteDocument {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteDocument {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "document", kind: "delete-document", record: "DeletedDocument" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete document \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteDocument

//#region 🔖️RenameDocument
/// ✏️ Sets the identity `name` field of one document row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameDocument {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameDocument {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "document", kind: "rename-document", record: "RenamedDocument" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename document to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameDocument

//#region 🔖️ReplaceDocument
/// 🔁️ Whole-value swap of one document row's non-identity content, addressed by
/// `document.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceDocument {
    pub document: ArtifactRecord,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceDocument {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "document", kind: "replace-document", record: "ReplacedDocument" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace document \"{}\"", self.document.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.document.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceDocument
