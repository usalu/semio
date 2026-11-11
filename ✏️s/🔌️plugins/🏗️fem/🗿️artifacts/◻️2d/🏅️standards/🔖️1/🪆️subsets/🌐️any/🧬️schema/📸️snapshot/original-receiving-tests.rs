//! 🧪️ Original job caller funds persistent FEM authoring intake and retirement.
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::{Fem2dSnapshot,Fem2dSnapshotValue};
use semio_s_fem_node::planar::FemNodeValue;
use semio_s_fem_receiving::reply::settled;
use semio_framework_value::{RetainedCloneGrant,RetainedWorkOutcome,ValueRefusalKind,retirement::{RetireOwned,controlled::ControlledRetirement}};
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OperationId,Generation,close_original_effect_step};
use semio_os_kernel::{NativeSnapshotDecodeOwner,NativeSnapshotFailure};
const BODY:usize=4096;
fn stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:2_000_000,maximum_copy_bytes:64*1024*1024,maximum_capacity_bytes:64*1024*1024,maximum_release_bytes:64*1024*1024,maximum_depth:512}}
fn retire_turn(caller:&mut OriginalJobFixtureCaller,run:impl FnOnce(&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>){
 let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(stock(),|cx|cx.with_original_retirement(run)));result.unwrap();let receipt=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));
}
fn close<T:RetireOwned>(caller:&mut OriginalJobFixtureCaller,value:T){
 let Ok(mut owner)=ControlledRetirement::new(value) else{panic!("owned value refused its controlled retirement")};
 while !owner.terminal_is_empty(){let demand=owner.next_demand(BODY).unwrap();retire_turn(caller,|cx|{close_original_effect_step(cx,demand,|effect|settled(effect,|effect,reply|owner.step_into(BODY,effect,reply)))?;Ok(())});}
}
fn close_decoder(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner){while !decoder.terminal_is_empty(){retire_turn(caller,|cx|{decoder.close_on_source(cx)?;Ok(())});}}
fn received<T>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,grant:RetainedCloneGrant,mut run:impl FnMut(&mut NativeSnapshotDecodeOwner<'_,'_>)->RetainedWorkOutcome<T,NativeSnapshotFailure>)->Option<RetainedWorkOutcome<T,NativeSnapshotFailure>>{
 for _ in 0..4096{
  let mut produced=None;
  let(outer,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(grant,decoder,|cx|{
   let mut scope=None;
   cx.with_native_receiving(&mut scope,|native,issued|{let mut owner=NativeSnapshotDecodeOwner::new(native,issued);let turn=run(&mut owner);produced=Some((turn,owner.progress()));Ok(())})?;
   if let Some((turn,progress))=produced.as_ref(){let issued=cx.receiving_issued_grant().expect("original receiving issued its grant");cx.consume_receiving_retained(issued,*progress)?;cx.consume_fuel(u64::try_from(turn.work).expect("turn work fits fuel"));}
   drop(scope);Ok(())
  }));outer.unwrap();
  let receipt=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));
  if let Some((turn,_))=produced{return Some(turn)}
 }
 None
}
#[test]
fn original_job_receives_and_retires_fem_rows(){
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(942),Generation(9),stock(),||Some(0),u64::MAX,10_000_000);
 let mut decoder=OriginalJobDecodeOwner::empty();
 let mut source=Some(Fem2dSnapshotValue::default());source.as_mut().unwrap().nodes.push(FemNodeValue{id:"a".into(),x:1.,y:2.,});
 let mut published=None;
 let turn=received(&mut caller,&mut decoder,stock(),|owner|Fem2dSnapshot::receive_authoring(&mut source,&mut published,owner)).expect("the original receiving scope ran");
 assert!(turn.work>0);turn.result.unwrap();assert_eq!(caller.stock(),stock());
 let snapshot=published.take().unwrap();assert_eq!(snapshot.nodes.len(),1);assert_eq!(snapshot.nodes.get(0).unwrap().id.as_str(),"a");close(&mut caller,snapshot);
 close_decoder(&mut caller,&mut decoder);
 if let Some(source)=source{close(&mut caller,source)}caller.retire(stock());
}

#[test]
fn original_cancelled_caller_preserves_the_source_and_runs_no_receiving(){
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(944),Generation(10),stock(),||Some(0),u64::MAX,10_000_000);
 let token=caller.admit_alias(stock());token.cancel_now();let mut decoder=OriginalJobDecodeOwner::empty();
 let mut source=Some(Fem2dSnapshotValue::default());source.as_mut().unwrap().nodes.push(FemNodeValue{id:"a".into(),x:0.,y:0.,});let published:Option<Fem2dSnapshot>=None;
 for _ in 0..64{
  let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(stock(),&mut decoder,|cx|{let mut ran=false;let mut scope=None;cx.with_native_receiving(&mut scope,|_,_|{ran=true;Ok(())})?;assert!(!ran,"a cancelled caller runs no receiving");Ok(())}));
  result.unwrap();let receipt=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));
 }
 assert!(source.is_some());assert!(published.is_none());assert_eq!(source.as_ref().unwrap().nodes[0].id,"a");
 caller.return_alias(token,stock());close_decoder(&mut caller,&mut decoder);
 if let Some(source)=source{close(&mut caller,source)}caller.retire(stock());
}

#[test]
fn original_zero_grant_retains_the_unadmitted_authoring_owner(){
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(946),Generation(11),stock(),||Some(0),u64::MAX,10_000_000);let mut decoder=OriginalJobDecodeOwner::empty();
 let mut source=Some(Fem2dSnapshotValue::default());source.as_mut().unwrap().nodes.push(FemNodeValue{id:"original".into(),x:1.,y:2.,});let mut published=None;
 let turn=received(&mut caller,&mut decoder,RetainedCloneGrant::default(),|owner|Fem2dSnapshot::receive_authoring(&mut source,&mut published,owner));
 if let Some(turn)=turn{assert_eq!(turn.work,0);let error=turn.result.unwrap_err();assert_eq!(error.kind(),ValueRefusalKind::OwnershipLimit);close(&mut caller,error);}
 assert_eq!(source.as_ref().unwrap().nodes[0].id,"original");assert!(published.is_none());assert_eq!(caller.stock(),stock());
 close_decoder(&mut caller,&mut decoder);
 if let Some(source)=source{close(&mut caller,source)}caller.retire(stock());
}

#[test]
fn original_job_receives_typed_record_into_caller_slot(){
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(948),Generation(12),stock(),||Some(0),u64::MAX,10_000_000);let mut decoder=OriginalJobDecodeOwner::empty();
 let mut source=Fem2dSnapshotValue::default();source.nodes.push(FemNodeValue{id:"node".into(),x:1.,y:2.,});let record=source.__dsl_to_record();let mut output=None;
 let turn=received(&mut caller,&mut decoder,stock(),|owner|Fem2dSnapshot::receive_exchange_record(&record,&mut output,owner)).expect("the original receiving scope ran");
 assert!(turn.work>0);turn.result.unwrap();let snapshot=output.take().unwrap();assert_eq!(snapshot.nodes.len(),1);assert_eq!(snapshot.nodes.get(0).unwrap().id.as_str(),"node");close(&mut caller,snapshot);
 close_decoder(&mut caller,&mut decoder);close(&mut caller,record);close(&mut caller,source);caller.retire(stock());
}
