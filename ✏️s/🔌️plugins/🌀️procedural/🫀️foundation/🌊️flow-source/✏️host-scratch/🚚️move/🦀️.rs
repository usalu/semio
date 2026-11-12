//! 🚚️ Relative canvas moves validate the complete original target set before changing scratch layout.
use super::{fixed,invalid};
use super::super::retained_scratch_scan::ScratchRowScan;
use super::super::retained_scratch_lookup::ScratchMapLookup;
use super::super::retained_scratch_driver::{begun,local_progress,pay,replied,settled_step};
use semio_framework_artifact_flow_flow::{FlowHostSnapshot,WidgetLayout,widget_id_for};
use semio_os_flow::{FlowTextCopyWork,OrderedMap,OriginalOwnedEntry};
use semio_framework_value::{OriginalRetirementFailure,OriginalRetirementPoll,RetainedAllocationEffect,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,RetirementPollExit,ValueError};
use semio_framework_value::retained_clone::{RetainedCloneClose,retained_clone_grant_admits};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use std::mem::{ManuallyDrop,size_of};
struct Fields{ids:Option<Vec<String>>,lookup:Option<ScratchMapLookup<WidgetLayout>>,text:FlowTextCopyWork,entry:Option<OriginalOwnedEntry<WidgetLayout>>,entry_close:Option<ControlledRetirement<OriginalOwnedEntry<WidgetLayout>>>,pending:Option<OrderedMap<WidgetLayout>>}
pub(super) struct MoveNodesWork{fields:ManuallyDrop<Fields>,retirement:RetainedCloneClose,scan:ScratchRowScan,dx:f64,dy:f64,target:usize,layout:Option<WidgetLayout>,phase:u8,writing:bool,found:bool,ready:bool,applied:bool,closing:bool,recover:bool,progress:RetainedCloneProgress}
fn target_id(value:&String)->&str{value.as_str()}
impl MoveNodesWork{
 pub(super) fn new(ids:Vec<String>,dx:f64,dy:f64)->Self{Self{fields:ManuallyDrop::new(Fields{ids:Some(ids),lookup:None,text:FlowTextCopyWork::new(),entry:None,entry_close:None,pending:None}),retirement:Default::default(),scan:ScratchRowScan::new(),dx,dy,target:0,layout:None,phase:0,writing:false,found:false,ready:false,applied:false,closing:false,recover:false,progress:Default::default()}}
 pub(super) fn ready(&self)->bool{self.ready}
 pub(super) fn applied(&self)->bool{self.applied}
 pub(super) fn progress(&self)->RetainedCloneProgress{self.progress}
 pub(super) fn retiring(&self)->bool{self.phase==6||self.phase==14||self.fields.entry.as_ref().is_some_and(OriginalOwnedEntry::advancing_retirement)}
 fn finish(&mut self,applied:bool){self.ready=true;self.applied=applied}
 pub(super) fn next_demands(&self,source:&FlowHostSnapshot,copy:usize)->Result<RetirementDemand,ValueError>{
  if self.closing{return Err(invalid("canvas move is closing"))}if self.ready{return Ok(Default::default())}
  if self.phase==14{
   if let Some(close)=self.fields.entry_close.as_ref(){if !close.terminal_is_empty(){return close.selected_demand(copy)}}
   return self.fields.text.close_demands(copy)
  }
  let ids=self.fields.ids.as_ref().ok_or_else(||invalid("canvas move targets are absent"))?;
  Ok(match self.phase{
   0=>fixed(2*size_of::<f64>()),
   1=>if self.target==ids.len()||ids[self.target].is_empty(){fixed(0)}else{return self.scan.next_demands(ids,&ids[self.target],target_id)},
   2=>return self.scan.next_demands(&source.widgets,&ids[self.target],widget_id_for),
   3|4=>return self.fields.lookup.as_ref().ok_or_else(||invalid("canvas layout lookup is absent"))?.next_demands(&source.layout,&ids[self.target],copy),
   5=>fixed(size_of::<WidgetLayout>()+2*size_of::<f64>()),
   6=>return self.fields.lookup.as_ref().ok_or_else(||invalid("canvas layout lookup is absent"))?.close_demands(copy),
   9=>return self.fields.text.next_demands(&ids[self.target],copy),
   10=>return self.fields.text.take_demands(),
   11=>return self.fields.entry.as_ref().ok_or_else(||invalid("canvas map edit is absent"))?.next_demands(copy),
   12=>fixed(size_of::<OrderedMap<WidgetLayout>>()),
   _=>fixed(0),
  })
 }
 pub(super) fn advance(&mut self,source:&mut FlowHostSnapshot,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<bool>{
  self.progress=Default::default();let result=self.advance_original(source,effect).map_err(|error|{if self.progress==Default::default(){self.progress=error.retained_progress()}error.with_retained_progress(self.progress)});RetainedWorkOutcome{result,work:effect.performed_work()}
 }
 fn next_target(&mut self){
  self.target+=1;self.scan=ScratchRowScan::new();self.layout=None;
  if self.target==self.fields.ids.as_ref().unwrap().len(){if self.writing{self.finish(true)}else{self.phase=8}}else{self.phase=2}
 }
 fn advance_original(&mut self,source:&mut FlowHostSnapshot,effect:&mut RetainedAllocationEffect<'_>)->Result<bool,ValueError>{
  let grant=effect.grant();let demand=self.next_demands(source,grant.maximum_copy_bytes)?;if self.ready{return Ok(true)}if !retained_clone_grant_admits(grant,demand){return Ok(false)}
  if matches!(self.phase,0|5|7|8|12|13){if !pay(effect,1)?{return Ok(false)}self.progress=local_progress(demand)}
  match self.phase{
   0=>{if self.fields.ids.as_ref().unwrap().is_empty()||!(self.dx.is_finite()&self.dy.is_finite()){self.finish(false)}else{self.phase=1}},
   1=>{
    let ids=self.fields.ids.as_ref().unwrap();
    if self.target==ids.len(){if !pay(effect,1)?{return Ok(false)}self.progress=local_progress(demand);self.target=0;self.scan=ScratchRowScan::new();self.phase=2}
    else if ids[self.target].is_empty(){if !pay(effect,1)?{return Ok(false)}self.progress=local_progress(demand);self.finish(false)}
    else{let turn=self.scan.advance(ids,&ids[self.target],target_id,effect);self.progress=self.scan.step_progress();if turn.result?{if self.scan.result()!=Some(self.target){self.finish(false)}else{self.target+=1;self.scan=ScratchRowScan::new()}}}
   },
   2=>{
    let turn=self.scan.advance(&source.widgets,&self.fields.ids.as_ref().unwrap()[self.target],widget_id_for,effect);self.progress=self.scan.step_progress();
    if turn.result?{if self.scan.result().is_none(){self.next_target()}else{self.fields.lookup=Some(ScratchMapLookup::new());self.phase=3}}
   },
   3|4=>{
    let Fields{lookup,ids,..}=&mut*self.fields;let lookup=lookup.as_mut().unwrap();
    let complete=lookup.advance(&source.layout,&ids.as_ref().unwrap()[self.target],effect)?;self.progress=lookup.step_progress();if complete{self.phase=5}
   },
   5=>{self.layout=self.fields.lookup.as_ref().unwrap().result().map(|layout|WidgetLayout{x:layout.x+self.dx,y:layout.y+self.dy});self.phase=6},
   6=>{
    let lookup=self.fields.lookup.as_mut().unwrap();lookup.begin_close();
    let step=settled_step(lookup.close_step(grant.maximum_copy_bytes,effect))?;self.progress=step.progress();
    if lookup.terminal_is_empty(){self.fields.lookup=None;self.phase=7}
   },
   7=>{
    if let Some(layout)=self.layout.as_ref(){if !(layout.x.is_finite()&layout.y.is_finite()){self.finish(false)}else if self.writing{self.phase=9}else{self.found=true;self.next_target()}}
    else{self.next_target()}
   },
   8=>{if !self.found||(self.dx==0.0&&self.dy==0.0){self.finish(false)}else{self.writing=true;self.target=0;self.scan=ScratchRowScan::new();self.phase=2}},
   9=>{let Fields{text,ids,..}=&mut*self.fields;self.progress=text.advance_original(&ids.as_ref().unwrap()[self.target],effect).result?.progress();if self.fields.text.ready(){self.phase=10}},
   10=>{if let Some((key,progress))=self.fields.text.take_original(effect).result?{self.progress=progress;self.fields.entry=Some(OriginalOwnedEntry::new(std::mem::take(&mut source.layout),key,self.layout.take().unwrap()));self.phase=11}},
   11=>{
    let Fields{entry,pending,..}=&mut*self.fields;let entry=entry.as_mut().unwrap();let body=grant.maximum_copy_bytes;let mut output=None;
    let reply=if self.recover{replied(effect,|effect,reply|entry.recover_advance_into(body,&mut output,effect,reply))}else{replied(effect,|effect,reply|entry.advance_into(body,&mut output,effect,reply))}?;
    match reply{
     Some(result)=>{self.recover=false;let step=result.map_err(OriginalRetirementFailure::into_value_error)?;self.progress=step.progress();if let Some(map)=output{*pending=Some(map);self.phase=12}},
     None=>self.recover=true,
    }
   },
   12=>{source.layout=self.fields.pending.take().unwrap();self.phase=13},
   13=>{let entry=self.fields.entry.take().unwrap();self.fields.entry_close=Some(ControlledRetirement::new(entry).unwrap_or_else(|_|unreachable!("canvas map edit declares controlled retirement")));self.fields.text.begin_close();self.phase=14},
   14=>{
    if let Some(close)=self.fields.entry_close.as_mut(){if close.terminal_is_empty(){self.fields.entry_close=None}else{let step=settled_step(close.step(grant.maximum_copy_bytes,effect))?;self.progress=step.progress();return Ok(false)}}
    if !self.fields.text.terminal_is_empty(){let step=settled_step(self.fields.text.close_step(grant.maximum_copy_bytes,effect))?;self.progress=step.progress()}
    else{if !pay(effect,1)?{return Ok(false)}self.progress=local_progress(demand);self.fields.text=FlowTextCopyWork::new();self.next_target()}
   },
   _=>return Err(invalid("canvas move phase is invalid")),
  }Ok(self.ready)
 }
 pub(super) fn begin_close(&mut self){self.closing=true;self.fields.text.begin_close();if let Some(lookup)=self.fields.lookup.as_mut(){lookup.begin_close()}}
 pub(super) fn close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if !self.retirement.is_empty(){return self.retirement.demands(copy)}
  if let Some(lookup)=self.fields.lookup.as_ref(){return lookup.close_demands(copy)}
  if let Some(close)=self.fields.entry_close.as_ref(){return close.selected_demand(copy)}
  if self.fields.entry.is_some(){return Ok(fixed(0))}
  if !self.fields.text.terminal_is_empty(){return self.fields.text.close_demands(copy)}
  if self.fields.pending.is_some(){return self.retirement.next_owner_demands::<OrderedMap<WidgetLayout>>(true,&None,copy)}
  if self.fields.ids.is_some(){return self.retirement.next_owner_demands::<Vec<String>>(true,&None,copy)}Ok(Default::default())
 }
 pub(super) fn close_step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
  self.progress=Default::default();let result=self.close_original(body,effect);RetirementPollExit::settle(effect,result)
 }
 fn close_original(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,RetirementPollExit>{
  if !self.closing{return Err(invalid("canvas move close was not begun").into())}if self.terminal(){return Ok(RetainedCloneStep::Complete(Default::default()))}
  if !retained_clone_grant_admits(effect.grant(),self.close_demands(body)?){return Ok(RetainedCloneStep::Progress(Default::default()))}
  let step=if !self.retirement.is_empty(){RetirementPollExit::resume(self.retirement.step_granted(body,effect))?}
  else if let Some(lookup)=self.fields.lookup.as_mut(){if lookup.terminal_is_empty(){self.fields.lookup=None;RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})}else{RetirementPollExit::resume(lookup.close_step(body,effect))?}}
  else if let Some(close)=self.fields.entry_close.as_mut(){let step=RetirementPollExit::resume(close.step(body,effect))?;if close.terminal_is_empty(){self.fields.entry_close=None}step}
  else if let Some(entry)=self.fields.entry.take(){self.fields.entry_close=Some(ControlledRetirement::new(entry).unwrap_or_else(|_|unreachable!("canvas map edit declares controlled retirement")));RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})}
  else if !self.fields.text.terminal_is_empty(){RetirementPollExit::resume(self.fields.text.close_step(body,effect))?}
  else if self.fields.pending.is_some(){begun(&mut self.retirement,&mut self.fields.pending,effect)?}
  else{begun(&mut self.retirement,&mut self.fields.ids,effect)?};
  self.progress=step.progress();Ok(RetainedCloneStep::Progress(self.progress))
 }
 pub(super) fn terminal(&self)->bool{self.closing&&self.retirement.is_empty()&&self.fields.ids.is_none()&&self.fields.lookup.is_none()&&self.fields.entry.is_none()&&self.fields.entry_close.is_none()&&self.fields.pending.is_none()&&self.fields.text.terminal_is_empty()}
}
impl Drop for MoveNodesWork{fn drop(&mut self){assert!(self.terminal(),"relative canvas move retains original targets or map owners");if self.terminal(){unsafe{ManuallyDrop::drop(&mut self.fields)}}}}
