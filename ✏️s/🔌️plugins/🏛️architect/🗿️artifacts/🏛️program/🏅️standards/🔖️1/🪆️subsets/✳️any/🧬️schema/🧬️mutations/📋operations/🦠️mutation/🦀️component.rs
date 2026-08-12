//! 🦠️ ProgramSnapshot mutation — `operations` leaf: create/delete/rename/replace operational requirement rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `OperationalRequirement` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::OperationalRequirement;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateOperationalRequirement
/// 🌱️ Brings a new operational requirement row into existence in `program.operations`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOperationalRequirement {
    pub operational_requirement: OperationalRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateOperationalRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "operational-requirement", kind: "create-operational-requirement", record: "CreatedOperationalRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create operational requirement \"{}\"", self.operational_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.operational_requirement.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateOperationalRequirement

//#region 🔖️DeleteOperationalRequirement
/// 🗑️ Removes a operational requirement row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteOperationalRequirement {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteOperationalRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "operational-requirement", kind: "delete-operational-requirement", record: "DeletedOperationalRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete operational requirement \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteOperationalRequirement

//#region 🔖️RenameOperationalRequirement
/// ✏️ Sets the identity `name` field of one operational requirement row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameOperationalRequirement {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameOperationalRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "operational-requirement", kind: "rename-operational-requirement", record: "RenamedOperationalRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename operational requirement to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameOperationalRequirement

//#region 🔖️ReplaceOperationalRequirement
/// 🔁️ Whole-value swap of one operational requirement row's non-identity content, addressed by
/// `operational_requirement.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceOperationalRequirement {
    pub operational_requirement: OperationalRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceOperationalRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "operational-requirement", kind: "replace-operational-requirement", record: "ReplacedOperationalRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace operational requirement \"{}\"", self.operational_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.operational_requirement.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceOperationalRequirement
