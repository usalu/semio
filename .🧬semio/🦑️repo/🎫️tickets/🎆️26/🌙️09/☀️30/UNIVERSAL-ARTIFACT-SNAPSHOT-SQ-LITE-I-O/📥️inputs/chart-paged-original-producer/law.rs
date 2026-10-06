#[test]
fn paged_chart_original_operation_keeps_exact_variants_policy_and_source_owner(){
    use protocol::operation_bytes::{OwnedOperationBytes,OperationBytePreparation,OperationByteMeasurement,OperationByteCloseStep};
    use semio_framework_value::{NativeEncodeControl,NativeDecodeControl,Number};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🧬️chart-mutations/📦️operation-pages.json")).unwrap();
    let payload=fixture["word"].as_str().unwrap().repeat(fixture["payloadBytes"].as_u64().unwrap()as usize);
    let operation=ChangeChartValue{path:serde_json::from_value(fixture["path"].clone()).unwrap(),value:Some(DslValue::Array(vec![DslValue::String(payload),DslValue::uint(u64::MAX),DslValue::int(i64::MIN),DslValue::float(-0.0)]))};
    let expected=protocol::OpBinary::encode_op(&operation).unwrap();
    let header:Vec<u8>=serde_json::from_value(fixture["header"].clone()).unwrap();assert!(expected.starts_with(&header));
    let metadata=fixture["metadataBytes"].as_u64().unwrap()as usize;
    let allocation=fixture["allocationBytes"].as_u64().unwrap()as usize;
    let items=fixture["maximumCloseItems"].as_u64().unwrap()as usize;
    let bytes=fixture["maximumCloseBytes"].as_u64().unwrap()as usize;
    let close=|owner:&mut OwnedOperationBytes|{let mut released=0;for _ in 0..fixture["maximumCloseSteps"].as_u64().unwrap(){match owner.close_one(items,bytes).unwrap(){OperationByteCloseStep::Complete=>break,OperationByteCloseStep::Pending{released_items,released_bytes}=>{assert!(released_items<=items);assert!(released_bytes<=bytes);released+=released_bytes;}}}assert!(owner.terminal_is_empty());assert_eq!(owner.allocated_bytes(),0);released};
    let mut options=protocol::codec::PackEncodeOptions::default();options.limits.max_file_len=expected.len()as u64;
    let mut allow=|_|true;let mut encoding=NativeEncodeControl::new(allocation,&mut allow);
    let mut measure=OperationByteMeasurement::new(options.limits.max_file_len);
    encoding.scoped_maximum(metadata,|control|Ok::<_,semio_framework_value::ValueError>(operation.encode_op_into(&options,&mut measure,control))).unwrap().unwrap();
    assert_eq!(measure.exact_length().unwrap(),expected.len());assert!(encoding.owned_bytes()<=metadata);
    let mut preparation=OperationBytePreparation::try_new(expected.len(),allocation).unwrap();
    for _ in 0..fixture["maximumCloseSteps"].as_u64().unwrap(){preparation.fund_one(items,bytes,&mut encoding).unwrap();if preparation.is_funded(){break;}}
    assert!(preparation.is_funded());let paid=encoding.owned_bytes();let backing=preparation.allocated_bytes();
    encoding.scoped_maximum(paid+metadata,|control|Ok::<_,semio_framework_value::ValueError>(operation.encode_op_into(&options,&mut preparation,control))).unwrap().unwrap();
    assert_eq!(preparation.allocated_bytes(),backing);assert!(encoding.owned_bytes()-paid<=metadata);
    let mut source=preparation.take_ready().unwrap();assert!(source.iter().eq(expected.iter().copied()));
    assert_eq!(serde_json::to_value(&source).unwrap(),serde_json::to_value(&expected).unwrap());
    let mut allow=|_|true;let mut decoding=NativeDecodeControl::new(allocation,&mut allow);
    let mut allow=|_|true;let mut canonical=NativeEncodeControl::new(metadata,&mut allow);
    let decoded=ChangeChartValue::decode_op_span(protocol::ByteSpan::from_source(&source),&Default::default(),&options,&mut decoding,&mut canonical).unwrap();
    assert_eq!(decoded,operation);
    let Some(DslValue::Array(values))=decoded.value else{panic!("original Chart array lost")};
    assert!(matches!(values[1],DslValue::Number(Number::UInt(u64::MAX))));assert!(matches!(values[2],DslValue::Number(Number::Int(i64::MIN))));
    let DslValue::Number(Number::Float(number))=values[3] else{panic!("exact Chart float variant lost")};assert_eq!(number.to_bits(),(-0.0f64).to_bits());
    assert_eq!(source.close_one(0,bytes).unwrap(),OperationByteCloseStep::Pending{released_items:0,released_bytes:0});assert_eq!(source.len(),expected.len());assert!(close(&mut source)>=expected.len());
    let mut short=options.clone();short.limits.max_file_len-=1;
    let mut prefix=OwnedOperationBytes::try_new(expected.len(),allocation).unwrap();
    let mut allow=|_|true;let mut encoding=NativeEncodeControl::new(allocation,&mut allow);
    assert_eq!(operation.encode_op_into(&short,&mut prefix,&mut encoding).unwrap_err().kind(),semio_framework_value::ValueRefusalKind::OwnershipLimit);
    assert!(prefix.len()<expected.len());assert!(prefix.iter().eq(expected[..prefix.len()].iter().copied()));close(&mut prefix);
    let mut prefix=OwnedOperationBytes::try_new(expected.len(),allocation).unwrap();
    let mut cancel=|work|work<1024;let mut encoding=NativeEncodeControl::new(allocation,&mut cancel);
    assert_eq!(operation.encode_op_into(&options,&mut prefix,&mut encoding).unwrap_err().kind(),semio_framework_value::ValueRefusalKind::Cancelled);
    assert!(prefix.len()<expected.len());assert!(prefix.iter().eq(expected[..prefix.len()].iter().copied()));close(&mut prefix);
    println!("[DEBUG] Original Chart8194 text source preserves [1,1], UInt/Int/-0float and whole-frame caller policy under512 metadata with exact refusal prefix and fixed4096 terminal page return");
}
