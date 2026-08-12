//! 🦠️ ProgramSnapshot mutation — `workshops` leaf: create/delete/rename/replace workshop rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `Workshop` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::Workshop;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateWorkshop
/// 🌱️ Brings a new workshop row into existence in `program.workshops`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWorkshop {
    pub workshop: Workshop,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateWorkshop {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "workshop", kind: "create-workshop", record: "CreatedWorkshop" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create workshop \"{}\"", self.workshop.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.workshop.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateWorkshop

//#region 🔖️DeleteWorkshop
/// 🗑️ Removes a workshop row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteWorkshop {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteWorkshop {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "workshop", kind: "delete-workshop", record: "DeletedWorkshop" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete workshop \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteWorkshop

//#region 🔖️RenameWorkshop
/// ✏️ Sets the identity `name` field of one workshop row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameWorkshop {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameWorkshop {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "workshop", kind: "rename-workshop", record: "RenamedWorkshop" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename workshop to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameWorkshop

//#region 🔖️ReplaceWorkshop
/// 🔁️ Whole-value swap of one workshop row's non-identity content, addressed by
/// `workshop.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceWorkshop {
    pub workshop: Workshop,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceWorkshop {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "workshop", kind: "replace-workshop", record: "ReplacedWorkshop" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace workshop \"{}\"", self.workshop.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.workshop.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceWorkshop
