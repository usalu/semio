//! 🦠️ ProgramSnapshot mutation — `create-search-filter` leaf (create). Split from the
//! pre-migration `🔍search-filters` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::artifacts::program::registers::SearchFilter;
use crate::artifacts::program::{ProgramDiff, ProgramMutation, ProgramSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

/// 🌱️ Brings a new search filter row into existence in `program.search_filters`.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct CreateSearchFilter {
    pub search_filter: SearchFilter,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateSearchFilter {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "search-filter", kind: "create-search-filter", record: "CreatedSearchFilter" };
    async fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Create search filter \"{}\"", self.search_filter.header.name)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.search_filter.header.id.0.clone()]
    }
}
