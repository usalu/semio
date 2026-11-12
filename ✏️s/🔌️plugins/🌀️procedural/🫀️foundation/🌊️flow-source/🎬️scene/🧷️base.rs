//! 🧷️ The original context-overlaid host stays immutable while one genuine projected clone becomes working scratch.
use super::retained_scratch_driver::{begun,funded_source_turn,funded_turn,local_progress,pay,replied,turn_work};
use super::retained_flow_source::ProceduralFlowSourceWork;
use super::retained_projected_clone::ProjectedFieldCloneWork;
use semio_framework_artifact_flow_flow::FlowHostSnapshot;
use semio_framework_job::{StepContext,OriginalSourceBinding,OriginalTurnFailure};
use semio_framework_value::{OriginalRetirementPoll,RetainedAllocationEffect,RetirementDemand,RetirementPollExit,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{OriginalEffectReply,OriginalEffectReplyComplete,RetainedCloneSource,RetainedCloneSourceAdmission,RetainedOwnedProjection,RetainedOwnedProjectionAdmission,RetainedCloneClose,retained_clone_grant_admits};
use semio_framework_value::retirement::RetireOwned;
use std::mem::{ManuallyDrop,size_of};
struct Fields{scratch:Option<FlowHostSnapshot>,returned:Option<(FlowHostSnapshot,())>,admission:Option<RetainedCloneSourceAdmission<FlowHostSnapshot,FlowHostSnapshot,()>>,base:Option<RetainedCloneSource<FlowHostSnapshot>>,clone:Option<ProjectedFieldCloneWork<FlowHostSnapshot>>,retain:RetainedOwnedProjectionAdmission<FlowHostSnapshot>,retained:Option<RetainedOwnedProjection<FlowHostSnapshot>>}
pub struct OriginalSceneBaseWork{fields:ManuallyDrop<Fields>,original:OriginalSourceBinding,source:Option<usize>,phase:u8,closing:bool,recover:bool,retirement:RetainedCloneClose,progress:RetainedCloneProgress}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"immutable procedural base changed its actual source, caller or custody")}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn event(cx:&mut StepContext<'_>,body:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome)->Result<bool,OriginalTurnFailure>{
 Ok(funded_turn(cx,body)?.is_some_and(|step|step.progress()!=Default::default()))
}
impl OriginalSceneBaseWork{
 pub fn new(cx:&StepContext<'_>)->Self{Self::new_original(OriginalSourceBinding::borrow(cx))}
 pub fn new_original(original:OriginalSourceBinding)->Self{Self{fields:ManuallyDrop::new(Fields{scratch:None,returned:None,admission:None,base:None,clone:None,retain:Default::default(),retained:None}),original,source:None,phase:0,closing:false,recover:false,retirement:Default::default(),progress:Default::default()}}
 pub fn ready(&self)->bool{!self.closing&&self.phase==9}
 pub fn base(&self)->Result<Option<&FlowHostSnapshot>,ValueError>{if !self.ready(){return Ok(None)}self.fields.base.as_ref().map(RetainedCloneSource::try_authority).transpose()}

 pub fn projection_demands(&self)->Result<RetirementDemand,ValueError>{if !self.ready()||self.fields.base.is_none(){return Err(invalid())}Ok(RetainedOwnedProjection::<FlowHostSnapshot>::constructor_demand())}
 pub fn retain_base_into<'reply>(&self,output:&mut RetainedOwnedProjectionAdmission<FlowHostSnapshot>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,ValueError>,ValueError>{
  if !self.ready(){return Err(invalid())}output.admit(self.fields.base.as_ref().ok_or_else(invalid)?.try_borrow()?,effect,reply)
 }
 fn validate<T:RetireOwned+Sync>(&self,source:&ProceduralFlowSourceWork<T>)->Result<(),ValueError>{if self.closing||self.source.is_some_and(|bound|bound!=source as*const _ as usize){return Err(invalid())}source.original_source_demands(0)?;Ok(())}
 pub fn next_demands<T:RetireOwned+Sync>(&self,source:&ProceduralFlowSourceWork<T>,copy:usize,work:u64)->Result<RetirementDemand,ValueError>{
  self.validate(source)?;
  match self.phase{
   0|8=>source.scratch_transfer_demands(),
   1=>Ok(fixed(size_of::<RetainedCloneSourceAdmission<FlowHostSnapshot,FlowHostSnapshot,()>>())),
   2=>self.fields.admission.as_ref().ok_or_else(invalid)?.next_demand(),
   3=>self.fields.retain.demand(self.fields.base.as_ref().ok_or_else(invalid)?.try_borrow()?),
   4=>{let mut d=self.fields.retain.take_demand()?;d.copy_bytes=d.copy_bytes.checked_add(size_of::<ProjectedFieldCloneWork<FlowHostSnapshot>>()).ok_or_else(invalid)?;Ok(d)},
   5|6=>self.fields.clone.as_ref().ok_or_else(invalid)?.next_demands(copy,work),
   7=>match self.fields.clone.as_ref(){Some(work)if !work.terminal_is_empty()=>work.next_close_demands(copy),_=>Ok(fixed(0))},
   9=>Ok(Default::default()),
   _=>Err(invalid()),
  }
 }
 pub fn step<T:RetireOwned+Sync>(&mut self,source:&mut ProceduralFlowSourceWork<T>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(source)?;if !self.original.matches(cx){return Err(invalid().into())}cx.original_source_parent()?.validate_borrowed_context(cx)?;
  if self.ready(){return Ok(true)}if cx.is_cancelled()||cx.should_yield(){return Ok(false)}
  match self.phase{
   0=>{if source.take_scratch_into(&mut self.fields.scratch,cx)?{self.source=Some(source as*const _ as usize);self.phase=1}},
   1=>{
    let d=self.next_demands(source,cx.retained_grant().maximum_copy_bytes,turn_work(cx))?;let fields=&mut*self.fields;
    if event(cx,|effect|{let result=match pay(effect,1){Ok(true)=>{fields.admission=Some(RetainedCloneSource::admission_owned(fields.scratch.take().unwrap(),()));Ok(RetainedCloneStep::Progress(local_progress(d)))},Ok(false)=>Ok(RetainedCloneStep::Progress(Default::default())),Err(error)=>Err(error)};RetainedWorkOutcome{result,work:effect.performed_work()}})?{self.phase=2}
   },
   2=>{
    let fields=&mut*self.fields;
    event(cx,|effect|{let admission=fields.admission.as_mut().unwrap();let turn=if admission.phase()=="ready"{admission.take_output(&mut fields.base,effect)}else{admission.advance(effect)};if admission.terminal_is_empty(){fields.admission=None}turn})?;
    if self.fields.base.is_some(){self.phase=3}
   },
   3=>{
    let fields=&mut*self.fields;let base=fields.base.as_ref().ok_or_else(invalid)?;let retain=&mut fields.retain;
    let d=retain.demand(base.try_borrow()?)?;let work=retain.work_demand(base.try_borrow()?)?;
    let step=funded_source_turn(cx,|effect|{
     if !retained_clone_grant_admits(effect.grant(),d)||effect.remaining_work_units().map_or(true,|remaining|remaining<work){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
     let result=match base.try_borrow().and_then(|source|replied(effect,|effect,reply|retain.admit(source,effect,reply))){Ok(Some(result))=>result,Ok(None)=>Err(invalid()),Err(error)=>Err(error)};
     RetainedWorkOutcome{result,work:effect.performed_work()}
    })?;
    if step.is_some_and(|step|step.progress()!=Default::default()){self.phase=4}
   },
   4=>{
    let d=self.next_demands(source,cx.retained_grant().maximum_copy_bytes,turn_work(cx))?;let original=self.original;let fields=&mut*self.fields;let(retain,retained,clone)=(&mut fields.retain,&mut fields.retained,&mut fields.clone);
    funded_source_turn(cx,|effect|{
     if !retained_clone_grant_admits(effect.grant(),d)||effect.remaining_work_units().map_or(true,|remaining|remaining<1){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
     let result=match replied(effect,|effect,reply|retain.take(retained,effect,reply)){Ok(Some(result))=>result.map(|step|{if let Some(projection)=retained.take(){*clone=Some(ProjectedFieldCloneWork::new_original(projection,original))}RetainedCloneStep::Progress(RetainedCloneProgress{copied_bytes:step.progress().copied_bytes+size_of::<ProjectedFieldCloneWork<FlowHostSnapshot>>(),..step.progress()})}),Ok(None)=>Err(invalid()),Err(error)=>Err(error)};
     RetainedWorkOutcome{result,work:effect.performed_work()}
    })?;
    if self.fields.clone.is_some(){self.phase=5}
   },
   5=>{if self.fields.clone.as_mut().ok_or_else(invalid)?.step(cx)?{self.phase=6}},
   6=>{let fields=&mut*self.fields;if fields.clone.as_mut().ok_or_else(invalid)?.take_into(&mut fields.scratch,cx)?{fields.clone.as_mut().unwrap().begin_close();self.phase=7}},
   7=>{
    if self.fields.clone.as_ref().ok_or_else(invalid)?.terminal_is_empty(){
     let fields=&mut*self.fields;if event(cx,|effect|{let result=match pay(effect,1){Ok(true)=>{fields.clone=None;Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))},Ok(false)=>Ok(RetainedCloneStep::Progress(Default::default())),Err(error)=>Err(error)};RetainedWorkOutcome{result,work:effect.performed_work()}})?{self.phase=8}
    }else{let work=self.fields.clone.as_mut().unwrap();let d=work.next_close_demands(cx.retained_grant().maximum_copy_bytes)?;cx.with_original_retirement(|cx|semio_framework_job::close_original_effect_step(cx,d,|effect|{let body=effect.grant().maximum_copy_bytes;work.close_step(body,effect)}))?;}
   },
   8=>{if source.restore_scratch_from(&mut self.fields.scratch,cx)?{self.phase=9}},
   _=>return Err(invalid().into()),
  }Ok(self.ready())
 }
 pub fn begin_close(&mut self){self.closing=true;if let Some(work)=self.fields.clone.as_mut(){work.begin_close()}}
 pub fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if let Some(work)=self.fields.clone.as_ref(){return if work.terminal_is_empty(){Ok(fixed(0))}else{work.next_close_demands(copy)}}
  if let Some(projection)=self.fields.retained.as_ref(){return projection.next_close_demand(copy)}
  if !self.fields.retain.terminal_is_empty(){return self.fields.retain.close_demand(copy)}
  if let Some(admission)=self.fields.admission.as_ref(){return if admission.phase()=="ready"{admission.next_demand()}else{admission.next_return_demand()}}
  if !self.retirement.is_empty(){return self.retirement.demands(copy)}
  if self.fields.returned.is_some(){return self.retirement.next_owner_demands::<(FlowHostSnapshot,())>(true,&None,copy)}
  if self.fields.scratch.is_some(){return self.retirement.next_owner_demands::<FlowHostSnapshot>(true,&None,copy)}
  if let Some(base)=self.fields.base.as_ref(){return base.next_close_demand(copy)}
  Ok(Default::default())
 }
 pub fn close_step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
  self.progress=Default::default();let result=self.close_original(body,effect);RetirementPollExit::settle(effect,result)
 }
 fn close_original(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,RetirementPollExit>{
  if !self.closing{return Err(invalid().into())}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
  if !retained_clone_grant_admits(effect.grant(),self.next_close_demands(body)?){return Ok(RetainedCloneStep::Progress(Default::default()))}
  let fields=&mut*self.fields;
  let step=if let Some(work)=fields.clone.as_mut(){
   if work.terminal_is_empty(){fields.clone=None;RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})}else{RetirementPollExit::resume(work.close_step(body,effect))?}
  }else if let Some(projection)=fields.retained.as_mut(){let step=RetirementPollExit::resume(projection.close_step(body,effect))?;if projection.terminal_is_empty(){fields.retained=None}step}
  else if !fields.retain.terminal_is_empty(){RetirementPollExit::resume(fields.retain.close_step(body,effect))?}
  else if let Some(admission)=fields.admission.as_mut(){
   let turn=if admission.phase()=="ready"{admission.take_output(&mut fields.base,effect)}else{admission.return_inputs(&mut fields.returned,effect)};if admission.terminal_is_empty(){fields.admission=None}turn.result.map_err(RetirementPollExit::from)?
  }else if !self.retirement.is_empty(){RetirementPollExit::resume(self.retirement.step_granted(body,effect))?}
  else if fields.returned.is_some(){begun(&mut self.retirement,&mut fields.returned,effect)?}
  else if fields.scratch.is_some(){begun(&mut self.retirement,&mut fields.scratch,effect)?}
  else{let base=fields.base.as_mut().ok_or_else(invalid)?;let step=RetirementPollExit::resume(base.close_step(body,effect))?;if base.terminal_is_empty(){fields.base=None}step};
  self.progress=step.progress();Ok(RetainedCloneStep::Progress(self.progress))
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.fields.clone.is_none()&&self.fields.retained.is_none()&&self.fields.retain.terminal_is_empty()&&self.fields.admission.is_none()&&self.fields.base.is_none()&&self.fields.scratch.is_none()&&self.fields.returned.is_none()&&self.retirement.is_empty()}
 pub fn step_progress(&self)->RetainedCloneProgress{self.progress}
}
impl Drop for OriginalSceneBaseWork{fn drop(&mut self){assert!(self.terminal_is_empty(),"immutable procedural base retains its actual source, admission or projected scratch");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.fields)}}}}
