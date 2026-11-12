//! ✏️ Procedural host edits retain native inputs and fund each validation, row move and map event.
#[path="🚚️move/🦀️.rs"]
mod moves;
use super::retained_scratch_rows::{ScratchRowsWork,ScratchRowsKind};
use super::retained_scratch_scan::ScratchRowScan;
use super::retained_scratch_lookup::ScratchMapLookup;
use super::retained_scratch_driver::{begun,local_progress,pay,replied,settled_step};
use semio_framework_artifact_flow_flow::{FlowHostSnapshot,FlowSnapshotSource,FlowSourceIdentity,Widget,SynapseSpec,WidgetLayout,CameraJson,widget_id_for,set_widget_slider_value};
use semio_os_flow::{OriginalOwnedEntry,OrderedMap};
use semio_framework_value::{OriginalRetirementFailure,OriginalRetirementPoll,RetainedAllocationEffect,RetirementDemand,RetirementPollExit,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneClose,retained_clone_grant_admits};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework_job::{StepContext,OriginalTurnFailure};
use std::mem::{ManuallyDrop,size_of};
#[derive(Clone,Copy,PartialEq,Eq)]
pub enum HostScratchDomain{Generation2d,Generation3d}
#[derive(semio_framework_value::FromValue,semio_framework_value::ToValue,semio_framework_value::RetireOwned)]
pub enum HostScratchEdit{
 InsertWidget{index:usize,widget:Widget},ReplaceWidget{widget:Widget},RemoveWidget{id:String},
 InsertSynapse{index:usize,synapse:SynapseSpec},ReplaceSynapse{synapse:SynapseSpec},RemoveSynapse{id:String},
 MoveNodes{ids:Vec<String>,dx:f64,dy:f64},SetLayout{id:String,layout:WidgetLayout},RemoveLayout{id:String},SetCamera{camera:CameraJson},SetSchema{schema:String},SetSlider{id:String,value:f64},
}
enum Rows{Widgets(ScratchRowsWork<Widget>),Synapses(ScratchRowsWork<SynapseSpec>)}
impl Rows{
 fn demands(&self,source:&FlowHostSnapshot,copy:usize)->Result<RetirementDemand,ValueError>{match self{Self::Widgets(work)=>work.next_demands(&source.widgets,copy),Self::Synapses(work)=>work.next_demands(&source.synapses,copy)}}
 fn advance(&mut self,source:&mut FlowHostSnapshot,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<bool>{match self{Self::Widgets(work)=>work.advance(&mut source.widgets,effect),Self::Synapses(work)=>work.advance(&mut source.synapses,effect)}}
 fn retiring(&self)->bool{match self{Self::Widgets(work)=>work.advancing_retirement(),Self::Synapses(work)=>work.advancing_retirement()}}
 fn progress(&self)->RetainedCloneProgress{match self{Self::Widgets(work)=>work.step_progress(),Self::Synapses(work)=>work.step_progress()}}
 fn begin_close(&mut self){match self{Self::Widgets(work)=>work.begin_close(),Self::Synapses(work)=>work.begin_close()}}
 fn close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{match self{Self::Widgets(work)=>work.next_close_demands(copy),Self::Synapses(work)=>work.next_close_demands(copy)}}
 fn close_step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{match self{Self::Widgets(work)=>work.close_step(body,effect),Self::Synapses(work)=>work.close_step(body,effect)}}
 fn terminal(&self)->bool{match self{Self::Widgets(work)=>work.terminal_is_empty(),Self::Synapses(work)=>work.terminal_is_empty()}}
}
struct Fields{moves:Option<moves::MoveNodesWork>,edit:Option<HostScratchEdit>,rows:Option<Rows>,entry:Option<OriginalOwnedEntry<WidgetLayout>>,entry_close:Option<ControlledRetirement<OriginalOwnedEntry<WidgetLayout>>>,lookup:Option<ScratchMapLookup<WidgetLayout>>,pending:Option<OrderedMap<WidgetLayout>>}
pub struct HostScratchWork{fields:ManuallyDrop<Fields>,retirement:RetainedCloneClose,domain:HostScratchDomain,original:semio_framework_job::OriginalSourceBinding,origin:(u64,u64),binding:Option<FlowSourceIdentity>,scan:ScratchRowScan,index:Option<usize>,phase:u8,edge:usize,field:usize,byte:usize,scalar:[u8;4],scalar_len:usize,scalar_need:usize,found:bool,ready:bool,applied:bool,closing:bool,recover:bool,progress:RetainedCloneProgress}
fn invalid(text:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,text)}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn synapse_id(row:&SynapseSpec)->&str{row.id.as_str()}
fn wire_field(row:&SynapseSpec,field:usize)->&str{match field{0=>&row.from,1=>&row.from_port,2=>&row.to,_=>&row.to_port}}
impl HostScratchWork{
 pub fn new(edit:HostScratchEdit,domain:HostScratchDomain,cx:&StepContext<'_>)->Self{Self::new_original(edit,domain,semio_framework_job::OriginalSourceBinding::borrow(cx))}
 pub fn new_original(edit:HostScratchEdit,domain:HostScratchDomain,original:semio_framework_job::OriginalSourceBinding)->Self{Self{fields:ManuallyDrop::new(Fields{moves:None,edit:Some(edit),rows:None,entry:None,entry_close:None,lookup:None,pending:None}),retirement:Default::default(),domain,original,origin:(original.operation().0,original.generation().0),binding:None,scan:ScratchRowScan::new(),index:None,phase:0,edge:0,field:0,byte:0,scalar:[0;4],scalar_len:0,scalar_need:0,found:false,ready:false,applied:false,closing:false,recover:false,progress:Default::default()}}
 pub fn ready(&self)->bool{!self.closing&&self.ready}
 pub fn applied(&self)->bool{self.ready&&self.applied}
 fn finish(&mut self,applied:bool){self.ready=true;self.applied=applied}
 fn validate(&self,source:&FlowHostSnapshot)->Result<(),ValueError>{if self.closing{return Err(invalid("procedural host scratch is closing"))}if self.binding.is_some_and(|binding|binding!=source.source_identity()){return Err(invalid("procedural host scratch changed its actual source"))}Ok(())}
 fn id(&self)->Result<&str,ValueError>{match self.fields.edit.as_ref().ok_or_else(||invalid("procedural host edit is absent"))?{HostScratchEdit::InsertWidget{widget,..}|HostScratchEdit::ReplaceWidget{widget}=>Ok(widget_id_for(widget)),HostScratchEdit::InsertSynapse{synapse,..}|HostScratchEdit::ReplaceSynapse{synapse}=>Ok(&synapse.id),HostScratchEdit::RemoveWidget{id}|HostScratchEdit::RemoveSynapse{id}|HostScratchEdit::SetLayout{id,..}|HostScratchEdit::RemoveLayout{id}|HostScratchEdit::SetSlider{id,..}=>Ok(id),_=>Err(invalid("procedural host edit has no row identity"))}}
 pub fn next_demands(&self,source:&FlowHostSnapshot,copy:usize)->Result<RetirementDemand,ValueError>{
  self.validate(source)?;if self.ready{return Ok(Default::default())}if self.phase==0{return Ok(fixed(0))}
  if let Some(moves)=self.fields.moves.as_ref(){return moves.next_demands(source,copy)}
  if self.fields.pending.is_some(){return Ok(fixed(size_of::<OrderedMap<WidgetLayout>>()))}
  if let Some(rows)=self.fields.rows.as_ref(){return rows.demands(source,copy)}
  if let Some(entry)=self.fields.entry.as_ref(){return entry.next_demands(copy)}
  if let Some(close)=self.fields.entry_close.as_ref(){return if close.terminal_is_empty(){Ok(fixed(0))}else{close.selected_demand(copy)}}
  let edit=self.fields.edit.as_ref().ok_or_else(||invalid("procedural host input is absent"))?;
  match edit{
   HostScratchEdit::InsertWidget{..}|HostScratchEdit::ReplaceWidget{..}|HostScratchEdit::RemoveWidget{..}if self.phase==1=>self.scan.next_demands(&source.widgets,self.id()?,widget_id_for),
   HostScratchEdit::InsertSynapse{..}|HostScratchEdit::ReplaceSynapse{..}|HostScratchEdit::RemoveSynapse{..}if self.phase==1=>self.scan.next_demands(&source.synapses,self.id()?,synapse_id),
   HostScratchEdit::InsertSynapse{synapse,..}if matches!(self.phase,2|3)=>self.scan.next_demands(&source.widgets,if self.phase==2{&synapse.from}else{&synapse.to},widget_id_for),
   HostScratchEdit::InsertSynapse{synapse,..}if self.phase==4&&self.domain==HostScratchDomain::Generation2d=>{let bytes=source.synapses.get(self.edge).map_or(0,|candidate|{let a=wire_field(candidate,self.field);let b=wire_field(synapse,self.field);usize::from(a.len()==b.len()&&self.byte<a.len())*2});Ok(fixed(bytes))},
   HostScratchEdit::SetLayout{..}|HostScratchEdit::SetSlider{..}if self.phase==1=>self.scan.next_demands(&source.widgets,self.id()?,widget_id_for),
   HostScratchEdit::SetLayout{..}if self.phase==2=>Ok(fixed(2*size_of::<f64>())),
   HostScratchEdit::RemoveLayout{id}=>match self.phase{1=>match self.fields.lookup.as_ref(){Some(lookup)=>lookup.next_demands(&source.layout,id,copy),None=>Ok(fixed(0))},3=>self.fields.lookup.as_ref().ok_or_else(||invalid("layout lookup is absent"))?.close_demands(copy),_=>Ok(fixed(0))},
   HostScratchEdit::SetCamera{..}=>Ok(fixed(match self.phase{1=>size_of::<CameraJson>(),_=>2*size_of::<CameraJson>()})),
   HostScratchEdit::SetSchema{schema}=>Ok(fixed(if self.phase==1&&self.domain==HostScratchDomain::Generation3d{usize::from(self.byte<schema.len())}else if self.phase==2&&source.schema.len()==schema.len()&&self.byte<schema.len(){2}else if self.phase==3{2*size_of::<String>()}else{0})),
   HostScratchEdit::SetSlider{..}=>Ok(fixed(if self.phase==2{size_of::<f64>()}else if self.phase==3{2*size_of::<f64>()}else{8*size_of::<f64>()})),
   _=>Ok(fixed(0)),
  }
 }
 pub fn advance(&mut self,source:&mut FlowHostSnapshot,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<bool>{self.progress=Default::default();let result=self.advance_original(source,effect);if effect.performed_work()>0||self.progress!=Default::default(){self.binding=Some(source.source_identity())}RetainedWorkOutcome{result:result.map_err(|error|{if self.progress==Default::default(){self.progress=error.retained_progress()}error.with_retained_progress(self.progress)}),work:effect.performed_work()}}
 fn advance_original(&mut self,source:&mut FlowHostSnapshot,effect:&mut RetainedAllocationEffect<'_>)->Result<bool,ValueError>{
  let grant=effect.grant();let demand=self.next_demands(source,grant.maximum_copy_bytes)?;if self.ready{return Ok(true)}if !retained_clone_grant_admits(grant,demand){return Ok(false)}
  if self.phase==0{if !pay(effect,1)?{return Ok(false)}self.binding=Some(source.source_identity());self.phase=1;self.progress.copied_items=1;return Ok(false)}
  if let Some(moves)=self.fields.moves.as_mut(){let turn=moves.advance(source,effect);self.progress=moves.progress();if turn.result?{let applied=moves.applied();self.finish(applied)}return Ok(self.ready)}
  if self.fields.pending.is_some(){if !pay(effect,1)?{return Ok(false)}source.layout=self.fields.pending.take().unwrap();self.progress=local_progress(demand);self.finish(true);return Ok(true)}
  if let Some(rows)=self.fields.rows.as_mut(){let turn=rows.advance(source,effect);self.progress=rows.progress();if turn.result?{self.finish(true)}return Ok(self.ready)}
  if let Some(entry)=self.fields.entry.as_mut(){
   let body=grant.maximum_copy_bytes;let mut output=None;
   let reply=if self.recover{replied(effect,|effect,reply|entry.recover_advance_into(body,&mut output,effect,reply))}else{replied(effect,|effect,reply|entry.advance_into(body,&mut output,effect,reply))}?;
   match reply{Some(result)=>{self.recover=false;self.progress=result.map_err(OriginalRetirementFailure::into_value_error)?.progress();if let Some(map)=output{self.fields.pending=Some(map);let entry=self.fields.entry.take().unwrap();self.fields.entry_close=Some(ControlledRetirement::new(entry).unwrap_or_else(|_|unreachable!("layout map edit declares controlled retirement")))}},None=>self.recover=true}
   return Ok(false)
  }
  if let Some(close)=self.fields.entry_close.as_mut(){let step=settled_step(close.step(grant.maximum_copy_bytes,effect))?;self.progress=step.progress();if close.terminal_is_empty(){self.fields.entry_close=None}return Ok(false)}
  let edit_ref=self.fields.edit.as_ref().ok_or_else(||invalid("procedural host edit disappeared"))?;
  let scanning=match edit_ref{
   HostScratchEdit::InsertWidget{..}|HostScratchEdit::ReplaceWidget{..}|HostScratchEdit::RemoveWidget{..}|HostScratchEdit::ReplaceSynapse{..}|HostScratchEdit::RemoveSynapse{..}|HostScratchEdit::SetLayout{..}|HostScratchEdit::SetSlider{..}=>self.phase==1,
   HostScratchEdit::InsertSynapse{..}=>matches!(self.phase,1|2|3),
   HostScratchEdit::RemoveLayout{..}=>matches!(self.phase,1|3),
   _=>false,
  };
  if !scanning{if !pay(effect,1)?{return Ok(false)}self.progress=local_progress(demand)}
  match self.fields.edit.as_ref().ok_or_else(||invalid("procedural host edit disappeared"))?{
   HostScratchEdit::InsertWidget{..}|HostScratchEdit::ReplaceWidget{..}|HostScratchEdit::RemoveWidget{..}|HostScratchEdit::InsertSynapse{..}|HostScratchEdit::ReplaceSynapse{..}|HostScratchEdit::RemoveSynapse{..}=>{
    if self.phase==1{
     let edit=self.fields.edit.as_ref().unwrap();let key=match edit{HostScratchEdit::InsertWidget{widget,..}|HostScratchEdit::ReplaceWidget{widget}=>widget_id_for(widget),HostScratchEdit::RemoveWidget{id}|HostScratchEdit::RemoveSynapse{id}=>id,HostScratchEdit::InsertSynapse{synapse,..}|HostScratchEdit::ReplaceSynapse{synapse}=>&synapse.id,_=>return Err(invalid("scratch row identity is absent"))};let turn=match edit{HostScratchEdit::InsertWidget{..}|HostScratchEdit::ReplaceWidget{..}|HostScratchEdit::RemoveWidget{..}=>self.scan.advance(&source.widgets,key,widget_id_for,effect),_=>self.scan.advance(&source.synapses,key,synapse_id,effect)};self.progress=self.scan.step_progress();
     if turn.result?{self.index=self.scan.result();let insert=matches!(self.fields.edit.as_ref(),Some(HostScratchEdit::InsertWidget{..}|HostScratchEdit::InsertSynapse{..}));if insert==self.index.is_some(){self.finish(false)}else{self.scan=ScratchRowScan::new();self.phase=2}}return Ok(self.ready)
    }
    if let Some(HostScratchEdit::InsertSynapse{synapse,..})=self.fields.edit.as_ref(){
     if matches!(self.phase,2|3){let turn=self.scan.advance(&source.widgets,if self.phase==2{&synapse.from}else{&synapse.to},widget_id_for,effect);self.progress=self.scan.step_progress();if turn.result?{if self.scan.result().is_none(){self.finish(false)}else{self.scan=ScratchRowScan::new();self.phase+=1}}return Ok(self.ready)}
     if self.phase==4{
      if self.domain==HostScratchDomain::Generation3d{self.phase=5;return Ok(false)}
      let Some(candidate)=source.synapses.get(self.edge)else{self.phase=5;return Ok(false)};let a=wire_field(candidate,self.field);let b=wire_field(synapse,self.field);
      if a.len()!=b.len(){self.edge+=1;self.field=0;self.byte=0;return Ok(false)}
      if self.byte<a.len(){if a.as_bytes()[self.byte]!=b.as_bytes()[self.byte]{self.edge+=1;self.field=0;self.byte=0;return Ok(false)}self.byte+=1}
      if self.byte==a.len(){self.byte=0;self.field+=1;if self.field==4{self.finish(false)}}return Ok(self.ready)
     }
    }
    self.start_rows()?;Ok(false)
   },
   HostScratchEdit::MoveNodes{..}=>{let Some(HostScratchEdit::MoveNodes{ids,dx,dy})=self.fields.edit.take()else{unreachable!()};self.fields.moves=Some(moves::MoveNodesWork::new(ids,dx,dy));Ok(false)},
   HostScratchEdit::SetLayout{id,layout}=>{
    if self.phase==1{let turn=self.scan.advance(&source.widgets,id,widget_id_for,effect);self.progress=self.scan.step_progress();if turn.result?{if self.scan.result().is_none(){self.finish(false)}else{self.phase=2}}return Ok(self.ready)}
    if self.phase==2{if !(layout.x.is_finite()&layout.y.is_finite()){self.finish(false)}else{self.phase=3}return Ok(self.ready)}
    let Some(HostScratchEdit::SetLayout{id,layout})=self.fields.edit.take()else{unreachable!()};self.fields.entry=Some(OriginalOwnedEntry::new(std::mem::take(&mut source.layout),id,layout));Ok(false)
   },
   HostScratchEdit::RemoveLayout{id}=>{
    match self.phase{
     1=>{let Fields{edit,lookup,..}=&mut*self.fields;let Some(HostScratchEdit::RemoveLayout{id})=edit.as_ref()else{unreachable!()};let lookup=lookup.get_or_insert_with(ScratchMapLookup::new);let complete=lookup.advance(&source.layout,id,effect)?;self.progress=lookup.step_progress();if complete{self.found=lookup.result().is_some();self.phase=3}},
     3=>{let lookup=self.fields.lookup.as_mut().ok_or_else(||invalid("layout lookup is absent"))?;lookup.begin_close();self.progress=settled_step(lookup.close_step(grant.maximum_copy_bytes,effect))?.progress();if lookup.terminal_is_empty(){self.fields.lookup=None;self.phase=5}},
     5=>{if !self.found{self.finish(false)}else{let Some(HostScratchEdit::RemoveLayout{id})=self.fields.edit.take()else{unreachable!()};self.fields.entry=Some(OriginalOwnedEntry::remove(std::mem::take(&mut source.layout),id))}},
     _=>return Err(invalid("layout removal phase is invalid")),
    }Ok(self.ready)
   },
   HostScratchEdit::SetCamera{camera}=>{
    match self.phase{1=>{if !(camera.x.is_finite()&camera.y.is_finite()&camera.zoom.is_finite()){self.finish(false)}else{self.phase=2}},2=>{if (source.camera.x==camera.x)&(source.camera.y==camera.y)&(source.camera.zoom==camera.zoom){self.finish(false)}else{self.phase=3}},3=>{let Some(HostScratchEdit::SetCamera{camera})=self.fields.edit.as_mut()else{unreachable!()};std::mem::swap(&mut source.camera,camera);self.finish(true)},_=>return Err(invalid("camera scratch phase is invalid"))}Ok(self.ready)
   },
   HostScratchEdit::SetSchema{schema}=>{
    if self.phase==1{
     if self.domain==HostScratchDomain::Generation2d{self.phase=2;return Ok(false)}if self.byte==schema.len(){self.finish(false);return Ok(true)}
     let byte=schema.as_bytes()[self.byte];self.byte+=1;if self.scalar_len==0{self.scalar_need=if byte<0x80{1}else if byte<0xe0{2}else if byte<0xf0{3}else{4}}self.scalar[self.scalar_len]=byte;self.scalar_len+=1;
     if self.scalar_len==self.scalar_need{let value=std::str::from_utf8(&self.scalar[..self.scalar_need]).map_err(|_|invalid("owned schema text lost UTF8"))?.chars().next().unwrap();self.scalar_len=0;if !value.is_whitespace(){self.phase=2;self.byte=0}}return Ok(false)
    }
    if self.phase==2{if source.schema.len()!=schema.len(){self.phase=3;return Ok(false)}if self.byte==schema.len(){self.finish(false);return Ok(true)}if source.schema.as_bytes()[self.byte]!=schema.as_bytes()[self.byte]{self.phase=3}else{self.byte+=1;if self.byte==schema.len(){self.finish(false)}}return Ok(self.ready)}
    let Some(HostScratchEdit::SetSchema{schema})=self.fields.edit.as_mut()else{unreachable!()};std::mem::swap(&mut source.schema,schema);self.finish(true);Ok(true)
   },
   HostScratchEdit::SetSlider{id,value}=>{
    match self.phase{
     1=>{let turn=self.scan.advance(&source.widgets,id,widget_id_for,effect);self.progress=self.scan.step_progress();if turn.result?{self.index=self.scan.result();if self.index.is_none(){self.finish(false)}else{self.phase=2}}},
     2=>{if value.is_finite(){self.phase=3}else{self.finish(false)}},
     3=>{match &source.widgets[self.index.unwrap()]{Widget::InputSlider{value:current,..}=>{if *current!=*value{self.phase=4}else{self.finish(false)}},_=>{self.progress.copied_bytes=0;self.finish(false)}}},
     4=>{let applied=set_widget_slider_value(&mut source.widgets[self.index.unwrap()],*value);self.finish(applied)},
     _=>return Err(invalid("slider scratch phase is invalid")),
    }Ok(self.ready)
   },
  }
 }
 fn start_rows(&mut self)->Result<(),ValueError>{
  let edit=self.fields.edit.take().ok_or_else(||invalid("scratch row input is absent"))?;
  macro_rules! payload{($kind:expr,$at:expr,$value:expr,$variant:ident,$restore:expr)=>{match ScratchRowsWork::new($kind,$at,Some($value)){Ok(work)=>{self.fields.rows=Some(Rows::$variant(work));Ok(())},Err((error,value))=>{self.fields.edit=Some(($restore)(value.expect("row admission returns the exact owned payload")));Err(error)}}}}
  match edit{
   HostScratchEdit::InsertWidget{index,widget}=>payload!(ScratchRowsKind::Insert,index,widget,Widgets,|widget|HostScratchEdit::InsertWidget{index,widget}),
   HostScratchEdit::ReplaceWidget{widget}=>payload!(ScratchRowsKind::Replace,self.index.unwrap(),widget,Widgets,|widget|HostScratchEdit::ReplaceWidget{widget}),
   HostScratchEdit::InsertSynapse{index,synapse}=>payload!(ScratchRowsKind::Insert,index,synapse,Synapses,|synapse|HostScratchEdit::InsertSynapse{index,synapse}),
   HostScratchEdit::ReplaceSynapse{synapse}=>payload!(ScratchRowsKind::Replace,self.index.unwrap(),synapse,Synapses,|synapse|HostScratchEdit::ReplaceSynapse{synapse}),
   HostScratchEdit::RemoveWidget{id}=>{self.fields.edit=Some(HostScratchEdit::RemoveWidget{id});self.fields.rows=Some(Rows::Widgets(ScratchRowsWork::new(ScratchRowsKind::Remove,self.index.unwrap(),None).map_err(|(error,_)|error)?));Ok(())},
   HostScratchEdit::RemoveSynapse{id}=>{self.fields.edit=Some(HostScratchEdit::RemoveSynapse{id});self.fields.rows=Some(Rows::Synapses(ScratchRowsWork::new(ScratchRowsKind::Remove,self.index.unwrap(),None).map_err(|(error,_)|error)?));Ok(())},
   edit=>{self.fields.edit=Some(edit);Err(invalid("scratch row request has a different kind"))},
  }
 }
 pub fn step(&mut self,source:&mut FlowHostSnapshot,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{super::retained_scratch_driver::drive(self,source,cx)}
 pub fn begin_close(&mut self){self.closing=true;if let Some(moves)=self.fields.moves.as_mut(){moves.begin_close()}if let Some(rows)=self.fields.rows.as_mut(){rows.begin_close()}if let Some(lookup)=self.fields.lookup.as_mut(){lookup.begin_close()}}
 pub fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if let Some(moves)=self.fields.moves.as_ref(){return if moves.terminal(){Ok(fixed(0))}else{moves.close_demands(copy)}}
  if !self.retirement.is_empty(){return self.retirement.demands(copy)}
  if let Some(rows)=self.fields.rows.as_ref(){return if rows.terminal(){Ok(fixed(0))}else{rows.close_demands(copy)}}
  if let Some(lookup)=self.fields.lookup.as_ref(){return if lookup.terminal_is_empty(){Ok(fixed(0))}else{lookup.close_demands(copy)}}
  if self.fields.entry.is_some(){return Ok(fixed(0))}
  if let Some(close)=self.fields.entry_close.as_ref(){return if close.terminal_is_empty(){Ok(fixed(0))}else{close.selected_demand(copy)}}
  if self.fields.pending.is_some(){return self.retirement.next_owner_demands::<OrderedMap<WidgetLayout>>(true,&None,copy)}
  if self.fields.edit.is_some(){return self.retirement.next_owner_demands::<HostScratchEdit>(true,&None,copy)}Ok(Default::default())
 }
 pub fn close_step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
  self.progress=Default::default();let result=self.close_original(body,effect).map_err(|exit|match exit{RetirementPollExit::Failed(failure)=>{if self.progress==Default::default(){self.progress=failure.retained_progress()}RetirementPollExit::Failed(failure.with_retained_progress(self.progress))},pending=>pending});RetirementPollExit::settle(effect,result)
 }
 fn close_original(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,RetirementPollExit>{
  if !self.closing{return Err(invalid("host scratch close was not begun").into())}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}if !retained_clone_grant_admits(effect.grant(),self.next_close_demands(body)?){return Ok(RetainedCloneStep::Progress(Default::default()))}
  if let Some(moves)=self.fields.moves.as_mut(){if moves.terminal(){self.fields.moves=None;self.progress.copied_items=1}else{self.progress=RetirementPollExit::resume(moves.close_step(body,effect))?.progress()}return Ok(RetainedCloneStep::Progress(self.progress))}
  if !self.retirement.is_empty(){self.progress=RetirementPollExit::resume(self.retirement.step_granted(body,effect))?.progress();return Ok(RetainedCloneStep::Progress(self.progress))}
  if let Some(rows)=self.fields.rows.as_mut(){if rows.terminal(){self.fields.rows=None;self.progress.copied_items=1}else{self.progress=RetirementPollExit::resume(rows.close_step(body,effect))?.progress()}return Ok(RetainedCloneStep::Progress(self.progress))}
  if let Some(lookup)=self.fields.lookup.as_mut(){if lookup.terminal_is_empty(){self.fields.lookup=None;self.progress.copied_items=1}else{lookup.begin_close();self.progress=RetirementPollExit::resume(lookup.close_step(body,effect))?.progress()}return Ok(RetainedCloneStep::Progress(self.progress))}
  if let Some(entry)=self.fields.entry.take(){self.fields.entry_close=Some(ControlledRetirement::new(entry).unwrap_or_else(|_|unreachable!("layout map edit declares controlled retirement")));self.progress.copied_items=1;return Ok(RetainedCloneStep::Progress(self.progress))}
  if let Some(close)=self.fields.entry_close.as_mut(){if close.terminal_is_empty(){self.fields.entry_close=None;self.progress.copied_items=1}else{self.progress=RetirementPollExit::resume(close.step(body,effect))?.progress()}return Ok(RetainedCloneStep::Progress(self.progress))}
  self.progress=if self.fields.pending.is_some(){begun(&mut self.retirement,&mut self.fields.pending,effect)?}else{begun(&mut self.retirement,&mut self.fields.edit,effect)?}.progress();
  Ok(RetainedCloneStep::Progress(self.progress))
 }
 pub fn step_progress(&self)->RetainedCloneProgress{self.progress}
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.retirement.is_empty()&&self.fields.moves.is_none()&&self.fields.edit.is_none()&&self.fields.rows.is_none()&&self.fields.entry.is_none()&&self.fields.entry_close.is_none()&&self.fields.lookup.is_none()&&self.fields.pending.is_none()}
}
impl Drop for HostScratchWork{fn drop(&mut self){assert!(self.terminal_is_empty(),"procedural host scratch retains original input, rows, lookup or maps");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.fields)}}}}

impl super::retained_scratch_driver::OriginalScratchWork<FlowHostSnapshot> for HostScratchWork{
 fn origin(&self)->(u64,u64){self.origin}
 fn source_binding(&self)->semio_framework_job::OriginalSourceBinding{self.original}
 fn ready(&self)->bool{HostScratchWork::ready(self)}
 fn advancing_retirement(&self)->bool{self.fields.moves.as_ref().is_some_and(moves::MoveNodesWork::retiring)||self.phase==3&&self.fields.lookup.is_some()||self.fields.rows.as_ref().is_some_and(Rows::retiring)||self.fields.entry.as_ref().is_some_and(OriginalOwnedEntry::advancing_retirement)||self.fields.entry_close.is_some()}
 fn next_demands(&self,source:&FlowHostSnapshot,copy:usize,_work:u64)->Result<RetirementDemand,ValueError>{HostScratchWork::next_demands(self,source,copy)}
 fn advance(&mut self,source:&mut FlowHostSnapshot,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<bool>{HostScratchWork::advance(self,source,effect)}
 fn step_progress(&self)->RetainedCloneProgress{self.progress}
}
