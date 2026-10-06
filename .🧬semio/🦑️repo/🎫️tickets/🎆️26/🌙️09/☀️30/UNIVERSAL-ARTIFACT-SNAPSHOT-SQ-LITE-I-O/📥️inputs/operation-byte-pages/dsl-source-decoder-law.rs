
#[semio_framework_async_macros::async_test]
async fn paged_dsl_operation_decodes_same_8194_source_and_checks_complete_canonical_frame(){
    use crate::os_spr::operation_bytes::{OwnedOperationBytes,OperationByteCloseStep,OperationByteOutput};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/📦️operation-pages/🔣️.json")).unwrap();
    let payload=fixture["word"].as_str().unwrap().repeat(fixture["payloadBytes"].as_u64().unwrap()as usize);
    let operation=DerivedMutation::SetCategory{category:payload.clone()};
    let mut expected:Vec<u8>=fixture["prefix"].as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap()as u8).collect();
    expected.extend_from_slice(payload.as_bytes());
    let close=|source:&mut OwnedOperationBytes|{
        for _ in 0..fixture["maximumCloseSteps"].as_u64().unwrap(){if source.close_one(1,4096).unwrap()==OperationByteCloseStep::Complete{break;}}
        assert!(source.terminal_is_empty());
        assert_eq!(source.allocated_bytes(),0);
    };
    for noncanonical in [false,true]{
        let mut source=OwnedOperationBytes::try_new(16384,65536).unwrap();
        let mut allow=|_|true;
        let mut encoding=semio_framework_value::NativeEncodeControl::new(131072,&mut allow);
        if noncanonical{
            source.write_bytes(&[1,128,0],&mut encoding).unwrap();
            source.write_bytes(&expected[2..],&mut encoding).unwrap();
        }else{variants_binary::encode_op_into(&operation,&Default::default(),&mut source,&mut encoding).unwrap();}
        let retained=source.allocated_bytes();
        let mut allow=|_|true;
        let mut decoding=semio_framework_value::NativeDecodeControl::new(131072,&mut allow);
        let mut allow=|_|true;
        let mut encoding=semio_framework_value::NativeEncodeControl::new(131072,&mut allow);
        let decoded=variants_binary::decode_op_span::<DerivedMutation>(crate::os_pack::ByteSpan::from_source(&source),&Default::default(),&Default::default(),&mut decoding,&mut encoding);
        if noncanonical{
            let crate::os_spr::ProtocolError::Pack(crate::os_pack::PackError::Refusal(refusal))=decoded.unwrap_err()else{panic!("canonical mismatch cause lost")};
            assert_eq!(refusal.kind(),semio_framework_value::ValueRefusalKind::InvalidValue);
        }else{
            assert_eq!(decoded.unwrap(),operation);
            assert!(source.iter().eq(expected.iter().copied()));
        }
        assert_eq!(source.allocated_bytes(),retained);
        let mut options=crate::os_pack::DecodeOptions::default();
        options.limits.max_file_len=128;
        let refusal=variants_binary::decode_op_span::<DerivedMutation>(crate::os_pack::ByteSpan::from_source(&source),&options,&Default::default(),&mut decoding,&mut encoding).unwrap_err();
        let crate::os_spr::ProtocolError::Pack(crate::os_pack::PackError::Refusal(refusal))=refusal else{panic!("caller source policy lost")};
        assert_eq!(refusal.kind(),semio_framework_value::ValueRefusalKind::OwnershipLimit);
        assert_eq!(source.allocated_bytes(),retained);
        close(&mut source);
    }
    let mut source=OwnedOperationBytes::try_new(16384,65536).unwrap();
    let mut options=crate::os_pack::EncodeOptions::default();
    options.limits.max_file_len=128;
    let mut allow=|_|true;
    let mut encoding=semio_framework_value::NativeEncodeControl::new(131072,&mut allow);
    let refusal=variants_binary::encode_op_into(&operation,&options,&mut source,&mut encoding).unwrap_err();
    let crate::os_spr::ProtocolError::Pack(crate::os_pack::PackError::Refusal(refusal))=refusal else{panic!("caller output policy lost")};
    assert_eq!(refusal.kind(),semio_framework_value::ValueRefusalKind::OwnershipLimit);
    assert!(source.iter().eq([1,0]));
    close(&mut source);
    println!("[DEBUG] Paged DSL decoder borrows exact full 8194 source, rejects redundant ordinal varint and applies genuine caller limits with fixed 4096 terminal retirement");
}
