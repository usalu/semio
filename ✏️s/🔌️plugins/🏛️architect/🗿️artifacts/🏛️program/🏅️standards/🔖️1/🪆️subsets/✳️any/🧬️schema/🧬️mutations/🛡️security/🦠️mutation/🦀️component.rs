//! 🦠️ ProgramSnapshot mutation — `security` leaf: create/delete/rename/replace security requirement rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `SecurityRequirement` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::SecurityRequirement;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateSecurityRequirement
/// 🌱️ Brings a new security requirement row into existence in `program.security`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSecurityRequirement {
    pub security_requirement: SecurityRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateSecurityRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "security-requirement", kind: "create-security-requirement", record: "CreatedSecurityRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create security requirement \"{}\"", self.security_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.security_requirement.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateSecurityRequirement

//#region 🔖️DeleteSecurityRequirement
/// 🗑️ Removes a security requirement row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSecurityRequirement {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteSecurityRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "security-requirement", kind: "delete-security-requirement", record: "DeletedSecurityRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete security requirement \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteSecurityRequirement

//#region 🔖️RenameSecurityRequirement
/// ✏️ Sets the identity `name` field of one security requirement row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameSecurityRequirement {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameSecurityRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "security-requirement", kind: "rename-security-requirement", record: "RenamedSecurityRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename security requirement to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameSecurityRequirement

//#region 🔖️ReplaceSecurityRequirement
/// 🔁️ Whole-value swap of one security requirement row's non-identity content, addressed by
/// `security_requirement.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceSecurityRequirement {
    pub security_requirement: SecurityRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceSecurityRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "security-requirement", kind: "replace-security-requirement", record: "ReplacedSecurityRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace security requirement \"{}\"", self.security_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.security_requirement.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceSecurityRequirement
