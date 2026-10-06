
#[test]
fn intrinsic_operation_pages_emit_and_decode_same_complete_owner_under_actual_policy() {
    use protocol::{ByteSpan, mutation::operation_bytes::OwnedOperationBytes};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/📦️operation-pages/🔣️.json")).unwrap();
    let case=&fixture["intrinsicSource"];
    let payload:Vec<u8>=(0..fixture["payloadBytes"].as_u64().unwrap()).map(|index|(index%251)as u8).collect();
    let value=DslValue::Bytes(payload.clone());
    let mut expected:Vec<u8>=case["prefix"].as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap()as u8).collect();
    expected.extend_from_slice(&payload);
    let mut output=OwnedOperationBytes::try_new(16384,65536).unwrap();
    let mut options=EncodeOptions::default();options.limits.max_file_len=expected.len()as u64;
    let mut allow=|_|true;let mut encoding=semio_framework_value::NativeEncodeControl::new(131072,&mut allow);
    assert_eq!(encode_value_record_body_into(1,&value,&options,&mut output,&mut encoding).unwrap(),expected.len());
    assert!(output.iter().eq(expected.iter().copied()));
    assert_eq!(serde_json::to_value(&output).unwrap(),serde_json::to_value(&expected).unwrap());
    let retained=output.allocated_bytes();
    let mut options=DecodeOptions::default();options.limits.max_file_len=expected.len()as u64;
    let mut allow=|_|true;let mut decoding=semio_framework_value::NativeDecodeControl::new(131072,&mut allow);
    assert_eq!(decode_value_record_body_span_exact_controlled(ByteSpan::from_source(&output),1,&options,&mut decoding).unwrap(),value);
    assert_eq!(output.allocated_bytes(),retained);
    options.limits.max_file_len-=1;
    assert_eq!(decode_value_record_body_span_exact_controlled(ByteSpan::from_source(&output),1,&options,&mut decoding).unwrap_err().kind(),ValueRefusalKind::WorkLimit);
    assert!(output.iter().eq(expected.iter().copied()));finish(&mut output);
    let mut output=OwnedOperationBytes::try_new(16384,65536).unwrap();
    let mut options=EncodeOptions::default();options.limits.max_file_len=expected.len()as u64-1;
    let mut allow=|_|true;let mut encoding=semio_framework_value::NativeEncodeControl::new(131072,&mut allow);
    assert_eq!(encode_value_record_body_into(1,&value,&options,&mut output,&mut encoding).unwrap_err().kind(),ValueRefusalKind::WorkLimit);
    assert_eq!(output.len(),0);finish(&mut output);
    assert_eq!(serde_json::to_value(&payload).unwrap(),serde_json::to_value(&expected[case["prefix"].as_array().unwrap().len()..]).unwrap());
    println!("[DEBUG] Intrinsic Core producer emits exact literal-framed complete8194 owner into paid pages; source decoding keeps the same backing and exact/one-byte-short policy before fixed4096 terminal release");
}
