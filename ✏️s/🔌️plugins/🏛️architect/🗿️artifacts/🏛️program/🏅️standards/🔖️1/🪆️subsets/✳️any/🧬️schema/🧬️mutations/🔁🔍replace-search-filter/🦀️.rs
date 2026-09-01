//! 🦠️ ProgramSnapshot mutation — `replace-search-filter` leaf (replace). Split from the
//! pre-migration `🔍search-filters` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::SearchFilter;
use crate::artifacts::program::{ProgramDiff, ProgramMutation, ProgramSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

/// 🔁️ Whole-value swap of one search filter row's non-identity content, addressed by
/// `search_filter.header.id`. Missing target ⇒ an empty diff (nothing to change).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceSearchFilter {
    pub search_filter: SearchFilter,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceSearchFilter {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "search-filter", kind: "replace-search-filter", record: "ReplacedSearchFilter" };
    async fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Replace search filter \"{}\"", self.search_filter.header.name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.search_filter.header.id.0.clone()]
    }
}
