use super::*;

fn quoted_close_grant(work:&FlowChildGroupWork,body:usize)->semio_framework_value::RetainedCloneGrant{
    let copy=work.next_close_copy_byte_demand().unwrap().max(body);
    semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:work.next_close_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:work.next_close_release_byte_demand().unwrap(),maximum_depth:work.next_close_depth_demand().unwrap().max(1)}
}

#[test]
fn rejected_child_preparation_remains_owned_until_bounded_flow_work_close(){
    use semio_framework_plugin::retained_command::{ArtifactCommandWork,ArtifactCommandWorkStep};
    use semio_framework_plugin::app::ChildEmitPreparation;
    use semio_framework_job::InteractiveJobCloseStep;
    use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::mutations::{SemioFlowMutation,drag_nodes::DragNodes};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🎮️commands/✏️node-graph-edit/🧫️fixtures/child-preparation/🔣️.json")).unwrap();
    let child_id=fixture["childId"].as_str().unwrap();
    let mut work=FlowChildGroupWork{tool_id:"nodeGraphEdit",instance_owner:None,output:None,completed:false,closing:false};
    for (slot,accepted) in [(fixture["foreignSlot"].as_str().unwrap(),false),(fixture["slot"].as_str().unwrap(),true)]{
        let operation=SemioFlowMutation::DragNodes(DragNodes{targets:vec![fixture["target"].as_str().unwrap().into()],dx:fixture["dx"].as_f64().unwrap(),dy:fixture["dy"].as_f64().unwrap()});
        let emit=semio_framework_plugin::Emit{child_preparations:std::collections::VecDeque::from([ChildEmitPreparation::of::<SemioFlowSnapshot,_>(slot,child_id,vec![operation])]),..Default::default()};
        let source=emit.child_preparations.front().unwrap();
        assert_eq!(source.matches_source::<SemioFlowMutation>(fixture["slot"].as_str().unwrap(),child_id,1),accepted);
        assert!(!source.matches_source::<u8>(slot,child_id,1));
        assert!(!source.matches_source::<SemioFlowMutation>(slot,"foreign-child",1));
        assert!(!source.matches_source::<SemioFlowMutation>(slot,child_id,0));
        assert_eq!(source.retained_operation_count(),fixture["operationCount"].as_u64().unwrap()as usize);
        assert!(source.accepted_prefix().unwrap().ops.is_empty());
        let result=work.accept_output(emit,child_id);
        if accepted{
            let ArtifactCommandWorkStep::Complete(emit)=result.unwrap()else{panic!("exact original typed source must transfer complete");};
            assert!(work.output.is_none());work.output=Some(emit);
        }else{
            let fault=match result{Err(fault)=>fault,_=>panic!("foreign literal slot must refuse")};
            assert_eq!(fault.message,fixture["refusalCode"].as_str().unwrap());
            assert_eq!(work.output.as_ref().unwrap().child_preparations.front().unwrap().retained_operation_count(),1);
        }
        work.begin_close();
        let zero=semio_framework_value::RetainedCloneGrant{maximum_items:0,..quoted_close_grant(&work,4096)};
        assert_eq!(work.close_step(zero),InteractiveJobCloseStep::Pending{progress:Default::default()});
        assert_eq!(work.output.as_ref().unwrap().child_preparations.front().unwrap().retained_operation_count(),1);
        let body=fixture["closeGrant"].as_u64().unwrap()as usize;
        let mut terminal=false;
        for _ in 0..fixture["closeSteps"].as_u64().unwrap(){
            let grant=quoted_close_grant(&work,body);
            match work.close_step(grant){
                InteractiveJobCloseStep::Pending{progress}=>assert!(progress.fits(grant)),
                InteractiveJobCloseStep::Complete{progress}=>{assert!(progress.fits(grant));assert!(work.terminal_is_empty());terminal=true;break;},
                other=>panic!("[DEBUG] actual child owner did not close: {other:?}"),
            }
        }
        assert!(terminal);work.closing=false;work.completed=false;
    }
    println!("[DEBUG] exact Flow preparation identity, foreign refusal, zero grants and 4096-byte owner close verified");
}
