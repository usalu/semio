//! 🦠️ ProgramSnapshot mutation — `decisions` leaf: create/delete/rename/replace decision rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `Decision` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::Decision;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateDecision
/// 🌱️ Brings a new decision row into existence in `program.decisions`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDecision {
    pub decision: Decision,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateDecision {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "decision", kind: "create-decision", record: "CreatedDecision" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create decision \"{}\"", self.decision.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.decision.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateDecision

//#region 🔖️DeleteDecision
/// 🗑️ Removes a decision row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteDecision {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteDecision {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "decision", kind: "delete-decision", record: "DeletedDecision" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete decision \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteDecision

//#region 🔖️RenameDecision
/// ✏️ Sets the identity `name` field of one decision row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameDecision {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameDecision {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "decision", kind: "rename-decision", record: "RenamedDecision" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename decision to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameDecision

//#region 🔖️ReplaceDecision
/// 🔁️ Whole-value swap of one decision row's non-identity content, addressed by
/// `decision.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceDecision {
    pub decision: Decision,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceDecision {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "decision", kind: "replace-decision", record: "ReplacedDecision" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace decision \"{}\"", self.decision.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.decision.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceDecision
