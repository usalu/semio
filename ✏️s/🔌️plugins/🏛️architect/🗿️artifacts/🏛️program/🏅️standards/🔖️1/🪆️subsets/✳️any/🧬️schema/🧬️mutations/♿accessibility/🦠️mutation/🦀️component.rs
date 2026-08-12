//! 🦠️ ProgramSnapshot mutation — `accessibility` leaf: create/delete/rename/replace accessibility requirement rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `AccessibilityRequirement` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::AccessibilityRequirement;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateAccessibilityRequirement
/// 🌱️ Brings a new accessibility requirement row into existence in `program.accessibility`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAccessibilityRequirement {
    pub accessibility_requirement: AccessibilityRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateAccessibilityRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "accessibility-requirement", kind: "create-accessibility-requirement", record: "CreatedAccessibilityRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create accessibility requirement \"{}\"", self.accessibility_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.accessibility_requirement.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateAccessibilityRequirement

//#region 🔖️DeleteAccessibilityRequirement
/// 🗑️ Removes a accessibility requirement row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteAccessibilityRequirement {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteAccessibilityRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "accessibility-requirement", kind: "delete-accessibility-requirement", record: "DeletedAccessibilityRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete accessibility requirement \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteAccessibilityRequirement

//#region 🔖️RenameAccessibilityRequirement
/// ✏️ Sets the identity `name` field of one accessibility requirement row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameAccessibilityRequirement {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameAccessibilityRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "accessibility-requirement", kind: "rename-accessibility-requirement", record: "RenamedAccessibilityRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename accessibility requirement to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameAccessibilityRequirement

//#region 🔖️ReplaceAccessibilityRequirement
/// 🔁️ Whole-value swap of one accessibility requirement row's non-identity content, addressed by
/// `accessibility_requirement.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceAccessibilityRequirement {
    pub accessibility_requirement: AccessibilityRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceAccessibilityRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "accessibility-requirement", kind: "replace-accessibility-requirement", record: "ReplacedAccessibilityRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace accessibility requirement \"{}\"", self.accessibility_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.accessibility_requirement.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceAccessibilityRequirement
