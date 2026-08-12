//! 🦠️ ProgramSnapshot mutation — `conflicts` leaf: create/delete/rename/replace conflict rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `Conflict` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::Conflict;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateConflict
/// 🌱️ Brings a new conflict row into existence in `program.conflicts`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateConflict {
    pub conflict: Conflict,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateConflict {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "conflict", kind: "create-conflict", record: "CreatedConflict" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create conflict \"{}\"", self.conflict.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.conflict.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateConflict

//#region 🔖️DeleteConflict
/// 🗑️ Removes a conflict row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteConflict {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteConflict {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "conflict", kind: "delete-conflict", record: "DeletedConflict" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete conflict \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteConflict

//#region 🔖️RenameConflict
/// ✏️ Sets the identity `name` field of one conflict row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameConflict {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameConflict {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "conflict", kind: "rename-conflict", record: "RenamedConflict" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename conflict to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameConflict

//#region 🔖️ReplaceConflict
/// 🔁️ Whole-value swap of one conflict row's non-identity content, addressed by
/// `conflict.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceConflict {
    pub conflict: Conflict,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceConflict {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "conflict", kind: "replace-conflict", record: "ReplacedConflict" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace conflict \"{}\"", self.conflict.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.conflict.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceConflict
