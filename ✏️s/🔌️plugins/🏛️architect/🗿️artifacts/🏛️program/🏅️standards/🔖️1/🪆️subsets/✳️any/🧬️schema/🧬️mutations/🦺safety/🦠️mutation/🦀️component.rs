//! 🦠️ ProgramSnapshot mutation — `safety` leaf: create/delete/rename/replace safety requirement rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `SafetyRequirement` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::SafetyRequirement;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateSafetyRequirement
/// 🌱️ Brings a new safety requirement row into existence in `program.safety`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSafetyRequirement {
    pub safety_requirement: SafetyRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateSafetyRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "safety-requirement", kind: "create-safety-requirement", record: "CreatedSafetyRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create safety requirement \"{}\"", self.safety_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.safety_requirement.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateSafetyRequirement

//#region 🔖️DeleteSafetyRequirement
/// 🗑️ Removes a safety requirement row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSafetyRequirement {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteSafetyRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "safety-requirement", kind: "delete-safety-requirement", record: "DeletedSafetyRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete safety requirement \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteSafetyRequirement

//#region 🔖️RenameSafetyRequirement
/// ✏️ Sets the identity `name` field of one safety requirement row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameSafetyRequirement {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameSafetyRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "safety-requirement", kind: "rename-safety-requirement", record: "RenamedSafetyRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename safety requirement to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameSafetyRequirement

//#region 🔖️ReplaceSafetyRequirement
/// 🔁️ Whole-value swap of one safety requirement row's non-identity content, addressed by
/// `safety_requirement.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceSafetyRequirement {
    pub safety_requirement: SafetyRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceSafetyRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "safety-requirement", kind: "replace-safety-requirement", record: "ReplacedSafetyRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace safety requirement \"{}\"", self.safety_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.safety_requirement.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceSafetyRequirement
