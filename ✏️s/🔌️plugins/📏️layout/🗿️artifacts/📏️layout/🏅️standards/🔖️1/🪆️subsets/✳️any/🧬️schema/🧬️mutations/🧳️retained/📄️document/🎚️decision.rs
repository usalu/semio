//! 📄️ Actual authored document validation precedes original IEEE equality and the no-op decision.
use crate::{LayoutMutationSource,LayoutMutationView as LayoutMutation};
use semio_s_artifact_layout_layout_model::{snapshot::LayoutSnapshot,validation::retained::LayoutValidationCursor};
use semio_framework_value::{RetainedAllocationEffect,RetainedWorkPoll,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneRef,RetainedCloneBinding,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits}};
use crate::simple::SimpleAdvance;
use crate::document_equality::LayoutDocumentEquality;
fn fault()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original whole-document decision custody is invalid")}
fn fixed()->RetirementDemand{RetirementDemand{copy_bytes:2048,depth:1,..Default::default()}}
fn nested(mut d:RetirementDemand)->RetirementDemand{d.copy_bytes+=2048;d.depth+=1;d}
fn child(g:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-2048,maximum_depth:g.maximum_depth-1,..g}}
fn paid(mut p:RetainedCloneProgress)->RetainedCloneProgress{p.copied_items=p.copied_items.max(1);p.copied_bytes+=2048;p}
fn refused(e:ValueError)->ValueError{let p=paid(e.retained_progress());e.with_retained_progress(p)}
pub fn authored<'a,M:LayoutMutationSource>(m:RetainedCloneRef<'a,M>)->RetainedCloneRef<'a,LayoutSnapshot>{m.project(8500,|m|match m.view(){LayoutMutation::ReplaceDocument(p)=>&p.document,_=>unreachable!("original ReplaceDocument source")})}
/// 🛡️ Original mutation/baseline aliases remain held through both the actual graph program and equality loans.
pub struct DocumentDecision{m:Option<RetainedCloneBinding>,b:Option<RetainedCloneBinding>,validation:LayoutValidationCursor,equality:LayoutDocumentEquality,phase:u8,error:Option<&'static str>,message:Option<&'static str>,unchanged:bool,closing:bool}
impl Default for DocumentDecision{fn default()->Self{Self{m:None,b:None,validation:Default::default(),equality:Default::default(),phase:0,error:None,message:None,unchanged:false,closing:false}}}
impl DocumentDecision{
 pub fn demand<M:LayoutMutationSource>(&self,m:RetainedCloneRef<'_,M>,b:RetainedCloneRef<'_,LayoutSnapshot>,body:usize)->Result<RetirementDemand,ValueError>{if self.closing||!matches!(m.get().view(),LayoutMutation::ReplaceDocument(_)){return Err(fault())}m.check_binding(&self.m)?;b.check_binding(&self.b)?;Ok(match self.phase{0=>nested(m.binding_demand(&self.m)?.unwrap_or_default()),1=>nested(b.binding_demand(&self.b)?.unwrap_or_default()),2=>nested(self.validation.demand(authored(m),body.min(256))?),3=>nested(self.equality.demand(authored(m),b)?),30=>fixed(),_=>return Err(fault())})}
 pub fn diagnostic_message(&self)->&'static str{self.message.unwrap_or("The document is already set.")}
 pub fn work_demand<M:LayoutMutationSource>(&self,m:RetainedCloneRef<'_,M>,b:RetainedCloneRef<'_,LayoutSnapshot>,body:usize)->Result<u64,ValueError>{
  self.demand(m,b,body)?;
  let work=match self.phase{
   0=>m.binding_work_demand(&self.m)?,
   1=>b.binding_work_demand(&self.b)?,
   2=>self.validation.work_demand(authored(m),body.min(256))?,
   3=>self.equality.work_demand(authored(m),b)?,
   30=>return Ok(0),
   _=>return Err(fault()),
  };
  work.checked_add(1).ok_or_else(fault)
 }
 pub fn advance<M:LayoutMutationSource>(&mut self,m:RetainedCloneRef<'_,M>,b:RetainedCloneRef<'_,LayoutSnapshot>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Result<SimpleAdvance,ValueError>{
  let g=effect.grant();let d=self.demand(m,b,body)?;
  let mut out=SimpleAdvance{progress:Default::default(),done:self.phase==30,unchanged:self.unchanged,error:self.error};
  if out.done||!retained_clone_grant_admits(g,d)||effect.remaining_work_units()?<self.work_demand(m,b,body)?{return Ok(out)}
  if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"Layout document decision exceeded original work"))}
  let mut nested=effect.nested(child(g));
  out.progress=paid(match self.phase{
   0=>{let turn=m.bind(&mut self.m,&mut nested).result.map_err(refused)?.unwrap_or_default();if self.m.is_some(){self.phase=1}turn},
   1=>{let turn=b.bind(&mut self.b,&mut nested).result.map_err(refused)?.unwrap_or_default();if self.b.is_some(){self.phase=2}turn},
   2=>{let step=self.validation.advance(authored(m),body.min(256),&mut nested).map_err(refused)?;if step.done{if let Some(message)=step.error{self.message=Some(message);self.error=Some("mutation.invariant");self.phase=30}else{self.phase=3}}step.progress},
   3=>{let step=self.equality.advance(authored(m),b,&mut nested).map_err(refused)?;if let Some(equal)=self.equality.result(){self.unchanged=equal;self.phase=30}step.progress()},
   _=>return Err(fault()),
  });
  out.done=self.phase==30;out.unchanged=self.unchanged;out.error=self.error;Ok(out)
 }
 pub fn begin_close(&mut self){self.closing=true;self.validation.begin_close();self.equality.begin_close()}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(fault())}if !self.validation.terminal_is_empty(){return Ok(nested(self.validation.close_demand(body.min(256))?))}if !self.equality.terminal_is_empty(){return Ok(nested(self.equality.close_demand(body.min(256))?))}Ok(nested(RetainedCloneBinding::close_demand(if self.b.is_some(){&self.b}else{&self.m},body.min(256))?))}
 pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{
  self.close_demand(body)?;
  if self.terminal_is_empty(){return Ok(0)}
  let body=body.min(256);
  let work=if !self.validation.terminal_is_empty(){self.validation.close_work_demand(body)?}else if !self.equality.terminal_is_empty(){self.equality.close_work_demand(body)?}else{RetainedCloneBinding::close_work_demand(if self.b.is_some(){&self.b}else{&self.m},body)?};
  work.checked_add(1).ok_or_else(fault)
 }
 pub fn close_step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>{
  let g=effect.grant();let d=self.close_demand(body)?;
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
  if !retained_clone_grant_admits(g,d)||effect.remaining_work_units()?<self.close_work_demand(body)?{return Ok(RetainedCloneStep::Progress(Default::default()))}
  if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"Layout document close exceeded original work"))}
  let body=body.min(256);
  let mut nested=effect.nested(child(g));
  let step=if !self.validation.terminal_is_empty(){self.validation.close_step(body,&mut nested).map_err(refused)?}else if !self.equality.terminal_is_empty(){self.equality.close_step(body,&mut nested).map_err(refused)?}else{
   let binding=if self.b.is_some(){&mut self.b}else{&mut self.m};
   let RetainedWorkPoll::Complete(turn)=RetainedCloneBinding::close_one(binding,body,&mut nested) else{return Err(fault())};
   turn.result.map_err(|e|refused(e.into_value_error()))?
  };
  let p=paid(step.progress());Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.validation.terminal_is_empty()&&self.equality.terminal_is_empty()&&self.m.is_none()&&self.b.is_none()}
}
impl Drop for DocumentDecision{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"whole-document decision requires original graph/equality/source closure")}}
