//! 🔺️ Sparse override rows transfer original removed/moved identifiers and actual prepared changed rows.
use crate::MutationPayload;
use semio_s_artifact_layout_layout_mutations_decisions::set_page_overrides::SetPageOverrides;
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use semio_s_artifact_layout_layout_diff::{LayoutDiff,PageOverrideRemoval,PageOverrideInsertion,PageOverrideRelocation};
use crate::original_field::OriginalField;
use semio_s_artifact_layout_layout_model::PageOverride;
use semio_framework_value::{OriginalRetirementCloseLoan,OriginalRetirementPoll,RetainedAllocationEffect,RetirementPollExit,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneRef,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits}};
use semio_s_artifact_layout_layout_mutations_decisions::override_plan::OverrideRowsPlan;
use std::marker::PhantomData;
use std::mem::size_of;
fn fault()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"override sparse output lost original field custody")}
/// 🧳️ The central output owns partial delta allocations while this cursor retains only original source bindings and key preparation.
pub struct OverrideOutput<M:MutationPayload<SetPageOverrides>>{mutation:PhantomData<fn() -> M>,plan:OverrideRowsPlan,key:OriginalField<String>,phase:u8,resume:u8,index:usize,page:usize,bound:bool,closing:bool}
impl<M:MutationPayload<SetPageOverrides>> Default for OverrideOutput<M>{fn default()->Self{Self{mutation:PhantomData,plan:Default::default(),key:Default::default(),phase:0,resume:0,index:0,page:0,bound:false,closing:false}}}
impl<M:MutationPayload<SetPageOverrides>> OverrideOutput<M>{
 const FIXED:usize=size_of::<Self>()*2+size_of::<LayoutDiff>()*2+size_of::<M>()*2+1024;
 fn fixed()->RetirementDemand{RetirementDemand{copy_bytes:Self::FIXED,depth:1,..Default::default()}}
 fn nested(d:RetirementDemand)->RetirementDemand{RetirementDemand{copy_bytes:d.copy_bytes+Self::FIXED,depth:d.depth+1,..d}}
 fn child(g:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-Self::FIXED,maximum_depth:g.maximum_depth-1,..g}}
 fn receipt(mut p:RetainedCloneProgress)->RetainedCloneProgress{p.copied_items=p.copied_items.max(1);p.copied_bytes+=Self::FIXED;p}
 fn failure(e:ValueError)->ValueError{let p=Self::receipt(e.retained_progress());e.with_retained_progress(p)}
 fn original_key<'a>(&self,b:RetainedCloneRef<'a,LayoutSnapshot>)->RetainedCloneRef<'a,String>{let(index,page)=(self.index,self.page);b.project(9,|b|&b.pages).project(page,move|rows|&rows[page]).project(6,|p|&p.overrides).project(index,move|rows|&rows[index].object_id)}
 fn has_key(&self)->bool{self.index<self.plan.old_len()&&if self.phase==5{self.plan.removed(self.index)}else{self.plan.moved(self.index).is_some()}}
 pub fn ready(&self)->bool{self.phase==8&&!self.closing}
 pub fn demand(&self,m:RetainedCloneRef<'_,M>,b:RetainedCloneRef<'_,LayoutSnapshot>,body:usize,maximum:u64)->Result<RetirementDemand,ValueError>{if self.closing{return Err(fault())}self.plan.verify(m,b)?;Ok(match self.phase{0=>Self::nested(self.plan.demand(m,b,self.page,body.min(256))?),2=>RetirementDemand{capacity_bytes:self.plan.removed_count().checked_mul(size_of::<PageOverrideRemoval>()).ok_or_else(fault)?,..Self::fixed()},3=>RetirementDemand{capacity_bytes:self.plan.inserted_count().checked_mul(size_of::<PageOverrideInsertion>()).ok_or_else(fault)?,..Self::fixed()},4=>RetirementDemand{capacity_bytes:self.plan.moved_count().checked_mul(size_of::<PageOverrideRelocation>()).ok_or_else(fault)?,..Self::fixed()},5|7 if self.has_key()=>Self::nested(if self.key.value().is_some(){self.key.exchange_demand()?}else{self.key.demand(self.original_key(b),256,maximum.saturating_sub(1))?}),9=>if self.key.terminal_is_empty(){Self::fixed()}else{Self::nested(self.key.close_demand(body.saturating_sub(Self::FIXED))?)},_=>Self::fixed()})}
 pub fn embedded_birth_work()->Result<u64,ValueError>{OriginalField::<String>::embedded_birth_work()}
 pub fn work_demand(&self,m:RetainedCloneRef<'_,M>,b:RetainedCloneRef<'_,LayoutSnapshot>,body:usize,maximum:u64)->Result<u64,ValueError>{
    let work=match self.phase{
        0=>self.plan.work_demand(m,b)?,
        5|7 if self.index<self.plan.old_len()&&self.has_key()=>if self.key.value().is_some(){1}else{self.key.work_demand(self.original_key(b),256,maximum.saturating_sub(1))?},
        9 if !self.key.terminal_is_empty()=>self.key.close_work_demand(body.saturating_sub(Self::FIXED))?,
        _=>0,
    };
    work.checked_add(1).ok_or_else(fault)
 }
 pub fn advance(&mut self,prepared:&mut Vec<M>,m:RetainedCloneRef<'_,M>,b:RetainedCloneRef<'_,LayoutSnapshot>,at:[usize;2],diff:&mut LayoutDiff,body:usize,maximum:u64,effect:&mut RetainedAllocationEffect<'_>,recover:bool)->Result<RetainedCloneStep,RetirementPollExit>{
    let g=effect.grant();
    let d=if recover{Self::fixed()}else{self.demand(m,b,body,maximum)?};
    if !recover{
        if !retained_clone_grant_admits(g,d)||self.work_demand(m,b,body,maximum)?>maximum.min(effect.remaining_work_units()?){return Ok(RetainedCloneStep::Progress(Default::default()))}
        if !effect.performed(1)?{return Err(fault().into())}
    }
    let limit=maximum.saturating_sub(1);
 if self.bound&&self.page!=at[0]{return Err(fault().into())}self.page=at[0];self.bound=true;let mut p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};match self.phase{
 0=>{let step=self.plan.advance(m,b,self.page,&mut effect.nested(Self::child(g))).map_err(Self::failure)?;p=Self::receipt(step.progress());if self.plan.ready(){self.phase=1}},
 1=>{self.phase=2},
 2|3|4=>{let phase=self.phase;self.phase=254;if d.capacity_bytes!=0{if effect.actual().is_some(){return Err(fault().into())}*effect.actual()=Some(0)}let rows=&mut diff.pages.as_mut().ok_or_else(fault)?.modified.get_mut(0).ok_or_else(fault)?.patch.overrides;let(failed,bytes)=match phase{2=>(rows.removed.try_reserve_exact(self.plan.removed_count()).is_err(),rows.removed.capacity()*size_of::<PageOverrideRemoval>()),3=>(rows.inserted.try_reserve_exact(self.plan.inserted_count()).is_err(),rows.inserted.capacity()*size_of::<PageOverrideInsertion>()),_=>(rows.moved.try_reserve_exact(self.plan.moved_count()).is_err(),rows.moved.capacity()*size_of::<PageOverrideRelocation>())};p.retained_capacity_bytes=bytes;if d.capacity_bytes!=0{*effect.actual()=Some(bytes)}if failed||bytes>g.maximum_capacity_bytes{return Err(fault().with_retained_progress(p).into())}self.phase=phase+1},
 5|7=>{if self.index==self.plan.old_len(){self.index=0;self.phase=if self.phase==5{6}else{10}}else if !self.has_key(){self.index+=1}else if self.key.value().is_some(){let(key,paid)=self.key.take(&mut effect.nested(Self::child(g))).map_err(Self::failure)?;p=Self::receipt(paid);let id=key.ok_or_else(fault)?;let rows=&mut diff.pages.as_mut().ok_or_else(fault)?.modified.get_mut(0).ok_or_else(fault)?.patch.overrides;if self.phase==5{rows.removed.push(PageOverrideRemoval{id,index:self.index})}else{rows.moved.push(PageOverrideRelocation{id,from:self.index,to:self.plan.moved(self.index).ok_or_else(fault)?})}self.resume=self.phase;self.phase=9;self.key.begin_close()}else{let source=self.original_key(b);p=Self::receipt(self.key.advance(source,256,limit,&mut effect.nested(Self::child(g)),recover)?.progress())}},
 6=>if self.index==self.plan.new_len(){self.index=0;self.phase=7}else{if self.plan.inserted(self.index){let Some(authored)=prepared.last_mut().and_then(MutationPayload::payload_mut)else{return Err(fault().into())};let row=std::mem::replace(&mut authored.overrides[self.index],PageOverride{object_id:String::new(),bounds:None,visible:None,locked:None});diff.pages.as_mut().ok_or_else(fault)?.modified.get_mut(0).ok_or_else(fault)?.patch.overrides.inserted.push(PageOverrideInsertion{index:self.index,row})}self.index+=1},
 9=>if self.key.terminal_is_empty(){self.key=Default::default();self.phase=self.resume;self.index+=1}else{p=Self::receipt(self.key.close_step(None,body.saturating_sub(Self::FIXED),&mut effect.nested(Self::child(g)),recover)?.progress())},10=>{let Some(authored)=prepared.last_mut().and_then(MutationPayload::payload_mut)else{return Err(fault().into())};diff.pages.as_mut().ok_or_else(fault)?.modified.get_mut(0).ok_or_else(fault)?.id=std::mem::take(&mut authored.id);self.phase=8;return Ok(RetainedCloneStep::Complete(p))},_=>return Err(fault().into())}Ok(if self.ready(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})}
 pub fn begin_close(&mut self){self.closing=true;self.key.begin_close();self.plan.begin_close()}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(fault())}if !self.key.terminal_is_empty(){return Ok(Self::nested(self.key.close_demand(body.saturating_sub(Self::FIXED))?))}if !self.plan.terminal_is_empty(){return Ok(Self::nested(self.plan.close_demand(body.saturating_sub(Self::FIXED))?))}Ok(Self::fixed())}
 pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{
    let inner=body.saturating_sub(Self::FIXED);
    if !self.key.terminal_is_empty(){self.key.close_work_demand(inner)}else{self.plan.close_work_demand(inner)}
 }
 pub fn requires_original_birth(&self)->bool{
    if !self.key.terminal_is_empty(){self.key.requires_original_birth()}else{self.plan.requires_original_birth()}
 }
 pub fn requires_original_control(&self)->bool{
    if !self.key.terminal_is_empty(){self.key.requires_original_control()}else{self.plan.requires_original_control()}
 }
 pub fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{
    if !self.key.terminal_is_empty(){self.key.close_original_birth_step(original,Self::child(grant))}
    else{self.plan.close_original_birth_step(original,Self::child(grant))}
 }
 pub fn close_step(&mut self,original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,recover:bool)->Result<RetainedCloneStep,RetirementPollExit>{
    let g=effect.grant();
    if !recover&&!retained_clone_grant_admits(g,self.close_demand(body)?){return Ok(RetainedCloneStep::Progress(Default::default()))}
    if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
    let inner=body.saturating_sub(Self::FIXED);
    let mut child=effect.nested(Self::child(g));
    let step=if !self.key.terminal_is_empty(){self.key.close_step(original,inner,&mut child,recover)?}
        else{self.plan.close_step(original,inner,&mut child)?};
    Ok(RetainedCloneStep::Progress(Self::receipt(step.progress())))
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.key.terminal_is_empty()&&self.plan.terminal_is_empty()}
}
impl<M:MutationPayload<SetPageOverrides>> Drop for OverrideOutput<M>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"override sparse output requires original-funded cleanup")}}
