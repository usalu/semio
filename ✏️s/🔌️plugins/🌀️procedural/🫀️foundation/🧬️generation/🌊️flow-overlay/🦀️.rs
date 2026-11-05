//! 🧬️ Selected generation values patch a retained native Flow scene through original lookup and copy events.
use semio_framework_artifact_flow_flow::{FlowHostSnapshot,FlowSnapshotSource,FlowSourceIdentity,Widget,widget_id_for};
use semio_framework_artifact_flow_flow::OrderedMap;
use semio_os_flow::{LookupCursor,SharedOwner};
use semio_framework_value::{DslValue,ValueError,ValueRefusalKind,RetirementDemand,RetainedWorkOutcome,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use std::mem::ManuallyDrop;
#[derive(semio_framework_value::RetireOwned)]
#[value(send_only)]
struct Fields{text:Option<String>,key:Option<SharedOwner<String>>,lookup:Option<LookupCursor<DslValue>>,displaced:Option<String>}
pub struct GenerationFlowOverlayWork{fields:ManuallyDrop<Option<Fields>>,child:Option<ControlledRetirement<LookupCursor<DslValue>>>,old:Option<ControlledRetirement<String>>,close:Option<ControlledRetirement<Fields>>,original:Option<semio_framework_job::OriginalSourceBinding>,binding:Option<(FlowSourceIdentity,Option<usize>,usize)>,widget:usize,offset:usize,text_origin:Option<(usize,usize)>,phase:u8,closing:bool,progress:RetainedCloneProgress,work:u64}
impl GenerationFlowOverlayWork{
 pub fn new()->Self{Self{fields:ManuallyDrop::new(Some(Fields{text:None,key:None,lookup:None,displaced:None})),child:None,old:None,close:None,original:None,binding:None,widget:0,offset:0,text_origin:None,phase:0,closing:false,progress:Default::default(),work:0}}
 fn check(&self,source:&FlowHostSnapshot,values:&OrderedMap<DslValue>)->Result<(),ValueError>{let actual=(source.source_identity(),values.original_root_identity(),values.len());if self.binding.is_some_and(|original|original!=actual){Err(ValueError::literal(ValueRefusalKind::Canceled,"original generation overlay source changed"))}else{Ok(())}}
 fn text<'a>(&'a self,source:&'a FlowHostSnapshot)->&'a str{if matches!(self.phase,1|2|3){widget_id_for(&source.widgets[self.widget])}else{self.fields.as_ref().unwrap().lookup.as_ref().unwrap().result().and_then(DslValue::as_str).unwrap()}}
 pub fn next_demands(&self,source:&FlowHostSnapshot,values:&OrderedMap<DslValue>,copy:usize)->Result<RetirementDemand,ValueError>{
  if self.closing{return self.next_close_demands(copy)}self.check(source,values)?;
  if let Some(child)=self.child.as_ref(){return child.selected_demand(copy)}if let Some(old)=self.old.as_ref(){return old.selected_demand(copy)}
  Ok(match self.phase{
   1|8=>RetirementDemand{capacity_bytes:self.text(source).len(),depth:1,..Default::default()},
   2|9=>{let text=self.text(source);if self.text_origin!=Some((text.as_ptr()as usize,text.len())){return Err(ValueError::literal(ValueRefusalKind::Canceled,"original overlay text source changed"))}let tail=text.get(self.offset..).ok_or_else(||ValueError::literal(ValueRefusalKind::Canceled,"original overlay text boundary changed"))?;RetirementDemand{copy_bytes:tail.chars().next().map_or(0,char::len_utf8),depth:1,..Default::default()}},
   4=>RetirementDemand{capacity_bytes:SharedOwner::<String>::allocation_bytes(),depth:1,..Default::default()},
   5=>values.lookup_constructor_demands(),
   6=>return self.fields.as_ref().unwrap().lookup.as_ref().unwrap().next_original_demands(),
   _=>RetirementDemand{depth:1,..Default::default()},
  })
 }
 pub fn advance(&mut self,source:&mut FlowHostSnapshot,values:&OrderedMap<DslValue>,grant:RetainedCloneGrant)->RetainedWorkOutcome<bool>{self.progress=Default::default();self.work=0;let result=self.advance_original(source,values,grant).map_err(|error|{let p=error.retained_progress();if p!=Default::default(){self.progress=p}error.with_retained_progress(self.progress)});RetainedWorkOutcome{result,work:self.work}}
 fn advance_original(&mut self,source:&mut FlowHostSnapshot,values:&OrderedMap<DslValue>,grant:RetainedCloneGrant)->Result<bool,ValueError>{
  if self.closing{return Err(ValueError::literal(ValueRefusalKind::Canceled,"original generation overlay is closing"))}self.check(source,values)?;if self.phase==255{return Ok(true)}
  let d=self.next_demands(source,values,grant.maximum_copy_bytes)?;if grant.maximum_items==0||grant.maximum_copy_bytes<d.copy_bytes||grant.maximum_capacity_bytes<d.capacity_bytes||grant.maximum_release_bytes<d.release_bytes||grant.maximum_depth<d.depth{return Ok(false)}
  self.binding=Some((source.source_identity(),values.original_root_identity(),values.len()));self.work=1;self.progress.copied_items=1;
  match self.phase{
   0=>{if values.is_empty()||self.widget>=source.widgets.len(){self.phase=255;return Ok(true)}if matches!(source.widgets[self.widget],Widget::InputSlider{..}|Widget::InputNote{..}|Widget::InputImage{..}|Widget::Variable{..}){self.phase=1}else{self.widget+=1}},
   1|8=>{let text=self.text(source);let(length,pointer)=(text.len(),text.as_ptr()as usize);self.text_origin=Some((pointer,length));let f=self.fields.as_mut().unwrap();f.text=Some(String::new());let output=f.text.as_mut().unwrap();output.try_reserve_exact(length).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original generation overlay text allocation refused"))?;self.progress.retained_capacity_bytes=output.capacity();self.offset=0;self.phase+=1},
   2|9=>{let f=self.fields.as_mut().unwrap();let text=if self.phase==2{widget_id_for(&source.widgets[self.widget])}else{f.lookup.as_ref().unwrap().result().and_then(DslValue::as_str).unwrap()};let character=text.get(self.offset..).and_then(|tail|tail.chars().next());if let Some(character)=character{let count=character.len_utf8();let output=f.text.as_mut().unwrap();if output.len()+count>output.capacity(){return Err(ValueError::literal(ValueRefusalKind::Canceled,"original overlay text grew beyond its original allocation"))}output.push(character);self.offset+=count;self.progress.copied_bytes=count}if self.offset==text.len(){self.phase+=1}},
   3|10=>self.phase+=1,
   4=>{let f=self.fields.as_mut().unwrap();let turn=SharedOwner::admit(f.text.take().unwrap(),grant);self.work=turn.work;self.progress=Default::default();match turn.result{Ok((key,p))=>{f.key=Some(key);self.progress=p;self.phase=5},Err((error,text))=>{f.text=Some(text);return Err(error)}}},
   5=>{let f=self.fields.as_mut().unwrap();let turn=values.admit_lookup_original(f.key.take().unwrap(),grant);self.work=turn.work;self.progress=Default::default();match turn.result{Ok((lookup,p))=>{f.lookup=Some(lookup);self.progress=p;self.phase=6},Err((error,key))=>{f.key=Some(key);return Err(error)}}},
   6=>{let f=self.fields.as_mut().unwrap();let lookup=f.lookup.as_mut().unwrap();let turn=lookup.advance_original(grant);self.work=turn.work;self.progress=Default::default();let step=turn.result?;self.progress=step.progress();if matches!(step,RetainedCloneStep::Complete(_)){self.phase=7}},
   7=>{let value=self.fields.as_ref().unwrap().lookup.as_ref().unwrap().result();match(&mut source.widgets[self.widget],value){(Widget::InputSlider{value:current,..},Some(value))=>{if let Some(value)=value.as_f64(){*current=value}self.phase=12},(Widget::InputNote{..}|Widget::InputImage{..}|Widget::Variable{..},Some(DslValue::String(_)))=>self.phase=8,_=>self.phase=12}},
   11=>{let f=self.fields.as_mut().unwrap();let text=f.text.take().unwrap();let target=match &mut source.widgets[self.widget]{Widget::InputNote{text,..}=>text,Widget::InputImage{src,..}=>src,Widget::Variable{name,..}=>name,_=>unreachable!("original overlay selected a native text field")};f.displaced=Some(std::mem::replace(target,text));self.phase=12},
   12=>{self.child=Some(ControlledRetirement::new(self.fields.as_mut().unwrap().lookup.take().unwrap()).unwrap_or_else(|_|unreachable!("original lookup declares controlled retirement")));self.phase=13},
   13=>{let child=self.child.as_mut().unwrap();let result=child.step(grant);self.work=child.last_turn_work();self.progress=child.step_progress();result?;if child.terminal_is_empty(){self.child=None;self.phase=14}},
   14=>{if let Some(old)=self.fields.as_mut().unwrap().displaced.take(){self.old=Some(ControlledRetirement::new(old).unwrap_or_else(|_|unreachable!("original text declares controlled retirement")));self.phase=15}else{self.widget+=1;self.phase=0}},
   15=>{let old=self.old.as_mut().unwrap();let result=old.step(grant);self.work=old.last_turn_work();self.progress=old.step_progress();result?;if old.terminal_is_empty(){self.old=None;self.widget+=1;self.phase=0}},
   _=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original generation overlay phase is unknown")),
  }Ok(false)
 }

 pub fn new_original(original:semio_framework_job::OriginalSourceBinding)->Self{let mut work=Self::new();work.original=Some(original);work}
 pub fn step(&mut self,source:&mut FlowHostSnapshot,values:&OrderedMap<DslValue>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
  if self.original.is_some_and(|original|!original.matches(cx)){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"generation overlay requires its initiating original caller").into())}
  cx.original_source_parent()?.validate_borrowed_context(cx)?;
  if self.original.is_none(){self.original=Some(semio_framework_job::OriginalSourceBinding::borrow(cx))}
  super::retained_scratch_driver::drive(self,&mut OverlaySource{source,values},cx)
 }

 pub fn begin_close(&mut self){self.closing=true}
 pub fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{if let Some(child)=self.child.as_ref(){return child.selected_demand(copy)}if let Some(old)=self.old.as_ref(){return old.selected_demand(copy)}if let Some(close)=self.close.as_ref(){return close.selected_demand(copy)}Ok(RetirementDemand{depth:usize::from(self.fields.is_some()),..Default::default()})}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{self.progress=Default::default();self.work=0;let result=self.close_original(grant).map_err(|error|error.with_retained_progress(self.progress));RetainedWorkOutcome{result,work:self.work}}
 fn close_original(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original generation overlay close was not begun"))}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
  if let Some(child)=self.child.as_mut(){let result=child.step(grant);self.work=child.last_turn_work();self.progress=child.step_progress();let step=result?;if child.terminal_is_empty(){self.child=None}return Ok(RetainedCloneStep::Progress(step.progress()))}
  if let Some(old)=self.old.as_mut(){let result=old.step(grant);self.work=old.last_turn_work();self.progress=old.step_progress();let step=result?;if old.terminal_is_empty(){self.old=None}return Ok(RetainedCloneStep::Progress(step.progress()))}
  if let Some(close)=self.close.as_mut(){let result=close.step(grant);self.work=close.last_turn_work();self.progress=close.step_progress();let step=result?;if close.terminal_is_empty(){self.close=None}return Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})}
  if grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()))}self.close=Some(ControlledRetirement::new(self.fields.take().unwrap()).unwrap_or_else(|_|unreachable!("original overlay fields declare controlled retirement")));self.work=1;self.progress.copied_items=1;Ok(RetainedCloneStep::Progress(self.progress))
 }
 pub fn step_progress(&self)->RetainedCloneProgress{self.progress}
 pub fn last_turn_work(&self)->u64{self.work}
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.fields.is_none()&&self.child.is_none()&&self.old.is_none()&&self.close.is_none()}
}
impl Drop for GenerationFlowOverlayWork{fn drop(&mut self){assert!(self.terminal_is_empty(),"original generation overlay retains native copy or lookup custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.fields)}}}}


struct OverlaySource<'a>{source:&'a mut FlowHostSnapshot,values:&'a OrderedMap<DslValue>}
impl<'a> super::retained_scratch_driver::OriginalScratchWork<OverlaySource<'a>> for GenerationFlowOverlayWork{
 fn origin(&self)->(u64,u64){let original=self.original.expect("original overlay driver is bound before work");(original.operation().0,original.generation().0)}
 fn source_binding(&self)->semio_framework_job::OriginalSourceBinding{self.original.expect("original overlay driver is bound before work")}
 fn ready(&self)->bool{!self.closing&&self.phase==255}
 fn advancing_retirement(&self)->bool{self.child.is_some()||self.old.is_some()}
 fn next_demands(&self,source:&OverlaySource<'a>,copy:usize)->Result<RetirementDemand,ValueError>{GenerationFlowOverlayWork::next_demands(self,source.source,source.values,copy)}
 fn advance(&mut self,source:&mut OverlaySource<'a>,grant:RetainedCloneGrant)->RetainedWorkOutcome<bool>{GenerationFlowOverlayWork::advance(self,source.source,source.values,grant)}
 fn step_progress(&self)->RetainedCloneProgress{self.progress}
}
