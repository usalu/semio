//! 🦠️ ProgramSnapshot mutation — `quantities` leaf: create/delete/rename/replace quantity requirement rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `QuantityRequirement` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::QuantityRequirement;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateQuantityRequirement
/// 🌱️ Brings a new quantity requirement row into existence in `program.quantities`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateQuantityRequirement {
    pub quantity_requirement: QuantityRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateQuantityRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "quantity-requirement", kind: "create-quantity-requirement", record: "CreatedQuantityRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create quantity requirement \"{}\"", self.quantity_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.quantity_requirement.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateQuantityRequirement

//#region 🔖️DeleteQuantityRequirement
/// 🗑️ Removes a quantity requirement row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteQuantityRequirement {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteQuantityRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "quantity-requirement", kind: "delete-quantity-requirement", record: "DeletedQuantityRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete quantity requirement \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteQuantityRequirement

//#region 🔖️RenameQuantityRequirement
/// ✏️ Sets the identity `name` field of one quantity requirement row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameQuantityRequirement {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameQuantityRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "quantity-requirement", kind: "rename-quantity-requirement", record: "RenamedQuantityRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename quantity requirement to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameQuantityRequirement

//#region 🔖️ReplaceQuantityRequirement
/// 🔁️ Whole-value swap of one quantity requirement row's non-identity content, addressed by
/// `quantity_requirement.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceQuantityRequirement {
    pub quantity_requirement: QuantityRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceQuantityRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "quantity-requirement", kind: "replace-quantity-requirement", record: "ReplacedQuantityRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace quantity requirement \"{}\"", self.quantity_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.quantity_requirement.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceQuantityRequirement
