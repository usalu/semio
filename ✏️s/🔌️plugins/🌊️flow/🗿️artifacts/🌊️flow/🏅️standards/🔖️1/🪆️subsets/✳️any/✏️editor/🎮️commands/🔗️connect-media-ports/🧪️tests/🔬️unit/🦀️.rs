use super::*;

#[test]
fn connection_keeps_the_original_typed_owner_until_publication() {
    use semio_framework_plugin::app::ChildEmitPreparationStep;
    crate::editor::flow::unit_tests::context::install_first_party_light_flow_extensions_for_tests();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/owned-connection/🔣️.json")).unwrap();
    assert_eq!(fixture["seedSource"], "publishedDemo");
    let composed = crate::examples::demo::snapshot_from_text(crate::examples::demo::PRIMARY_TEXT).expect("published neutral seed has explicit layout");
    let config = FlowMainWindowConfig::default();
    let mut session = FlowEvalSession::new();
    for accepted in [true, false] {
        let payload = ConnectMediaPorts { source_node_id: fixture["sourceNode"].as_str().unwrap().into(), source_port_id: fixture[if accepted { "sourcePort" } else { "foreignPort" }].as_str().unwrap().into(), target_node_id: fixture["targetNode"].as_str().unwrap().into(), target_port_id: fixture["targetPort"].as_str().unwrap().into() };
        let result = connect_edit(&payload, &composed, &config, &session);
        if accepted {
            let mut emit = result.expect("a valid changed connection retains its source");
            assert_eq!(emit.child_preparations.len(), fixture["acceptedGroups"].as_u64().unwrap() as usize);
            let mut ready = false;
            for _ in 0..fixture["closeSteps"].as_u64().unwrap() {
                match emit.prepare_child_one(1, fixture["closeGrant"].as_u64().unwrap() as usize).unwrap() {
                    ChildEmitPreparationStep::Pending => {},
                    ChildEmitPreparationStep::Ready => { ready = true; break; },
                    ChildEmitPreparationStep::Refused(fault) => panic!("connection source refused: {fault:?}"),
                }
            }
            assert!(ready);
            assert_eq!(emit.child_emits.len(), fixture["acceptedGroups"].as_u64().unwrap() as usize);
            assert_eq!(emit.child_emits[0].child_id, composed.content.child_id);
            assert_eq!(emit.child_emits[0].slot, "content");
            let oracle: serde_json::Value = serde_json::from_slice(&serde_json::to_vec(&emit.child_emits[0]).unwrap()).unwrap();
            assert_eq!(oracle["ops"].as_array().unwrap().len(), fixture["acceptedOperations"].as_u64().unwrap() as usize);
            let packed = semio_framework_plugin::app::ChildEmit::encode_groups(&emit.child_emits);
            let decoded = semio_framework_plugin::app::ChildEmit::decode_groups(&packed).unwrap();
            assert_eq!(serde_json::to_value(&decoded[0]).unwrap(), oracle);
            let mut empty = false;
            for _ in 0..fixture["closeSteps"].as_u64().unwrap() {
                if emit.close_child_one(1, fixture["closeGrant"].as_u64().unwrap() as usize).is_none() { empty = true; break; }
            }
            assert!(empty, "published wire prefix closes within the neutral grant census");
        } else {
            let fault = match result {
                Err(fault) => fault,
                Ok(mut emit) => {
                    let mut empty = false;
                    for _ in 0..fixture["closeSteps"].as_u64().unwrap() {
                        if emit.close_child_one(1, fixture["closeGrant"].as_u64().unwrap() as usize).is_none() { empty = true; break; }
                    }
                    assert!(empty, "unexpected accepted owner closes before the refusal assertion");
                    panic!("incompatible port must refuse before publication");
                },
            };
            assert_eq!(fault.code.0.as_str(), fixture["refusalCode"].as_str().unwrap());
        }
    }
    session.retire_cold();
    println!("[DEBUG] valid Flow connection published its retained original child source; incompatible port refused before any owner existed; serde_json wire-group census matched");
}

#[semio_framework_async_macros::async_test]
async fn removal_routes_publish_original_typed_owners() {
    use crate::editor::flow::unit_tests::context::{composed_scene, dispatch, flow_app_with_registry, select_graph, settle};
    use crate::editor::flow::FlowCommand;
    use crate::editor::flow::commands::{delete_selection::DeleteSelection, disconnect::Disconnect, remove_widget::RemoveWidget};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/owned-connection/🔣️.json")).unwrap();
    for case in fixture["removals"].as_array().unwrap() {
        let mut app = flow_app_with_registry().await;
        let coordinate = app.snapshot().unwrap().content.child_id.clone();
        let target = case["target"].as_str().unwrap();
        let kind = case["command"].as_str().unwrap();
        let before = composed_scene(&app).await;
        let original: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&before)).unwrap();
        before.retire_cold();
        let lane = if kind == "disconnect" { "synapses" } else { "widgets" };
        assert!(original[lane].as_array().unwrap().iter().any(|item| item["id"] == target));
        let command = match kind {
            "removeWidget" => FlowCommand::RemoveWidget(RemoveWidget { widget_id: target.into() }),
            "disconnect" => FlowCommand::Disconnect(Disconnect { synapse_id: target.into() }),
            "deleteSelection" => {
                select_graph(&mut app, &[target], &[]).await;
                FlowCommand::DeleteSelection(DeleteSelection {})
            },
            _ => panic!("neutral removal command is supported"),
        };
        assert!(dispatch(&mut app, command).await.mutations.is_empty());
        settle(&mut app).await;
        let after = composed_scene(&app).await;
        let oracle: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&after)).unwrap();
        after.retire_cold();
        assert!(!oracle[lane].as_array().unwrap().iter().any(|item| item["id"] == target));
        assert_eq!(oracle["widgets"].as_array().unwrap().len(), case["remainingWidgets"].as_u64().unwrap() as usize);
        assert_eq!(oracle["synapses"].as_array().unwrap().len(), case["remainingEdges"].as_u64().unwrap() as usize);
        assert_eq!(app.snapshot().unwrap().content.child_id, coordinate);
        println!("[DEBUG] Flow {kind} published the original typed child owner; serde_json scene census matched widgets={} synapses={}", oracle["widgets"].as_array().unwrap().len(), oracle["synapses"].as_array().unwrap().len());
    }
}
