//! 🏃️ Original-source batch supervision for bounded WFC search.
use super::*;
use crate::job::{HeadlessFailure,HeadlessOutcome,HeadlessSite,run_headless_supervised};
use semio_framework_job::{InteractiveJob,InteractiveJobStep,OriginalJobReturnClaim,StepContext,JobOutcomeBorrow,JobOutcomeDescriptor,JobOutcomeView,InteractiveJobCloseStep,OriginalTurnFailure,Operation,Generation,RevisionId,InteractiveStage,allocate_operation_id};
use semio_framework_value::{ValueError,RetainedCloneGrant};

/// 🎛️ Preserves the search producer while exposing paid supervision turns.
pub struct SearchJob<T:Topology+Clone> { inner:WfcJob<T>,yield_next:bool }
/// 🧳️ Retains failed search work and its same original teardown authority.
pub type SearchFailure<T:Topology+Clone+Send+'static> = HeadlessFailure<SearchJob<T>,SolveOutcome>;
/// 🏁️ Returns a search result only after its actual producer has retired.
pub type SearchResult<T:Topology+Clone+Send+'static> = Result<SolveOutcome,SearchFailure<T>>;

impl<T:Topology+Clone+Send> InteractiveJob for SearchJob<T> {
    fn step<'a>(&'a mut self,cx:&mut StepContext<'_>,_:&mut OriginalJobReturnClaim<'_>)->InteractiveJobStep<'a>{
        if self.yield_next {
            return InteractiveJobStep::from_callback(match JobOutcomeBorrow::admit_yield(cx) {
                Ok(Some(outcome))=>{self.yield_next=false;Ok(Some(outcome))},
                Ok(None)=>Ok(None),Err(error)=>Err(error.into())
            });
        }
        let before=cx.original_work_units_spent();
        InteractiveJobStep::from_callback(match self.inner.step_value(cx) {
            Ok(None)=>{self.yield_next=cx.original_work_units_spent()>before;Ok(None)},
            result=>result
        })
    }
    fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{self.inner.borrow_outcome(descriptor)}
    fn begin_close(&mut self){self.inner.begin_close();}
    fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{self.inner.close_step(cx)}
    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{self.inner.next_close_copy_byte_demand()}
    fn next_close_capacity_byte_demand(&self,n:usize)->Result<usize,ValueError>{self.inner.next_close_capacity_byte_demand(n)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{self.inner.next_close_release_byte_demand()}
    fn next_close_depth_demand(&self)->Result<usize,ValueError>{self.inner.next_close_depth_demand()}
    fn terminal_is_empty(&self)->bool{self.inner.terminal_is_empty()}
}

fn report<T:Topology+Clone>(job:&WfcJob<T>,model:&CompiledModel,config:&SearchConfig,seed:u64,start:std::time::Instant,terminal:Event)->RunReport{
    let (observations,propagations,backtracks)=job.metrics();
    let metrics=Metrics {observations,propagations,backtracks,elapsed_millis:start.elapsed().as_millis() as u64,..Default::default()};
    let mut events=Vec::new();
    if config.diag_level>=DiagLevel::Decisions {events.extend(job.observed().iter().map(|&(node,chosen)|Event::Observed {node,chosen}));}
    if config.diag_level!=DiagLevel::Off {events.push(terminal);}
    RunReport {metrics,model_fingerprint:model.fingerprint(),seed,events}
}

pub(super) fn drive_batch_job<T:Topology+Clone+Send+'static>(model:&CompiledModel,topo:&T,config:&SearchConfig,seed:u64,init_domains:Option<&[PatternSet]>,fixed:&[(NodeId,PatternId)],cancel:Option<&CancelToken>)->SearchResult<T>{
    if config.mode==SearchMode::RestartOnly||config.nogood.enabled {return Ok(solve_inner(model,topo,config,seed,init_domains,fixed,cancel,None));}
    let start=std::time::Instant::now();
    let operation=Operation::new(allocate_operation_id(),RevisionId(0),Generation(0),seed);
    let job_config=WfcJobConfig {sampler:match config.sampler {ValueSampler::WeightedRoulette=>WfcSampler::WeightedRoulette,ValueSampler::Uniform=>WfcSampler::Uniform}};
    let job=SearchJob {inner:WfcJob::new(operation,model.clone(),topo.clone(),job_config,init_domains.map(<[PatternSet]>::to_vec),fixed.to_vec()),yield_next:false};
    let site=HeadlessSite {site:"wfc.search.batch",label:"wfc-search-batch",stage:InteractiveStage::UserVisibleSimStep,fuel_per_step:4096,step_budget_us:5_000,retained_stock:RetainedCloneGrant {maximum_items:16_777_216,maximum_copy_bytes:1_073_741_824,maximum_capacity_bytes:1_073_741_824,maximum_release_bytes:1_073_741_824,maximum_depth:256},lifetime_fuel:100_000_000,lifetime_budget_us:60_000_000};
    run_headless_supervised(job,operation,semio_framework_job::default_now_us,site,|job|{
        let (observations,_,backtracks)=job.inner.metrics();
        let cancelled=cancel.is_some_and(CancelToken::is_cancelled);
        let exceeded=config.budget.max_observations.is_some_and(|limit|observations>=limit)||config.budget.max_backtracks.is_some_and(|limit|backtracks>=limit)||config.budget.max_millis.is_some_and(|limit|start.elapsed().as_millis()>=u128::from(limit));
        if !cancelled&&!exceeded {return None;}
        let partial=partial_from_job(&job.inner);
        let report=report(&job.inner,model,config,seed,start,Event::BudgetExceeded);
        Some(if cancelled {SolveOutcome::Cancelled {partial,report}} else {SolveOutcome::BudgetExceeded {partial,report}})
    },|job,outcome|{
        Ok(match outcome {
            HeadlessOutcome::Complete(bytes)=>{
                let text=std::str::from_utf8(bytes).map_err(|_|"wfc-search-output-not-utf8".to_string())?;
                let commit=semio_framework_pack_json::from_json_str::<crate::job::WfcCommit>(text,semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|_|"wfc-search-output-invalid".to_string())?;
                SolveOutcome::Solved(Solution {assignment:commit.assignment.into_iter().map(PatternId).collect(),report:report(&job.inner,model,config,seed,start,Event::Solved)})
            },
            HeadlessOutcome::Fault(detail)=>{
                let report=report(&job.inner,model,config,seed,start,Event::Contradiction {node:NodeId(0)});
                if detail==b"wfc-unsatisfiable" {SolveOutcome::Unsatisfiable(UnsatReport {proven:true,report})} else {SolveOutcome::Contradiction(ContradictionReport {node:NodeId(0),report})}
            },
            HeadlessOutcome::Cancelled=>SolveOutcome::Cancelled {partial:partial_from_job(&job.inner),report:report(&job.inner,model,config,seed,start,Event::BudgetExceeded)}
        })
    })
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
