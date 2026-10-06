#[test]
fn record_operation_pages_decode_cross_page_bytes_and_utf8_without_flattening_the_source() {
    use protocol::{ByteSpan, mutation::bytes::{OwnedOperationBytes, OperationByteOutput}};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/📦️operation-pages/🔣️.json")).unwrap();
    let case=&fixture["sourceRange"];
    let payload:Vec<u8>=(0..case["payloadBytes"].as_u64().unwrap()).map(|index|(index%251)as u8).collect();
    let text=case["utf8Word"].as_str().unwrap().repeat(case["utf8Repetitions"].as_u64().unwrap()as usize);
    let spec=RecordSpec::new(None,RecordLayout::Inline,vec![FieldSpec::new(1,"bytes",Shape::Bytes64),FieldSpec::new(2,"text",Shape::Text)]);
    let mut record=RecordValue::default();
    record.fields.insert(1,FieldValue::Bytes64(payload.clone()));
    record.fields.insert(2,FieldValue::Text(text.clone()));
    let mut source=OwnedOperationBytes::try_new(32768,131072).unwrap();
    let mut allow=|_|true;
    let mut encoding=semio_framework_value::NativeEncodeControl::new(131072,&mut allow);
    encode_record_body_into(&spec,&record,&Default::default(),&mut source,&mut encoding).unwrap();
    let retained=source.allocated_bytes();
    let mut allow=|_|true;
    let mut decoding=semio_framework_value::NativeDecodeControl::new(131072,&mut allow);
    let decoded=decode_record_body_span_exact_controlled(ByteSpan::from_source(&source),&spec,&Default::default(),&mut decoding).unwrap();
    assert_eq!(decoded,record);
    let FieldValue::Bytes64(bytes)=decoded.fields.get(&1).unwrap() else{panic!("complete octet owner lost")};
    let FieldValue::Text(text)=decoded.fields.get(&2).unwrap() else{panic!("complete utf8 owner lost")};
    assert_eq!(serde_json::to_value((bytes,text)).unwrap(),serde_json::json!([payload,case["utf8Word"].as_str().unwrap().repeat(case["utf8Repetitions"].as_u64().unwrap()as usize)]));
    assert_eq!(source.allocated_bytes(),retained);
    finish(&mut source);

    let bad:Vec<u8>=case["malformedUtf8Record"].as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap()as u8).collect();
    let mut source=OwnedOperationBytes::try_new(32,65536).unwrap();
    let mut allow=|_|true;
    let mut encoding=semio_framework_value::NativeEncodeControl::new(65536,&mut allow);
    source.write_bytes(&bad,&mut encoding).unwrap();
    let mut allow=|_|true;
    let mut decoding=semio_framework_value::NativeDecodeControl::new(65536,&mut allow);
    let error=decode_record_body_span_exact_controlled(ByteSpan::from_source(&source),&spec,&Default::default(),&mut decoding).unwrap_err();
    assert_eq!(error.kind().as_str(),case["malformedKind"].as_str().unwrap());
    assert!(source.iter().eq(bad));
    finish(&mut source);
    println!("[DEBUG] Core source-range decoding preserves all cross-page octets and UTF8 words; malformed input retains the same source until bounded terminal retirement");
}
