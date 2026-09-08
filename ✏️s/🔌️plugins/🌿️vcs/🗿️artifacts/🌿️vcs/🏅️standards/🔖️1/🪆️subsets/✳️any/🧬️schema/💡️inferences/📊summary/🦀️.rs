//! 📊 `summary` — one named inference: a scalar digest of the VCS document's two free-form fields
//! (`tags`, `notes`). Whole-snapshot scalar, not per-entity, so this leaf holds a plain pure
//! function rather than an `InferredField` chain — the family root's
//! `impl protocol::Inference<VcsSnapshot>` calls it directly.

use crate::VcsSnapshot;

//#region 🔖️Summary
/// 📊️ Scalar summary of the tags/notes free-form fields.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct VcsSummary {
    pub tag_count: u32,
    pub notes_word_count: u32,
    pub has_notes: bool,
}

/// 📊️ `tagCount` = `tags.len()`; `notesWordCount`/`hasNotes` derived from a whitespace split of
/// `notes` — real, cheap, deterministic derivations over the only two free-form persistent fields
/// this document has.
pub fn compute_vcs_summary(snapshot: &VcsSnapshot) -> VcsSummary {
    let trimmed = snapshot.notes.trim();
    VcsSummary { tag_count: snapshot.tags.len() as u32, notes_word_count: trimmed.split_whitespace().count() as u32, has_notes: !trimmed.is_empty() }
}
//#endregion 🔖️Summary

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
