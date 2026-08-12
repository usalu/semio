//! 🦠️ ProgramSnapshot mutation — `schedules` leaf: create/delete/rename/replace schedule requirement rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `ScheduleRequirement` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::ScheduleRequirement;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateScheduleRequirement
/// 🌱️ Brings a new schedule requirement row into existence in `program.schedules`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateScheduleRequirement {
    pub schedule_requirement: ScheduleRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateScheduleRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "schedule-requirement", kind: "create-schedule-requirement", record: "CreatedScheduleRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create schedule requirement \"{}\"", self.schedule_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.schedule_requirement.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateScheduleRequirement

//#region 🔖️DeleteScheduleRequirement
/// 🗑️ Removes a schedule requirement row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteScheduleRequirement {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteScheduleRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "schedule-requirement", kind: "delete-schedule-requirement", record: "DeletedScheduleRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete schedule requirement \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteScheduleRequirement

//#region 🔖️RenameScheduleRequirement
/// ✏️ Sets the identity `name` field of one schedule requirement row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameScheduleRequirement {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameScheduleRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "schedule-requirement", kind: "rename-schedule-requirement", record: "RenamedScheduleRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename schedule requirement to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameScheduleRequirement

//#region 🔖️ReplaceScheduleRequirement
/// 🔁️ Whole-value swap of one schedule requirement row's non-identity content, addressed by
/// `schedule_requirement.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceScheduleRequirement {
    pub schedule_requirement: ScheduleRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceScheduleRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "schedule-requirement", kind: "replace-schedule-requirement", record: "ReplacedScheduleRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace schedule requirement \"{}\"", self.schedule_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.schedule_requirement.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceScheduleRequirement
