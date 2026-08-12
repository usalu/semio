//! 🦠️ ProgramSnapshot mutation — `approvals` leaf: create/delete/rename/replace approval record rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `ApprovalRecord` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::ApprovalRecord;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateApprovalRecord
/// 🌱️ Brings a new approval record row into existence in `program.approvals`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateApprovalRecord {
    pub approval_record: ApprovalRecord,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateApprovalRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "approval-record", kind: "create-approval-record", record: "CreatedApprovalRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create approval record \"{}\"", self.approval_record.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.approval_record.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateApprovalRecord

//#region 🔖️DeleteApprovalRecord
/// 🗑️ Removes a approval record row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteApprovalRecord {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteApprovalRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "approval-record", kind: "delete-approval-record", record: "DeletedApprovalRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete approval record \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteApprovalRecord

//#region 🔖️RenameApprovalRecord
/// ✏️ Sets the identity `name` field of one approval record row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameApprovalRecord {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameApprovalRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "approval-record", kind: "rename-approval-record", record: "RenamedApprovalRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename approval record to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameApprovalRecord

//#region 🔖️ReplaceApprovalRecord
/// 🔁️ Whole-value swap of one approval record row's non-identity content, addressed by
/// `approval_record.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceApprovalRecord {
    pub approval_record: ApprovalRecord,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceApprovalRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "approval-record", kind: "replace-approval-record", record: "ReplacedApprovalRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace approval record \"{}\"", self.approval_record.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.approval_record.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceApprovalRecord
