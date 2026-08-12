//! 🦠️ ProgramSnapshot mutation — `infrastructure` leaf: create/delete/rename/replace infrastructure requirement rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `InfrastructureRequirement` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::InfrastructureRequirement;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateInfrastructureRequirement
/// 🌱️ Brings a new infrastructure requirement row into existence in `program.infrastructure`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateInfrastructureRequirement {
    pub infrastructure_requirement: InfrastructureRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateInfrastructureRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "infrastructure-requirement", kind: "create-infrastructure-requirement", record: "CreatedInfrastructureRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create infrastructure requirement \"{}\"", self.infrastructure_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.infrastructure_requirement.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateInfrastructureRequirement

//#region 🔖️DeleteInfrastructureRequirement
/// 🗑️ Removes a infrastructure requirement row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteInfrastructureRequirement {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteInfrastructureRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "infrastructure-requirement", kind: "delete-infrastructure-requirement", record: "DeletedInfrastructureRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete infrastructure requirement \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteInfrastructureRequirement

//#region 🔖️RenameInfrastructureRequirement
/// ✏️ Sets the identity `name` field of one infrastructure requirement row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameInfrastructureRequirement {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameInfrastructureRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "infrastructure-requirement", kind: "rename-infrastructure-requirement", record: "RenamedInfrastructureRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename infrastructure requirement to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameInfrastructureRequirement

//#region 🔖️ReplaceInfrastructureRequirement
/// 🔁️ Whole-value swap of one infrastructure requirement row's non-identity content, addressed by
/// `infrastructure_requirement.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceInfrastructureRequirement {
    pub infrastructure_requirement: InfrastructureRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceInfrastructureRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "infrastructure-requirement", kind: "replace-infrastructure-requirement", record: "ReplacedInfrastructureRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace infrastructure requirement \"{}\"", self.infrastructure_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.infrastructure_requirement.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceInfrastructureRequirement
