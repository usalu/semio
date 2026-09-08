
use super::*;
use store::SpaceMember;

#[semio_framework_async_macros::async_test]
async fn max_semantic_config_publication_cancel_retry_and_close_use_real_grants() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️fixtures/🧫️grant-frontier/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        for grant_value in fixture["preparationGrantBytes"].as_array().unwrap() {
            let maximum_bytes = grant_value.as_u64().unwrap() as usize;
            for cancel in [None, Some(row["cancelAt"].as_u64().unwrap() as usize)] {
                let text = row["unit"].as_str().unwrap().repeat(row["repetitions"].as_u64().unwrap() as usize);
                let initial = FlowConfig::default();
                let expected = if cancel.is_some() { initial.catalogue_sections_json.clone() } else { text.clone() };
                let envelope = store::create_document_envelope::<FlowConfig, FlowConfigMutation>("flow.config", "grant-frontier", initial, None);
                let mut store = store::ArtifactStore::new(envelope).await.unwrap();
                store.install_member_store_owners_exact(store_owners());
                let grant = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes };
                let generation = store.generation_now();
                let mut publication = store
                    .begin_apply_one(
                        semio_framework_job::OperationId(1),
                        generation,
                        store.content_revision_now(),
                        "flow-test".into(),
                        FlowConfigMutation::SetCatalogueSections { sections_json: text },
                        None,
                        store::HistoryLane::Document,
                        Some(&PreparationFactory),
                    )
                    .unwrap();
                let mut last_bytes = 0;
                let mut finished = false;
                for step in 0..500_000 {
                    if cancel == Some(step) {
                        publication.begin_close();
                        finished = true;
                        break;
                    }
                    let result = store.advance_apply_one(&mut publication, grant).unwrap();
                    let bytes = publication.progress().completed_bytes;
                    assert!(bytes >= last_bytes && bytes - last_bytes <= maximum_bytes as u64);
                    last_bytes = bytes;
                    if matches!(result, store::ArtifactStoreOneItemAdvance::Published(_)) {
                        assert!(cancel.is_none());
                        assert!(publication.retry());
                        assert_eq!(store.generation_now(), generation + 1);
                        assert!(publication.acknowledge());
                        finished = true;
                        break;
                    }
                }
                assert!(finished, "preparation reached its live frontier");
                assert_eq!(store.snapshot().unwrap().catalogue_sections_json, expected);
                for _ in 0..500_000 {
                    let step = publication.close_step(grant).unwrap();
                    if let store::SnapshotRetirementStep::Pending { released_items, released_bytes } = step {
                        assert!(released_items <= 1 && released_bytes <= maximum_bytes);
                    }
                    if step == store::SnapshotRetirementStep::Complete {
                        break;
                    }
                }
                assert!(publication.terminal_is_empty());
                for _ in 0..500_000 {
                    if store.close_owned_step(1, maximum_bytes).unwrap() == store::SnapshotRetirementStep::Complete {
                        break;
                    }
                }
                assert!(store.close_owned_terminal_is_empty());
                eprintln!("[DEBUG] Flow semantic config case={} grant={} cancel={cancel:?} closed=true", row["id"], maximum_bytes);
            }
        }
    }
}

fn oracle(value: &impl ArtifactCanonicalJson, path: &mut Vec<usize>) -> serde_json::Value {
    match value.canonical_json_node(path).unwrap() {
        Json::Null => serde_json::Value::Null,
        Json::Bool(value) => value.into(),
        Json::U64(value) => value.into(),
        Json::F64(value) => serde_json::json!(value),
        Json::String(value) => value.into(),
        Json::Array(length) => serde_json::Value::Array(
            (0..length)
                .map(|index| {
                    path.push(index);
                    let item = oracle(value, path);
                    path.pop();
                    item
                })
                .collect(),
        ),
        Json::Object(length) => {
            let mut object = serde_json::Map::new();
            for index in 0..length {
                let key = value.canonical_json_key(path, index).unwrap().to_owned();
                path.push(index);
                let item = oracle(value, path);
                path.pop();
                object.insert(key, item);
            }
            serde_json::Value::Object(object)
        }
        other => panic!("unexpected Flow scalar: {other:?}"),
    }
}

#[test]
fn typed_config_canonical_tree_matches_serde_for_every_variant() {
    let text = "\0🌊\"\\".repeat(1024);
    let variants = [
        FlowConfigMutation::Snapshot { config: FlowConfig::default() },
        FlowConfigMutation::SetContributions { json: text.clone() },
        FlowConfigMutation::SetPreviewOff { node_ids: vec![text.clone(), "next".into()] },
        FlowConfigMutation::SetCamera { camera: semio_framework_artifact_flow_flow::CameraJson { x: 1.0, y: -0.0, zoom: 2.0 } },
        FlowConfigMutation::SetLodMode { value: text.clone() },
        FlowConfigMutation::SetProximityDistance { value: 3.0 },
        FlowConfigMutation::SetGridVisible { value: true },
        FlowConfigMutation::SetGridSnapEnabled { value: false },
        FlowConfigMutation::SetGridFactor { value: 4.0 },
        FlowConfigMutation::SetCatalogueSections { sections_json: text.clone() },
        FlowConfigMutation::SetAutomationEnabled { json: text.clone() },
        FlowConfigMutation::SetGeneration { json: text.clone() },
        FlowConfigMutation::SetDuplicateWidgetProgress { json: text.clone() },
        FlowConfigMutation::CancelDuplicateWidget { generation: u64::MAX },
        FlowConfigMutation::SetLocale { value: text },
    ];
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️fixtures/🧫️grant-frontier/🔣️.json")).unwrap();
    assert_eq!(variants.len(), fixture["canonicalVariants"].as_array().unwrap().len());
    for (index, value) in variants.into_iter().enumerate() {
        assert_eq!(value.canonical_json_key(&[], 0).unwrap(), fixture["canonicalVariants"][index].as_str().unwrap());
        assert_eq!(oracle(&value, &mut Vec::new()), serde_json::Value::from(dsl::ToValue::to_value(&value)));
        assert!(value.canonical_json_node(&[9]).is_err());
        assert!(value.canonical_json_key(&[], 1).is_err());
    }
}
