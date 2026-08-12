//! 🔺️ Sparse diff construction for the `replace-search-filter` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔍search-filters` per Wave C.

use super::mutation::ReplaceSearchFilter;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramSearchFiltersDelta, ProgramSearchFiltersPatchEntry};

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplaceSearchFilter, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.search_filters.iter().find(|row| row.header.id == payload.search_filter.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.search_filter).expect("diff_patch always produces a full patch");
    ProgramDiff { search_filters: Some(ProgramSearchFiltersDelta { patched: vec![ProgramSearchFiltersPatchEntry { id: payload.search_filter.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
