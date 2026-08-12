//! 🦠️ ProgramSnapshot mutation — `update_project` leaf: `RenameProject`/`ReplaceProject`.
//! `ProjectDefinition` is a document-level scalar facet (`program.project`) per
//! `📓️derivation-rules.md` rule 1 — same shape/rationale as `🏷️update-meta`, identity field is
//! `code`. Supersedes the banned raw-Patch-payload `UpdateProject { patch: ProjectDefinitionPatch }`.

use crate::artifacts::program::registers::ProjectDefinition;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️RenameProject
/// ✏️ Sets `program.project.code`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameProject {
    pub new_code: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameProject {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "project", kind: "rename-project", record: "RenamedProject" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename project to \"{}\"", self.new_code)
    }
}
//#endregion 🔖️RenameProject

//#region 🔖️ReplaceProject
/// 🔁️ Whole-value swap of `program.project`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceProject {
    pub new_project: ProjectDefinition,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceProject {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "project", kind: "replace-project", record: "ReplacedProject" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace project definition \"{}\"", self.new_project.code)
    }
}
//#endregion 🔖️ReplaceProject
