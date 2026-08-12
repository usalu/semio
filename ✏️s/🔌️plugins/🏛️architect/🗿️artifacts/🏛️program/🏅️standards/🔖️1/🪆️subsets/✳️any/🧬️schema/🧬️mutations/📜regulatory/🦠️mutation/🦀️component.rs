//! 🦠️ ProgramSnapshot mutation — `regulatory` leaf: create/delete/rename/replace regulatory requirement rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `RegulatoryRequirement` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::RegulatoryRequirement;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateRegulatoryRequirement
/// 🌱️ Brings a new regulatory requirement row into existence in `program.regulatory`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRegulatoryRequirement {
    pub regulatory_requirement: RegulatoryRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateRegulatoryRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "regulatory-requirement", kind: "create-regulatory-requirement", record: "CreatedRegulatoryRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create regulatory requirement \"{}\"", self.regulatory_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.regulatory_requirement.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateRegulatoryRequirement

//#region 🔖️DeleteRegulatoryRequirement
/// 🗑️ Removes a regulatory requirement row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteRegulatoryRequirement {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteRegulatoryRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "regulatory-requirement", kind: "delete-regulatory-requirement", record: "DeletedRegulatoryRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete regulatory requirement \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteRegulatoryRequirement

//#region 🔖️RenameRegulatoryRequirement
/// ✏️ Sets the identity `name` field of one regulatory requirement row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameRegulatoryRequirement {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameRegulatoryRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "regulatory-requirement", kind: "rename-regulatory-requirement", record: "RenamedRegulatoryRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename regulatory requirement to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameRegulatoryRequirement

//#region 🔖️ReplaceRegulatoryRequirement
/// 🔁️ Whole-value swap of one regulatory requirement row's non-identity content, addressed by
/// `regulatory_requirement.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceRegulatoryRequirement {
    pub regulatory_requirement: RegulatoryRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceRegulatoryRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "regulatory-requirement", kind: "replace-regulatory-requirement", record: "ReplacedRegulatoryRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace regulatory requirement \"{}\"", self.regulatory_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.regulatory_requirement.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceRegulatoryRequirement
