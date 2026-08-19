//! 🦠️ ProgramSnapshot mutation — `rename-service-requirement` leaf (rename). Split from the
//! pre-migration `🛎️services` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// ✏️ Sets the identity `name` field of one service requirement row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameServiceRequirement {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameServiceRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "service-requirement", kind: "rename-service-requirement", record: "RenamedServiceRequirement" };
    async fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Rename service requirement to \"{}\"", self.new_name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
