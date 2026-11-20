//! ↩️ Frame membership inverses prepare only original identities or the removed original Frame.
use crate::LayoutMutation;
use semio_s_artifact_layout_layout_model::{snapshot::LayoutSnapshot,Frame};
use semio_os_kernel::RetainedMutationCloseContext;
use super::super::original_field::{OriginalField,close_rows,rows_work_demand};
use semio_framework_value::{OriginalRetirementCloseLoan,OriginalRetirementPoll,RetainedAllocationEffect,RetirementPollExit,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneRef,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits}};
use semio_framework_value_factory::vector::FactoryVectorRetirement;
use std::mem::{size_of,ManuallyDrop};
fn fault()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original Layout frame inverse custody is invalid")}
fn create(m:&LayoutMutation)->Result<bool,ValueError>{match m{LayoutMutation::CreateFrame(_)=>Ok(true),LayoutMutation::DeleteFrame(_)=>Ok(false),_=>Err(fault())}}
fn fixed()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<LayoutMutation>()*4+512,depth:1,..Default::default()}}
fn nested(mut d:RetirementDemand)->RetirementDemand{d.copy_bytes+=256;d.depth+=1;d}
fn child(g:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes.saturating_sub(256),maximum_depth:g.maximum_depth.saturating_sub(1),..g}}
fn receipt(mut p:RetainedCloneProgress)->RetainedCloneProgress{p.copied_bytes+=256;p.copied_items=p.copied_items.max(1);p}
fn refused(e:ValueError)->ValueError{let p=receipt(e.retained_progress());e.with_retained_progress(p)}
fn page<'a>(m:RetainedCloneRef<'a,LayoutMutation>,b:RetainedCloneRef<'a,LayoutSnapshot>,at:[usize;2])->RetainedCloneRef<'a,String>{if matches!(m.get(),LayoutMutation::CreateFrame(_)){m.project(2100,|m|match m{LayoutMutation::CreateFrame(p)=>&p.page_id,_=>unreachable!("authored frame page")})}else{b.project(9,|b|&b.pages).project(at[0],move|p|&p[at[0]]).project(0,|p|&p.id)}}
fn frame_id<'a>(m:RetainedCloneRef<'a,LayoutMutation>)->RetainedCloneRef<'a,String>{m.project(2101,|m|match m{LayoutMutation::CreateFrame(p)=>match &p.frame{Frame::Rect{id,..}|Frame::Text{id,..}|Frame::Image{id,..}=>id},_=>unreachable!("authored frame identity")})}
fn frame<'a>(b:RetainedCloneRef<'a,LayoutSnapshot>,at:[usize;2])->RetainedCloneRef<'a,Frame>{b.project(9,|b|&b.pages).project(at[0],move|p|&p[at[0]]).project(2,|p|&p.frames).project(at[1],move|frames|&frames[at[1]])}
/// 🧳 Every prepared field and partial typed inverse keeps its original source/factory retirement owner.
pub struct FrameInverse{page:OriginalField<String>,id:OriginalField<String>,frame:OriginalField<Frame>,rows:ManuallyDrop<Option<Vec<LayoutMutation>>>,retirement:Option<FactoryVectorRetirement<LayoutMutation>>,phase:u8,closing:bool}
impl Default for FrameInverse{fn default()->Self{Self{page:Default::default(),id:Default::default(),frame:Default::default(),rows:ManuallyDrop::new(None),retirement:None,phase:0,closing:false}}}
impl FrameInverse{
 pub fn demand(&self,m:RetainedCloneRef<'_,LayoutMutation>,b:RetainedCloneRef<'_,LayoutSnapshot>,at:[usize;2],body:usize,maximum:u64)->Result<RetirementDemand,ValueError>{if self.closing{return Err(fault())}let create=create(m.get())?;Ok(match self.phase{0=>RetirementDemand{capacity_bytes:size_of::<LayoutMutation>(),..fixed()},1=>if self.page.value().is_some(){fixed()}else{nested(self.page.demand(page(m,b,at),body.saturating_sub(256),maximum.saturating_sub(1))?)},2=>if create{if self.id.value().is_some(){fixed()}else{nested(self.id.demand(frame_id(m),body.saturating_sub(256),maximum.saturating_sub(1))?)}}else if self.frame.value().is_some(){fixed()}else{nested(self.frame.demand(frame(b,at),body.saturating_sub(256),maximum.saturating_sub(1))?)},3=>{let mut d=nested(self.page.exchange_demand()?);d.copy_bytes+=size_of::<LayoutMutation>()*2;d},4=>{let mut d=nested(if create{self.id.exchange_demand()?}else{self.frame.exchange_demand()?});d.copy_bytes+=size_of::<LayoutMutation>()*2;d},5=>fixed(),_=>return Err(fault())})}
 pub fn embedded_birth_work()->Result<u64,ValueError>{
    [OriginalField::<String>::embedded_birth_work()?,OriginalField::<String>::embedded_birth_work()?,OriginalField::<Frame>::embedded_birth_work()?]
        .into_iter().try_fold(1u64,|work,child|work.checked_add(child).ok_or_else(fault))
 }
 pub fn work_demand(&self,m:RetainedCloneRef<'_,LayoutMutation>,b:RetainedCloneRef<'_,LayoutSnapshot>,at:[usize;2],body:usize,maximum:u64)->Result<u64,ValueError>{
    let limit=maximum.saturating_sub(1);
    let selected=match self.phase{1 if self.page.value().is_none()=>self.page.work_demand(page(m,b,at),body.saturating_sub(256),limit)?,2 if create(m.get())?&&self.id.value().is_none()=>self.id.work_demand(frame_id(m),body.saturating_sub(256),limit)?,2 if !create(m.get())?&&self.frame.value().is_none()=>self.frame.work_demand(frame(b,at),body.saturating_sub(256),limit)?,3|4=>1,_=>0};
    selected.checked_add(1).ok_or_else(fault)
 }
 pub fn advance(&mut self,m:RetainedCloneRef<'_,LayoutMutation>,b:RetainedCloneRef<'_,LayoutSnapshot>,at:[usize;2],body:usize,maximum:u64,effect:&mut RetainedAllocationEffect<'_>,recover:bool)->Result<RetainedCloneStep,RetirementPollExit>{
    let g=effect.grant();
    let d=if recover{fixed()}else{self.demand(m,b,at,body,maximum)?};
    if !recover{
        if !retained_clone_grant_admits(g,d)||self.work_demand(m,b,at,body,maximum)?>maximum.min(effect.remaining_work_units()?){return Ok(RetainedCloneStep::Progress(Default::default()))}
        if !effect.performed(1)?{return Err(fault().into())}
    }
 let mut p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};match self.phase{0=>{if effect.actual().is_some(){return Err(fault().into())}*effect.actual()=Some(0);self.phase=254;*self.rows=Some(Vec::new());let rows=self.rows.as_mut().unwrap();let reserved=rows.try_reserve_exact(1);p.retained_capacity_bytes=rows.capacity()*size_of::<LayoutMutation>();*effect.actual()=Some(p.retained_capacity_bytes);if reserved.is_err()||p.retained_capacity_bytes>g.maximum_capacity_bytes{return Err(fault().with_retained_progress(p).into())}self.phase=1},1=>if self.page.value().is_some(){self.phase=2}else{p=receipt(self.page.advance(page(m,b,at),body.saturating_sub(256),maximum.saturating_sub(1),&mut effect.nested(child(g)),recover)?.progress())},2=>if create(m.get())?{if self.id.value().is_some(){self.phase=3}else{p=receipt(self.id.advance(frame_id(m),body.saturating_sub(256),maximum.saturating_sub(1),&mut effect.nested(child(g)),recover)?.progress())}}else if self.frame.value().is_some(){self.phase=3}else{p=receipt(self.frame.advance(frame(b,at),body.saturating_sub(256),maximum.saturating_sub(1),&mut effect.nested(child(g)),recover)?.progress())},3=>{let(value,paid)=self.page.take(&mut effect.nested(RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-256-size_of::<LayoutMutation>()*2,maximum_depth:g.maximum_depth-1,..g})).map_err(refused)?;p=receipt(paid);p.copied_bytes+=size_of::<LayoutMutation>()*2;let page_id=value.expect("original frame inverse page");self.rows.as_mut().unwrap().push(if create(m.get())?{LayoutMutation::DeleteFrame(crate::delete_frame::DeleteFrame{page_id,frame_id:String::new()})}else{LayoutMutation::CreateFrame(crate::create_frame::CreateFrame{page_id,frame:super::vacant_frame(),index:Some(at[1]),layer_id:None})});self.phase=4},4=>{let inner=RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-256-size_of::<LayoutMutation>()*2,maximum_depth:g.maximum_depth-1,..g};if create(m.get())?{let(value,paid)=self.id.take(&mut effect.nested(inner)).map_err(refused)?;p=receipt(paid);let LayoutMutation::DeleteFrame(row)=&mut self.rows.as_mut().unwrap()[0]else{return Err(fault().into())};row.frame_id=value.expect("original authored frame identity")}else{let(value,paid)=self.frame.take(&mut effect.nested(inner)).map_err(refused)?;p=receipt(paid);let LayoutMutation::CreateFrame(row)=&mut self.rows.as_mut().unwrap()[0]else{return Err(fault().into())};row.frame=value.expect("original removed frame")};p.copied_bytes+=size_of::<LayoutMutation>()*2;self.phase=5},5=>return Ok(RetainedCloneStep::Complete(p)),_=>return Err(fault().into())}Ok(if self.phase==5{RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})}
 pub fn take_demand(&self)->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Option<Vec<LayoutMutation>>>()*2,depth:1,..Default::default()}}
 pub fn take(&mut self,g:RetainedCloneGrant)->Result<(Option<Vec<LayoutMutation>>,RetainedCloneProgress),ValueError>{if self.phase!=5||self.closing{return Err(fault())}let d=self.take_demand();if !retained_clone_grant_admits(g,d){return Ok((None,Default::default()))}self.phase=6;Ok((self.rows.take(),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
 pub fn begin_close(&mut self){self.closing=true;self.page.begin_close();self.id.begin_close();self.frame.begin_close()}
 pub fn close_demand(&self,c:RetainedMutationCloseContext<'_,LayoutMutation>,body:usize)->Result<RetirementDemand,ValueError>{if !self.page.terminal_is_empty(){return self.page.close_demand(body)}if !self.id.terminal_is_empty(){return self.id.close_demand(body)}if !self.frame.terminal_is_empty(){return self.frame.close_demand(body)}if let Some(owner)=self.retirement.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<FactoryVectorRetirement<LayoutMutation>>>()*2+256,depth:1,..Default::default()})}else{owner.demands(c.mutations,body)}}if self.rows.is_some(){return Ok(FactoryVectorRetirement::<LayoutMutation>::constructor_demand())}Ok(Default::default())}
 pub fn close_work_demand(&self,c:RetainedMutationCloseContext<'_,LayoutMutation>,body:usize)->Result<u64,ValueError>{
    if !self.page.terminal_is_empty(){return self.page.close_work_demand(body)}
    if !self.id.terminal_is_empty(){return self.id.close_work_demand(body)}
    if !self.frame.terminal_is_empty(){return self.frame.close_work_demand(body)}

    rows_work_demand(&self.retirement,self.rows.is_some(),c,body)
 }
 pub fn requires_original_birth(&self)->bool{
    if !self.page.terminal_is_empty(){return self.page.requires_original_birth()}
    if !self.id.terminal_is_empty(){return self.id.requires_original_birth()}
    if !self.frame.terminal_is_empty(){return self.frame.requires_original_birth()}

    self.retirement.as_ref().is_some_and(|owner|owner.requires_original_birth())
 }
 pub fn requires_original_control(&self)->bool{
    if !self.page.terminal_is_empty(){return self.page.requires_original_control()}
    if !self.id.terminal_is_empty(){return self.id.requires_original_control()}
    if !self.frame.terminal_is_empty(){return self.frame.requires_original_control()}

    self.retirement.as_ref().is_some_and(|owner|owner.requires_original_control())
 }
 pub fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{
    if !self.page.terminal_is_empty(){return self.page.close_original_birth_step(original,grant)}
    if !self.id.terminal_is_empty(){return self.id.close_original_birth_step(original,grant)}
    if !self.frame.terminal_is_empty(){return self.frame.close_original_birth_step(original,grant)}

    self.retirement.as_mut().expect("selected Layout inverse retirement").close_original_birth_step(original,grant)
 }
 pub fn close_step(&mut self,c:RetainedMutationCloseContext<'_,LayoutMutation>,original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,recover:bool)->Result<RetainedCloneStep,RetirementPollExit>{
    if !self.closing{return Err(fault().into())}
    let grant=effect.grant();
    if !recover&&!retained_clone_grant_admits(grant,self.close_demand(c,body)?){return Ok(RetainedCloneStep::Progress(Default::default()))}
    if !self.page.terminal_is_empty(){return self.page.close_step(original,body,effect,recover)}
    if !self.id.terminal_is_empty(){return self.id.close_step(original,body,effect,recover)}
    if !self.frame.terminal_is_empty(){return self.frame.close_step(original,body,effect,recover)}

    close_rows(&mut self.retirement,&mut self.rows,c,original,body,effect)
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.page.terminal_is_empty()&&self.id.terminal_is_empty()&&self.frame.terminal_is_empty()&&self.rows.is_none()&&self.retirement.is_none()}}
impl Drop for FrameInverse{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"frame inverse needs original funded cleanup")}}
