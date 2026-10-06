
#[semio_framework_async_macros::async_test]
async fn paged_dsl_operation_keeps_exact_header_whole_8194_text_and_refused_prefix() {
    use crate::os_spr::mutation::bytes::{OwnedOperationBytes,OperationByteCloseStep};
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
    let mut source=OwnedOperationBytes::try_new(16384,65536).unwrap();
    let mut allow=|_|true;
    let mut control=semio_framework_value::NativeEncodeControl::new(131072,&mut allow);
    variants_binary::encode_op_into(&operation,&mut source,&mut control).unwrap();
    assert!(source.iter().eq(expected.iter().copied()));
    assert_eq!(serde_json::to_value(&source).unwrap(),serde_json::to_value(&expected).unwrap());
    assert_eq!(serde_json::to_value(&operation).unwrap(),serde_json::json!({"SetCategory":{"category":payload}}));
    close(&mut source);
    let mut source=OwnedOperationBytes::try_new(16384,65536).unwrap();
    let mut cancel=|progress:semio_framework_value::native_encoding::NativeEncodeProgress| !(progress.total==payload.len()&&progress.completed>=256);
    let mut control=semio_framework_value::NativeEncodeControl::new(131072,&mut cancel);
    let error=variants_binary::encode_op_into(&operation,&mut source,&mut control).unwrap_err();
    assert_eq!(error.kind().as_str(),fixture["canceledKind"].as_str().unwrap());
    assert!(source.len()>0&&source.len()<expected.len());
    assert!(source.iter().eq(expected[..source.len()].iter().copied()));
    assert_eq!(operation,DerivedMutation::SetCategory{category:payload});
    close(&mut source);
    println!("[DEBUG] Derived DSL operation preserves full 8194 text, exact ordinal header and refused prefix until fixed 4096 backing retirement");
}
