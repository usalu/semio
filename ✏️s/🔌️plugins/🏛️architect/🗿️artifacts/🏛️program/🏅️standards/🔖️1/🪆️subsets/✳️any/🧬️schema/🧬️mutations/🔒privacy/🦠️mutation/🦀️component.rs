//! 🦠️ ProgramSnapshot mutation — `privacy` leaf: create/delete/rename/replace privacy requirement rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `PrivacyRequirement` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::PrivacyRequirement;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreatePrivacyRequirement
/// 🌱️ Brings a new privacy requirement row into existence in `program.privacy`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePrivacyRequirement {
    pub privacy_requirement: PrivacyRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreatePrivacyRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "privacy-requirement", kind: "create-privacy-requirement", record: "CreatedPrivacyRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create privacy requirement \"{}\"", self.privacy_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.privacy_requirement.header.id.0.clone()]
    }
}
//#endregion 🔖️CreatePrivacyRequirement

//#region 🔖️DeletePrivacyRequirement
/// 🗑️ Removes a privacy requirement row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeletePrivacyRequirement {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeletePrivacyRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "privacy-requirement", kind: "delete-privacy-requirement", record: "DeletedPrivacyRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete privacy requirement \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeletePrivacyRequirement

//#region 🔖️RenamePrivacyRequirement
/// ✏️ Sets the identity `name` field of one privacy requirement row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenamePrivacyRequirement {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenamePrivacyRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "privacy-requirement", kind: "rename-privacy-requirement", record: "RenamedPrivacyRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename privacy requirement to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenamePrivacyRequirement

//#region 🔖️ReplacePrivacyRequirement
/// 🔁️ Whole-value swap of one privacy requirement row's non-identity content, addressed by
/// `privacy_requirement.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplacePrivacyRequirement {
    pub privacy_requirement: PrivacyRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplacePrivacyRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "privacy-requirement", kind: "replace-privacy-requirement", record: "ReplacedPrivacyRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace privacy requirement \"{}\"", self.privacy_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.privacy_requirement.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplacePrivacyRequirement
