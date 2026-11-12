//! 🎯️ Original gesture selections classify and group through bounded borrowed byte frontiers.
use crate::source_birth::{Birth,begun,closed,construction,fixed,resumed};
use semio_framework_job::{OriginalSourceBinding,OriginalTurnFailure,StepContext};
use semio_framework_value::{RetainedAllocationEffect,RetireOwned,RetirementDemand,RetirementPollExit,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneClose,retained_clone_grant_admits};
use semio_s_artifact_procedural_3d_editor_selection::{ComponentTarget,AnalyticComponent};
use std::mem::{ManuallyDrop,size_of};
const QUANTUM:usize=64;
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum SelectionKind{Basic,Components}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum SelectionRefusal{FirstInvalid,InvalidTarget,MixedTargets,ConflictingIdentity}
impl SelectionRefusal{
 pub fn detail(self)->&'static str{match self{Self::FirstInvalid=>"Select mesh faces, edges, or vertices in the preview",Self::InvalidTarget=>"The component selection contains an invalid target",Self::MixedTargets=>"Select components of one evaluated geometry at the same granularity",Self::ConflictingIdentity=>"A renderer group has inconsistent component identity"}}
 #[cfg(test)]
 fn key(self)->&'static str{match self{Self::FirstInvalid=>"first-invalid",Self::InvalidTarget=>"invalid-target",Self::MixedTargets=>"mixed-targets",Self::ConflictingIdentity=>"conflicting-identity"}}
}
#[derive(Clone,Copy,Default)]
struct Parsed{at:Option<usize>,hash:Option<usize>,dots:[Option<usize>;2],tildes:[usize;3],tilde_count:usize,ats:usize,hashes:usize,end:usize,index:u32,component:u32}
impl Parsed{
 fn analytic(self)->bool{self.tilde_count==3}
 fn span(self,lane:usize)->(usize,usize){match lane{0=>(0,self.dots[0].unwrap()),1=>(self.dots[0].unwrap()+1,self.dots[1].unwrap()),2=>(self.tildes[0]+1,self.tildes[1]),_=>(self.tildes[2]+1,self.end)}}
 fn label(self)->(usize,usize){(self.tildes[1]+1,self.tildes[2])}
}
#[derive(Clone,Copy,RetireOwned)]
struct Entry{component:u32,ordinal:usize,label_start:usize,label_end:usize}
#[derive(Clone,Copy,PartialEq,Eq)]
enum Phase{Scan,Validate,Index,Component,Label,Handle,Revision,Classify,Capacity,Compare,Search,LabelCompare,Insert,Shift,Next,Ready}
pub struct GumballSelectionWork{
 entries:ManuallyDrop<Option<Vec<Entry>>>,retirement:RetainedCloneClose,birth:Birth,original:OriginalSourceBinding,source:Option<(usize,usize)>,current:Option<(usize,usize)>,first_identity:Option<(usize,usize)>,parsed:Parsed,first:Option<Parsed>,ordinal:usize,byte:usize,lane:usize,position:usize,shift:usize,phase:Phase,outcome:Option<Result<SelectionKind,SelectionRefusal>>,closing:bool,progress:RetainedCloneProgress
}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"gumball selection changed its actual borrowed gesture, caller or owner phase")}
fn identity(ids:&[String])->(usize,usize){(ids.as_ptr()as usize,ids.len())}
fn text_identity(value:&str)->(usize,usize){(value.as_ptr()as usize,value.len())}
fn number(text:&str,maximum:u64,zero:bool)->Option<u64>{
 if text.is_empty()||text.len()>20||text.len()>1&&text.as_bytes()[0]==b'0'||!zero&&text.as_bytes()[0]==b'0'{return None}
 let mut value=0u64;for byte in text.bytes(){if !byte.is_ascii_digit(){return None}value=value.checked_mul(10)?.checked_add(u64::from(byte-b'0'))?;if value>maximum{return None}}Some(value)
}
fn same(a:&str,b:&str,offset:usize,count:usize)->bool{a.as_bytes()[offset..offset+count].iter().zip(&b.as_bytes()[offset..offset+count]).fold(true,|equal,(a,b)|equal&(*a==*b))}
impl GumballSelectionWork{
 pub fn new(cx:&StepContext<'_>)->Self{Self::new_original(OriginalSourceBinding::borrow(cx))}
 pub fn new_original(original:OriginalSourceBinding)->Self{Self{entries:ManuallyDrop::new(None),retirement:Default::default(),birth:Default::default(),original,source:None,current:None,first_identity:None,parsed:Default::default(),first:None,ordinal:0,byte:0,lane:0,position:0,shift:0,phase:Phase::Scan,outcome:None,closing:false,progress:Default::default()}}
 fn validate(&self,ids:&[String])->Result<(),ValueError>{
  if self.closing||self.source.is_some_and(|source|source!=identity(ids))||self.current.is_some_and(|current|ids.get(self.ordinal).is_none_or(|id|current!=text_identity(id)))||self.first_identity.is_some_and(|first|ids.first().is_none_or(|id|first!=text_identity(id))){return Err(invalid())}Ok(())
 }
 pub fn ready(&self)->bool{!self.closing&&self.phase==Phase::Ready}
 pub fn result(&self)->Option<Result<SelectionKind,SelectionRefusal>>{if self.ready(){self.outcome}else{None}}
 pub fn component_count(&self)->usize{if self.result()==Some(Ok(SelectionKind::Components)){self.entries.as_ref().map_or(0,Vec::len)}else{0}}
 pub fn component(&self,index:usize)->Option<u32>{if self.result()!=Some(Ok(SelectionKind::Components)){return None}self.entries.as_ref()?.get(index).map(|entry|entry.component)}
 pub fn target<'a>(&self,ids:&'a[String],cx:&StepContext<'_>)->Result<Option<ComponentTarget<'a>>,OriginalTurnFailure>{
  self.validate(ids)?;if !self.original.matches(cx){return Err(invalid().into())}cx.original_source_parent()?.validate_borrowed_context(cx)?;
  if self.result()!=Some(Ok(SelectionKind::Components)){return Ok(None)}let p=self.first.ok_or_else(invalid)?;let id=ids.first().ok_or_else(invalid)?;
  Ok(Some(ComponentTarget{instance:&id[..p.dots[0].unwrap()],widget:&id[..p.at.unwrap()],channel:&id[p.at.unwrap()+1..p.hash.unwrap()],index:p.index as usize,granularity:&id[p.dots[0].unwrap()+1..p.dots[1].unwrap()],component:p.component,analytic:if p.analytic(){Some(AnalyticComponent{handle:&id[p.tildes[0]+1..p.tildes[1]],label:&id[p.tildes[1]+1..p.tildes[2]],revision:&id[p.tildes[2]+1..p.end]})}else{None}}))
 }
 fn finish(&mut self,result:Result<SelectionKind,SelectionRefusal>){self.outcome=Some(result);self.phase=Phase::Ready}
 fn bad(&mut self){if self.first.is_some(){self.finish(Err(SelectionRefusal::InvalidTarget))}else{self.phase=Phase::Next}}
 fn span<'a>(&self,ids:&'a[String],first:bool,lane:usize)->Result<&'a str,ValueError>{let p=if first{self.first.ok_or_else(invalid)?}else{self.parsed};let id=ids.get(if first{0}else{self.ordinal}).ok_or_else(invalid)?;let(a,b)=p.span(lane);id.get(a..b).ok_or_else(invalid)}
 pub fn next_demands(&self,ids:&[String],_copy:usize)->Result<RetirementDemand,ValueError>{
  self.validate(ids)?;if self.ready(){return Ok(Default::default())}
  let metadata=size_of::<Parsed>()+4*size_of::<usize>();
  Ok(match self.phase{
   Phase::Scan=>fixed(metadata+ids.get(self.ordinal).map_or(0,|id|id.len().saturating_sub(self.byte).min(QUANTUM))),
   Phase::Index|Phase::Component|Phase::Label=>fixed(metadata+20),
   Phase::Handle|Phase::Revision=>fixed(metadata+64),
   Phase::Capacity=>RetirementDemand{copy_bytes:size_of::<Vec<Entry>>(),capacity_bytes:ids.len().checked_mul(size_of::<Entry>()).ok_or_else(invalid)?,depth:1,..Default::default()},
   Phase::Compare=>{let a=self.span(ids,true,self.lane)?;let b=self.span(ids,false,self.lane)?;fixed(4*size_of::<usize>()+if a.len()==b.len(){2*a.len().saturating_sub(self.byte).min(QUANTUM)}else{0})},
   Phase::LabelCompare=>fixed(4*size_of::<usize>()+40),
   Phase::Search=>fixed(size_of::<Entry>()+2*size_of::<usize>()),
   Phase::Insert=>fixed(2*size_of::<Entry>()+2*size_of::<usize>()),
   Phase::Shift=>fixed(3*size_of::<Entry>()+size_of::<usize>()),
   _=>fixed(metadata),
  })
 }
 pub fn step(&mut self,ids:&[String],cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(ids)?;if !self.original.matches(cx){return Err(invalid().into())}cx.original_source_parent()?.validate_borrowed_context(cx)?;if self.ready()&&!self.birth.settling(){return Ok(true)}if cx.should_yield()||cx.is_cancelled(){return Ok(false)}
  let d=self.next_demands(ids,cx.retained_grant().maximum_copy_bytes)?;let grant=cx.retained_grant();
  let mut birth=std::mem::take(&mut self.birth);let settled=birth.run(cx,construction(d),||self.advance(ids,grant));self.birth=birth;Ok(settled?&&self.ready())
 }
 fn advance(&mut self,ids:&[String],g:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  self.progress=Default::default();self.advance_inner(ids,g).map(|_|self.progress).map_err(|error|error.with_retained_progress(self.progress))
 }
 fn advance_inner(&mut self,ids:&[String],g:RetainedCloneGrant)->Result<bool,ValueError>{
  let d=self.next_demands(ids,g.maximum_copy_bytes)?;if self.ready(){return Ok(true)}if !retained_clone_grant_admits(g,d){return Ok(false)}
  self.source=Some(identity(ids));self.progress=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()};
  match self.phase{
   Phase::Scan=>{
    let Some(id)=ids.get(self.ordinal)else{self.finish(Ok(if self.first.is_some(){SelectionKind::Components}else{SelectionKind::Basic}));return Ok(true)};
    self.current=Some(text_identity(id));let end=(self.byte+QUANTUM).min(id.len());for index in self.byte..end{let b=id.as_bytes()[index];if b==b'~'{if self.parsed.tilde_count<3{self.parsed.tildes[self.parsed.tilde_count]=index}self.parsed.tilde_count+=1}else if self.parsed.tilde_count==0{match b{b'@'=>{if self.parsed.at.is_none(){self.parsed.at=Some(index)}self.parsed.ats+=1},b'#'=>{self.parsed.hash=Some(index);self.parsed.hashes+=1},b'.'=>{self.parsed.dots[0]=self.parsed.dots[1];self.parsed.dots[1]=Some(index)},_=>{}}}}self.byte=end;if end==id.len(){self.parsed.end=id.len();self.phase=Phase::Validate}
   },
   Phase::Validate=>{
    let p=self.parsed;let Some(id)=ids.get(self.ordinal)else{return Err(invalid())};
    let valid=match(p.at,p.hash,p.dots[0],p.dots[1]){(Some(at),Some(hash),Some(dot),Some(last))=>p.ats==1&&p.hashes==1&&at>0&&at+1<hash&&hash+1<dot&&dot<last&&last<p.end&&matches!(p.tilde_count,0|3)&&matches!(&id[dot+1..last],"vertex"|"edge"|"face")&&(p.tilde_count==0||last<p.tildes[0]),_=>false};
    if valid{self.phase=Phase::Index}else{self.bad()}
   },
   Phase::Index=>{let p=self.parsed;let id=&ids[self.ordinal];let end=p.dots[0].ok_or_else(invalid)?;match number(&id[p.hash.ok_or_else(invalid)?+1..end],u32::MAX as u64,true){Some(value)=>{self.parsed.index=value as u32;self.phase=Phase::Component},None=>self.bad()}},
   Phase::Component=>{let p=self.parsed;let id=&ids[self.ordinal];let end=if p.analytic(){p.tildes[0]}else{p.end};match number(&id[p.dots[1].ok_or_else(invalid)?+1..end],u32::MAX as u64,true){Some(value)=>{self.parsed.component=value as u32;self.phase=if p.analytic(){Phase::Label}else{Phase::Classify}},None=>self.bad()}},
   Phase::Label=>{let p=self.parsed;let(a,b)=p.label();if number(&ids[self.ordinal][a..b],u64::MAX,false).is_some(){self.phase=Phase::Handle}else{self.bad()}},
   Phase::Handle|Phase::Revision=>{let p=self.parsed;let lane=if self.phase==Phase::Handle{2}else{3};let(a,b)=p.span(lane);let id=&ids[self.ordinal];if b-a==64&&id.as_bytes()[a..b].iter().all(|byte|byte.is_ascii_digit()||(b'a'..=b'f').contains(byte)){self.phase=if lane==2{Phase::Revision}else{Phase::Classify}}else{self.bad()}},
   Phase::Classify=>{
    if self.first.is_none(){if self.ordinal!=0{self.finish(Err(SelectionRefusal::FirstInvalid))}else{self.first=Some(self.parsed);self.first_identity=Some(text_identity(&ids[0]));self.phase=Phase::Capacity}}
    else if self.first.unwrap().analytic()!=self.parsed.analytic(){self.finish(Err(SelectionRefusal::MixedTargets))}
    else{self.lane=0;self.byte=0;self.phase=Phase::Compare}
   },
   Phase::Capacity=>{*self.entries=Some(Vec::with_capacity(ids.len()));self.position=0;self.phase=Phase::Insert},
   Phase::Compare=>{
    let a=self.span(ids,true,self.lane)?;let b=self.span(ids,false,self.lane)?;let count=a.len().saturating_sub(self.byte).min(QUANTUM);
    if a.len()!=b.len()||!same(a,b,self.byte,count){self.finish(Err(SelectionRefusal::MixedTargets))}
    else{self.byte+=count;if self.byte==a.len(){self.byte=0;self.lane+=1;if self.lane==2&&!self.parsed.analytic()||self.lane==4{self.position=0;self.phase=Phase::Search}}}
   },
   Phase::Search=>{
    let entries=self.entries.as_ref().ok_or_else(invalid)?;
    if let Some(entry)=entries.get(self.position){if entry.component==self.parsed.component{self.phase=if self.parsed.analytic(){Phase::LabelCompare}else{Phase::Next}}else if entry.component>self.parsed.component{self.phase=Phase::Insert}else{self.position+=1}}else{self.phase=Phase::Insert}
   },
   Phase::LabelCompare=>{
    let entry=self.entries.as_ref().and_then(|entries|entries.get(self.position)).ok_or_else(invalid)?;let(a,b)=self.parsed.label();let old=&ids[entry.ordinal][entry.label_start..entry.label_end];let new=&ids[self.ordinal][a..b];
    if old.len()==new.len()&&same(old,new,0,old.len()){self.phase=Phase::Next}else{self.finish(Err(SelectionRefusal::ConflictingIdentity))}
   },
   Phase::Insert=>{
    let(a,b)=if self.parsed.analytic(){self.parsed.label()}else{(0,0)};let entries=self.entries.as_mut().ok_or_else(invalid)?;if entries.len()==entries.capacity(){return Err(invalid())}
    self.shift=entries.len();entries.push(Entry{component:self.parsed.component,ordinal:self.ordinal,label_start:a,label_end:b});self.phase=Phase::Shift
   },
   Phase::Shift=>{if self.shift>self.position{self.entries.as_mut().ok_or_else(invalid)?.swap(self.shift,self.shift-1);self.shift-=1}else{self.phase=Phase::Next}},
   Phase::Next=>{self.ordinal+=1;self.current=None;self.parsed=Default::default();self.byte=0;self.phase=Phase::Scan},
   Phase::Ready=>{},
  }Ok(self.ready())
 }
 pub fn begin_close(&mut self){self.closing=true}
 pub fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if !self.birth.terminal_is_empty(){return self.birth.close_demand(copy)}if !self.retirement.is_empty(){return self.retirement.demands(copy)}if self.entries.is_some(){return self.retirement.next_owner_demands::<Vec<Entry>>(true,&None,copy)}Ok(if self.source.is_some()||self.first_identity.is_some()||self.current.is_some()||self.first.is_some()||self.outcome.is_some(){fixed(size_of::<Parsed>()+8*size_of::<usize>())}else{Default::default()})
 }
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.terminal_is_empty(){return Ok(true)}if !self.closing{return Err(invalid().into())}if !self.birth.close_step(cx)?{return Ok(false)}
  let body=cx.retained_grant().maximum_copy_bytes;let d=self.next_close_demands(body)?;closed(cx,d,|effect|self.close_original(body,d,effect))?;Ok(self.terminal_is_empty())
 }
 fn close_original(&mut self,body:usize,d:RetirementDemand,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,RetirementPollExit>{
  if !self.retirement.is_empty(){return resumed(&mut self.retirement,body,effect)}if self.entries.is_some(){return begun(&mut self.retirement,&mut self.entries,effect)}
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}if !retained_clone_grant_admits(effect.grant(),d){return Ok(RetainedCloneStep::Progress(Default::default()))}
  self.source=None;self.current=None;self.first_identity=None;self.first=None;self.outcome=None;self.parsed=Default::default();Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.entries.is_none()&&self.retirement.is_empty()&&self.birth.terminal_is_empty()&&self.source.is_none()&&self.current.is_none()&&self.first_identity.is_none()&&self.first.is_none()&&self.outcome.is_none()}
}
impl Drop for GumballSelectionWork{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"gumball selection retains its original spans or component frontier");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.entries)}}}}

#[cfg(test)]
#[path="🧪️.rs"]
mod tests;

#[path="📝️text.rs"]
mod selection_text;
pub use selection_text::GumballSelectionTextWork;
