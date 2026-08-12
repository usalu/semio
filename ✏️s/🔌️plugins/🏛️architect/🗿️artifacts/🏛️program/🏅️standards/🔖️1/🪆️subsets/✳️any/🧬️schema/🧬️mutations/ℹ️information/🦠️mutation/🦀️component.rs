//! 🦠️ ProgramSnapshot mutation — `information` leaf: create/delete/rename/replace information requirement rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `InformationRequirement` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::InformationRequirement;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateInformationRequirement
/// 🌱️ Brings a new information requirement row into existence in `program.information`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateInformationRequirement {
    pub information_requirement: InformationRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateInformationRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "information-requirement", kind: "create-information-requirement", record: "CreatedInformationRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create information requirement \"{}\"", self.information_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.information_requirement.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateInformationRequirement

//#region 🔖️DeleteInformationRequirement
/// 🗑️ Removes a information requirement row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteInformationRequirement {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteInformationRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "information-requirement", kind: "delete-information-requirement", record: "DeletedInformationRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete information requirement \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteInformationRequirement

//#region 🔖️RenameInformationRequirement
/// ✏️ Sets the identity `name` field of one information requirement row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameInformationRequirement {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameInformationRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "information-requirement", kind: "rename-information-requirement", record: "RenamedInformationRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename information requirement to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameInformationRequirement

//#region 🔖️ReplaceInformationRequirement
/// 🔁️ Whole-value swap of one information requirement row's non-identity content, addressed by
/// `information_requirement.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceInformationRequirement {
    pub information_requirement: InformationRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceInformationRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "information-requirement", kind: "replace-information-requirement", record: "ReplacedInformationRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace information requirement \"{}\"", self.information_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.information_requirement.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceInformationRequirement
