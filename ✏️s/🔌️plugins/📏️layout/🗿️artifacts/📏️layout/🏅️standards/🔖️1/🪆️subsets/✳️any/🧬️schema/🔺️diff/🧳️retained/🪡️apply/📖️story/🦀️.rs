//! 📖️ Story style-run edits move original rows after every ordered positional prefix is admitted.
use crate::{LayoutStoriesDelta,LayoutStoriesModification,TextStyleRunsDelta};
use semio_s_artifact_layout_layout_model::{TextStory,TextStyleRun};
use semio_os_kernel::ApplyCapability;
use semio_framework_value::{OriginalRetirementCloseLoan,OriginalRetirementPoll,RetainedAllocationEffect,RetirementPollExit,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use super::{final_list::{FinalList,VacantRow},super::{positional_chain::PositionalChain,borrowed_lists::BorrowedPositionalChain,keyed_plan::{Origin,PlanAdvance}}};
use std::mem::size_of;
fn delta(d:&LayoutStoriesDelta,i:usize)->&TextStyleRunsDelta{d.modified[i].patch.style_runs.as_ref().expect("selected authored style runs")}
/// 🧳 A root row address and paid sparse indices retain ordered source identity.
pub struct StoryWork{pub target:usize,pub origin:Origin,pub indices:Vec<usize>,pub count:usize,plan:PositionalChain,rows:FinalList<TextStyleRun>,phase:u8,closing:bool}
impl StoryWork{
 pub fn new(target:usize,origin:Origin)->Self{Self{target,origin,indices:Vec::new(),count:0,plan:PositionalChain::new(),rows:FinalList::new(),phase:0,closing:false}}
 fn fixed()->RetirementDemand{RetirementDemand{copy_bytes:128,depth:1,..Default::default()}}
 fn fault()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"Layout retained story stage is invalid")}
 fn source<'a>(&'a self,base:&TextStory,d:&'a LayoutStoriesDelta)->BorrowedPositionalChain<'a,TextStyleRunsDelta,LayoutStoriesDelta>{BorrowedPositionalChain{held_len:base.style_runs.len(),indices:&self.indices,owner:d,delta}}
 pub fn preflight_demand(&self,base:&TextStory,d:&LayoutStoriesDelta)->Result<RetirementDemand,ValueError>{match self.phase{0=>self.plan.demand(&self.source(base,d)),1=>self.rows.preflight_demand(base.style_runs.len(),self.plan.origins().ok_or_else(Self::fault)?),2=>Ok(Self::fixed()),_=>Err(Self::fault())}}
 pub fn preflight_work_demand(&self,base:&TextStory,d:&LayoutStoriesDelta)->Result<u64,ValueError>{self.preflight_demand(base,d)?;if self.phase==1{return self.rows.preflight_work_demand(base.style_runs.len(),self.plan.origins().ok_or_else(Self::fault)?)}Ok(1)}
 pub fn preflight(&mut self,base:&TextStory,d:&LayoutStoriesDelta,effect:&mut RetainedAllocationEffect<'_>,work:&mut u128)->Result<PlanAdvance,ValueError>{let g=effect.grant();let demand=self.preflight_demand(base,d)?;let mut out=PlanAdvance{done:false,error:None,progress:Default::default()};if !semio_framework_value::retained_clone::retained_clone_grant_admits(g,demand){return Ok(out)}let delegated=self.phase==1;if !delegated{*work+=1}match self.phase{0=>{let source=BorrowedPositionalChain{held_len:base.style_runs.len(),indices:&self.indices,owner:d,delta};out=self.plan.advance(&source,g)?;if out.done{self.phase=1;out.done=false}},1=>{let step=self.rows.preflight(base.style_runs.len(),self.plan.origins().ok_or_else(Self::fault)?,effect)?;out.progress=step.progress();if matches!(step,RetainedCloneStep::Complete(_)){self.phase=2;out.done=true}},2=>{out.progress=RetainedCloneProgress{copied_items:1,copied_bytes:128,..Default::default()};out.done=true},_=>return Err(Self::fault())}Ok(out)}
 pub fn commit_demand(&self)->Result<RetirementDemand,ValueError>{self.rows.commit_demand()}
 pub fn commit(&mut self,base:&mut TextStory,modified:&mut [LayoutStoriesModification],g:RetainedCloneGrant,capability:ApplyCapability)->Result<RetainedCloneStep,ValueError>{let indices=&self.indices;self.rows.commit(&mut base.style_runs,self.plan.origins().ok_or_else(Self::fault)?,g,capability,|at|{let run=&mut modified[indices[at.step.unwrap()]].patch.style_runs.as_mut().expect("owned style runs").rows[at.row].run;std::mem::replace(run,TextStyleRun::vacant())})}
 pub fn begin_close(&mut self){self.closing=true;self.plan.begin_close();self.rows.begin_close()}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.rows.terminal_is_empty(){return self.rows.close_demand(body)}if !self.plan.terminal_is_empty(){return Ok(self.plan.close_demand())}Ok(RetirementDemand{release_bytes:if self.indices.is_empty(){self.indices.capacity()*size_of::<usize>()}else{0},..Self::fixed()})}
 pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.rows.terminal_is_empty(){return self.rows.close_work_demand(body)}Ok(1)}
 pub fn requires_original_birth(&self)->bool{if !self.rows.terminal_is_empty(){return self.rows.requires_original_birth()}false}
 pub fn requires_original_control(&self)->bool{if !self.rows.terminal_is_empty(){return self.rows.requires_original_control()}false}
 pub fn close_original_birth_step(&mut self,loan:&mut OriginalRetirementCloseLoan<'_>,g:RetainedCloneGrant)->OriginalRetirementPoll{if !self.rows.terminal_is_empty(){return self.rows.close_original_birth_step(loan,g)}semio_framework_value::RetainedWorkPoll::Complete(semio_framework_value::RetainedWorkOutcome{result:Err(Self::fault().into()),work:0})}
 pub fn close_step(&mut self,loan:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,recover:bool,work:&mut u128)->Result<RetainedCloneStep,RetirementPollExit>{let g=effect.grant();if !self.closing{return Err(Self::fault().into())}let d=if recover{Default::default()}else{self.close_demand(body)?};if !recover&&!semio_framework_value::retained_clone::retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}if !self.rows.terminal_is_empty(){return self.rows.close_step(loan,body,effect,recover,work).map(|step|RetainedCloneStep::Progress(step.progress()))}*work+=1;if !self.plan.terminal_is_empty(){return self.plan.close_step(g).map(RetainedCloneStep::Progress).map_err(Into::into)}let mut p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};if self.indices.pop().is_some(){return Ok(RetainedCloneStep::Progress(p))}p.released_bytes=self.indices.capacity()*size_of::<usize>();self.indices=Vec::new();Ok(RetainedCloneStep::Complete(p))}
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.rows.terminal_is_empty()&&self.plan.terminal_is_empty()&&self.indices.is_empty()&&self.indices.capacity()==0}
}
impl Drop for StoryWork{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"story run originals require funded close")}}

impl StoryWork{
 pub fn final_run_count(&self)->usize{self.plan.origins().expect("admitted final style run extent").len()}
 pub fn final_run<'a>(&self,base:&'a TextStory,d:&'a LayoutStoriesDelta,index:usize)->&'a TextStyleRun{let at=self.plan.origins().expect("admitted final style run view")[index];match at.step{None=>&base.style_runs[at.row],Some(step)=>&d.modified[self.indices[step]].patch.style_runs.as_ref().expect("original final runs delta").rows[at.row].run}}
}
