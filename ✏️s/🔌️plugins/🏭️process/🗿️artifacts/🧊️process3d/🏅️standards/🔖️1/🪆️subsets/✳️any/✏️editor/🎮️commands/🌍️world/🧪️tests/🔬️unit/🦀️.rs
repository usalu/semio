use super::*;

#[semio_framework_async_macros::async_test]
async fn face_drag_negative_distance_yields_cut() {
    let step = process3d_step_from_face_drag(&crate::empty_process3d_snapshot(), [0.0, 0.0, 1.0], [0.0, 0.0, 1.0], -0.5, None, &Process3dLabels::NATIVE_EN).expect("step");
    assert!(matches!(step.measure, ProcessMeasure::Cut { .. }));
    assert_eq!(step.label, "Push Cut");
}

#[semio_framework_async_macros::async_test]
async fn face_drag_positive_distance_yields_attach() {
    let step = process3d_step_from_face_drag(&crate::empty_process3d_snapshot(), [0.0, 0.0, 1.0], [0.0, 0.0, 1.0], 0.5, None, &Process3dLabels::NATIVE_EN).expect("step");
    assert!(matches!(step.measure, ProcessMeasure::Attach { .. }));
    assert_eq!(step.label, "Pull Attach");
}

#[semio_framework_async_macros::async_test]
async fn face_drag_zero_distance_is_noop() {
    assert!(process3d_step_from_face_drag(&crate::empty_process3d_snapshot(), [0.0, 0.0, 1.0], [0.0, 0.0, 1.0], 0.0, None, &Process3dLabels::NATIVE_EN).is_none());
}

/// 🛠️ One face drag is ONE world-tool transaction: its `create-step` leaf, stamped with a minted ref of
/// `<appId>#worldFaceDragEnd`; two gestures are two transactions.
#[semio_framework_async_macros::async_test]
async fn a_face_drag_is_one_world_tool_transaction() {
    let document = crate::empty_process3d_snapshot();
    let step = process3d_step_from_face_drag(&document, [0.0, 0.0, 1.0], [0.0, 0.0, 1.0], -0.5, None, &Process3dLabels::NATIVE_EN).expect("step");
    let mutations = crate::schema::insert_step_mutations(&document, step, None);
    let (reference, committed) = process3d_world_commit("worldFaceDragEnd", "seed-one", WorldToolRequest { mutations: mutations.clone() }).expect("the world tool accepts the gesture").expect("the gesture commits");
    assert!(reference.id.starts_with("tx-"), "{reference:?}");
    assert_eq!(reference.tool, "s.process.process3d@1/*#editor#worldFaceDragEnd");
    assert_eq!(committed, mutations, "the transaction holds exactly the placed step's leaf");
    let (second, _) = process3d_world_commit("worldFaceDragEnd", "seed-two", WorldToolRequest { mutations }).expect("the world tool accepts the gesture").expect("the second gesture commits");
    assert_ne!(second.id, reference.id, "two gestures are two transactions");
}

/// 🧹️ A gesture that places nothing leaves zero trace: no transaction, no edit.
#[semio_framework_async_macros::async_test]
async fn a_gesture_that_places_nothing_leaves_zero_trace() {
    assert!(process3d_world_commit("worldFaceDragEnd", "seed", WorldToolRequest { mutations: Vec::new() }).expect("the world tool accepts the gesture").is_none());
}

/// 📤️ A placed world gesture is ONE edit stamped with its transaction, and the viewer's replay cursor (config, never
/// history) moves past the inserted step on the config lane of the same emit.
#[semio_framework_async_macros::async_test]
async fn a_world_gesture_emits_one_stamped_edit_and_moves_the_viewers_cursor() {
    use semio_framework_plugin::{AppOperationContext, HistoryView};
    let snapshot = crate::empty_process3d_snapshot();
    let history = HistoryView::empty();
    let operation = AppOperationContext { app_instance_id: 1, parent_document_id: "process3d-world".into(), operation_id: 1, generation: 0, canonical_base_revision: [0; 32], authoring_seed: "world-seed".into() };
    let doc = ArtifactView::with_operation(&snapshot, &history, operation);
    let config = Process3dConfig { resolved_up_to: Some(0), ..Process3dConfig::default() };
    let cfg = ConfigView { snapshot: &config, window: None };
    let step = process3d_step_from_face_drag(&snapshot, [0.0, 0.0, 1.0], [0.0, 0.0, 1.0], -0.5, None, &Process3dLabels::NATIVE_EN).expect("step");
    let emit = world_emit("worldFaceDragEnd", &doc, &cfg, step, Vec::new()).expect("the gesture emits");
    let transaction = emit.transaction.as_ref().expect("the gesture is a tool transaction");
    assert_eq!(transaction.tool, "s.process.process3d@1/*#editor#worldFaceDragEnd");
    assert!(matches!(&emit.artifact_mutations[..], [Process3dMutation::CreateStep(create)] if create.index == 0), "the step lands at the viewer's cursor");
    assert_eq!(emit.config_mutations, vec![Process3dConfigMutation::SetCursor(Process3dConfigSetCursor{ value: Some(1) })], "the cursor moves past the new step on the config lane");
}
