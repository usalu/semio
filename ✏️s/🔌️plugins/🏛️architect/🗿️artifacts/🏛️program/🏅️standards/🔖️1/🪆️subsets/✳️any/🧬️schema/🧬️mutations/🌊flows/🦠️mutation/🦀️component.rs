//! 🦠️ ProgramSnapshot mutation — `flows` leaf: create/delete/rename/replace flow requirement rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `FlowRequirement` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::FlowRequirement;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateFlowRequirement
/// 🌱️ Brings a new flow requirement row into existence in `program.flows`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFlowRequirement {
    pub flow_requirement: FlowRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateFlowRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "flow-requirement", kind: "create-flow-requirement", record: "CreatedFlowRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create flow requirement \"{}\"", self.flow_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.flow_requirement.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateFlowRequirement

//#region 🔖️DeleteFlowRequirement
/// 🗑️ Removes a flow requirement row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteFlowRequirement {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteFlowRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "flow-requirement", kind: "delete-flow-requirement", record: "DeletedFlowRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete flow requirement \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteFlowRequirement

//#region 🔖️RenameFlowRequirement
/// ✏️ Sets the identity `name` field of one flow requirement row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameFlowRequirement {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameFlowRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "flow-requirement", kind: "rename-flow-requirement", record: "RenamedFlowRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename flow requirement to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameFlowRequirement

//#region 🔖️ReplaceFlowRequirement
/// 🔁️ Whole-value swap of one flow requirement row's non-identity content, addressed by
/// `flow_requirement.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceFlowRequirement {
    pub flow_requirement: FlowRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceFlowRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "flow-requirement", kind: "replace-flow-requirement", record: "ReplacedFlowRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace flow requirement \"{}\"", self.flow_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.flow_requirement.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceFlowRequirement
