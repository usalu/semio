//! ✏️ Actual procedural leaves move into their retained host or generation domain fold.
use super::retained_host_scratch::HostScratchWork;
use super::retained_generation_fold::GenerationFoldWork;
use super::GenerationPlayState;
use semio_framework_artifact_flow_flow::FlowHostSnapshot;
use semio_framework_job::{StepContext,OriginalTurnFailure};
use semio_framework_value::{OriginalRetirementPoll,RetainedAllocationEffect,RetirementDemand,RetainedCloneProgress,ValueError};
pub enum NativeProvisionalFold<X:NativeAdditionalFold>{Host(HostScratchWork),Generation(GenerationFoldWork),Additional(X)}
impl<X:NativeAdditionalFold> NativeProvisionalFold<X>{
 pub fn step(&mut self,host:&mut FlowHostSnapshot,generation:&mut GenerationPlayState,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{match self{Self::Host(work)=>work.step(host,cx),Self::Generation(work)=>work.step(generation,cx),Self::Additional(work)=>work.step(host,generation,cx)}}
 pub fn next_demands(&self,host:&FlowHostSnapshot,generation:&GenerationPlayState,copy:usize)->Result<RetirementDemand,ValueError>{match self{Self::Host(work)=>work.next_demands(host,copy),Self::Generation(work)=>work.next_demands(generation,copy),Self::Additional(work)=>work.next_demands(host,generation,copy)}}
 pub fn begin_close(&mut self){match self{Self::Host(work)=>work.begin_close(),Self::Generation(work)=>work.begin_close(),Self::Additional(work)=>work.begin_close()}}
 pub fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{match self{Self::Host(work)=>work.next_close_demands(copy),Self::Generation(work)=>work.next_close_demands(copy),Self::Additional(work)=>work.next_close_demands(copy)}}
 pub fn close_step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{match self{Self::Host(work)=>work.close_step(body,effect),Self::Generation(work)=>work.close_step(body,effect),Self::Additional(work)=>work.close_step(body,effect)}}
 pub fn terminal_is_empty(&self)->bool{match self{Self::Host(work)=>work.terminal_is_empty(),Self::Generation(work)=>work.terminal_is_empty(),Self::Additional(work)=>work.terminal_is_empty()}}
 pub fn step_progress(&self)->RetainedCloneProgress{match self{Self::Host(work)=>work.step_progress(),Self::Generation(work)=>work.step_progress(),Self::Additional(work)=>work.step_progress()}}
}
pub trait NativeProvisionalFolds<M>{
 type Additional:NativeAdditionalFold;
 fn into_fold_original(mutation:M,original:semio_framework_job::OriginalSourceBinding)->Result<NativeProvisionalFold<Self::Additional>,M>;
}

pub trait NativeAdditionalFold{
 fn step(&mut self,host:&mut FlowHostSnapshot,generation:&mut GenerationPlayState,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>;
 fn next_demands(&self,host:&FlowHostSnapshot,generation:&GenerationPlayState,copy:usize)->Result<RetirementDemand,ValueError>;
 fn begin_close(&mut self);
 fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>;
 fn close_step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll;
 fn terminal_is_empty(&self)->bool;
 fn step_progress(&self)->RetainedCloneProgress;
}
pub enum NeverAdditional{}
impl NativeAdditionalFold for NeverAdditional{
 fn step(&mut self,_host:&mut FlowHostSnapshot,_generation:&mut GenerationPlayState,_cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{match *self{}}
 fn next_demands(&self,_host:&FlowHostSnapshot,_generation:&GenerationPlayState,_copy:usize)->Result<RetirementDemand,ValueError>{match *self{}}
 fn begin_close(&mut self){match *self{}}
 fn next_close_demands(&self,_copy:usize)->Result<RetirementDemand,ValueError>{match *self{}}
 fn close_step(&mut self,_body:usize,_effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{match *self{}}
 fn terminal_is_empty(&self)->bool{match *self{}}
 fn step_progress(&self)->RetainedCloneProgress{match *self{}}
}
