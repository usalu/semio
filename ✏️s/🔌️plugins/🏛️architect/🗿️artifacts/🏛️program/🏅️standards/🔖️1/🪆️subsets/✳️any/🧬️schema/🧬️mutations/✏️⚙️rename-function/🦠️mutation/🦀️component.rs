//! 🦠️ ProgramSnapshot mutation — `rename-function` leaf (rename). Split from the
//! pre-migration `⚙️functions` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::kernel::EntityId;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// ✏️ Sets the identity `name` field of one function row, addressed by id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameFunction {
    pub id: EntityId,
    pub new_name: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameFunction {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "function", kind: "rename-function", record: "RenamedFunction" };
    async fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Rename function to \"{}\"", self.new_name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
