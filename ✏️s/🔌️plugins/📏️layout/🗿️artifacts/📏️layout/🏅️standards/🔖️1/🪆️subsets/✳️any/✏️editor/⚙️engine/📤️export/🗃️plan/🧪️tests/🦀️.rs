//! 🗃️ Neutral plan custody, original physical backing and paid cancellation use the real paged owner.
use super::*;
fn fixture()->serde_json::Value{serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../../🗃️plan/🧪️tests/🧫️fixtures/🔣️.json")).unwrap()}
fn original()->RetainedCloneGrant{let f=fixture();let g=&f["original"];RetainedCloneGrant{maximum_items:g["maximumItems"].as_u64().unwrap()as usize,maximum_copy_bytes:g["maximumCopyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:g["maximumCapacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:g["maximumReleaseBytes"].as_u64().unwrap()as usize,maximum_depth:g["maximumDepth"].as_u64().unwrap()as usize}}
fn close_turn(owner:&mut PlanOffsets,grant:RetainedCloneGrant)->semio_framework_value::RetainedCloneProgress{
    crate::fixture_effect(grant,|effect|{
        let (turn,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.close_one(effect));
        let semio_framework_value::RetainedWorkPoll::Complete(turn)=turn else{panic!("native plan close unexpectedly needs recovery")};
        let progress=turn.result.expect("funded plan close").progress();
        assert!(progress.fits(grant));
        assert_eq!(physical.requested_bytes,progress.retained_capacity_bytes);
        assert_eq!(physical.released_bytes,progress.released_bytes);
        (progress,progress)
    })
}
fn close(owner:&mut PlanOffsets){
    for _ in 0..10000{
        if owner.terminal_is_empty(){return;}
        let demand=owner.next_close_demand().unwrap();
        let grant=original();
        assert_eq!(close_turn(owner,RetainedCloneGrant{maximum_items:0,..grant}),Default::default());
        if demand.depth>0{
            assert_eq!(close_turn(owner,RetainedCloneGrant{maximum_depth:demand.depth-1,..grant}),Default::default());
        }
        close_turn(owner,grant);
    }
    panic!("plan original close stalled");
}
fn append_turn(owner:&mut PlanOffsets,source:&mut usize,stride:u32,grant:RetainedCloneGrant)->bool{
    crate::fixture_effect(grant,|effect|{
        let ((ready,receipt),physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{
            let mut work=RetainedWorkLedger::new(grant);
            let ready=owner.reserve_one_in(&mut work,effect).unwrap();
            if ready{
                owner.push_in(*source as u32*stride,&mut work).unwrap();
                *source+=1;
            }
            (ready,work.receipt())
        });
        assert!(receipt.fits(grant));
        assert_eq!(physical.requested_bytes,receipt.retained_capacity_bytes);
        assert_eq!(physical.released_bytes,receipt.released_bytes);
        (ready,receipt)
    })
}
#[test]
fn original_plan_rows_refuse_before_producer_and_close_every_cancel_cut(){
    let f=fixture();
    let stride=f["offsetStride"].as_u64().unwrap()as u32;
    for cut in f["cancelCuts"].as_array().unwrap(){
        let sheet:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../../🧪️tests/🧫️fixtures/📄️large-sheet/🔣️.json")).unwrap();
        let target=(cut.as_u64().unwrap()as usize).min(sheet["expectedX"].as_array().unwrap().len());
        let mut owner=PlanOffsets::new();
        let mut source=0;
        for _ in 0..10000{
            if source==target{break;}
            let d=owner.next_append_demand().unwrap();
            let original=original();
            let mut denied=vec![RetainedCloneGrant{maximum_items:0,..original},RetainedCloneGrant{maximum_depth:d.depth.saturating_sub(1),..original}];
            if d.copy_bytes>0{denied.push(RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes-1,..original});}
            if d.capacity_bytes>0{denied.push(RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes-1,..original});}
            for grant in denied{
                let before=source;
                assert!(!append_turn(&mut owner,&mut source,stride,grant));
                assert_eq!(source,before);
            }
            append_turn(&mut owner,&mut source,stride,original);
        }
        assert_eq!(source,target);
        for i in 0..target{assert_eq!(owner.get(i).copied(),Some(i as u32*stride));}
        close(&mut owner);
        assert!(owner.terminal_is_empty());
    }
}
#[test]
fn child_original_receipts_share_plan_metadata_holds(){
    let grant=original();
    let mut owner=PlanOffsets::new();
    let mut child=PlanOffsets::new();
    let mut done=false;
    for _ in 0..100{
        done=crate::fixture_effect(grant,|effect|{
            let mut work=RetainedWorkLedger::new(grant);
            let root=work.enter().unwrap();
            if !owner.reserve_one_in(&mut work,effect).unwrap(){work.exit();return (false,work.receipt());}
            let child_grant=work.child_grant();
            assert_eq!(child_grant.maximum_depth,grant.maximum_depth-1);
            let mut nested=RetainedWorkLedger::new(child_grant);
            let ready=child.reserve_one_in(&mut nested,&mut effect.nested(child_grant)).unwrap();
            if ready{child.push_in(17,&mut nested).unwrap();}
            let used=nested.receipt();
            work.consume_child(used).unwrap();
            if ready{owner.push_in(17,&mut work).unwrap();work.note_source(root);}
            work.exit();
            let receipt=work.receipt();
            assert!(receipt.fits(grant));
            assert_eq!(receipt.copied_bytes,used.copied_bytes+if ready{size_of::<u32>()}else{0});
            (ready,receipt)
        });
        if done{break;}
    }
    assert!(done);
    close(&mut owner);
    close(&mut child);
}
