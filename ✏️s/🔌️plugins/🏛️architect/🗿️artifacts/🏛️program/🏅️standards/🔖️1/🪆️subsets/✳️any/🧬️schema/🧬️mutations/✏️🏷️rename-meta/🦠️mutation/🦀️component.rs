//! 🦠️ ProgramSnapshot mutation — `rename-meta` leaf (rename). Split from the
//! pre-migration `🏷️update-meta` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// ✏️ Sets `program.meta.title`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameMeta {
    pub new_title: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameMeta {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "meta", kind: "rename-meta", record: "RenamedMeta" };
    fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Rename document to \"{}\"", self.new_title)
    }
}
