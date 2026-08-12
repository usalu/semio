//! 🦠️ ProgramSnapshot mutation — `services` leaf: create/delete/rename/replace service requirement rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `ServiceRequirement` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::ServiceRequirement;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateServiceRequirement
/// 🌱️ Brings a new service requirement row into existence in `program.services`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateServiceRequirement {
    pub service_requirement: ServiceRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateServiceRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "service-requirement", kind: "create-service-requirement", record: "CreatedServiceRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create service requirement \"{}\"", self.service_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.service_requirement.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateServiceRequirement

//#region 🔖️DeleteServiceRequirement
/// 🗑️ Removes a service requirement row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteServiceRequirement {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteServiceRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "service-requirement", kind: "delete-service-requirement", record: "DeletedServiceRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete service requirement \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteServiceRequirement

//#region 🔖️RenameServiceRequirement
/// ✏️ Sets the identity `name` field of one service requirement row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameServiceRequirement {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameServiceRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "service-requirement", kind: "rename-service-requirement", record: "RenamedServiceRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename service requirement to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameServiceRequirement

//#region 🔖️ReplaceServiceRequirement
/// 🔁️ Whole-value swap of one service requirement row's non-identity content, addressed by
/// `service_requirement.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceServiceRequirement {
    pub service_requirement: ServiceRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceServiceRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "service-requirement", kind: "replace-service-requirement", record: "ReplacedServiceRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace service requirement \"{}\"", self.service_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.service_requirement.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceServiceRequirement
