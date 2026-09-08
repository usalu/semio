
use super::*;
use crate::default_presentation_snapshot;
use crate::op::PresentationMutation;
use crate::schema::mutations::replace_source;
use protocol::Mutation;

#[test]
fn replace_source_diff_applies_onto_the_base_snapshot() {
    let base = default_presentation_snapshot();
    let (source, _tiles) = crate::presentation_working_scene(&base);
    let mut next_source = source;
    next_source.kind = "video".into();
    let operation = PresentationMutation::ReplaceSource(replace_source::ReplaceSource { new_source: next_source });
    let diff: PresentationDiff = operation.diff(&base).into_parts().0;
    assert!(diff.presentation.is_some());
    assert!(diff.artifact.is_none());
    let (applied_source, _) = crate::presentation_working_scene(&diff.apply(&base).expect("valid mutation diff"));
    assert_eq!(applied_source.kind, "video");
}
