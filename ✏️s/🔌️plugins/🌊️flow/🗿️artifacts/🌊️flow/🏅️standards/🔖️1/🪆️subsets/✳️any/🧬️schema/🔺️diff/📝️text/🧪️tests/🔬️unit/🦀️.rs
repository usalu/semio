use super::*;
use crate::schema::mutations::FlowMutation;
use protocol::Mutation;

#[semio_framework_async_macros::async_test]
async fn move_widgets_diff_touches_only_the_content_slot() {
    let base = FlowSnapshot::default();
    let operation = FlowMutation::MoveWidgets(crate::schema::mutations::move_widgets::MoveWidgets {
        entries: vec![semio_framework_artifact_flow_flow::FlowLayoutEntry { id: "slider".into(), layout: Some(semio_framework_artifact_flow_flow::WidgetLayout { x: 3.0, y: 4.0 }) }],
    });
    let outcome = operation.diff(&base);
    let diff = outcome.diff();
    assert!(diff.content.is_some(), "MoveWidgets must produce a content diff: {diff:?}");
    assert!(diff.artifact.is_none(), "MoveWidgets must not replace the whole artifact: {diff:?}");
    let after = diff.apply(&base).expect("valid mutation diff");
    assert_eq!(after.to_fixture().layout.get("slider"), Some(&semio_framework_artifact_flow_flow::WidgetLayout { x: 3.0, y: 4.0 }));
}

#[semio_framework_async_macros::async_test]
async fn a_whole_artifact_diff_wins_over_every_content_diff() {
    let base = FlowSnapshot::default();
    let mut replacement = base.clone();
    replacement.schema = "flow.replaced".into();
    let mut diff = diff_replace_content(Vec::new(), Vec::new(), Default::default());
    diff.absorb(diff_set_snapshot(&replacement));
    assert_eq!(diff.apply(&base).expect("valid mutation diff"), replacement);
}
