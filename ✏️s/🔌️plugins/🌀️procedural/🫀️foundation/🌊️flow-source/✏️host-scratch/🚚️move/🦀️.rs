//! 🚚️ Relative canvas moves validate the complete original target set before changing scratch layout.
use super::{fixed,invalid};
use super::super::retained_scratch_scan::ScratchRowScan;
use semio_framework_artifact_flow_flow::{FlowHostSnapshot,WidgetLayout,widget_id_for};
use semio_os_flow::{FlowTextCopyWork,LookupCursor,OrderedMap,OriginalOwnedEntry};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError};
use semio_framework_value::retained_clone::{RetainedCloneClose,retained_clone_grant_admits};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use std::mem::{ManuallyDrop,size_of};
struct Fields{ids:Option<Vec<String>>,lookup:Option<LookupCursor<WidgetLayout>>,lookup_close:Option<ControlledRetirement<LookupCursor<WidgetLayout>>>,text:FlowTextCopyWork,entry:Option<OriginalOwnedEntry<WidgetLayout>>,pending:Option<OrderedMap<WidgetLayout>>}
pub(super) struct MoveNodesWork{fields:ManuallyDrop<Fields>,retirement:RetainedCloneClose,scan:ScratchRowScan,dx:f64,dy:f64,target:usize,layout:Option<WidgetLayout>,phase:u8,writing:bool,found:bool,ready:bool,applied:bool,closing:bool,progress:RetainedCloneProgress,work:u64}
fn target_id(value:&String)->&str{value.as_str()}
impl MoveNodesWork{
 pub(super) fn new(ids:Vec<String>,dx:f64,dy:f64)->Self{Self{fields:ManuallyDrop::new(Fields{ids:Some(ids),lookup:None,lookup_close:None,text:FlowTextCopyWork::new(),entry:None,pending:None}),retirement:Default::default(),scan:ScratchRowScan::new(),dx,dy,target:0,layout:None,phase:0,writing:false,found:false,ready:false,applied:false,closing:false,progress:Default::default(),work:0}}
 pub(super) fn ready(&self)->bool{self.ready}
 pub(super) fn applied(&self)->bool{self.applied}
 pub(super) fn progress(&self)->RetainedCloneProgress{self.progress}
 pub(super) fn retiring(&self)->bool{self.fields.lookup_close.is_some()||self.phase==14||self.fields.entry.as_ref().is_some_and(OriginalOwnedEntry::advancing_retirement)}
 fn finish(&mut self,applied:bool){self.ready=true;self.applied=applied}
 pub(super) fn next_demands(&self,source:&FlowHostSnapshot,copy:usize)->Result<RetirementDemand,ValueError>{
  if self.closing{return Err(invalid("canvas move is closing"))}if self.ready{return Ok(Default::default())}
  if let Some(close)=self.fields.lookup_close.as_ref(){return if close.terminal_is_empty(){Ok(fixed(0))}else{close.selected_demand(copy)}}
  if self.phase==14{
   if let Some(entry)=self.fields.entry.as_ref(){if !entry.terminal_is_empty(){return entry.next_close_demands(copy)}}
   return self.fields.text.close_demands(copy)
  }
  let ids=self.fields.ids.as_ref().ok_or_else(||invalid("canvas move targets are absent"))?;
  Ok(match self.phase{
   0=>fixed(2*size_of::<f64>()),
   1=>if self.target==ids.len()||ids[self.target].is_empty(){fixed(0)}else{return self.scan.next_demands(ids,&ids[self.target],target_id)},
   2=>return self.scan.next_demands(&source.widgets,&ids[self.target],widget_id_for),
   3=>source.layout.lookup_borrowed_constructor_demands(),
   4=>return self.fields.lookup.as_ref().ok_or_else(||invalid("canvas layout lookup is absent"))?.next_borrowed_original_demands(&ids[self.target]),
   5=>fixed(size_of::<WidgetLayout>()+2*size_of::<f64>()),
   9=>return self.fields.text.next_demands(&ids[self.target],copy),
   10=>return self.fields.text.take_demands(),
   11=>return self.fields.entry.as_ref().ok_or_else(||invalid("canvas map edit is absent"))?.next_demands(copy),
   12=>fixed(size_of::<OrderedMap<WidgetLayout>>()),
   _=>fixed(0),
  })
 }
 pub(super) fn advance(&mut self,source:&mut FlowHostSnapshot,grant:RetainedCloneGrant)->RetainedWorkOutcome<bool>{
  self.progress=Default::default();self.work=0;let result=self.advance_original(source,grant).map_err(|error|{if self.progress==Default::default(){self.progress=error.retained_progress()}error.with_retained_progress(self.progress)});RetainedWorkOutcome{result,work:self.work}
 }
 fn next_target(&mut self){
  self.target+=1;self.scan=ScratchRowScan::new();self.layout=None;
  if self.target==self.fields.ids.as_ref().unwrap().len(){if self.writing{self.finish(true)}else{self.phase=8}}else{self.phase=2}
 }
 fn advance_original(&mut self,source:&mut FlowHostSnapshot,grant:RetainedCloneGrant)->Result<bool,ValueError>{
  let demand=self.next_demands(source,grant.maximum_copy_bytes)?;if self.ready{return Ok(true)}if !retained_clone_grant_admits(grant,demand){return Ok(false)}
  if let Some(close)=self.fields.lookup_close.as_mut(){
   if close.terminal_is_empty(){self.fields.lookup_close=None;self.phase=7;self.work=1;self.progress.copied_items=1}
   else{let turn=close.step(grant);self.work=close.last_turn_work();self.progress=close.step_progress();turn?}return Ok(false)
  }
  self.work=1;self.progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
  match self.phase{
   0=>{if self.fields.ids.as_ref().unwrap().is_empty()||!(self.dx.is_finite()&self.dy.is_finite()){self.finish(false)}else{self.phase=1}},
   1=>{
    let ids=self.fields.ids.as_ref().unwrap();
    if self.target==ids.len(){self.target=0;self.scan=ScratchRowScan::new();self.phase=2}
    else if ids[self.target].is_empty(){self.finish(false)}
    else{let turn=self.scan.advance(ids,&ids[self.target],target_id,grant);self.work=turn.work;self.progress=self.scan.step_progress();if turn.result?{if self.scan.result()!=Some(self.target){self.finish(false)}else{self.target+=1;self.scan=ScratchRowScan::new()}}}
   },
   2=>{
    let turn=self.scan.advance(&source.widgets,&self.fields.ids.as_ref().unwrap()[self.target],widget_id_for,grant);self.work=turn.work;self.progress=self.scan.step_progress();
    if turn.result?{if self.scan.result().is_none(){self.next_target()}else{self.phase=3}}
   },
   3=>{let turn=source.layout.admit_lookup_borrowed_original(&self.fields.ids.as_ref().unwrap()[self.target],grant);self.work=turn.work;self.progress=Default::default();let(lookup,progress)=turn.result?;self.fields.lookup=Some(lookup);self.progress=progress;self.phase=4},
   4=>{let lookup=self.fields.lookup.as_mut().unwrap();let turn=lookup.advance_borrowed_original(&self.fields.ids.as_ref().unwrap()[self.target],grant);self.work=turn.work;self.progress=turn.result?.progress();if lookup.is_complete(){self.phase=5}},
   5=>{self.layout=self.fields.lookup.as_ref().unwrap().result().map(|layout|WidgetLayout{x:layout.x+self.dx,y:layout.y+self.dy});self.phase=6},
   6=>{self.fields.lookup_close=Some(ControlledRetirement::new(self.fields.lookup.take().unwrap()).unwrap_or_else(|_|unreachable!("layout lookup declares controlled retirement")))},
   7=>{
    if let Some(layout)=self.layout.as_ref(){if !(layout.x.is_finite()&layout.y.is_finite()){self.finish(false)}else if self.writing{self.phase=9}else{self.found=true;self.next_target()}}
    else{self.next_target()}
   },
   8=>{if !self.found||(self.dx==0.0&&self.dy==0.0){self.finish(false)}else{self.writing=true;self.target=0;self.scan=ScratchRowScan::new();self.phase=2}},
   9=>{let turn=self.fields.text.advance_original(&self.fields.ids.as_ref().unwrap()[self.target],grant);self.work=turn.work;self.progress=Default::default();self.progress=turn.result?.progress();if self.fields.text.ready(){self.phase=10}},
   10=>{let turn=self.fields.text.take_original(grant);self.work=turn.work;self.progress=Default::default();if let Some((key,progress))=turn.result?{self.progress=progress;self.fields.entry=Some(OriginalOwnedEntry::new(std::mem::take(&mut source.layout),key,self.layout.take().unwrap()));self.phase=11}},
   11=>{let entry=self.fields.entry.as_mut().unwrap();let turn=entry.advance(grant);self.work=turn.work;self.progress=entry.step_progress();if let Some(map)=turn.result?{self.fields.pending=Some(map);self.phase=12}},
   12=>{source.layout=self.fields.pending.take().unwrap();self.phase=13},
   13=>{self.fields.entry.as_mut().unwrap().begin_close();self.fields.text.begin_close();self.phase=14},
   14=>{
    if let Some(entry)=self.fields.entry.as_mut(){if entry.terminal_is_empty(){self.fields.entry=None}else{let turn=entry.close_step(grant);self.work=turn.work;self.progress=turn.result?.progress();return Ok(false)}}
    if !self.fields.text.terminal_is_empty(){let turn=self.fields.text.close_step(grant);self.work=turn.work;self.progress=Default::default();self.progress=turn.result?.progress()}
    else{self.fields.text=FlowTextCopyWork::new();self.next_target()}
   },
   _=>return Err(invalid("canvas move phase is invalid")),
  }Ok(self.ready)
 }
 pub(super) fn begin_close(&mut self){self.closing=true;self.fields.text.begin_close();if let Some(entry)=self.fields.entry.as_mut(){entry.begin_close()}}
 pub(super) fn close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if !self.retirement.is_empty(){return self.retirement.demands(copy)}
  if let Some(close)=self.fields.lookup_close.as_ref(){return if close.terminal_is_empty(){Ok(fixed(0))}else{close.selected_demand(copy)}}
  if self.fields.lookup.is_some(){return Ok(fixed(0))}
  if let Some(entry)=self.fields.entry.as_ref(){return if entry.terminal_is_empty(){Ok(fixed(0))}else{entry.next_close_demands(copy)}}
  if !self.fields.text.terminal_is_empty(){return self.fields.text.close_demands(copy)}
  if self.fields.pending.is_some(){return self.retirement.next_owner_demands::<OrderedMap<WidgetLayout>>(true,&None,copy)}
  if self.fields.ids.is_some(){return self.retirement.next_owner_demands::<Vec<String>>(true,&None,copy)}Ok(Default::default())
 }
 pub(super) fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  self.progress=Default::default();self.work=0;let result=self.close_original(grant).map_err(|error|{if self.progress==Default::default(){self.progress=error.retained_progress()}error.with_retained_progress(self.progress)});RetainedWorkOutcome{result,work:self.work}
 }
 fn close_original(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if !self.closing{return Err(invalid("canvas move close was not begun"))}if self.terminal(){return Ok(RetainedCloneStep::Complete(Default::default()))}
  if !retained_clone_grant_admits(grant,self.close_demands(grant.maximum_copy_bytes)?){return Ok(RetainedCloneStep::Progress(Default::default()))}
  if !self.retirement.is_empty(){let turn=self.retirement.step_granted(grant);self.work=turn.work;self.progress=turn.result?.progress()}
  else if let Some(close)=self.fields.lookup_close.as_mut(){if close.terminal_is_empty(){self.fields.lookup_close=None;self.work=1;self.progress.copied_items=1}else{let turn=close.step(grant);self.work=close.last_turn_work();self.progress=close.step_progress();turn?}}
  else if self.fields.lookup.is_some(){self.fields.lookup_close=Some(ControlledRetirement::new(self.fields.lookup.take().unwrap()).unwrap_or_else(|_|unreachable!("layout lookup declares controlled retirement")));self.work=1;self.progress.copied_items=1}
  else if let Some(entry)=self.fields.entry.as_mut(){if entry.terminal_is_empty(){self.fields.entry=None;self.work=1;self.progress.copied_items=1}else{let turn=entry.close_step(grant);self.work=turn.work;self.progress=turn.result?.progress()}}
  else if !self.fields.text.terminal_is_empty(){let turn=self.fields.text.close_step(grant);self.work=turn.work;self.progress=turn.result?.progress()}
  else{let turn=if self.fields.pending.is_some(){self.retirement.begin_granted(&mut self.fields.pending,grant)}else{self.retirement.begin_granted(&mut self.fields.ids,grant)};self.work=turn.work;self.progress=turn.result?.map_or(Default::default(),|step|step.progress())}
  Ok(RetainedCloneStep::Progress(self.progress))
 }
 pub(super) fn terminal(&self)->bool{self.closing&&self.retirement.is_empty()&&self.fields.ids.is_none()&&self.fields.lookup.is_none()&&self.fields.lookup_close.is_none()&&self.fields.entry.is_none()&&self.fields.pending.is_none()&&self.fields.text.terminal_is_empty()}
}
impl Drop for MoveNodesWork{fn drop(&mut self){assert!(self.terminal(),"relative canvas move retains original targets or map owners");if self.terminal(){unsafe{ManuallyDrop::drop(&mut self.fields)}}}}
