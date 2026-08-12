//! 🦠️ ProgramSnapshot mutation — `collaboration` leaf: create/delete/rename/replace collaboration record rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `CollaborationRecord` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::CollaborationRecord;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateCollaborationRecord
/// 🌱️ Brings a new collaboration record row into existence in `program.collaboration`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCollaborationRecord {
    pub collaboration_record: CollaborationRecord,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateCollaborationRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "collaboration-record", kind: "create-collaboration-record", record: "CreatedCollaborationRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create collaboration record \"{}\"", self.collaboration_record.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.collaboration_record.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateCollaborationRecord

//#region 🔖️DeleteCollaborationRecord
/// 🗑️ Removes a collaboration record row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCollaborationRecord {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteCollaborationRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "collaboration-record", kind: "delete-collaboration-record", record: "DeletedCollaborationRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete collaboration record \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteCollaborationRecord

//#region 🔖️RenameCollaborationRecord
/// ✏️ Sets the identity `name` field of one collaboration record row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameCollaborationRecord {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameCollaborationRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "collaboration-record", kind: "rename-collaboration-record", record: "RenamedCollaborationRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename collaboration record to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameCollaborationRecord

//#region 🔖️ReplaceCollaborationRecord
/// 🔁️ Whole-value swap of one collaboration record row's non-identity content, addressed by
/// `collaboration_record.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceCollaborationRecord {
    pub collaboration_record: CollaborationRecord,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceCollaborationRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "collaboration-record", kind: "replace-collaboration-record", record: "ReplacedCollaborationRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace collaboration record \"{}\"", self.collaboration_record.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.collaboration_record.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceCollaborationRecord
