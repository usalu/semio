//! 🔺️ Sparse diff construction for the `delete-search-filter` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔍search-filters` per Wave C.

use super::DeleteSearchFilter;
use crate::standards::v1::subsets::any::schema::diff::ProgramSearchFiltersDelta;
use crate::standards::v1::subsets::any::schema::diff::ProgramDiff;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [{id, index}]`.
pub fn diff(payload: &DeleteSearchFilter, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(position) = base.search_filters.iter().position(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No search filter exists with this id.", [payload.id.0.clone()]);
    };
    protocol::MutationOutcome::new(ProgramDiff { search_filters: Some(ProgramSearchFiltersDelta::removal(&base.search_filters, position)), ..Default::default() })
}
