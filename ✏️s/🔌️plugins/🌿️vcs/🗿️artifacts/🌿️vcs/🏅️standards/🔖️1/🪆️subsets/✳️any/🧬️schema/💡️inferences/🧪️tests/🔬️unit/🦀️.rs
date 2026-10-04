use super::*;
use protocol::Inference;

fn tagged_snapshot() -> VcsSnapshot {
    VcsSnapshot { tags: vec!["alpha".into(), "beta".into()], notes: "demo notes here".into(), ..VcsSnapshot::default() }
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = tagged_snapshot();
    assert_eq!(VcsInference::infer(&snapshot).expect("valid materialized inference fixture"), VcsInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(VcsInference::infer(&VcsSnapshot::default()).expect("valid materialized inference fixture"), VcsInference::default());
}

#[semio_framework_async_macros::async_test]
async fn summary_counts_tags_and_words() {
    let inferred = VcsInference::infer(&tagged_snapshot()).expect("valid materialized inference fixture");
    assert_eq!(inferred.summary.tag_count, 2);
    assert_eq!(inferred.summary.notes_word_count, 3);
    assert!(inferred.summary.has_notes);
}
