//! 🦠️ ProgramSnapshot mutation — `functions` leaf: create/delete/rename/replace function rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `Function` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::Function;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateFunction
/// 🌱️ Brings a new function row into existence in `program.functions`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFunction {
    pub function: Function,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateFunction {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "function", kind: "create-function", record: "CreatedFunction" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create function \"{}\"", self.function.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.function.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateFunction

//#region 🔖️DeleteFunction
/// 🗑️ Removes a function row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteFunction {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteFunction {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "function", kind: "delete-function", record: "DeletedFunction" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete function \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteFunction

//#region 🔖️RenameFunction
/// ✏️ Sets the identity `name` field of one function row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameFunction {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameFunction {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "function", kind: "rename-function", record: "RenamedFunction" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename function to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameFunction

//#region 🔖️ReplaceFunction
/// 🔁️ Whole-value swap of one function row's non-identity content, addressed by
/// `function.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceFunction {
    pub function: Function,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceFunction {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "function", kind: "replace-function", record: "ReplacedFunction" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace function \"{}\"", self.function.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.function.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceFunction
