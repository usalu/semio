//! 🦠️ ProgramSnapshot mutation — `elements` leaf: create/delete/rename/replace program element rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `ProgramElement` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::ProgramElement;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateProgramElement
/// 🌱️ Brings a new program element row into existence in `program.elements`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProgramElement {
    pub program_element: ProgramElement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateProgramElement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "program-element", kind: "create-program-element", record: "CreatedProgramElement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create program element \"{}\"", self.program_element.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.program_element.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateProgramElement

//#region 🔖️DeleteProgramElement
/// 🗑️ Removes a program element row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteProgramElement {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteProgramElement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "program-element", kind: "delete-program-element", record: "DeletedProgramElement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete program element \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteProgramElement

//#region 🔖️RenameProgramElement
/// ✏️ Sets the identity `name` field of one program element row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameProgramElement {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameProgramElement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "program-element", kind: "rename-program-element", record: "RenamedProgramElement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename program element to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameProgramElement

//#region 🔖️ReplaceProgramElement
/// 🔁️ Whole-value swap of one program element row's non-identity content, addressed by
/// `program_element.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceProgramElement {
    pub program_element: ProgramElement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceProgramElement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "program-element", kind: "replace-program-element", record: "ReplacedProgramElement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace program element \"{}\"", self.program_element.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.program_element.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceProgramElement
