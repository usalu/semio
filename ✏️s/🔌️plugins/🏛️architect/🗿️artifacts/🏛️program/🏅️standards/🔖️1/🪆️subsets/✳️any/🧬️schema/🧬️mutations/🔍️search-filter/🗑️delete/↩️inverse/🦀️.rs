//! ↩️ Inverse (undo) construction for the `delete-search-filter` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🔍search-filters` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteSearchFilter, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.search_filters.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateSearchFilter(super::super::create_search_filter::CreateSearchFilter { search_filter: base.search_filters[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
