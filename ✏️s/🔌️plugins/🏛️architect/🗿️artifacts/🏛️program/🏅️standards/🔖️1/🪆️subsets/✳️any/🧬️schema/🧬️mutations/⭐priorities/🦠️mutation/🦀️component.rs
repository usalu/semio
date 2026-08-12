//! 🦠️ ProgramSnapshot mutation — `priorities` leaf: create/delete/rename/replace priority record rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `PriorityRecord` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::PriorityRecord;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreatePriorityRecord
/// 🌱️ Brings a new priority record row into existence in `program.priorities`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePriorityRecord {
    pub priority_record: PriorityRecord,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreatePriorityRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "priority-record", kind: "create-priority-record", record: "CreatedPriorityRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create priority record \"{}\"", self.priority_record.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.priority_record.header.id.0.clone()]
    }
}
//#endregion 🔖️CreatePriorityRecord

//#region 🔖️DeletePriorityRecord
/// 🗑️ Removes a priority record row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeletePriorityRecord {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeletePriorityRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "priority-record", kind: "delete-priority-record", record: "DeletedPriorityRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete priority record \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeletePriorityRecord

//#region 🔖️RenamePriorityRecord
/// ✏️ Sets the identity `name` field of one priority record row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenamePriorityRecord {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenamePriorityRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "priority-record", kind: "rename-priority-record", record: "RenamedPriorityRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename priority record to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenamePriorityRecord

//#region 🔖️ReplacePriorityRecord
/// 🔁️ Whole-value swap of one priority record row's non-identity content, addressed by
/// `priority_record.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplacePriorityRecord {
    pub priority_record: PriorityRecord,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplacePriorityRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "priority-record", kind: "replace-priority-record", record: "ReplacedPriorityRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace priority record \"{}\"", self.priority_record.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.priority_record.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplacePriorityRecord
