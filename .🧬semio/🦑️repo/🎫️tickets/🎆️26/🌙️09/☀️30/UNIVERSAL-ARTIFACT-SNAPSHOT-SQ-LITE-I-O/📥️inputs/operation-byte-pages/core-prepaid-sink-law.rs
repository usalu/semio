
#[test]
fn record_operation_pages_real_core_sink_uses_step_funded_backing_and_same_control(){
    use protocol::mutation::operation_bytes::{OperationBytePreparation,OperationByteCloseStep};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/📦️operation-pages/🔣️.json")).unwrap();
    let case=&fixture["prepaidRecord"];
    let payload:Vec<u8>=(0..fixture["payloadBytes"].as_u64().unwrap()).map(|index|(index%251)as u8).collect();
    let mut expected:Vec<u8>=case["prefix"].as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap()as u8).collect();
    expected.extend_from_slice(&payload);
    let spec=RecordSpec::new(None,RecordLayout::Inline,vec![FieldSpec::new(1,"bytes",Shape::Bytes64)]);
    let mut record=RecordValue::default();record.fields.insert(1,FieldValue::Bytes64(payload));
    let mut options=EncodeOptions::default();options.limits.max_file_len=expected.len()as u64;
    let mut prepared=OperationBytePreparation::try_new(expected.len(),65536).unwrap();
    let mut allow=|_|true;let control=semio_framework_value::NativeEncodeControl::new(65536,&mut allow);
    let mut receipt=control.pause().unwrap();
    for _ in 0..17000{
        let mut allow=|_|true;let mut control=semio_framework_value::NativeEncodeControl::resume(receipt,&mut allow).unwrap();
        prepared.fund_one(1,4096,&mut control).unwrap();receipt=control.pause().unwrap();
        if prepared.is_funded(){break;}
    }
    assert!(prepared.is_funded());assert_eq!(prepared.accepted_prefix().unwrap().len(),0);
    let paid=prepared.allocated_bytes();
    let mut allow=|_|true;let mut control=semio_framework_value::NativeEncodeControl::resume(receipt,&mut allow).unwrap();
    assert_eq!(control.owned_bytes(),paid);
    assert_eq!(encode_record_body_into(&spec,&record,&options,&mut prepared,&mut control).unwrap(),expected.len());
    assert_eq!(prepared.allocated_bytes(),paid);
    assert_eq!(control.owned_bytes(),paid+case["fieldIndexAdmissionBytes"].as_u64().unwrap()as usize);
    let mut source=prepared.take_ready().unwrap();assert!(prepared.terminal_is_empty());
    assert!(source.iter().eq(expected.iter().copied()));assert_eq!(serde_json::to_value(&source).unwrap(),serde_json::to_value(&expected).unwrap());
    let mut allow=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(65536,&mut allow);
    assert_eq!(decode_record_body_span_exact_controlled(protocol::ByteSpan::from_source(&source),&spec,&Default::default(),&mut control).unwrap(),record);
    for _ in 0..17000{if source.close_one(1,4096).unwrap()==OperationByteCloseStep::Complete{break;}}
    assert!(source.terminal_is_empty());assert_eq!(source.allocated_bytes(),0);
    println!("[DEBUG] Real Core Record producer uses paid8194 payload backing from separate4096 funding hops with same cumulative Native control; only literal4-byte field indexes are newly admitted and same source decodes exactly before4096 terminal release");
}
