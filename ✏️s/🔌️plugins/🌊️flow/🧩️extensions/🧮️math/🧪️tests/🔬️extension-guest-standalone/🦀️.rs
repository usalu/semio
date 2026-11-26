#[test]
    fn bundle_identity_matches_catalogue_fixture() {
        let stock = semio_framework_value::RetainedCloneGrant { maximum_items: 32768, maximum_copy_bytes: 1 << 25, maximum_capacity_bytes: 1 << 26, maximum_release_bytes: 1 << 26, maximum_depth: 4096 };
        let mut caller = semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::OperationId(71), semio_framework_job::Generation(3), stock, || Some(1000), 500_000, 8_000_000);
        caller.retain_dispatch().expect("the fixture retains its initiating dispatch");
        let fixture = semio_framework_pack_json::parse(include_str!("../../../🧪️tests/🧫️fixtures/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let mut bundle = bundle();
        assert_eq!(Some(bundle.manifest.extension_id.as_str()), fixture.get("math").and_then(|entry| entry.get("pluginId")).and_then(semio_framework_pack_json::Value::as_str));
        assert_eq!(bundle.manifest.topic_contributions.len(), 2);
        for contribution in &bundle.manifest.topic_contributions {
            assert_eq!(contribution.payload.get("extensionId").and_then(|value| value.as_str()), fixture.get("math").and_then(|entry| entry.get("flowId")).and_then(semio_framework_pack_json::Value::as_str));
        }
        bundle.begin_close();
        for _ in 0..65536 {
            if bundle.terminal_is_empty() { break; }
            caller.refreshed_turn(stock, |cx| cx.with_original_retirement(|cx| bundle.close_step(cx).map(|_| ()))).unwrap();
        }
        assert!(bundle.terminal_is_empty());
        caller.retire(stock);
    }
