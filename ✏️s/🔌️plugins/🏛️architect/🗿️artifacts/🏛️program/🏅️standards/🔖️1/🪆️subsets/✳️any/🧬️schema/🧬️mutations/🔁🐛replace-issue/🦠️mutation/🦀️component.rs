//! 🦠️ ProgramSnapshot mutation — `replace-issue` leaf (replace). Split from the
//! pre-migration `🐛issues` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::Issue;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🔁️ Whole-value swap of one issue row's non-identity content, addressed by
/// `issue.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceIssue {
    pub issue: Issue,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceIssue {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "issue", kind: "replace-issue", record: "ReplacedIssue" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace issue \"{}\"", self.issue.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.issue.header.id.0.clone()]
    }
}
