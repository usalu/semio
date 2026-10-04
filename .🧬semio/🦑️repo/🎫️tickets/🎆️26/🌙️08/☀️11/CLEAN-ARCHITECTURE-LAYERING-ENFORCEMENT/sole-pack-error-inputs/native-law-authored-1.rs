#[test]
fn public_pack_primitives_and_replication_share_the_defining_error_identity(){
    use semio_framework_pack_error::PackError as CanonicalError;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/⚠️error/🪪️identity/🔣️.json")).unwrap();
    for row in fixture["primitiveCases"].as_array().unwrap(){
        let bytes:Vec<u8>=row["bytes"].as_array().unwrap().iter().map(|value|u8::try_from(value.as_u64().unwrap()).unwrap()).collect();let mut position=0;
        let result:Result<u64,CanonicalError>=crate::read_varint_u64(&bytes,&mut position);
        let error=result.unwrap_err();assert!(matches!(error,CanonicalError::Truncated(_)));assert_eq!(position,row["position"].as_u64().unwrap()as usize);assert_eq!(error.to_string(),row["display"].as_str().unwrap());
        let container:crate::PackError=error;let replica:protocol::PackError=container;let canonical:CanonicalError=replica;let converted=canonical.into_value_error();
        let oracle=serde_json::json!({"kind":row["convertedKind"],"message":row["display"]});assert_eq!(serde_json::json!({"kind":format!("{:?}",converted.kind),"message":converted.message}),oracle);
    }
    eprintln!("[DEBUG] public Pack primitive/container/Replication errors retain one defining native type");
}

#[test]
fn public_pack_typed_causes_preserve_owned_messages_and_fault_identity(){
    use semio_framework_pack_error::PackError as CanonicalError;
    use semio_framework_diagnostic::{FaultFrom,FaultOrigin,Severity};
    use semio_framework_value::{ValueError,ValueRefusalKind};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../⚠️error/🧫️fixtures/⚠️refusal/🔣️.json")).unwrap();
    let identity:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/⚠️error/🪪️identity/🔣️.json")).unwrap();
    fn kind(value:&str)->ValueRefusalKind{match value{"InvalidValue"=>ValueRefusalKind::InvalidValue,"Canceled"=>ValueRefusalKind::Canceled,"OwnershipLimit"=>ValueRefusalKind::OwnershipLimit,"AllocationFailed"=>ValueRefusalKind::AllocationFailed,"WorkLimit"=>ValueRefusalKind::WorkLimit,"DepthLimit"=>ValueRefusalKind::DepthLimit,"UnsupportedOwner"=>ValueRefusalKind::UnsupportedOwner,"InvariantViolated"=>ValueRefusalKind::InvariantViolated,_=>panic!("unknown authored kind")}}
    for row in fixture["cases"].as_array().unwrap(){
        let expected_kind=kind(row["kind"].as_str().unwrap());let mut cause=ValueError::new(expected_kind,row["message"].as_str().unwrap());for part in row["path"].as_array().unwrap(){cause=cause.under(part.as_str().unwrap());}let allocation=cause.message.as_ptr();
        let canonical=CanonicalError::from(cause);let wrapper:crate::PackError=canonical;let replica:protocol::PackError=wrapper;let wrapper:CanonicalError=replica;
        let CanonicalError::ValueRefusal(refusal)=&wrapper else{panic!("typed cause")};assert_eq!(refusal.kind,expected_kind);assert_eq!(refusal.message.as_ptr(),allocation);assert_eq!(wrapper.to_string(),row["display"].as_str().unwrap());
        let source=std::error::Error::source(&wrapper).unwrap().downcast_ref::<ValueError>().unwrap();assert!(std::ptr::eq(source,refusal));assert_eq!(wrapper.fault_origin(),FaultOrigin::Module);assert_eq!(wrapper.fault_code().0,identity["fault"]["code"].as_str().unwrap());assert_eq!(wrapper.fault_severity(),Severity::Error);
        let converted=wrapper.into_value_error();assert_eq!(converted.kind,expected_kind);assert_eq!(converted.message.as_ptr(),allocation);assert_eq!(converted.message,row["display"].as_str().unwrap().strip_prefix("schema error: ").unwrap());
    }
    eprintln!("[DEBUG] public typed Pack cause eight vectors preserve allocation/path/kind/source and module.pack Fault; conversion retains raw owned cause");
}
