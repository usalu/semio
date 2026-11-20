//! ↩️ Page override inverses prepare the original affected identity and override vector only.
use crate::MutationPayload;
use semio_s_artifact_layout_layout_mutations_decisions::set_page_overrides::SetPageOverrides;
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use semio_s_artifact_layout_layout_model::PageOverride;
use semio_os_kernel::RetainedMutationCloseContext;
use crate::original_field::{OriginalField,close_rows,rows_work_demand,close_binding};
use semio_framework_value::{OriginalRetirementCloseLoan,OriginalRetirementPoll,RetainedAllocationEffect,RetirementPollExit,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneRef,RetainedCloneBinding,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits}};
use semio_framework_value_factory::vector::FactoryVectorRetirement;
use std::mem::{ManuallyDrop,size_of};
fn fault()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"Page override inverse lost original source custody")}
fn fixed<M>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<M>()*2+256,depth:1,..Default::default()}}
fn nested(mut d:RetirementDemand)->RetirementDemand{d.copy_bytes+=256;d.depth+=1;d}
fn child(g:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-256,maximum_depth:g.maximum_depth-1,..g}}
fn receipt(mut p:RetainedCloneProgress)->RetainedCloneProgress{p.copied_bytes+=256;p.copied_items=p.copied_items.max(1);p}
fn refused(e:ValueError)->ValueError{let p=receipt(e.retained_progress());e.with_retained_progress(p)}
/// 🧳️ Only one inverse command and its affected field become newly owned state.
pub struct OverrideInverse<M:MutationPayload<SetPageOverrides>>{b:Option<RetainedCloneBinding>,at:usize,id:OriginalField<String>,field:OriginalField<Vec<PageOverride>>,rows:ManuallyDrop<Option<Vec<M>>>,retirement:Option<FactoryVectorRetirement<M>>,phase:u8,closing:bool}
impl<M:MutationPayload<SetPageOverrides>> Default for OverrideInverse<M>{fn default()->Self{Self{b:None,at:0,id:Default::default(),field:Default::default(),rows:ManuallyDrop::new(None),retirement:None,phase:10,closing:false}}}
fn page_id<'a>(b:RetainedCloneRef<'a,LayoutSnapshot>,at:usize)->RetainedCloneRef<'a,String>{b.project(9,|b|&b.pages).project(at,move|p|&p[at]).project(0,|p|&p.id)}
fn overrides<'a>(b:RetainedCloneRef<'a,LayoutSnapshot>,at:usize)->RetainedCloneRef<'a,Vec<PageOverride>>{b.project(9,|b|&b.pages).project(at,move|p|&p[at]).project(6,|p|&p.overrides)}
impl<M:MutationPayload<SetPageOverrides>> OverrideInverse<M>{
 pub fn ready(&self)->bool{self.phase==5}
 pub fn demand(&self,b:RetainedCloneRef<'_,LayoutSnapshot>,at:usize,body:usize,maximum:u64)->Result<RetirementDemand,ValueError>{if self.closing{return Err(fault())}b.check_binding(&self.b)?;if self.b.is_some()&&self.at!=at{return Err(fault())}Ok(match self.phase{10=>nested(b.binding_demand(&self.b)?.unwrap_or_default()),0=>RetirementDemand{capacity_bytes:size_of::<M>(),..fixed::<M>()},1=>if self.id.value().is_some(){fixed::<M>()}else{nested(self.id.demand(page_id(b,at),body.min(256),maximum.saturating_sub(1))?)},2=>{let mut d=nested(self.id.exchange_demand()?);d.copy_bytes+=size_of::<M>()*2;d},3=>if self.field.value().is_some(){fixed::<M>()}else{nested(self.field.demand(overrides(b,at),body.min(256),maximum.saturating_sub(1))?)},4=>{let mut d=nested(self.field.exchange_demand()?);d.copy_bytes+=size_of::<M>()*2;d},5=>fixed::<M>(),_=>return Err(fault())})}
 pub fn embedded_birth_work()->Result<u64,ValueError>{
    [OriginalField::<String>::embedded_birth_work()?,OriginalField::<Vec<PageOverride>>::embedded_birth_work()?]
        .into_iter().try_fold(1u64,|work,child|work.checked_add(child).ok_or_else(fault))
 }
 pub fn work_demand(&self,b:RetainedCloneRef<'_,LayoutSnapshot>,at:usize,body:usize,maximum:u64)->Result<u64,ValueError>{
    let limit=maximum.saturating_sub(1);
    let selected=match self.phase{10=>b.binding_work_demand(&self.b)?,1 if self.id.value().is_none()=>self.id.work_demand(page_id(b,at),body.min(256),limit)?,3 if self.field.value().is_none()=>self.field.work_demand(overrides(b,at),body.min(256),limit)?,2|4=>1,_=>0};
    selected.checked_add(1).ok_or_else(fault)
 }
 pub fn advance(&mut self,b:RetainedCloneRef<'_,LayoutSnapshot>,at:usize,body:usize,maximum:u64,effect:&mut RetainedAllocationEffect<'_>,recover:bool)->Result<RetainedCloneStep,RetirementPollExit>{
    let g=effect.grant();
    let d=if recover{fixed::<M>()}else{self.demand(b,at,body,maximum)?};
    if !recover{
        if !retained_clone_grant_admits(g,d)||self.work_demand(b,at,body,maximum)?>maximum.min(effect.remaining_work_units()?){return Ok(RetainedCloneStep::Progress(Default::default()))}
        if !effect.performed(1)?{return Err(fault().into())}
    }
 let mut p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};match self.phase{10=>{p=receipt(b.bind(&mut self.b,&mut effect.nested(child(g))).result.map_err(refused)?.unwrap_or_default());self.at=at;self.phase=0},0=>{if effect.actual().is_some(){return Err(fault().into())}*effect.actual()=Some(0);self.phase=254;*self.rows=Some(Vec::new());let rows=self.rows.as_mut().unwrap();let admitted=rows.try_reserve_exact(1);p.retained_capacity_bytes=rows.capacity()*size_of::<M>();*effect.actual()=Some(p.retained_capacity_bytes);if admitted.is_err()||p.retained_capacity_bytes>g.maximum_capacity_bytes{return Err(fault().with_retained_progress(p).into())}self.phase=1},1=>if self.id.value().is_some(){self.phase=2}else{p=receipt(self.id.advance(page_id(b,at),body.min(256),maximum.saturating_sub(1),&mut effect.nested(child(g)),recover)?.progress())},2=>{let(value,paid)=self.id.take(&mut effect.nested(RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-256-size_of::<M>()*2,maximum_depth:g.maximum_depth-1,..g})).map_err(refused)?;p=receipt(paid);p.copied_bytes+=size_of::<M>()*2;self.rows.as_mut().unwrap().push(M::from_payload(SetPageOverrides{id:value.ok_or_else(fault)?,overrides:Vec::new()}));self.phase=3},3=>if self.field.value().is_some(){self.phase=4}else{p=receipt(self.field.advance(overrides(b,at),body.min(256),maximum.saturating_sub(1),&mut effect.nested(child(g)),recover)?.progress())},4=>{let(value,paid)=self.field.take(&mut effect.nested(RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-256-size_of::<M>()*2,maximum_depth:g.maximum_depth-1,..g})).map_err(refused)?;p=receipt(paid);p.copied_bytes+=size_of::<M>()*2;let Some(row)=self.rows.as_mut().unwrap()[0].payload_mut()else{return Err(fault().into())};row.overrides=value.ok_or_else(fault)?;self.phase=5},5=>return Ok(RetainedCloneStep::Complete(p)),_=>return Err(fault().into())}Ok(RetainedCloneStep::Progress(p))}
 pub fn take_demand(&self)->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Option<Vec<M>>>()*2,depth:1,..Default::default()}}
 pub fn take(&mut self,g:RetainedCloneGrant)->Result<(Option<Vec<M>>,RetainedCloneProgress),ValueError>{if !self.ready()||self.closing{return Err(fault())}let d=self.take_demand();if !retained_clone_grant_admits(g,d){return Ok((None,Default::default()))}self.phase=6;Ok((self.rows.take(),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
 pub fn begin_close(&mut self){self.closing=true;self.id.begin_close();self.field.begin_close()}
 pub fn close_demand(&self,c:RetainedMutationCloseContext<'_,M>,body:usize)->Result<RetirementDemand,ValueError>{if !self.id.terminal_is_empty(){return self.id.close_demand(body)}if !self.field.terminal_is_empty(){return self.field.close_demand(body)}if self.b.is_some(){return Ok(nested(RetainedCloneBinding::close_demand(&self.b,body.min(256))?))}if let Some(owner)=self.retirement.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<FactoryVectorRetirement<M>>>()*2,depth:1,..Default::default()})}else{owner.demands(c.mutations,body)}}if self.rows.is_some(){return Ok(FactoryVectorRetirement::<M>::constructor_demand())}Ok(Default::default())}
 pub fn close_work_demand(&self,c:RetainedMutationCloseContext<'_,M>,body:usize)->Result<u64,ValueError>{
    if !self.id.terminal_is_empty(){return self.id.close_work_demand(body)}
    if !self.field.terminal_is_empty(){return self.field.close_work_demand(body)}
    if self.b.is_some(){return RetainedCloneBinding::close_work_demand(&self.b,body.min(256))}
    rows_work_demand(&self.retirement,self.rows.is_some(),c,body)
 }
 pub fn requires_original_birth(&self)->bool{
    if !self.id.terminal_is_empty(){return self.id.requires_original_birth()}
    if !self.field.terminal_is_empty(){return self.field.requires_original_birth()}
    if self.b.is_some(){return RetainedCloneBinding::requires_original_birth(&self.b)}
    self.retirement.as_ref().is_some_and(|owner|owner.requires_original_birth())
 }
 pub fn requires_original_control(&self)->bool{
    if !self.id.terminal_is_empty(){return self.id.requires_original_control()}
    if !self.field.terminal_is_empty(){return self.field.requires_original_control()}
    if self.b.is_some(){return RetainedCloneBinding::requires_original_control(&self.b)}
    self.retirement.as_ref().is_some_and(|owner|owner.requires_original_control())
 }
 pub fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{
    if !self.id.terminal_is_empty(){return self.id.close_original_birth_step(original,grant)}
    if !self.field.terminal_is_empty(){return self.field.close_original_birth_step(original,grant)}
    if self.b.is_some(){return RetainedCloneBinding::close_original_birth_step(&mut self.b,original,child(grant))}
    self.retirement.as_mut().expect("selected Layout inverse retirement").close_original_birth_step(original,grant)
 }
 pub fn close_step(&mut self,c:RetainedMutationCloseContext<'_,M>,original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,recover:bool)->Result<RetainedCloneStep,RetirementPollExit>{
    if !self.closing{return Err(fault().into())}
    let grant=effect.grant();
    if !recover&&!retained_clone_grant_admits(grant,self.close_demand(c,body)?){return Ok(RetainedCloneStep::Progress(Default::default()))}
    if !self.id.terminal_is_empty(){return self.id.close_step(original,body,effect,recover)}
    if !self.field.terminal_is_empty(){return self.field.close_step(original,body,effect,recover)}
    if self.b.is_some(){return close_binding(&mut self.b,original,body.min(256),&mut effect.nested(child(grant))).map(|step|RetainedCloneStep::Progress(receipt(step.progress())))}
    close_rows(&mut self.retirement,&mut self.rows,c,original,body,effect)
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.b.is_none()&&self.id.terminal_is_empty()&&self.field.terminal_is_empty()&&self.rows.is_none()&&self.retirement.is_none()}
}
impl<M:MutationPayload<SetPageOverrides>> Drop for OverrideInverse<M>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"Page override inverse requires funded original cleanup")}}
