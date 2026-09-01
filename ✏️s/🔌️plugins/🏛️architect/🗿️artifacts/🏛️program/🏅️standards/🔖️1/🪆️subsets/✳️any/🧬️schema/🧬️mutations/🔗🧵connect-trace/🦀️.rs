//! 🦠️ ProgramSnapshot mutation — `connect-trace` leaf (connect). Split from the
//! pre-migration `🧵traces` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::kernel::TraceLink;
use crate::artifacts::program::{ProgramDiff, ProgramMutation, ProgramSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🔌️ Upserts a trace edge by its own id: adds it if new, replaces its full content if present.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ConnectTrace {
    pub trace: TraceLink,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ConnectTrace {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "connect", entity: "trace", kind: "connect-trace", record: "ConnectedTrace" };
    async fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Connect trace \"{}\" -> \"{}\"", self.trace.from_id.0, self.trace.to_id.0)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.trace.id.0.clone()]
    }
}
