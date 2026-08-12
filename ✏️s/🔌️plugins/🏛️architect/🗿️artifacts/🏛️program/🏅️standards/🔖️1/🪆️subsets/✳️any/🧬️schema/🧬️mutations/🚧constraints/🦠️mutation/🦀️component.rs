//! 🦠️ ProgramSnapshot mutation — `constraints` leaf: create/delete/rename/replace constraint record rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `ConstraintRecord` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::ConstraintRecord;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateConstraintRecord
/// 🌱️ Brings a new constraint record row into existence in `program.constraints`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateConstraintRecord {
    pub constraint_record: ConstraintRecord,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateConstraintRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "constraint-record", kind: "create-constraint-record", record: "CreatedConstraintRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create constraint record \"{}\"", self.constraint_record.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.constraint_record.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateConstraintRecord

//#region 🔖️DeleteConstraintRecord
/// 🗑️ Removes a constraint record row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteConstraintRecord {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteConstraintRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "constraint-record", kind: "delete-constraint-record", record: "DeletedConstraintRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete constraint record \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteConstraintRecord

//#region 🔖️RenameConstraintRecord
/// ✏️ Sets the identity `name` field of one constraint record row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameConstraintRecord {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameConstraintRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "constraint-record", kind: "rename-constraint-record", record: "RenamedConstraintRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename constraint record to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameConstraintRecord

//#region 🔖️ReplaceConstraintRecord
/// 🔁️ Whole-value swap of one constraint record row's non-identity content, addressed by
/// `constraint_record.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceConstraintRecord {
    pub constraint_record: ConstraintRecord,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceConstraintRecord {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "constraint-record", kind: "replace-constraint-record", record: "ReplacedConstraintRecord" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace constraint record \"{}\"", self.constraint_record.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.constraint_record.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceConstraintRecord
