use super::*;

fn finish(owner: &mut protocol::mutation::bytes::OwnedOperationBytes) {
    for _ in 0..17000 {
        if owner.close_one(1, 4096).unwrap() == protocol::mutation::bytes::OperationByteCloseStep::Complete { break; }
    }
    assert!(owner.terminal_is_empty());
    assert_eq!(owner.allocated_bytes(), 0);
}

#[test]
fn record_operation_pages_emit_exact_canonical_words_and_keep_canceled_prefix_owned() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📦️operation-pages/🔣️.json")).unwrap();
    let spec = RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(1, "bytes", Shape::Bytes64), FieldSpec::new(2, "numbers", Shape::Value)]);
    let mut record = RecordValue::default();
    let payload: Vec<u8> = (0..fixture["payloadBytes"].as_u64().unwrap()).map(|index| (index % 251) as u8).collect();
    record.fields.insert(1, FieldValue::Bytes64(payload.clone()));
    record.fields.insert(2, FieldValue::Value(DslValue::Array(vec![DslValue::uint(u64::MAX), DslValue::int(i64::MIN), DslValue::float(-0.0)])));
    record.fields.insert(99, FieldValue::UInt(7));
    for preserve_unknown in [true, false] {
        let mut options = EncodeOptions::default();
        options.preserve_unknown = preserve_unknown;
        let expected = encode_record_body(&spec, &record, &options).unwrap();
        let literal = |key: &str| fixture[key].as_array().unwrap().iter().map(|byte| byte.as_u64().unwrap() as u8).collect::<Vec<_>>();
        let mut independent = literal(if preserve_unknown { "preservedPrefix" } else { "knownPrefix" });
        independent.extend_from_slice(&payload);
        independent.extend(literal("numberSuffix"));
        if preserve_unknown { independent.extend(literal("unknownSuffix")); }
        assert_eq!(expected, independent);
        let mut output = protocol::mutation::bytes::OwnedOperationBytes::try_new(16384, 65536).unwrap();
        let mut allow = |_| true;
        let mut control = semio_framework_value::NativeEncodeControl::new(65536, &mut allow);
        let length = encode_record_body_into(&spec, &record, &options, &mut output, &mut control).unwrap();
        assert_eq!(length, expected.len());
        assert!(output.iter().eq(expected.iter().copied()));
        assert_eq!(serde_json::to_value(&output).unwrap(), serde_json::to_value(&expected).unwrap());
        let (decoded, report) = decode_record_body(&expected, &spec, &DecodeOptions::default()).unwrap();
        let mut expected_record = record.clone();
        if !preserve_unknown { expected_record.fields.remove(&99); }
        assert_eq!(decoded, expected_record);
        assert_eq!(report.unknown_field_ids, if preserve_unknown { vec![99] } else { vec![] });
        let FieldValue::Value(DslValue::Array(numbers)) = decoded.fields.get(&2).unwrap() else { panic!("exact number array lost") };
        assert!(matches!(numbers[0], DslValue::Number(Number::UInt(u64::MAX))));
        assert!(matches!(numbers[1], DslValue::Number(Number::Int(i64::MIN))));
        let DslValue::Number(Number::Float(word)) = numbers[2] else { panic!("exact float variant lost") };
        assert_eq!(word.to_bits(), 0x8000000000000000);
        finish(&mut output);

        let mut output = protocol::mutation::bytes::OwnedOperationBytes::try_new(16384, 65536).unwrap();
        let mut allow = |_| true;
        let mut control = semio_framework_value::NativeEncodeControl::new(65536, &mut allow);
        protocol::mutation::bytes::OperationByteOutput::write_bytes(&mut output, &[1, 7], &mut control).unwrap();
        assert_eq!(encode_record_body_into(&spec, &record, &options, &mut output, &mut control).unwrap(), expected.len());
        assert!(output.iter().eq([1, 7].into_iter().chain(expected.iter().copied())));
        finish(&mut output);

        let mut output = protocol::mutation::bytes::OwnedOperationBytes::try_new(16384, 65536).unwrap();
        let mut cancel = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| !(progress.total == payload.len() && progress.completed >= 256);
        let mut control = semio_framework_value::NativeEncodeControl::new(65536, &mut cancel);
        let error = encode_record_body_into(&spec, &record, &options, &mut output, &mut control).unwrap_err();
        let PackRefusal::ValueRefusal(refusal) = error else { panic!("cancellation category lost") };
        assert_eq!(refusal.kind.as_str(), fixture["canceledKind"].as_str().unwrap());
        assert!(output.len() >= 256 && output.len() < expected.len());
        assert!(output.iter().eq(expected[..output.len()].iter().copied()));
        assert_eq!(record.fields.get(&1), Some(&FieldValue::Bytes64(payload.clone())));
        finish(&mut output);
    }
    println!("[DEBUG] Core streamed canonical Record bytes directly into source-owned pages; cancellation retains the exact emitted prefix until actual bounded terminal retirement");
}
