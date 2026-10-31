//! 🧪️ Real original Writer fold receivers preserve input owners and close every native allocation.
use super::*;
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OperationId,Generation};
use semio_framework_value::{RetainedCloneGrant,retirement::{RetireOwned,controlled::ControlledRetirement}};
#[global_allocator]
static HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
fn clock()->Option<u64>{Some(1)}
fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:4096}}
fn caller()->OriginalJobFixtureCaller{OriginalJobFixtureCaller::admit(OperationId(619),Generation(19),RetainedCloneGrant{maximum_items:1<<24,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<30,maximum_release_bytes:1<<30,maximum_depth:4096},clock,u64::MAX,1<<24)}
fn turn<T>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,g:RetainedCloneGrant,run:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->T{
 let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(g,decoder,run));let p=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));result.unwrap()
}
fn retire<T:RetireOwned+Send+'static>(value:T,caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner){
 let mut owner=ControlledRetirement::retain(value);
 for step in 0..1000000{if owner.terminal_is_empty(){return}let d=owner.next_demand(32768).unwrap();turn(caller,decoder,grant(),|cx|cx.with_original_retirement(|cx|{close_original_retained_step(cx,d,|g|owner.step(g))?;Ok(())}));assert!(step<999999)}
}
fn close(producer:&mut Option<Box<dyn ArtifactOriginalTypingFoldProducer<WriterMutation>>>,caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner){
 for step in 0..1000000{
  if producer.is_none(){return}
  if !producer.as_ref().unwrap().terminal_is_empty(){turn(caller,decoder,RetainedCloneGrant{maximum_items:0,..grant()},|cx|cx.with_original_retirement(|cx|{assert!(!producer.as_mut().unwrap().close_step(cx)?);Ok(())}));assert_eq!(caller.receipt(),Default::default());}
  turn(caller,decoder,grant(),|cx|cx.with_original_retirement(|cx|{
   if producer.as_ref().unwrap().terminal_is_empty(){let d=RetirementDemand{copy_bytes:2*size_of::<Option<Box<dyn ArtifactOriginalTypingFoldProducer<WriterMutation>>>>(),release_bytes:std::mem::size_of_val(&**producer.as_ref().unwrap()),depth:1,..Default::default()};close_original_retained_step(cx,d,|_|{drop(producer.take());Ok(RetainedCloneStep::Complete(receipt(d)))})?;}else{producer.as_mut().unwrap().close_step(cx)?;}Ok(())
  }));assert!(step<999999)
 }
}
fn finish(mut caller:OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner){
 while !decoder.terminal_is_empty(){let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(grant(),|cx|cx.with_original_retirement(|cx|{let d=decoder.retirement_demands(32768)?;close_original_retained_step(cx,d,|g|decoder.close_step(g))?;Ok(())})));let p=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));result.unwrap()}caller.retire(grant());
}
fn inputs(row:&serde_json::Value)->(Vec<WriterMutation>,Vec<WriterMutation>){
 if let Some(id)=row["compositionCase"].as_str(){let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/✂️text-splice/🔣️.json")).unwrap();let source=fixture["compositions"].as_array().unwrap().iter().find(|v|v["id"].as_str()==Some(id)).unwrap();let mutation=|splice:&serde_json::Value|{let mut value=splice.clone();value.as_object_mut().unwrap().insert("mutation".into(),"spliceText".into());serde_json::from_value(value).unwrap()};(vec![mutation(&source["net"])],vec![mutation(&source["next"])])}else{(serde_json::from_value(row["previous"].clone()).unwrap(),serde_json::from_value(row["incoming"].clone()).unwrap())}
}
fn result(value:&OriginalTypingFold<WriterMutation>)->serde_json::Value{match value{OriginalTypingFold::Split=>serde_json::json!({"kind":"split"}),OriginalTypingFold::ReplaceIncoming=>serde_json::json!({"kind":"replaceIncoming"}),OriginalTypingFold::Net(net)=>serde_json::json!({"kind":"net","net":net})}}
#[test]
fn original_writer_fold_matches_neutral_inputs_without_moving_or_cloning_original_nets(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let(previous,incoming)=inputs(row);let pointers=(previous.as_ptr(),incoming.as_ptr());let before=serde_json::json!([previous,incoming]);let mut caller=caller();let mut decoder=OriginalJobDecodeOwner::empty();let mut producer=None;
  for denied in [RetainedCloneGrant{maximum_items:0,..grant()},RetainedCloneGrant{maximum_copy_bytes:0,..grant()},RetainedCloneGrant{maximum_capacity_bytes:0,..grant()},RetainedCloneGrant{maximum_depth:0,..grant()}]{assert!(!turn(&mut caller,&mut decoder,denied,|cx|WRITER_ORIGINAL_TYPING_FOLD.admit(&previous,&incoming,cx,&mut producer)));assert!(producer.is_none());assert_eq!(caller.receipt(),Default::default())}
  assert!(turn(&mut caller,&mut decoder,grant(),|cx|WRITER_ORIGINAL_TYPING_FOLD.admit(&previous,&incoming,cx,&mut producer)));
  let mut foreign=OriginalJobDecodeOwner::empty();turn(&mut caller,&mut foreign,grant(),|cx|{assert!(producer.as_mut().unwrap().advance(&previous,&incoming,cx).is_err());Ok(())});assert!(foreign.terminal_is_empty());
  for step in 0..1000000{if producer.as_ref().unwrap().ready(){break}assert!(!turn(&mut caller,&mut decoder,RetainedCloneGrant{maximum_items:0,..grant()},|cx|producer.as_mut().unwrap().advance(&previous,&incoming,cx)));assert_eq!(caller.receipt(),Default::default());turn(&mut caller,&mut decoder,grant(),|cx|producer.as_mut().unwrap().advance(&previous,&incoming,cx));assert!(step<999999)}
  let mut output=None;assert!(!turn(&mut caller,&mut decoder,RetainedCloneGrant{maximum_items:0,..grant()},|cx|producer.as_mut().unwrap().take_into(&mut output,cx)));assert!(output.is_none());assert!(turn(&mut caller,&mut decoder,grant(),|cx|producer.as_mut().unwrap().take_into(&mut output,cx)));
  assert_eq!(pointers,(previous.as_ptr(),incoming.as_ptr()));assert_eq!(serde_json::json!([previous,incoming]),before);let actual=result(output.as_ref().unwrap());assert_eq!(actual["kind"],row["expected"]);println!("[writer-native-fold] {}",serde_json::json!({"name":row["id"],"result":actual}));
  close(&mut producer,&mut caller,&mut decoder);retire(output,&mut caller,&mut decoder);retire(previous,&mut caller,&mut decoder);retire(incoming,&mut caller,&mut decoder);finish(caller,&mut decoder);
 }
}
#[test]
fn original_writer_fold_cancel_keeps_borrowed_inputs_until_all_partial_native_owners_close(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🔣️.json")).unwrap();let row=fixture["cases"].as_array().unwrap().iter().find(|row|row["id"]=="adjacent-native-splice").unwrap();
 for cut in [0,1,2,4,8,32]{let(previous,incoming)=inputs(row);let before=serde_json::json!([previous,incoming]);let mut caller=caller();let mut decoder=OriginalJobDecodeOwner::empty();let mut producer=None;turn(&mut caller,&mut decoder,grant(),|cx|WRITER_ORIGINAL_TYPING_FOLD.admit(&previous,&incoming,cx,&mut producer));for _ in 0..cut{if producer.as_ref().unwrap().ready(){break}turn(&mut caller,&mut decoder,grant(),|cx|producer.as_mut().unwrap().advance(&previous,&incoming,cx));}caller.cancel().cancel_now();close(&mut producer,&mut caller,&mut decoder);assert!(producer.is_none());assert_eq!(serde_json::json!([previous,incoming]),before);retire(previous,&mut caller,&mut decoder);retire(incoming,&mut caller,&mut decoder);finish(caller,&mut decoder);}
}
