//! 🦠️ ProgramSnapshot mutation — `environmental` leaf: create/delete/rename/replace environmental requirement rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `EnvironmentalRequirement` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::EnvironmentalRequirement;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateEnvironmentalRequirement
/// 🌱️ Brings a new environmental requirement row into existence in `program.environmental`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEnvironmentalRequirement {
    pub environmental_requirement: EnvironmentalRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateEnvironmentalRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "environmental-requirement", kind: "create-environmental-requirement", record: "CreatedEnvironmentalRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create environmental requirement \"{}\"", self.environmental_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.environmental_requirement.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateEnvironmentalRequirement

//#region 🔖️DeleteEnvironmentalRequirement
/// 🗑️ Removes a environmental requirement row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteEnvironmentalRequirement {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteEnvironmentalRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "environmental-requirement", kind: "delete-environmental-requirement", record: "DeletedEnvironmentalRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete environmental requirement \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteEnvironmentalRequirement

//#region 🔖️RenameEnvironmentalRequirement
/// ✏️ Sets the identity `name` field of one environmental requirement row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameEnvironmentalRequirement {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameEnvironmentalRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "environmental-requirement", kind: "rename-environmental-requirement", record: "RenamedEnvironmentalRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename environmental requirement to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameEnvironmentalRequirement

//#region 🔖️ReplaceEnvironmentalRequirement
/// 🔁️ Whole-value swap of one environmental requirement row's non-identity content, addressed by
/// `environmental_requirement.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceEnvironmentalRequirement {
    pub environmental_requirement: EnvironmentalRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceEnvironmentalRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "environmental-requirement", kind: "replace-environmental-requirement", record: "ReplacedEnvironmentalRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace environmental requirement \"{}\"", self.environmental_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.environmental_requirement.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceEnvironmentalRequirement
