//! 🦠️ ProgramSnapshot mutation — `delete-issue` leaf (delete). Split from the
//! pre-migration `🐛issues` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🗑️ Removes a issue row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteIssue {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteIssue {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "issue", kind: "delete-issue", record: "DeletedIssue" };
    async fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Delete issue \"{}\"", self.id.0)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
