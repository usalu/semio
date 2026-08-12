//! 🦠️ ProgramSnapshot mutation — `surveys` leaf: create/delete/rename/replace survey rows.
//! Semantic vocabulary derived from `🗄️registers/🦀️component.rs`'s `Survey` shape per
//! `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` rule 2
//! (per id-keyed collection): create/delete the row, rename its identity field, replace its
//! remaining content as one sparse patch. `diff`/`inverse` delegate to the sibling `🔺️diff`/
//! `↩️inverse` leaves — never hand-computed here.

use crate::artifacts::program::registers::Survey;
use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️CreateSurvey
/// 🌱️ Brings a new survey row into existence in `program.surveys`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSurvey {
    pub survey: Survey,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateSurvey {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "survey", kind: "create-survey", record: "CreatedSurvey" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_create(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_create(self, base)
    }
    fn label(&self) -> String {
        format!("Create survey \"{}\"", self.survey.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.survey.header.id.0.clone()]
    }
}
//#endregion 🔖️CreateSurvey

//#region 🔖️DeleteSurvey
/// 🗑️ Removes a survey row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSurvey {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteSurvey {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "survey", kind: "delete-survey", record: "DeletedSurvey" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_delete(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_delete(self, base)
    }
    fn label(&self) -> String {
        format!("Delete survey \"{}\"", self.id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️DeleteSurvey

//#region 🔖️RenameSurvey
/// ✏️ Sets the identity `name` field of one survey row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameSurvey {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameSurvey {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "survey", kind: "rename-survey", record: "RenamedSurvey" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename survey to \"{}\"", self.new_name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
//#endregion 🔖️RenameSurvey

//#region 🔖️ReplaceSurvey
/// 🔁️ Whole-value swap of one survey row's non-identity content, addressed by
/// `survey.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceSurvey {
    pub survey: Survey,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceSurvey {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "survey", kind: "replace-survey", record: "ReplacedSurvey" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace survey \"{}\"", self.survey.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.survey.header.id.0.clone()]
    }
}
//#endregion 🔖️ReplaceSurvey
