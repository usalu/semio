//! 🔎️ A funded owned-key lookup over one immutable ordered map keeps its copied key, shared key and cursor until its explicit close.
use super::retained_scratch_driver::{begun,local_progress,pay};
use semio_os_flow::{FlowTextCopyWork,LookupCursor,OrderedMap,SharedOwner};
use semio_framework_value::{OriginalRetirementPoll,RetainedAllocationEffect,RetainedCloneProgress,RetainedCloneStep,RetirementDemand,RetirementPollExit,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneClose,retained_clone_grant_admits};
use semio_framework_value::retirement::{RetireOwned,controlled::ControlledRetirement};
use semio_framework_value_ordered::ordered::Grant;
const COMPARE_BYTES:usize=64;
pub struct ScratchMapLookup<V:RetireOwned+Sync>{text:FlowTextCopyWork,copied:Option<String>,key:Option<SharedOwner<String>>,cursor:Option<LookupCursor<V>>,close:Option<ControlledRetirement<LookupCursor<V>>>,retirement:RetainedCloneClose,phase:u8,closing:bool,progress:RetainedCloneProgress}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"scratch map lookup changed its key, map or phase")}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
impl<V:RetireOwned+Sync> ScratchMapLookup<V>{
 pub fn new()->Self{Self{text:FlowTextCopyWork::new(),copied:None,key:None,cursor:None,close:None,retirement:Default::default(),phase:0,closing:false,progress:Default::default()}}
 pub fn ready(&self)->bool{!self.closing&&self.phase==5}
 pub fn result(&self)->Option<&V>{self.cursor.as_ref().filter(|_|self.ready()).and_then(LookupCursor::result)}
 pub fn retiring(&self)->bool{self.close.is_some()}
 pub fn step_progress(&self)->RetainedCloneProgress{self.progress}
 pub fn next_demands(&self,map:&OrderedMap<V>,key:&str,copy:usize)->Result<RetirementDemand,ValueError>{
  if self.closing{return Err(invalid())}
  match self.phase{0=>self.text.next_demands(key,copy),1=>self.text.take_demands(),2=>SharedOwner::<String>::source_admission_demand(),3=>Ok(map.owned_lookup_cursor_demand()),4=>Ok(fixed(0)),_=>Ok(Default::default())}
 }
 pub fn next_work_demand(&self,map:&OrderedMap<V>,key:&str,copy:usize)->Result<u64,ValueError>{
  match self.phase{0=>self.text.next_work_demand(key,copy),1=>self.text.take_work_demand(),2=>Ok(SharedOwner::<String>::source_admission_work_demand()),3=>Ok(map.owned_lookup_cursor_work()),4=>Ok(1),_=>Ok(0)}
 }
 pub fn advance(&mut self,map:&OrderedMap<V>,key:&str,effect:&mut RetainedAllocationEffect<'_>)->Result<bool,ValueError>{
  self.progress=Default::default();let grant=effect.grant();let demand=self.next_demands(map,key,grant.maximum_copy_bytes)?;
  if self.ready(){return Ok(true)}if !retained_clone_grant_admits(grant,demand)||effect.remaining_work_units()?<self.next_work_demand(map,key,grant.maximum_copy_bytes)?{return Ok(false)}
  match self.phase{
   0=>{self.progress=self.text.advance_original(key,effect).result?.progress();if self.text.ready(){self.phase=1}},
   1=>{if let Some((text,progress))=self.text.take_original(effect).result?{self.copied=Some(text);self.progress=progress;self.phase=2}},
   2=>{let step=SharedOwner::admit_from(&mut self.copied,&mut self.key,effect).result?;self.progress=step.progress();if self.key.is_some(){self.phase=3}},
   3=>{let step=map.admit_owned_lookup_cursor(&mut self.key,&mut self.cursor,effect).result?;self.progress=step.progress();if self.cursor.is_some(){self.phase=4}},
   4=>{
    if !pay(effect,1)?{return Ok(false)}self.progress=local_progress(demand);
    let cursor=self.cursor.as_mut().ok_or_else(invalid)?;cursor.advance(Grant{maximum_items:1,maximum_bytes:COMPARE_BYTES});if cursor.is_complete(){self.phase=5}
   },
   _=>return Err(invalid()),
  }Ok(self.ready())
 }
 pub fn begin_close(&mut self){self.closing=true;self.text.begin_close()}
 pub fn close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if !self.retirement.is_empty(){return self.retirement.demands(copy)}
  if !self.text.terminal_is_empty(){return self.text.close_demands(copy)}
  if let Some(close)=self.close.as_ref(){return close.selected_demand(copy)}
  if self.copied.is_some(){return self.retirement.next_owner_demands::<String>(true,&None,copy)}
  if self.key.is_some(){return self.retirement.next_owner_demands::<SharedOwner<String>>(true,&None,copy)}
  Ok(if self.cursor.is_some(){fixed(0)}else{Default::default()})
 }
 pub fn close_step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{self.progress=Default::default();let result=self.close_original(body,effect);RetirementPollExit::settle(effect,result)}
 fn close_original(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,RetirementPollExit>{
  if !self.closing{return Err(invalid().into())}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
  if !retained_clone_grant_admits(effect.grant(),self.close_demands(body)?){return Ok(RetainedCloneStep::Progress(Default::default()))}
  let step=if !self.retirement.is_empty(){RetirementPollExit::resume(self.retirement.step_granted(body,effect))?}
  else if !self.text.terminal_is_empty(){RetirementPollExit::resume(self.text.close_step(body,effect))?}
  else if let Some(close)=self.close.as_mut(){let step=RetirementPollExit::resume(close.step(body,effect))?;if close.terminal_is_empty(){self.close=None}step}
  else if self.copied.is_some(){begun(&mut self.retirement,&mut self.copied,effect)?}
  else if self.key.is_some(){begun(&mut self.retirement,&mut self.key,effect)?}
  else if let Some(cursor)=self.cursor.take(){self.close=Some(ControlledRetirement::new(cursor).unwrap_or_else(|_|unreachable!("layout lookup declares controlled retirement")));RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})}
  else{RetainedCloneStep::Complete(Default::default())};
  self.progress=step.progress();Ok(RetainedCloneStep::Progress(self.progress))
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.text.terminal_is_empty()&&self.copied.is_none()&&self.key.is_none()&&self.cursor.is_none()&&self.close.is_none()&&self.retirement.is_empty()}
}
impl<V:RetireOwned+Sync> Drop for ScratchMapLookup<V>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"scratch map lookup retains its copied key, shared key or cursor")}}
