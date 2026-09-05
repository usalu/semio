//! ↩️ Inverse (undo) construction for the `replace-search-filter` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🔍search-filters` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::ReplaceSearchFilter, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.search_filters.iter().find(|row| row.header.id == payload.search_filter.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceSearchFilter(super::ReplaceSearchFilter { search_filter: existing.clone() })],
        None => Vec::new(),
    }
}
