
#[semio_framework_async_macros::async_test]
async fn paged_dsl_operation_store_intrinsic_bridge_keeps_actual_field_and_source_owner(){
    use crate::os_spr::operation_bytes::{OwnedOperationBytes,OperationByteCloseStep,OperationByteOutput};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/📦️operation-pages/🔣️.json")).unwrap();
    let case=&fixture["storeIntrinsic"];
    let payload:Vec<u8>=(0..fixture["payloadBytes"].as_u64().unwrap()).map(|index|(index%251)as u8).collect();
    let value=semio_framework_value::DslValue::Bytes(payload.clone());
    let mut expected:Vec<u8>=case["prefix"].as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap()as u8).collect();
    expected.extend_from_slice(&payload);
    let close=|source:&mut OwnedOperationBytes|{for _ in 0..fixture["maximumCloseSteps"].as_u64().unwrap(){if source.close_one(1,4096).unwrap()==OperationByteCloseStep::Complete{break;}}assert!(source.terminal_is_empty());assert_eq!(source.allocated_bytes(),0);};
    let mut source=OwnedOperationBytes::try_new(16384,65536).unwrap();
    let mut options=crate::os_pack::EncodeOptions::default();options.limits.max_file_len=expected.len()as u64;
    let mut allow=|_|true;let mut encoding=semio_framework_value::NativeEncodeControl::new(131072,&mut allow);
    assert_eq!(crate::os_store::pack_rt::encode_wire_value_into(&value,&options,&mut source,&mut encoding).unwrap(),expected.len());
    assert!(source.iter().eq(expected.iter().copied()));
    assert_eq!(serde_json::to_value(&source).unwrap(),serde_json::to_value(&expected).unwrap());
    let retained=source.allocated_bytes();
    let mut options=crate::os_pack::DecodeOptions::default();options.limits.max_file_len=expected.len()as u64;
    let mut allow=|_|true;let mut decoding=semio_framework_value::NativeDecodeControl::new(131072,&mut allow);
    assert_eq!(crate::os_store::pack_rt::decode_wire_value_span(crate::os_pack::ByteSpan::from_source(&source),&options,&mut decoding).unwrap(),value);
    options.limits.max_file_len-=1;
    assert_eq!(crate::os_store::pack_rt::decode_wire_value_span(crate::os_pack::ByteSpan::from_source(&source),&options,&mut decoding).unwrap_err().kind(),semio_framework_value::ValueRefusalKind::WorkLimit);
    assert_eq!(source.allocated_bytes(),retained);assert!(source.iter().eq(expected.iter().copied()));close(&mut source);
    let mut source=OwnedOperationBytes::try_new(16384,65536).unwrap();
    let mut options=crate::os_pack::EncodeOptions::default();options.limits.max_total_alloc=128;
    let mut allow=|_|true;let mut encoding=semio_framework_value::NativeEncodeControl::new(131072,&mut allow);
    assert_eq!(crate::os_store::pack_rt::encode_wire_value_into(&value,&options,&mut source,&mut encoding).unwrap_err().kind(),semio_framework_value::ValueRefusalKind::OwnershipLimit);
    assert_eq!(source.len(),0);close(&mut source);
    let mut source=OwnedOperationBytes::try_new(16384,65536).unwrap();
    let mut foreign=expected.clone();foreign[2]=case["foreignFieldId"].as_u64().unwrap()as u8;
    let mut allow=|_|true;let mut encoding=semio_framework_value::NativeEncodeControl::new(131072,&mut allow);
    source.write_bytes(&foreign,&mut encoding).unwrap();
    let mut allow=|_|true;let mut decoding=semio_framework_value::NativeDecodeControl::new(131072,&mut allow);
    assert_eq!(crate::os_store::pack_rt::decode_wire_value_span(crate::os_pack::ByteSpan::from_source(&source),&Default::default(),&mut decoding).unwrap_err().kind(),semio_framework_value::ValueRefusalKind::InvalidValue);
    assert!(source.iter().eq(foreign));close(&mut source);
    println!("[DEBUG] Actual Store field1 bridge preserves complete8194 intrinsic octets, exact source backing and explicit caller policies; foreign field refusal retains the same owner through fixed4096 release");
}
