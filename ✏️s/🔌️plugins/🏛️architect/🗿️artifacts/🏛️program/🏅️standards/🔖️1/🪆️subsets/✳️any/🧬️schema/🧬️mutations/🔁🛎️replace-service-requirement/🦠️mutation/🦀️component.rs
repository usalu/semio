//! 🦠️ ProgramSnapshot mutation — `replace-service-requirement` leaf (replace). Split from the
//! pre-migration `🛎️services` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::ServiceRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🔁️ Whole-value swap of one service requirement row's non-identity content, addressed by
/// `service_requirement.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceServiceRequirement {
    pub service_requirement: ServiceRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceServiceRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "service-requirement", kind: "replace-service-requirement", record: "ReplacedServiceRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace service requirement \"{}\"", self.service_requirement.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.service_requirement.header.id.0.clone()]
    }
}
