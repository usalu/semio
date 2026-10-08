//! 🔺️ Sparse diff construction for the `create-search-filter` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔍search-filters` per Wave C.

use super::CreateSearchFilter;
use crate::diff::ProgramSearchFiltersDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.index-out-of-range` if `index` lies past the end (both empty diff); else `added = [payload row]`, plus `reordered` (the base order with the row inserted at `index`) unless the row lands last.
pub fn diff(payload: &CreateSearchFilter, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.search_filter.header.id;
    if base.search_filters.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A search filter already exists with this id.", [id.0.clone()]);
    }
    let length = base.search_filters.len();
    let at = payload.index.unwrap_or(length);
    if at > length {
        return protocol::MutationOutcome::error("mutation.index-out-of-range", "The index lies beyond the end of the search filter list.", [id.0.clone()]);
    }
    let reordered = (at < length).then(|| {
        let mut order: Vec<String> = base.search_filters.iter().map(|row| row.header.id.0.clone()).collect();
        order.insert(at, id.0.clone());
        order
    });
    protocol::MutationOutcome::new(ProgramDiff { search_filters: Some(ProgramSearchFiltersDelta { added: vec![payload.search_filter.clone()], reordered, ..Default::default() }), ..Default::default() })
}
