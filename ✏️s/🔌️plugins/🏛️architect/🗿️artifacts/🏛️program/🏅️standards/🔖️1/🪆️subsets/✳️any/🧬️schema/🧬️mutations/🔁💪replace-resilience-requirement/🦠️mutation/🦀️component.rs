//! 🦠️ ProgramSnapshot mutation — `replace-resilience-requirement` leaf (replace). Split from the
//! pre-migration `💪resilience` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::ResilienceRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🔁️ Whole-value swap of one resilience requirement row's non-identity content, addressed by
/// `resilience_requirement.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceResilienceRequirement {
    pub resilience_requirement: ResilienceRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceResilienceRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "resilience-requirement", kind: "replace-resilience-requirement", record: "ReplacedResilienceRequirement" };
    async fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Replace resilience requirement \"{}\"", self.resilience_requirement.header.name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.resilience_requirement.header.id.0.clone()]
    }
}
