//! 🦠️ ProgramSnapshot mutation — `risks` leaf: create/delete/rename/replace risk rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `Risk` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::Risk;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateRisk
/// 🌱️ Brings a new risk row into existence in `program.risks`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRisk {
    pub risk: Risk,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateRisk {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "risk", kind: "create-risk", record: "CreatedRisk" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create risk \"{}\"", self.risk.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.risk.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateRisk

//#region 🔖️DeleteRisk
/// 🗑️ Removes a risk row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteRisk {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteRisk {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "risk", kind: "delete-risk", record: "DeletedRisk" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete risk \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteRisk

//#region 🔖️RenameRisk
/// ✏️ Sets the identity `name` field of one risk row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameRisk {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameRisk {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "risk", kind: "rename-risk", record: "RenamedRisk" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename risk to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameRisk

//#region 🔖️ReplaceRisk
/// 🔁️ Whole-value swap of one risk row's non-identity content, addressed by
/// `risk.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceRisk {
    pub risk: Risk,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceRisk {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "risk", kind: "replace-risk", record: "ReplacedRisk" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace risk \"{}\"", self.risk.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.risk.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceRisk
