//! 🦠️ ProgramSnapshot mutation — `replace-assumption` leaf (replace). Split from the
//! pre-migration `💭assumptions` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::Assumption;
use crate::artifacts::program::{ProgramDiff, ProgramMutation, ProgramSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

/// 🔁️ Whole-value swap of one assumption row's non-identity content, addressed by
/// `assumption.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ReplaceAssumption {
    pub assumption: Assumption,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceAssumption {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "assumption", kind: "replace-assumption", record: "ReplacedAssumption" };
    async fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Replace assumption \"{}\"", self.assumption.header.name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.assumption.header.id.0.clone()]
    }
}
