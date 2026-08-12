//! 🦠️ ProgramSnapshot mutation — `sustainability` leaf: create/delete/rename/replace sustainability requirement rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `SustainabilityRequirement` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::SustainabilityRequirement;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateSustainabilityRequirement
/// 🌱️ Brings a new sustainability requirement row into existence in `program.sustainability`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSustainabilityRequirement {
    pub sustainability_requirement: SustainabilityRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateSustainabilityRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "sustainability-requirement", kind: "create-sustainability-requirement", record: "CreatedSustainabilityRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create sustainability requirement \"{}\"", self.sustainability_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.sustainability_requirement.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateSustainabilityRequirement

//#region 🔖️DeleteSustainabilityRequirement
/// 🗑️ Removes a sustainability requirement row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSustainabilityRequirement {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteSustainabilityRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "sustainability-requirement", kind: "delete-sustainability-requirement", record: "DeletedSustainabilityRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete sustainability requirement \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteSustainabilityRequirement

//#region 🔖️RenameSustainabilityRequirement
/// ✏️ Sets the identity `name` field of one sustainability requirement row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameSustainabilityRequirement {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameSustainabilityRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "sustainability-requirement", kind: "rename-sustainability-requirement", record: "RenamedSustainabilityRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename sustainability requirement to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameSustainabilityRequirement

//#region 🔖️ReplaceSustainabilityRequirement
/// 🔁️ Whole-value swap of one sustainability requirement row's non-identity content, addressed by
/// `sustainability_requirement.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceSustainabilityRequirement {
    pub sustainability_requirement: SustainabilityRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceSustainabilityRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "sustainability-requirement", kind: "replace-sustainability-requirement", record: "ReplacedSustainabilityRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace sustainability requirement \"{}\"", self.sustainability_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.sustainability_requirement.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceSustainabilityRequirement
