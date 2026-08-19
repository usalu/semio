//! 🔺️ Sparse diff construction for the `delete-search-filter` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔍search-filters` per Wave C.

use super::mutation::DeleteSearchFilter;
use crate::artifacts::program::diff::ProgramSearchFiltersDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [id]`.
pub async fn diff(payload: &DeleteSearchFilter, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if !base.search_filters.iter().any(|row| row.header.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No search filter exists with this id.", [payload.id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { search_filters: Some(ProgramSearchFiltersDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() })
}
