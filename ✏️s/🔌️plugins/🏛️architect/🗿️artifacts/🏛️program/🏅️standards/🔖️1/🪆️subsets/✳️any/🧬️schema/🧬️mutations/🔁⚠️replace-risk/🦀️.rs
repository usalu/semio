//! 🦠️ ProgramSnapshot mutation — `replace-risk` leaf (replace). Split from the
//! pre-migration `⚠️risks` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::Risk;
use crate::artifacts::program::{ProgramDiff, ProgramMutation, ProgramSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🔁️ Whole-value swap of one risk row's non-identity content, addressed by
/// `risk.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceRisk {
    pub risk: Risk,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceRisk {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "risk", kind: "replace-risk", record: "ReplacedRisk" };
    async fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Replace risk \"{}\"", self.risk.header.name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.risk.header.id.0.clone()]
    }
}
