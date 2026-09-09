//! 🧪️ Writer partial updates retain unrelated fields and close with tiny byte grants.

use super::*;
use semio_framework_plugin::WindowTransientOwner;

#[test]
fn writer_window_state_partial_construction_preserves_large_utf8_and_cancels() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧩️partial-construction/🔣️.json")).unwrap();
    let text = fixture["payload"]["text"].as_str().unwrap().repeat(fixture["payload"]["repeat"].as_u64().unwrap() as usize);
    assert_eq!(text.len(), fixture["payload"]["utf8Bytes"].as_u64().unwrap() as usize);
    let owners = WriterMainWindowTransientOwner::build_owners();
    for row in fixture["cases"].as_array().unwrap() {
        for cancel in [false, true] {
            let mut before = fixture["base"].clone();
            before["engagementInput"] = text.clone().into();
            let base: WriterMainWindowTransient = dsl::json::from_json_str(&before.to_string()).unwrap();
            let mutation: WriterMainWindowTransientMutation = dsl::json::from_json_str(&row["mutation"].to_string()).unwrap();
            let mut state = store::TransientStore::<_, WriterMainWindowTransientMutation>::new(base);
            let mut publication = state.begin_publish_one_leased(semio_framework_job::OperationId(1), 0, mutation, owners.preparation.as_ref(), owners.state_retirement.clone()).unwrap();
            let zero = store::ArtifactStoreOneItemGrant { maximum_items: 0, maximum_bytes: 4096 };
            assert!(matches!(state.advance_publish_one(&mut publication, zero).unwrap(), store::ArtifactStoreOneItemAdvance::Blocked));
            assert_eq!(publication.progress().completed_bytes, 0);
            let grant = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 1 };
            for turn in 0..32_768 {
                let previous = publication.progress();
                let step = state.advance_publish_one(&mut publication, grant).unwrap();
                let next = publication.progress();
                assert!(next.completed_bytes - previous.completed_bytes <= 1);
                assert!(next.completed_items - previous.completed_items <= 1);
                if cancel && turn == 1 { assert!(state.cancel_publish_one(&mut publication)); break; }
                if matches!(step, store::ArtifactStoreOneItemAdvance::Published(_)) { assert!(publication.acknowledge()); break; }
            }
            let expected = if cancel { before } else {
                let mut expected = before;
                for (key, value) in row["patch"].as_object().unwrap() { expected[key] = value.clone(); }
                assert_eq!(publication.progress().completed_bytes, row["copiedBytes"].as_u64().unwrap());
                expected
            };
            assert_eq!(serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(state.current_root().as_ref())).unwrap(), expected);
            assert_eq!(publication.close_step(zero).unwrap(), store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            for _ in 0..65_536 {
                match publication.close_step(grant).unwrap() {
                    store::SnapshotRetirementStep::Complete => break,
                    store::SnapshotRetirementStep::Pending { released_items, released_bytes } => assert!(released_items <= 1 && released_bytes <= 1),
                    store::SnapshotRetirementStep::Blocked => panic!("isolated Writer preparation must retire"),
                }
            }
            assert!(publication.terminal_is_empty());
        }
    }
    eprintln!("[DEBUG] Writer partial construction: three neutral serde projections and mid-construction cancellation preserve 12KiB UTF-8 state under one-item/one-byte grants");
}
