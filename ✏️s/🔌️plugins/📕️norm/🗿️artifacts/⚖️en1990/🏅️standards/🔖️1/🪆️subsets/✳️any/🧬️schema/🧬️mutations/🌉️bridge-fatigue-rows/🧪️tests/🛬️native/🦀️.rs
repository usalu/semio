//! 🛬️ The real bridge SLS leaf receives its committed payload under original decoding custody.
use semio_framework_value_native_receiving_original::NativeReceivingOriginal;
use crate::insert_bridge_sls::InsertBridgeSls;
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OperationId,Generation};
use semio_framework_value::{DslValue,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome};
use semio_framework_value::retained_clone::{RetainedCloneSource,RetainedCloneSourceAdmission,RetainedCloneClose};
use semio_framework_value_native_owned::{OriginalReceivingPrimary};

#[global_allocator]
static ALLOCATOR:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;

fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:4096}}
fn stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:2_000_000,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<28,maximum_release_bytes:1<<28,maximum_depth:4096}}
struct Fixture{caller:OriginalJobFixtureCaller,decoder:OriginalJobDecodeOwner}
impl Fixture{
 fn new()->Self{Self{caller:OriginalJobFixtureCaller::admit(OperationId(199001),Generation(1),stock(),||Some(1),u64::MAX,2_000_000),decoder:OriginalJobDecodeOwner::empty()}}
 fn paid(&mut self,run:impl FnOnce()->RetainedWorkOutcome)->RetainedCloneStep{
  self.caller.turn_with_decoder(grant(),&mut self.decoder,|cx|{
   let(turn,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(run);
   let step=turn.result?;let p=step.progress();
   assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));
   cx.consume_retained(p)?;cx.consume_fuel(turn.work);Ok(step)
  }).unwrap()
 }
 fn capture(&mut self,value:DslValue)->RetainedCloneSource<DslValue>{
  let mut admission=RetainedCloneSourceAdmission::new(value,(),|v|v);
  for _ in 0..256{if admission.phase()=="ready"{break;}self.paid(||admission.advance(grant()));}
  assert_eq!(admission.phase(),"ready");let mut source=None;
  self.paid(||admission.take_output(&mut source,grant()));assert!(admission.terminal_is_empty());source.unwrap()
 }
 fn advance(&mut self,owner:&mut NativeReceivingOriginal<DslValue,InsertBridgeSls>,g:RetainedCloneGrant)->RetainedCloneStep{
  self.caller.turn_with_decoder(grant(),&mut self.decoder,|cx|{
   cx.with_original_work_loan(4096,cx.deadline_us(),|cx|{
    let ((result,work),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{
     let mut work=0;let result=cx.with_native_receiving::<_,OriginalReceivingPrimary>(|control|{let turn=owner.advance(control,4096,g);work=turn.work;turn.result});(result,work)
    });
    let step=result?;let p=step.progress();
    assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));
    cx.consume_receiving_retained(g,p)?;cx.consume_fuel(work);
    if g.maximum_items==0{assert_eq!(p,RetainedCloneProgress::default());assert_eq!(work,0);}
    Ok(step)
   })
  }).unwrap()
 }
 fn close(&mut self,owner:&mut NativeReceivingOriginal<DslValue,InsertBridgeSls>){
  owner.begin_close().unwrap();
  for _ in 0..100_000{if owner.terminal_is_empty(){return;}self.paid(||owner.close_step(grant()));}
  panic!("native leaf original ownership did not retire");
 }
 fn close_value(&mut self,value:InsertBridgeSls){
  let mut value=Some(value);let mut close=RetainedCloneClose::default();
  for _ in 0..100_000{if value.is_none()&&close.is_empty(){return;}self.paid(||if close.is_empty(){let t=close.begin_granted(&mut value,grant());RetainedWorkOutcome{result:t.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:t.work}}else{close.step_granted(grant())});}
  panic!("native leaf output did not retire");
 }
 fn finish(mut self){
  for _ in 0..100_000{if self.decoder.terminal_is_empty(){self.caller.retire(stock());return;}
   self.caller.turn(stock(),|cx|cx.with_original_retirement(|cx|{let d=self.decoder.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;self.decoder.close_on_source(cx)?;Ok(())})).unwrap();
  }panic!("original decoder did not retire");
 }
}
fn payload()->(&'static str,DslValue){
 let text=semio_framework_test_fixtures::repo_text!("✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🌉️insert-bridge-sls/✅apply/🔣️.mutation.json");
 let parsed=semio_framework_pack_json::parse(text,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
 let mut value=semio_framework_pack_json::to_dsl_value(&parsed);
 assert_eq!(value.get("mutation").and_then(DslValue::as_str),Some("insertBridgeSls"));
 let DslValue::Object(fields)=&mut value else{panic!("actual mutation object");};fields.retain(|(key,_)|key!="mutation");
 (text,value)
}
#[test]
fn original_bridge_sls_receiving_matches_serde_and_closes_selected_prefixes(){
 let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{let bytes=std::hint::black_box(vec![0_u8;73]);std::hint::black_box(&bytes);drop(bytes);});
 assert_eq!((heap.requested_bytes,heap.released_bytes),(73,73));
 for prefix in [0,1,3,11,64,128,usize::MAX]{
  let(text,input)=payload();let mut independent:serde_json::Value=serde_json::from_str(text).unwrap();
  assert_eq!(independent.as_object_mut().unwrap().remove("mutation"),Some(serde_json::Value::String("insertBridgeSls".into())));
  let expected:InsertBridgeSls=serde_json::from_value(independent).unwrap();
  let mut fixture=Fixture::new();let source=fixture.capture(input);
  let mut owner=NativeReceivingOriginal::<DslValue,InsertBridgeSls>::new(source);
  for step in 0..100_000{
   if step==prefix||owner.output_ready(){break;}
   let before=owner.demands(4096).unwrap();
   fixture.advance(&mut owner,RetainedCloneGrant{maximum_items:0,..grant()});
   assert_eq!(owner.demands(4096).unwrap(),before);
   fixture.advance(&mut owner,grant());
  }
  if prefix==usize::MAX{
   assert!(owner.output_ready());
   let mut output=None;
   fixture.paid(||{let t=owner.take(grant());RetainedWorkOutcome{result:t.result.map(|v|{let(value,p)=v.expect("ready native output");output=Some(value);RetainedCloneStep::Complete(p)}),work:t.work}});
   let actual=output.unwrap();assert_eq!(actual,expected);fixture.close_value(actual);
  }
  fixture.close(&mut owner);fixture.finish();
 }
}
