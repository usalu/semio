//! 🔺️ Sparse diff construction for the `replace-search-filter` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔍search-filters` per Wave C.

use super::ReplaceSearchFilter;
use crate::diff::ProgramSearchFiltersDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceSearchFilter, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.search_filter.header.id;
    let Some(position) = base.search_filters.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No search filter exists with this id.", [id.0.clone()]);
    };
    if base.search_filters[position] == payload.search_filter {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This search filter already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramSearchFiltersDelta::removal(&base.search_filters, position);
    delta.absorb(ProgramSearchFiltersDelta::insertion(position, payload.search_filter.clone()));
    protocol::MutationOutcome::new(ProgramDiff { search_filters: Some(delta), ..Default::default() })
}
