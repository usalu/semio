//! 🦠️ ProgramSnapshot mutation — `delivery` leaf: create/delete/rename/replace delivery constraint rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `DeliveryConstraint` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::DeliveryConstraint;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateDeliveryConstraint
/// 🌱️ Brings a new delivery constraint row into existence in `program.delivery`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDeliveryConstraint {
    pub delivery_constraint: DeliveryConstraint,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateDeliveryConstraint {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "delivery-constraint", kind: "create-delivery-constraint", record: "CreatedDeliveryConstraint" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create delivery constraint \"{}\"", self.delivery_constraint.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.delivery_constraint.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateDeliveryConstraint

//#region 🔖️DeleteDeliveryConstraint
/// 🗑️ Removes a delivery constraint row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteDeliveryConstraint {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteDeliveryConstraint {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "delivery-constraint", kind: "delete-delivery-constraint", record: "DeletedDeliveryConstraint" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete delivery constraint \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteDeliveryConstraint

//#region 🔖️RenameDeliveryConstraint
/// ✏️ Sets the identity `name` field of one delivery constraint row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameDeliveryConstraint {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameDeliveryConstraint {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "delivery-constraint", kind: "rename-delivery-constraint", record: "RenamedDeliveryConstraint" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename delivery constraint to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameDeliveryConstraint

//#region 🔖️ReplaceDeliveryConstraint
/// 🔁️ Whole-value swap of one delivery constraint row's non-identity content, addressed by
/// `delivery_constraint.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceDeliveryConstraint {
    pub delivery_constraint: DeliveryConstraint,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceDeliveryConstraint {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "delivery-constraint", kind: "replace-delivery-constraint", record: "ReplacedDeliveryConstraint" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace delivery constraint \"{}\"", self.delivery_constraint.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.delivery_constraint.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceDeliveryConstraint
