//! 🦠️ ProgramSnapshot mutation — `assumptions` leaf: create/delete/rename/replace assumption rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `Assumption` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::Assumption;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateAssumption
/// 🌱️ Brings a new assumption row into existence in `program.assumptions`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAssumption {
    pub assumption: Assumption,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateAssumption {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "assumption", kind: "create-assumption", record: "CreatedAssumption" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create assumption \"{}\"", self.assumption.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.assumption.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateAssumption

//#region 🔖️DeleteAssumption
/// 🗑️ Removes a assumption row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteAssumption {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteAssumption {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "assumption", kind: "delete-assumption", record: "DeletedAssumption" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete assumption \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteAssumption

//#region 🔖️RenameAssumption
/// ✏️ Sets the identity `name` field of one assumption row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameAssumption {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameAssumption {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "assumption", kind: "rename-assumption", record: "RenamedAssumption" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename assumption to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameAssumption

//#region 🔖️ReplaceAssumption
/// 🔁️ Whole-value swap of one assumption row's non-identity content, addressed by
/// `assumption.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceAssumption {
    pub assumption: Assumption,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceAssumption {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "assumption", kind: "replace-assumption", record: "ReplacedAssumption" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace assumption \"{}\"", self.assumption.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.assumption.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceAssumption
