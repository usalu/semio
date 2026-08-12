//! 🦠️ ProgramSnapshot mutation — `organizational` leaf: create/delete/rename/replace organizational requirement rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `OrganizationalRequirement` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::OrganizationalRequirement;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateOrganizationalRequirement
/// 🌱️ Brings a new organizational requirement row into existence in `program.organizational`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrganizationalRequirement {
    pub organizational_requirement: OrganizationalRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateOrganizationalRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "organizational-requirement", kind: "create-organizational-requirement", record: "CreatedOrganizationalRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create organizational requirement \"{}\"", self.organizational_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.organizational_requirement.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateOrganizationalRequirement

//#region 🔖️DeleteOrganizationalRequirement
/// 🗑️ Removes a organizational requirement row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteOrganizationalRequirement {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteOrganizationalRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "organizational-requirement", kind: "delete-organizational-requirement", record: "DeletedOrganizationalRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete organizational requirement \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteOrganizationalRequirement

//#region 🔖️RenameOrganizationalRequirement
/// ✏️ Sets the identity `name` field of one organizational requirement row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameOrganizationalRequirement {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameOrganizationalRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "organizational-requirement", kind: "rename-organizational-requirement", record: "RenamedOrganizationalRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename organizational requirement to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameOrganizationalRequirement

//#region 🔖️ReplaceOrganizationalRequirement
/// 🔁️ Whole-value swap of one organizational requirement row's non-identity content, addressed by
/// `organizational_requirement.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceOrganizationalRequirement {
    pub organizational_requirement: OrganizationalRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceOrganizationalRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "organizational-requirement", kind: "replace-organizational-requirement", record: "ReplacedOrganizationalRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace organizational requirement \"{}\"", self.organizational_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.organizational_requirement.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceOrganizationalRequirement
