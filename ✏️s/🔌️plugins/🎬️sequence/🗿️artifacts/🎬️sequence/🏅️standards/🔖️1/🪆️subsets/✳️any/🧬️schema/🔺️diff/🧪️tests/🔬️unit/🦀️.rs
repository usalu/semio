use super::*;
use crate::default_snapshot;
use protocol::os_spr::protocol_laws::{assert_diff_algebra_inverse_law};

/// ⚖️ LAW: a parent diff carries only parent fields — a `schema` delta applies onto its base and leaves the composed `content`
/// handle untouched (design §20.15: content changes are child-lane leaves, never a parent diff).
#[semio_framework_async_macros::async_test]
async fn a_parent_diff_applies_onto_its_base_without_touching_the_content_child() {
    let base = neural_engine::ColdOwner::new(default_snapshot());
    let diff = SequenceDiff { schema: Some("sequence.renamed".into()), ..Default::default() };
    let applied = neural_engine::ColdOwner::new(protocol::apply_diff(&diff, &*base).expect("valid parent diff"));
    assert_eq!(applied.schema, "sequence.renamed");
    assert_eq!((&applied.content.child_id, &applied.content.target), (&base.content.child_id, &base.content.target));
}

#[semio_framework_async_macros::async_test]
async fn absorb_keeps_the_later_slot_and_the_inverse_restores_exactly_the_named_slots() {
    use protocol::DiffAlgebra;
    let base = neural_engine::ColdOwner::new(default_snapshot());
    let mut sum = SequenceDiff { schema: Some("sequence.first".into()), ..Default::default() };
    sum.absorb(SequenceDiff { schema: Some("sequence.last".into()), ..Default::default() });
    assert_eq!(sum.schema.as_deref(), Some("sequence.last"));
    assert!(sum.content.is_none());
    let after = neural_engine::ColdOwner::new(protocol::apply_diff(&sum, &*base).expect("valid parent diff"));
    let inverse = sum.inverse(&base);
    assert_eq!(inverse, SequenceDiff { schema: Some(base.schema.clone()), content: None });
    let restored = neural_engine::ColdOwner::new(protocol::apply_diff(&inverse, &*after).expect("valid inverse diff"));
    assert_eq!(*restored, *base);
    assert!(SequenceDiff::default().is_empty() && !sum.is_empty());
}

