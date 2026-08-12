//! 🦠️ ProgramSnapshot mutation — `stakeholders` leaf: create/delete/rename/replace stakeholder rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `Stakeholder` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::Stakeholder;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateStakeholder
/// 🌱️ Brings a new stakeholder row into existence in `program.stakeholders`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateStakeholder {
    pub stakeholder: Stakeholder,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateStakeholder {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "stakeholder", kind: "create-stakeholder", record: "CreatedStakeholder" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create stakeholder \"{}\"", self.stakeholder.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.stakeholder.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateStakeholder

//#region 🔖️DeleteStakeholder
/// 🗑️ Removes a stakeholder row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteStakeholder {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteStakeholder {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "stakeholder", kind: "delete-stakeholder", record: "DeletedStakeholder" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete stakeholder \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteStakeholder

//#region 🔖️RenameStakeholder
/// ✏️ Sets the identity `name` field of one stakeholder row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameStakeholder {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameStakeholder {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "stakeholder", kind: "rename-stakeholder", record: "RenamedStakeholder" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename stakeholder to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameStakeholder

//#region 🔖️ReplaceStakeholder
/// 🔁️ Whole-value swap of one stakeholder row's non-identity content, addressed by
/// `stakeholder.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceStakeholder {
    pub stakeholder: Stakeholder,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceStakeholder {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "stakeholder", kind: "replace-stakeholder", record: "ReplacedStakeholder" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace stakeholder \"{}\"", self.stakeholder.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.stakeholder.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceStakeholder
