
use super::*;

#[semio_framework_async_macros::async_test]
async fn empty_notes_and_tags_yield_a_zero_summary() {
    let summary = compute_vcs_summary(&VcsSnapshot::default());
    assert_eq!(summary.tag_count, 0);
    assert_eq!(summary.notes_word_count, 0);
    assert!(!summary.has_notes);
}

#[semio_framework_async_macros::async_test]
async fn tags_and_notes_are_counted_exactly() {
    let snapshot = VcsSnapshot { tags: vec!["a".into(), "b".into(), "c".into()], notes: "  three real words  ".into(), ..VcsSnapshot::default() };
    let summary = compute_vcs_summary(&snapshot);
    assert_eq!(summary.tag_count, 3);
    assert_eq!(summary.notes_word_count, 3);
    assert!(summary.has_notes);
}

#[semio_framework_async_macros::async_test]
async fn summary_is_deterministic() {
    let snapshot = VcsSnapshot { tags: vec!["a".into()], notes: "hello world".into(), ..VcsSnapshot::default() };
    assert_eq!(compute_vcs_summary(&snapshot), compute_vcs_summary(&snapshot));
}
