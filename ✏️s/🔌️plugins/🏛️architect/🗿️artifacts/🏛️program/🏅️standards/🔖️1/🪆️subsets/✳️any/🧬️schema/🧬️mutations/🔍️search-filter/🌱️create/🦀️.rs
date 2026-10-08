//! 🦠️ ProgramSnapshot mutation — `create-search-filter` leaf (create). Split from the
//! pre-migration `🔍search-filters` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::registers::SearchFilter;
use crate::{ProgramDiff, ProgramMutation, ProgramSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

/// 🌱️ Brings a new search filter row into existence in `program.search_filters`.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct CreateSearchFilter {
    pub search_filter: SearchFilter,
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    pub index: Option<usize>,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateSearchFilter {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "search-filter", kind: "create-search-filter", record: "CreatedSearchFilter" };
    fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create search filter \"{}\"", self.search_filter.header.name), &format!("Suchfilter \"{}\" erstellen", self.search_filter.header.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.search_filter.header.id.0.clone()]
    }
}
