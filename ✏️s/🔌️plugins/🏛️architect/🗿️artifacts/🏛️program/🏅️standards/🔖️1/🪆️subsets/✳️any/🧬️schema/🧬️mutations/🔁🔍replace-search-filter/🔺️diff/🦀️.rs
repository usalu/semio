//! 🔺️ Sparse diff construction for the `replace-search-filter` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔍search-filters` per Wave C.

use super::ReplaceSearchFilter;
use crate::artifacts::program::diff::{ProgramSearchFiltersDelta, ProgramSearchFiltersPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplaceSearchFilter, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.search_filters.iter().find(|row| row.header.id == payload.search_filter.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No search filter exists with this id.", [payload.search_filter.header.id.0.clone()]);
    };
    if existing == &payload.search_filter {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This search filter already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.search_filter).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { search_filters: Some(ProgramSearchFiltersDelta { patched: vec![ProgramSearchFiltersPatchEntry { id: payload.search_filter.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
