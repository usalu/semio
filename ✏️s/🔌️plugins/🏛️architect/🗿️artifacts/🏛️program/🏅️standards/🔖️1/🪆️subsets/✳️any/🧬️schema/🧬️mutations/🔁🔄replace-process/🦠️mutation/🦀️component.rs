//! 🦠️ ProgramSnapshot mutation — `replace-process` leaf (replace). Split from the
//! pre-migration `🔄processes` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::Process;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🔁️ Whole-value swap of one process row's non-identity content, addressed by
/// `process.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceProcess {
    pub process: Process,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceProcess {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "process", kind: "replace-process", record: "ReplacedProcess" };
    async fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Replace process \"{}\"", self.process.header.name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.process.header.id.0.clone()]
    }
}
