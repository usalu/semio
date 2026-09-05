//! 🧪️ Public config protocol laws checked against the language-neutral fixture and serde.

use semio_framework_os_kernel::{Mutation, MutationDiff, OpBinary, OpText};
use semio_s_plugin_norm::config::{NormConfig, NormConfigMutation};

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct OraclePayload {
    index: Option<u32>,
}

#[test]
fn config_mutation_fixture_matches_serde_and_round_trips() {
    assert_eq!(std::any::TypeId::of::<NormConfig>(), std::any::TypeId::of::<semio_s_plugin_norm::config::schema::NormConfig>(), "the runtime config must be the schema-owned type");
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    let descriptors = <NormConfigMutation as Mutation<NormConfig>>::DESCRIPTORS;
    assert_eq!(descriptors.len(), 1, "config has exactly one concrete semantic field mutation");
    let descriptor = &descriptors[0];
    assert_eq!(descriptor.semantic_kind, "change-selected-check-index");
    assert_eq!(descriptor.text_opcode, Some("change-selected-check-index"));
    assert_eq!(descriptor.binary_tag, Some(0));
    assert!(descriptor.validate().is_ok());
    for case in fixture["cases"].as_array().unwrap() {
        let payload: OraclePayload = serde_json::from_value(case["payload"].clone()).unwrap();
        let wire = serde_json::json!({ "ChangeSelectedCheckIndex": case["payload"] });
        let mutation: NormConfigMutation = pack::json::from_json_str(&wire.to_string()).unwrap();
        let base = NormConfig { selected_check_index: serde_json::from_value(case["before"].clone()).unwrap() };
        let outcome = mutation.diff(&base);
        assert_eq!(!outcome.messages().is_empty(), case["warning"].as_bool().unwrap());
        let next = outcome.diff().apply(&base).unwrap();
        assert_eq!(next.selected_check_index, payload.index, "{}", case["id"]);
        assert_eq!(serde_json::to_value(next.selected_check_index).unwrap(), case["after"]);
        let backwards = mutation.inverse(&base);
        assert_eq!(backwards.len(), 1);
        assert_eq!(backwards[0].diff(&next).diff().apply(&next).unwrap(), base);
        assert_eq!(NormConfigMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
        assert_eq!(NormConfigMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);
    }
    for payload in fixture["invalid"].as_array().unwrap() {
        assert!(serde_json::from_value::<OraclePayload>(payload.clone()).is_err());
        let wire = serde_json::json!({ "ChangeSelectedCheckIndex": payload });
        assert!(pack::json::from_json_str::<NormConfigMutation>(&wire.to_string()).is_err());
    }
    for text in ["snapshot", "selected-check index=5", "unknown-operation"] {
        assert!(NormConfigMutation::parse_op(text).is_err());
    }
    for wire in ["{}", r#"{"Snapshot":{}}"#, r#"{"SetSelectedCheckIndex":{"index":1}}"#, r#"{"ChangeSelectedCheckIndex":{},"unknown":true}"#] {
        assert!(pack::json::from_json_str::<NormConfigMutation>(wire).is_err());
    }
    for bytes in [&[][..], &[0][..], &[1][..], &[1, 1][..]] {
        assert!(NormConfigMutation::decode_op(bytes).is_err());
    }
    for row in fixture["text"].as_array().unwrap() {
        let decoded = NormConfigMutation::parse_op(row["wire"].as_str().unwrap());
        assert_eq!(decoded.is_ok(), row["accepted"].as_bool().unwrap(), "text {}: {decoded:?}", row["id"]);
        if let Ok(mutation) = decoded {
            assert_wire_transition(&mutation, &row["after"]);
        }
    }
    for row in fixture["binary"].as_array().unwrap() {
        let hex = row["hex"].as_str().unwrap();
        let bytes: Vec<u8> = (0..hex.len()).step_by(2).map(|index| u8::from_str_radix(&hex[index..index + 2], 16).unwrap()).collect();
        let decoded = NormConfigMutation::decode_op(&bytes);
        assert_eq!(decoded.is_ok(), row["accepted"].as_bool().unwrap(), "binary {}: {decoded:?}", row["id"]);
        if let Ok(mutation) = decoded {
            assert_wire_transition(&mutation, &row["after"]);
            assert_eq!(mutation.encode_op().unwrap(), bytes, "{}", row["id"]);
        }
    }
    eprintln!("[DEBUG] Norm config public protocol: 5 fixtures, 5 hostile payloads, 13 text and 25 binary vectors, serde oracle, inverse/text/binary laws passed");
}

fn assert_wire_transition(mutation: &NormConfigMutation, expected: &serde_json::Value) {
    let base = NormConfig { selected_check_index: Some(17) };
    let next = mutation.diff(&base).diff().apply(&base).unwrap();
    assert_eq!(serde_json::to_value(next.selected_check_index).unwrap(), *expected);
    let inverse = mutation.inverse(&base);
    assert_eq!(inverse.len(), 1);
    assert_eq!(inverse[0].diff(&next).diff().apply(&next).unwrap(), base);
}
