//! 🔺️ Sparse diff construction for the `rename-search-filter` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔍search-filters` per Wave C.

use super::mutation::RenameSearchFilter;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramSearchFiltersDelta, ProgramSearchFiltersPatchEntry};
use crate::artifacts::program::registers::SearchFilterPatch;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameSearchFilter, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = SearchFilterPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { search_filters: Some(ProgramSearchFiltersDelta { patched: vec![ProgramSearchFiltersPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
