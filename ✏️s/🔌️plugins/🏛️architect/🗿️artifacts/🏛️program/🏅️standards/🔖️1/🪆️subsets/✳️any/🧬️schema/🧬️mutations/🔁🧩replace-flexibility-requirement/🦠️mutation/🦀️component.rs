//! 🦠️ ProgramSnapshot mutation — `replace-flexibility-requirement` leaf (replace). Split from the
//! pre-migration `🧩flexibility` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::FlexibilityRequirement;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🔁️ Whole-value swap of one flexibility requirement row's non-identity content, addressed by
/// `flexibility_requirement.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceFlexibilityRequirement {
    pub flexibility_requirement: FlexibilityRequirement,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceFlexibilityRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "flexibility-requirement", kind: "replace-flexibility-requirement", record: "ReplacedFlexibilityRequirement" };
    async fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Replace flexibility requirement \"{}\"", self.flexibility_requirement.header.name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.flexibility_requirement.header.id.0.clone()]
    }
}
