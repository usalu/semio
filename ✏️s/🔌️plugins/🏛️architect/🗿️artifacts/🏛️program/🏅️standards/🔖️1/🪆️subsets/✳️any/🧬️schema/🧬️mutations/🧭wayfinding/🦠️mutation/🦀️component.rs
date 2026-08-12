//! 🦠️ ProgramSnapshot mutation — `wayfinding` leaf: create/delete/rename/replace wayfinding requirement rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `WayfindingRequirement` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::WayfindingRequirement;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateWayfindingRequirement
/// 🌱️ Brings a new wayfinding requirement row into existence in `program.wayfinding`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWayfindingRequirement {
    pub wayfinding_requirement: WayfindingRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateWayfindingRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "wayfinding-requirement", kind: "create-wayfinding-requirement", record: "CreatedWayfindingRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create wayfinding requirement \"{}\"", self.wayfinding_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.wayfinding_requirement.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateWayfindingRequirement

//#region 🔖️DeleteWayfindingRequirement
/// 🗑️ Removes a wayfinding requirement row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteWayfindingRequirement {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteWayfindingRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "wayfinding-requirement", kind: "delete-wayfinding-requirement", record: "DeletedWayfindingRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete wayfinding requirement \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteWayfindingRequirement

//#region 🔖️RenameWayfindingRequirement
/// ✏️ Sets the identity `name` field of one wayfinding requirement row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameWayfindingRequirement {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameWayfindingRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "wayfinding-requirement", kind: "rename-wayfinding-requirement", record: "RenamedWayfindingRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename wayfinding requirement to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameWayfindingRequirement

//#region 🔖️ReplaceWayfindingRequirement
/// 🔁️ Whole-value swap of one wayfinding requirement row's non-identity content, addressed by
/// `wayfinding_requirement.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceWayfindingRequirement {
    pub wayfinding_requirement: WayfindingRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceWayfindingRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "wayfinding-requirement", kind: "replace-wayfinding-requirement", record: "ReplacedWayfindingRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace wayfinding requirement \"{}\"", self.wayfinding_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.wayfinding_requirement.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceWayfindingRequirement
