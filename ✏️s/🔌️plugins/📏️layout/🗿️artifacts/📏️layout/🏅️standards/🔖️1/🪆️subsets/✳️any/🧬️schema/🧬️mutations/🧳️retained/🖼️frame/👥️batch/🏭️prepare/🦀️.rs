//! 🏭️ Batched original numeric decisions publish genuine sparse rows, exact messages and executable inverse events.
use crate::LayoutMutation;
use semio_s_artifact_layout_layout_diff::LayoutDiff;
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use crate::retained::original_field::{OriginalField,close_rows,rows_work_demand,settle_poll};
use semio_os_kernel::{RetainedMutationDecision,RetainedMutationCloseContext,MutationMessage,MutationMessageLedgerRetirement};
use semio_framework_value::{OriginalRetirementCloseLoan,OriginalRetirementPoll,RetainedAllocationEffect,RetainedWorkOutcome,RetainedWorkPoll,RetirementPollExit,RetirementDemand,ValueError,ValueRefusalKind,ErasedSnapshotRetirement,retained_clone::{RetainedCloneRef,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits}};
use semio_framework_value_factory::vector::FactoryVectorRetirement;
use super::{batch_plan::FrameSelectionPlan,batch_output::BatchOutput,batch_inverse::BatchInverse,batch_message::DynamicMessagePreparation,batch_source::{BatchDiagnosticSource,count}};
use std::mem::{ManuallyDrop,size_of};
pub fn selected(m:&LayoutMutation)->bool{matches!(m,LayoutMutation::DragFrames(_)|LayoutMutation::RotateFrames(_)|LayoutMutation::ScaleFrames(_))}
fn fault()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"batch mutation original preparation custody is invalid")}
fn fixed()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<LayoutMutation>()*3+size_of::<LayoutDiff>()*3+size_of::<MutationMessage>()*3+512,depth:1,..Default::default()}}
fn nested(mut d:RetirementDemand)->RetirementDemand{d.copy_bytes+=fixed().copy_bytes;d.depth+=1;d}
fn child(g:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-fixed().copy_bytes,maximum_depth:g.maximum_depth-1,..g}}
fn receipt(mut p:RetainedCloneProgress)->RetainedCloneProgress{p.copied_bytes+=fixed().copy_bytes;p.copied_items=p.copied_items.max(1);p}
fn refused(e:ValueError)->ValueError{let p=receipt(e.retained_progress());e.with_retained_progress(p)}
/// 🧳 Original plan authority outlives every partial diagnostic, payload, output and inverse owner.
pub struct BatchPreparation{plan:FrameSelectionPlan,payload:OriginalField<LayoutMutation>,output:BatchOutput,inverse:BatchInverse,diagnostic:DynamicMessagePreparation,messages:ManuallyDrop<Option<Vec<MutationMessage>>>,scratch_ledger:ManuallyDrop<Option<MutationMessageLedgerRetirement>>,payload_rows:ManuallyDrop<Option<Vec<LayoutMutation>>>,inverse_rows:ManuallyDrop<Option<Vec<LayoutMutation>>>,mutation_retirement:Option<FactoryVectorRetirement<LayoutMutation>>,diff:ManuallyDrop<Option<LayoutDiff>>,retirement:RetainedCloneClose,ledger:ManuallyDrop<Option<MutationMessageLedgerRetirement>>,phase:u8,index:usize,inverse_phase:u8,decision_ready:bool,inverse_ready:bool,closing:bool}
impl Default for BatchPreparation{fn default()->Self{Self{plan:Default::default(),payload:Default::default(),output:Default::default(),inverse:Default::default(),diagnostic:Default::default(),messages:ManuallyDrop::new(None),scratch_ledger:ManuallyDrop::new(None),payload_rows:ManuallyDrop::new(None),inverse_rows:ManuallyDrop::new(None),mutation_retirement:None,diff:ManuallyDrop::new(None),retirement:Default::default(),ledger:ManuallyDrop::new(None),phase:0,index:0,inverse_phase:0,decision_ready:false,inverse_ready:false,closing:false}}}
impl BatchPreparation{
 pub fn decision_demand(&self,m:RetainedCloneRef<'_,LayoutMutation>,b:RetainedCloneRef<'_,LayoutSnapshot>,body:usize,max:u64)->Result<RetirementDemand,ValueError>{if self.closing||self.decision_ready{return Err(fault())}if self.phase>0{self.plan.verify(m,b)?}Ok(match self.phase{0=>nested(self.plan.demand(m,b)?),1=>RetirementDemand{capacity_bytes:count(&self.plan)*size_of::<MutationMessage>(),..fixed()},2=>if self.index==count(&self.plan){fixed()}else{nested(self.diagnostic.demand(&BatchDiagnosticSource{m,b,plan:&self.plan,index:self.index},body.min(256),max.saturating_sub(1))?)},3=>nested(self.diagnostic.take_demand()),4=>if !self.diagnostic.terminal_is_empty(){nested(self.diagnostic.close_demand(body.min(256))?)}else if let Some(ledger)=self.scratch_ledger.as_ref(){nested(ledger.next_demand(body.min(256))?)}else{fixed()},5=>if self.payload.value().is_some(){fixed()}else{nested(self.payload.demand(m,body.min(256),max.saturating_sub(1))?)},6=>RetirementDemand{capacity_bytes:size_of::<LayoutMutation>(),..fixed()},7=>nested(self.payload.exchange_demand()?),8=>nested(self.output.demand(&self.plan,m,b)?),9=>nested(self.output.take_demand()),90=>fixed(),_=>return Err(fault())})}
 pub fn embedded_birth_work()->Result<u64,ValueError>{
  [OriginalField::<LayoutMutation>::embedded_birth_work()?,BatchInverse::embedded_birth_work()?,DynamicMessagePreparation::embedded_birth_work()?].into_iter().try_fold(1u64,|work,child|work.checked_add(child).ok_or_else(fault))
 }
 pub fn decision_work_demand(&self,m:RetainedCloneRef<'_,LayoutMutation>,b:RetainedCloneRef<'_,LayoutSnapshot>,body:usize,max:u64)->Result<u64,ValueError>{
  self.decision_demand(m,b,body,max)?;
  let body=body.min(256);let ceiling=max.saturating_sub(1);
  let work=match self.phase{
   0=>self.plan.work_demand(m,b)?,
   2 if self.index<count(&self.plan)=>self.diagnostic.work_demand(&BatchDiagnosticSource{m,b,plan:&self.plan,index:self.index},body,ceiling)?,
   4 if !self.diagnostic.terminal_is_empty()=>self.diagnostic.close_work_demand(body)?,
   4=>self.scratch_ledger.as_ref().map_or(Ok(0),|ledger|ledger.next_computational_work(body))?,
   5 if self.payload.value().is_none()=>self.payload.work_demand(m,body,ceiling)?,
   7|8=>1,
   _=>0,
  };
  let work=work.checked_add(1).ok_or_else(fault)?;
  Ok(if work<=max{work}else{0})
 }
 pub fn decide(&mut self,m:RetainedCloneRef<'_,LayoutMutation>,b:RetainedCloneRef<'_,LayoutSnapshot>,body:usize,max:u64,effect:&mut RetainedAllocationEffect<'_>,recover:bool)->Result<RetainedCloneStep,RetirementPollExit>{let g=effect.grant();let d=if recover{Default::default()}else{let d=self.decision_demand(m,b,body,max)?;let work=self.decision_work_demand(m,b,body,max)?;if !retained_clone_grant_admits(g,d)||work==0||effect.remaining_work_units()?<work{return Ok(RetainedCloneStep::Progress(Default::default()))}if !effect.performed(1)?{return Err(fault().into())}d};let ceiling=max.saturating_sub(1);let body=body.min(256);let mut p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};match self.phase{
 0=>{let step=self.plan.advance(m,b,&mut effect.nested(child(g))).map_err(refused)?;p=receipt(step.progress());if self.plan.ready(){self.phase=1}},
 1=>{self.phase=254;*self.messages=Some(Vec::new());let rows=self.messages.as_mut().unwrap();let amount=count(&self.plan);if amount!=0{if effect.actual().is_some(){return Err(fault().into())}*effect.actual()=Some(0);}let reserved=rows.try_reserve_exact(amount);p.retained_capacity_bytes=rows.capacity()*size_of::<MutationMessage>();if amount!=0{*effect.actual()=Some(p.retained_capacity_bytes);}if reserved.is_err()||p.retained_capacity_bytes>g.maximum_capacity_bytes{return Err(fault().with_retained_progress(p).into())}self.phase=2},
 2=>if self.index==count(&self.plan){if self.plan.failure.is_some()||self.plan.changed().is_empty(){*self.diff=Some(LayoutDiff::default());self.phase=90}else{self.phase=5}}else{let step=self.diagnostic.advance(&BatchDiagnosticSource{m,b,plan:&self.plan,index:self.index},body,ceiling,&mut effect.nested(child(g)),recover)?;p=receipt(step.progress());if self.diagnostic.ready(){self.phase=3}},
 3=>{let(mut rows,paid)=self.diagnostic.take(child(g)).map_err(refused)?.ok_or_else(fault)?;p=receipt(paid);let row=rows.pop().ok_or_else(fault)?;self.messages.as_mut().ok_or_else(fault)?.push(row);*self.scratch_ledger=Some(MutationMessageLedgerRetirement::new(String::new(),rows));self.diagnostic.begin_close();self.phase=4},
 4=>if !self.diagnostic.terminal_is_empty(){p=receipt(self.diagnostic.close_step(None,body,&mut effect.nested(child(g)),recover)?.progress())}else if let Some(ledger)=self.scratch_ledger.as_mut(){if ledger.terminal_is_empty(){*self.scratch_ledger=None}else{let step=ledger.close_step(child(g)).map_err(refused)?;if !effect.performed(ledger.last_turn_work())?{return Err(fault().into())}p=receipt(step.progress())}}else{self.diagnostic=Default::default();self.index+=1;self.phase=2},
 5=>if self.payload.value().is_some(){self.phase=6}else{p=receipt(self.payload.advance(m,body,ceiling,&mut effect.nested(child(g)),recover)?.progress())},
 6=>{self.phase=254;*self.payload_rows=Some(Vec::new());let rows=self.payload_rows.as_mut().unwrap();let amount=1;if amount!=0{if effect.actual().is_some(){return Err(fault().into())}*effect.actual()=Some(0);}let reserved=rows.try_reserve_exact(amount);p.retained_capacity_bytes=rows.capacity()*size_of::<LayoutMutation>();if amount!=0{*effect.actual()=Some(p.retained_capacity_bytes);}if reserved.is_err()||p.retained_capacity_bytes>g.maximum_capacity_bytes{return Err(fault().with_retained_progress(p).into())}self.phase=7},
 7=>{let(value,paid)=self.payload.take(&mut effect.nested(child(g))).map_err(refused)?;p=receipt(paid);self.payload_rows.as_mut().ok_or_else(fault)?.push(value.ok_or_else(fault)?);self.phase=8},
 8=>{let step=self.output.advance(self.payload_rows.as_mut().ok_or_else(fault)?,&self.plan,m,b,&mut effect.nested(child(g))).map_err(refused)?;p=receipt(step.progress());if matches!(step,RetainedCloneStep::Complete(_)){self.phase=9}},
 9=>{let(value,paid)=self.output.take(child(g)).map_err(refused)?;p=receipt(paid);*self.diff=value;self.phase=90},90=>{self.decision_ready=true;return Ok(RetainedCloneStep::Complete(p))},_=>return Err(fault().into())}Ok(RetainedCloneStep::Progress(p))}
 pub fn take_decision(&mut self)->RetainedMutationDecision<LayoutDiff>{assert!(self.decision_ready&&!self.closing);let rows=self.messages.take().unwrap_or_default();if self.plan.failure.is_some(){RetainedMutationDecision::Refused(rows)}else{RetainedMutationDecision::Admitted(self.diff.take().expect("original sparse batch output"),rows)}}
 pub fn inverse_demand(&self,m:RetainedCloneRef<'_,LayoutMutation>,b:RetainedCloneRef<'_,LayoutSnapshot>,body:usize,max:u64)->Result<RetirementDemand,ValueError>{if self.closing||!self.decision_ready||self.plan.failure.is_some()||self.inverse_ready{return Err(fault())}self.plan.verify(m,b)?;Ok(if self.inverse_phase==0{nested(self.inverse.demand(&self.plan,m,b,body.min(256),max.saturating_sub(1))?)}else{nested(self.inverse.take_demand())})}
 pub fn inverse_work_demand(&self,m:RetainedCloneRef<'_,LayoutMutation>,b:RetainedCloneRef<'_,LayoutSnapshot>,body:usize,max:u64)->Result<u64,ValueError>{
  self.inverse_demand(m,b,body,max)?;
  let work=if self.inverse_phase==0{self.inverse.work_demand(&self.plan,m,b,body.min(256),max.saturating_sub(1))?}else{0};
  let work=work.checked_add(1).ok_or_else(fault)?;
  Ok(if work<=max{work}else{0})
 }
 pub fn inverse(&mut self,m:RetainedCloneRef<'_,LayoutMutation>,b:RetainedCloneRef<'_,LayoutSnapshot>,body:usize,max:u64,effect:&mut RetainedAllocationEffect<'_>,recover:bool)->Result<RetainedCloneStep,RetirementPollExit>{
  let g=effect.grant();
  if !recover{
   let d=self.inverse_demand(m,b,body,max)?;let work=self.inverse_work_demand(m,b,body,max)?;
   if !retained_clone_grant_admits(g,d)||work==0||effect.remaining_work_units()?<work{return Ok(RetainedCloneStep::Progress(Default::default()))}
   if !effect.performed(1)?{return Err(fault().into())}
  }
  if self.inverse_phase==0{
   let step=self.inverse.advance(&self.plan,m,b,body.min(256),max.saturating_sub(1),&mut effect.nested(child(g)),recover)?;
   let p=receipt(step.progress());if matches!(step,RetainedCloneStep::Complete(_)){self.inverse_phase=1}
   Ok(RetainedCloneStep::Progress(p))
  }else{let(rows,p)=self.inverse.take(child(g)).map_err(refused)?;*self.inverse_rows=rows;self.inverse_ready=true;Ok(RetainedCloneStep::Complete(receipt(p)))}
 }
 pub fn take_inverse(&mut self)->Vec<LayoutMutation>{assert!(self.inverse_ready&&!self.closing);self.inverse_rows.take().unwrap_or_default()}
 pub fn begin_close(&mut self){self.closing=true;self.inverse.begin_close();self.output.begin_close();self.payload.begin_close();self.diagnostic.begin_close();self.plan.begin_close()}
 pub fn close_demand(&self,c:RetainedMutationCloseContext<'_,LayoutMutation>,body:usize)->Result<RetirementDemand,ValueError>{if !self.inverse.terminal_is_empty(){return self.inverse.close_demand(c,body)}if !self.output.terminal_is_empty(){return self.output.close_demand(body)}if !self.payload.terminal_is_empty(){return self.payload.close_demand(body)}if !self.diagnostic.terminal_is_empty(){return self.diagnostic.close_demand(body)}if let Some(ledger)=self.scratch_ledger.as_ref(){return ledger.next_demand(body)}if !self.plan.terminal_is_empty(){return self.plan.close_demand(body)}if !self.retirement.is_empty()||self.diff.is_some(){return self.retirement.next_owner_demands::<LayoutDiff>(self.diff.is_some(),&None,body)}if let Some(owner)=self.mutation_retirement.as_ref(){return if owner.terminal_is_empty(){Ok(fixed())}else{owner.demands(c.mutations,body)}}if self.payload_rows.is_some()||self.inverse_rows.is_some(){return Ok(FactoryVectorRetirement::<LayoutMutation>::constructor_demand())}if self.messages.is_some(){return Ok(fixed())}if let Some(ledger)=self.ledger.as_ref(){return ledger.next_demand(body)}Ok(fixed())}
 pub fn close_work_demand(&self,c:RetainedMutationCloseContext<'_,LayoutMutation>,body:usize)->Result<u64,ValueError>{
  if !self.inverse.terminal_is_empty(){return self.inverse.close_work_demand(c,body)}
  if !self.output.terminal_is_empty(){return self.output.close_work_demand(body)}
  if !self.payload.terminal_is_empty(){return self.payload.close_work_demand(body)}
  if !self.diagnostic.terminal_is_empty(){return self.diagnostic.close_work_demand(body)}
  if let Some(ledger)=self.scratch_ledger.as_ref(){return if ledger.terminal_is_empty(){Ok(1)}else{ledger.next_computational_work(body)}}
  if !self.plan.terminal_is_empty(){return self.plan.close_work_demand(body)}
  if !self.retirement.is_empty()||self.diff.is_some(){return self.retirement.next_owner_work::<LayoutDiff>(self.diff.is_some(),&None,body)}
  if self.mutation_retirement.is_some()||self.payload_rows.is_some()||self.inverse_rows.is_some(){return rows_work_demand(&self.mutation_retirement,true,c,body)}
  if self.messages.is_some(){return Ok(1)}
  if let Some(ledger)=self.ledger.as_ref(){return if ledger.terminal_is_empty(){Ok(1)}else{ledger.next_computational_work(body)}}
  Ok(0)
 }
 pub fn requires_original_birth(&self)->bool{
  if !self.inverse.terminal_is_empty(){return self.inverse.requires_original_birth()}
  if !self.output.terminal_is_empty(){return self.output.requires_original_birth()}
  if !self.payload.terminal_is_empty(){return self.payload.requires_original_birth()}
  if !self.diagnostic.terminal_is_empty(){return self.diagnostic.requires_original_birth()}
  if self.scratch_ledger.is_some()||!self.plan.terminal_is_empty(){return false}
  if !self.retirement.is_empty(){return self.retirement.requires_original_birth()}
  self.diff.is_none()&&self.mutation_retirement.as_ref().is_some_and(FactoryVectorRetirement::requires_original_birth)
 }
 pub fn requires_original_control(&self)->bool{
  if !self.inverse.terminal_is_empty(){return self.inverse.requires_original_control()}
  if !self.output.terminal_is_empty(){return self.output.requires_original_control()}
  if !self.payload.terminal_is_empty(){return self.payload.requires_original_control()}
  if !self.diagnostic.terminal_is_empty(){return self.diagnostic.requires_original_control()}
  if self.scratch_ledger.is_some()||!self.plan.terminal_is_empty(){return false}
  if !self.retirement.is_empty(){return self.retirement.requires_original_control()}
  self.diff.is_none()&&self.mutation_retirement.as_ref().is_some_and(FactoryVectorRetirement::requires_original_control)
 }
 pub fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{
  if !self.inverse.terminal_is_empty(){return self.inverse.close_original_birth_step(original,grant)}
  if !self.output.terminal_is_empty(){return self.output.close_original_birth_step(original,grant)}
  if !self.payload.terminal_is_empty(){return self.payload.close_original_birth_step(original,grant)}
  if !self.diagnostic.terminal_is_empty(){return self.diagnostic.close_original_birth_step(original,grant)}
  if !self.retirement.is_empty(){return self.retirement.step_original_birth(original,grant)}
  match self.mutation_retirement.as_mut(){Some(rows)=>rows.close_original_birth_step(original,grant),None=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(fault().into()),work:0})}
 }
 pub fn close_step(&mut self,c:RetainedMutationCloseContext<'_,LayoutMutation>,original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,recover:bool)->Result<RetainedCloneStep,RetirementPollExit>{
  if !self.closing{return Err(fault().into())}
  let g=effect.grant();
  let d=if recover{Default::default()}else{let d=self.close_demand(c,body)?;if !retained_clone_grant_admits(g,d)||effect.remaining_work_units()?<self.close_work_demand(c,body)?{return Ok(RetainedCloneStep::Progress(Default::default()))}d};
  let progress=|step:RetainedCloneStep|RetainedCloneStep::Progress(step.progress());
  if !self.inverse.terminal_is_empty(){return self.inverse.close_step(c,original,body,effect,recover).map(progress)}
  if !self.output.terminal_is_empty(){return self.output.close_step(original,body,effect).map(progress)}
  if !self.payload.terminal_is_empty(){return self.payload.close_step(original,body,effect,recover).map(progress)}
  if !self.diagnostic.terminal_is_empty(){return self.diagnostic.close_step(original,body,effect,recover).map(progress)}
  let header=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};
  if let Some(ledger)=self.scratch_ledger.as_mut(){
   if ledger.terminal_is_empty(){if !effect.performed(1)?{return Err(fault().into())}*self.scratch_ledger=None;return Ok(RetainedCloneStep::Progress(header))}
   let step=ledger.close_step(g)?;
   if !effect.performed(ledger.last_turn_work())?{return Err(fault().into())}
   return Ok(progress(step))
  }
  if !self.plan.terminal_is_empty(){return self.plan.close_step(body,effect).map(progress).map_err(Into::into)}
  if !self.retirement.is_empty(){
   let poll=if self.retirement.requires_original_control(){self.retirement.step_original(original.ok_or_else(fault)?,body,effect)}else{self.retirement.step_granted(body,effect)};
   return settle_poll(poll).map(progress)
  }
  if self.diff.is_some(){
   let step=self.retirement.begin_granted(&mut self.diff,effect).result?;
   return Ok(progress(step.ok_or_else(fault)?))
  }
  if self.mutation_retirement.is_some()||self.payload_rows.is_some()||self.inverse_rows.is_some(){
   let rows=if self.payload_rows.is_some(){&mut self.payload_rows}else{&mut self.inverse_rows};
   return close_rows(&mut self.mutation_retirement,rows,c,original,body,effect).map(progress)
  }
  if self.messages.is_some(){
   if !effect.performed(1)?{return Err(fault().into())}
   *self.ledger=Some(MutationMessageLedgerRetirement::new(String::new(),self.messages.take().unwrap()));
   return Ok(RetainedCloneStep::Progress(header))
  }
  if let Some(ledger)=self.ledger.as_mut(){
   if ledger.terminal_is_empty(){if !effect.performed(1)?{return Err(fault().into())}*self.ledger=None;return Ok(RetainedCloneStep::Progress(header))}
   let step=ledger.close_step(g)?;
   if !effect.performed(ledger.last_turn_work())?{return Err(fault().into())}
   return Ok(progress(step))
  }
  Ok(RetainedCloneStep::Complete(Default::default()))
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.inverse.terminal_is_empty()&&self.output.terminal_is_empty()&&self.payload.terminal_is_empty()&&self.diagnostic.terminal_is_empty()&&self.plan.terminal_is_empty()&&self.scratch_ledger.is_none()&&self.diff.is_none()&&self.retirement.is_empty()&&self.payload_rows.is_none()&&self.inverse_rows.is_none()&&self.mutation_retirement.is_none()&&self.messages.is_none()&&self.ledger.is_none()}
}
impl Drop for BatchPreparation{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"batch preparation needs original funded cleanup")}}
