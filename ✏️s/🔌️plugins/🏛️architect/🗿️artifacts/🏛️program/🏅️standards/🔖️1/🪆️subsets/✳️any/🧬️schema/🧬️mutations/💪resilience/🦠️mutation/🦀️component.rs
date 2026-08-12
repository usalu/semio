//! 🦠️ ProgramSnapshot mutation — `resilience` leaf: create/delete/rename/replace resilience requirement rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `ResilienceRequirement` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::ResilienceRequirement;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateResilienceRequirement
/// 🌱️ Brings a new resilience requirement row into existence in `program.resilience`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateResilienceRequirement {
    pub resilience_requirement: ResilienceRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateResilienceRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "resilience-requirement", kind: "create-resilience-requirement", record: "CreatedResilienceRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create resilience requirement \"{}\"", self.resilience_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.resilience_requirement.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateResilienceRequirement

//#region 🔖️DeleteResilienceRequirement
/// 🗑️ Removes a resilience requirement row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteResilienceRequirement {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteResilienceRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "resilience-requirement", kind: "delete-resilience-requirement", record: "DeletedResilienceRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete resilience requirement \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteResilienceRequirement

//#region 🔖️RenameResilienceRequirement
/// ✏️ Sets the identity `name` field of one resilience requirement row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameResilienceRequirement {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameResilienceRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "resilience-requirement", kind: "rename-resilience-requirement", record: "RenamedResilienceRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename resilience requirement to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameResilienceRequirement

//#region 🔖️ReplaceResilienceRequirement
/// 🔁️ Whole-value swap of one resilience requirement row's non-identity content, addressed by
/// `resilience_requirement.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceResilienceRequirement {
    pub resilience_requirement: ResilienceRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceResilienceRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "resilience-requirement", kind: "replace-resilience-requirement", record: "ReplacedResilienceRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace resilience requirement \"{}\"", self.resilience_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.resilience_requirement.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceResilienceRequirement
