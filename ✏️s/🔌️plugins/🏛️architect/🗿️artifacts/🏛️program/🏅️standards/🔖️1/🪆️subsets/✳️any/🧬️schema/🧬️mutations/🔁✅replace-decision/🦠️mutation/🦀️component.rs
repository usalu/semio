//! 🦠️ ProgramSnapshot mutation — `replace-decision` leaf (replace). Split from the
//! pre-migration `✅decisions` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::Decision;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

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
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace decision \"{}\"", self.decision.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.decision.header.id.0.clone()]
    }
}
