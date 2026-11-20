//! ↩️ Explicit whole-document inverses prepare the genuine original LayoutSnapshot under its original baseline binding.
use crate::MutationPayload;
use semio_s_artifact_layout_layout_mutations_decisions::replace_document::ReplaceDocument;
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use semio_os_kernel::RetainedMutationCloseContext;
use crate::original_field::{OriginalField,close_binding,close_rows,rows_work_demand};
use semio_framework_value::{OriginalRetirementCloseLoan,OriginalRetirementPoll,RetainedAllocationEffect,RetirementPollExit,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneRef,RetainedCloneBinding,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits}};
use semio_framework_value_factory::vector::FactoryVectorRetirement;
use std::mem::{ManuallyDrop,size_of};
fn fault()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"Whole document inverse lost original source custody")}
fn fixed<M>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<M>()*2+256,depth:1,..Default::default()}}
fn nested(mut d:RetirementDemand)->RetirementDemand{d.copy_bytes+=256;d.depth+=1;d}
fn child(g:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-256,maximum_depth:g.maximum_depth-1,..g}}
fn receipt(mut p:RetainedCloneProgress)->RetainedCloneProgress{p.copied_bytes+=256;p.copied_items=p.copied_items.max(1);p}
fn refused(e:ValueError)->ValueError{let p=receipt(e.retained_progress());e.with_retained_progress(p)}
/// 🧳️ Only the explicitly affected whole-document field and one inverse command become newly owned state.
pub struct DocumentInverse<M:MutationPayload<ReplaceDocument>>{b:Option<RetainedCloneBinding>,field:OriginalField<LayoutSnapshot>,rows:ManuallyDrop<Option<Vec<M>>>,retirement:Option<FactoryVectorRetirement<M>>,phase:u8,closing:bool}
impl<M:MutationPayload<ReplaceDocument>> Default for DocumentInverse<M>{fn default()->Self{Self{b:None,field:Default::default(),rows:ManuallyDrop::new(None),retirement:None,phase:10,closing:false}}}
impl<M:MutationPayload<ReplaceDocument>> DocumentInverse<M>{
 pub fn ready(&self)->bool{self.phase==5}
 pub fn demand(&self,b:RetainedCloneRef<'_,LayoutSnapshot>,body:usize,maximum:u64)->Result<RetirementDemand,ValueError>{if self.closing{return Err(fault())}b.check_binding(&self.b)?;Ok(match self.phase{10=>nested(b.binding_demand(&self.b)?.unwrap_or_default()),0=>RetirementDemand{capacity_bytes:size_of::<M>(),..fixed::<M>()},1=>if self.field.value().is_some(){fixed::<M>()}else{nested(self.field.demand(b,body.min(256),maximum.saturating_sub(1))?)},2=>{let mut d=nested(self.field.exchange_demand()?);d.copy_bytes+=size_of::<M>()*2;d},5=>fixed::<M>(),_=>return Err(fault())})}
 pub fn embedded_birth_work()->Result<u64,ValueError>{OriginalField::<LayoutSnapshot>::embedded_birth_work()}
 pub fn work_demand(&self,b:RetainedCloneRef<'_,LayoutSnapshot>,body:usize,maximum:u64)->Result<u64,ValueError>{
    let work=match self.phase{
        10=>b.binding_work_demand(&self.b)?,
        1 if self.field.value().is_none()=>self.field.work_demand(b,body.min(256),maximum.saturating_sub(1))?,
        2=>1,
        _=>0,
    };
    work.checked_add(1).ok_or_else(fault)
 }
 pub fn advance(&mut self,b:RetainedCloneRef<'_,LayoutSnapshot>,body:usize,maximum:u64,effect:&mut RetainedAllocationEffect<'_>,recover:bool)->Result<RetainedCloneStep,RetirementPollExit>{let g=effect.grant();let d=if recover{fixed::<M>()}else{self.demand(b,body,maximum)?};if !recover{if !retained_clone_grant_admits(g,d)||self.work_demand(b,body,maximum)?>maximum.min(effect.remaining_work_units()?){return Ok(RetainedCloneStep::Progress(Default::default()))}if !effect.performed(1)?{return Err(fault().into())}}let mut p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};match self.phase{10=>{p=receipt(b.bind(&mut self.b,&mut effect.nested(child(g))).result.map_err(refused)?.unwrap_or_default());self.phase=0},0=>{if effect.actual().is_some(){return Err(fault().into())}*effect.actual()=Some(0);self.phase=254;*self.rows=Some(Vec::new());let rows=self.rows.as_mut().unwrap();let admitted=rows.try_reserve_exact(1);p.retained_capacity_bytes=rows.capacity()*size_of::<M>();*effect.actual()=Some(p.retained_capacity_bytes);if admitted.is_err()||p.retained_capacity_bytes>g.maximum_capacity_bytes{return Err(fault().with_retained_progress(p).into())}self.phase=1},1=>if self.field.value().is_some(){self.phase=2}else{p=receipt(self.field.advance(b,body.min(256),maximum.saturating_sub(1),&mut effect.nested(child(g)),recover)?.progress())},2=>{let(value,paid)=self.field.take(&mut effect.nested(RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-256-size_of::<M>()*2,maximum_depth:g.maximum_depth-1,..g})).map_err(refused)?;p=receipt(paid);p.copied_bytes+=size_of::<M>()*2;self.rows.as_mut().unwrap().push(M::from_payload(ReplaceDocument{document:value.ok_or_else(fault)?}));self.phase=5},5=>return Ok(RetainedCloneStep::Complete(p)),_=>return Err(fault().into())}Ok(RetainedCloneStep::Progress(p))}
 pub fn take_demand(&self)->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Option<Vec<M>>>()*2,depth:1,..Default::default()}}
 pub fn take(&mut self,g:RetainedCloneGrant)->Result<(Option<Vec<M>>,RetainedCloneProgress),ValueError>{if !self.ready()||self.closing{return Err(fault())}let d=self.take_demand();if !retained_clone_grant_admits(g,d){return Ok((None,Default::default()))}self.phase=6;Ok((self.rows.take(),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
 pub fn begin_close(&mut self){self.closing=true;self.field.begin_close()}
 pub fn close_demand(&self,c:RetainedMutationCloseContext<'_,M>,body:usize)->Result<RetirementDemand,ValueError>{if !self.field.terminal_is_empty(){return self.field.close_demand(body)}if self.b.is_some(){return Ok(nested(RetainedCloneBinding::close_demand(&self.b,body.min(256))?))}if let Some(owner)=self.retirement.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<FactoryVectorRetirement<M>>>()*2,depth:1,..Default::default()})}else{owner.demands(c.mutations,body)}}if self.rows.is_some(){return Ok(FactoryVectorRetirement::<M>::constructor_demand())}Ok(Default::default())}
 pub fn close_work_demand(&self,c:RetainedMutationCloseContext<'_,M>,body:usize)->Result<u64,ValueError>{
    if !self.field.terminal_is_empty(){return self.field.close_work_demand(body)}
    if self.b.is_some(){return RetainedCloneBinding::close_work_demand(&self.b,body.min(256))}
    rows_work_demand(&self.retirement,self.rows.is_some(),c,body)
 }
 pub fn requires_original_birth(&self)->bool{
    if !self.field.terminal_is_empty(){self.field.requires_original_birth()}
    else if self.b.is_some(){RetainedCloneBinding::requires_original_birth(&self.b)}
    else{self.retirement.as_ref().is_some_and(|owner|owner.requires_original_birth())}
 }
 pub fn requires_original_control(&self)->bool{
    if !self.field.terminal_is_empty(){self.field.requires_original_control()}
    else if self.b.is_some(){RetainedCloneBinding::requires_original_control(&self.b)}
    else{self.retirement.as_ref().is_some_and(|owner|owner.requires_original_control())}
 }
 pub fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{
    if !self.field.terminal_is_empty(){return self.field.close_original_birth_step(original,grant)}
    if self.b.is_some(){return RetainedCloneBinding::close_original_birth_step(&mut self.b,original,child(grant))}
    self.retirement.as_mut().expect("selected whole-document inverse retirement").close_original_birth_step(original,grant)
 }
 pub fn close_step(&mut self,c:RetainedMutationCloseContext<'_,M>,original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,recover:bool)->Result<RetainedCloneStep,RetirementPollExit>{
    if !self.closing{return Err(fault().into())}
    let grant=effect.grant();
    if !recover&&!retained_clone_grant_admits(grant,self.close_demand(c,body)?){return Ok(RetainedCloneStep::Progress(Default::default()))}
    if !self.field.terminal_is_empty(){return self.field.close_step(original,body,effect,recover)}
    if self.b.is_some(){return close_binding(&mut self.b,original,body.min(256),&mut effect.nested(child(grant))).map(|step|RetainedCloneStep::Progress(receipt(step.progress())))}
    close_rows(&mut self.retirement,&mut self.rows,c,original,body,effect)
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.b.is_none()&&self.field.terminal_is_empty()&&self.rows.is_none()&&self.retirement.is_none()}
}
impl<M:MutationPayload<ReplaceDocument>> Drop for DocumentInverse<M>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"Whole document inverse requires funded original cleanup")}}
