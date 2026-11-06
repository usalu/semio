import{readFileSync,writeFileSync,renameSync}from"node:fs";import{relative,resolve}from"node:path";
const dir="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context",file=dir+"/🌉️call/📤️turn-output/🦀️.rs",old=readFileSync(file,"utf8");
const next=`//! 🧹️ The actual typed turn, encoder and borrowed bytes remain until same-Source paid closure.
use semio_framework::kernel::TurnResult;
use semio_framework_job::{OriginalTurnFailure,OriginalSourceBinding,OriginalCallerReturnPlace,StepContext,close_original_retained_step};
use semio_framework_pack_json_source::SourceJsonEncoder;
use semio_framework_kernel_json::KernelTurnJsonSource;
use semio_framework_value::{ControlledRetirement,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,RetireOwned,ValueError,ValueRefusalKind};
use std::mem::ManuallyDrop;
pub struct OriginalTurnOutput{
 input:Option<semio_framework_actor::RetainedTurnInput>,binding:Option<OriginalSourceBinding>,capture:OriginalCallerReturnPlace,
 value:ManuallyDrop<Option<TurnResult>>,retirement:ManuallyDrop<Option<ControlledRetirement<TurnResult>>>,
 encoder:SourceJsonEncoder,encoded:ManuallyDrop<Option<Vec<u8>>>,encoded_retirement:ManuallyDrop<Option<ControlledRetirement<Vec<u8>>>>,
}
impl OriginalTurnOutput{
 pub const fn empty()->Self{Self{input:None,binding:None,capture:OriginalCallerReturnPlace::empty(),value:ManuallyDrop::new(None),retirement:ManuallyDrop::new(None),encoder:SourceJsonEncoder::new(),encoded:ManuallyDrop::new(None),encoded_retirement:ManuallyDrop::new(None)}}
 pub fn get(&self)->Option<&TurnResult>{self.value.as_ref()}
 pub fn encoded(&self)->Option<&[u8]>{self.encoder.output_is_ready(&self.encoded).then(||self.encoded.as_deref()).flatten()}
 pub fn is_empty(&self)->bool{self.input.is_none()&&self.binding.is_none()&&self.capture.terminal_is_empty()&&self.value.is_none()&&self.retirement.is_none()&&self.encoder.terminal_is_empty()&&self.encoded.is_none()&&self.encoded_retirement.is_none()}
 fn capture_bytes()->usize{size_of::<Option<TurnResult>>()+size_of::<Option<semio_framework_actor::RetainedTurnInput>>()+size_of::<Option<OriginalSourceBinding>>()}
 fn destination(&self)->usize{&*self.value as *const Option<TurnResult> as usize}
 fn validate(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{
  if self.binding.is_some_and(|binding|!binding.matches(cx))||self.input.is_some_and(|input|cx.original_source_epoch().is_none_or(|epoch|epoch<input.epoch)){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"resident turn output requires its exact original Source and a current issued epoch").into())}Ok(())
 }
 /// 📥️ Producer work starts only after its actual preborn result and metadata have held return credit.
 pub fn capture_step<E:Into<OriginalTurnFailure>>(&mut self,cx:&mut StepContext<'_>,produce:impl FnOnce(&mut StepContext<'_>)->Result<TurnResult,E>)->Result<bool,OriginalTurnFailure>{
  self.validate(cx)?;
  if self.value.is_some(){return self.capture.settle_return(cx)}
  if self.retirement.is_some()||self.encoded.is_some()||self.encoded_retirement.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"prior turn output is retiring").into())}
  let destination=self.destination();if !self.capture.admit_sized(cx,destination,Self::capture_bytes())?{return Ok(false)}
  let input=semio_framework_actor::RetainedTurnInput::borrow_original(cx)?;let binding=OriginalSourceBinding::borrow(cx);let value=produce(cx).map_err(Into::into)?;
  *self.value=Some(value);self.input=Some(input);self.binding=Some(binding);
  self.capture.record_sized_return(destination,Self::capture_bytes()).map_err(OriginalTurnFailure::from)?;Ok(false)
 }
 /// 🔤️ Each synchronous encoder slice borrows the real immutable UI arena and original result.
 pub fn encode_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(cx)?;if !self.capture.terminal_is_empty(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"turn capture must settle before encoding").into())}
  let value=self.value.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original turn result is absent"))?;
  let Some(source)=KernelTurnJsonSource::try_borrow(value)?else{return Ok(false)};self.encoder.advance(&source,&mut self.encoded,cx)
 }
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(cx)?;
  if !self.capture.terminal_is_empty(){self.capture.settle_return(cx)?;return Ok(false)}
  if !self.encoder.terminal_is_empty(){self.encoder.close_step(cx)?;return Ok(false)}
  if self.encoded.is_some(){
   let demand=RetirementDemand{copy_bytes:size_of::<Option<Vec<u8>>>()+size_of::<Option<ControlledRetirement<Vec<u8>>>>(),depth:1,..Default::default()};
   close_original_retained_step(cx,demand,|_|{let value=self.encoded.take().expect("retained encoded turn");*self.encoded_retirement=Some(ControlledRetirement::retain(value));RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})),work:1}})?;return Ok(false)
  }
  if let Some(owner)=self.encoded_retirement.as_mut(){
   if !owner.terminal_is_empty(){let demand=owner.next_demand(cx.retained_grant().maximum_copy_bytes)?;close_original_retained_step(cx,demand,|grant|{let result=owner.step(grant);RetainedWorkOutcome{result,work:owner.last_turn_work()}})?;return Ok(false)}
   let demand=RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<Vec<u8>>>>(),depth:1,..Default::default()};close_original_retained_step(cx,demand,|_|{self.encoded_retirement.take();RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})),work:1}})?;return Ok(false)
  }
  if self.value.is_some(){
   if !TurnResult::controlled_retirement_supported(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"turn result has an unsupported original descendant").into())}
   let demand=RetirementDemand{copy_bytes:size_of::<Option<TurnResult>>()+size_of::<Option<ControlledRetirement<TurnResult>>>(),depth:1,..Default::default()};
   close_original_retained_step(cx,demand,|_|{let value=self.value.take().expect("retained turn output");*self.retirement=Some(ControlledRetirement::retain(value));RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})),work:1}})?;return Ok(false)
  }
  if let Some(owner)=self.retirement.as_mut(){if !owner.terminal_is_empty(){let demand=owner.next_demand(cx.retained_grant().maximum_copy_bytes)?;close_original_retained_step(cx,demand,|grant|{let result=owner.step(grant);RetainedWorkOutcome{result,work:owner.last_turn_work()}})?;return Ok(false)}}
  if self.input.is_none()&&self.binding.is_none(){return Ok(self.is_empty())}
  let demand=RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<TurnResult>>>()+size_of::<Option<semio_framework_actor::RetainedTurnInput>>()+size_of::<Option<OriginalSourceBinding>>()+size_of::<SourceJsonEncoder>(),depth:1,..Default::default()};
  close_original_retained_step(cx,demand,|_|{self.retirement.take();self.input=None;self.binding=None;self.encoder=SourceJsonEncoder::new();RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})),work:1}})?;Ok(self.is_empty())
 }
}
impl Drop for OriginalTurnOutput{fn drop(&mut self){assert!(std::thread::panicking()||self.is_empty(),"resident turn output reached Drop before original acknowledgement")}}

#[cfg(test)]mod tests{
 use super::*;use semio_framework_job::{OriginalJobFixtureCaller,OperationId,Generation};
 fn clock()->Option<u64>{Some(100)}
 #[test]
 fn captured_typed_turn_and_encoded_capsule_survive_denial_foreign_source_and_cancelled_paid_close(){
  let _arena=semio_ui_contract_values::testkit::UiArenaFixture::new();let law:serde_json::Value=serde_json::from_str(include_str!("🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let grant=serde_json::from_value(law["grant"].clone()).unwrap();
  for cut in[0,3,19,100000]{
   let mut caller=OriginalJobFixtureCaller::admit(OperationId(71),Generation(3),grant,clock,1000000,1000000);let mut output=OriginalTurnOutput::empty();let mut incoming=Some(serde_json::from_value(law["result"].clone()).unwrap());let mut epoch=0;let mut produced=0;
   for _ in 0..10000{epoch+=1;let ready=caller.turn(grant,|cx|{cx.bind_original_source_epoch(epoch)?;output.capture_step(cx,|_|{produced+=1;Ok::<_,OriginalTurnFailure>(incoming.take().unwrap())})}).unwrap();if ready{break}}
   assert!(incoming.is_none());assert_eq!(produced,1);let pointer=output.get().unwrap().effects.as_ptr();let denied=semio_framework_value::RetainedCloneGrant{maximum_copy_bytes:0,..grant};epoch+=1;
   let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(denied,|cx|{cx.bind_original_source_epoch(epoch)?;output.close_step(cx)}));assert!(!result.unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(caller.receipt(),Default::default());assert_eq!(output.get().unwrap().effects.as_ptr(),pointer);
   let mut foreign=OriginalJobFixtureCaller::admit(OperationId(71),Generation(3),grant,clock,1000000,1000000);let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||foreign.turn(grant,|cx|{cx.bind_original_source_epoch(epoch+1)?;output.close_step(cx)}));assert!(result.is_err());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));foreign.retire(grant);
   let expected=serde_json::to_vec(&Ok::<_,()>(output.get().unwrap())).unwrap();for _ in 0..cut{epoch+=1;let(ready,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(grant,|cx|{cx.bind_original_source_epoch(epoch)?;output.encode_step(cx)}).unwrap());let receipt=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));if ready{break}}
   if cut==100000{assert_eq!(output.encoded().unwrap(),expected)}caller.cancel().cancel_now();let mut released=0;
   for _ in 0..100000{epoch+=1;let(done,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(grant,|cx|{cx.bind_original_source_epoch(epoch)?;cx.with_original_retirement(|cx|output.close_step(cx))}).unwrap());let receipt=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));released+=heap.released_bytes;if done{break}}
   assert!(output.is_empty());assert!(released>0);caller.retire(grant);
  }
 }
}
`;
if(readFileSync(file,"utf8")!==old)throw Error("concurrent output edit");let temp=import.meta.dir+"/🗑️generated/turn-output.tmp";writeFileSync(temp,next);renameSync(temp,file);
const manifest=dir+"/📦️packages/🦀️rust/Cargo.toml",prior=readFileSync(manifest,"utf8"),pkg=manifest.slice(0,manifest.lastIndexOf("/"));let changed=prior;
for(const[name,path]of [["semio-framework-kernel-json","🧰️framework/🔨️modules/🎠️kernel/📤️return/🔤️json"],["semio-framework-pack-json-source","🧰️framework/🔨️modules/🎒️pack/🔤️json/🎟️source"]]){const target=relative(resolve(pkg),resolve(path+"/📦️packages/🦀️rust")).replaceAll("\\","/");changed=changed.replace("[dependencies]","[dependencies]\n"+name+' = { path = "'+target+'" }');}
const testpath=relative(resolve(pkg),resolve("🧰️framework/🔨️modules/🖱️ui/🧬️contract/🌱️values/📦️packages/🦀️rust")).replaceAll("\\","/");changed=changed.replace("[dev-dependencies]",'[dev-dependencies]\nsemio-ui-contract-values = { path = "'+testpath+'", features = ["testkit"] }');if(readFileSync(manifest,"utf8")!==prior)throw Error("concurrent manifest edit");temp=import.meta.dir+"/🗑️generated/turn-output-cargo.tmp";writeFileSync(temp,changed);renameSync(temp,manifest);
console.log("Authored prepaid typed turn capture, guarded encoder capsule and paid complete retirement");
