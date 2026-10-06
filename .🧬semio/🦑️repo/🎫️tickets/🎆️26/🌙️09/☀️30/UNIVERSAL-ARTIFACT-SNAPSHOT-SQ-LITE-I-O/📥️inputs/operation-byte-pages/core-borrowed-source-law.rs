
#[test]
fn record_operation_pages_borrowed_source_excludes_large_payload_mirrors_and_keeps_literal_wire(){
    use protocol::mutation::operation_bytes::{OperationBytePreparation,OperationByteMeasurement};
    use semio_framework_dsl_record::native_encoding::{FieldProjectionSource,FieldProjectionView as V};
    struct Source{bytes:Vec<u8>,text:String}
    impl FieldProjectionSource for Source{
        fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{
            match path{[]=>Ok(V::Record(&[1,2])),[0]=>Ok(V::Bytes(&self.bytes)),[1]=>Ok(V::Text(&self.text)),_=>Err(semio_framework_dsl_record::native_encoding::projection_path_error())}
        }
    }
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/📦️operation-pages/🔣️.json")).unwrap();
    let case=&fixture["borrowedSource"];
    let source=Source{bytes:(0..fixture["payloadBytes"].as_u64().unwrap()).map(|index|(index%251)as u8).collect(),text:case["textUnit"].as_str().unwrap().repeat(fixture["payloadBytes"].as_u64().unwrap()as usize)};
    let spec=RecordSpec::new(None,RecordLayout::Inline,vec![FieldSpec::new(1,"bytes",Shape::Bytes64),FieldSpec::new(2,"text",Shape::Text)]);
    let mut record=RecordValue::default();record.fields.insert(1,FieldValue::Bytes64(source.bytes.clone()));record.fields.insert(2,FieldValue::Text(source.text.clone()));
    let literal=|key:&str|case[key].as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap()as u8).collect::<Vec<_>>();
    let mut expected=literal("prefix");expected.extend_from_slice(&source.bytes);expected.extend(literal("textHeader"));expected.extend_from_slice(source.text.as_bytes());
    let mut options=EncodeOptions::default();options.limits.max_file_len=expected.len()as u64;
    assert_eq!(encode_record_body(&spec,&record,&options).unwrap(),expected);
    let mut measure=OperationByteMeasurement::new(options.limits.max_file_len);
    let mut allow=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(65536,&mut allow);
    assert_eq!(encode_projected_record_body_into(&spec,&source,&options,&mut measure,&mut control).unwrap(),expected.len());
    assert!(control.owned_bytes()<case["maximumMetadataBytes"].as_u64().unwrap()as usize);
    let mut prepared=OperationBytePreparation::try_new(measure.exact_length().unwrap(),65536).unwrap();
    for _ in 0..34000{prepared.fund_one(1,4096,&mut control).unwrap();if prepared.is_funded(){break;}}
    assert!(prepared.is_funded());let before=control.owned_bytes();let backing=prepared.allocated_bytes();
    assert_eq!(encode_projected_record_body_into(&spec,&source,&options,&mut prepared,&mut control).unwrap(),expected.len());
    assert!(control.owned_bytes()-before<case["maximumMetadataBytes"].as_u64().unwrap()as usize);assert_eq!(prepared.allocated_bytes(),backing);
    let mut wire=prepared.take_ready().unwrap();assert!(wire.iter().eq(expected.iter().copied()));assert_eq!(serde_json::to_value(&wire).unwrap(),serde_json::to_value(&expected).unwrap());
    let mut allow=|_|true;let mut decoding=semio_framework_value::NativeDecodeControl::new(65536,&mut allow);
    assert_eq!(decode_record_body_span_exact_controlled(protocol::ByteSpan::from_source(&wire),&spec,&Default::default(),&mut decoding).unwrap(),record);
    finish(&mut wire);
    let mut wire=protocol::mutation::operation_bytes::OwnedOperationBytes::try_new(expected.len(),65536).unwrap();
    let mut canceled=|event:semio_framework_value::native_encoding::NativeEncodeProgress|!(event.total==source.bytes.len()&&event.completed>=256);
    let mut control=semio_framework_value::NativeEncodeControl::new(65536,&mut canceled);
    let refusal=encode_projected_record_body_into(&spec,&source,&options,&mut wire,&mut control).unwrap_err();
    assert!(matches!(refusal,PackRefusal::ValueRefusal(ValueError{kind:ValueRefusalKind::Canceled,..})));
    assert!(wire.len()>0&&wire.len()<expected.len());assert!(wire.iter().eq(expected[..wire.len()].iter().copied()));finish(&mut wire);
    assert_eq!(source.bytes.len(),fixture["payloadBytes"].as_u64().unwrap()as usize);assert_eq!(source.text.len(),source.bytes.len());
    println!("[DEBUG] Core borrowed source emits literal bytes plus8194 text without an owned payload mirror; same cumulative control funds fixed4096 pages and retires real wire, canceled prefix retained");
}
