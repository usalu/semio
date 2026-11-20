//! 🔺️ Actual original text spans and genuinely prepared original run fields form the canonical story splice diff.
use semio_s_artifact_layout_layout_mutations_decisions::{LayoutMutationSource,LayoutMutationView,story_plan::StorySplicePlan,story_text::OriginalTextPreparation,story_source::{SpliceTextSource,story_id}};
use semio_s_artifact_layout_layout_model::{snapshot::LayoutSnapshot,TextStyleRun};
use semio_s_artifact_layout_layout_diff::*;
use semio_os_kernel::RetainedFlight;
use crate::original_field::OriginalField;
use semio_s_artifact_layout_layout_diff::retained::apply::page_cache::close_owned;
use semio_framework_value::{OriginalRetirementCloseLoan,OriginalRetirementPoll,RetainedAllocationEffect,RetirementPollExit,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneRef,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits}};
use std::mem::{ManuallyDrop,size_of};
fn fault()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original story output field custody is invalid")}
fn fixed()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<LayoutDiff>()*4+size_of::<TextStyleRunRow>()*2+2048,depth:1,..Default::default()}}
fn nested(mut d:RetirementDemand)->RetirementDemand{d.copy_bytes+=fixed().copy_bytes;d.depth+=1;d}
fn child(g:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-fixed().copy_bytes,maximum_depth:g.maximum_depth-1,..g}}
fn receipt(mut p:RetainedCloneProgress)->RetainedCloneProgress{p.copied_items=p.copied_items.max(1);p.copied_bytes+=fixed().copy_bytes;p}
fn refused(e:ValueError)->ValueError{let p=receipt(e.retained_progress());e.with_retained_progress(p)}
pub(crate) fn runs<'a>(b:RetainedCloneRef<'a,LayoutSnapshot>,at:usize)->RetainedCloneRef<'a,Vec<TextStyleRun>>{b.project(5,|b|&b.stories).project(at,move|r|&r[at]).project(2,|s|&s.style_runs)}
/// 🧳️ Original-bound partial text, run fields, skipped owned rows, sparse headers and diff are individually closeable.
pub struct StoryOutput{flight:RetainedFlight,text:OriginalTextPreparation,id:OriginalField<String>,runs:OriginalField<Vec<TextStyleRun>>,held:ManuallyDrop<Option<Vec<TextStyleRun>>>,diff:ManuallyDrop<Option<LayoutDiff>>,retirement:RetainedCloneClose,phase:u8,index:usize,next:usize,closing:bool}
impl Default for StoryOutput{fn default()->Self{Self{flight:Default::default(),text:Default::default(),id:Default::default(),runs:Default::default(),held:ManuallyDrop::new(None),diff:ManuallyDrop::new(Some(LayoutDiff::default())),retirement:Default::default(),phase:0,index:0,next:0,closing:false}}}
impl StoryOutput{
 pub fn birth_work()->Result<u64,ValueError>{Ok(OriginalField::<String>::embedded_birth_work()?+OriginalField::<Vec<TextStyleRun>>::embedded_birth_work()?)}
 pub fn ready(&self)->bool{self.phase==7&&!self.closing}
 fn source<'a,M:LayoutMutationSource>(plan:&StorySplicePlan,m:RetainedCloneRef<'a,M>,b:RetainedCloneRef<'a,LayoutSnapshot>)->SpliceTextSource<'a,M>{SpliceTextSource{m,b,story:plan.story,range:plan.range(),inverse:false}}
 pub fn demand<M:LayoutMutationSource>(&self,plan:&StorySplicePlan,m:RetainedCloneRef<'_,M>,b:RetainedCloneRef<'_,LayoutSnapshot>,body:usize,max:u64)->Result<RetirementDemand,ValueError>{plan.verify(m,b)?;if self.closing||!plan.ready()||plan.failure.is_some()||plan.unchanged{return Err(fault())}Ok(match self.phase{0=>RetirementDemand{capacity_bytes:size_of::<LayoutStoriesModification>()+if plan.runs_changed(){plan.new_run_count().checked_mul(size_of::<TextStyleRunRow>()).ok_or_else(fault)?}else{0},..fixed()},1=>nested(self.text.demand(&Self::source(plan,m,b))?),2=>nested(self.text.take_demand()),3=>nested(if self.id.value().is_some(){self.id.exchange_demand()?}else{self.id.demand(story_id(m),body.min(256),max)?}),4 if plan.runs_changed()=>nested(if self.runs.value().is_some(){self.runs.exchange_demand()?}else{self.runs.demand(runs(b,plan.story),body.min(256),max)?}),_=>fixed()})}
 pub fn work_demand<M:LayoutMutationSource>(&self,plan:&StorySplicePlan,m:RetainedCloneRef<'_,M>,b:RetainedCloneRef<'_,LayoutSnapshot>,body:usize,max:u64)->Result<u64,ValueError>{
  match self.phase{
   1=>self.text.work_demand(&Self::source(plan,m,b)),
   3 if self.id.value().is_none()=>self.id.work_demand(story_id(m),body.min(256),max),
   4 if plan.runs_changed()&&self.runs.value().is_none()=>self.runs.work_demand(runs(b,plan.story),body.min(256),max),
   _=>Ok(1)
  }
 }
 pub fn advance<M:LayoutMutationSource>(&mut self,plan:&StorySplicePlan,m:RetainedCloneRef<'_,M>,b:RetainedCloneRef<'_,LayoutSnapshot>,body:usize,max:u64,effect:&mut RetainedAllocationEffect<'_>,recover:bool)->Result<RetainedCloneStep,RetirementPollExit>{let g=effect.grant();let d=if recover{fixed()}else{self.demand(plan,m,b,body,max)?};if !recover&&(!retained_clone_grant_admits(g,d)||self.work_demand(plan,m,b,body,max)?>max.min(effect.remaining_work_units()?)){return Ok(RetainedCloneStep::Progress(Default::default()))}if !recover&&(matches!(self.phase,0|2|5|7)||self.phase==4&&!plan.runs_changed())&&!effect.performed(1)?{return Ok(RetainedCloneStep::Progress(Default::default()))}let mut p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};match self.phase{0=>{*effect.actual()=Some(0);self.phase=254;let diff=self.diff.as_mut().ok_or_else(fault)?;diff.stories=Some(LayoutStoriesDelta::default());let rows=&mut diff.stories.as_mut().unwrap().modified;let mut failed=rows.try_reserve_exact(1).is_err();p.retained_capacity_bytes=rows.capacity()*size_of::<LayoutStoriesModification>();if !failed{rows.push(LayoutStoriesModification{id:String::new(),patch:TextStoryPatch{content:None,style_runs:if plan.runs_changed(){Some(TextStyleRunsDelta{len:Some(plan.new_run_count()),rows:Vec::new()})}else{None}}});if let Some(runs)=rows[0].patch.style_runs.as_mut(){failed|=runs.rows.try_reserve_exact(plan.new_run_count()).is_err();p.retained_capacity_bytes+=runs.rows.capacity()*size_of::<TextStyleRunRow>()}}*effect.actual()=Some(p.retained_capacity_bytes);if failed||p.retained_capacity_bytes>g.maximum_capacity_bytes{return Err(fault().with_retained_progress(p).into())}self.phase=1},1=>{p=receipt(self.text.advance(&Self::source(plan,m,b),&mut effect.nested(child(g)),recover)?.progress());if self.text.ready(){self.phase=2}},2=>{let(value,paid)=self.text.take(child(g)).map_err(refused)?;p=receipt(paid);self.diff.as_mut().unwrap().stories.as_mut().unwrap().modified[0].patch.content=value;self.phase=3},3=>if self.id.value().is_some(){let(value,paid)=self.id.take(&mut effect.nested(child(g))).map_err(refused)?;p=receipt(paid);self.diff.as_mut().unwrap().stories.as_mut().unwrap().modified[0].id=value.ok_or_else(fault)?;self.phase=4}else{p=receipt(self.id.advance(story_id(m),body.min(256),max,&mut effect.nested(child(g)),recover)?.progress())},4=>if !plan.runs_changed(){self.phase=7}else if self.runs.value().is_some(){let(value,paid)=self.runs.take(&mut effect.nested(child(g))).map_err(refused)?;p=receipt(paid);*self.held=value;self.phase=5}else{p=receipt(self.runs.advance(runs(b,plan.story),body.min(256),max,&mut effect.nested(child(g)),recover)?.progress())},5=>{if self.index==b.get().stories[plan.story].style_runs.len(){self.phase=7}else{let LayoutMutationView::EditStory(payload)=m.get().view()else{return Err(fault().into())};let next=plan.transformed(&b.get().stories[plan.story].style_runs[self.index],payload.insert.len()).ok_or_else(fault)?;if let Some([start,end])=next{let mut run=std::mem::replace(&mut self.held.as_mut().ok_or_else(fault)?[self.index],TextStyleRun{start:0,end:0,paragraph_style_id:None,character_style_id:None});run.start=start;run.end=end;self.diff.as_mut().unwrap().stories.as_mut().unwrap().modified[0].patch.style_runs.as_mut().ok_or_else(fault)?.rows.push(TextStyleRunRow{index:self.next,run});self.next+=1}self.index+=1}},7=>{},_=>return Err(fault().into())}Ok(if self.ready(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})}
 pub fn take_demand(&self)->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Option<LayoutDiff>>()*2,depth:1,..Default::default()}}
 pub fn take(&mut self,g:RetainedCloneGrant)->Result<(Option<LayoutDiff>,RetainedCloneProgress),ValueError>{if !self.ready(){return Err(fault())}let d=self.take_demand();if !retained_clone_grant_admits(g,d){return Ok((None,Default::default()))}self.phase=8;Ok((self.diff.take(),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
 pub fn begin_close(&mut self){self.closing=true;self.text.begin_close();self.id.begin_close();self.runs.begin_close()}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(fault())}if !self.text.terminal_is_empty(){return self.text.close_demand(body)}if !self.id.terminal_is_empty(){return self.id.close_demand(body)}if !self.runs.terminal_is_empty(){return self.runs.close_demand(body)}if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<LayoutDiff>(false,&None,body)}if self.held.is_some(){return self.retirement.next_owner_demands::<Vec<TextStyleRun>>(true,&None,body)}self.retirement.next_owner_demands::<LayoutDiff>(self.diff.is_some(),&None,body)}
 pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.text.terminal_is_empty(){return self.text.close_work_demand(body)}
  if !self.id.terminal_is_empty(){return self.id.close_work_demand(body)}
  if !self.runs.terminal_is_empty(){return self.runs.close_work_demand(body)}
  if !self.retirement.is_empty(){return self.retirement.next_owner_work::<LayoutDiff>(false,&None,body)}
  if self.held.is_some(){return self.retirement.next_owner_work::<Vec<TextStyleRun>>(true,&None,body)}
  self.retirement.next_owner_work::<LayoutDiff>(self.diff.is_some(),&None,body)
 }
 pub fn requires_original_birth(&self)->bool{if !self.text.terminal_is_empty(){return self.text.requires_original_birth()}
  if !self.id.terminal_is_empty(){return self.id.requires_original_birth()}
  if !self.runs.terminal_is_empty(){return self.runs.requires_original_birth()}self.flight.idle()&&self.retirement.requires_original_birth()}
 pub fn requires_original_control(&self)->bool{if !self.text.terminal_is_empty(){return self.text.requires_original_control()}
  if !self.id.terminal_is_empty(){return self.id.requires_original_control()}
  if !self.runs.terminal_is_empty(){return self.runs.requires_original_control()}self.flight.loaned().unwrap_or_else(||self.retirement.requires_original_control())}
 pub fn close_original_birth_step(&mut self,loan:&mut OriginalRetirementCloseLoan<'_>,g:RetainedCloneGrant)->OriginalRetirementPoll{if !self.text.terminal_is_empty(){return self.text.close_original_birth_step(loan,g)}
  if !self.id.terminal_is_empty(){return self.id.close_original_birth_step(loan,g)}
  if !self.runs.terminal_is_empty(){return self.runs.close_original_birth_step(loan,g)}self.retirement.step_original_birth(loan,g)}
 pub fn close_step(&mut self,loan:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,recover:bool)->Result<RetainedCloneStep,RetirementPollExit>{
  if !recover&&(!retained_clone_grant_admits(effect.grant(),self.close_demand(body)?)||self.close_work_demand(body)?>effect.remaining_work_units()?){return Ok(RetainedCloneStep::Progress(Default::default()))}
  if !self.text.terminal_is_empty(){return self.text.close_step(loan,body,effect,recover)}
  if !self.id.terminal_is_empty(){return self.id.close_step(loan,body,effect,recover)}
  if !self.runs.terminal_is_empty(){return self.runs.close_step(loan,body,effect,recover)}
  let mut work=0;
  if self.held.is_some()&&self.retirement.is_empty(){return close_owned(&mut self.retirement,&mut self.held,&mut self.flight,loan,body,effect,recover,&mut work)}
  close_owned(&mut self.retirement,&mut self.diff,&mut self.flight,loan,body,effect,recover,&mut work)
 }
 pub fn terminal_is_empty(&self)->bool{self.flight.idle()&&self.closing&&self.text.terminal_is_empty()&&self.id.terminal_is_empty()&&self.runs.terminal_is_empty()&&self.held.is_none()&&self.diff.is_none()&&self.retirement.is_empty()}
}
impl Drop for StoryOutput{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original story output requires funded actual field retirement")}}
