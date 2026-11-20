//! 🪡️ Layout central apply stages authored sparse fields and moves untouched live ownership.
#[path="🧱️scalars/🦀️.rs"]pub mod scalars;
#[path="🗂️collections/🦀️.rs"]pub mod collections;
pub use semio_s_artifact_layout_layout_diff_plan::final_list;
#[path="📄️page/🦀️.rs"]pub mod page;
#[path="📖️story/🦀️.rs"]pub mod story;
#[path="🪢️nested/🦀️.rs"]pub mod nested;
#[path="🧾️data/🦀️.rs"]pub mod data;
pub use semio_s_artifact_layout_layout_diff_plan::page_cache;
use crate::LayoutDiff;
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use semio_os_kernel as protocol;
use protocol::{RetainedDiffApply,RetainedDiffPreflightAdvance,RetainedDiagnosticPreparation,RetainedDiagnosticSpec,MutationMessage,MutationMessageLedgerRetirement,ApplyCapability,RetainedReply as Reply,RetainedDone as Done,RetainedFlight,retained_conclude,retained_settle};
use semio_framework_value::{RetainedAllocationEffect,OriginalRetirementCloseLoan,OriginalRetirementPoll,OriginalRetirementOutcome,OriginalRetirementFailure,RetirementPollExit,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneRef,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use std::mem::{size_of,ManuallyDrop};
use scalars::RootScalars;
use collections::RootCollections;
use nested::NestedWork;
use data::DataWork;
#[path="👁️final-graph/🦀️.rs"]pub(crate) mod final_graph;
use final_graph::FinalGraphPreparation;
const META:usize=512;
fn fault()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"Layout retained central apply stage is invalid")}
fn fixed()->RetirementDemand{RetirementDemand{copy_bytes:META,depth:1,..Default::default()}}
fn nested_demand(mut d:RetirementDemand)->RetirementDemand{d.copy_bytes+=META;d.depth+=1;d}
fn child(g:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-META,maximum_depth:g.maximum_depth-1,..g}}
fn receipt(mut p:RetainedCloneProgress)->RetainedCloneProgress{p.copied_items=p.copied_items.max(1);p.copied_bytes+=META;p}
fn refused(e:ValueError)->ValueError{let p=receipt(e.retained_progress());e.with_retained_progress(p)}
fn paid()->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:META,..Default::default()}}
/// 🧳 Original post fields, sparse source aliases, diagnostics and partial row owners close independently.
pub struct RetainedLayoutApply{flight:RetainedFlight,scalars:RootScalars,roots:RootCollections,nested:NestedWork,data:DataWork,final_graph:FinalGraphPreparation,reason:Option<&'static str>,diagnostic:RetainedDiagnosticPreparation<0>,messages:ManuallyDrop<Option<Vec<MutationMessage>>>,ledger:ManuallyDrop<Option<MutationMessageLedgerRetirement>>,phase:u8,index:usize,modification:usize,field:usize,prepared:bool,committed:bool,closing:bool,error:Option<&'static str>}
impl protocol::RetainedDiff<LayoutSnapshot> for LayoutDiff{type Apply=RetainedLayoutApply;}
impl RetainedLayoutApply{fn body(body:usize)->usize{body.saturating_sub(META)}fn refuse(&mut self,code:&'static str){self.error=Some(code);if !self.final_graph.terminal_is_empty(){self.final_graph.begin_close();self.phase=90}else{self.phase=100}}}
impl RetainedDiffApply<LayoutSnapshot,LayoutDiff> for RetainedLayoutApply{
 fn new()->Self{Self{flight:Default::default(),scalars:Default::default(),roots:Default::default(),nested:Default::default(),data:Default::default(),final_graph:Default::default(),reason:None,diagnostic:Default::default(),messages:ManuallyDrop::new(None),ledger:ManuallyDrop::new(None),phase:0,index:0,modification:0,field:0,prepared:false,committed:false,closing:false,error:None}}
 fn preflight_demand(&self,d:RetainedCloneRef<'_,LayoutDiff>,_:RetainedCloneRef<'_,LayoutSnapshot>,post:RetainedCloneRef<'_,LayoutSnapshot>,body:usize,max:u64)->Result<RetirementDemand,ValueError>{if self.closing||self.prepared{return Err(fault())}Ok(match self.phase{0=>if self.final_graph.sources_ready(){fixed()}else{nested_demand(self.final_graph.sources_demand(d,post,Self::body(body))?)},1=>nested_demand(self.scalars.preflight_demand(d,self.index,Self::body(body),max)?),2=>nested_demand(self.roots.preflight_demand(d,post.get(),self.index,Self::body(body),max)?),3=>nested_demand(self.nested.preflight_demand(d.get(),post.get(),&self.roots)?),4=>nested_demand(self.data.preflight_demand(d,post.get(),Self::body(body),max)?),5=>nested_demand(self.final_graph.demand(d,post,&self.roots,&self.nested,Self::body(body))?),6=>fixed(),90=>nested_demand(self.final_graph.close_demand(Self::body(body))?),100=>nested_demand(RetainedDiagnosticPreparation::<0>::start_demand()),101=>nested_demand(self.diagnostic.demand([],Self::body(body),max)?),102=>nested_demand(RetirementDemand{copy_bytes:size_of::<Vec<MutationMessage>>()*3,depth:1,..Default::default()}),_=>return Err(fault())})}
 fn preflight_work_demand(&self,d:RetainedCloneRef<'_,LayoutDiff>,original:RetainedCloneRef<'_,LayoutSnapshot>,post:RetainedCloneRef<'_,LayoutSnapshot>,body:usize,max:u64)->Result<u64,ValueError>{
  self.preflight_demand(d,original,post,body,max)?;
  if max==0{return Ok(0)}
  let child=Self::body(body);
  match self.phase{
   0 if !self.final_graph.sources_ready()=>self.final_graph.sources_work_demand(d,post,child),
   1=>self.scalars.preflight_work_demand(d,self.index,child,max),
   2=>self.roots.preflight_work_demand(d,post.get(),self.index,child,max),
   3=>self.nested.preflight_work_demand(d.get(),post.get(),&self.roots),
   4=>self.data.preflight_work_demand(d,post.get(),child,max),
   5=>self.final_graph.work_demand(d,post,&self.roots,&self.nested,child),
   90=>self.final_graph.close_work_demand(child),
   101=>self.diagnostic.advance_work_demand([],child,max),
   102=>self.diagnostic.take_work_demand(),_=>Ok(1)}
 }
 fn preflight_reply<'r>(&mut self,d:RetainedCloneRef<'_,LayoutDiff>,original:RetainedCloneRef<'_,LayoutSnapshot>,post:RetainedCloneRef<'_,LayoutSnapshot>,body:usize,max:u64,effect:&mut RetainedAllocationEffect<'_>,receiving:&mut semio_framework_value::NativeDecodeControl<'_>,reply:Reply<'r,RetainedDiffPreflightAdvance>,recover:bool)->Result<Done<'r,RetainedDiffPreflightAdvance>,ValueError>{
 let g=effect.grant();let base=effect.performed_work();let mut work=0u128;
 let result:Result<_,RetirementPollExit>=(||{
 if !recover{let demand=self.preflight_demand(d,original,post,body,max)?;let units=self.preflight_work_demand(d,original,post,body,max)?;if units==0||units>max||!protocol::retained_mutation_demand_fits(demand,g){return Ok(RetainedDiffPreflightAdvance::Progress(Default::default()))}receiving.checkpoint()?}
 let mut p=paid();match self.phase{
  0=>{if !self.final_graph.sources_ready(){p=receipt(self.final_graph.bind_sources(d,post,Self::body(body),&mut effect.nested(child(g)),recover,&mut work)?.progress)}else{work+=1;self.phase=1;self.index=0}},
  1=>{let step=self.scalars.preflight(d,self.index,Self::body(body),max,&mut effect.nested(child(g)),recover,&mut work)?;p=receipt(step.progress());if matches!(step,RetainedCloneStep::Complete(_)){self.index+=1;if self.index==6{self.phase=2;self.index=0}}},
  2=>{let step=self.roots.preflight(d,post.get(),self.index,Self::body(body),max,&mut effect.nested(child(g)),recover,&mut work)?;p=receipt(step.progress);if let Some(code)=step.error{self.refuse(code)}else if step.done{self.index+=1;if self.index==7{self.phase=3;self.index=0}}},
  3=>{let step=self.nested.preflight(d.get(),post.get(),&self.roots,&mut effect.nested(child(g)),&mut work)?;p=receipt(step.progress);if let Some(code)=step.error{self.refuse(code)}else if step.done{self.phase=4}},
  4=>{let step=self.data.preflight(d,post.get(),Self::body(body),max,&mut effect.nested(child(g)),recover,&mut work)?;p=receipt(step.progress);if let Some(code)=step.error{self.refuse(code)}else if step.done{self.phase=5}},
  5=>{let step=self.final_graph.advance(d,post,&self.roots,&self.nested,Self::body(body),&mut effect.nested(child(g)),recover,&mut work)?;p=receipt(step.progress);if step.done{if let Some(reason)=step.error{self.reason=Some(reason);self.refuse("layout.retained.final-graph")}else{self.phase=6}}},
  6=>{work+=1;self.prepared=true;self.phase=0;self.index=0;return Ok(RetainedDiffPreflightAdvance::Ready(p,true))},
  90=>{p=receipt(self.final_graph.close_step(None,Self::body(body),&mut effect.nested(child(g)),recover,&mut work)?.progress());if self.final_graph.terminal_is_empty(){self.phase=100}},
  100=>{work+=1;p=receipt(self.diagnostic.start(RetainedDiagnosticSpec{level:semio_framework_diagnostic::Severity::Fatal,code:self.error.unwrap_or("layout.retained.invariant"),message:self.reason.unwrap_or("The authored Layout diff cannot be committed under the retained ownership contract.")},child(g)).map_err(refused)?);self.phase=101},
  101=>{let step=protocol::retained_routed!(self.flight,effect,recover,101,body,max,None,work,|r| self.diagnostic.advance_into([],Self::body(body),max,&mut effect.nested(child(g)),r),self.diagnostic.recover_advance_into([],Self::body(body),max,&mut effect.nested(child(g)),r)).map_err(RetirementPollExit::from)?;p=receipt(step.progress());if matches!(step,RetainedCloneStep::Complete(_)){self.phase=102}},
  102=>{let turn=self.diagnostic.take(child(g));work+=turn.work;let(rows,progress)=turn.result?.ok_or_else(fault)?;p=receipt(progress);*self.messages=Some(rows);self.prepared=true;return Ok(RetainedDiffPreflightAdvance::Ready(p,false))},_=>return Err(fault().into())}Ok(RetainedDiffPreflightAdvance::Progress(p))})();
 retained_conclude(effect,base,result,work,RetainedDiffPreflightAdvance::progress,reply)
 }
 fn take_messages(&mut self)->Vec<MutationMessage>{self.messages.take().unwrap_or_default()}
 fn commit_demand(&self,d:RetainedCloneRef<'_,LayoutDiff>,_:RetainedCloneRef<'_,LayoutSnapshot>,_:&LayoutSnapshot,_:usize)->Result<RetirementDemand,ValueError>{if !self.prepared||self.committed||self.closing||self.error.is_some(){return Err(fault())}Ok(match self.phase{0=>nested_demand(self.nested.commit_demand(&self.roots)?),1=>nested_demand(self.roots.commit_demand(d.get(),self.index)?),2=>if self.index==7||self.modification==self.roots.modifications(self.index){fixed()}else{nested_demand(self.roots.patch_demand(self.index,self.modification,self.field)?)},3=>nested_demand(self.scalars.commit_demand(d.get(),self.index)?),4=>nested_demand(self.data.commit_demand(d.get())?),5=>fixed(),_=>return Err(fault())})}
 fn commit_work_demand(&self,d:RetainedCloneRef<'_,LayoutDiff>,original:RetainedCloneRef<'_,LayoutSnapshot>,post:&LayoutSnapshot,body:usize)->Result<u64,ValueError>{self.commit_demand(d,original,post,body)?;match self.phase{1=>self.roots.commit_work_demand(d.get(),self.index),3=>self.scalars.commit_work_demand(d.get(),self.index),4=>self.data.commit_work_demand(d.get()),_=>Ok(1)}}
 fn commit(&mut self,d:RetainedCloneRef<'_,LayoutDiff>,original:RetainedCloneRef<'_,LayoutSnapshot>,post:&mut LayoutSnapshot,effect:&mut RetainedAllocationEffect<'_>,capability:ApplyCapability)->OriginalRetirementOutcome{let g=effect.grant();let base=effect.performed_work();let mut work=0u128;let result:Result<_,OriginalRetirementFailure>=(||{let demand=self.commit_demand(d,original,post,g.maximum_copy_bytes)?;if !protocol::retained_mutation_demand_fits(demand,g){return Ok(RetainedCloneStep::Progress(Default::default()))}let mut p=paid();match self.phase{
  0=>{work+=1;let step=self.nested.commit(post,&mut self.roots,child(g),capability).map_err(refused)?;p=receipt(step.progress());if matches!(step,RetainedCloneStep::Complete(_)){self.phase=1;self.index=0}},
  1=>{let turn=self.roots.commit(d.get(),post,self.index,&mut effect.nested(child(g)),capability);work+=turn.work;let step=turn.result?;p=receipt(step.progress());if matches!(step,RetainedCloneStep::Complete(_)){self.index+=1;if self.index==7{self.phase=2;self.index=0;self.modification=0;self.field=0}}},
  2=>{work+=1;if self.index==7{self.phase=3;self.index=0}else if self.modification==self.roots.modifications(self.index){self.index+=1;self.modification=0;self.field=0}else{let step=self.roots.patch(post,self.index,self.modification,self.field,child(g),capability).map_err(refused)?;p=receipt(step.progress());self.field+=1;if self.field==RootCollections::fields(self.index)?{self.field=0;self.modification+=1}}},
  3=>{let turn=self.scalars.commit(d.get(),post,self.index,&mut effect.nested(child(g)),capability);work+=turn.work;let step=turn.result?;p=receipt(step.progress());if matches!(step,RetainedCloneStep::Complete(_)){self.index+=1;if self.index==8{self.phase=4}}},
  4=>{let turn=self.data.commit(d.get(),post,&mut effect.nested(child(g)),capability);work+=turn.work;let step=turn.result?;p=receipt(step.progress());if matches!(step,RetainedCloneStep::Complete(_)){self.phase=5}},5=>{work+=1;self.committed=true;return Ok(RetainedCloneStep::Complete(p))},_=>return Err(fault().into())}Ok(RetainedCloneStep::Progress(p))})();retained_settle(effect,base,result,work)}
 fn begin_close(&mut self){self.closing=true;self.final_graph.begin_close();self.scalars.begin_close();self.roots.begin_close();self.nested.begin_close();self.data.begin_close();self.diagnostic.begin_close()}
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.final_graph.terminal_is_empty(){return Ok(nested_demand(self.final_graph.close_demand(Self::body(body))?))}if !self.nested.terminal_is_empty(){return self.nested.close_demand(body)}if !self.roots.terminal_is_empty(){return self.roots.close_demand(body)}if !self.scalars.terminal_is_empty(){return self.scalars.close_demand(body)}if !self.data.terminal_is_empty(){return self.data.close_demand(body)}if !self.diagnostic.terminal_is_empty(){return self.diagnostic.close_demand(body)}if self.messages.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Vec<MutationMessage>>()+size_of::<MutationMessageLedgerRetirement>(),depth:1,..Default::default()})}if let Some(ledger)=self.ledger.as_ref(){return semio_framework_value::ErasedSnapshotRetirement::next_demand(ledger,body)}Ok(Default::default())}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.final_graph.terminal_is_empty(){return self.final_graph.close_work_demand(Self::body(body))}
  if !self.nested.terminal_is_empty(){return self.nested.close_work_demand(body)}
  if !self.roots.terminal_is_empty(){return self.roots.close_work_demand(body)}
  if !self.scalars.terminal_is_empty(){return self.scalars.close_work_demand(body)}
  if !self.data.terminal_is_empty(){return self.data.close_work_demand(body)}
  if !self.diagnostic.terminal_is_empty(){return self.diagnostic.close_work_demand(body)}Ok(1)}
 fn requires_original_birth(&self)->bool{if !self.final_graph.terminal_is_empty(){return false}
  if !self.nested.terminal_is_empty(){return self.nested.requires_original_birth()}
  if !self.roots.terminal_is_empty(){return self.roots.requires_original_birth()}
  if !self.scalars.terminal_is_empty(){return self.scalars.requires_original_birth()}
  if !self.data.terminal_is_empty(){return self.data.requires_original_birth()}false}
 fn requires_original_control(&self)->bool{if !self.final_graph.terminal_is_empty(){return false}
  if !self.nested.terminal_is_empty(){return self.nested.requires_original_control()}
  if !self.roots.terminal_is_empty(){return self.roots.requires_original_control()}
  if !self.scalars.terminal_is_empty(){return self.scalars.requires_original_control()}
  if !self.data.terminal_is_empty(){return self.data.requires_original_control()}false}
 fn close_original_birth_step(&mut self,loan:&mut OriginalRetirementCloseLoan<'_>,g:RetainedCloneGrant)->OriginalRetirementPoll{if !self.final_graph.terminal_is_empty(){return semio_framework_value::RetainedWorkPoll::Complete(semio_framework_value::RetainedWorkOutcome{result:Err(fault().into()),work:0})}
  if !self.nested.terminal_is_empty(){return self.nested.close_original_birth_step(loan,g)}
  if !self.roots.terminal_is_empty(){return self.roots.close_original_birth_step(loan,g)}
  if !self.scalars.terminal_is_empty(){return self.scalars.close_original_birth_step(loan,g)}
  if !self.data.terminal_is_empty(){return self.data.close_original_birth_step(loan,g)}
  semio_framework_value::RetainedWorkPoll::Complete(semio_framework_value::RetainedWorkOutcome{result:Err(fault().into()),work:0})}
 fn close_reply<'r>(&mut self,loan:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let g=effect.grant();let base=effect.performed_work();let mut work=0u128;let result:Result<_,RetirementPollExit>=(||{
  if !self.closing{return Err(fault().into())}
  let d=if recover{Default::default()}else{let d=self.close_demand(body)?;if !protocol::retained_mutation_demand_fits(d,g)||self.close_work_demand(body)?>effect.maximum_work_units(){return Ok(RetainedCloneStep::Progress(Default::default()))}d};
  if !self.final_graph.terminal_is_empty(){let step=self.final_graph.close_step(loan,Self::body(body),&mut effect.nested(child(g)),recover,&mut work)?;return Ok(RetainedCloneStep::Progress(receipt(step.progress())))}
  if !self.nested.terminal_is_empty(){let step=self.nested.close_step(loan,body,effect,recover,&mut work)?;return Ok(RetainedCloneStep::Progress(step.progress()))}
  if !self.roots.terminal_is_empty(){let step=self.roots.close_step(loan,body,effect,recover,&mut work)?;return Ok(RetainedCloneStep::Progress(step.progress()))}
  if !self.scalars.terminal_is_empty(){let step=self.scalars.close_step(loan,body,effect,recover,&mut work)?;return Ok(RetainedCloneStep::Progress(step.progress()))}
  if !self.data.terminal_is_empty(){let step=self.data.close_step(loan,body,effect,recover,&mut work)?;return Ok(RetainedCloneStep::Progress(step.progress()))}
  if !self.diagnostic.terminal_is_empty(){let step=protocol::retained_routed!(self.flight,effect,recover,103,body,0,None,work,|r| self.diagnostic.close_into(body,effect,r),self.diagnostic.recover_close_into(body,effect,r)).map_err(RetirementPollExit::from)?;return Ok(RetainedCloneStep::Progress(step.progress()))}
  if let Some(rows)=self.messages.take(){work+=1;*self.ledger=Some(MutationMessageLedgerRetirement::new(String::new(),rows));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
  if let Some(ledger)=self.ledger.as_mut(){let step=ledger.close_step(g)?;work+=u128::from(ledger.last_turn_work());if ledger.terminal_is_empty(){*self.ledger=None}return Ok(RetainedCloneStep::Progress(step.progress()))}
  Ok(RetainedCloneStep::Complete(Default::default()))})();retained_conclude(effect,base,result,work,|step|step.progress(),reply)
 }
 fn terminal_is_empty(&self)->bool{self.flight.idle()&&self.closing&&self.final_graph.terminal_is_empty()&&self.nested.terminal_is_empty()&&self.roots.terminal_is_empty()&&self.scalars.terminal_is_empty()&&self.data.terminal_is_empty()&&self.diagnostic.terminal_is_empty()&&self.messages.is_none()&&self.ledger.is_none()}
}
impl Drop for RetainedLayoutApply{fn drop(&mut self){let empty=<Self as RetainedDiffApply<LayoutSnapshot,LayoutDiff>>::terminal_is_empty(self);assert!(std::thread::panicking()||empty,"Layout central owner requires funded original cleanup");if empty{unsafe{ManuallyDrop::drop(&mut self.messages);ManuallyDrop::drop(&mut self.ledger)}}}}
