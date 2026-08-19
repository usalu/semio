//! 🔺️ Sparse diff construction for the `rename-search-filter` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔍search-filters` per Wave C.

use super::mutation::RenameSearchFilter;
use crate::artifacts::program::diff::{ProgramSearchFiltersDelta, ProgramSearchFiltersPatchEntry};
use crate::artifacts::program::registers::SearchFilterPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the name is unchanged (both empty diff), else `patched = [{id, name: Some(new_name)}]`.
pub async fn diff(payload: &RenameSearchFilter, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.search_filters.iter().find(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No search filter exists with this id.", [payload.id.0.clone()]);
    };
    if existing.header.name == payload.new_name {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This search filter already has this name.").at([payload.id.0.clone()])]);
    }
    let patch = SearchFilterPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    protocol::MutationOutcome::new(ProgramDiff { search_filters: Some(ProgramSearchFiltersDelta { patched: vec![ProgramSearchFiltersPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
