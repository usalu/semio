//! 🔺️ Sparse diff construction for the `create-search-filter` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔍search-filters` per Wave C.

use super::mutation::CreateSearchFilter;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramSearchFiltersDelta};

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.search_filters` on apply.
pub fn diff(payload: &CreateSearchFilter, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { search_filters: Some(ProgramSearchFiltersDelta { added: vec![payload.search_filter.clone()], ..Default::default() }), ..Default::default() }
}
