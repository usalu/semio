//! 🦠️ ProgramSnapshot mutation — `replace-stakeholder` leaf (replace). Split from the
//! pre-migration `👥stakeholders` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::Stakeholder;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🔁️ Whole-value swap of one stakeholder row's non-identity content, addressed by
/// `stakeholder.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceStakeholder {
    pub stakeholder: Stakeholder,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceStakeholder {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "stakeholder", kind: "replace-stakeholder", record: "ReplacedStakeholder" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace stakeholder \"{}\"", self.stakeholder.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.stakeholder.header.id.0.clone()]
    }
}
