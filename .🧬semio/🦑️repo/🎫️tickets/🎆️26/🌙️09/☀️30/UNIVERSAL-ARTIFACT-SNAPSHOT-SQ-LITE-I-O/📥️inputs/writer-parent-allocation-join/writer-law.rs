#[test]
fn writer_parent_return_keeps_actual_allocations_and_typed_fault_until_physical_close(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/📦️parent-return.json")).unwrap();
    let mut emit:Emit<WriterMutation,NoConfigMutation,NoDraftMutation>=Emit::default();
    emit.child_emits.push(semio_framework_plugin::app::ChildEmit::open(fixture["slot"].as_str().unwrap(),fixture["childId"].as_str().unwrap(),0));
    let rejected=ArtifactToolCompletionRejection::<EditorApp<WriterPlayApp>>{emit:Ok(emit),ephemeral:EphemeralEmit::default(),fault:Fault::new(semio_framework_plugin::FaultOrigin::Framework,semio_framework_plugin::FaultCode::new(fixture["faultCode"].as_str().unwrap()),fixture["faultMessage"].as_str().unwrap())};
    let scope=rejected.fault.scope.as_ref()as*const _;let message=rejected.fault.message.as_ptr();
    let mut job=writer_command_job(WriterCommand::EngagementSubmit(engagement_submit::EngagementSubmit{value:Some("lint".into())}),Arc::from("writer text"));job.raw_bytes=Vec::new();job.pending_completion_rejection=Some(rejected);job.begin_close();
    assert_eq!(job.close_step(0,1),InteractiveJobCloseStep::Pending{released_items:0,released_bytes:0});
    for _ in 0..fixture["childTurns"].as_u64().unwrap(){if job.pending_completion_rejection.is_none(){break;}let step=job.close_step(1,fixture["childBytes"].as_u64().unwrap()as usize);if let InteractiveJobCloseStep::Pending{released_items,released_bytes}=step{assert!(released_items<=1&&released_bytes<=4);}}
    assert!(job.pending_completion_rejection.is_none());assert!(job.command.is_some());assert!(!job.terminal_is_empty());assert!(job.returned_allocations.retained_bytes()>0);let fault=job.returned_fault.fault().unwrap();assert_eq!(fault.scope.as_ref()as*const _,scope);assert_eq!(fault.message.as_ptr(),message);assert_eq!(fault.code.0,fixture["faultCode"].as_str().unwrap());assert_eq!(fault.message,fixture["faultMessage"].as_str().unwrap());
    for _ in 0..fixture["parentTurns"].as_u64().unwrap(){if job.terminal_is_empty(){break;}let _=job.close_step(1,fixture["parentBytes"].as_u64().unwrap()as usize);}
    assert!(job.returned_allocations.terminal_is_empty());assert_eq!(job.returned_allocations.retained_bytes(),0);assert!(job.returned_fault.terminal_is_empty());assert!(job.terminal_is_empty());eprintln!("[DEBUG] Writer exact typed rejection handed genuine allocations to registered job parent; physical parent and fault owners empty only after original4096 grant");
}
