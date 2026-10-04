//! 🧪️ Canonical intrinsic Pack boundaries preserve literal bytes and physical caller admission.
use crate::record::{self,intrinsic,EncodeOptions,DecodeOptions};
use semio_framework_value::{DslValue,NativeEncodeControl,NativeDecodeControl,ValueRefusalKind};

fn octets(text:&str)->Vec<u8>{text.as_bytes().chunks_exact(2).map(|word|u8::from_str_radix(std::str::from_utf8(word).unwrap(),16).unwrap()).collect()}
fn tagged(value:&DslValue)->serde_json::Value{serde_json::Value::from(semio_framework_value::intrinsic_json::encode(value))}
fn fixture()->serde_json::Value{serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap()}
fn literal(row:&serde_json::Value)->DslValue{semio_framework_value::intrinsic_json::decode(&DslValue::from(&row["value"])).unwrap()}
fn reference_record(value:&DslValue)->(semio_framework_dsl_record::RecordSpec,semio_framework_dsl_record::RecordValue){
    use semio_framework_dsl_record::{RecordSpec,RecordLayout,FieldSpec,Shape,RecordFields,RecordValue,FieldValue};
    let spec=RecordSpec::new(None,RecordLayout::Lines,vec![FieldSpec::new(1,"value",Shape::Value)]);
    let mut fields=RecordFields::default();fields.insert(1,FieldValue::Value(value.clone()));(spec,RecordValue{fields})
}

#[test]
fn intrinsic_boundary_literal_body_and_document_match_the_owned_schema(){
    let fixture=fixture();let output=EncodeOptions::default();let input=DecodeOptions::default();
    for row in fixture["cases"].as_array().unwrap(){
        let value=literal(row);let expected=octets(row["bodyHex"].as_str().unwrap());let mut allow=|_|true;
        let mut encode=NativeEncodeControl::new(usize::MAX,&mut allow);let bytes=intrinsic::encode_body(&value,&output,&mut encode).unwrap();assert_eq!(bytes,expected,"{}",row["id"]);
        let mut decode=NativeDecodeControl::new(usize::MAX,&mut allow);let decoded=intrinsic::decode_body(&bytes,&input,&mut decode).unwrap();assert_eq!(tagged(&decoded),row["value"]);
        let(spec,record)=reference_record(&value);let reference=record::encode_document(&spec,&record,&output).unwrap();
        let mut encode=NativeEncodeControl::new(usize::MAX,&mut allow);let document=intrinsic::encode_document(&value,&output,&mut encode).unwrap();assert_eq!(document,reference,"{}",row["id"]);
        let mut decode=NativeDecodeControl::new(usize::MAX,&mut allow);let decoded=intrinsic::decode_document(&document,&input,&mut decode).unwrap();assert_eq!(tagged(&decoded),row["value"]);
        let semantic:serde_json::Value=serde_json::from_str(&semio_framework_pack_json::to_json_string(&decoded)).unwrap();assert_eq!(semantic,serde_json::Value::from(&value));
    }
    eprintln!("[DEBUG] canonical intrinsic Pack literal/body/document/serde oracle cases={}",fixture["cases"].as_array().unwrap().len());
}

#[test]
fn intrinsic_boundary_refuses_extra_missing_unknown_and_suffix_fields(){
    let fixture=fixture();let input=DecodeOptions::default();
    for row in fixture["refusals"].as_array().unwrap(){let bytes=octets(row["bodyHex"].as_str().unwrap());let mut allow=|_|true;let mut control=NativeDecodeControl::new(usize::MAX,&mut allow);let error=intrinsic::decode_body(&bytes,&input,&mut control).unwrap_err();assert_eq!(error.kind(),ValueRefusalKind::InvalidValue,"{}",row["id"]);}
    let(spec,mut record)=reference_record(&DslValue::Null);record.fields.insert(2,semio_framework_dsl_record::FieldValue::Value(DslValue::Bool(true)));let bytes=record::encode_document(&spec,&record,&EncodeOptions::default()).unwrap();let mut allow=|_|true;let mut control=NativeDecodeControl::new(usize::MAX,&mut allow);assert_eq!(intrinsic::decode_document(&bytes,&input,&mut control).unwrap_err().kind(),ValueRefusalKind::InvalidValue);
    let(spec,mut record)=reference_record(&DslValue::Null);record.fields.remove(&1);let bytes=record::encode_document(&spec,&record,&EncodeOptions::default()).unwrap();let mut control=NativeDecodeControl::new(usize::MAX,&mut allow);assert_eq!(intrinsic::decode_document(&bytes,&input,&mut control).unwrap_err().kind(),ValueRefusalKind::InvalidValue);
    let(mut spec,record)=reference_record(&DslValue::Null);spec.fields[0].key="other".into();let bytes=record::encode_document(&spec,&record,&EncodeOptions::default()).unwrap();let mut control=NativeDecodeControl::new(usize::MAX,&mut allow);assert_eq!(intrinsic::decode_document(&bytes,&input,&mut control).unwrap_err().kind(),ValueRefusalKind::InvalidValue);
}

#[test]
fn intrinsic_boundary_settles_actual_requests_and_cumulative_cancellation(){
    let value=DslValue::String("a".repeat(131072));let output=EncodeOptions::default();let input=DecodeOptions::default();
    for document in [false,true]{
        let mut allow=|_|true;let mut control=NativeEncodeControl::new(usize::MAX,&mut allow);
        let(encoded,requested)=crate::test_allocation::observe(||if document{intrinsic::encode_document(&value,&output,&mut control)}else{intrinsic::encode_body(&value,&output,&mut control)});let bytes=encoded.unwrap();assert_eq!(control.owned_bytes(),requested,"complete physical producer allowance");
        let mut exact=NativeEncodeControl::new(requested,&mut allow);let(encoded,actual)=crate::test_allocation::observe(||if document{intrinsic::encode_document(&value,&output,&mut exact)}else{intrinsic::encode_body(&value,&output,&mut exact)});assert_eq!(encoded.unwrap(),bytes);assert_eq!(actual,requested);
        let mut short=NativeEncodeControl::new(requested-1,&mut allow);let error=if document{intrinsic::encode_document(&value,&output,&mut short)}else{intrinsic::encode_body(&value,&output,&mut short)}.unwrap_err();assert_eq!(error.kind(),ValueRefusalKind::OwnershipLimit);
        let mut cumulative=NativeEncodeControl::new(requested*2-1,&mut allow);let first=if document{intrinsic::encode_document(&value,&output,&mut cumulative)}else{intrinsic::encode_body(&value,&output,&mut cumulative)}.unwrap();assert_eq!(first,bytes);let error=if document{intrinsic::encode_document(&value,&output,&mut cumulative)}else{intrinsic::encode_body(&value,&output,&mut cumulative)}.unwrap_err();assert_eq!(error.kind(),ValueRefusalKind::OwnershipLimit);
        let mut deny=|_|false;let mut canceled=NativeEncodeControl::new(usize::MAX,&mut deny);let error=if document{intrinsic::encode_document(&value,&output,&mut canceled)}else{intrinsic::encode_body(&value,&output,&mut canceled)}.unwrap_err();assert_eq!(error.kind(),ValueRefusalKind::Canceled);assert_eq!(canceled.owned_bytes(),0);
        let mut allow=|_|true;let mut decode=NativeDecodeControl::new(usize::MAX,&mut allow);let(decoded,requested)=crate::test_allocation::observe(||if document{intrinsic::decode_document(&bytes,&input,&mut decode)}else{intrinsic::decode_body(&bytes,&input,&mut decode)});assert_eq!(decoded.unwrap(),value);assert_eq!(decode.owned_bytes(),requested,"complete physical decoder allowance");
        let mut exact=NativeDecodeControl::new(requested,&mut allow);let(decoded,actual)=crate::test_allocation::observe(||if document{intrinsic::decode_document(&bytes,&input,&mut exact)}else{intrinsic::decode_body(&bytes,&input,&mut exact)});assert_eq!(decoded.unwrap(),value);assert_eq!(actual,requested);
        let mut short=NativeDecodeControl::new(requested-1,&mut allow);let error=if document{intrinsic::decode_document(&bytes,&input,&mut short)}else{intrinsic::decode_body(&bytes,&input,&mut short)}.unwrap_err();assert_eq!(error.kind(),ValueRefusalKind::OwnershipLimit);
        let mut cumulative=NativeDecodeControl::new(requested*2-1,&mut allow);let first=if document{intrinsic::decode_document(&bytes,&input,&mut cumulative)}else{intrinsic::decode_body(&bytes,&input,&mut cumulative)}.unwrap();assert_eq!(first,value);let error=if document{intrinsic::decode_document(&bytes,&input,&mut cumulative)}else{intrinsic::decode_body(&bytes,&input,&mut cumulative)}.unwrap_err();assert_eq!(error.kind(),ValueRefusalKind::OwnershipLimit);
        let mut deny=|_|false;let mut canceled=NativeDecodeControl::new(usize::MAX,&mut deny);let error=if document{intrinsic::decode_document(&bytes,&input,&mut canceled)}else{intrinsic::decode_body(&bytes,&input,&mut canceled)}.unwrap_err();assert_eq!(error.kind(),ValueRefusalKind::Canceled);assert_eq!(canceled.owned_bytes(),0);
        eprintln!("[DEBUG] intrinsic Pack document={document} caller ownership requests={requested} exact/short/cancel executed");
    }
}
