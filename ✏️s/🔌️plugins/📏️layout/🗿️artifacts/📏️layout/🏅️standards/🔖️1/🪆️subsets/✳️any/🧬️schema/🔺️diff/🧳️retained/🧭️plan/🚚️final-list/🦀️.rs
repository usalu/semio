//! 🚚️ Final nested membership moves validated original rows and selected authored rows once.
use crate::{chained_plan::Address,keyed_plan::Origin,list_move::TypedListMove};
use semio_s_artifact_layout_layout_model::{Frame,Layer,PageOverride,TextStyleRun,LayoutRect,LayoutBounds};
use semio_os_kernel::{ApplyCapability,RetainedFlight};
use crate::page_cache::close_owned;
use semio_framework_value::{OriginalRetirementCloseLoan,OriginalRetirementPoll,RetainedAllocationEffect,RetirementPollExit,RetirementDemand,ValueError,ValueRefusalKind,retirement::RetireOwned,retained_clone::{RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use std::mem::{size_of,ManuallyDrop};
/// 🕳 Allocation-free holes retain only metadata after an authored row transfers its children.
pub trait VacantRow:RetireOwned{fn vacant()->Self;}
impl VacantRow for Frame{fn vacant()->Self{Frame::Rect{id:String::new(),layer_id:String::new(),bounds:LayoutBounds{x:0.0,y:0.0,width:0.0,height:0.0,rotation:0.0},locked:None,visible:None,fill:None,stroke:None}}}
impl VacantRow for Layer{fn vacant()->Self{Self{id:String::new(),name:String::new(),visible:false,locked:false,object_ids:Vec::new()}}}
impl VacantRow for PageOverride{fn vacant()->Self{Self{object_id:String::new(),bounds:None,visible:None,locked:None}}}
impl VacantRow for TextStyleRun{fn vacant()->Self{Self{start:0,end:0,paragraph_style_id:None,character_style_id:None}}}
impl VacantRow for LayoutRect{fn vacant()->Self{Self{x:0.0,y:0.0,width:0.0,height:0.0}}}
/// 🧳 Only final surviving authored row headers enter the flat move frontier.
pub struct FinalList<R:VacantRow>{flight:RetainedFlight,mover:TypedListMove<R,R>,authored:ManuallyDrop<Option<Vec<R>>>,origins:Vec<Option<Origin>>,retirement:RetainedCloneClose,index:usize,inserted:usize,held:usize,phase:u8,closing:bool}
impl<R:VacantRow> Default for FinalList<R>{fn default()->Self{Self::new()}}
impl<R:VacantRow> FinalList<R>{
 pub fn new()->Self{Self{flight:Default::default(),mover:TypedListMove::new(),authored:ManuallyDrop::new(Some(Vec::new())),origins:Vec::new(),retirement:Default::default(),index:0,inserted:0,held:0,phase:0,closing:false}}
 fn fault(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
 fn fixed()->RetirementDemand{RetirementDemand{copy_bytes:128,depth:1,..Default::default()}}
 fn nested(mut d:RetirementDemand)->RetirementDemand{d.copy_bytes+=128;d.depth+=1;d}
 fn child(g:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-128,maximum_depth:g.maximum_depth-1,..g}}
 fn receipt(mut p:RetainedCloneProgress)->RetainedCloneProgress{p.copied_bytes+=128;p.copied_items=p.copied_items.max(1);p}
 fn refused(error:ValueError)->ValueError{let p=Self::receipt(error.retained_progress());error.with_retained_progress(p)}
 pub fn preflight_demand(&self,held:usize,rows:&[Address])->Result<RetirementDemand,ValueError>{if self.closing||self.phase>4{return Err(Self::fault("final nested list preflight settled"))}match self.phase{0|2|4=>Ok(Self::fixed()),1=>Ok(RetirementDemand{capacity_bytes:self.inserted.checked_mul(size_of::<R>()).and_then(|n|rows.len().checked_mul(size_of::<Option<Origin>>()).and_then(|m|n.checked_add(m))).ok_or_else(||Self::fault("final nested header extent overflow"))?,..Self::fixed()}),3=>self.mover.reserve_demand(held,self.inserted,rows.len()).map(Self::nested),_=>Err(Self::fault("final nested preflight phase"))}}
 pub fn preflight_work_demand(&self,held:usize,rows:&[Address])->Result<u64,ValueError>{self.preflight_demand(held,rows)?;Ok(if self.phase==3{2}else{1})}
 pub fn preflight(&mut self,held:usize,rows:&[Address],effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>{let g=effect.grant();let d=self.preflight_demand(held,rows)?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(g,d)||effect.remaining_work_units()?<self.preflight_work_demand(held,rows)?{return Ok(RetainedCloneStep::Progress(Default::default()))}if !effect.performed(1)?{return Err(Self::fault("final nested list exceeded admitted work"))}let mut p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};match self.phase{
  0=>if self.index==rows.len(){self.phase=1;self.index=0;self.held=held}else{if rows[self.index].step.is_some(){self.inserted+=1}self.index+=1},
  1=>{let rows_out=self.authored.as_mut().unwrap();let reserved=rows_out.try_reserve_exact(self.inserted).and_then(|_|self.origins.try_reserve_exact(rows.len()));p.retained_capacity_bytes=rows_out.capacity()*size_of::<R>()+self.origins.capacity()*size_of::<Option<Origin>>();if reserved.is_err()||p.retained_capacity_bytes>g.maximum_capacity_bytes{return Err(Self::fault("final nested header reservation failed").with_retained_progress(p))}self.phase=2;self.inserted=0},
  2=>if self.index==rows.len(){self.phase=3;self.index=0}else{let at=rows[self.index];self.origins.push(Some(if at.step.is_some(){let index=self.inserted;self.inserted+=1;Origin{inserted:true,index}}else{Origin{inserted:false,index:at.row}}));self.index+=1},
  3=>{p=Self::receipt(self.mover.reserve_effect(held,self.inserted,rows.len(),&mut effect.nested(Self::child(g))).map_err(Self::refused)?.progress());self.phase=4;return Ok(RetainedCloneStep::Complete(p))},4=>return Ok(RetainedCloneStep::Complete(p)),_=>unreachable!()
 }Ok(RetainedCloneStep::Progress(p))}
 pub fn commit_demand(&self)->Result<RetirementDemand,ValueError>{if self.closing||self.phase<4{return Err(Self::fault("final nested membership lacks admission"))}if self.phase==4{Ok(RetirementDemand{copy_bytes:size_of::<R>()*2+128,depth:1,..Default::default()})}else if self.phase==7{Ok(Self::fixed())}else{self.mover.commit_demand().map(Self::nested)}}
 pub fn commit(&mut self,post:&mut Vec<R>,rows:&[Address],g:RetainedCloneGrant,capability:ApplyCapability,take:impl FnOnce(Address)->R)->Result<RetainedCloneStep,ValueError>{let d=self.commit_demand()?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}let p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};if self.phase==4{if self.index==rows.len(){self.phase=5}else{let at=rows[self.index];if at.step.is_some(){self.authored.as_mut().expect("reserved authored final rows").push(take(at))}self.index+=1}return Ok(RetainedCloneStep::Progress(p))}if self.phase==5{let step=self.mover.capture(post,&mut self.authored,Self::child(g),capability).map_err(Self::refused)?;self.phase=6;return Ok(RetainedCloneStep::Progress(Self::receipt(step.progress())))}if self.phase==7{return Ok(RetainedCloneStep::Complete(p))}let step=self.mover.advance(post,&self.origins,Self::child(g),capability).map_err(Self::refused)?;if matches!(step,RetainedCloneStep::Complete(_)){self.phase=7;Ok(RetainedCloneStep::Complete(Self::receipt(step.progress())))}else{Ok(RetainedCloneStep::Progress(Self::receipt(step.progress())))}}
 pub fn begin_close(&mut self){self.closing=true;self.mover.begin_close()}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.mover.terminal_is_empty(){return self.mover.close_demand(body)}if !self.retirement.is_empty()||self.authored.is_some(){return self.retirement.next_owner_demands::<Vec<R>>(self.authored.is_some(),&None,body)}Ok(RetirementDemand{release_bytes:if self.origins.is_empty(){self.origins.capacity()*size_of::<Option<Origin>>()}else{0},..Self::fixed()})}
 pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{
  if !self.mover.terminal_is_empty(){return self.mover.close_work_demand(body)}
  if !self.retirement.is_empty()||self.authored.is_some(){return self.retirement.next_owner_work::<Vec<R>>(self.authored.is_some(),&None,body)}
  Ok(u64::from(!self.terminal_is_empty()))
 }
 pub fn requires_original_birth(&self)->bool{
  if !self.mover.terminal_is_empty(){return self.mover.requires_original_birth()}
  self.flight.idle()&&self.retirement.requires_original_birth()
 }
 pub fn requires_original_control(&self)->bool{
  if !self.mover.terminal_is_empty(){return self.mover.requires_original_control()}
  self.flight.loaned().unwrap_or_else(||self.retirement.requires_original_control())
 }
 pub fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{
  if !self.mover.terminal_is_empty(){return self.mover.close_original_birth_step(original,grant)}
  self.retirement.step_original_birth(original,grant)
 }
 pub fn close_step(&mut self,original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,recover:bool,work:&mut u128)->Result<RetainedCloneStep,RetirementPollExit>{
  if !self.closing{return Err(Self::fault("final nested original close not begun").into())}
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
  let d=if recover{Default::default()}else{let d=self.close_demand(body)?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(effect.grant(),d)||effect.remaining_work_units()?<self.close_work_demand(body)?{return Ok(RetainedCloneStep::Progress(Default::default()))}d};
  if !self.mover.terminal_is_empty(){return self.mover.close_step(original,body,effect,recover,work).map(|p|RetainedCloneStep::Progress(p.progress()))}
  if !self.retirement.is_empty()||self.authored.is_some(){return close_owned(&mut self.retirement,&mut self.authored,&mut self.flight,original,body,effect,recover,work).map(|p|RetainedCloneStep::Progress(p.progress()))}
  if !effect.performed(1)?{return Err(Self::fault("final nested list close exceeded admitted work").into())}
  *work+=1;
  let mut p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};
  if self.origins.pop().is_none(){p.released_bytes=d.release_bytes;self.origins=Vec::new()}
  Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.mover.terminal_is_empty()&&self.authored.is_none()&&self.retirement.is_empty()&&self.origins.is_empty()&&self.origins.capacity()==0&&self.flight.idle()}
}
impl<R:VacantRow> Drop for FinalList<R>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"final nested row originals need funded close")}}
