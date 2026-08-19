//! 🦠️ ProgramSnapshot mutation — `create-scenario` leaf (create). Split from the
//! pre-migration `🎬scenarios` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::Scenario;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🌱️ Brings a new scenario row into existence in `program.scenarios`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateScenario {
    pub scenario: Scenario,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateScenario {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "scenario", kind: "create-scenario", record: "CreatedScenario" };
    async fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Create scenario \"{}\"", self.scenario.header.name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.scenario.header.id.0.clone()]
    }
}
