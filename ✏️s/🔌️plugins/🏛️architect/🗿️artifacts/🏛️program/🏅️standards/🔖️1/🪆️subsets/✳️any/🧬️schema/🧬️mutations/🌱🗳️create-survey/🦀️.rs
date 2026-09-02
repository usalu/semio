//! 🦠️ ProgramSnapshot mutation — `create-survey` leaf (create). Split from the
//! pre-migration `🗳️surveys` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::Survey;
use crate::artifacts::program::{ProgramDiff, ProgramMutation, ProgramSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

/// 🌱️ Brings a new survey row into existence in `program.surveys`.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct CreateSurvey {
    pub survey: Survey,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateSurvey {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "survey", kind: "create-survey", record: "CreatedSurvey" };
    async fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Create survey \"{}\"", self.survey.header.name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.survey.header.id.0.clone()]
    }
}
