use super::*;
use crate::default_presentation_snapshot;
use crate::standards::v1::subsets::any::schema::mutations::PresentationMutation;
use crate::standards::v1::subsets::any::schema::mutations::replace_source;
use protocol::Mutation;

#[semio_framework_async_macros::async_test]
async fn replace_source_diff_applies_onto_the_base_snapshot() {
    let base = default_presentation_snapshot();
    let (source, _tiles) = crate::presentation_working_scene(&base);
    let mut next_source = source;
    next_source.kind = "video".into();
    let operation = PresentationMutation::ReplaceSource(replace_source::ReplaceSource { new_source: next_source });
    let diff: PresentationDiff = operation.diff(&base).into_parts().0;
    assert!(diff.presentation.is_some());
    assert!(diff.tiles.is_none(), "a source replacement names no tile row");
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&operation, &base).await;
    let (applied_source, _) = crate::presentation_working_scene(&protocol::apply_diff(&diff, &base).expect("valid mutation diff"));
    assert_eq!(applied_source.kind, "video");
}
