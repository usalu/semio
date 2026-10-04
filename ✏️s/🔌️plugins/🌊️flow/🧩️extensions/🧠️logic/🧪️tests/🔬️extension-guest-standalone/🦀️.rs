#[test]
    fn bundle_identity_matches_catalogue_fixture() {
        let fixture = semio_framework_pack_json::parse(include_str!("../../../🧫️fixtures/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let mut bundle = bundle();
        assert_eq!(Some(bundle.manifest.extension_id.as_str()), fixture.get("logic").and_then(|entry| entry.get("pluginId")).and_then(semio_framework_pack_json::Value::as_str));
        assert_eq!(bundle.manifest.topic_contributions.len(), 2);
        for contribution in &bundle.manifest.topic_contributions {
            assert_eq!(contribution.payload.get("extensionId").and_then(|value| value.as_str()), fixture.get("logic").and_then(|entry| entry.get("flowId")).and_then(semio_framework_pack_json::Value::as_str));
        }
        bundle.dispose_cold().unwrap();
    }
