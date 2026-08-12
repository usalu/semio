//! 🔺️ Sparse diff construction for the `search_filters` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateSearchFilter, DeleteSearchFilter, RenameSearchFilter, ReplaceSearchFilter};
use crate::artifacts::program::diff::{ProgramSearchFiltersDelta, ProgramSearchFiltersPatchEntry};
use crate::artifacts::program::registers::SearchFilterPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.search_filters` on apply.
pub fn diff_create(payload: &CreateSearchFilter, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { search_filters: Some(ProgramSearchFiltersDelta { added: vec![payload.search_filter.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteSearchFilter, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { search_filters: Some(ProgramSearchFiltersDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameSearchFilter, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = SearchFilterPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { search_filters: Some(ProgramSearchFiltersDelta { patched: vec![ProgramSearchFiltersPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceSearchFilter, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.search_filters.iter().find(|row| row.header.id == payload.search_filter.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.search_filter).expect("diff_patch always produces a full patch");
    ProgramDiff { search_filters: Some(ProgramSearchFiltersDelta { patched: vec![ProgramSearchFiltersPatchEntry { id: payload.search_filter.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
