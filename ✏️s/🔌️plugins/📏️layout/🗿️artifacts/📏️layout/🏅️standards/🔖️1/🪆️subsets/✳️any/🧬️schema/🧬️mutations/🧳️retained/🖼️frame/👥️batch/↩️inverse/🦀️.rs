//! ↩️ Changed frame inverses preserve original absolute scalar values and original identity owners.
use crate::LayoutMutation;
use semio_s_artifact_layout_layout_model::{Frame,snapshot::LayoutSnapshot};
use semio_os_kernel::RetainedMutationCloseContext;
use crate::retained::original_field::{OriginalField,close_rows,rows_work_demand};
use semio_framework_value::{OriginalRetirementCloseLoan,OriginalRetirementPoll,RetainedAllocationEffect,RetainedWorkOutcome,RetainedWorkPoll,RetirementPollExit,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneRef,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits}};
use semio_framework_value_factory::vector::FactoryVectorRetirement;
use super::batch_plan::FrameSelectionPlan;
use std::mem::{size_of,ManuallyDrop};
fn fault()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"batch inverse original custody is invalid")}
fn fixed()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<LayoutMutation>()*4+512,depth:1,..Default::default()}}
fn nested(mut d:RetirementDemand)->RetirementDemand{d.copy_bytes+=fixed().copy_bytes;d.depth+=1;d}
fn child(g:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-fixed().copy_bytes,maximum_depth:g.maximum_depth-1,..g}}
fn receipt(mut p:RetainedCloneProgress)->RetainedCloneProgress{p.copied_bytes+=fixed().copy_bytes;p.copied_items=p.copied_items.max(1);p}
fn refused(e:ValueError)->ValueError{let p=receipt(e.retained_progress());e.with_retained_progress(p)}
fn factor(m:&LayoutMutation)->Result<usize,ValueError>{match m{LayoutMutation::DragFrames(_)=>Ok(1),LayoutMutation::RotateFrames(_)|LayoutMutation::ScaleFrames(_)=>Ok(2),_=>Err(fault())}}
fn page<'a>(b:RetainedCloneRef<'a,LayoutSnapshot>,at:usize)->RetainedCloneRef<'a,String>{b.project(9,|b|&b.pages).project(at,move|rows|&rows[at]).project(0,|p|&p.id)}
fn id<'a>(b:RetainedCloneRef<'a,LayoutSnapshot>,at:usize,frame:usize)->RetainedCloneRef<'a,String>{b.project(9,|b|&b.pages).project(at,move|rows|&rows[at]).project(2,|p|&p.frames).project(frame,move|rows|&rows[frame]).project(0,|f|match f{Frame::Rect{id,..}|Frame::Text{id,..}|Frame::Image{id,..}=>id})}
/// 🧳 Every partial event owns its transferred String before the next original field is visited.
pub struct BatchInverse{page:OriginalField<String>,id:OriginalField<String>,rows:ManuallyDrop<Option<Vec<LayoutMutation>>>,retirement:Option<FactoryVectorRetirement<LayoutMutation>>,phase:u8,index:usize,closing:bool}
impl Default for BatchInverse{fn default()->Self{Self{page:Default::default(),id:Default::default(),rows:ManuallyDrop::new(None),retirement:None,phase:0,index:0,closing:false}}}
impl BatchInverse{
 pub fn demand(&self,plan:&FrameSelectionPlan,m:RetainedCloneRef<'_,LayoutMutation>,b:RetainedCloneRef<'_,LayoutSnapshot>,body:usize,max:u64)->Result<RetirementDemand,ValueError>{plan.verify(m,b)?;if self.closing||!plan.ready()||plan.failure.is_some(){return Err(fault())}let n=factor(m.get())?;let frame=plan.changed().get(self.index/n).map(|row|row.frame);Ok(match self.phase{0=>RetirementDemand{capacity_bytes:plan.changed().len().checked_mul(n).and_then(|n|n.checked_mul(size_of::<LayoutMutation>())).ok_or_else(fault)?,..fixed()},1=>if frame.is_none()||self.page.value().is_some(){fixed()}else{nested(self.page.demand(page(b,plan.page),body.min(256),max.saturating_sub(1))?)},2=>if self.id.value().is_some(){fixed()}else{nested(self.id.demand(id(b,plan.page,frame.ok_or_else(fault)?),body.min(256),max.saturating_sub(1))?)},3=>nested(self.page.exchange_demand()?),4=>nested(self.id.exchange_demand()?),5=>nested(self.page.close_demand(body.min(256))?),6=>nested(self.id.close_demand(body.min(256))?),7|8=>fixed(),_=>return Err(fault())})}
 pub fn embedded_birth_work()->Result<u64,ValueError>{OriginalField::<String>::embedded_birth_work()?.checked_mul(2).ok_or_else(fault)}
 pub fn work_demand(&self,plan:&FrameSelectionPlan,m:RetainedCloneRef<'_,LayoutMutation>,b:RetainedCloneRef<'_,LayoutSnapshot>,body:usize,max:u64)->Result<u64,ValueError>{
  self.demand(plan,m,b,body,max)?;
  let n=factor(m.get())?;let body=body.min(256);let ceiling=max.saturating_sub(1);
  let work=match self.phase{
   1 if self.index<plan.changed().len()*n&&self.page.value().is_none()=>self.page.work_demand(page(b,plan.page),body,ceiling)?,
   2 if self.id.value().is_none()=>self.id.work_demand(id(b,plan.page,plan.changed()[self.index/n].frame),body,ceiling)?,
   3|4=>1,
   5=>self.page.close_work_demand(body)?,
   6=>self.id.close_work_demand(body)?,
   8=>return Ok(0),
   _=>0,
  };
  let work=work.checked_add(1).ok_or_else(fault)?;
  Ok(if work<=max{work}else{0})
 }
 pub fn advance(&mut self,plan:&FrameSelectionPlan,m:RetainedCloneRef<'_,LayoutMutation>,b:RetainedCloneRef<'_,LayoutSnapshot>,body:usize,max:u64,effect:&mut RetainedAllocationEffect<'_>,recover:bool)->Result<RetainedCloneStep,RetirementPollExit>{let g=effect.grant();if self.phase==8{return Ok(RetainedCloneStep::Complete(Default::default()))}let d=if recover{Default::default()}else{let d=self.demand(plan,m,b,body,max)?;let work=self.work_demand(plan,m,b,body,max)?;if !retained_clone_grant_admits(g,d)||work==0||effect.remaining_work_units()?<work{return Ok(RetainedCloneStep::Progress(Default::default()))}if !effect.performed(1)?{return Err(fault().into())}d};let ceiling=max.saturating_sub(1);let body=body.min(256);let mut p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};let n=factor(m.get())?;match self.phase{
 0=>{self.phase=254;*self.rows=Some(Vec::new());let rows=self.rows.as_mut().unwrap();if effect.actual().is_some(){return Err(fault().into())}*effect.actual()=Some(0);let reserved=rows.try_reserve_exact(plan.changed().len()*n);p.retained_capacity_bytes=rows.capacity()*size_of::<LayoutMutation>();*effect.actual()=Some(p.retained_capacity_bytes);if reserved.is_err()||p.retained_capacity_bytes>g.maximum_capacity_bytes{return Err(fault().with_retained_progress(p).into())}self.phase=1},
 1=>if self.index==plan.changed().len()*n{self.phase=8}else if self.page.value().is_some(){self.phase=2}else{p=receipt(self.page.advance(page(b,plan.page),body,ceiling,&mut effect.nested(child(g)),recover)?.progress())},
 2=>if self.id.value().is_some(){self.phase=3}else{let at=plan.changed()[self.index/n].frame;p=receipt(self.id.advance(id(b,plan.page,at),body,ceiling,&mut effect.nested(child(g)),recover)?.progress())},
 3=>{let(value,paid)=self.page.take(&mut effect.nested(child(g))).map_err(refused)?;p=receipt(paid);let page_id=value.ok_or_else(fault)?;let bounds=b.get().pages[plan.page].frames[plan.changed()[self.index/n].frame].bounds();let row=if self.index%n==0{LayoutMutation::MoveFrame(crate::move_frame::MoveFrame{page_id,frame_id:String::new(),new_x:bounds.x,new_y:bounds.y})}else if matches!(m.get(),LayoutMutation::RotateFrames(_)){LayoutMutation::RotateFrame(crate::rotate_frame::RotateFrame{page_id,frame_id:String::new(),new_rotation:bounds.rotation})}else{LayoutMutation::ResizeFrame(crate::resize_frame::ResizeFrame{page_id,frame_id:String::new(),new_width:bounds.width,new_height:bounds.height})};self.rows.as_mut().ok_or_else(fault)?.push(row);self.phase=4},
 4=>{let(value,paid)=self.id.take(&mut effect.nested(child(g))).map_err(refused)?;p=receipt(paid);let id=value.ok_or_else(fault)?;match self.rows.as_mut().ok_or_else(fault)?.last_mut().ok_or_else(fault)?{LayoutMutation::MoveFrame(p)=>p.frame_id=id,LayoutMutation::RotateFrame(p)=>p.frame_id=id,LayoutMutation::ResizeFrame(p)=>p.frame_id=id,_=>return Err(fault().into())}self.page.begin_close();self.id.begin_close();self.phase=5},
 5=>{p=receipt(self.page.close_step(None,body,&mut effect.nested(child(g)),recover)?.progress());if self.page.terminal_is_empty(){self.phase=6}},6=>{p=receipt(self.id.close_step(None,body,&mut effect.nested(child(g)),recover)?.progress());if self.id.terminal_is_empty(){self.phase=7}},7=>{self.page=Default::default();self.id=Default::default();self.index+=1;self.phase=1},8=>{},_=>return Err(fault().into())}Ok(if self.phase==8{RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})}
 pub fn take_demand(&self)->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Option<Vec<LayoutMutation>>>()*2,depth:1,..Default::default()}}
 pub fn take(&mut self,g:RetainedCloneGrant)->Result<(Option<Vec<LayoutMutation>>,RetainedCloneProgress),ValueError>{if self.phase!=8||self.closing{return Err(fault())}let d=self.take_demand();if !retained_clone_grant_admits(g,d){return Ok((None,Default::default()))}self.phase=9;Ok((self.rows.take(),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
 pub fn begin_close(&mut self){self.closing=true;self.page.begin_close();self.id.begin_close()}
 pub fn close_demand(&self,c:RetainedMutationCloseContext<'_,LayoutMutation>,body:usize)->Result<RetirementDemand,ValueError>{if !self.page.terminal_is_empty(){return self.page.close_demand(body)}if !self.id.terminal_is_empty(){return self.id.close_demand(body)}if let Some(owner)=self.retirement.as_ref(){return if owner.terminal_is_empty(){Ok(fixed())}else{owner.demands(c.mutations,body)}}if self.rows.is_some(){return Ok(FactoryVectorRetirement::<LayoutMutation>::constructor_demand())}Ok(fixed())}
 pub fn close_work_demand(&self,c:RetainedMutationCloseContext<'_,LayoutMutation>,body:usize)->Result<u64,ValueError>{
  if !self.page.terminal_is_empty(){return self.page.close_work_demand(body)}
  if !self.id.terminal_is_empty(){return self.id.close_work_demand(body)}
  rows_work_demand(&self.retirement,self.rows.is_some(),c,body)
 }
 pub fn requires_original_birth(&self)->bool{
  if !self.page.terminal_is_empty(){return self.page.requires_original_birth()}
  if !self.id.terminal_is_empty(){return self.id.requires_original_birth()}
  self.retirement.as_ref().is_some_and(FactoryVectorRetirement::requires_original_birth)
 }
 pub fn requires_original_control(&self)->bool{
  if !self.page.terminal_is_empty(){return self.page.requires_original_control()}
  if !self.id.terminal_is_empty(){return self.id.requires_original_control()}
  self.retirement.as_ref().is_some_and(FactoryVectorRetirement::requires_original_control)
 }
 pub fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{
  if !self.page.terminal_is_empty(){return self.page.close_original_birth_step(original,grant)}
  if !self.id.terminal_is_empty(){return self.id.close_original_birth_step(original,grant)}
  match self.retirement.as_mut(){Some(owner)=>owner.close_original_birth_step(original,grant),None=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(fault().into()),work:0})}
 }
 pub fn close_step(&mut self,c:RetainedMutationCloseContext<'_,LayoutMutation>,original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,recover:bool)->Result<RetainedCloneStep,RetirementPollExit>{
  if !self.closing{return Err(fault().into())}
  if !self.page.terminal_is_empty(){return self.page.close_step(original,body,effect,recover)}
  if !self.id.terminal_is_empty(){return self.id.close_step(original,body,effect,recover)}
  close_rows(&mut self.retirement,&mut self.rows,c,original,body,effect)
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.page.terminal_is_empty()&&self.id.terminal_is_empty()&&self.rows.is_none()&&self.retirement.is_none()}
}
impl Drop for BatchInverse{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"batch inverse needs original funded cleanup")}}
