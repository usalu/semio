use super::*;
use crate::default_snapshot;

/// ⚖️ LAW: a parent diff carries only parent fields — a `schema` delta applies onto its base and leaves the composed `content`
/// handle untouched (design §20.15: content changes are child-lane leaves, never a parent diff).
#[semio_framework_async_macros::async_test]
async fn a_parent_diff_applies_onto_its_base_without_touching_the_content_child() {
    let base = neural_engine::ColdOwner::new(default_snapshot());
    let diff = SequenceDiff { schema: Some("sequence.renamed".into()), ..Default::default() };
    let applied = neural_engine::ColdOwner::new(protocol::MutationDiff::apply(&diff, &*base).expect("valid parent diff"));
    assert_eq!(applied.schema, "sequence.renamed");
    assert_eq!((&applied.content.child_id, &applied.content.target), (&base.content.child_id, &base.content.target));
}
