
#[semio_framework_async_macros::async_test]
async fn paged_dsl_operation_original_variant_borrows8194_payload_under512_metadata_admission(){
    use crate::os_spr::operation_bytes::{OperationByteMeasurement,OperationBytePreparation,OperationByteCloseStep};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/📦️operation-pages/🔣️.json")).unwrap();
    let payload=fixture["word"].as_str().unwrap().repeat(fixture["payloadBytes"].as_u64().unwrap()as usize);
    let operation=DerivedMutation::SetCategory{category:payload};
    let mut expected:Vec<u8>=fixture["prefix"].as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap()as u8).collect();
    let DerivedMutation::SetCategory{category}=&operation else{unreachable!()};expected.extend_from_slice(category.as_bytes());
    assert_eq!(variants_binary::encode_op(&operation).unwrap(),expected);
    let mut options=crate::os_pack::EncodeOptions::default();options.limits.max_file_len=expected.len()as u64;
    let mut allow=|_|true;let mut encoding=semio_framework_value::NativeEncodeControl::new(65536,&mut allow);
    let mut measurement=OperationByteMeasurement::new(options.limits.max_file_len);
    encoding.scoped_maximum(512,|encoding|Ok::<_,semio_framework_value::ValueError>(variants_binary::encode_op_into(&operation,&options,&mut measurement,encoding))).unwrap().unwrap();
    assert_eq!(measurement.exact_length().unwrap(),expected.len());assert!(encoding.owned_bytes()<512);
    let mut prepared=OperationBytePreparation::try_new(expected.len(),65536).unwrap();
    for _ in 0..fixture["maximumCloseSteps"].as_u64().unwrap(){prepared.fund_one(1,4096,&mut encoding).unwrap();if prepared.is_funded(){break;}}
    assert!(prepared.is_funded());let paid=encoding.owned_bytes();let backing=prepared.allocated_bytes();
    encoding.scoped_maximum(paid+512,|encoding|Ok::<_,semio_framework_value::ValueError>(variants_binary::encode_op_into(&operation,&options,&mut prepared,encoding))).unwrap().unwrap();
    assert_eq!(prepared.allocated_bytes(),backing);assert!(encoding.owned_bytes()-paid<512);
    let mut source=prepared.take_ready().unwrap();assert!(source.iter().eq(expected.iter().copied()));assert_eq!(serde_json::to_value(&source).unwrap(),serde_json::to_value(&expected).unwrap());
    let mut allow=|_|true;let mut decoding=semio_framework_value::NativeDecodeControl::new(65536,&mut allow);
    let mut allow=|_|true;let mut canonical=semio_framework_value::NativeEncodeControl::new(512,&mut allow);
    assert_eq!(variants_binary::decode_op_span::<DerivedMutation>(crate::os_pack::ByteSpan::from_source(&source),&Default::default(),&options,&mut decoding,&mut canonical).unwrap(),operation);
    for _ in 0..fixture["maximumCloseSteps"].as_u64().unwrap(){if source.close_one(1,4096).unwrap()==OperationByteCloseStep::Complete{break;}}
    assert!(source.terminal_is_empty());assert_eq!(source.allocated_bytes(),0);
    println!("[DEBUG] Original declared DerivedMutation borrows8194 category directly through512 metadata, exact full protocol frame, same paid source/control and bounded4096 terminal release; same-source canonical decoder owns genuine typed value");
}
