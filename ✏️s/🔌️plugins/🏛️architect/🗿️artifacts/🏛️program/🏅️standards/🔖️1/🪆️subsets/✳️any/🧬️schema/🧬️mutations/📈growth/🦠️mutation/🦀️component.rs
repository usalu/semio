//! 🦠️ ProgramSnapshot mutation — `growth` leaf: create/delete/rename/replace growth plan rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `GrowthPlan` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::GrowthPlan;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateGrowthPlan
/// 🌱️ Brings a new growth plan row into existence in `program.growth`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateGrowthPlan {
    pub growth_plan: GrowthPlan,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateGrowthPlan {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "growth-plan", kind: "create-growth-plan", record: "CreatedGrowthPlan" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create growth plan \"{}\"", self.growth_plan.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.growth_plan.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateGrowthPlan

//#region 🔖️DeleteGrowthPlan
/// 🗑️ Removes a growth plan row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteGrowthPlan {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteGrowthPlan {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "growth-plan", kind: "delete-growth-plan", record: "DeletedGrowthPlan" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete growth plan \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteGrowthPlan

//#region 🔖️RenameGrowthPlan
/// ✏️ Sets the identity `name` field of one growth plan row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameGrowthPlan {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameGrowthPlan {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "growth-plan", kind: "rename-growth-plan", record: "RenamedGrowthPlan" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename growth plan to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameGrowthPlan

//#region 🔖️ReplaceGrowthPlan
/// 🔁️ Whole-value swap of one growth plan row's non-identity content, addressed by
/// `growth_plan.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceGrowthPlan {
    pub growth_plan: GrowthPlan,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceGrowthPlan {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "growth-plan", kind: "replace-growth-plan", record: "ReplacedGrowthPlan" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace growth plan \"{}\"", self.growth_plan.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.growth_plan.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceGrowthPlan
