//! 🔺️ Sparse diff construction for the `delete-search-filter` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🔍search-filters` per Wave C.

use super::mutation::DeleteSearchFilter;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramSearchFiltersDelta};

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteSearchFilter, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { search_filters: Some(ProgramSearchFiltersDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
