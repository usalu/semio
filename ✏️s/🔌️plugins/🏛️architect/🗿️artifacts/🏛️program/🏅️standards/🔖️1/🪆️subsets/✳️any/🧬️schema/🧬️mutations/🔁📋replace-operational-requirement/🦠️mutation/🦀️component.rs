//! 🦠️ ProgramSnapshot mutation — `replace-operational-requirement` leaf (replace). Split from the
//! pre-migration `📋operations` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::OperationalRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🔁️ Whole-value swap of one operational requirement row's non-identity content, addressed by
/// `operational_requirement.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceOperationalRequirement {
    pub operational_requirement: OperationalRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceOperationalRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "operational-requirement", kind: "replace-operational-requirement", record: "ReplacedOperationalRequirement" };
    async fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Replace operational requirement \"{}\"", self.operational_requirement.header.name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.operational_requirement.header.id.0.clone()]
    }
}
