//! 🦠️ ProgramSnapshot mutation — `costs` leaf: create/delete/rename/replace cost requirement rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `CostRequirement` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::CostRequirement;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateCostRequirement
/// 🌱️ Brings a new cost requirement row into existence in `program.costs`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCostRequirement {
    pub cost_requirement: CostRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateCostRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "cost-requirement", kind: "create-cost-requirement", record: "CreatedCostRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create cost requirement \"{}\"", self.cost_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.cost_requirement.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateCostRequirement

//#region 🔖️DeleteCostRequirement
/// 🗑️ Removes a cost requirement row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCostRequirement {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteCostRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "cost-requirement", kind: "delete-cost-requirement", record: "DeletedCostRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete cost requirement \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteCostRequirement

//#region 🔖️RenameCostRequirement
/// ✏️ Sets the identity `name` field of one cost requirement row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameCostRequirement {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameCostRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "cost-requirement", kind: "rename-cost-requirement", record: "RenamedCostRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename cost requirement to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameCostRequirement

//#region 🔖️ReplaceCostRequirement
/// 🔁️ Whole-value swap of one cost requirement row's non-identity content, addressed by
/// `cost_requirement.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceCostRequirement {
    pub cost_requirement: CostRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceCostRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "cost-requirement", kind: "replace-cost-requirement", record: "ReplacedCostRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace cost requirement \"{}\"", self.cost_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.cost_requirement.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceCostRequirement
