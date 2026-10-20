//! 🎛️ Guest runtimes and test doubles: the guest runtimes enum with its runtime implementation and conversions and the recording runtime test double. See [`super`].

use super::*;

//#region 🎭️GuestRuntime
//#region 🐎️WasmtimeRuntime
//#region 🎛️GuestRuntimes
/// 🎛️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (host-dedyn): the closed-set enum replacing every
/// `Arc<dyn GuestRuntime>` in this crate (O1/R1 — `async fn` in a trait cannot be `dyn`-dispatched).
/// Hand-written, not `#[dyn_enum]`/`dyn_enum_close!` (`semio-framework-dispatch-macros`): TWO of the
/// three variants are `#[cfg(any(test, feature = "testkit"))]`-gated, and the macro's own acceptance suite
/// (`📓️terra-dyn-enum-macro-report.md`) never exercised cfg-gated variants at all — a hand-written
/// match-delegation is the lower-risk choice here, exactly the fallback the packet brief
/// anticipates ("if cfg-gated variants defeat the macro, hand-write the enum ... say so").
///
/// `Mock`/`Recording` wrap an `Arc<..>` of their concrete type, not the bare value — many existing
/// tests hold their OWN `Arc<MockGuestRuntime>`/`Arc<RecordingRuntime>` to call inherent,
/// non-`GuestRuntime` methods (`script_turn`, `observed_events`, `last_turn_budget`, ...) on the
/// SAME instance a `PluginInstanceHandle`/`ShardLoop` is driving; a bare-value variant would force
/// every one of those call sites to construct a SECOND, disconnected instance instead of sharing
/// state through the clone `Arc` already gives for free. `Wasmtime` needs no such indirection — no
/// test in this crate keeps a second handle to the SAME `WasmtimeRuntime` for out-of-band inherent
/// calls — so it stays a bare value, one indirection (the enum's own `Arc<GuestRuntimes>` wrapper at
/// every call site) instead of two.
#[cfg_attr(target_pointer_width = "64", expect(clippy::large_enum_variant, reason = "The runtime is retained behind shared Arc owners; another box would add indirection and allocation to the dominant native runtime."))]
pub enum GuestRuntimes {
    Owned(OwnedRuntime),
    Wasmtime(WasmtimeRuntime),
    /// 🔮️ a later packet adds `AsyncActor(AsyncPluginRuntime)` here, backed by wasmtime's
    /// `component-model-async` — do not mount `⏳️runtime/🦀️.rs` from this packet (out of scope, needs a
    /// rewritten schema; see this ticket's brief).
    #[cfg(any(test, feature = "testkit"))]
    Mock(Arc<MockGuestRuntime>),
    #[cfg(any(test, feature = "testkit"))]
    Recording(Arc<crate::RecordingRuntime>),
}

impl GuestRuntime for GuestRuntimes {
    async fn admit_actor_context(&self,inst:&mut GuestInstance,input:semio_framework::kernel::RetainedTurnInput,budget:Budget,identity:&mut semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)->Result<semio_framework_actor::retained_context::OriginalActorAdmissionReply,TurnFault>{match self{Self::Owned(runtime)=>runtime.admit_actor_context(inst,input,budget,identity).await,Self::Wasmtime(runtime)=>runtime.admit_actor_context(inst,input,budget,identity).await,#[cfg(any(test, feature = "testkit"))]Self::Mock(runtime)=>runtime.admit_actor_context(inst,input,budget,identity).await,#[cfg(any(test, feature = "testkit"))]Self::Recording(runtime)=>runtime.admit_actor_context(inst,input,budget,identity).await}}
    async fn close_actor_context(&self,inst:&mut GuestInstance,input:semio_framework::kernel::RetainedTurnInput,budget:Budget,identity:&mut semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)->Result<semio_framework_actor::retained_context::OriginalActorCloseReply,TurnFault>{match self{Self::Owned(runtime)=>runtime.close_actor_context(inst,input,budget,identity).await,Self::Wasmtime(runtime)=>runtime.close_actor_context(inst,input,budget,identity).await,#[cfg(any(test, feature = "testkit"))]Self::Mock(runtime)=>runtime.close_actor_context(inst,input,budget,identity).await,#[cfg(any(test, feature = "testkit"))]Self::Recording(runtime)=>runtime.close_actor_context(inst,input,budget,identity).await}}

    async fn compile(&self, package: &PackageRef, bytes: &[u8]) -> Result<CompiledHandle, PluginHostError> {
        match self {
            Self::Owned(r) => r.compile(package, bytes).await,
            Self::Wasmtime(r) => r.compile(package, bytes).await,
            #[cfg(any(test, feature = "testkit"))]
            Self::Mock(r) => r.compile(package, bytes).await,
            #[cfg(any(test, feature = "testkit"))]
            Self::Recording(r) => r.compile(package, bytes).await,
        }
    }

    async fn instantiate(&self, compiled: &CompiledHandle, actor: RuntimeActorId, caps: &[BrokerCapabilityGrant], budget: &Budget) -> Result<GuestInstance, PluginHostError> {
        match self {
            Self::Owned(r) => r.instantiate(compiled, actor, caps, budget).await,
            Self::Wasmtime(r) => r.instantiate(compiled, actor, caps, budget).await,
            #[cfg(any(test, feature = "testkit"))]
            Self::Mock(r) => r.instantiate(compiled, actor, caps, budget).await,
            #[cfg(any(test, feature = "testkit"))]
            Self::Recording(r) => r.instantiate(compiled, actor, caps, budget).await,
        }
    }

    async fn execute_turn(&self, inst: &mut GuestInstance, events: &[Event], budget: Budget, identity: &mut semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<TurnResult, TurnFault> {
        match self {
            Self::Owned(r) => r.execute_turn(inst, events, budget, identity).await,
            Self::Wasmtime(r) => r.execute_turn(inst, events, budget, identity).await,
            #[cfg(any(test, feature = "testkit"))]
            Self::Mock(r) => r.execute_turn(inst, events, budget, identity).await,
            #[cfg(any(test, feature = "testkit"))]
            Self::Recording(r) => r.execute_turn(inst, events, budget, identity).await,
        }
    }

    async fn start_job(&self,inst:&mut GuestInstance,source:Option<OriginalJobSource>,budget:OriginalJobBudget,execution:OriginalGuestExecutionBudget)->OriginalGuestJobCall<OriginalJobStartReply>{match self{Self::Owned(runtime)=>runtime.start_job(inst,source,budget,execution).await,Self::Wasmtime(runtime)=>runtime.start_job(inst,source,budget,execution).await,#[cfg(any(test, feature = "testkit"))]Self::Mock(runtime)=>runtime.start_job(inst,source,budget,execution).await,#[cfg(any(test, feature = "testkit"))]Self::Recording(runtime)=>runtime.start_job(inst,source,budget,execution).await}}
    async fn step_job(&self,inst:&mut GuestInstance,job:u64,budget:OriginalJobBudget,execution:OriginalGuestExecutionBudget)->OriginalGuestJobCall<OriginalJobStepReply<OriginalJobOutcome>>{match self{Self::Owned(runtime)=>runtime.step_job(inst,job,budget,execution).await,Self::Wasmtime(runtime)=>runtime.step_job(inst,job,budget,execution).await,#[cfg(any(test, feature = "testkit"))]Self::Mock(runtime)=>runtime.step_job(inst,job,budget,execution).await,#[cfg(any(test, feature = "testkit"))]Self::Recording(runtime)=>runtime.step_job(inst,job,budget,execution).await}}
    async fn read_job_output(&self,inst:&mut GuestInstance,job:u64,budget:OriginalJobBudget,execution:OriginalGuestExecutionBudget)->OriginalGuestJobCall<OriginalJobOutputReply>{match self{Self::Owned(runtime)=>runtime.read_job_output(inst,job,budget,execution).await,Self::Wasmtime(runtime)=>runtime.read_job_output(inst,job,budget,execution).await,#[cfg(any(test, feature = "testkit"))]Self::Mock(runtime)=>runtime.read_job_output(inst,job,budget,execution).await,#[cfg(any(test, feature = "testkit"))]Self::Recording(runtime)=>runtime.read_job_output(inst,job,budget,execution).await}}
    async fn acknowledge_job_output(&self,inst:&mut GuestInstance,job:u64,budget:OriginalJobBudget,execution:OriginalGuestExecutionBudget)->OriginalGuestJobCall<OriginalJobOutputAckReply>{match self{Self::Owned(runtime)=>runtime.acknowledge_job_output(inst,job,budget,execution).await,Self::Wasmtime(runtime)=>runtime.acknowledge_job_output(inst,job,budget,execution).await,#[cfg(any(test, feature = "testkit"))]Self::Mock(runtime)=>runtime.acknowledge_job_output(inst,job,budget,execution).await,#[cfg(any(test, feature = "testkit"))]Self::Recording(runtime)=>runtime.acknowledge_job_output(inst,job,budget,execution).await}}
    async fn cancel_job(&self,inst:&mut GuestInstance,job:u64,budget:OriginalJobBudget,execution:OriginalGuestExecutionBudget)->OriginalGuestJobCall<OriginalJobCancelReply>{match self{Self::Owned(runtime)=>runtime.cancel_job(inst,job,budget,execution).await,Self::Wasmtime(runtime)=>runtime.cancel_job(inst,job,budget,execution).await,#[cfg(any(test, feature = "testkit"))]Self::Mock(runtime)=>runtime.cancel_job(inst,job,budget,execution).await,#[cfg(any(test, feature = "testkit"))]Self::Recording(runtime)=>runtime.cancel_job(inst,job,budget,execution).await}}
    async fn close_jobs(&self,inst:&mut GuestInstance,budget:OriginalJobBudget,execution:OriginalGuestExecutionBudget)->OriginalGuestJobCall<OriginalJobCancelReply>{match self{Self::Owned(runtime)=>runtime.close_jobs(inst,budget,execution).await,Self::Wasmtime(runtime)=>runtime.close_jobs(inst,budget,execution).await,#[cfg(any(test, feature = "testkit"))]Self::Mock(runtime)=>runtime.close_jobs(inst,budget,execution).await,#[cfg(any(test, feature = "testkit"))]Self::Recording(runtime)=>runtime.close_jobs(inst,budget,execution).await}}

    async fn checkpoint(&self, inst: &mut GuestInstance, identity: &mut semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<Vec<u8>, PluginHostError> {
        match self {
            Self::Owned(r) => r.checkpoint(inst, identity).await,
            Self::Wasmtime(r) => r.checkpoint(inst, identity).await,
            #[cfg(any(test, feature = "testkit"))]
            Self::Mock(r) => r.checkpoint(inst, identity).await,
            #[cfg(any(test, feature = "testkit"))]
            Self::Recording(r) => r.checkpoint(inst, identity).await,
        }
    }

    async fn restore(&self, inst: &mut GuestInstance, state: &[u8], identity: &mut semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), PluginHostError> {
        match self {
            Self::Owned(r) => r.restore(inst, state, identity).await,
            Self::Wasmtime(r) => r.restore(inst, state, identity).await,
            #[cfg(any(test, feature = "testkit"))]
            Self::Mock(r) => r.restore(inst, state, identity).await,
            #[cfg(any(test, feature = "testkit"))]
            Self::Recording(r) => r.restore(inst, state, identity).await,
        }
    }

    async fn drop_instance(&self, inst: GuestInstance) {
        match self {
            Self::Owned(r) => r.drop_instance(inst).await,
            Self::Wasmtime(r) => r.drop_instance(inst).await,
            #[cfg(any(test, feature = "testkit"))]
            Self::Mock(r) => r.drop_instance(inst).await,
            #[cfg(any(test, feature = "testkit"))]
            Self::Recording(r) => r.drop_instance(inst).await,
        }
    }
}

impl From<WasmtimeRuntime> for GuestRuntimes {
    fn from(r: WasmtimeRuntime) -> Self {
        Self::Wasmtime(r)
    }
}

impl From<OwnedRuntime> for GuestRuntimes {
    fn from(r: OwnedRuntime) -> Self {
        Self::Owned(r)
    }
}

#[cfg(any(test, feature = "testkit"))]
impl From<Arc<MockGuestRuntime>> for GuestRuntimes {
    fn from(r: Arc<MockGuestRuntime>) -> Self {
        Self::Mock(r)
    }
}

#[cfg(any(test, feature = "testkit"))]
impl From<Arc<crate::RecordingRuntime>> for GuestRuntimes {
    fn from(r: Arc<crate::RecordingRuntime>) -> Self {
        Self::Recording(r)
    }
}
//#endregion 🎛️GuestRuntimes
//#endregion 🐎️WasmtimeRuntime
//#endregion 🎭️GuestRuntime

/// 🛂️ Generated concurrent imports forward the real inline admission slot to the original caller pump.
impl semio_framework_plugin_host_wit::actor_bindings::semio::framework::pure::HostWithStore<ActorHostState> for wasm::component::HasSelf<ActorHostState> {
 async fn operation_begin(accessor:&Accessor<ActorHostState,Self>)->Result<u64,u32>{match semio_framework_plugin_host_operation::request(accessor,semio_framework_plugin_host_operation::OperationRequest::Begin).await{semio_framework_plugin_host_operation::OperationReply::Begin(value)=>value,_=>Err(4)}}
 async fn operation_progress(accessor:&Accessor<ActorHostState,Self>,completed:u64,total:u64,owned:u64)->u32{match semio_framework_plugin_host_operation::request(accessor,semio_framework_plugin_host_operation::OperationRequest::Progress{completed,total,owned}).await{semio_framework_plugin_host_operation::OperationReply::Code(value)=>value,_=>4}}
 async fn operation_allocation(accessor:&Accessor<ActorHostState,Self>,bytes:u64,owned:u64,next:u64,maximum:u64)->u32{match semio_framework_plugin_host_operation::request(accessor,semio_framework_plugin_host_operation::OperationRequest::Allocation{bytes,owned,next,maximum}).await{semio_framework_plugin_host_operation::OperationReply::Code(value)=>value,_=>4}}
 async fn operation_reserve_return(accessor:&Accessor<ActorHostState,Self>,kind:semio_framework_plugin_host_wit::actor_bindings::semio::framework::pure::OperationReturnAllocation,count:u64)->u32{match semio_framework_plugin_host_operation::request(accessor,semio_framework_plugin_host_operation::OperationRequest::ReserveReturn{kind:semio_framework_plugin_host_operation::allocation_kind(kind),count}).await{semio_framework_plugin_host_operation::OperationReply::Code(value)=>value,_=>4}}
 async fn operation_finish(accessor:&Accessor<ActorHostState,Self>,owned:u64)->u32{match semio_framework_plugin_host_operation::request(accessor,semio_framework_plugin_host_operation::OperationRequest::Finish{owned}).await{semio_framework_plugin_host_operation::OperationReply::Code(value)=>value,_=>4}}
}

//#region 🧪️TestDoubles
/// 🧪️ A `GuestRuntime` that records the EXACT `Budget`/`JobBudget` it was invoked with — unlike
/// `MockGuestRuntime` (owned by `🖥️host/🦀️.rs`, out of this packet's edit scope, and which
/// ignores its `budget` parameter entirely), this proves the property terra-shard-grants demanded:
/// "a Grant's budget is what the turn actually executes under (prove the constants are gone, not
/// merely unused)". `pub(crate)`, not private — moved out of `mod tests` so `GuestRuntimes::
/// Recording` (`🖥️host/🦀️.rs`) can name it.
#[cfg(any(test, feature = "testkit"))]
pub struct RecordingRuntime {
    pub last_turn_budget: Mutex<Option<Budget>>,
    pub last_job_budget: Mutex<Option<JobBudget>>,
}

#[cfg(any(test, feature = "testkit"))]
impl RecordingRuntime {
    pub async fn new() -> Self {
        Self { last_turn_budget: Mutex::new(None), last_job_budget: Mutex::new(None) }
    }
}

#[cfg(any(test, feature = "testkit"))]
impl GuestRuntime for RecordingRuntime {
    async fn admit_actor_context(&self,inst:&mut GuestInstance,input:semio_framework::kernel::RetainedTurnInput,_budget:Budget,_identity:&mut semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)->Result<semio_framework_actor::retained_context::OriginalActorAdmissionReply,TurnFault>{let GuestInstanceState::Mock(state)=&mut inst.state else{return Err(semio_framework_plugin_guest_contract::identity_turn_fault(semio_framework_plugin_guest_contract::operation_authority::refusal(4)))};Ok(semio_framework_actor::retained_context::OriginalActorAdmissionReply::borrow_original(&state.actor_context.admit_step(input,_budget.retained_policy,semio_framework_job::logical_now_us)))}
    async fn close_actor_context(&self,inst:&mut GuestInstance,input:semio_framework::kernel::RetainedTurnInput,_budget:Budget,_identity:&mut semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)->Result<semio_framework_actor::retained_context::OriginalActorCloseReply,TurnFault>{let GuestInstanceState::Mock(state)=&mut inst.state else{return Err(semio_framework_plugin_guest_contract::identity_turn_fault(semio_framework_plugin_guest_contract::operation_authority::refusal(4)))};Ok(semio_framework_actor::retained_context::OriginalActorCloseReply::borrow_original(&state.actor_context.close_step(input,_budget.retained_policy)))}

    async fn compile(&self, package: &PackageRef, _bytes: &[u8]) -> Result<crate::CompiledHandle, PluginHostError> {
        Ok(crate::CompiledHandle { package_hash: package.hash.0, component: None, owned: None })
    }
    async fn instantiate(&self, _compiled: &crate::CompiledHandle, actor: ActorId, _caps: &[semio_framework::kernel::BrokerCapabilityGrant], _budget: &Budget) -> Result<GuestInstance, PluginHostError> {
        Ok(GuestInstance { actor, state: GuestInstanceState::Mock(crate::MockInstanceState::default()) })
    }
    async fn drop_instance(&self, _inst: GuestInstance) {}
    async fn execute_turn(&self, _inst: &mut GuestInstance, _events: &[Event], budget: Budget, _identity: &mut semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<TurnResult, TurnFault> {
        *self.last_turn_budget.lock().expect("lock") = Some(budget);
        let retained_receipt = budget.retained.return_original(semio_framework_value::RetainedCloneProgress::default()).expect("recorded idle turn spends nothing");
        Ok(TurnResult {
            retained_receipt,
            ui_patches: semio_framework::kernel::UiTurnPatches::default(),
            effects: vec![],
            presence: vec![],
            next_wake: None,
            status: semio_framework::kernel::TurnStatus::Idle,
            fuel_used: 0,
            command_ingress: semio_framework::kernel::CommandIngressStatus::Idle,
            cold_pair_ingress: semio_framework::kernel::ColdPairIngressStatus::Idle,
            lifecycle_receipt: None,
            ui_patch_receipt: None,
        })
    }
    async fn start_job(&self, _inst: &mut GuestInstance, _job: u64, _kind: &str, _input: Vec<u8>) -> Result<(), TurnFault> {
        Ok(())
    }
    async fn step_job(&self, _inst: &mut GuestInstance, _job: u64, budget: JobBudget) -> Result<JobStep, TurnFault> {
        *self.last_job_budget.lock().expect("lock") = Some(budget);
        Ok(JobStep::Running { progress: None })
    }
    async fn cancel_job(&self, _inst: &mut GuestInstance, _job: u64) -> Result<(), TurnFault> {
        Ok(())
    }
    async fn checkpoint(&self, _inst: &mut GuestInstance, _identity: &mut semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<Vec<u8>, PluginHostError> {
        Ok(vec![])
    }
    async fn restore(&self, _inst: &mut GuestInstance, _state: &[u8], _identity: &mut semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), PluginHostError> {
        Ok(())
    }
}
//#endregion 🧪️TestDoubles
