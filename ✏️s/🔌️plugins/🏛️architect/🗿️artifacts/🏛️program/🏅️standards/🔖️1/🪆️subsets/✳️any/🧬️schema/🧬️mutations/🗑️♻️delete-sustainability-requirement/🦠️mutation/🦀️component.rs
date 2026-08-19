//! 🦠️ ProgramSnapshot mutation — `delete-sustainability-requirement` leaf (delete). Split from the
//! pre-migration `♻️sustainability` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🗑️ Removes a sustainability requirement row by id (captures the removed row for undo via `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSustainabilityRequirement {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteSustainabilityRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "sustainability-requirement", kind: "delete-sustainability-requirement", record: "DeletedSustainabilityRequirement" };
    async fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Delete sustainability requirement \"{}\"", self.id.0)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
