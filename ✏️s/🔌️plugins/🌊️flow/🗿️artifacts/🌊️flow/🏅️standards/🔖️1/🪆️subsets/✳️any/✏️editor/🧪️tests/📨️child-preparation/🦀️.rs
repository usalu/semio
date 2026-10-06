use super::*;

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
        assert_eq!(work.close_step(0,4096),InteractiveJobCloseStep::Pending{released_items:0,released_bytes:0});
        assert_eq!(work.close_step(1,0),InteractiveJobCloseStep::Pending{released_items:0,released_bytes:0});
        assert_eq!(work.output.as_ref().unwrap().child_preparations.front().unwrap().retained_operation_count(),1);
        let grant=fixture["closeGrant"].as_u64().unwrap()as usize;
        let mut terminal=false;
        for _ in 0..fixture["closeSteps"].as_u64().unwrap(){
            match work.close_step(1,grant){
                InteractiveJobCloseStep::Pending{released_bytes,..}=>assert!(released_bytes<=grant),
                InteractiveJobCloseStep::Complete=>{assert!(work.terminal_is_empty());terminal=true;break;},
                InteractiveJobCloseStep::Blocked=>panic!("[DEBUG] actual child owner blocked: source={:?}, refusal={:?}, retirement={:?}, preparations={}, groups={}",work.output.as_ref().and_then(|emit|emit.child_preparations.front()).map(|source|source.retained_operation_count()),work.output.as_ref().and_then(|emit|emit.child_preparations.front()).and_then(|source|source.refusal()),work.output.as_ref().and_then(|emit|emit.child_preparations.front()).and_then(|source|source.retirement_refusal()),work.output.as_ref().map_or(0,|emit|emit.child_preparations.len()),work.output.as_ref().map_or(0,|emit|emit.child_emits.len())),
            }
        }
        assert!(terminal);work.closing=false;work.completed=false;
    }
    println!("[DEBUG] exact Flow preparation identity, foreign refusal, zero grants and 4096-byte owner close verified");
}
