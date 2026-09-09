use super::*;
use store::SpaceMember;

#[semio_framework_async_macros::async_test]
async fn semantic_artifact_prepare_publish_retry_cancel_and_close_use_production_and_one_byte_grants() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧫️fixtures/🧬️artifact-recipes.json")).unwrap();
    let label = fixture["label"]["unit"].as_str().unwrap().repeat(fixture["label"]["repetitions"].as_u64().unwrap() as usize);
    for bytes in fixture["grants"].as_array().unwrap() {
        let grant = Grant { maximum_items: 1, maximum_bytes: bytes.as_u64().unwrap() as usize };
        for row in fixture["cases"].as_array().unwrap() {
            for cancel in [None, Some(0), Some(131), Some(5001)] {
                let scene = super::super::recipe::tests::source(&label);
                let content = crate::flow_content_child_handle_and_cache(scene.widgets, scene.synapses, scene.layout);
                let initial = FlowSnapshot { schema: "flow".into(), camera: semio_framework_artifact_flow_flow::CameraJson::default(), content };
                let initial_scene = initial.content.local_owner::<FlowWorkingScene>().unwrap();
                let baseline = serde_json::Value::from(dsl::ToValue::to_value(&*initial_scene));
                drop(initial_scene);
                let envelope = store::create_document_envelope::<FlowSnapshot, FlowMutation>("flow.flow", "retained-recipe", initial, None);
                let mut store = store::ArtifactStore::new(envelope).await.unwrap();
                store.install_member_store_owners_exact(crate::retirement::store_owners());
                let generation = store.generation_now();
                let mutation = dsl::FromValue::from_value(dsl::DslValue::from(row["mutation"].clone())).unwrap();
                let factory: std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<FlowSnapshot, FlowMutation>> = std::sync::Arc::new(PreparationFactory);
                let mut publication = store.begin_apply_batch(semio_framework_job::OperationId(1), generation, store.content_revision_now(), "flow-test".into(), vec![mutation], None, store::HistoryLane::Document, Some(&factory)).unwrap();
                let mut finished = false;
                let mut published = false;
                let mut previous = 0;
                for step in 0..500_000 {
                    if cancel == Some(step) {
                        publication.begin_close();
                        finished = true;
                        break;
                    }
                    let outcome = store.advance_apply_batch(&mut publication, grant).unwrap();
                    let bytes = publication.progress().completed_bytes;
                    assert!(bytes >= previous && bytes - previous <= grant.maximum_bytes as u64);
                    previous = bytes;
                    if matches!(outcome, store::ArtifactStoreOneItemAdvance::Published(_)) {
                        assert!(publication.retry());
                        assert_eq!(store.generation_now(), generation + 1);
                        assert!(publication.acknowledge());
                        finished = true;
                        published = true;
                        break;
                    }
                }
                assert!(finished);
                {
                    let snapshot = store.snapshot().unwrap();
                    let scene = snapshot.content.local_owner::<FlowWorkingScene>().unwrap();
                    let json = serde_json::Value::from(dsl::ToValue::to_value(&*scene));
                    if !published {
                        assert_eq!(json, baseline);
                    } else {
                        assert_eq!(json["widgets"].as_array().unwrap().iter().map(|widget| widget["id"].clone()).collect::<Vec<_>>(), *row["widgets"].as_array().unwrap());
                        assert_eq!(json["synapses"].as_array().unwrap().iter().map(|edge| edge["id"].clone()).collect::<Vec<_>>(), *row["synapses"].as_array().unwrap());
                        if row["id"] == "replace-widget" {
                            assert_eq!(json["widgets"][1]["label"], "changed");
                        }
                        if row["id"] == "move-widget" {
                            assert_eq!(json["layout"]["b"], serde_json::json!({ "x": 5.0, "y": 7.0 }));
                        }
                    }
                }
                for _ in 0..500_000 {
                    let step = publication.close_step(grant).unwrap();
                    if let Close::Pending { released_items, released_bytes } = step {
                        assert!(released_items <= 1 && released_bytes <= grant.maximum_bytes);
                    }
                    if step == Close::Complete {
                        break;
                    }
                }
                assert!(publication.terminal_is_empty());
                for _ in 0..500_000 {
                    let step = store.close_owned_step(1, grant.maximum_bytes).unwrap();
                    if let Close::Pending { released_items, released_bytes } = step {
                        assert!(released_items <= 1 && released_bytes <= grant.maximum_bytes);
                    }
                    if step == Close::Complete {
                        break;
                    }
                }
                assert!(store.close_owned_terminal_is_empty());
                eprintln!("[DEBUG] Flow artifact recipe={} grant={} cancel={cancel:?} published={published} terminal=true", row["id"], grant.maximum_bytes);
            }
        }
    }
}
