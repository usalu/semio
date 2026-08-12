//! 🦠️ ProgramSnapshot mutation — `activities` leaf: create/delete/rename/replace activity rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `Activity` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::Activity;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateActivity
/// 🌱️ Brings a new activity row into existence in `program.activities`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateActivity {
    pub activity: Activity,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateActivity {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "activity", kind: "create-activity", record: "CreatedActivity" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create activity \"{}\"", self.activity.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.activity.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateActivity

//#region 🔖️DeleteActivity
/// 🗑️ Removes a activity row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteActivity {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteActivity {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "activity", kind: "delete-activity", record: "DeletedActivity" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete activity \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteActivity

//#region 🔖️RenameActivity
/// ✏️ Sets the identity `name` field of one activity row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameActivity {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameActivity {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "activity", kind: "rename-activity", record: "RenamedActivity" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename activity to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameActivity

//#region 🔖️ReplaceActivity
/// 🔁️ Whole-value swap of one activity row's non-identity content, addressed by
/// `activity.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceActivity {
    pub activity: Activity,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceActivity {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "activity", kind: "replace-activity", record: "ReplacedActivity" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace activity \"{}\"", self.activity.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.activity.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceActivity
