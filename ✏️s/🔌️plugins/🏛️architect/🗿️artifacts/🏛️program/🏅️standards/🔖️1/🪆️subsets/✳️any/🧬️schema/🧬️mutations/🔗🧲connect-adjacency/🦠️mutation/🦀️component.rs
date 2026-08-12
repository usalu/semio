//! 🦠️ ProgramSnapshot mutation — `connect-adjacency` leaf (connect). Split from the
//! pre-migration `🗺️set-adjacency` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::registers::Adjacency;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🔌️ Upserts an adjacency edge between two elements: normalizes the endpoint pair, replaces the
/// existing edge for that pair if present (keeping its id), otherwise adds a new edge.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectAdjacency {
    pub adjacency: Adjacency,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ConnectAdjacency {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "connect", entity: "adjacency", kind: "connect-adjacency", record: "ConnectedAdjacency" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Connect adjacency between \"{}\" and \"{}\"", self.adjacency.element_a_id.0, self.adjacency.element_b_id.0)
    }
    fn target(&self) -> Vec<String> {
        vec![self.adjacency.header.id.0.clone()]
    }
}
