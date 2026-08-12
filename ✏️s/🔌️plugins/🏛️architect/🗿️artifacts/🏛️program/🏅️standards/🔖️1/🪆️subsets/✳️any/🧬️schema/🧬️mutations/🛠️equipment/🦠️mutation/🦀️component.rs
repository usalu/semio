//! 🦠️ ProgramSnapshot mutation — `equipment` leaf: create/delete/rename/replace equipment rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `Equipment` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::Equipment;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateEquipment
/// 🌱️ Brings a new equipment row into existence in `program.equipment`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEquipment {
    pub equipment: Equipment,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateEquipment {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "equipment", kind: "create-equipment", record: "CreatedEquipment" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create equipment \"{}\"", self.equipment.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.equipment.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateEquipment

//#region 🔖️DeleteEquipment
/// 🗑️ Removes a equipment row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteEquipment {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteEquipment {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "equipment", kind: "delete-equipment", record: "DeletedEquipment" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete equipment \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteEquipment

//#region 🔖️RenameEquipment
/// ✏️ Sets the identity `name` field of one equipment row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameEquipment {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameEquipment {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "equipment", kind: "rename-equipment", record: "RenamedEquipment" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename equipment to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameEquipment

//#region 🔖️ReplaceEquipment
/// 🔁️ Whole-value swap of one equipment row's non-identity content, addressed by
/// `equipment.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceEquipment {
    pub equipment: Equipment,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceEquipment {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "equipment", kind: "replace-equipment", record: "ReplacedEquipment" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace equipment \"{}\"", self.equipment.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.equipment.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceEquipment
