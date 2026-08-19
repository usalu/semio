//! 🦠️ ProgramSnapshot mutation — `rename-project` leaf (rename). Split from the
//! pre-migration `📁update-project` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// ✏️ Sets `program.project.code`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameProject {
    pub new_code: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameProject {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "project", kind: "rename-project", record: "RenamedProject" };
    async fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Rename project to \"{}\"", self.new_code)
    }
}
