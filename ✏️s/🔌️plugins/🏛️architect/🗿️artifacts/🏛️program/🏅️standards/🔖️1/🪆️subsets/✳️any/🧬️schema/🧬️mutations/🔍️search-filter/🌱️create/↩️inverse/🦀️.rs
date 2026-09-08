//! ↩️ Inverse (undo) construction for the `create-search-filter` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🔍search-filters` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse(payload: &super::CreateSearchFilter, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteSearchFilter(super::super::delete_search_filter::DeleteSearchFilter { id: payload.search_filter.header.id.clone() })]
}
