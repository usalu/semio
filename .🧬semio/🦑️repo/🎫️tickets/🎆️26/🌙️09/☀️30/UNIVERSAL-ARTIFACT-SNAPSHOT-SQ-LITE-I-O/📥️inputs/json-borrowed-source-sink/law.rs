#[test]
fn borrowed_json_source_sink_keeps_caller_prefix_policy_and_original_words(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🫳️source-sink.json")).unwrap();
    let text=fixture["word"].as_str().unwrap().repeat(fixture["repeats"].as_u64().unwrap()as usize);
    let source=DslValue::Object(vec![("first".into(),DslValue::String(text.clone())),("second".into(),DslValue::Array(vec![DslValue::uint(u64::MAX),DslValue::int(i64::MIN),DslValue::float(-0.0),DslValue::float(2.5),DslValue::Bool(false),DslValue::Null,DslValue::Bytes(vec![0,127,255])]))]);
    let independent=serde_json::json!({"first":text,"second":[u64::MAX,i64::MIN,-0.0,2.5,false,null,[0,127,255]]}).to_string();
    assert_eq!(to_json_string(&source),independent);
    let pointer=match &source{DslValue::Object(entries)=>match &entries[0].1{DslValue::String(text)=>text.as_ptr(),_=>unreachable!()},_=>unreachable!()};
    let prefix=fixture["prefix"].as_str().unwrap().as_bytes();let maximum=fixture["maximumOutputBytes"].as_u64().unwrap();let depth=fixture["maximumDepth"].as_u64().unwrap()as usize;
    let mut output=Vec::with_capacity(prefix.len()+independent.len());output.extend_from_slice(prefix);
    let mut accept=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(0,&mut accept);
    let length=write_json_source_into(&source,maximum,depth,&mut|bytes:&[u8],_:&mut semio_framework_value::NativeEncodeControl<'_>|{output.extend_from_slice(bytes);Ok::<_,ValueError>(())},&mut control).unwrap();
    assert_eq!(length,independent.len()as u64);assert_eq!(&output[..prefix.len()],prefix);assert_eq!(&output[prefix.len()..],independent.as_bytes());assert_eq!(control.owned_bytes(),0);
    for reason in ["policy","cancel","sink"]{
        output.truncate(prefix.len());
        let mut callback=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|reason!="cancel"||progress.completed<fixture["cancelAt"].as_u64().unwrap()as usize;
        let mut control=semio_framework_value::NativeEncodeControl::new(0,&mut callback);
        let limit=if reason=="policy"{independent.len()as u64-1}else{maximum};
        let error=write_json_source_into(&source,limit,depth,&mut|bytes:&[u8],_:&mut semio_framework_value::NativeEncodeControl<'_>|{if reason=="sink"&&output.len()-prefix.len()+bytes.len()>fixture["sinkRefuseAt"].as_u64().unwrap()as usize{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"literal caller sink refusal"));}output.extend_from_slice(bytes);Ok(())},&mut control).unwrap_err();
        assert_eq!(error.kind,match reason{"policy"=>ValueRefusalKind::OwnershipLimit,"cancel"=>ValueRefusalKind::Canceled,_=>ValueRefusalKind::WorkLimit});
        assert_eq!(&output[..prefix.len()],prefix);assert!(output.len()-prefix.len()<independent.len());assert_eq!(&output[prefix.len()..],&independent.as_bytes()[..output.len()-prefix.len()]);assert_eq!(control.owned_bytes(),0);
    }
    let original=match &source{DslValue::Object(entries)=>match &entries[0].1{DslValue::String(text)=>text.as_ptr(),_=>unreachable!()},_=>unreachable!()};assert_eq!(original,pointer);
    let mut output=Vec::new();let mut accept=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(0,&mut accept);
    assert_eq!(write_json_source_into(&source,maximum,1,&mut|bytes:&[u8],_:&mut semio_framework_value::NativeEncodeControl<'_>|{output.extend_from_slice(bytes);Ok::<_,ValueError>(())},&mut control).unwrap_err().kind,ValueRefusalKind::DepthLimit);
    println!("[DEBUG] Original JSON source pointer and exact Serde words survive caller prefix, full-byte/depth policy and original typed sink/cancellation refusal without a payload mirror; caller output backing is independently preowned and transferred refusal ownership is separate");
}
