//! 🦠️ ProgramSnapshot mutation — `human_factors` leaf: create/delete/rename/replace human factor requirement rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `HumanFactorRequirement` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::HumanFactorRequirement;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateHumanFactorRequirement
/// 🌱️ Brings a new human factor requirement row into existence in `program.human_factors`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateHumanFactorRequirement {
    pub human_factor_requirement: HumanFactorRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateHumanFactorRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "human-factor-requirement", kind: "create-human-factor-requirement", record: "CreatedHumanFactorRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create human factor requirement \"{}\"", self.human_factor_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.human_factor_requirement.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateHumanFactorRequirement

//#region 🔖️DeleteHumanFactorRequirement
/// 🗑️ Removes a human factor requirement row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteHumanFactorRequirement {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteHumanFactorRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "human-factor-requirement", kind: "delete-human-factor-requirement", record: "DeletedHumanFactorRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete human factor requirement \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteHumanFactorRequirement

//#region 🔖️RenameHumanFactorRequirement
/// ✏️ Sets the identity `name` field of one human factor requirement row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameHumanFactorRequirement {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameHumanFactorRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "human-factor-requirement", kind: "rename-human-factor-requirement", record: "RenamedHumanFactorRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename human factor requirement to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameHumanFactorRequirement

//#region 🔖️ReplaceHumanFactorRequirement
/// 🔁️ Whole-value swap of one human factor requirement row's non-identity content, addressed by
/// `human_factor_requirement.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceHumanFactorRequirement {
    pub human_factor_requirement: HumanFactorRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceHumanFactorRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "human-factor-requirement", kind: "replace-human-factor-requirement", record: "ReplacedHumanFactorRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace human factor requirement \"{}\"", self.human_factor_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.human_factor_requirement.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceHumanFactorRequirement
