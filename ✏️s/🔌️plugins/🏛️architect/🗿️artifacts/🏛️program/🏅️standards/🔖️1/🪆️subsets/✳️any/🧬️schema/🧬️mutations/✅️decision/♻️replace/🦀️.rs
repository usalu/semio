//! 🦠️ ProgramSnapshot mutation — `replace-decision` leaf (replace). Split from the
//! pre-migration `✅decisions` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::Decision;
use crate::artifacts::program::{ProgramDiff, ProgramMutation, ProgramSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

/// 🔁️ Whole-value swap of one decision row's non-identity content, addressed by
/// `decision.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ReplaceDecision {
    pub decision: Decision,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceDecision {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "decision", kind: "replace-decision", record: "ReplacedDecision" };
    async fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Replace decision \"{}\"", self.decision.header.name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.decision.header.id.0.clone()]
    }
}
