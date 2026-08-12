//! 🦠️ ProgramSnapshot mutation — `compliance_records` leaf: create/delete/rename/replace compliance record rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `ComplianceRecord` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::ComplianceRecord;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateComplianceRecord
/// 🌱️ Brings a new compliance record row into existence in `program.compliance_records`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateComplianceRecord {
    pub compliance_record: ComplianceRecord,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateComplianceRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "compliance-record", kind: "create-compliance-record", record: "CreatedComplianceRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create compliance record \"{}\"", self.compliance_record.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.compliance_record.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateComplianceRecord

//#region 🔖️DeleteComplianceRecord
/// 🗑️ Removes a compliance record row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteComplianceRecord {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteComplianceRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "compliance-record", kind: "delete-compliance-record", record: "DeletedComplianceRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete compliance record \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteComplianceRecord

//#region 🔖️RenameComplianceRecord
/// ✏️ Sets the identity `name` field of one compliance record row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameComplianceRecord {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameComplianceRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "compliance-record", kind: "rename-compliance-record", record: "RenamedComplianceRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename compliance record to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameComplianceRecord

//#region 🔖️ReplaceComplianceRecord
/// 🔁️ Whole-value swap of one compliance record row's non-identity content, addressed by
/// `compliance_record.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceComplianceRecord {
    pub compliance_record: ComplianceRecord,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceComplianceRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "compliance-record", kind: "replace-compliance-record", record: "ReplacedComplianceRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace compliance record \"{}\"", self.compliance_record.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.compliance_record.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceComplianceRecord
