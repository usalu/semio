//! 🦠️ ProgramSnapshot mutation — `create-issue` leaf (create). Split from the
//! pre-migration `🐛issues` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::Issue;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🌱️ Brings a new issue row into existence in `program.issues`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateIssue {
    pub issue: Issue,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateIssue {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "issue", kind: "create-issue", record: "CreatedIssue" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create issue \"{}\"", self.issue.header.name)
    }
    fn target(&self) -> Vec<String> {
        vec![self.issue.header.id.0.clone()]
    }
}
