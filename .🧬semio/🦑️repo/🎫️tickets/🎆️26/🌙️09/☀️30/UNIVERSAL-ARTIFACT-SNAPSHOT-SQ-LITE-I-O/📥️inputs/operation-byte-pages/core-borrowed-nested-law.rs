
#[test]
fn record_operation_pages_borrowed_source_joins_actual_nested_derived_tables_numbers_and_wire(){
    use semio_framework_dsl_record_derive::DslRecord;
    use semio_framework_dsl_record::Wire;
    #[derive(DslRecord)]
    struct Row{active:Option<bool>,name:String,count:u64}
    #[derive(DslRecord)]
    struct Owner{signed:Vec<i64>,unsigned:Vec<u64>,wide:Vec<u64>,floats:Vec<f64>,tuple:[f64;3],missing:Option<String>,nested:Box<Row>,#[dsl(table)]rows:Vec<Row>,value:DslValue,wire:Wire}
    let owner=Owner{signed:vec![i64::MIN,-1,0,i64::MAX],unsigned:vec![0,127,128,i64::MAX as u64],wide:vec![u64::MAX],floats:vec![-0.0,1.5],tuple:[1.0,2.0,3.0],missing:None,nested:Box::new(Row{active:Some(false),name:"same".into(),count:17}),rows:vec![Row{active:Some(true),name:"same".into(),count:1},Row{active:None,name:"other".into(),count:u64::MAX}],value:DslValue::Object(vec![("z".into(),DslValue::Array(vec![DslValue::uint(u64::MAX),DslValue::int(i64::MIN),DslValue::float(-0.0)])),("a".into(),DslValue::String("same".into()))]),wire:Wire(WireValue{from:WireNode{id:"from".into(),kind:Some("node".into()),port:None},edge:Some((true,WireNode{id:"to".into(),kind:None,port:Some("port".into())})),edge_label:WireEdgeLabel{id:Some("edge".into()),kind:Some("kind".into())},properties:DslValue::Object(vec![("weight".into(),DslValue::float(-0.0))])})};
    let spec=Owner::__dsl_spec();let record=owner.__dsl_to_record();let options=EncodeOptions::default();
    let expected=encode_record_body(&spec,&record,&options).unwrap();
    let mut source=protocol::mutation::operation_bytes::OwnedOperationBytes::try_new(32768,131072).unwrap();
    let mut allow=|_|true;let mut encoding=semio_framework_value::NativeEncodeControl::new(131072,&mut allow);
    assert_eq!(encode_projected_record_body_into(&spec,&owner,&options,&mut source,&mut encoding).unwrap(),expected.len());
    assert!(source.iter().eq(expected.iter().copied()));assert_eq!(serde_json::to_value(&source).unwrap(),serde_json::to_value(&expected).unwrap());
    let mut allow=|_|true;let mut decoding=semio_framework_value::NativeDecodeControl::new(131072,&mut allow);
    assert_eq!(decode_record_body_span_exact_controlled(protocol::ByteSpan::from_source(&source),&spec,&Default::default(),&mut decoding).unwrap(),record);
    finish(&mut source);
    let mut output=protocol::mutation::operation_bytes::OwnedOperationBytes::try_new(32768,131072).unwrap();
    let mut allow=|_|true;let mut encoding=semio_framework_value::NativeEncodeControl::new(131072,&mut allow);
    let mut narrow=options.clone();narrow.limits.max_depth=0;
    let refusal=encode_projected_record_body_into(&spec,&owner,&narrow,&mut output,&mut encoding).unwrap_err();
    assert!(matches!(refusal,PackRefusal::ValueRefusal(ValueError{kind:ValueRefusalKind::DepthLimit,..})));assert_eq!(output.len(),0);finish(&mut output);
    #[derive(DslRecord)]
    struct Only{nested:Row}
    let only=Only{nested:Row{active:Some(true),name:"same".into(),count:1}};let spec=Only::__dsl_spec();let mut narrow=options.clone();narrow.limits.max_depth=2;
    assert!(matches!(encode_record_body(&spec,&only.__dsl_to_record(),&narrow),Err(PackRefusal::LimitExceeded{kind:ValueRefusalKind::DepthLimit,..})));
    let mut output=protocol::mutation::operation_bytes::OwnedOperationBytes::try_new(32768,131072).unwrap();
    let mut allow=|_|true;let mut encoding=semio_framework_value::NativeEncodeControl::new(131072,&mut allow);
    let result=encode_projected_record_body_into(&spec,&only,&narrow,&mut output,&mut encoding);let rejected=result.is_err();finish(&mut output);
    assert!(rejected,"nested canonical Record value depth3 must refuse at2 even when source path has only2 ordinals");
    println!("[DEBUG] Actual derived original Record source preserves packed signed/unsigned/Float words, nested field IDs, sparse table bitmap, forced symbols, exact intrinsic variants and wire presence through paged canonical emission and same-source decode");
}
