//! 🔺️ Sparse diff construction for the `create-search-filter` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔍search-filters` per Wave C.

use super::mutation::CreateSearchFilter;
use crate::artifacts::program::diff::ProgramSearchFiltersDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub async fn diff(payload: &CreateSearchFilter, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.search_filter.header.id.clone();
    if base.search_filters.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A search filter already exists with this id.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { search_filters: Some(ProgramSearchFiltersDelta { added: vec![payload.search_filter.clone()], ..Default::default() }), ..Default::default() })
}
