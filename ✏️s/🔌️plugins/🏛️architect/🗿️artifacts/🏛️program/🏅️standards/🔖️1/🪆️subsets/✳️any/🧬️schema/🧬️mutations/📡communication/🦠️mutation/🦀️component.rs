//! 🦠️ ProgramSnapshot mutation — `communication` leaf: create/delete/rename/replace communication requirement rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `CommunicationRequirement` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::CommunicationRequirement;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateCommunicationRequirement
/// 🌱️ Brings a new communication requirement row into existence in `program.communication`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCommunicationRequirement {
    pub communication_requirement: CommunicationRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateCommunicationRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "communication-requirement", kind: "create-communication-requirement", record: "CreatedCommunicationRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create communication requirement \"{}\"", self.communication_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.communication_requirement.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateCommunicationRequirement

//#region 🔖️DeleteCommunicationRequirement
/// 🗑️ Removes a communication requirement row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCommunicationRequirement {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteCommunicationRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "communication-requirement", kind: "delete-communication-requirement", record: "DeletedCommunicationRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete communication requirement \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteCommunicationRequirement

//#region 🔖️RenameCommunicationRequirement
/// ✏️ Sets the identity `name` field of one communication requirement row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameCommunicationRequirement {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameCommunicationRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "communication-requirement", kind: "rename-communication-requirement", record: "RenamedCommunicationRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename communication requirement to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameCommunicationRequirement

//#region 🔖️ReplaceCommunicationRequirement
/// 🔁️ Whole-value swap of one communication requirement row's non-identity content, addressed by
/// `communication_requirement.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceCommunicationRequirement {
    pub communication_requirement: CommunicationRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceCommunicationRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "communication-requirement", kind: "replace-communication-requirement", record: "ReplacedCommunicationRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace communication requirement \"{}\"", self.communication_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.communication_requirement.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceCommunicationRequirement
