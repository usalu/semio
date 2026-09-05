//! ↩️ Inverse (undo) construction for the `delete-search-filter` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🔍search-filters` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::DeleteSearchFilter, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.search_filters.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateSearchFilter(super::super::create_search_filter::CreateSearchFilter { search_filter: existing.clone() })],
        None => Vec::new(),
    }
}
