//! 🪜️ Timeline drop laws exercised through the editor's retained command reducer.

use super::*;

#[test]
fn timeline_drops_resolve_current_step_ids_and_publish_only_real_moves() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪜️timeline-drop.json")).expect("timeline drop fixture");
    let mut snapshot = retained_snapshot(0);
    snapshot.step_payloads = fixture["steps"].as_array().unwrap().iter().map(|id| ProcessStep {
        id: id.as_str().unwrap().into(),
        label: String::new(),
        enabled: true,
        origin: None,
        measure: ProcessMeasure::Cut { tool: WorkingSolid::Box { width: 0.1, depth: 0.1, height: 0.1 }, pose: Default::default() },
    }).collect();
    let mut operation = retained_operation();
    operation.parent_document_id = fixture["documentId"].as_str().unwrap().into();
    for case in fixture["cases"].as_array().unwrap() {
        let args = serde_json::json!({
            "documentId": case.get("documentId").unwrap_or(&fixture["documentId"]),
            "stepId": case["stepId"],
            "targetId": case["targetId"],
            "dropPosition": case["dropPosition"],
        });
        let command = Process3dPlayApp::command_from_action("dropStep", &mut Some(DslValue::from(&args)), &mut None).expect("drop action bridge");
        let emit = process3d_retained_reduce(&command, &snapshot, &Process3dConfig::default(), &HistoryView::empty(), &protocol::InteractionState::default(), &Default::default(), None, &operation).expect("retained drop");
        if let Some(order) = case["order"].as_array() {
            let [mutation @ Process3dMutation::ReorderSteps(_)] = emit.artifact_mutations.as_slice() else { panic!("expected one reorder for {args}") };
            let (post, _) = prepare_process3d_document(&snapshot, mutation).expect("drop publishes a valid reorder");
            assert_eq!(post.step_payloads.iter().map(|step| step.id.as_str()).collect::<Vec<_>>(), order.iter().map(|id| id.as_str().unwrap()).collect::<Vec<_>>(), "{args}");
            let repeated = process3d_retained_reduce(&command, &post, &Process3dConfig::default(), &HistoryView::empty(), &protocol::InteractionState::default(), &Default::default(), None, &operation).expect("retained duplicate");
            assert!(repeated.artifact_mutations.is_empty(), "{args}");
        } else {
            assert!(emit.artifact_mutations.is_empty(), "{args}");
        }
    }
}
