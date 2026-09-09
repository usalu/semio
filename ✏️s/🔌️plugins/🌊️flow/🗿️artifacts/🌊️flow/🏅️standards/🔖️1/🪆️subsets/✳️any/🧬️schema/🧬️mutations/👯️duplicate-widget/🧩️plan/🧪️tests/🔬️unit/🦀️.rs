use super::*;
use protocol::{fold_plan_diff, fold_plan_inverse, Mutation, MutationDiff};
use semio_framework_artifact_flow_flow::Widget;

fn base_with_source_widget() -> FlowSnapshot {
    let base = FlowSnapshot::default();
    let create = FlowMutation::CreateWidget(CreateWidget { index: 0, widget: Widget::InputNote { id: "note-1".into(), text: "hello".into() } });
    create.diff(&base).diff().apply(&base).expect("valid mutation diff")
}

fn sample_payload() -> DuplicateWidget {
    DuplicateWidget { source_id: "note-1".into(), new_id: "note-2".into(), synapse_id: "note-1-to-note-2".into(), from_port: "out".into(), to_port: "in".into() }
}

#[semio_framework_async_macros::async_test]
async fn plan_folds_to_the_same_snapshot_as_applying_create_then_connect_by_hand() {
    let base = base_with_source_widget();
    let payload = sample_payload();

    let via_composite = fold_plan_diff(&payload, &base).diff().apply(&base).expect("valid mutation diff");

    let create = FlowMutation::CreateWidget(CreateWidget { index: 1, widget: Widget::InputNote { id: "note-2".into(), text: "hello".into() } });
    let after_create = create.diff(&base).diff().apply(&base).expect("valid mutation diff");
    let connect = FlowMutation::ConnectWidgets(ConnectWidgets { index: 0, id: "note-1-to-note-2".into(), from: "note-1".into(), from_port: "out".into(), to: "note-2".into(), to_port: "in".into() });
    let by_hand = connect.diff(&after_create).diff().apply(&after_create).expect("valid mutation diff");

    assert_eq!(via_composite, by_hand);
}

#[semio_framework_async_macros::async_test]
async fn fold_plan_inverse_restores_base_exactly() {
    let base = base_with_source_widget();
    let payload = sample_payload();

    let forward = fold_plan_diff(&payload, &base).diff().apply(&base).expect("valid mutation diff");
    assert_ne!(forward, base, "the composite must actually change the snapshot");

    let inverses = fold_plan_inverse(&payload, &base);
    let restored = inverses.iter().fold(forward, |snapshot, inverse| inverse.diff(&snapshot).diff().apply(&snapshot).expect("valid mutation diff"));
    assert_eq!(restored, base);
}

#[semio_framework_async_macros::async_test]
async fn precondition_rejects_a_missing_source_widget() {
    let base = FlowSnapshot::default();
    let error = precondition(&sample_payload(), &base).expect_err("note-1 does not exist yet");
    assert!(error.contains("note-1"));
}

#[semio_framework_async_macros::async_test]
async fn precondition_rejects_a_new_id_already_taken() {
    let base = base_with_source_widget();
    let payload = DuplicateWidget { new_id: "note-1".into(), ..sample_payload() };
    let error = precondition(&payload, &base).expect_err("new_id collides with source_id");
    assert!(error.contains("differ"));
}
