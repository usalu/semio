fn settle_paged_dsl_capsule<T:semio_framework_dsl_record::native_encoding::FieldProjectionSource>(capsule:&mut crate::os_pack::record::BorrowedProjectedPackOperation<T>){
 let backing=capsule.allocated_bytes();let(zero,requested,released)=crate::test_allocation::observe_backing(||capsule.retire_one(0,4096).unwrap());assert_eq!((zero,requested,released),((false,0,0),0,0));assert_eq!(capsule.allocated_bytes(),backing);
 let mut disposed=0;for _ in 0..32768{let(step,requested,released)=crate::test_allocation::observe_backing(||capsule.retire_one(1,4096).unwrap());assert_eq!(requested,0);assert_eq!(released,step.2);assert!(step.1<=1&&released<=4096);disposed+=released;if !step.0{break}}assert_eq!(disposed,backing);assert_eq!(capsule.allocated_bytes(),0);assert_eq!(capsule.variant_identity().unwrap_err().kind(),semio_framework_value::ValueRefusalKind::InvariantViolated);
}
fn take_paged_dsl_decoded<T:DslVariants+semio_framework_dsl_record::BorrowedDslVariants>(owner:&mut Option<variants_binary::RetainedDecodedOperation<T>>)->Option<T>{
 let mut capsule=owner.take()?;settle_paged_dsl_capsule(&mut capsule);match capsule.take_settled_source(){Ok(source)=>Some(source),Err(retained)=>{*owner=Some(retained);panic!("decoded source cannot leave unsettled canonical scratch")}}
}

#[semio_framework_async_macros::async_test]
async fn paged_dsl_operation_keeps_exact_header_whole_8194_text_and_refused_prefix() {
    use protocol::io::binary::operation_bytes::{OwnedOperationBytes,OperationByteCloseStep};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/📦️operation-pages/🔣️.json")).unwrap();
    let payload=fixture["word"].as_str().unwrap().repeat(fixture["payloadBytes"].as_u64().unwrap()as usize);
    let operation=DerivedMutation::SetCategory{category:payload.clone()};
    let mut expected:Vec<u8>=fixture["prefix"].as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap()as u8).collect();
    expected.extend_from_slice(payload.as_bytes());
    assert_eq!(variants_binary::encode_op(&operation).unwrap(),expected);
    let close=|source:&mut OwnedOperationBytes|{
        assert_ne!(source.close_one(0,4096).unwrap(),OperationByteCloseStep::Complete);
        let retained=source.allocated_bytes();
        source.close_one(1,0).unwrap();
        assert_eq!(source.allocated_bytes(),retained);
        for _ in 0..fixture["maximumCloseSteps"].as_u64().unwrap(){
            if source.close_one(1,fixture["maximumPhysicalPageBytes"].as_u64().unwrap()as usize).unwrap()==OperationByteCloseStep::Complete{break;}
        }
        assert!(source.terminal_is_empty());
        assert_eq!(source.allocated_bytes(),0);
    };
    let mut capsule=crate::os_pack::record::BorrowedProjectedPackOperation::from_variant(&operation);
    let mut source=OwnedOperationBytes::try_new(16384,65536).unwrap();
    let mut allow=|_|true;
    let mut control=semio_framework_value::NativeEncodeControl::new(131072,&mut allow);
    variants_binary::encode_op_into(&mut capsule,&Default::default(),&mut source,&mut control).unwrap();
    assert!(source.iter().eq(expected.iter().copied()));
    assert_eq!(serde_json::to_value(&source).unwrap(),serde_json::to_value(&expected).unwrap());
    assert_eq!(serde_json::to_value(&operation).unwrap(),serde_json::json!({"SetCategory":{"category":payload}}));
    settle_paged_dsl_capsule(&mut capsule);
    close(&mut source);
    let mut capsule=crate::os_pack::record::BorrowedProjectedPackOperation::from_variant(&operation);
    let mut source=OwnedOperationBytes::try_new(16384,65536).unwrap();
    struct EmissionProbe<'a>{source:&'a mut OwnedOperationBytes,entered:&'a std::cell::Cell<bool>,length:usize}
    impl protocol::io::binary::operation_bytes::OperationByteOutput for EmissionProbe<'_>{
        fn write_bytes(&mut self,bytes:&[u8],control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(),crate::os_pack::PackRefusal>{
            if bytes.len()==self.length{self.entered.set(true);}
            protocol::io::binary::operation_bytes::OperationByteOutput::write_bytes(self.source,bytes,control)
        }
    }
    let entered=std::cell::Cell::new(false);
    let mut cancel=|progress:semio_framework_value::native_encoding::NativeEncodeProgress| !(entered.get()&&progress.total==payload.len()&&progress.completed>=256);
    let mut control=semio_framework_value::NativeEncodeControl::new(131072,&mut cancel);
    let error={let mut probe=EmissionProbe{source:&mut source,entered:&entered,length:payload.len()};variants_binary::encode_op_into(&mut capsule,&Default::default(),&mut probe,&mut control).unwrap_err()};
    let crate::os_spr::ProtocolError::Pack(crate::os_pack::PackError::Refusal(refusal))=error else{panic!("typed cancellation cause lost")};
    assert_eq!(refusal.kind().as_str(),fixture["canceledKind"].as_str().unwrap());
    assert!(source.len()>=256&&source.len()<expected.len());
    assert!(source.iter().eq(expected[..source.len()].iter().copied()));
    assert_eq!(operation,DerivedMutation::SetCategory{category:payload});
    settle_paged_dsl_capsule(&mut capsule);
    close(&mut source);
    println!("[DEBUG] Derived DSL operation preserves full 8194 text, exact ordinal header and refused prefix until fixed 4096 backing retirement");
}

#[semio_framework_async_macros::async_test]
async fn paged_dsl_operation_decodes_same_8194_source_and_checks_complete_canonical_frame(){
    use protocol::io::binary::operation_bytes::{OwnedOperationBytes,OperationByteCloseStep,OperationByteOutput};
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
        let mut capsule=crate::os_pack::record::BorrowedProjectedPackOperation::from_variant(&operation);
        let mut source=OwnedOperationBytes::try_new(16384,65536).unwrap();
        let mut allow=|_|true;
        let mut encoding=semio_framework_value::NativeEncodeControl::new(131072,&mut allow);
        if noncanonical{
            source.write_bytes(&[1,128,0],&mut encoding).unwrap();
            source.write_bytes(&expected[2..],&mut encoding).unwrap();
        }else{variants_binary::encode_op_into(&mut capsule,&Default::default(),&mut source,&mut encoding).unwrap();}
        let retained=source.allocated_bytes();
        let mut allow=|_|true;
        let mut decoding=semio_framework_value::NativeDecodeControl::new(131072,&mut allow);
        let mut allow=|_|true;
        let mut encoding=semio_framework_value::NativeEncodeControl::new(131072,&mut allow);
        let mut canonical_owner=None;
        let decoded=variants_binary::decode_op_span::<DerivedMutation>(crate::os_pack::ByteSpan::from_source(&source),&Default::default(),&Default::default(),&mut decoding,&mut encoding,&mut canonical_owner);
        if noncanonical{
            let crate::os_spr::ProtocolError::Pack(crate::os_pack::PackError::Refusal(refusal))=decoded.unwrap_err()else{panic!("canonical mismatch cause lost")};
            assert_eq!(refusal.kind(),semio_framework_value::ValueRefusalKind::InvalidValue);
        }else{
            decoded.unwrap();
            assert_eq!(canonical_owner.as_ref().unwrap().retained_source(),&operation);
            assert!(source.iter().eq(expected.iter().copied()));
        }
        assert_eq!(source.allocated_bytes(),retained);
        assert_eq!(take_paged_dsl_decoded(&mut canonical_owner).unwrap(),operation);
        let mut options=crate::os_pack::DecodeOptions::default();
        options.limits.max_file_len=128;
        let refusal=variants_binary::decode_op_span::<DerivedMutation>(crate::os_pack::ByteSpan::from_source(&source),&options,&Default::default(),&mut decoding,&mut encoding,&mut canonical_owner).unwrap_err();
        let crate::os_spr::ProtocolError::Pack(crate::os_pack::PackError::Refusal(refusal))=refusal else{panic!("caller source policy lost")};
        assert_eq!(refusal.kind(),semio_framework_value::ValueRefusalKind::OwnershipLimit);
        assert_eq!(source.allocated_bytes(),retained);
        settle_paged_dsl_capsule(&mut capsule);
        close(&mut source);
    }
    let mut capsule=crate::os_pack::record::BorrowedProjectedPackOperation::from_variant(&operation);
    let mut source=OwnedOperationBytes::try_new(16384,65536).unwrap();
    let mut options=crate::os_pack::EncodeOptions::default();
    options.limits.max_file_len=128;
    let mut allow=|_|true;
    let mut encoding=semio_framework_value::NativeEncodeControl::new(131072,&mut allow);
    let refusal=variants_binary::encode_op_into(&mut capsule,&options,&mut source,&mut encoding).unwrap_err();
    let crate::os_spr::ProtocolError::Pack(crate::os_pack::PackError::Refusal(refusal))=refusal else{panic!("caller output policy lost")};
    assert_eq!(refusal.kind(),semio_framework_value::ValueRefusalKind::WorkLimit);
    assert!(source.iter().eq([1,0]));
    settle_paged_dsl_capsule(&mut capsule);
    close(&mut source);
    println!("[DEBUG] Paged DSL decoder borrows exact full 8194 source, rejects redundant ordinal varint and applies genuine caller limits with fixed 4096 terminal retirement");
}

#[semio_framework_async_macros::async_test]
async fn paged_dsl_operation_whole_frame_policy_includes_exact_header_and_admitted_body(){
    use protocol::io::binary::operation_bytes::{OwnedOperationBytes,OperationByteCloseStep};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/📦️operation-pages/🔣️.json")).unwrap();
    let payload=fixture["word"].as_str().unwrap().repeat(fixture["payloadBytes"].as_u64().unwrap()as usize);
    let operation=DerivedMutation::SetCategory{category:payload.clone()};
    let mut expected:Vec<u8>=fixture["prefix"].as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap()as u8).collect();
    expected.extend_from_slice(payload.as_bytes());
    assert_eq!(expected.len(),fixture["wholeOperationBytes"].as_u64().unwrap()as usize);
    for maximum in [expected.len(),expected.len()-1]{
        let mut capsule=crate::os_pack::record::BorrowedProjectedPackOperation::from_variant(&operation);
        let mut source=OwnedOperationBytes::try_new(16384,65536).unwrap();
        let mut options=crate::os_pack::EncodeOptions::default();
        options.limits.max_file_len=maximum as u64;
        let mut allow=|_|true;
        let mut encoding=semio_framework_value::NativeEncodeControl::new(131072,&mut allow);
        let result=variants_binary::encode_op_into(&mut capsule,&options,&mut source,&mut encoding);
        let length=source.len();
        let exact=source.iter().eq(expected[..length.min(expected.len())].iter().copied());
        let decoded=if result.is_ok()&&maximum==expected.len(){
            let mut decoding_options=crate::os_pack::DecodeOptions::default();decoding_options.limits.max_file_len=maximum as u64;
            let mut allow=|_|true;let mut decoding=semio_framework_value::NativeDecodeControl::new(131072,&mut allow);
            let mut allow=|_|true;let mut encoding=semio_framework_value::NativeEncodeControl::new(131072,&mut allow);
            let mut canonical_owner=None;
            let result=variants_binary::decode_op_span::<DerivedMutation>(crate::os_pack::ByteSpan::from_source(&source),&decoding_options,&options,&mut decoding,&mut encoding,&mut canonical_owner);
            let decoded=take_paged_dsl_decoded(&mut canonical_owner);
            Some(result.map(|()|decoded.unwrap()))
        }else{None};
        settle_paged_dsl_capsule(&mut capsule);
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

#[semio_framework_async_macros::async_test]
async fn paged_dsl_operation_store_intrinsic_bridge_keeps_actual_field_and_source_owner(){
    use protocol::io::binary::operation_bytes::{OwnedOperationBytes,OperationByteCloseStep,OperationByteOutput};
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

#[semio_framework_async_macros::async_test]
async fn paged_dsl_operation_original_variant_borrows8194_payload_under512_metadata_admission(){
    use protocol::io::binary::operation_bytes::{OperationByteMeasurement,OperationBytePreparation,OperationByteCloseStep};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/📦️operation-pages/🔣️.json")).unwrap();
    let payload=fixture["word"].as_str().unwrap().repeat(fixture["payloadBytes"].as_u64().unwrap()as usize);
    let operation=DerivedMutation::SetCategory{category:payload};
    let mut expected:Vec<u8>=fixture["prefix"].as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap()as u8).collect();
    let DerivedMutation::SetCategory{category}=&operation else{unreachable!()};expected.extend_from_slice(category.as_bytes());
    assert_eq!(variants_binary::encode_op(&operation).unwrap(),expected);
    let mut options=crate::os_pack::EncodeOptions::default();options.limits.max_file_len=expected.len()as u64;
    let mut allow=|_|true;let mut encoding=semio_framework_value::NativeEncodeControl::new(65536,&mut allow);
    let mut capsule=crate::os_pack::record::BorrowedProjectedPackOperation::from_variant(&operation);
    let mut measurement=OperationByteMeasurement::new(options.limits.max_file_len);
    encoding.scoped_maximum(512,|encoding|Ok::<_,semio_framework_value::ValueError>(variants_binary::encode_op_into(&mut capsule,&options,&mut measurement,encoding))).unwrap().unwrap();
    assert_eq!(measurement.exact_length().unwrap(),expected.len());assert!(encoding.owned_bytes()<512);
    let mut prepared=OperationBytePreparation::try_new(expected.len(),65536).unwrap();
    for _ in 0..fixture["maximumCloseSteps"].as_u64().unwrap(){prepared.fund_one(1,4096,&mut encoding).unwrap();if prepared.is_funded(){break;}}
    assert!(prepared.is_funded());let paid=encoding.owned_bytes();let backing=prepared.allocated_bytes();
    encoding.scoped_maximum(paid+512,|encoding|Ok::<_,semio_framework_value::ValueError>(variants_binary::encode_op_into(&mut capsule,&options,&mut prepared,encoding))).unwrap().unwrap();
    assert_eq!(prepared.allocated_bytes(),backing);assert!(encoding.owned_bytes()-paid<512);
    let mut source=prepared.take_ready().unwrap();assert!(source.iter().eq(expected.iter().copied()));assert_eq!(serde_json::to_value(&source).unwrap(),serde_json::to_value(&expected).unwrap());
    let mut allow=|_|true;let mut decoding=semio_framework_value::NativeDecodeControl::new(65536,&mut allow);
    let mut allow=|_|true;let mut canonical=semio_framework_value::NativeEncodeControl::new(512,&mut allow);
    let mut canonical_owner=None;
    assert_eq!(variants_binary::decode_op_span::<DerivedMutation>(crate::os_pack::ByteSpan::from_source(&source),&Default::default(),&options,&mut decoding,&mut canonical,&mut canonical_owner).unwrap(),());assert_eq!(take_paged_dsl_decoded(&mut canonical_owner).unwrap(),operation);
    settle_paged_dsl_capsule(&mut capsule);
    for _ in 0..fixture["maximumCloseSteps"].as_u64().unwrap(){if source.close_one(1,4096).unwrap()==OperationByteCloseStep::Complete{break;}}
    assert!(source.terminal_is_empty());assert_eq!(source.allocated_bytes(),0);
    println!("[DEBUG] Original declared DerivedMutation borrows8194 category directly through512 metadata, exact full protocol frame, same paid source/control and bounded4096 terminal release; same-source canonical decoder owns genuine typed value");
}