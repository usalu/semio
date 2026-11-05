//! 🧷️ The original context-overlaid host stays immutable while one genuine projected clone becomes working scratch.
use super::retained_flow_source::ProceduralFlowSourceWork;
use super::retained_projected_clone::ProjectedFieldCloneWork;
use semio_framework_artifact_flow_flow::FlowHostSnapshot;
use semio_framework_job::{StepContext,OriginalSourceBinding,OriginalTurnFailure,OriginalConstructionDemand,admit_original_construction,close_original_retained_step};
use semio_framework_value::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneSource,RetainedCloneSourceAdmission,RetainedOwnedProjection,RetainedCloneClose,retained_clone_grant_admits};
use semio_framework_value::retirement::RetireOwned;
use std::mem::{ManuallyDrop,size_of};
struct Fields{scratch:Option<FlowHostSnapshot>,returned:Option<(FlowHostSnapshot,())>,admission:Option<RetainedCloneSourceAdmission<FlowHostSnapshot,FlowHostSnapshot,()>>,base:Option<RetainedCloneSource<FlowHostSnapshot>>,clone:Option<ProjectedFieldCloneWork<FlowHostSnapshot>>}
pub(crate) struct OriginalSceneBaseWork{fields:ManuallyDrop<Fields>,original:OriginalSourceBinding,source:Option<usize>,phase:u8,closing:bool,retirement:RetainedCloneClose,progress:RetainedCloneProgress,work:u64}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"immutable procedural base changed its actual source, caller or custody")}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn event(cx:&mut StepContext<'_>,d:RetirementDemand,body:impl FnOnce(RetainedCloneGrant)->RetainedWorkOutcome)->Result<bool,OriginalTurnFailure>{
 let mut accepted=None;
 admit_original_construction(cx,&mut accepted,OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth},|g|{let turn=body(g);RetainedWorkOutcome{result:turn.result.map(|step|((),step.progress())),work:turn.work}})
}
impl OriginalSceneBaseWork{
 pub(crate) fn new(cx:&StepContext<'_>)->Self{Self::new_original(OriginalSourceBinding::borrow(cx))}
 pub(crate) fn new_original(original:OriginalSourceBinding)->Self{Self{fields:ManuallyDrop::new(Fields{scratch:None,returned:None,admission:None,base:None,clone:None}),original,source:None,phase:0,closing:false,retirement:Default::default(),progress:Default::default(),work:0}}
 pub(crate) fn ready(&self)->bool{!self.closing&&self.phase==8}
 pub(crate) fn base(&self)->Result<Option<&FlowHostSnapshot>,ValueError>{if !self.ready(){return Ok(None)}self.fields.base.as_ref().map(RetainedCloneSource::try_authority).transpose()}

 pub(crate) fn projection_demands(&self)->Result<RetirementDemand,ValueError>{if !self.ready()||self.fields.base.is_none(){return Err(invalid())}Ok(RetainedOwnedProjection::<FlowHostSnapshot>::constructor_demand())}
 pub(crate) fn retain_base_original(&self,grant:RetainedCloneGrant)->RetainedWorkOutcome<(RetainedOwnedProjection<FlowHostSnapshot>,RetainedCloneProgress)>{
  if !self.ready(){return RetainedWorkOutcome{result:Err(invalid()),work:0}}
  match self.fields.base.as_ref(){Some(base)=>base.retain_original(grant),None=>RetainedWorkOutcome{result:Err(invalid()),work:0}}
 }
 fn validate<T:RetireOwned+Sync>(&self,source:&ProceduralFlowSourceWork<T>)->Result<(),ValueError>{if self.closing||self.source.is_some_and(|bound|bound!=source as*const _ as usize){return Err(invalid())}source.original_source_demands()?;Ok(())}
 pub(crate) fn next_demands<T:RetireOwned+Sync>(&self,source:&ProceduralFlowSourceWork<T>,copy:usize)->Result<RetirementDemand,ValueError>{
  self.validate(source)?;
  match self.phase{
   0|7=>source.scratch_transfer_demands(),
   1=>Ok(fixed(size_of::<RetainedCloneSourceAdmission<FlowHostSnapshot,FlowHostSnapshot,()>>())),
   2=>self.fields.admission.as_ref().ok_or_else(invalid)?.next_demand(),
   3=>{let mut d=RetainedOwnedProjection::<FlowHostSnapshot>::constructor_demand();d.copy_bytes=d.copy_bytes.checked_add(size_of::<ProjectedFieldCloneWork<FlowHostSnapshot>>()).ok_or_else(invalid)?;Ok(d)},
   4|5=>self.fields.clone.as_ref().ok_or_else(invalid)?.next_demands(copy),
   6=>match self.fields.clone.as_ref(){Some(work)if !work.terminal_is_empty()=>work.next_close_demands(copy),_=>Ok(fixed(0))},
   8=>Ok(Default::default()),
   _=>Err(invalid()),
  }
 }
 pub(crate) fn step<T:RetireOwned+Sync>(&mut self,source:&mut ProceduralFlowSourceWork<T>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(source)?;if !self.original.matches(cx){return Err(invalid().into())}cx.original_source_parent()?.validate_borrowed_context(cx)?;
  if self.ready(){return Ok(true)}if cx.is_cancelled()||cx.should_yield(){return Ok(false)}
  match self.phase{
   0=>{if source.take_scratch_into(&mut self.fields.scratch,cx)?{self.source=Some(source as*const _ as usize);self.phase=1}},
   1=>{
    let d=self.next_demands(source,cx.retained_grant().maximum_copy_bytes)?;let fields=&mut*self.fields;
    if event(cx,d,|_|{fields.admission=Some(RetainedCloneSource::admission_owned(fields.scratch.take().unwrap(),()));RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),work:1}})?{self.phase=2}
   },
   2=>{
    let d=self.next_demands(source,cx.retained_grant().maximum_copy_bytes)?;let fields=&mut*self.fields;
    event(cx,d,|g|{let admission=fields.admission.as_mut().unwrap();let turn=if admission.phase()=="ready"{admission.take_output(&mut fields.base,g)}else{admission.advance(g)};if admission.terminal_is_empty(){fields.admission=None}turn})?;
    if self.fields.base.is_some(){self.phase=3}
   },
   3=>{
    let d=self.next_demands(source,cx.retained_grant().maximum_copy_bytes)?;let original=self.original;let fields=&mut*self.fields;let(base,clone)=(&fields.base,&mut fields.clone);
    if admit_original_construction(cx,clone,OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth},|g|{
     let turn=base.as_ref().unwrap().retain_original(g);
     RetainedWorkOutcome{result:turn.result.map(|(projection,mut p)|{p.copied_bytes+=size_of::<ProjectedFieldCloneWork<FlowHostSnapshot>>();(ProjectedFieldCloneWork::new_original(projection,original),p)}),work:turn.work}
    })?{self.phase=4}
   },
   4=>{if self.fields.clone.as_mut().ok_or_else(invalid)?.step(cx)?{self.phase=5}},
   5=>{let fields=&mut*self.fields;if fields.clone.as_mut().ok_or_else(invalid)?.take_into(&mut fields.scratch,cx)?{fields.clone.as_mut().unwrap().begin_close();self.phase=6}},
   6=>{
    if self.fields.clone.as_ref().ok_or_else(invalid)?.terminal_is_empty(){
     let fields=&mut*self.fields;if event(cx,fixed(0),|_|{fields.clone=None;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}})?{self.phase=7}
    }else{let work=self.fields.clone.as_mut().unwrap();let d=work.next_close_demands(cx.retained_grant().maximum_copy_bytes)?;cx.with_original_retirement(|cx|{close_original_retained_step(cx,d,|g|work.close_step(g))?;Ok(())})?;}
   },
   7=>{if source.restore_scratch_from(&mut self.fields.scratch,cx)?{self.phase=8}},
   _=>return Err(invalid().into()),
  }Ok(self.ready())
 }
 pub(crate) fn begin_close(&mut self){self.closing=true;if let Some(work)=self.fields.clone.as_mut(){work.begin_close()}}
 pub(crate) fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if let Some(work)=self.fields.clone.as_ref(){return if work.terminal_is_empty(){Ok(fixed(0))}else{work.next_close_demands(copy)}}
  if let Some(admission)=self.fields.admission.as_ref(){return if admission.phase()=="ready"{admission.next_demand()}else{admission.next_return_demand()}}
  if !self.retirement.is_empty(){return self.retirement.demands(copy)}
  if self.fields.returned.is_some(){return self.retirement.next_owner_demands::<(FlowHostSnapshot,())>(true,&None,copy)}
  if self.fields.scratch.is_some(){return self.retirement.next_owner_demands::<FlowHostSnapshot>(true,&None,copy)}
  if let Some(base)=self.fields.base.as_ref(){return base.next_close_demand(copy)}
  Ok(Default::default())
 }
 pub(crate) fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  self.progress=Default::default();self.work=0;let result=self.close_original(grant).map_err(|error|{let p=error.retained_progress();if p!=Default::default(){self.progress=p}error.with_retained_progress(self.progress)});RetainedWorkOutcome{result,work:self.work}
 }
 fn close_original(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if !self.closing{return Err(invalid())}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}if !retained_clone_grant_admits(grant,self.next_close_demands(grant.maximum_copy_bytes)?){return Ok(RetainedCloneStep::Progress(Default::default()))}
  let fields=&mut*self.fields;
  let turn=if let Some(work)=fields.clone.as_mut(){
   if work.terminal_is_empty(){fields.clone=None;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}}else{work.close_step(grant)}
  }else if let Some(admission)=fields.admission.as_mut(){
   let turn=if admission.phase()=="ready"{admission.take_output(&mut fields.base,grant)}else{admission.return_inputs(&mut fields.returned,grant)};if admission.terminal_is_empty(){fields.admission=None}turn
  }else if !self.retirement.is_empty(){self.retirement.step_granted(grant)}
  else if fields.returned.is_some(){let turn=self.retirement.begin_granted(&mut fields.returned,grant);RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Complete(Default::default()))),work:turn.work}}
  else if fields.scratch.is_some(){let turn=self.retirement.begin_granted(&mut fields.scratch,grant);RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Complete(Default::default()))),work:turn.work}}
  else{let base=fields.base.as_mut().ok_or_else(invalid)?;let turn=base.close_step(grant);if base.terminal_is_empty(){fields.base=None}turn};
  self.work=turn.work;self.progress=turn.result?.progress();Ok(RetainedCloneStep::Progress(self.progress))
 }
 pub(crate) fn terminal_is_empty(&self)->bool{self.closing&&self.fields.clone.is_none()&&self.fields.admission.is_none()&&self.fields.base.is_none()&&self.fields.scratch.is_none()&&self.fields.returned.is_none()&&self.retirement.is_empty()}
 pub(crate) fn step_progress(&self)->RetainedCloneProgress{self.progress}
 pub(crate) fn last_turn_work(&self)->u64{self.work}
}
impl Drop for OriginalSceneBaseWork{fn drop(&mut self){assert!(self.terminal_is_empty(),"immutable procedural base retains its actual source, admission or projected scratch");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.fields)}}}}
