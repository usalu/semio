#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🎞️tick/🛬️native/🦀️.rs"]
mod native;

#[global_allocator]
static HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
use semio_framework_job::*;
use semio_framework_tool_run::*;
use semio_framework_tool_run_tick_wire::tick_receive::OriginalTickWireSource;
use semio_framework_value::retirement::controlled::ControlledRetirement;
struct Source<'a>{bytes:&'a[u8],page:usize}
impl OriginalTickWireSource for Source<'_>{fn identity(&self)->usize{self.bytes.as_ptr()as usize}fn byte_len(&self)->usize{self.bytes.len()}fn page(&self,index:usize)->Option<&[u8]>{let start=index.checked_mul(self.page)?;self.bytes.get(start..start.saturating_add(self.page).min(self.bytes.len()))}}
const GRANT:RetainedCloneGrant=RetainedCloneGrant{maximum_items:64,maximum_copy_bytes:32768,maximum_capacity_bytes:1<<20,maximum_release_bytes:2<<20,maximum_depth:128};
#[test]
fn actual_native_preview_pages_publish_nine_counters_all_ranges_and_keep_captures(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
 let identity=ToolRunIdentity::new(ToolRunId{app_instance_id:71,run:9},[23;32]);
 let payload:Vec<u8>=(0..law["payloadBytes"].as_u64().unwrap()as usize).map(|index|((index*17+3)%251)as u8).collect();
 for order in law["orders"].as_array().unwrap(){
  let stock=RetainedCloneGrant{maximum_items:1<<30,maximum_copy_bytes:1<<29,maximum_capacity_bytes:1<<26,maximum_release_bytes:1<<26,maximum_depth:4096};
  let mut original=OriginalJobFixtureCaller::admit(OperationId(901),Generation(7),stock,default_now_us,u64::MAX,1<<28);
  let mut decoder=OriginalJobDecodeOwner::empty();let mut state=native::OriginalToolRunNativeState::empty();let mut capture=None;
  let mut born=0;let mut released=0;
  let chunk=law["chunkBytes"].as_u64().unwrap()as usize;
  for (index,bytes)in payload.chunks(chunk).enumerate(){
   let tick=ToolRunTick{identity,sequence:index as u64+1,progress:Some(ToolRunProgress{identity,sequence:index as u64+1,state:ToolRunState::Running,stage:3,completed:index as u64+1,total:Some(2),counters:law["counters"].as_array().unwrap().iter().enumerate().map(|(counter,value)|ToolRunCounter{counter:counter as u16,value:value.as_u64().unwrap()}).collect(),units_per_second:0.0,conflicts:0,steps:ToolRunStepRing::new()}),steps:Vec::new(),trace:Vec::new(),append_ops:Vec::new(),append_entities:Vec::new(),retract_to:None,payload:Some(bytes.to_vec()),payload_update:Some(ToolRunPayloadUpdate::Range{publication:1,total_bytes:payload.len()as u64,offset:(index*chunk)as u64,chunk_bytes:chunk as u32,final_chunk:(index+1)*chunk>=payload.len()})};
   let wire=tick.encode().unwrap();let pointer=wire.as_ptr();let source=Source{bytes:&wire,page:law["sourcePageBytes"].as_u64().unwrap()as usize};let mut done=false;
   for _ in 0..law["maximumTurns"].as_u64().unwrap(){
    let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||original.turn_with_decoder(GRANT,&mut decoder,|cx|{
     for _ in 0..32{if cx.should_yield(){break}if state.initialize("energySimulation",identity,ToolRunState::Running,cx)?&&state.receive(&source,identity,ToolRunState::Running,cx)?{return Ok(true)}}Ok(false)
    }));
    done=result.unwrap();let p=original.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));born+=heap.requested_bytes;released+=heap.released_bytes;assert_eq!(wire.as_ptr(),pointer);if done{break}
   }
   assert!(done,"original native preview must complete under its finite source stock");
   let view=state.borrowed_view(identity,ToolRunState::Running).unwrap();assert_eq!(view.progress.counters.len(),9);assert_eq!(view.progress.completed,index as u64+1);let published=view.payload.unwrap();assert_eq!(published.received_bytes(),(index*chunk+bytes.len()));
   if index==0{assert!(!published.complete());original.turn_with_decoder(GRANT,&mut decoder,|cx|{let(value,p)=state.capture(identity,ToolRunState::Running,cx.retained_grant())?.unwrap();capture=Some(value);cx.consume_retained(p)?;Ok(())}).unwrap();}
   else{assert!(published.complete());for start in [0,chunk-31,chunk,payload.len()-29]{let count=29;let actual=published.read_fixed::<29>(start).unwrap();assert_eq!(actual.as_slice(),&payload[start..start+count]);}assert!(!capture.as_ref().unwrap().borrowed_view().payload.unwrap().complete());}
  }
  let mut capture_close=ControlledRetirement::new(capture.take()).ok().unwrap();
  for native_first in [order.as_str().unwrap()=="producerFirst",order.as_str().unwrap()!="producerFirst"]{
   for turn in 0..200000{
    if native_first&&state.terminal_is_empty()||!native_first&&capture_close.terminal_is_empty(){break}
    let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||original.turn_with_decoder(GRANT,&mut decoder,|cx|cx.with_original_retirement(|cx|{
     let demand=if native_first{state.next_close_demands(4096)?}else{capture_close.selected_demand(4096)?};
     close_original_retained_step(cx,demand,|grant|if native_first{state.close_step(grant)}else{capture_close.step(grant)})?;Ok(())
    })));
    result.unwrap();let p=original.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));born+=heap.requested_bytes;released+=heap.released_bytes;assert!(turn<199999,"original native preview close must progress");
   }
  }
  assert!(state.terminal_is_empty());assert!(capture_close.terminal_is_empty());assert_eq!(born,released);drop((state,capture_close));while !decoder.terminal_is_empty(){original.turn(GRANT,|cx|cx.with_original_retirement(|cx|{let d=decoder.retirement_demands(4096)?;close_original_retained_step(cx,d,|grant|decoder.close_step(grant))?;Ok(())})).unwrap();}original.retire(GRANT);
 }
}

struct PreviewProducer<'a>{wire:&'a[u8],publication:semio_framework_job_publication::RetainedJobPublication,delivered:bool,resumed:usize,closing:bool}
impl InteractiveJob for PreviewProducer<'_>{
 fn step<'a>(&'a mut self,cx:&mut StepContext<'_>)->Result<Option<JobOutcomeBorrow<'a>>,OriginalTurnFailure>{
  if self.delivered{self.resumed+=1;if !self.publication.terminal_is_empty(){let d=self.publication.retirement_demands()?;close_original_retained_step(cx,d,|grant|self.publication.close_step(grant))?;return Ok(None)}return JobOutcomeBorrow::admit_complete(cx,None,None).map_err(Into::into)}
  let result=self.publication.advance_from_source(semio_framework_job_publication::JobPublicationKind::Preview,self.wire,cx)?;if result.is_some(){self.delivered=true}Ok(result)
 }
 fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,semio_framework_value::ValueError>{match descriptor.kind(){JobOutcomeKind::Complete=>descriptor.complete(None,None),_=>self.publication.borrow_outcome(descriptor)}}
 fn begin_close(&mut self){self.closing=true;}
 fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{let before=cx.retained_progress();let d=self.publication.retirement_demands()?;close_original_retained_step(cx,d,|grant|self.publication.close_step(grant))?;let after=cx.retained_progress();let progress=RetainedCloneProgress{copied_items:after.copied_items-before.copied_items,copied_bytes:after.copied_bytes-before.copied_bytes,retained_capacity_bytes:after.retained_capacity_bytes-before.retained_capacity_bytes,released_bytes:after.released_bytes-before.released_bytes};Ok(if self.terminal_is_empty(){InteractiveJobCloseStep::Complete{progress}}else{InteractiveJobCloseStep::Pending{progress}})}
 fn terminal_is_empty(&self)->bool{self.closing&&self.publication.terminal_is_empty()}
}
#[test]
fn original_job_preview_updates_presentation_before_acknowledgement_and_producer_resumption(){
 let identity=ToolRunIdentity::new(ToolRunId{app_instance_id:71,run:10},[24;32]);
 let tick=ToolRunTick{identity,sequence:7,progress:Some(ToolRunProgress{identity,sequence:7,state:ToolRunState::Running,stage:3,completed:8760,total:Some(8760),counters:(0..9).map(|counter|ToolRunCounter{counter,value:counter as u64*12345}).collect(),units_per_second:0.0,conflicts:0,steps:ToolRunStepRing::new()}),steps:Vec::new(),trace:Vec::new(),append_ops:Vec::new(),append_entities:Vec::new(),retract_to:None,payload:Some(vec![17;65536]),payload_update:Some(ToolRunPayloadUpdate::Range{publication:1,total_bytes:65536,offset:0,chunk_bytes:65536,final_chunk:true})};
 let wire=tick.encode().unwrap();
 let mut producer=PreviewProducer{wire:&wire,publication:semio_framework_job_publication::RetainedJobPublication::new(),delivered:false,resumed:0,closing:false};
 let stock=RetainedCloneGrant{maximum_items:1<<30,maximum_copy_bytes:1<<29,maximum_capacity_bytes:1<<26,maximum_release_bytes:1<<26,maximum_depth:4096};let mut caller=OriginalJobFixtureCaller::admit(OperationId(902),Generation(8),stock,default_now_us,u64::MAX,1<<28);let mut decoder=OriginalJobDecodeOwner::empty();let mut state=native::OriginalToolRunNativeState::empty();let mut outcome=JobOutcomeDescriptorSlot::empty();let mut published=false;let mut complete=false;
 for turn in 0..200000{
  let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(GRANT,&mut decoder,|cx|{
   if let Some(descriptor)=outcome.original(){
    if descriptor.is_acknowledged(){let d=outcome.retirement_demands();close_original_retained_step(cx,d,|grant|Ok(outcome.close_step(grant)))?;return Ok(())}
    match producer.borrow_outcome(descriptor)?{
     JobOutcomeView::PreviewReady{payload,..}=>{assert_eq!(producer.resumed,0);let source=native::OriginalPreviewPages(payload);if state.initialize("energySimulation",identity,ToolRunState::Running,cx)?&&state.receive(&source,identity,ToolRunState::Running,cx)?{let view=state.borrowed_view(identity,ToolRunState::Running).unwrap();assert_eq!(view.progress.completed,8760);assert_eq!(view.progress.counters.len(),9);assert_eq!(view.payload.unwrap().read_fixed::<29>(65536-29).unwrap(),[17;29]);published=true;}}
     JobOutcomeView::Complete{..}=>{assert!(published);assert!(producer.resumed>0);complete=true;},_=>panic!("fixture has only real preview and complete outcomes")
    }
    if published{let d=outcome.retirement_demands();close_original_retained_step(cx,d,|grant|Ok(outcome.close_step(grant)))?;}
   }else{let mut verdict=None;if let Some(loan)=drive_step(&mut producer,cx,"energy.native-preview",InteractiveStage::InteractiveStep,&mut verdict)?{outcome.retain(loan.into_descriptor()).unwrap();}}
   Ok(())
  }));result.unwrap();let p=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));if complete&&outcome.is_empty(){break}assert!(turn<199999,"actual producer-preview-presentation boundary must finish");
 }
 assert!(published&&complete&&outcome.is_empty());producer.begin_close();
 for turn in 0..200000{if state.terminal_is_empty()&&producer.terminal_is_empty(){break}let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(GRANT,&mut decoder,|cx|cx.with_original_retirement(|cx|{if !state.terminal_is_empty(){let d=state.next_close_demands(4096)?;close_original_retained_step(cx,d,|grant|state.close_step(grant))?;}else{producer.close_step(cx)?;}Ok(())})));result.unwrap();let p=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));assert!(turn<199999);}
 assert!(state.terminal_is_empty()&&producer.terminal_is_empty());while !decoder.terminal_is_empty(){caller.turn(GRANT,|cx|cx.with_original_retirement(|cx|{let d=decoder.retirement_demands(4096)?;close_original_retained_step(cx,d,|grant|decoder.close_step(grant))?;Ok(())})).unwrap();}caller.retire(GRANT);
}

#[test]
fn partial_native_tick_cancels_before_original_source_release_and_restarts_next_generation(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();let law=&law["cancelAndRestart"];let stock=RetainedCloneGrant{maximum_items:1<<30,maximum_copy_bytes:1<<29,maximum_capacity_bytes:1<<26,maximum_release_bytes:1<<26,maximum_depth:4096};let mut caller=OriginalJobFixtureCaller::admit(OperationId(903),Generation(9),stock,default_now_us,u64::MAX,1<<28);let mut decoder=OriginalJobDecodeOwner::empty();let mut state=native::OriginalToolRunNativeState::empty();
 for generation in law["generations"].as_array().unwrap(){let mut identity=ToolRunIdentity::new(ToolRunId{app_instance_id:71,run:11},[25;32]);identity.generation=generation.as_u64().unwrap()as u32;let tick=ToolRunTick{identity,sequence:1,progress:Some(ToolRunProgress{identity,sequence:1,state:ToolRunState::Running,stage:3,completed:law["completed"].as_u64().unwrap(),total:Some(8760),counters:Vec::new(),units_per_second:0.0,conflicts:0,steps:ToolRunStepRing::new()}),steps:Vec::new(),trace:Vec::new(),append_ops:Vec::new(),append_entities:Vec::new(),retract_to:None,payload:Some(vec![identity.generation as u8;65536]),payload_update:Some(ToolRunPayloadUpdate::Range{publication:1,total_bytes:65536,offset:0,chunk_bytes:65536,final_chunk:true})};let wire=tick.encode().unwrap();let source=Source{bytes:&wire,page:67};let mut started=0;let mut complete=false;
  for turn in 0..200000{let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(GRANT,&mut decoder,|cx|{if state.initialize("energySimulation",identity,ToolRunState::Running,cx)?{let result=state.receive(&source,identity,ToolRunState::Running,cx)?;if state.has_pending_tick(){started+=1;}return Ok(result)}Ok(false)}));complete=result.unwrap();let receipt=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));if identity.generation==1&&started==law["cancelAfterTurns"].as_u64().unwrap(){assert!(state.has_pending_tick());assert!(!complete);break}if complete{break}assert!(turn<199999);}
  if identity.generation==2{assert!(complete);assert_eq!(state.borrowed_view(identity,ToolRunState::Running).unwrap().progress.completed,8760);}
  state.request_close();assert!(state.borrowed_view(identity,ToolRunState::Aborted).is_none());let original=wire.as_ptr();for turn in 0..200000{if state.terminal_is_empty(){break}let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(GRANT,&mut decoder,|cx|cx.with_original_retirement(|cx|{let d=state.next_close_demands(4096)?;close_original_retained_step(cx,d,|grant|state.close_step(grant))?;Ok(())})));result.unwrap();let receipt=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));assert_eq!(wire.as_ptr(),original);assert!(turn<199999);}assert!(state.terminal_is_empty());
 }
 while !decoder.terminal_is_empty(){caller.turn(GRANT,|cx|cx.with_original_retirement(|cx|{let d=decoder.retirement_demands(4096)?;close_original_retained_step(cx,d,|grant|decoder.close_step(grant))?;Ok(())})).unwrap();}caller.retire(GRANT);
}

extern crate self as semio_framework_plugin_tool_run;
#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🎞️tick/📦️payload/🦀️.rs"]
pub mod original_tick_payload;
#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-run/📬️outcome/🦀️.rs"]
mod original_toy_emission;
use original_toy_emission::OriginalToyEmission;
use semio_framework_value::{ValueError,RetirementDemand};
#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-run/📬️native-ledger/🦀️.rs"]
mod original_readonly_preview;
#[test]
fn mounted_fixture_incremental_original_writer_publishes_canonical_nine_counter_preview(){
 let identity=ToolRunIdentity::new(ToolRunId{app_instance_id:71,run:12},[26;32]);let stock=RetainedCloneGrant{maximum_items:1<<30,maximum_copy_bytes:1<<29,maximum_capacity_bytes:1<<26,maximum_release_bytes:1<<26,maximum_depth:4096};let mut caller=OriginalJobFixtureCaller::admit(OperationId(904),Generation(10),stock,default_now_us,u64::MAX,1<<28);let mut decoder=OriginalJobDecodeOwner::empty();let mut producer=original_readonly_preview::OriginalReadonlyPreview::new(identity);let mut state=native::OriginalToolRunNativeState::empty();let mut outcome=JobOutcomeDescriptorSlot::empty();let mut accepted=false;let mut complete=false;
 for turn in 0..200000{let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(GRANT,&mut decoder,|cx|{if let Some(descriptor)=outcome.original(){if descriptor.is_acknowledged()||accepted{let d=outcome.retirement_demands();close_original_retained_step(cx,d,|grant|Ok(outcome.close_step(grant)))?;return Ok(())}match producer.borrow_outcome(descriptor)?{JobOutcomeView::PreviewReady{payload,..}=>{let source=native::OriginalPreviewPages(payload);if state.initialize("energySimulation",identity,ToolRunState::Running,cx)?&&state.receive(&source,identity,ToolRunState::Running,cx)?{let view=state.borrowed_view(identity,ToolRunState::Running).unwrap();assert_eq!(view.progress.completed,8760);assert_eq!(view.progress.counters.len(),9);assert_eq!(view.payload.unwrap().read_fixed::<4>(0).unwrap(),[17,23,41,67]);accepted=true;}},JobOutcomeView::Complete{..}=>{complete=true;let d=outcome.retirement_demands();close_original_retained_step(cx,d,|grant|Ok(outcome.close_step(grant)))?;},_=>panic!("original readonly fixture emits only preview and completion")}}else{if accepted{accepted=false;}let mut verdict=None;if let Some(loan)=drive_step(&mut producer,cx,"energy.native-incremental-preview",InteractiveStage::InteractiveStep,&mut verdict)?{outcome.retain(loan.into_descriptor()).unwrap();}}Ok(())}));result.unwrap();let p=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));if complete&&outcome.is_empty(){break}assert!(turn<199999);}
 assert!(complete);producer.begin_close();for turn in 0..200000{if producer.terminal_is_empty()&&state.terminal_is_empty(){break}let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(GRANT,&mut decoder,|cx|cx.with_original_retirement(|cx|{if !state.terminal_is_empty(){let d=state.next_close_demands(4096)?;close_original_retained_step(cx,d,|grant|state.close_step(grant))?;}else{producer.close_step(cx)?;}Ok(())})));result.unwrap();let p=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));assert!(turn<199999);}assert!(producer.terminal_is_empty()&&state.terminal_is_empty());while !decoder.terminal_is_empty(){caller.turn(GRANT,|cx|cx.with_original_retirement(|cx|{let d=decoder.retirement_demands(4096)?;close_original_retained_step(cx,d,|grant|decoder.close_step(grant))?;Ok(())})).unwrap();}caller.retire(GRANT);
}
