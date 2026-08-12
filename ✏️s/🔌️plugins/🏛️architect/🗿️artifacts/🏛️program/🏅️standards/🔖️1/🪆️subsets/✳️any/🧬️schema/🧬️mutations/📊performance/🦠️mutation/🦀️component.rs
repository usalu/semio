//! 🦠️ ProgramSnapshot mutation — `performance` leaf: create/delete/rename/replace performance criterion rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `PerformanceCriterion` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::PerformanceCriterion;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreatePerformanceCriterion
/// 🌱️ Brings a new performance criterion row into existence in `program.performance`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePerformanceCriterion {
    pub performance_criterion: PerformanceCriterion,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreatePerformanceCriterion {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "performance-criterion", kind: "create-performance-criterion", record: "CreatedPerformanceCriterion" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create performance criterion \"{}\"", self.performance_criterion.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.performance_criterion.header.id.0.clone()]
    }
}
//#endregion 🔖️CreatePerformanceCriterion

//#region 🔖️DeletePerformanceCriterion
/// 🗑️ Removes a performance criterion row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeletePerformanceCriterion {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeletePerformanceCriterion {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "performance-criterion", kind: "delete-performance-criterion", record: "DeletedPerformanceCriterion" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete performance criterion \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeletePerformanceCriterion

//#region 🔖️RenamePerformanceCriterion
/// ✏️ Sets the identity `name` field of one performance criterion row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenamePerformanceCriterion {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenamePerformanceCriterion {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "performance-criterion", kind: "rename-performance-criterion", record: "RenamedPerformanceCriterion" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename performance criterion to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenamePerformanceCriterion

//#region 🔖️ReplacePerformanceCriterion
/// 🔁️ Whole-value swap of one performance criterion row's non-identity content, addressed by
/// `performance_criterion.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplacePerformanceCriterion {
    pub performance_criterion: PerformanceCriterion,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplacePerformanceCriterion {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "performance-criterion", kind: "replace-performance-criterion", record: "ReplacedPerformanceCriterion" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace performance criterion \"{}\"", self.performance_criterion.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.performance_criterion.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplacePerformanceCriterion
