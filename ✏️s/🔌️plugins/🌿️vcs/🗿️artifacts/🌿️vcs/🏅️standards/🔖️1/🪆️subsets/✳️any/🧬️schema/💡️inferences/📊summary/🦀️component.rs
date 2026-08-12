//! 📊 `summary` — one named inference: a scalar digest of the VCS document's two free-form fields
//! (`tags`, `notes`). Whole-snapshot scalar, not per-entity, so this leaf holds a plain pure
//! function rather than an `InferredField` chain — the family root's
//! `impl protocol::Inference<VcsSnapshot>` calls it directly.

use crate::artifacts::vcs::VcsSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Summary
/// 📊️ Scalar summary of the tags/notes free-form fields.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
    VcsSummary {
        tag_count: snapshot.tags.len() as u32,
        notes_word_count: trimmed.split_whitespace().count() as u32,
        has_notes: !trimmed.is_empty(),
    }
}
//#endregion 🔖️Summary

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;

    #[test]
    fn empty_notes_and_tags_yield_a_zero_summary() {
        let summary = compute_vcs_summary(&VcsSnapshot::default());
        assert_eq!(summary.tag_count, 0);
        assert_eq!(summary.notes_word_count, 0);
        assert!(!summary.has_notes);
    }

    #[test]
    fn tags_and_notes_are_counted_exactly() {
        let snapshot = VcsSnapshot { tags: vec!["a".into(), "b".into(), "c".into()], notes: "  three real words  ".into(), ..VcsSnapshot::default() };
        let summary = compute_vcs_summary(&snapshot);
        assert_eq!(summary.tag_count, 3);
        assert_eq!(summary.notes_word_count, 3);
        assert!(summary.has_notes);
    }

    #[test]
    fn summary_is_deterministic() {
        let snapshot = VcsSnapshot { tags: vec!["a".into()], notes: "hello world".into(), ..VcsSnapshot::default() };
        assert_eq!(compute_vcs_summary(&snapshot), compute_vcs_summary(&snapshot));
    }
}
//#endregion 🧪️Tests
