//! 🦠️ ProgramSnapshot mutation — `knowledge` leaf: create/delete/rename/replace knowledge record rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `KnowledgeRecord` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::KnowledgeRecord;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateKnowledgeRecord
/// 🌱️ Brings a new knowledge record row into existence in `program.knowledge`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateKnowledgeRecord {
    pub knowledge_record: KnowledgeRecord,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateKnowledgeRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "knowledge-record", kind: "create-knowledge-record", record: "CreatedKnowledgeRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create knowledge record \"{}\"", self.knowledge_record.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.knowledge_record.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateKnowledgeRecord

//#region 🔖️DeleteKnowledgeRecord
/// 🗑️ Removes a knowledge record row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteKnowledgeRecord {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteKnowledgeRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "knowledge-record", kind: "delete-knowledge-record", record: "DeletedKnowledgeRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete knowledge record \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteKnowledgeRecord

//#region 🔖️RenameKnowledgeRecord
/// ✏️ Sets the identity `name` field of one knowledge record row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameKnowledgeRecord {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameKnowledgeRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "knowledge-record", kind: "rename-knowledge-record", record: "RenamedKnowledgeRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename knowledge record to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameKnowledgeRecord

//#region 🔖️ReplaceKnowledgeRecord
/// 🔁️ Whole-value swap of one knowledge record row's non-identity content, addressed by
/// `knowledge_record.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceKnowledgeRecord {
    pub knowledge_record: KnowledgeRecord,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceKnowledgeRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "knowledge-record", kind: "replace-knowledge-record", record: "ReplacedKnowledgeRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace knowledge record \"{}\"", self.knowledge_record.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.knowledge_record.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceKnowledgeRecord
