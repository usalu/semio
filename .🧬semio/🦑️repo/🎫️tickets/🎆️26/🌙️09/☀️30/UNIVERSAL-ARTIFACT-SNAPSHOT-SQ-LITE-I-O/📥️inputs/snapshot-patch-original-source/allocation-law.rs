#[test]
fn paged_snapshot_patch_original_json_source_honors_paid_allocation_policy(){
    use kernel::operation_bytes::{OperationBytePreparation,OperationByteCloseStep};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/📦️operation-source.json")).unwrap();
    let patch=SnapshotPatch::Set{path:"/title".into(),value:DslValue::String(fixture["word"].as_str().unwrap().repeat(fixture["payloadBytes"].as_u64().unwrap()as usize))};
    let expected=patch.encode_op().unwrap();let allocation=fixture["allocationBytes"].as_u64().unwrap()as usize;let items=fixture["maximumCloseItems"].as_u64().unwrap()as usize;let bytes=fixture["maximumCloseBytes"].as_u64().unwrap()as usize;let steps=expected.len()+fixture["closeStepScaffold"].as_u64().unwrap()as usize;
    let mut preparation=OperationBytePreparation::try_new(expected.len(),allocation).unwrap();let mut allow=|_|true;let mut encoding=semio_framework_value::NativeEncodeControl::new(allocation,&mut allow);
    for _ in 0..steps{preparation.fund_one(items,bytes,&mut encoding).unwrap();if preparation.is_funded(){break;}}assert!(preparation.is_funded());let paid=encoding.owned_bytes();let backing=preparation.allocated_bytes();
    let mut options=kernel::codec::PackEncodeOptions::default();options.limits.max_file_len=expected.len()as u64;options.limits.max_total_alloc=fixture["refusedAllocationBytes"].as_u64().unwrap();assert!(paid>options.limits.max_total_alloc as usize);
    let result=patch.encode_op_into(&options,&mut preparation,&mut encoding);let accepted=preparation.accepted_prefix().unwrap().len();assert_eq!(encoding.owned_bytes(),paid);assert_eq!(preparation.allocated_bytes(),backing);assert_eq!(encoding.maximum_bytes(),allocation);
    for _ in 0..steps{match preparation.close_one(items,bytes).unwrap(){OperationByteCloseStep::Complete=>break,OperationByteCloseStep::Pending{released_items,released_bytes}=>{assert!(released_items<=items);assert!(released_bytes<=bytes);}}}assert!(preparation.terminal_is_empty());assert_eq!(preparation.allocated_bytes(),0);
    let error=result.expect_err("original caller allocation policy must reject already-admitted backing before accepting source bytes");let kernel::ProtocolError::Pack(kernel::PackError::Refusal(refusal))=error else{panic!("expected exact typed Pack allocation refusal")};assert_eq!(refusal.kind(),semio_framework_value::ValueRefusalKind::OwnershipLimit);assert_eq!(accepted,0);
    println!("[DEBUG] Original8194 source and prepaid operation backing retained on caller allocation-policy refusal; same cumulative control restored and every page physically returned under fixed4096 grants");
}
