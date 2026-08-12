//! 🦠️ ProgramSnapshot mutation — `validations` leaf: create/delete/rename/replace validation record rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `ValidationRecord` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::ValidationRecord;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateValidationRecord
/// 🌱️ Brings a new validation record row into existence in `program.validations`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateValidationRecord {
    pub validation_record: ValidationRecord,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateValidationRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "validation-record", kind: "create-validation-record", record: "CreatedValidationRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create validation record \"{}\"", self.validation_record.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.validation_record.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateValidationRecord

//#region 🔖️DeleteValidationRecord
/// 🗑️ Removes a validation record row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteValidationRecord {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteValidationRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "validation-record", kind: "delete-validation-record", record: "DeletedValidationRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete validation record \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteValidationRecord

//#region 🔖️RenameValidationRecord
/// ✏️ Sets the identity `name` field of one validation record row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameValidationRecord {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameValidationRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "validation-record", kind: "rename-validation-record", record: "RenamedValidationRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename validation record to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameValidationRecord

//#region 🔖️ReplaceValidationRecord
/// 🔁️ Whole-value swap of one validation record row's non-identity content, addressed by
/// `validation_record.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceValidationRecord {
    pub validation_record: ValidationRecord,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceValidationRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "validation-record", kind: "replace-validation-record", record: "ReplacedValidationRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace validation record \"{}\"", self.validation_record.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.validation_record.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceValidationRecord
