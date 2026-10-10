use super::*;
use semio_framework_value::{retirement::controlled::ControlledRetirement,retained_clone::RetainedCloneGrant};

#[global_allocator]
static ORIGINAL_COMMAND_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;

fn observe<T>(work:impl FnOnce()->T)->(T,usize,usize){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(work);(value,heap.requested_bytes,heap.released_bytes)}

#[test]
fn original_flow_commands_close_full_sources_with_system_receipts(){
 let corpus:Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for copy in [1,3,64]{for cancel in [0,1,9,37]{
  let(commands,born,free)=observe(unit_tests::every_command);assert_eq!(commands.len(),corpus["commands"].as_array().unwrap().len());let original=born-free;let(mut births,mut releases)=(0,0);
  let(mut owner,born,free)=observe(||ControlledRetirement::new(commands).unwrap_or_else(|_|unreachable!("every defining command must retain full close authority")));assert_eq!((born,free),(0,0));let mut idle=0;let mut cancelled=false;
  for turn in 0..100000{
   if owner.terminal_is_empty(){break}let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};
   let(step,born,free)=observe(||owner.step(RetainedCloneGrant{maximum_items:0,..grant}).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((born,free),(0,0));if turn==cancel{cancelled=true;}
   let(step,born,free)=observe(||owner.step(grant).unwrap());let receipt=step.progress();assert!(receipt.fits(grant));assert_eq!((born,free),(receipt.retained_capacity_bytes,receipt.released_bytes));births+=born;releases+=free;if receipt==Default::default(){idle+=1}else{idle=0}assert!(idle<64,"original full command close stalled at turn {turn} copy {copy}");
  }
  assert!(cancelled);assert!(owner.terminal_is_empty());assert_eq!(original+births,releases);assert_eq!(observe(||drop(owner)),((),0,0));eprintln!("[DEBUG] Original Flow commands=36 nested=7 copy={copy} cancel={cancel} physicalReceipts=true terminalDrop=0");
 }}
}

#[test]
fn original_flow_snapshot_preserves_local_scene_and_full_system_receipts(){
 for copy in [1,3,64]{for cancel in [0,1,9,37]{
  let(source,born,free)=observe(||{let mut snapshot=FlowSnapshot::default();crate::cache_flow_content(&mut snapshot.content,vec![Widget::InputNote{id:"n雪".into(),text:"original🌊️".into()}],Vec::new(),flow::OrderedMap::default());snapshot});let original=born-free;
  let scene=source.content.local_owner::<FlowWorkingScene>().unwrap();let pointer=Arc::as_ptr(&scene);assert_eq!(scene.widgets.len(),1);assert_eq!(observe(||drop(scene)),((),0,0));
  let(mut owner,born,free)=observe(||ControlledRetirement::new(source).unwrap_or_else(|_|unreachable!()));assert_eq!((born,free),(0,0));let(mut births,mut releases,mut idle)=(0,0,0);let mut cancelled=false;
  for turn in 0..100000{if owner.terminal_is_empty(){break}let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};
   let(step,born,free)=observe(||owner.step(RetainedCloneGrant{maximum_items:0,..grant}).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((born,free),(0,0));if let Some(source)=owner.original(){let scene=source.content.local_owner::<FlowWorkingScene>().unwrap();assert_eq!(Arc::as_ptr(&scene),pointer);assert_eq!(observe(||drop(scene)),((),0,0));}if turn==cancel{cancelled=true;}
   let(step,born,free)=observe(||owner.step(grant).unwrap());let receipt=step.progress();assert!(receipt.fits(grant));assert_eq!((born,free),(receipt.retained_capacity_bytes,receipt.released_bytes));births+=born;releases+=free;if receipt==Default::default(){idle+=1}else{idle=0}assert!(idle<64,"original editor snapshot close stalled");}
  assert!(cancelled);assert!(owner.terminal_is_empty());assert_eq!(original+births,releases);assert_eq!(observe(||drop(owner)),((),0,0));eprintln!("[DEBUG] Original editor snapshot localSceneSameArc=true copy={copy} cancel={cancel} physicalReceipts=true terminalDrop=0");
 }}
}

#[test]
fn original_flow_completion_rejection_admits_whole_source_and_exact_system_receipts(){
 type Source=semio_framework_plugin::ArtifactToolCompletionRejection<semio_framework_plugin::EditorApp<FlowPlayApp>>;
 let corpus:Value=serde_json::from_str(include_str!("../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📬️completion/♻️retirement/🚫️refusal/🧫️fixtures/🔣️.json")).unwrap();
 for copy in corpus["copies"].as_array().unwrap().iter().map(|copy|copy.as_u64().unwrap()as usize){for row in corpus["receivingCases"].as_array().unwrap(){
  let(source,born,free)=observe(||{
   let fault=||{let mut fault=Fault::new(semio_framework_diagnostic::FaultOrigin::App,"original-completion-rejection",row["message"].as_str().unwrap());fault.scope.module=Some(row["scope"].as_str().unwrap().into());fault.params=Some(Box::new(semio_framework_diagnostic::FaultParams(vec![("original".into(),row["parameter"].as_str().unwrap().into())])));fault};
   let emit=if row["outcome"]=="fault"{Err(fault())}else{Ok(Emit{effects:vec![Effect::DispatchAction{req:semio_framework_plugin::RequestId(107),action:"flowEvalTick".into(),args:None,delay_ms:0}],extension_invocations:vec![semio_framework_plugin::ExtensionInvocation::new(row["extension"].as_str().unwrap(),"evaluate",row["input"].as_str().unwrap(),"flowEvalResolve")],..Default::default()})};
   Source{emit,ephemeral:semio_framework_plugin::EphemeralEmit::default(),fault:fault()}
  });let original=born-free;
  let pointer=source.fault.message.as_ptr();let capacity=semio_framework_value::retirement::owned_retirement_birth_bytes::<Source>();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:capacity,maximum_release_bytes:0,maximum_depth:1};
  let((error,source),born,free)=observe(||semio_framework_value::retirement::admit_owned_retirement(source,RetainedCloneGrant{maximum_items:0,..grant}).err().unwrap());assert_eq!((born,free),(0,0));assert_eq!(source.fault.message.as_ptr(),pointer);assert_eq!(error.retained_progress(),Default::default());assert_eq!(observe(||drop(error)),((),0,0));
  let((owner,receipt),born,free)=observe(||semio_framework_value::retirement::admit_owned_retirement(source,grant).unwrap_or_else(|_|panic!("whole original rejection admission")));assert!(receipt.fits(grant));assert_eq!((born,free),(receipt.retained_capacity_bytes,receipt.released_bytes));let(mut births,mut releases)=(born,free);let mut owner=Some(owner);let mut idle=0;
  for turn in 0..100000{if owner.is_none(){break}let demand=semio_framework_value::factory_ticket_demands(owner.as_ref().unwrap(),copy).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};let(step,born,free)=observe(||semio_framework_value::close_factory_ticket(&mut owner,RetainedCloneGrant{maximum_items:0,..grant}).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((born,free),(0,0));let(step,born,free)=observe(||semio_framework_value::close_factory_ticket(&mut owner,grant).unwrap());let receipt=step.progress();assert!(receipt.fits(grant));assert_eq!((born,free),(receipt.retained_capacity_bytes,receipt.released_bytes));births+=born;releases+=free;if receipt==Default::default(){idle+=1}else{idle=0}assert!(idle<64,"original whole rejection stalled turn={turn} copy={copy} demand={demand:?}");}
  assert!(owner.is_none());assert_eq!(original+births,releases);assert_eq!(observe(||drop(owner)),((),0,0));eprintln!("[DEBUG] Original whole completion rejection copy={copy} outcome={} exactSystemReceipts=true terminalDrop=0",row["outcome"]);
 }}
}
