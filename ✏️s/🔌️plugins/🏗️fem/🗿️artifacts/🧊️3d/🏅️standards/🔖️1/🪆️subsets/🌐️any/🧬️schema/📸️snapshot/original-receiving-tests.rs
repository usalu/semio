//! 🧪️ Original job caller funds persistent FEM authoring intake and retirement.
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::{Fem3dSnapshot,Fem3dSnapshotValue};
use semio_s_fem_node::volumetric::FemNodeValue;
use semio_framework_value::{ValueError,RetainedWorkOutcome,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetirementDemand,retirement::RetireOwned};
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OperationId,Generation};
use semio_framework_dsl_record::receiving::OwnedSlot;
use std::mem::size_of;
#[global_allocator] static HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
fn stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:2_000_000,maximum_copy_bytes:64*1024*1024,maximum_capacity_bytes:64*1024*1024,maximum_release_bytes:64*1024*1024,maximum_depth:512}}
fn retire_turn(caller:&mut OriginalJobFixtureCaller,run:impl FnOnce(&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>){
 let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(stock(),|cx|cx.with_original_retirement(run)));result.unwrap();let receipt=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));
}
fn close<T:RetireOwned>(caller:&mut OriginalJobFixtureCaller,value:T){
 let mut value=Some(value);let mut slot=OwnedSlot::<T>::default();let d=RetirementDemand{copy_bytes:size_of::<Option<T>>()+size_of::<OwnedSlot<T>>(),depth:1,..Default::default()};
 retire_turn(caller,|cx|{semio_framework_job::close_original_retained_step(cx,d,|_|{*slot.value=value.take();RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),work:1}})?;Ok(())});
 while !slot.terminal_is_empty(){let d=slot.close_demands(4096).unwrap();retire_turn(caller,|cx|{semio_framework_job::close_original_retained_step(cx,d,|g|{let turn=slot.close_step(g);RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work}})?;Ok(())});}
}
#[test]
fn original_job_receives_and_retires_fem_rows(){
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(943),Generation(9),stock(),||Some(0),u64::MAX,10_000_000);
 let mut decoder=OriginalJobDecodeOwner::empty();
 let mut source=Some(Fem3dSnapshotValue::default());source.as_mut().unwrap().nodes.push(FemNodeValue{id:"a".into(),x:1.,y:2.,z:3.,});
 let mut output=None;let mut published=None;
 let (outer,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(stock(),&mut decoder,|cx|{
  let grant=cx.retained_grant();cx.with_native_receiving::<_,ValueError>(|native|{let mut owner=semio_os_kernel::NativeSnapshotDecodeOwner::new(native,grant);output=Some(Fem3dSnapshot::receive_authoring(&mut source,&mut published,&mut owner));Ok(())})?;
  let work=output.as_ref().unwrap().work;let progress=cx.receiving_child_progress();if let Some(grant)=cx.receiving_issued_grant(){cx.consume_receiving_retained(grant,progress)?}cx.consume_fuel(work);Ok(())
 }));outer.unwrap();let receipt=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));assert_eq!(caller.stock(),stock());
 let turn=output.unwrap();assert!(turn.work>0);turn.result.unwrap();let snapshot=published.take().unwrap();assert_eq!(snapshot.nodes.len(),1);assert_eq!(snapshot.nodes.get(0).unwrap().id.as_str(),"a");close(&mut caller,snapshot);
 while !decoder.terminal_is_empty(){retire_turn(&mut caller,|cx|{let turn=decoder.close_step(cx.retained_grant());let step=turn.result?;cx.consume_retained(step.progress())?;cx.consume_fuel(turn.work);Ok(())});}
 if let Some(source)=source{close(&mut caller,source)}caller.retire(stock());
}

#[test]
fn original_cancelled_caller_preserves_the_failure_and_source(){
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(945),Generation(10),stock(),||Some(0),u64::MAX,10_000_000);
 let token=caller.admit_alias(stock());token.cancel_now();let mut decoder=OriginalJobDecodeOwner::empty();
 let mut source=Some(Fem3dSnapshotValue::default());source.as_mut().unwrap().nodes.push(FemNodeValue{id:"a".into(),x:0.,y:0.,z:0.,});let mut outcome=None;let mut published=None;
 let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(stock(),&mut decoder,|cx|{let grant=cx.retained_grant();let native_result=cx.with_native_receiving::<_,ValueError>(|native|{let mut owner=semio_os_kernel::NativeSnapshotDecodeOwner::new(native,grant);outcome=Some(Fem3dSnapshot::receive_authoring(&mut source,&mut published,&mut owner));Ok(())});let work=outcome.as_ref().map_or(0,|turn|turn.work);let progress=cx.receiving_child_progress();if let Some(grant)=cx.receiving_issued_grant(){cx.consume_receiving_retained(grant,progress)?}cx.consume_fuel(work);if let Err(error)=native_result{assert!(outcome.is_none());outcome=Some(RetainedWorkOutcome{result:Err(error.into()),work:0});}Ok(())}));
 result.unwrap();let receipt=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));let turn=outcome.unwrap();let error=turn.result.unwrap_err();assert_eq!(error.kind(),semio_framework_value::ValueRefusalKind::Canceled);assert!(source.is_some());assert!(published.is_none());close(&mut caller,error);caller.return_alias(token,stock());
 while !decoder.terminal_is_empty(){retire_turn(&mut caller,|cx|{let turn=decoder.close_step(cx.retained_grant());let step=turn.result?;cx.consume_retained(step.progress())?;cx.consume_fuel(turn.work);Ok(())});}
 if let Some(source)=source{close(&mut caller,source)}caller.retire(stock());
}

#[test]
fn original_zero_grant_retains_the_unadmitted_authoring_owner(){
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(947),Generation(11),stock(),||Some(0),u64::MAX,10_000_000);let mut decoder=OriginalJobDecodeOwner::empty();
 let mut source=Some(Fem3dSnapshotValue::default());source.as_mut().unwrap().nodes.push(FemNodeValue{id:"original".into(),x:1.,y:2.,z:3.,});let mut outcome=None;let mut published=None;
 let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(RetainedCloneGrant::default(),&mut decoder,|cx|{let grant=cx.retained_grant();let received=cx.with_native_receiving::<_,ValueError>(|native|{let mut owner=semio_os_kernel::NativeSnapshotDecodeOwner::new(native,grant);outcome=Some(Fem3dSnapshot::receive_authoring(&mut source,&mut published,&mut owner));Ok(())});let work=outcome.as_ref().map_or(0,|turn|turn.work);let progress=cx.receiving_child_progress();if let Some(issued)=cx.receiving_issued_grant(){cx.consume_receiving_retained(issued,progress)?}cx.consume_fuel(work);if let Err(error)=received{assert!(outcome.is_none());outcome=Some(RetainedWorkOutcome{result:Err(error.into()),work:0});}Ok(())}));
 result.unwrap();let receipt=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));assert_eq!(caller.stock(),stock());let turn=outcome.unwrap();assert_eq!(turn.work,0);let error=turn.result.unwrap_err();assert_eq!(error.kind(),semio_framework_value::ValueRefusalKind::OwnershipLimit);assert_eq!(source.as_ref().unwrap().nodes[0].id,"original");assert!(published.is_none());close(&mut caller,error);
 while !decoder.terminal_is_empty(){retire_turn(&mut caller,|cx|{let turn=decoder.close_step(cx.retained_grant());let step=turn.result?;cx.consume_retained(step.progress())?;cx.consume_fuel(turn.work);Ok(())});}if let Some(source)=source{close(&mut caller,source)}caller.retire(stock());
}

#[test]
fn original_job_receives_typed_record_into_caller_slot(){
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(949),Generation(12),stock(),||Some(0),u64::MAX,10_000_000);let mut decoder=OriginalJobDecodeOwner::empty();
 let mut source=Fem3dSnapshotValue::default();source.nodes.push(FemNodeValue{id:"node".into(),x:1.,y:2.,z:3.,});let record=source.__dsl_to_record();let(mut output,mut outcome)=(None,None);
 let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(stock(),&mut decoder,|cx|{let grant=cx.retained_grant();cx.with_native_receiving::<_,ValueError>(|native|{let mut owner=semio_os_kernel::NativeSnapshotDecodeOwner::new(native,grant);outcome=Some(Fem3dSnapshot::receive_exchange_record(&record,&mut output,&mut owner));Ok(())})?;let work=outcome.as_ref().unwrap().work;let progress=cx.receiving_child_progress();if let Some(issued)=cx.receiving_issued_grant(){cx.consume_receiving_retained(issued,progress)?}cx.consume_fuel(work);Ok(())}));
 result.unwrap();let receipt=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));let turn=outcome.unwrap();assert!(turn.work>0);turn.result.unwrap();let snapshot=output.take().unwrap();assert_eq!(snapshot.nodes.len(),1);assert_eq!(snapshot.nodes.get(0).unwrap().id.as_str(),"node");close(&mut caller,snapshot);
 while !decoder.terminal_is_empty(){retire_turn(&mut caller,|cx|{let turn=decoder.close_step(cx.retained_grant());let step=turn.result?;cx.consume_retained(step.progress())?;cx.consume_fuel(turn.work);Ok(())});}close(&mut caller,record);close(&mut caller,source);caller.retire(stock());
}
