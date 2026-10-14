//! 🚂️ The instance-owned geometry engine: the typed inference cache, the retained base snapshot and the stepped run.

use semio_s_artifact_procedural_3d_geometry_value::WidgetEvaluation;
use super::{Generation3dGeometry, GeometryInput};
use semio_s_artifact_procedural_3d_model::catalogue::Catalogue;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use protocol::{infer_field_step, InferenceCache, InferenceCacheConfig, InferenceCacheStats, InferenceCursor, InferenceValues, InferenceWorkFailure};
use std::mem::ManuallyDrop;
use semio_framework_job::{StepContext,OriginalCallerSourceLoan};
use semio_framework_value::{RetainedCloneProgress,ValueError,ValueRefusalKind,retirement::{controlled::ControlledRetirement}};
use std::sync::Arc;

/// ⚖️ The bytes the engine's inference cache may hold.
pub const GEOMETRY_CACHE_BUDGET_BYTES: usize = 64 * 1024 * 1024;

/// 🗃️ How a run uses the cache: `Incremental` reads and writes it, `Cold` clears it first, `Bypass` neither reads nor writes it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CacheMode {
    Incremental,
    Cold,
    Bypass,
}

/// 🚫️ Why an engine call could not advance.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EngineError {
    NotStarted,
    Busy,
}

impl std::fmt::Display for EngineError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotStarted => write!(formatter, "the geometry engine has no run"),
            Self::Busy => write!(formatter, "the geometry engine is busy"),
        }
    }
}

impl std::error::Error for EngineError {}

/// 📋️ What one engine step did.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EngineStep {
    pub done: bool,
    pub fuel_used: usize,
    pub computed: usize,
    pub hits: usize,
    pub completed: usize,
    pub total: usize,
    pub fraction: f32,
}

/// 📊️ Lifetime counters of one engine.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct EngineTotals {
    pub runs: u64,
    pub computed: u64,
    pub hits: u64,
}

#[derive(semio_framework_value::RetireOwned)]
struct Base {
    digest: String,
    snapshot: Generation3dSnapshot,
}

struct Run {
    cursor: InferenceCursor<String>,
    values: InferenceValues<Arc<WidgetEvaluation>>,
    bypass: bool,
    cancellation_id: String,
}

type BaseClose = (Option<Base>,Option<String>);

struct Owners {
    base:Option<Base>,
    run:Option<Run>,
    closing:Option<ControlledRetirement<BaseClose>>,
    source:Option<OriginalCallerSourceLoan>,
    scope:Option<Source>,
    admission_identity:Option<AdmissionIdentity>,
}

#[derive(PartialEq,Eq)]
struct AdmissionIdentity{snapshot:usize,digest:usize,digest_bytes:usize,cancellation:usize,cancellation_bytes:usize}
impl AdmissionIdentity{fn of(request:&EngineAdmission)->Option<Self>{let(digest,snapshot,cancellation)=request.payload.as_ref()?;Some(Self{snapshot:snapshot as*const Generation3dSnapshot as usize,digest:digest.as_ptr()as usize,digest_bytes:digest.len(),cancellation:cancellation.as_ptr()as usize,cancellation_bytes:cancellation.len()})}}

struct Source {operation:u64,generation:u64,parent:usize,recipient:usize,clock:fn()->Option<u64>,clock_binding:Option<u64>,deadline:u64}
impl Source {
    fn capture(cx:&StepContext<'_>)->Result<Self,ValueError>{Ok(Self{operation:cx.operation().0,generation:cx.generation().0,parent:cx.original_source_parent()?.source_identity()?,recipient:cx.original_recipient_identity(),clock:cx.clock_source(),clock_binding:cx.original_clock_binding(),deadline:cx.original_absolute_deadline_us()})}
    fn matches(&self,cx:&StepContext<'_>)->Result<bool,ValueError>{Ok(self.operation==cx.operation().0&&self.generation==cx.generation().0&&self.parent==cx.original_source_parent()?.source_identity()?&&self.recipient==cx.original_recipient_identity()&&std::ptr::fn_addr_eq(self.clock,cx.clock_source())&&self.clock_binding==cx.original_clock_binding()&&self.deadline==cx.original_absolute_deadline_us())}
}

/// 📥️ The actual request donor survives a refused engine admission without copying its snapshot or IDs.
pub struct EngineAdmission {payload:ManuallyDrop<Option<(String,Generation3dSnapshot,String)>>,mode:CacheMode,closing:Option<ControlledRetirement<(String,Generation3dSnapshot,String)>>,scope:Option<Source>}
impl EngineAdmission {
    pub fn new(digest:String,snapshot:Generation3dSnapshot,mode:CacheMode,cancellation_id:String)->Self{Self{payload:ManuallyDrop::new(Some((digest,snapshot,cancellation_id))),mode,closing:None,scope:None}}
    pub fn digest(&self)->Option<&str>{self.payload.as_ref().map(|(digest,_,_)|digest.as_str())}
    pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,InferenceWorkFailure>{
        if let Some(source)=self.scope.as_ref(){if !source.matches(cx)?{return Err(changed().into())}}else{self.scope=Some(Source::capture(cx)?);}
        if self.closing.is_none()&&self.payload.is_some(){if !metadata(cx,size_of::<(String,Generation3dSnapshot,String)>())?{return Ok(false)}let original=self.payload.take().unwrap();match ControlledRetirement::new(original){Ok(owner)=>self.closing=Some(owner),Err((error,original))=>{*self.payload=Some(original);return Err(error.into())}}return Ok(false)}
        if let Some(owner)=self.closing.as_mut(){if !owner.terminal_is_empty(){let demand=owner.selected_demand(cx.retained_grant().maximum_copy_bytes)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|owner.step(grant))?;return Ok(false)}self.closing=None;}self.scope=None;Ok(true)
    }
    pub fn terminal_is_empty(&self)->bool{self.payload.is_none()&&self.closing.is_none()}
}
impl Drop for EngineAdmission{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"geometry request donor requires paid original closure");}}

/// 🚂️ One original inference source owns its retained snapshot and every completed evaluation.
pub struct GeometryEngine {
    catalogue:Arc<Catalogue>,
    computes:Arc<dyn semio_s_artifact_procedural_3d_geometry_compute::GeometryComputeContext>,
    cache:InferenceCache,
    owners:ManuallyDrop<Owners>,
    totals:EngineTotals,
    pub(super) failure:ManuallyDrop<Option<InferenceWorkFailure>>,
}
impl Drop for GeometryEngine{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"geometry engine requires paid same-caller closure");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.owners)}}}}

fn changed()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"geometry engine original caller or owner changed")}
fn metadata(cx:&mut StepContext<'_>,bytes:usize)->Result<bool,InferenceWorkFailure>{let grant=cx.retained_grant();if cx.retirement_should_yield()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)}

impl GeometryEngine {
    /// 🏗️ An engine whose cache holds at most `budget_bytes`; a zero budget disables the cache.
    pub fn new(catalogue: Arc<Catalogue>, computes:Arc<dyn semio_s_artifact_procedural_3d_geometry_compute::GeometryComputeContext>, budget_bytes: usize) -> Self {
        let config = InferenceCacheConfig { enabled: budget_bytes > 0, budget_bytes, record_stats: true, ..Default::default() };
        Self { catalogue, computes, cache: semio_framework_async::poll::resolve_ready(InferenceCache::new(config)), owners:ManuallyDrop::new(Owners{base:None,run:None,closing:None,source:None,scope:None,admission_identity:None}), totals: EngineTotals::default(),failure:ManuallyDrop::new(None) }
    }

    /// 🧷️ Whether the retained run of the request with this digest is open: neither finished nor replaced. A cancelled run stays open, so continuing it reports the cancellation.
    pub fn is_running(&self, digest: &str) -> bool {
        self.owners.base.as_ref().is_some_and(|base| base.digest == digest) && self.owners.run.as_ref().is_some_and(|run| !run.cursor.is_done())
    }

    /// 🧷️ Whether the retained base belongs to the request with this digest.
    pub fn holds(&self, digest: &str) -> bool {
        self.owners.base.as_ref().is_some_and(|base| base.digest == digest)
    }

    /// 🎟️ Admits one actual retained snapshot and request IDs under their original source.
    pub fn start_original_step(&mut self,donor:&mut Option<EngineAdmission>,cx:&mut StepContext<'_>)->Result<bool,InferenceWorkFailure>{
        self.validate(cx)?;
        if self.owners.base.is_some()||self.owners.run.is_some()||self.owners.closing.is_some()||self.failure.is_some(){return Err(changed().into())}
        let request=donor.as_ref().ok_or_else(changed)?;
        let identity=AdmissionIdentity::of(request).ok_or_else(changed)?;
        if request.closing.is_some()||self.owners.admission_identity.as_ref().is_some_and(|original|*original!=identity){return Err(changed().into())}
        if cx.should_yield()||cx.is_cancelled(){return Ok(false)}
        if self.owners.source.is_none(){let scope=Source::capture(cx)?;let parent=cx.original_source_parent()?;let Some(source)=parent.admit_loan(cx)?else{return Ok(false)};self.owners.source=Some(source);self.owners.scope=Some(scope);self.owners.admission_identity=Some(identity);return Ok(false)}
        if !metadata(cx,size_of::<Base>()+size_of::<Run>())?{return Ok(false)}
        let mut request=donor.take().unwrap();let(digest,snapshot,cancellation_id)=request.payload.take().unwrap();
        self.owners.base=Some(Base{digest,snapshot});self.owners.run=Some(Run{cursor:InferenceCursor::new(),values:InferenceValues::new(),bypass:request.mode==CacheMode::Bypass,cancellation_id});self.owners.admission_identity=None;self.totals.runs+=1;Ok(true)
    }

    fn validate(&self,cx:&StepContext<'_>)->Result<(),InferenceWorkFailure>{if let Some(source)=self.owners.scope.as_ref(){if !source.matches(cx)?{return Err(changed().into())}}Ok(())}

    /// ♻️ Closes compute, plan, typed values, snapshot and request IDs before returning the initiating source loan.
    pub fn close_original_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,InferenceWorkFailure>{
        self.validate(cx)?;
        if self.failure.is_some(){return Ok(false)}
        if let Some(run)=self.owners.run.as_mut(){
            run.cursor.cancel();
            if !run.cursor.close_original_plan_step(cx)?{return Ok(false)}
            if !run.values.close_step(cx)?{return Ok(false)}
            if !metadata(cx,size_of::<BaseClose>())?{return Ok(false)}
            let mut retired_run=self.owners.run.take().unwrap();let cancellation=Some(std::mem::take(&mut retired_run.cancellation_id));let payload=(self.owners.base.take(),cancellation);
            match ControlledRetirement::new(payload){Ok(owner)=>self.owners.closing=Some(owner),Err((error,(base,cancellation)))=>{self.owners.base=base;retired_run.cancellation_id=cancellation.unwrap();self.owners.run=Some(retired_run);return Err(error.into())}}return Ok(false)
        }
        if let Some(owner)=self.owners.closing.as_mut(){if !owner.terminal_is_empty(){let demand=owner.selected_demand(cx.retained_grant().maximum_copy_bytes)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|owner.step(grant))?;return Ok(false)}self.owners.closing=None;}
        if let Some(source)=self.owners.source.as_mut(){let parent=cx.original_source_parent()?;source.close_step(parent,cx)?;if !source.terminal_is_empty(){return Ok(false)}self.owners.source=None;}
        self.owners.scope=None;self.owners.admission_identity=None;Ok(true)
    }

    /// 🪜️ Spends at most `fuel` units on the run. A finished run becomes the retained evaluation.
    pub fn step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<EngineStep,InferenceWorkFailure> {
        self.validate(cx)?;
        let owners=&mut *self.owners;
        let (Some(base), Some(run)) = (owners.base.as_ref(), owners.run.as_mut()) else {return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"geometry engine has no original run").into())};
        let cache = if run.bypass { None } else { Some(&mut self.cache) };
        let report = infer_field_step::<GeometryInput<'_>, Generation3dGeometry>(&GeometryInput::new(&base.snapshot,&self.catalogue,self.computes.as_ref()), cache, &mut run.cursor, &mut run.values,cx)?;
        self.totals.computed += report.computed as u64;
        self.totals.hits += report.hits as u64;
        Ok(EngineStep { done: report.done, fuel_used: report.fuel_used, computed: report.computed, hits: report.hits, completed: run.cursor.completed(), total: run.cursor.total(), fraction: report.progress })
    }

    /// 🪢️ Retains the whole field refusal until the actual caller pays its original envelope handoff.
    pub fn hold_failure(&mut self,original:InferenceWorkFailure){assert!(self.failure.is_none(),"original geometry engine already retains its field refusal");*self.failure=Some(original);}
    pub fn has_failure(&self)->bool{self.failure.is_some()}
    pub fn admit_failure(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<semio_framework_job::OriginalTurnFailure>,semio_framework_job::OriginalTurnFailure>{
        protocol::admit_inference_failure(&mut self.failure,cx)
    }
    /// 🛑️ Cancels the run in flight: the in-flight widget job is cancelled and finished widgets stay valid.
    pub fn cancel(&mut self) {
        if let Some(run) = self.owners.run.as_mut() {
            run.cursor.cancel();
        }
    }

    /// 🛑️ Cancels the run in flight when it was started under `cancellation_id`; answers whether it was.
    pub fn cancel_run(&mut self, cancellation_id: &str) -> bool {
        match self.owners.run.as_mut().filter(|run| run.cancellation_id == cancellation_id && !run.cursor.is_done()) {
            Some(run) => {
                run.cursor.cancel();
                true
            }
            None => false,
        }
    }

    /// 📦️ Borrows the actual value row through its retained snapshot's source ordinal.
    pub fn evaluation(&self,widget:&str)->Option<&Arc<WidgetEvaluation>>{self.evaluations().find(|(key,_)|key.as_str()==widget).map(|(_,value)|value)}

    /// 📦️ Borrows source IDs and completed values without allocating keys or duplicating shared payloads.
    pub fn evaluations(&self)->impl Iterator<Item=(&String,&Arc<WidgetEvaluation>)>{
        self.owners.base.as_ref().zip(self.owners.run.as_ref()).into_iter().flat_map(move|(base,run)|run.values.iter().filter_map(move|(source,value)|super::source_widget_key(&base.snapshot,source).map(|key|(key,value))))
    }

    /// 🔗️ Formats a cold diagnostic hash from the existing borrowed source key.
    pub fn dependency_hash(&self,widget:&str)->Option<String>{let(key,_)=self.evaluations().find(|(key,_)|key.as_str()==widget)?;self.owners.run.as_ref()?.cursor.hash(key).map(|hash|hash.0.iter().map(|byte|format!("{byte:02x}")).collect())}

    /// 🧮️ Finished and total widgets of the current run.
    pub fn progress(&self) -> (usize, usize) {
        self.owners.run.as_ref().map_or((0, 0), |run| (run.cursor.completed(), run.cursor.total()))
    }

    /// 📊️ The cache's hit, miss and eviction counters.
    pub fn cache_stats(&self) -> InferenceCacheStats {
        semio_framework_async::poll::resolve_ready(self.cache.stats())
    }

    /// 📊️ Lifetime counters of the engine: runs started, widgets computed and cache hits served.
    pub fn totals(&self) -> EngineTotals {
        self.totals
    }

    /// 📏️ The bytes the cache currently holds.
    pub fn cache_bytes(&self) -> usize {
        self.cache.used_bytes()
    }

    /// 🧊️ An empty engine needs no cold cleanup; original sources remain held for their paid closure.
    pub fn retire_cold(&mut self){if self.terminal_is_empty(){semio_framework_async::poll::resolve_ready(self.cache.clear());}}

    /// 🧾️ Every original request, value and source loan has returned through its paid closure.
    pub fn terminal_is_empty(&self)->bool{self.failure.is_none()&&self.owners.base.is_none()&&self.owners.run.is_none()&&self.owners.closing.is_none()&&self.owners.source.is_none()&&self.owners.scope.is_none()&&self.owners.admission_identity.is_none()}

    /// 📸️ The base snapshot of the current run.
    pub fn base(&self) -> Option<&Generation3dSnapshot> {
        self.owners.base.as_ref().map(|base| &base.snapshot)
    }
}

