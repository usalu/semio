//! ➕️ The actual Add command supplies only its selected descriptor fields through the shared funded text owner.
use super::super::add_widget::AddWidget;
use flow::FlowTextCopyWork;
use semio_framework_artifact_flow_flow::WidgetDescriptor;
use semio_framework_job::{StepContext,OriginalTurnFailure,OriginalSourceBinding,close_original_retained_step,InteractiveJobCloseStep as Close};
use semio_framework_value::{RetireOwned,RetirementDemand,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneClose,retained_clone_grant_admits}};
use std::mem::{ManuallyDrop,size_of};
#[derive(Clone,Copy,PartialEq,Eq)]
struct Binding{source:OriginalSourceBinding,payload:usize}
#[derive(RetireOwned)]
struct Fields{text:Option<String>,output:Option<WidgetDescriptor>}
pub struct FlowAddDescriptorWork{fields:ManuallyDrop<Option<Fields>>,copy:Option<FlowTextCopyWork>,retirement:RetainedCloneClose,binding:Option<Binding>,kind:u8,phase:u8,closing:bool}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original Add descriptor changed its caller or donated command")}
fn fixed(bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()}}
fn p(bytes:usize)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}}
impl FlowAddDescriptorWork{
 pub fn new()->Self{Self{fields:ManuallyDrop::new(Some(Fields{text:None,output:None})),copy:None,retirement:Default::default(),binding:None,kind:0,phase:0,closing:false}}
 fn kind(payload:&AddWidget)->Result<u8,ValueError>{Ok(match payload.kind.as_str(){"neuron"=>0,"inputSlider"=>1,"inputNote"=>2,"inputImage"=>3,"outputPreview"=>4,"outputAction"=>5,"outputExport"=>6,"variable"=>7,_=>return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original Add widget kind is unsupported"))})}
 fn text<'a>(&self,payload:&'a AddWidget)->&'a str{match self.kind{0=>payload.neuron_kind.as_deref().unwrap_or("math.add"),1=>payload.label.as_deref().unwrap_or(""),5=>payload.action.as_deref().unwrap_or(""),6=>payload.format.as_deref().unwrap_or(""),_=>""}}
 fn paid(cx:&mut StepContext<'_>,bytes:usize)->Result<bool,OriginalTurnFailure>{if cx.should_yield()||cx.is_cancelled()||!cx.retained_work_can_enter(fixed(bytes)){return Ok(false)}cx.consume_fuel(1);cx.consume_retained(p(bytes))?;Ok(true)}
 pub fn step(&mut self,payload:&AddWidget,cx:&mut StepContext<'_>)->Result<Option<WidgetDescriptor>,OriginalTurnFailure>{for _ in 0..512{let before=(cx.retained_progress(),cx.fuel_remaining());if let Some(output)=self.once(payload,cx)?{return Ok(Some(output))}if cx.should_yield()||cx.is_cancelled()||before==(cx.retained_progress(),cx.fuel_remaining()){break}}Ok(None)}
 fn once(&mut self,payload:&AddWidget,cx:&mut StepContext<'_>)->Result<Option<WidgetDescriptor>,OriginalTurnFailure>{
  let binding=Binding{source:OriginalSourceBinding::borrow(cx),payload:payload as*const _ as usize};if self.closing||self.binding.is_some_and(|original|original!=binding){return Err(invalid().into())}if cx.should_yield()||cx.is_cancelled(){return Ok(None)}
  match self.phase{
   0=>{if !cx.retained_work_can_enter(fixed(size_of::<Option<Binding>>()+size_of::<FlowTextCopyWork>())){return Ok(None)}let kind=Self::kind(payload)?;if Self::paid(cx,size_of::<Option<Binding>>()+size_of::<FlowTextCopyWork>())?{self.kind=kind;self.binding=Some(binding);if matches!(kind,0|1|5|6){self.copy=Some(FlowTextCopyWork::new());self.phase=1}else{self.phase=3}}},
   1=>{let text=self.text(payload);let copy=self.copy.as_mut().ok_or_else(invalid)?;if !copy.ready(){copy.advance(text,cx)?;return Ok(None)}if copy.take_into(&mut self.fields.as_mut().ok_or_else(invalid)?.text,cx)?{copy.begin_close();self.phase=2}},
   2=>{let copy=self.copy.as_mut().ok_or_else(invalid)?;if copy.terminal_is_empty(){if Self::paid(cx,size_of::<FlowTextCopyWork>())?{self.copy=None;self.phase=3}}else{let d=copy.close_demands(cx.retained_grant().maximum_copy_bytes)?;close_original_retained_step(cx,d,|g|copy.close_step(g))?;}},
   3=>if Self::paid(cx,2*size_of::<WidgetDescriptor>())?{let fields=self.fields.as_mut().ok_or_else(invalid)?;fields.output=Some(match self.kind{0=>WidgetDescriptor::Neuron{neuron_kind:fields.text.take().ok_or_else(invalid)?,id:None},1=>WidgetDescriptor::InputSlider{label:fields.text.take().ok_or_else(invalid)?,id:None,value:None,min:None,max:None,step:None},2=>WidgetDescriptor::InputNote{id:None,text:None},3=>WidgetDescriptor::InputImage{id:None},4=>WidgetDescriptor::OutputPreview{id:None},5=>WidgetDescriptor::OutputAction{id:None,action:fields.text.take().ok_or_else(invalid)?},6=>WidgetDescriptor::OutputExport{id:None,format:fields.text.take().ok_or_else(invalid)?},7=>WidgetDescriptor::Variable{id:None,name:None,schema:None},_=>return Err(invalid().into())});self.phase=4},
   4=>if Self::paid(cx,2*size_of::<Option<WidgetDescriptor>>())?{self.phase=5;return Ok(self.fields.as_mut().ok_or_else(invalid)?.output.take())},
   _=>return Err(invalid().into())
  }Ok(None)
 }
 pub fn restore_output(&mut self,output:WidgetDescriptor){assert_eq!(self.phase,5);self.fields.as_mut().expect("original Add descriptor").output=Some(output);self.phase=4}
 pub fn begin_close(&mut self){self.closing=true;if let Some(copy)=self.copy.as_mut(){copy.begin_close()}}
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if let Some(copy)=self.copy.as_ref(){return if copy.terminal_is_empty(){Ok(fixed(size_of::<FlowTextCopyWork>()))}else{copy.close_demands(body)}}if !self.retirement.is_empty(){return self.retirement.demands(body)}if self.fields.is_some(){return self.retirement.next_owner_demands::<Fields>(true,&None,body)}Ok(fixed(size_of::<Option<Binding>>()))}
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<Close,OriginalTurnFailure>{if !self.closing||self.binding.is_some_and(|binding|!binding.source.matches(cx)){return Err(invalid().into())}
  if let Some(copy)=self.copy.as_mut(){if !copy.terminal_is_empty(){let d=copy.close_demands(cx.retained_grant().maximum_copy_bytes)?;let step=close_original_retained_step(cx,d,|g|copy.close_step(g))?;return Ok(Close::Pending{progress:step.progress()})}let d=fixed(size_of::<FlowTextCopyWork>());if cx.fuel_remaining()==0||cx.should_yield()||!cx.retained_work_can_enter(d){return Ok(Close::Blocked)}self.copy=None;cx.consume_fuel(1);cx.consume_retained(p(d.copy_bytes))?;return Ok(Close::Pending{progress:p(d.copy_bytes)})}
  let d=self.close_demands(cx.retained_grant().maximum_copy_bytes)?;let step=close_original_retained_step(cx,d,|g|{if !self.retirement.is_empty(){return self.retirement.step_granted(g)}if self.fields.is_some(){let turn=self.retirement.begin_granted(&mut self.fields,g);return RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work}}if self.binding.is_none(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}self.binding=None;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(p(d.copy_bytes))),work:1}})?;Ok(if self.terminal_is_empty(){Close::Complete{progress:step.progress()}}else{Close::Pending{progress:step.progress()}})
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.copy.is_none()&&self.fields.is_none()&&self.retirement.is_empty()&&self.binding.is_none()}
}
impl Drop for FlowAddDescriptorWork{fn drop(&mut self){assert!(self.terminal_is_empty(),"original Add descriptor must close its actual selected text and result");unsafe{ManuallyDrop::drop(&mut self.fields)}}}

#[cfg(test)]
#[path="🧪️descriptor.rs"]
mod tests;
