#[test]
fn paged_snapshot_patch_original_json_source_keeps_every_variant_and_owned_prefix(){
    use kernel::operation_bytes::{OwnedOperationBytes,OperationByteMeasurement,OperationBytePreparation,OperationByteCloseStep};
    use semio_framework_value::{NativeEncodeControl,ValueRefusalKind};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/📦️operation-source.json")).unwrap();
    let mut operations=Vec::new();
    for case in fixture["cases"].as_array().unwrap(){let text=case.as_str().unwrap();let patch=SnapshotPatch::parse_op(text).unwrap();assert_eq!(patch.encode_op().unwrap(),text.as_bytes());assert_eq!(serde_json::from_str::<serde_json::Value>(&patch.print_op()).unwrap(),serde_json::from_str::<serde_json::Value>(text).unwrap());operations.push(patch);}
    operations.push(SnapshotPatch::Set{path:"/title".into(),value:DslValue::String(fixture["word"].as_str().unwrap().repeat(fixture["payloadBytes"].as_u64().unwrap()as usize))});
    let allocation=fixture["allocationBytes"].as_u64().unwrap()as usize;let items=fixture["maximumCloseItems"].as_u64().unwrap()as usize;let bytes=fixture["maximumCloseBytes"].as_u64().unwrap()as usize;
    let close=|owner:&mut OwnedOperationBytes,maximum_steps:usize|{let mut released=0;for _ in 0..maximum_steps{match owner.close_one(items,bytes).unwrap(){OperationByteCloseStep::Complete=>break,OperationByteCloseStep::Pending{released_items,released_bytes}=>{assert!(released_items<=items);assert!(released_bytes<=bytes);released+=released_bytes;}}}assert!(owner.terminal_is_empty());assert_eq!(owner.allocated_bytes(),0);released};
    let kind=|error:kernel::ProtocolError|match error{kernel::ProtocolError::Pack(kernel::PackError::Refusal(refusal))=>refusal.kind(),error=>panic!("expected genuine typed Pack refusal, got {error:?}")};
    for patch in &operations{
        let expected=patch.encode_op().unwrap();let steps=expected.len()+fixture["closeStepScaffold"].as_u64().unwrap()as usize;let mut options=kernel::codec::PackEncodeOptions::default();options.limits.max_file_len=expected.len()as u64;
        let mut measure=OperationByteMeasurement::new(options.limits.max_file_len);let mut allow=|_|true;let mut encoding=NativeEncodeControl::new(0,&mut allow);
        patch.encode_op_into(&options,&mut measure,&mut encoding).unwrap();assert_eq!(measure.exact_length().unwrap(),expected.len());assert_eq!(encoding.owned_bytes(),0);
        let mut preparation=OperationBytePreparation::try_new(expected.len(),allocation).unwrap();let mut allow=|_|true;let mut encoding=NativeEncodeControl::new(allocation,&mut allow);
        for _ in 0..steps{preparation.fund_one(items,bytes,&mut encoding).unwrap();if preparation.is_funded(){break;}}assert!(preparation.is_funded());let paid=encoding.owned_bytes();let backing=preparation.allocated_bytes();
        patch.encode_op_into(&options,&mut preparation,&mut encoding).unwrap();assert_eq!(encoding.owned_bytes(),paid);assert_eq!(preparation.allocated_bytes(),backing);
        let mut owner=preparation.take_ready().unwrap();assert!(owner.iter().eq(expected.iter().copied()));assert_eq!(serde_json::to_value(&owner).unwrap(),serde_json::to_value(&expected).unwrap());assert_eq!(owner.close_one(0,bytes).unwrap(),OperationByteCloseStep::Pending{released_items:0,released_bytes:0});assert_eq!(owner.len(),expected.len());assert!(close(&mut owner,steps)>=expected.len());
        let mut short=options.clone();short.limits.max_file_len-=1;let mut prefix=OwnedOperationBytes::try_new(expected.len(),allocation).unwrap();let mut allow=|_|true;let mut encoding=NativeEncodeControl::new(allocation,&mut allow);
        assert_eq!(kind(patch.encode_op_into(&short,&mut prefix,&mut encoding).unwrap_err()),ValueRefusalKind::OwnershipLimit);assert!(prefix.len()<expected.len());assert!(prefix.iter().eq(expected[..prefix.len()].iter().copied()));close(&mut prefix,steps);
    }
    let patch=operations.last().unwrap();let expected=patch.encode_op().unwrap();let mut prefix=OwnedOperationBytes::try_new(expected.len(),allocation).unwrap();let mut options=kernel::codec::PackEncodeOptions::default();options.limits.max_file_len=expected.len()as u64;
    let mut cancel=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|progress.completed<fixture["cancelAt"].as_u64().unwrap()as usize;let mut encoding=NativeEncodeControl::new(allocation,&mut cancel);
    assert_eq!(kind(patch.encode_op_into(&options,&mut prefix,&mut encoding).unwrap_err()),ValueRefusalKind::Canceled);assert!(prefix.len()<expected.len());assert!(prefix.iter().eq(expected[..prefix.len()].iter().copied()));close(&mut prefix,expected.len()+fixture["closeStepScaffold"].as_u64().unwrap()as usize);
    println!("[DEBUG] All six original SnapshotPatch JSON variants preserve authored occurrences, optional omissions and neutral Serde values; direct8194 source and exact refused prefix return all page allocations under fixed4096 grants");
}
