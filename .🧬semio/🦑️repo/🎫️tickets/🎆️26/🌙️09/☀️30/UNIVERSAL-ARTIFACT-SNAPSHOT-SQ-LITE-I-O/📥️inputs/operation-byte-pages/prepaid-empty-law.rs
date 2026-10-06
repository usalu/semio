
#[test]
fn owned_operation_byte_pages_empty_prepaid_handback_keeps_exact_zero_payload_authority(){
    use super::operation_bytes::OperationBytePreparation;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let mut prepared=OperationBytePreparation::try_new(fixture["emptyOperationLength"].as_u64().unwrap()as usize,65536).unwrap();
    assert!(prepared.is_funded());assert_eq!(prepared.accepted_prefix().unwrap().len(),0);
    let mut source=prepared.take_ready().unwrap();assert!(prepared.terminal_is_empty());assert_eq!(source.allocated_bytes(),0);
    assert_eq!(serde_json::to_value(&source).unwrap(),serde_json::json!([]));
    let demand=source.next_allocation_bytes();
    assert_eq!(source.close_one(1,4096).unwrap(),OperationByteCloseStep::Complete);assert!(source.terminal_is_empty());
    assert_eq!(demand.expect_err("exact zero emitted operation cannot acquire a later payload allocation").kind,crate::value::ValueRefusalKind::OwnershipLimit);
    println!("[DEBUG] Empty measured operation handback preserves exact zero payload authority and cannot acquire future backing after publication");
}
