//! 🔺️ Sparse diff construction for the `create-search-filter` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔍search-filters` per Wave C.

use super::CreateSearchFilter;
use crate::diff::ProgramSearchFiltersDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateSearchFilter, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.search_filter.header.id;
    if base.search_filters.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A search filter already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.search_filters.len());
    if at > base.search_filters.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the search filter list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { search_filters: Some(ProgramSearchFiltersDelta::insertion(at, payload.search_filter.clone())), ..Default::default() })
}
