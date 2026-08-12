//! 🦠️ ProgramSnapshot mutation — `flexibility` leaf: create/delete/rename/replace flexibility requirement rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `FlexibilityRequirement` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::FlexibilityRequirement;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateFlexibilityRequirement
/// 🌱️ Brings a new flexibility requirement row into existence in `program.flexibility`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFlexibilityRequirement {
    pub flexibility_requirement: FlexibilityRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateFlexibilityRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "flexibility-requirement", kind: "create-flexibility-requirement", record: "CreatedFlexibilityRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create flexibility requirement \"{}\"", self.flexibility_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.flexibility_requirement.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateFlexibilityRequirement

//#region 🔖️DeleteFlexibilityRequirement
/// 🗑️ Removes a flexibility requirement row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteFlexibilityRequirement {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteFlexibilityRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "flexibility-requirement", kind: "delete-flexibility-requirement", record: "DeletedFlexibilityRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete flexibility requirement \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteFlexibilityRequirement

//#region 🔖️RenameFlexibilityRequirement
/// ✏️ Sets the identity `name` field of one flexibility requirement row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameFlexibilityRequirement {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameFlexibilityRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "flexibility-requirement", kind: "rename-flexibility-requirement", record: "RenamedFlexibilityRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename flexibility requirement to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameFlexibilityRequirement

//#region 🔖️ReplaceFlexibilityRequirement
/// 🔁️ Whole-value swap of one flexibility requirement row's non-identity content, addressed by
/// `flexibility_requirement.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceFlexibilityRequirement {
    pub flexibility_requirement: FlexibilityRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceFlexibilityRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "flexibility-requirement", kind: "replace-flexibility-requirement", record: "ReplacedFlexibilityRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace flexibility requirement \"{}\"", self.flexibility_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.flexibility_requirement.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceFlexibilityRequirement
