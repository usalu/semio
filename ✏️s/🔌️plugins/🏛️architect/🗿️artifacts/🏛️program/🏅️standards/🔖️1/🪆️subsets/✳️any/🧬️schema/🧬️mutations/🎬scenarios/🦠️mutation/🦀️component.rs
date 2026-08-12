//! 🦠️ ProgramSnapshot mutation — `scenarios` leaf: create/delete/rename/replace scenario rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `Scenario` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::Scenario;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateScenario
/// 🌱️ Brings a new scenario row into existence in `program.scenarios`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateScenario {
    pub scenario: Scenario,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateScenario {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "scenario", kind: "create-scenario", record: "CreatedScenario" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create scenario \"{}\"", self.scenario.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.scenario.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateScenario

//#region 🔖️DeleteScenario
/// 🗑️ Removes a scenario row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteScenario {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteScenario {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "scenario", kind: "delete-scenario", record: "DeletedScenario" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete scenario \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteScenario

//#region 🔖️RenameScenario
/// ✏️ Sets the identity `name` field of one scenario row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameScenario {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameScenario {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "scenario", kind: "rename-scenario", record: "RenamedScenario" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename scenario to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameScenario

//#region 🔖️ReplaceScenario
/// 🔁️ Whole-value swap of one scenario row's non-identity content, addressed by
/// `scenario.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceScenario {
    pub scenario: Scenario,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceScenario {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "scenario", kind: "replace-scenario", record: "ReplacedScenario" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace scenario \"{}\"", self.scenario.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.scenario.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceScenario
