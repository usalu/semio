mod typed_operation_fault_tests {
    use super::*;

    /// 🧵️ A fixture string: `name` itself, or its `nameRepeat` spelling (`text` repeated `count` times).
    fn spelled(value: &serde_json::Value, name: &str) -> String {
        match (value.get(name).and_then(serde_json::Value::as_str), value.get(format!("{name}Repeat"))) {
            (Some(text), _) => text.to_string(),
            (None, Some(repeat)) => repeat["text"].as_str().expect("repeat text").repeat(repeat["count"].as_u64().expect("repeat count") as usize),
            (None, None) => panic!("fixture value names neither `{name}` nor `{name}Repeat`: {value}"),
        }
    }

    fn origin(value: &serde_json::Value) -> FaultOrigin {
        <FaultOrigin as protocol::FromValue>::from_value(DslValue::String(value["origin"].as_str().expect("origin").to_string())).expect("a known origin")
    }

    fn expected(value: &serde_json::Value) -> TypedOperationFault {
        TypedOperationFault { schema: TYPED_OPERATION_FAULT_SCHEMA.to_string(), code: spelled(value, "code"), origin: origin(value), message: spelled(value, "message") }
    }

    /// ⚖️ LAW: every case of the language-agnostic typed-operation fault fixture — one `Fault` becomes exactly its
    /// record, the record round-trips through its wire bytes within one result page, and only a record decodes.
    #[test]
    fn typed_operation_fault_records_match_the_language_agnostic_fixture() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧯️typed-operation-fault.json")).expect("typed-operation fault fixture parses");
        let limits = &fixture["limits"];
        assert_eq!(limits["codeBytes"].as_u64(), Some(TYPED_OPERATION_FAULT_CODE_BYTES as u64));
        assert_eq!(limits["messageBytes"].as_u64(), Some(TYPED_OPERATION_FAULT_BYTES as u64));
        assert_eq!(limits["pageBytes"].as_u64(), Some(TYPED_OPERATION_RESULT_PAGE_BYTES as u64));
        assert_eq!(limits["untypedCode"].as_str(), Some(TYPED_OPERATION_UNTYPED_FAULT_CODE));
        for case in fixture["records"].as_array().expect("records") {
            let name = case["name"].as_str().expect("case name");
            let fault = Fault::new(origin(&case["fault"]), FaultCode::new(spelled(&case["fault"], "code")), spelled(&case["fault"], "message"));
            let record = TypedOperationFault::of_fault(&fault);
            assert_eq!(record, expected(&case["record"]), "{name}");
            let bytes = record.encode();
            assert!(bytes.len() <= TYPED_OPERATION_RESULT_PAGE_BYTES, "{name}: {} encoded bytes", bytes.len());
            assert_eq!(TypedOperationFault::decode(&bytes), Some(record.clone()), "{name}: the wire bytes decode to the record");
            assert_eq!(ArtifactBoundedToolFault::from_fault(&fault).record(), record, "{name}: the retained owner publishes the same record");
        }
        for case in fixture["decodes"].as_array().expect("decodes") {
            let name = case["name"].as_str().expect("case name");
            let decoded = TypedOperationFault::decode(case["bytes"].as_str().expect("bytes").as_bytes());
            assert_eq!(decoded, (!case["record"].is_null()).then(|| expected(&case["record"])), "{name}");
        }
    }
}
