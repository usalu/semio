//! 🦠️ ProgramSnapshot mutation — `processes` leaf: create/delete/rename/replace process rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `Process` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::Process;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateProcess
/// 🌱️ Brings a new process row into existence in `program.processes`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProcess {
    pub process: Process,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateProcess {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "process", kind: "create-process", record: "CreatedProcess" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create process \"{}\"", self.process.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.process.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateProcess

//#region 🔖️DeleteProcess
/// 🗑️ Removes a process row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteProcess {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteProcess {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "process", kind: "delete-process", record: "DeletedProcess" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete process \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteProcess

//#region 🔖️RenameProcess
/// ✏️ Sets the identity `name` field of one process row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameProcess {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameProcess {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "process", kind: "rename-process", record: "RenamedProcess" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename process to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameProcess

//#region 🔖️ReplaceProcess
/// 🔁️ Whole-value swap of one process row's non-identity content, addressed by
/// `process.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceProcess {
    pub process: Process,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceProcess {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "process", kind: "replace-process", record: "ReplacedProcess" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace process \"{}\"", self.process.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.process.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceProcess
