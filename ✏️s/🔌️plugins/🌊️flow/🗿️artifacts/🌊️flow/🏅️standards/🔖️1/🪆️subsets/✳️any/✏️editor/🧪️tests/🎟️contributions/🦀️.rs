use super::*;
use semio_framework_job::{CancelToken,Generation,OperationId,StepBudget,StepContext};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,ValueError,ValueRefusalKind};

fn progress(value:&serde_json::Value)->RetainedCloneProgress{
    RetainedCloneProgress{copied_items:value["copiedItems"].as_u64().unwrap() as usize,copied_bytes:value["copiedBytes"].as_u64().unwrap() as usize,retained_capacity_bytes:value["retainedCapacityBytes"].as_u64().unwrap() as usize,released_bytes:value["releasedBytes"].as_u64().unwrap() as usize}
}

fn grant(value:&serde_json::Value)->RetainedCloneGrant{
    RetainedCloneGrant{maximum_items:value["maximumItems"].as_u64().unwrap() as usize,maximum_copy_bytes:value["maximumCopyBytes"].as_u64().unwrap() as usize,maximum_capacity_bytes:value["maximumCapacityBytes"].as_u64().unwrap() as usize,maximum_release_bytes:value["maximumReleaseBytes"].as_u64().unwrap() as usize,maximum_depth:value["maximumDepth"].as_u64().unwrap() as usize}
}

/// ⚖️ Every original success or refusal reaches the same external recipient exactly once.
#[test]
fn flow_contributions_records_original_registry_receipts_before_yield_or_refusal(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎟️contributions/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap(){
        let mut actual=progress(&row["initial"]);let mut sequence=0;
        let mut cx=StepContext::new(OperationId(107_901),Generation(9),StepBudget::new(1,u64::MAX,grant(&row["grant"])),CancelToken::root_now(),semio_framework_job::logical_now_us,&mut sequence,&mut actual);
        let effects=progress(&row["progress"]);
        let result=match row["status"].as_str().unwrap(){"progress"=>Ok(RetainedCloneStep::Progress(effects)),"complete"=>Ok(RetainedCloneStep::Complete(effects)),"refused"=>Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"original contribution registry refusal").with_retained_progress(effects)),_=>unreachable!()};
        let received=flow_contributions_registry_receipt(&mut cx,result);
        assert_eq!(received.is_err(),row["failed"].as_bool().unwrap(),"{}",row["id"]);
        match received{Ok(ready)=>assert_eq!(ready,row["ready"].as_bool().unwrap()),Err(fault)=>assert_eq!(fault.retained_progress,effects)}
        assert_eq!(cx.retained_progress(),progress(&row["recorded"]),"{}",row["id"]);
        assert_eq!(cx.retained_grant(),grant(&row["remaining"]),"{}",row["id"]);
        drop(cx);assert_eq!(actual,progress(&row["recorded"]));
    }
    println!("[DEBUG] original contributions native receiving cases={} actual external receipt preserved; producer physical effects and install closure remain separate",fixture["cases"].as_array().unwrap().len());
}

/// 🪢️ A command retires its own original alias while the live shared instance remains usable.
#[test]
fn flow_contributions_retires_only_its_original_alias_before_terminal_work(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎟️contributions/🔣️.json")).unwrap();
    for policy in fixture["aliasPolicies"].as_array().unwrap(){
        let shared=semio_framework_plugin::ArtifactInstanceOperationOwnerHandle::new(Box::new(FlowInstanceOperationOwner::new()));
        let source=shared.with_mut::<FlowInstanceOperationOwner,_>(|owner|Ok(owner as *mut _ as usize)).unwrap();
        let mut work=FlowContributionsWork::new(shared.clone());work.begin_close();
        let incoming=grant(policy);let denied=RetainedCloneGrant{maximum_items:0,..incoming};
        assert!(matches!(work.close_step(denied),semio_framework_job::InteractiveJobCloseStep::Blocked));
        assert!(work.instance_owner.is_some()&&work.instance_retirement.is_none());
        for turn in 0..128{
            if work.terminal_is_empty(){break}
            assert!(turn<127,"original contribution alias must reach terminal");
            match work.close_step(incoming){semio_framework_job::InteractiveJobCloseStep::Pending{progress}|semio_framework_job::InteractiveJobCloseStep::Complete{progress}=>{assert!(progress.fits(incoming));assert_eq!(progress.retained_capacity_bytes,0);assert_eq!(progress.released_bytes,0);},other=>panic!("original nonfinal alias close refused: {other:?}")}
        }
        assert!(work.terminal_is_empty());assert_eq!(work.terminal_frame_release_bytes(),Some(std::mem::size_of::<FlowContributionsWork>()));
        shared.with_mut::<FlowInstanceOperationOwner,_>(|owner|{assert_eq!(owner as *mut _ as usize,source);assert!(!owner.closing&&owner.eval_session.is_some());Ok(())}).unwrap();
        drop(work);
        let mut final_owner=semio_framework_plugin::ArtifactInstanceOperationOwnerAliasRetirement::new(shared);
        for turn in 0..1_000_000{if final_owner.terminal_is_empty(){break}assert!(turn<999_999,"actual original final instance owner stalled");final_owner.step(incoming).unwrap();}
        assert!(final_owner.terminal_is_empty());drop(final_owner);
    }
    println!("[DEBUG] original contributions native alias custody policies=3 same source and shared instance preserved; physical dynamic-shell parity requires its defining allocator law");
}
