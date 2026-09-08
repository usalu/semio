use super::*;
use semio_framework_job::InteractiveJobCloseStep;

fn retire_child_local_owner(mut child: crate::FlowContentChild) {
    let Some(owner) = child.take_local_owner::<FlowWorkingScene>().unwrap() else {
        return;
    };
    let mut retirement = store::SnapshotRetirementFactory::retire(&SceneRetirementFactory, owner);
    for _ in 0..100_000 {
        if retirement.close_step(1, 4096).unwrap() == store::SnapshotRetirementStep::Complete {
            break;
        }
    }
    assert!(retirement.terminal_is_empty());
}

fn close(cursor: &mut SceneCopy, grant: usize) -> usize {
    cursor.begin_close();
    let mut bytes = 0;
    for _ in 0..200_000 {
        match cursor.close_step(1, grant).unwrap() {
            InteractiveJobCloseStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1 && released_bytes <= grant);
                bytes += released_bytes;
            }
            InteractiveJobCloseStep::Complete => {
                assert!(cursor.terminal_is_empty());
                return bytes;
            }
            InteractiveJobCloseStep::Blocked => panic!("positive byte grant must advance scene close"),
        }
    }
    panic!("scene close failed to terminate")
}

#[test]
fn sixteen_kib_authored_label_copies_and_retires_at_actual_grants() {
    let frontier: serde_json::Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🧫️grant-frontier/🔣️.json")).unwrap();
    for row in frontier["cases"].as_array().unwrap() {
        let label = row["unit"].as_str().unwrap().repeat(row["repetitions"].as_u64().unwrap() as usize);
        let grant = row["grantBytes"].as_u64().unwrap() as usize;
        let bytes = label.len() + "slider".len();
        let source = Arc::new(FlowWorkingScene { widgets: vec![Widget::InputSlider { id: "slider".into(), label, value: 6.0, min: 0.0, max: 10.0, step: 0.5 }], ..Default::default() });
        let expected = serde_json::Value::from(dsl::ToValue::to_value(&*source));
        let weak = Arc::downgrade(&source);
        let mut cursor = SceneCopy::new(source);
        let mut copied = 0;
        for _ in 0..100_000 {
            if cursor.complete() {
                break;
            }
            let completed = cursor.advance(1, grant).unwrap().unwrap();
            assert!(completed <= grant);
            copied += completed;
        }
        assert!(cursor.complete());
        assert_eq!(copied, bytes);
        assert_eq!(serde_json::Value::from(dsl::ToValue::to_value(cursor.result.as_ref().unwrap())), expected);
        assert_eq!(close(&mut cursor, grant), bytes * 2);
        assert!(weak.upgrade().is_none());
    }
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn cancelled_nested_map_cursor_keeps_source_alive_across_worker_transfer() {
    let params = neural::Dictionary::new().insert("🌊".repeat(2048), neural::Value::Dictionary(neural::Dictionary::new().insert("value", neural::Value::Atom(neural::Atom::String("x".repeat(8192))))));
    let source = Arc::new(FlowWorkingScene { widgets: vec![Widget::Neuron { id: "node".into(), neuron_kind: "nested".into(), params, input_ports: vec![], output_ports: vec![], preview: false }], ..Default::default() });
    let weak = Arc::downgrade(&source);
    let mut cursor = SceneCopy::new(source);
    for _ in 0..5 {
        cursor.advance(1, 1).unwrap();
    }
    assert!(weak.upgrade().is_some());
    let mut cursor = std::thread::spawn(move || {
        for _ in 0..5 {
            cursor.advance(1, 1).unwrap();
        }
        cursor
    })
    .join()
    .unwrap();
    assert!(weak.upgrade().is_some());
    assert!(!cursor.complete());
    assert!(close(&mut cursor, 1) >= 16384);
    assert!(weak.upgrade().is_none());
}

#[test]
fn scene_identity_matches_node_crypto_and_adopts_the_exact_root() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🪪️content-identity/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let canonical = row["canonicalJson"].as_str().unwrap();
        let (widgets, synapses, layout) = crate::schema::mutations::decode_flow_scene_json(canonical).unwrap();
        let root = Arc::new(FlowWorkingScene { widgets, synapses, layout });
        let expected_id = format!("flow-content-sha256-{}", row["expectedSha256"].as_str().unwrap());
        assert_eq!(serde_json::to_string(&serde_json::Value::from(dsl::ToValue::to_value(&*root))).unwrap(), canonical);
        let derived = crate::flow_content_child_handle(&root.widgets, &root.synapses, &root.layout);
        assert_eq!(derived.child_id, expected_id);
        retire_child_local_owner(derived);
        for maximum_bytes in [1, 64, 4096] {
            let grant = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes };
            let mut cursor = SceneHash::new(Arc::clone(&root));
            let mut bytes = 0;
            for _ in 0..100_000 {
                if cursor.complete() {
                    break;
                }
                let completed = cursor.advance(grant).unwrap();
                assert!(completed <= maximum_bytes);
                bytes += completed;
            }
            assert!(cursor.complete());
            assert_eq!(bytes, crate::FLOW_CONTENT_ID_DOMAIN.len() + canonical.len());
            let (scene, digest) = cursor.take().unwrap();
            assert!(Arc::ptr_eq(&scene, &root));
            let mut child = crate::flow_content_child_from_digest(digest, scene);
            assert_eq!(child.child_id, expected_id);
            assert_eq!(child.target.artifact_id, expected_id);
            assert_eq!(child.target.dialect.artifact_kind, fixture["dialect"]["artifactKind"].as_str().unwrap());
            assert_eq!(child.target.dialect.standard, fixture["dialect"]["standard"].as_str().unwrap());
            assert_eq!(child.target.dialect.subset, fixture["dialect"]["subset"].as_str().unwrap());
            assert!(Arc::ptr_eq(&child.local_owner::<FlowWorkingScene>().unwrap(), &root));
            drop(child.take_local_owner::<FlowWorkingScene>().unwrap().unwrap());
            cursor.begin_close();
            for _ in 0..100_000 {
                if cursor.close_step(grant).unwrap() == store::SnapshotRetirementStep::Complete {
                    break;
                }
            }
            assert!(cursor.terminal_is_empty());
        }
        assert_eq!(Arc::strong_count(&root), 1);
        let mut retirement = store::SnapshotRetirementFactory::retire(&SceneRetirementFactory, root);
        for _ in 0..100_000 {
            if retirement.close_step(1, 4096).unwrap() == store::SnapshotRetirementStep::Complete {
                break;
            }
        }
        assert!(retirement.terminal_is_empty());
    }
    eprintln!("[DEBUG] Flow content-addressed child and target identities matched the five Node crypto vectors at three grants");
}
