
#[semio_framework_async_macros::async_test]
async fn paged_dsl_operation_whole_frame_policy_includes_exact_header_and_admitted_body(){
    use crate::os_spr::operation_bytes::{OwnedOperationBytes,OperationByteCloseStep};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/📦️operation-pages/🔣️.json")).unwrap();
    let payload=fixture["word"].as_str().unwrap().repeat(fixture["payloadBytes"].as_u64().unwrap()as usize);
    let operation=DerivedMutation::SetCategory{category:payload.clone()};
    let mut expected:Vec<u8>=fixture["prefix"].as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap()as u8).collect();
    expected.extend_from_slice(payload.as_bytes());
    assert_eq!(expected.len(),fixture["wholeOperationBytes"].as_u64().unwrap()as usize);
    for maximum in [expected.len(),expected.len()-1]{
        let mut source=OwnedOperationBytes::try_new(16384,65536).unwrap();
        let mut options=crate::os_pack::EncodeOptions::default();
        options.limits.max_file_len=maximum as u64;
        let mut allow=|_|true;
        let mut encoding=semio_framework_value::NativeEncodeControl::new(131072,&mut allow);
        let result=variants_binary::encode_op_into(&operation,&options,&mut source,&mut encoding);
        let length=source.len();
        let exact=source.iter().eq(expected[..length.min(expected.len())].iter().copied());
        let decoded=if result.is_ok()&&maximum==expected.len(){
            let mut decoding_options=crate::os_pack::DecodeOptions::default();decoding_options.limits.max_file_len=maximum as u64;
            let mut allow=|_|true;let mut decoding=semio_framework_value::NativeDecodeControl::new(131072,&mut allow);
            let mut allow=|_|true;let mut encoding=semio_framework_value::NativeEncodeControl::new(131072,&mut allow);
            Some(variants_binary::decode_op_span::<DerivedMutation>(crate::os_pack::ByteSpan::from_source(&source),&decoding_options,&options,&mut decoding,&mut encoding))
        }else{None};
        for _ in 0..fixture["maximumCloseSteps"].as_u64().unwrap(){if source.close_one(1,4096).unwrap()==OperationByteCloseStep::Complete{break;}}
        assert!(source.terminal_is_empty());assert_eq!(source.allocated_bytes(),0);
        assert!(exact);
        if maximum==expected.len(){result.unwrap();assert_eq!(length,expected.len());assert_eq!(decoded.unwrap().unwrap(),operation);}else{
            let crate::os_spr::ProtocolError::Pack(crate::os_pack::PackError::Refusal(refusal))=result.expect_err("one-byte-short whole operation policy must refuse including its header")else{panic!("whole operation ceiling cause lost")};
            assert_eq!(refusal.kind(),semio_framework_value::ValueRefusalKind::OwnershipLimit);
            assert!(length<=maximum&&length<expected.len());
        }
    }
    assert_eq!(serde_json::to_value(&operation).unwrap(),serde_json::json!({"SetCategory":{"category":payload}}));
    println!("[DEBUG] Exact whole operation limit admits header plus complete 8194 source; one-byte-short authority refuses before excess handoff and preserves same owned prefix for fixed 4096 release");
}
