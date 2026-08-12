//! 🦠️ ProgramSnapshot mutation — `storage` leaf: create/delete/rename/replace storage requirement rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `StorageRequirement` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::StorageRequirement;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateStorageRequirement
/// 🌱️ Brings a new storage requirement row into existence in `program.storage`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateStorageRequirement {
    pub storage_requirement: StorageRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateStorageRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "storage-requirement", kind: "create-storage-requirement", record: "CreatedStorageRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create storage requirement \"{}\"", self.storage_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.storage_requirement.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateStorageRequirement

//#region 🔖️DeleteStorageRequirement
/// 🗑️ Removes a storage requirement row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteStorageRequirement {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteStorageRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "storage-requirement", kind: "delete-storage-requirement", record: "DeletedStorageRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete storage requirement \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteStorageRequirement

//#region 🔖️RenameStorageRequirement
/// ✏️ Sets the identity `name` field of one storage requirement row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameStorageRequirement {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameStorageRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "storage-requirement", kind: "rename-storage-requirement", record: "RenamedStorageRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename storage requirement to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameStorageRequirement

//#region 🔖️ReplaceStorageRequirement
/// 🔁️ Whole-value swap of one storage requirement row's non-identity content, addressed by
/// `storage_requirement.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceStorageRequirement {
    pub storage_requirement: StorageRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceStorageRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "storage-requirement", kind: "replace-storage-requirement", record: "ReplacedStorageRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace storage requirement \"{}\"", self.storage_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.storage_requirement.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceStorageRequirement
