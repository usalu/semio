//! 📨️ The plugin artifact channel: pending exchanges, budgets and the channel implementation.

#[cfg(not(target_arch = "wasm32"))]
use semio_framework_plugin_host::GuestRuntime as _;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_plugin_host::GuestRuntimes;
#[cfg(not(target_arch = "wasm32"))]
use std::collections::HashMap;
#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::Arc;
#[cfg(not(target_arch = "wasm32"))]
use crate::artifact_app_frames::{CommandTurn, app_command_seq_mut, app_frame_is_transaction_terminal, app_frame_reply_seq, app_frame_tag, bind_inference_document, child_head_dependencies, decode_guest_fault, document_backbone_payload, effect_shape, ingress_fault_text, named_shapes, persistent_command_completion_port_ready, shell_app_frame_payload, shell_lane_fault};
#[cfg(not(target_arch = "wasm32"))]
use crate::compiled_components::{ComponentBytes, scoped_compiled_component, shared_plugin_runtime};
#[cfg(not(target_arch = "wasm32"))]
use crate::guest_document_codec::DOCUMENT_BACKBONE_BINDING_GENERATION;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_mcp_catalog::plugin_paths_and_descriptor::{PluginRegistryEntry, find_repo_root, resolve_plugin_wasm_path};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_mcp_foundation::errors::GatewayError;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_mcp_port::channel_port::{ACTIVATION_CANCELLED_FAULT_CODE, AGENT_LANE_UNCARRIED_FAULT_CODE, ActivationPhase, ActivationScope, Fault, InferCommand, PreparedOps};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_mcp_schema::schema::{ARTIFACT_INFERENCE_WIRE_VERSION, ArtifactInferenceBudgetV1, ArtifactInferenceCacheModeV1, ArtifactInferenceRequestV1, ArtifactInferenceResultV1};

//#region 🔖️ArtifactChannel

/// 🔌️ Real `crate::actions::ArtifactChannel` implementation — the seam `P6-actions-policy`'s
/// `ActionAdapter` drives (`🔀️dispatch/🦀️.rs`'s own module doc: "is P7's job when it
/// implements `ArtifactChannel` for real"). `ReadHistory` is driven for REAL over
/// `GuestRuntime::execute_turn` (one retained command page per turn, scan `TurnResult.effects` for
/// `Effect::Respond{req, result}` where `req.0 == seq`, decode `RequestOutcome::Ok` as a real
/// `store::AppFrame::HistorySnapshot` — exactly the "real body" `🏃️run/🦀️.rs`'s own
/// `WasmtimeNodeHost::exchange` doc comment describes and does not itself build). The mutation
/// variants (`PureCommand`/`TransactionPrepare`/`Commit`/`Rollback`/`Undo`/`Redo`) need a real
/// `manifest::ActionInvocation{address: ActionAddress{plugin_id, app_id, mode_id, window_kind_id,
/// window_instance_id, action_id}, arguments}` — every one of those five address fields beyond
/// `plugin_id`/`app_id` is normally supplied by a LIVE window/mode the headless gateway has none of;
/// inventing placeholder values for them is a real design decision (what does a headless agent's
/// "window"/"mode" even mean) this packet is not positioned to make unilaterally in the time
/// available, so — same honesty rule as `🏃️run`'s own `run_transaction`/`undo_transaction_group` —
/// these return a well-formed, typed `"channel.not-wired"` `Fault` naming exactly what is missing,
/// never a fabricated result.
#[cfg(not(target_arch = "wasm32"))]
pub struct PluginArtifactChannel {
    pub host_driver_policy:semio_framework_plugin_host::GuestRelayDriverPolicy,
    host_driver:Arc<semio_framework_plugin_host::GuestRelayWakeDriver>,
    /// 🐎️ The process-wide compiled runtime ([`shared_plugin_runtime`]), held as the `GuestRuntimes`
    /// enum rather than a bare `WasmtimeRuntime` because the inference lane's
    /// `PluginInstanceHandle` takes exactly that `Arc` — and it MUST be the same one: a
    /// `wasmtime::Component` belongs to the `Engine` that compiled it, so a second runtime could not
    /// instantiate `compiled` at all.
    pub runtime: Arc<GuestRuntimes>,
    pub compiled: semio_framework_plugin_host::CompiledHandle,
    /// 🔌️ The plugin this channel drives. It is the id alone, never a registry ROW: the row names a
    /// repo path this channel has no use for, and a hub-authorized component has no repo path at all.
    pub plugin_id: String,
    pub descriptor: semio_framework::PackageDescriptor,
    pub app_ref: semio_framework::AppRef,
    pub actor_label: String,
    instances: HashMap<u32, semio_framework_plugin_host::GuestInstance>,
    pending_exchanges: PendingExchangeRegistry<1>,
    pending_command_closes: semio_framework::kernel::CommandDriverRegistry<1>,
    rejected_command_builds: semio_framework::kernel::RejectedCommandBuildRegistry<1>,
    next_seq: u64,
    /// 🗿️ The artifact id this channel's ONE live guest session document is, when the owning
    /// workspace has bound exactly one artifact to this plugin. Every `RevisionStamp` this channel
    /// answers names it.
    ///
    /// 🐛️ Before ticket 26/09/18 slice M8 the stamp carried `entry.plugin_id`, so the headless
    /// lane's `expectedRevision` named `"note"` while `artifact_create`/`artifact_snapshot` named
    /// `"…-typed"` — two ids for one document, and `artifact_snapshot(revisionAfter.artifactId)`
    /// answered `no such artifact: note` (`📓️ce1-client-e2e-pinning-and-puzzle-bound.md` §8 gap 4).
    pub(super) session_artifact_id: Option<String>,
    /// 📥️ Per guest instance, the artifact id whose canonical pair that guest's session document
    /// currently IS — see [`Self::load_session_document`]. Empty for a guest still sitting on the
    /// plugin's own genesis document, which is every folder-lane guest and was, before ticket
    /// 26/09/18 slice M10, every hub-lane one as well.
    pub(crate) session_documents: HashMap<u32, String>,
    /// 🧵️ Document-backbone messages this guest published and this channel has not handed on yet —
    /// the `Effect::SendMessage { target: Backbone { uri } }` payloads addressed at this channel's
    /// own actor uri. They used to be dropped on the floor: `exchange_one_turn` named every effect
    /// for diagnostics and admitted only the `Shell{instance}` reply lane, so a committed
    /// transaction's envelopes — the ONE thing that has to leave the process for anyone else to see
    /// the edit — went nowhere at all.
    backbone_egress: Vec<Vec<u8>>,
    /// 🧵️ The instances whose document backbone this channel has bound — see
    /// [`Self::ensure_document_backbone`]. Cleared per instance by [`Self::discard_instance`],
    /// because a guest that was thrown away holds no binding.
    backbone_bindings: std::collections::BTreeSet<u32>,
    /// 💡️ Lazily opened on the FIRST `AppCommand::Infer` — a second guest activation of the same
    /// component, dedicated to the cold `semio.infer` job lane. Deliberately separate from
    /// `instances` above: `PluginInstanceHandle` takes ownership of its `GuestInstance` and drives
    /// `start-job`/`step-job` against it from a worker pool, which the retained command-page loop
    /// `exchange_one_real` runs on the same instance cannot share.
    pub(super) inference: Option<PluginInferenceRoute>,
}

/// 💡️ One plugin's real `semio.infer` route — the SAME `ArtifactInferenceRouter` +
/// `PluginInstanceHandle` pair `🏃️run/🦀️.rs`'s own `load_runtime_recursive` builds, constructed here
/// from the committed descriptor instead of from a live `run` process. The router owns the handle
/// (which owns the `Arc<GuestRuntimes>` and the `GuestInstance`), so this struct holds only it.
#[cfg(not(target_arch = "wasm32"))]
pub(super) struct PluginInferenceRoute {
    pub router: semio_framework_plugin_host::ArtifactInferenceRouter,
}

#[cfg(not(target_arch = "wasm32"))]
pub struct PendingExchange {
    pub instance: u32,
    pub seq: u64,
    pub response: PendingResponsePage,
    /// 🧾️ The guest's own reason for a faulted ingress, kept from the turn that reported it until
    /// the bounded cleanup that follows reaches its terminal and can answer the caller with it.
    pub ingress_fault: Option<String>,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PendingResponseFault {
    Oversized,
    Duplicate,
}

/// 📬️ One command's answer, as it arrives: the guest's own encoded `AppFrame` bytes off the
/// `Shell{instance}` message lane.
///
/// 🧭️ This used to hold a single [`semio_framework::kernel::FixedCommandPage`], because the answer
/// was read off `Effect::Respond` and a `RequestOutcome`'s bytes are one page. An `AppFrame` is not:
/// `ReadDocument` answers a whole `Document{pack, spr}`, which for any real artifact is far past
/// 4 KiB, so that ceiling would have refused every genuine answer the moment the lane was read
/// correctly. The ceiling that does apply is the declared one for a single host↔guest answer,
/// [`semio_framework::kernel::COMMAND_MAXIMUM_BYTES`] — 8 MiB — and it is still a hard bound, so an
/// unbounded guest answer is a typed fault rather than an allocation.
#[cfg(not(target_arch = "wasm32"))]
pub enum PendingResponsePage {
    Empty,
    Frame { bytes: Vec<u8>, duplicate: bool },
    /// 🏁️ The command settled and the guest published no frame naming it — the same reading the
    /// React host takes (`🔌️PluginRuntime/🟦️.tsx`'s `commandIngressNeedsReplyStampV1`, which stamps
    /// `AppFrame::Done{in_reply_to}` on exactly this shape so a caller never waits on an answer a
    /// completed command was never going to publish). A caller that needs a real frame reports the
    /// `Done` as the unexpected variant it is; a caller whose verb genuinely answers `Done`
    /// (`LoadDocumentArchive`'s admission and acknowledgement, `TransactionUndo`/`Redo`) reads its own success.
    Stamped,
    Fault(PendingResponseFault),
}

#[cfg(not(target_arch = "wasm32"))]
impl PendingResponsePage {
    pub fn admit_frame(&mut self, bytes: &[u8]) {
        match self {
            Self::Empty => {
                *self = if bytes.len() > semio_framework::kernel::COMMAND_MAXIMUM_BYTES { Self::Fault(PendingResponseFault::Oversized) } else { Self::Frame { bytes: bytes.to_vec(), duplicate: false } };
            }
            Self::Frame { duplicate, .. } => *duplicate = true,
            Self::Stamped | Self::Fault(_) => *self = Self::Fault(PendingResponseFault::Duplicate),
        }
    }

    pub fn stamp_settled(&mut self) {
        if matches!(self, Self::Empty) {
            *self = Self::Stamped;
        }
    }

    /// 🏁️ Whether a real answer is already held — the test that tells a redundant `AppFrame::Done`
    /// stamp apart from a genuine second answer (see [`RoutingArtifactChannel`]'s sibling
    /// `admit_reply_frame`).
    fn holds_frame(&self) -> bool {
        matches!(self, Self::Frame { .. })
    }

    pub fn take(&mut self, seq: u64) -> Result<store::AppFrame, Fault> {
        match std::mem::replace(self, Self::Empty) {
            Self::Empty => Err(PluginArtifactChannel::not_wired("execute_turn", format!("no AppFrame answering seq {seq} before terminal acknowledgement"))),
            Self::Frame { duplicate: true, .. } => Err(PluginArtifactChannel::not_wired("AppFrame response", format!("the guest published more than one frame answering seq {seq}"))),
            Self::Frame { bytes, duplicate: false } => ::semio_framework_async::poll::resolve_ready(store::decode_app_frame(&bytes)).map_err(|error| PluginArtifactChannel::not_wired("decoding AppFrame", error)),
            Self::Stamped => Ok(store::AppFrame::Done { in_reply_to: seq }),
            Self::Fault(PendingResponseFault::Oversized) => Err(PluginArtifactChannel::not_wired("AppFrame response", format!("the guest's answer exceeds the declared {}-byte host answer ceiling", semio_framework::kernel::COMMAND_MAXIMUM_BYTES))),
            Self::Fault(PendingResponseFault::Duplicate) => Err(PluginArtifactChannel::not_wired("AppFrame response", format!("the guest published more than one frame answering seq {seq}"))),
        }
    }

    /// 🧹️ A frame larger than one retirement grant is released a grant at a time, so a
    /// faulted command's cleanup stays bounded however big the answer the guest had queued.
    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize) {
        let released = match self {
            Self::Frame { bytes, .. } if bytes.len() <= maximum_bytes => bytes.len(),
            Self::Frame { bytes, .. } => {
                let remaining = bytes.len() - maximum_bytes;
                bytes.truncate(remaining);
                return (false, maximum_bytes);
            }
            Self::Empty | Self::Stamped | Self::Fault(_) => 0,
        };
        *self = Self::Empty;
        (true, released)
    }

    pub fn terminal_is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }
}

/// 🪟️ What a HEADLESS agent's transaction carries of one previewed verb: its document operations and owned children, never
/// its config or draft lanes — those are the view state (a selection, a viewport, a mode) of a shell the agent does not
/// have, and the real wire `TransactionPrepare` has no lane for them (coordinator decision, ticket 26/09/23 session 14c: a
/// headless agent edits DOCUMENTS). Beside a document change they are omitted and the preview says so
/// ([`VIEW_STATE_OMITTED_WARNING`]); a verb whose whole effect is view state is refused
/// [`crate::actions::AGENT_LANE_UNCARRIED_FAULT_CODE`] at prepare — it runs only from the shell — instead of failing at
/// invoke with "no wire representation" (live hub coverage on 7800/p33: procedural's add/rename/removeGeneration publish
/// `SetSelectedGeneration` beside their document op).
#[cfg(not(target_arch = "wasm32"))]
pub fn headless_agent_ops(capability_id: &str, ops: PreparedOps) -> Result<(PreparedOps, Vec<String>), Fault> {
    let view_lanes = [("config", !ops.config.is_empty()), ("draft", !ops.draft.is_empty())].into_iter().filter_map(|(lane, held)| held.then_some(lane)).collect::<Vec<_>>();
    if view_lanes.is_empty() {
        return Ok((ops, Vec::new()));
    }
    if ops.document.is_empty() && ops.children.is_empty() {
        return Err(Fault {
            code: AGENT_LANE_UNCARRIED_FAULT_CODE.to_string(),
            message: format!("action '{capability_id}' changes only {} view state, which a headless agent's transaction cannot carry; it runs only from the shell", view_lanes.join(" and ")),
        });
    }
    Ok((PreparedOps { config: Vec::new(), draft: Vec::new(), ..ops }, vec![VIEW_STATE_OMITTED_WARNING.to_string()]))
}

/// 🪟️ The preview warning of a verb whose config/draft view state a headless agent's transaction leaves in the shell.
#[cfg(not(target_arch = "wasm32"))]
pub const VIEW_STATE_OMITTED_WARNING: &str = "view-state-omitted: the action's config/draft view state (a selection, a viewport, a mode) stays in the shell; this transaction commits its document operations only";

/// 🧹️ Begins closing the exchange an ABANDONED command left on `instance` — a pending exchange whose seq is not `seq`'s —
/// so the bounded teardown in `exchange_one_turn` retires it before the next command on the instance is admitted.
#[cfg(not(target_arch = "wasm32"))]
pub fn begin_closing_abandoned_exchange(pending_exchanges: &PendingExchangeRegistry<1>, closes: &mut semio_framework::kernel::CommandDriverRegistry<1>, instance: u32, seq: u64) -> Result<(), Fault> {
    let Some(abandoned) = pending_exchanges.get(instance).map(|pending| pending.seq).filter(|pending_seq| *pending_seq != seq) else { return Ok(()) };
    if closes.is_active(u64::from(instance), abandoned) {
        closes.begin_close(u64::from(instance), abandoned).map_err(|fault| Fault { code: "channel.not-wired".to_string(), message: format!("abandoned command owner: {}: {}", fault.code.0, fault.message) })?;
    }
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub struct PendingExchangeRegistry<const CAPACITY: usize> {
    pub slots: [Option<PendingExchange>; CAPACITY],
}

#[cfg(not(target_arch = "wasm32"))]
impl<const CAPACITY: usize> PendingExchangeRegistry<CAPACITY> {
    pub fn new() -> Self {
        Self { slots: std::array::from_fn(|_| None) }
    }

    pub fn can_insert(&self, instance: u32) -> bool {
        self.slots[instance as usize % CAPACITY].is_none()
    }

    pub fn insert_admitted(&mut self, pending: PendingExchange) {
        let index = pending.instance as usize % CAPACITY;
        assert!(self.slots[index].is_none(), "fixed pending-exchange admission changed before insert");
        self.slots[index] = Some(pending);
    }

    pub fn get_mut(&mut self, instance: u32) -> Option<&mut PendingExchange> {
        self.slots[instance as usize % CAPACITY].as_mut().filter(|pending| pending.instance == instance)
    }

    pub fn get(&self, instance: u32) -> Option<&PendingExchange> {
        self.slots[instance as usize % CAPACITY].as_ref().filter(|pending| pending.instance == instance)
    }

    pub fn remove(&mut self, instance: u32) -> Option<PendingExchange> {
        let index = instance as usize % CAPACITY;
        if self.slots[index].as_ref().is_some_and(|pending| pending.instance == instance) {
            self.slots[index].take()
        } else {
            None
        }
    }

    pub fn close_response_step(&mut self, instance: u32, maximum_bytes: usize) -> (bool, usize) {
        self.get_mut(instance).map_or((true, 0), |pending| pending.response.close_step(maximum_bytes))
    }
}

#[cfg(not(target_arch = "wasm32"))]
/// 🔢️ The actor ordinal the inference-only activation uses. Distinct from every `ensure_instance`
/// ordinal (those are `instance + 1`, i.e. `1..=u16::MAX - 1`) so an inference guest can never
/// collide with a command-lane guest of the same plugin.
#[cfg(not(target_arch = "wasm32"))]
const INFERENCE_ACTOR_ORDINAL: u16 = u16::MAX;

/// ⏱️ The finite allocation/recursion limits every `AppCommand::Infer` carries. Work units come from
/// the caller (clamped to at least one); these two are host policy, not caller input, because an
/// agent must not be able to widen a guest's memory or dependency-recursion envelope.
#[cfg(not(target_arch = "wasm32"))]
const INFERENCE_ALLOCATION_BYTES: u64 = 1 << 20;

#[cfg(not(target_arch = "wasm32"))]
const INFERENCE_RECURSION_DEPTH: u32 = 4;

/// 💡️ The budget the INFERENCE lane's own guest runs under. An 8 ms interpreter slice
/// exists to keep a UI frame responsive; the inference lane has no frame to keep — it activates its
/// own guest (`INFERENCE_ACTOR_ORDINAL`) and `ArtifactInferenceRouter` drives the cold `semio.infer`
/// job to completion inside ONE call this crate cannot resume from the outside, so an 8 ms epoch
/// deadline made every real declared inference answer `epoch deadline exceeded` (measured
/// 2026-09-19 against an installed owner service). Still a hard ceiling:
/// a runaway guest is stopped, just on a headless job's own time scale. `fuel` is disarmed for the
/// same reason the two budgets below disarm it: under the compiled runtime a fuel yield is as
/// unresumable as an epoch cut, so the time ceiling is the only one worth arming.
#[cfg(not(target_arch = "wasm32"))]
fn headless_inference_budget() -> semio_framework::kernel::Budget {
    semio_framework::kernel::Budget { fuel: u64::MAX, deadline_ms: 30_000, max_effects: 4096, max_patch_bytes: 1 << 24, max_frames: 4096 }
}

/// 🎬️ The budget a COLD OPEN turn runs under. A ceiling, not a slice: under the compiled runtime a
/// turn is atomic — an epoch deadline cut leaves the component instance mid-call and every later
/// call answers `cannot enter component instance` — so this number is the point at which the guest
/// is declared wedged and its instance is thrown away, never a quantum the host resumes from. 60 s
/// is far above what the os dev host needs to boot the same components (~20 plugins in seconds) and
/// far below the MCP client's own 240 s per-call budget, so a wedged open still answers the agent.
///
/// ⛽️ `fuel` is deliberately disarmed. Under the JIT a fuel yield is exactly as unresumable as an
/// epoch cut, so arming both would only add a second, harder-to-read way to say "wedged"; the time
/// ceiling is the one an agent and an operator can both reason about. `🔬️owned-instance-open`'s
/// `wasmtime_runtime_instance_open_settles_against_a_real_plugin_component` opens a real component
/// under exactly this shape.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn headless_open_budget() -> semio_framework::kernel::Budget {
    semio_framework::kernel::Budget { fuel: u64::MAX, deadline_ms: 60_000, max_effects: 4096, max_patch_bytes: 1 << 24, max_frames: 4096 }
}

/// 📨️ The ceiling ONE `AppCommand` page turn runs under — same reasoning as
/// [`headless_open_budget`], on the scale of a bounded unit of work rather than a whole app
/// assembly. A command still takes several turns (one retained page per turn, then the guest's
/// terminal acknowledgement), but each of those turns runs to completion; the loop that drives them
/// is [`COMMAND_RESUME_WALL_BUDGET`]'s, and reaching THIS ceiling means one turn wedged.
#[cfg(not(target_arch = "wasm32"))]
fn headless_command_budget() -> semio_framework::kernel::Budget {
    semio_framework::kernel::Budget { fuel: u64::MAX, deadline_ms: 5_000, max_effects: 4096, max_patch_bytes: 1 << 24, max_frames: 4096 }
}

/// ⏱️ How long the page loop for ONE `AppCommand` may keep taking turns before it reports the stall
/// to the caller. A wedged guest must still surface as a bounded, honest `budget.exceeded` rather
/// than a hung MCP tool call, so this is a wall clock, not an attempt count. Every turn inside it
/// completes, so this bounds the NUMBER of pages and host-side cleanup steps a command may need,
/// not the duration of any single guest call.
#[cfg(not(target_arch = "wasm32"))]
const COMMAND_RESUME_WALL_BUDGET: std::time::Duration = std::time::Duration::from_secs(60);

/// ⏱️ How long one whole-document load ([`PluginArtifactChannel::load_document_archive`]) may keep polling
/// before it cancels itself. A long history folds over many bounded polls, so this is longer than one
/// command's [`COMMAND_RESUME_WALL_BUDGET`], and it leaves that budget for settling the cancel inside the MCP
/// client's own 240 s per-call budget.
#[cfg(not(target_arch = "wasm32"))]
const DOCUMENT_LOAD_WALL_BUDGET: std::time::Duration = std::time::Duration::from_secs(150);

/// ⏱️ How long a cold open may keep taking turns. Larger than [`headless_open_budget`] because an
/// open is at least two turns — the `Event::InstanceOpen` itself and the `InstanceLifecycleAck` for
/// the receipt it publishes — and well under the MCP client's own 240 s per-call budget, so a guest
/// that cannot come up is reported as a bounded `budget.exceeded` instead of killing the tool call.
#[cfg(not(target_arch = "wasm32"))]
const INSTANCE_OPEN_WALL_BUDGET: std::time::Duration = std::time::Duration::from_secs(150);

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn shared_compiled_component(runtime: &GuestRuntimes, plugin_id: &str, bytes: &[u8]) -> Result<semio_framework_plugin_host::CompiledHandle, GatewayError> {
    scoped_compiled_component(runtime, plugin_id, ComponentBytes::Held(bytes), &ActivationScope::detached())
}

#[cfg(not(target_arch = "wasm32"))]
impl PluginArtifactChannel {
    fn command_closes_terminal_is_empty(&self) -> bool {
        self.pending_command_closes.terminal_is_empty()
    }

    pub fn new(repo_root: PathBuf, entry: PluginRegistryEntry, descriptor: semio_framework::PackageDescriptor, app_ref: semio_framework::AppRef, actor_label: String, host_driver_policy:semio_framework_plugin_host::GuestRelayDriverPolicy) -> Result<Self, GatewayError> {
        let wasm_path = resolve_plugin_wasm_path(&repo_root, &entry)?;
        let runtime = shared_plugin_runtime()?;
        let compiled = scoped_compiled_component(runtime.as_ref(), &entry.plugin_id, ComponentBytes::File(&wasm_path), &ActivationScope::detached())?;
        Ok(Self::from_compiled(runtime, compiled, entry.plugin_id, descriptor, app_ref, actor_label, host_driver_policy))
    }

    /// 🧩️ Opens a channel over component bytes the caller already holds, whatever their provenance —
    /// a repo build read from disk, or the execution-target component a Hub authorized and served.
    /// Everything past the bytes is identical, which is exactly the point: a hub-bound agent's guest
    /// is the SAME guest a local one gets, not a second code path with its own behaviour.
    pub fn from_component(bytes: &[u8], plugin_id: String, descriptor: semio_framework::PackageDescriptor, app_ref: semio_framework::AppRef, actor_label: String, host_driver_policy:semio_framework_plugin_host::GuestRelayDriverPolicy) -> Result<Self, GatewayError> {
        let runtime = shared_plugin_runtime()?;
        let compiled = shared_compiled_component(runtime.as_ref(), &plugin_id, bytes)?;
        Ok(Self::from_compiled(runtime, compiled, plugin_id, descriptor, app_ref, actor_label, host_driver_policy))
    }

    /// 🧩️ A channel over a component this process already compiled — see [`scoped_compiled_component`].
    pub fn from_compiled(runtime: Arc<GuestRuntimes>, compiled: semio_framework_plugin_host::CompiledHandle, plugin_id: String, descriptor: semio_framework::PackageDescriptor, app_ref: semio_framework::AppRef, actor_label: String, host_driver_policy:semio_framework_plugin_host::GuestRelayDriverPolicy) -> Self {
        Self {
            runtime,
            compiled,
            plugin_id,
            descriptor,
            app_ref,
            actor_label,
            host_driver_policy,
            host_driver:Arc::new(semio_framework_plugin_host::GuestRelayWakeDriver::new(host_driver_policy)),
            instances: HashMap::new(),
            pending_exchanges: PendingExchangeRegistry::new(),
            pending_command_closes: semio_framework::kernel::CommandDriverRegistry::new(),
            rejected_command_builds: semio_framework::kernel::RejectedCommandBuildRegistry::new(),
            next_seq: 1,
            session_artifact_id: None,
            session_documents: HashMap::new(),
            backbone_egress: Vec::new(),
            backbone_bindings: std::collections::BTreeSet::new(),
            inference: None,
        }
    }

    /// 🧵️ Takes every document-backbone message this guest has published since the last drain. The
    /// caller owns the relay: this channel knows the plugin, not the document actor.
    pub fn drain_backbone_egress(&mut self) -> Vec<Vec<u8>> {
        std::mem::take(&mut self.backbone_egress)
    }

    /// 🧵️ Binds this guest instance's DOCUMENT BACKBONE to this channel's own actor uri, once.
    ///
    /// 🐛️ A guest publishes `Effect::SendMessage { target: Backbone { uri } }` **only for an
    /// instance a host has explicitly bound** (`🔌️plugin/🦀️.rs`'s
    /// `plugin_handle_document_backbone_binding` → `document_backbone_effects`). Without this
    /// command the agent's committed transaction is real inside the guest — `action_invoke` answers
    /// `SUCCEEDED` with a moved `headEditId` — and its envelopes are published to nobody, so the hub
    /// ledger stays at `head_seq 0`. Measured live on hub 7681, 2026-09-22
    /// (`🗑️generated/m10-7681-note-status-after.txt`): an invoke that succeeded against a document
    /// whose ledger never moved. A silent success is the worst shape this lane can have, which is
    /// why the bind is a hard precondition of dispatch rather than a best-effort extra.
    ///
    /// 🪢 The uri is [`Self::actor_label`]'s value, which is the SAME string
    /// [`document_backbone_payload`] fences egress on — bind and relay cannot drift apart because
    /// they read one field.
    ///
    /// 🧭️ Ordering: the guest refuses a bind-then-load
    /// (`plugin.document-backbone.load-while-bound`), so this runs AFTER
    /// [`Self::load_session_document`], never before.
    ///
    /// 🧵️ A bind turn can already carry egress — the guest flushes whatever its store had
    /// pending onto the freshly bound backbone — so it is captured here exactly as a command
    /// turn's is, rather than left in the turn and lost.
    pub fn ensure_document_backbone(&mut self, instance: u32) -> Result<(), Fault> {
        if self.backbone_bindings.contains(&instance) {
            return Ok(());
        }
        self.ensure_instance(instance)?;
        let command = semio_framework_plugin::document_backbone_binding::DocumentBackboneBindingCommandV1 {
            operation: semio_framework_plugin::document_backbone_binding::DocumentBackboneBindingOperationV1::Bind,
            instance_id: instance,
            binding_generation: DOCUMENT_BACKBONE_BINDING_GENERATION,
            uri: self.actor_label.clone(),
        };
        let payload = command.encode().map_err(|error| Self::not_wired("document-backbone bind", error))?;
        let event = semio_framework::kernel::Event::Message { source: semio_framework::kernel::MessageEndpoint::Shell { instance: semio_framework::kernel::PluginInstanceId(instance.to_string()) }, payload };
        let runtime = Arc::clone(&self.runtime);
        let guest = self.instances.get_mut(&instance).ok_or_else(|| Self::not_wired("document-backbone bind", format!("no open instance {instance}")))?;
        let turn = semio_framework_async::block_on(runtime.execute_turn(guest, std::slice::from_ref(&event), headless_command_budget())).map_err(|error| Self::not_wired("document-backbone bind", error))?;
        let receipts: Vec<&Vec<u8>> = turn
            .effects
            .iter()
            .filter_map(|effect| match effect {
                semio_framework::kernel::Effect::SendMessage { target: semio_framework::kernel::MessageEndpoint::Shell { instance: target }, payload } if target.0 == instance.to_string() => Some(payload),
                _ => None,
            })
            .collect();
        if receipts.len() != 1 {
            return Err(Self::not_wired("document-backbone bind", format!("the guest answered {} shell receipt(s) for one bind", receipts.len())));
        }
        semio_framework_plugin::document_backbone_binding::require_document_backbone_binding_receipt_v1(receipts[0], &command).map_err(|error| Self::not_wired("document-backbone bind", error))?;
        for effect in &turn.effects {
            if let Some(egress) = document_backbone_payload(&self.actor_label, effect) {
                self.backbone_egress.push(egress.to_vec());
            }
        }
        self.backbone_bindings.insert(instance);
        Ok(())
    }

    /// 🗿️ Names the artifact this channel's guest session document is, for the stamps it answers.
    /// `None` means the owning workspace has bound no artifact to this plugin, or more than one —
    /// see [`Self::session_artifact_id`].
    pub fn bind_session_artifact(&mut self, artifact_id: Option<String>) {
        self.session_artifact_id = artifact_id;
    }

    /// 📥️ Makes `artifact_id`'s canonical pair this guest's session document — one stepped archive
    /// load ([`Self::load_document_archive`]), the same whole-document load every host and this file's
    /// `ExportMedia` arm drive, so nothing about the bytes is interpreted here. `scope` hears its progress
    /// and may cancel it; a cancelled or faulted load leaves the guest's previous document as it was and
    /// records nothing.
    ///
    /// 🧭️ Idempotent per guest: a channel that already holds this artifact's pair does nothing, so
    /// the load happens once per (instance, artifact) and never once per command — re-loading on
    /// every exchange would discard the guest's own uncommitted transaction state between
    /// `TransactionPrepare` and `TransactionCommit`, which is the whole two-phase contract.
    /// [`Self::discard_instance`] forgets it, because a thrown-away guest holds nothing.
    ///
    /// 🔀️ A guest that holds ANOTHER artifact (its app's session document moved to a document the agent opened since,
    /// [`route_session_artifact`]) is thrown away first: a guest is only ever seeded fresh, never while its backbone is
    /// bound to a different document.
    pub fn load_session_document(&mut self, instance: u32, artifact_id: &str, pack: &[u8], spr: &[u8], scope: &ActivationScope) -> Result<(), Fault> {
        match self.session_documents.get(&instance) {
            Some(loaded) if loaded == artifact_id => return Ok(()),
            Some(_) => self.discard_instance(instance),
            None => {}
        }
        self.ensure_instance_scoped(instance, scope)?;
        self.load_document_archive(instance, pack, spr, scope)?;
        self.session_documents.insert(instance, artifact_id.to_string());
        Ok(())
    }

    /// 🗃️ Makes `pack`/`spr` `instance`'s whole document through the stepped, ACK-owned archive load
    /// (`📓️api-stepped-document-load.md` §2, driven by `store::DocumentArchiveLoadHost`): `LoadDocumentArchive`
    /// with no members, `PollDocumentArchiveLoad` until a terminal status, then `AcknowledgeDocumentArchiveLoad`,
    /// each one bounded command exchange, so a long history folds across many guest turns instead of inside one.
    ///
    /// 📊️ Every polled status reaches `scope` as [`ActivationPhase::LoadingDocument`] at `completed / total`.
    ///
    /// 🛑️ A cancel `scope` reports, or [`DOCUMENT_LOAD_WALL_BUDGET`] running out, cancels the load in the guest and
    /// keeps polling until the guest says it restored the previous document with zero trace; the caller gets
    /// `job.cancelled` or `budget.exceeded`. A load that won that race is a loaded document. A guest that does not
    /// settle its cancel within one more [`COMMAND_RESUME_WALL_BUDGET`] is discarded, so the next command starts on
    /// a fresh instance.
    ///
    /// ⚠️ A guest fault, terminal or refused, is the guest's own typed fault with its own code (`document.loading`, …).
    pub(crate) fn load_document_archive(&mut self, instance: u32, pack: &[u8], spr: &[u8], scope: &ActivationScope) -> Result<(), Fault> {
        let mut load = store::DocumentArchiveLoadHost::new(store::DocumentArchivePack { parent_pack: pack.to_vec(), parent_spr: spr.to_vec(), members: Vec::new() });
        let deadline = std::time::Instant::now() + DOCUMENT_LOAD_WALL_BUDGET;
        let mut stopped: Option<Fault> = None;
        loop {
            let now = std::time::Instant::now();
            if stopped.is_none() {
                stopped = scope.checkpoint().err().or_else(|| (now >= deadline).then(|| Self::budget_fault("the document load")));
                if stopped.is_some() {
                    load.request_cancel();
                }
            } else if now >= deadline + COMMAND_RESUME_WALL_BUDGET {
                self.discard_instance(instance);
                return Err(Fault { code: "budget.exceeded".to_string(), message: "the document load did not settle its cancel within its wall budget; the instance was discarded and the next command starts on a fresh one".to_string() });
            }
            let (seq, command) = match load.step(|| self.take_seq()) {
                store::DocumentArchiveLoadStep::Send { seq, command } => (seq, command),
                store::DocumentArchiveLoadStep::Finished(store::DocumentArchiveLoadOutcome::Ready) => return Ok(()),
                store::DocumentArchiveLoadStep::Finished(store::DocumentArchiveLoadOutcome::Cancelled) => {
                    return Err(stopped.unwrap_or_else(|| Fault { code: ACTIVATION_CANCELLED_FAULT_CODE.to_string(), message: "the guest cancelled the document load; its previous document is unchanged".to_string() }));
                }
                store::DocumentArchiveLoadStep::Finished(store::DocumentArchiveLoadOutcome::Fault(fault)) => return Err(decode_guest_fault(&fault)),
            };
            let frame = self.exchange_sequenced(instance, &command, seq)?;
            match load.answer(seq, &frame) {
                Ok(Some(status)) => {
                    let fraction = if status.total == 0 { 0.0 } else { status.completed as f64 / status.total as f64 };
                    if let (Err(cancelled), true) = (scope.advance(ActivationPhase::LoadingDocument, fraction), stopped.is_none()) {
                        stopped = Some(cancelled);
                        load.request_cancel();
                    }
                }
                Ok(None) => {}
                Err(store::DocumentArchiveLoadRefusal::Refused(fault)) => return Err(decode_guest_fault(&fault)),
                Err(store::DocumentArchiveLoadRefusal::Unanswered) => return Err(Self::not_wired("document archive load", format!("the guest answered seq {seq} with {}", app_frame_tag(&frame)))),
            }
        }
    }

    /// 🔁️ Throws this guest's session document away so the next [`Self::load_session_document`] seeds
    /// it from a new baseline — a guest can only be reseeded fresh, never while its backbone is bound.
    pub fn reseed_session_document(&mut self, instance: u32) {
        self.discard_instance(instance);
    }

    /// 📥️ Hands one mutation batch the hub document's actor delivered to this guest's bound document
    /// backbone — the same `Event::Message { source: Backbone }` turn the wgpu and React shells drive
    /// (`ProgramBridge::receive_document_backbone`) — and drives the guest until it is idle. Whatever it
    /// publishes back onto the backbone is kept for the relay, and an `AppFrame::Error` it answers on
    /// its shell lane is the refusal it is.
    pub fn receive_document_backbone(&mut self, instance: u32, message: Vec<u8>) -> Result<(), Fault> {
        store::os_store::backbone::decode_hot_backbone_message_exact(&message).map_err(|error| Self::not_wired("document-backbone ingress", error))?;
        self.ensure_instance(instance)?;
        let mut owed = vec![semio_framework::kernel::Event::Message { source: semio_framework::kernel::MessageEndpoint::Backbone { uri: self.actor_label.clone() }, payload: message }];
        let deadline = std::time::Instant::now() + COMMAND_RESUME_WALL_BUDGET;
        loop {
            let events = std::mem::take(&mut owed);
            let runtime = Arc::clone(&self.runtime);
            let guest = self.instances.get_mut(&instance).ok_or_else(|| Self::not_wired("document-backbone ingress", format!("no open instance {instance}")))?;
            let turn = match semio_framework_async::block_on(runtime.execute_turn(guest, &events, headless_command_budget())) {
                Ok(turn) => turn,
                Err(error) => {
                    self.discard_instance(instance);
                    return Err(Self::not_wired("document-backbone ingress", format!("{error}; the instance was discarded")));
                }
            };
            for effect in &turn.effects {
                if let Some(egress) = document_backbone_payload(&self.actor_label, effect) {
                    self.backbone_egress.push(egress.to_vec());
                }
                if let Some(fault) = shell_lane_fault(instance, effect) {
                    return Err(fault);
                }
            }
            if let Some(receipt) = turn.lifecycle_receipt {
                owed.push(semio_framework::kernel::Event::InstanceLifecycleAck(semio_framework::kernel::ActorInstanceLifecycleAck { receipt }));
            }
            match turn.status {
                semio_framework::kernel::TurnStatus::Idle if owed.is_empty() => return Ok(()),
                semio_framework::kernel::TurnStatus::Faulted(detail) => return Err(decode_guest_fault(&detail)),
                _ if owed.is_empty() => owed.push(semio_framework::kernel::Event::Wake),
                _ => {}
            }
            if std::time::Instant::now() >= deadline {
                return Err(Self::budget_fault("document-backbone ingress"));
            }
        }
    }

    /// 🗿️ What a `RevisionStamp` from this channel names. A channel with no bound artifact answers
    /// the plugin-scoped session pseudo-id `plugin:<id>` — which is what the document IS in that
    /// case, a plugin's genesis session no client can address — never the bare plugin id, which a
    /// client would read as an artifact id and pass straight back into `artifact_snapshot`.
    pub(crate) fn stamped_artifact_id(&self) -> String {
        self.session_artifact_id.clone().unwrap_or_else(|| format!("plugin:{}", self.plugin_id))
    }

    /// 💡️ Opens (once) this plugin's real inference route: a fresh `GuestRuntimes` over the SAME
    /// compiled component, one `PluginInstanceHandle`, and an `ArtifactInferenceRouter` registered
    /// with the plugin's own declared roster — byte-for-byte the `serde_json` roster
    /// `🏃️run/🦀️.rs` builds (`contributions.inference_services` chained with every
    /// `artifact_contributions[].inferences`). A plugin that declares no inference at all is a
    /// typed refusal here rather than an empty registration that would fail later with a worse
    /// message.
    ///
    /// 🐎️ The SAME runtime the command lane uses, not a second one: `self.compiled` carries a
    /// `wasmtime::Component` that only its own `Engine` can instantiate. The inference lane is
    /// still a SEPARATE guest instance on its own actor ordinal — `PluginInstanceHandle` takes
    /// ownership of the instance it drives — it just shares the engine and the compiled code.
    fn ensure_inference_route(&mut self) -> Result<&PluginInferenceRoute, Fault> {
        if self.inference.is_none() {
            let roster: Vec<serde_json::Value> = self
                .descriptor
                .contributions
                .inference_services
                .iter()
                .chain(self.descriptor.contributions.artifact_contributions.iter().flat_map(|contribution| contribution.inferences.iter()))
                .map(serde_json::to_value)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|error| Self::not_wired("inference roster", error))?;
            if roster.is_empty() {
                return Err(Fault { code: "capability.not-found".to_string(), message: format!("plugin `{}` declares no inference service in its committed descriptor", self.plugin_id) });
            }
            let roster_bytes = serde_json::to_vec(&roster).map_err(|error| Self::not_wired("inference roster", error))?;
            let actor = ::semio_framework_async::poll::resolve_ready(semio_framework_actor::ActorId::new(INFERENCE_ACTOR_ORDINAL, 0, 1, 0));
            let caps: Vec<semio_framework::kernel::BrokerCapabilityGrant> =
                self.descriptor.capability_requests.iter().map(|request| semio_framework::kernel::BrokerCapabilityGrant { token: semio_framework::CapabilityToken(0), id: request.id.clone(), scope: request.scope.clone(), expires_ms: None }).collect();
            let runtimes = Arc::clone(&self.runtime);
            let budget = headless_inference_budget();
            let instance = semio_framework_async::block_on(semio_framework_plugin_host::GuestRuntime::instantiate(runtimes.as_ref(), &self.compiled, actor, &caps, &budget)).map_err(|error| Self::not_wired("inference instantiate", error))?;
            let handle = Arc::new(semio_framework_async::block_on(semio_framework_plugin_host::PluginInstanceHandle::new(actor, Arc::clone(&runtimes), instance,semio_framework_plugin_host::GuestRelayWakeAuthority{drive_policy:self.host_driver_policy.drive,policy:self.host_driver_policy.wake,issuer:semio_framework_job_worker::admit_original_thread_worker_wake,receiver:self.host_driver.clone()})));
            let router = semio_framework_plugin_host::ArtifactInferenceRouter::new();
            semio_framework_async::block_on(router.register_plugin(&self.plugin_id, &self.descriptor.manifest.dependencies, handle, &roster_bytes)).map_err(|error| Self::not_wired("inference route registration", error))?;
            self.inference = Some(PluginInferenceRoute { router });
        }
        Ok(self.inference.as_ref().expect("inference route was just opened"))
    }

    /// 💡️ Runs ONE declared inference through the real router — the general replacement for the old
    /// blanket `channel.not-wired`. The router resolves the route, toposorts and injects
    /// `dependsOn` results, drives the guest's `semio.infer` cold job, validates the guest's echo
    /// field-for-field and re-checks commit freshness; this function only builds the request wire
    /// and decodes the result, and reports whatever the router answers verbatim.
    ///
    /// 🗣️ The artifact schema's own canonical identity, exactly as the plugin committed it.
    /// It is an `ArtifactIdentity`, which the guest validates with `ArtifactIdentity::parse`
    /// (`🔌️plugin/🦀️.rs`, `validate_wire_request_resources`): dot-delimited segments of
    /// `[a-z0-9_-]` only. The `<kind>@<schemaVersion>/*` form this used to send is the
    /// CAPABILITY-id grammar, not an identity — `@`, `/` and `*` are all non-canonical
    /// segment bytes, so every real inference died `artifact-inference.source-dialect:
    /// identity "s.wfc.bitmap@1/*" has a non-canonical segment` before the guest ran a
    /// single step (measured 2026-09-22, slice CE3). The version is not lost by dropping the
    /// suffix: `artifact_schema_version` is its own field on this same request.
    pub(crate) fn infer_real(&mut self, command: &InferCommand) -> Result<ArtifactInferenceResultV1, Fault> {
        let declared = self
            .descriptor
            .contributions
            .inference_services
            .iter()
            .chain(self.descriptor.contributions.artifact_contributions.iter().flat_map(|contribution| contribution.inferences.iter()))
            .find(|item| item.artifact_kind == command.artifact_kind && item.inference_schema == command.inference_schema)
            .cloned()
            .ok_or_else(|| Fault {
                code: "capability.not-found".to_string(),
                message: format!("plugin `{}` declares no inference `{}` on artifact kind `{}`", self.plugin_id, command.inference_schema, command.artifact_kind),
            })?;
        let request = ArtifactInferenceRequestV1 {
            wire_version: ARTIFACT_INFERENCE_WIRE_VERSION,
            owner: declared.owner.clone(),
            artifact_kind: declared.artifact_kind.clone(),
            artifact_schema: declared.artifact_schema.clone(),
            artifact_schema_version: declared.artifact_schema_version,
            inference_schema: declared.inference_schema.clone(),
            inference_schema_version: declared.inference_schema_version,
            algorithm_version: declared.algorithm_version,
            policy_version: declared.policy_version,
            revision: command.revision,
            generation: command.turn.generation,
            source_dialect: declared.artifact_schema.clone(),
            policy: Vec::new(),
            budgets: ArtifactInferenceBudgetV1 { allocation_bytes: INFERENCE_ALLOCATION_BYTES, work_units: command.work_units, recursion_depth: INFERENCE_RECURSION_DEPTH },
            retained: command.retained_policy,
            cancellation_id: command.cancellation_id.clone(),
            previous_state: None,
            requested_cache_mode: ArtifactInferenceCacheModeV1::Cold,
            canonical_payload: bind_inference_document(&declared, command)?,
            dependencies: match command.artifact_document {
                Some(_) => self.inference_child_dependencies(&command.artifact_id)?,
                None => Vec::new(),
            },
        };
        let request_bytes = serde_json::to_vec(&request).map_err(|error| Self::not_wired("encoding the inference request", error))?;
        let route = self.ensure_inference_route()?;
        let result_bytes = semio_framework_async::block_on(route.router.infer(&request_bytes, &command.cancel.0)).map_err(|error| Fault { code: "app.command.rejected".to_string(), message: format!("`{}` refused: {error}", request.inference_schema) })?;
        serde_json::from_slice(&result_bytes).map_err(|error| Self::not_wired("decoding the inference result", error))
    }

    /// 🎯️ The owner-qualified address a typed command frame needs, resolved out of the plugin's OWN
    /// committed manifest rather than parsed out of the capability id's punctuation.
    ///
    /// 🗂️ The walk is `🗂️catalog`'s own `app_action_verbs` walk, in the same order and with the same
    /// precedence, so the address this builds always names the window kind the catalog published the
    /// verb under: window-kind declarations first, then APP-scope actions (`🗒️note` declares all 48
    /// of its verbs this way) under the catalog's own `"*"` marker for "belongs to no particular
    /// window". Recomputing the catalog's id (`{plugin}.{app}.{action}`) and comparing is what keeps
    /// the two in step — an app id is itself a dotted canonical surface ref
    /// (`s.draw.drawing@1/*#editor`), so splitting the id on `.` could never have recovered it.
    ///
    /// 🪟️ `window_instance_id` is the window kind's own id, which is exactly what the React shell's
    /// `sessionWindowInstances` mints for a freshly opened app (`app.windowKinds.map(kind => ({id:
    /// kind.id, …}))`) — an agent therefore addresses the very pane a human's first window is,
    /// rather than a synthetic id no shell would ever produce.
    pub(crate) fn addressed_action_invocation(&self, capability_id: &str, input: &serde_json::Value) -> Result<semio_framework::manifest::ActionInvocation, Fault> {
        let plugin_id = self.plugin_id.as_str();
        let mut address: Option<semio_framework::manifest::ActionAddress> = None;
        'apps: for app in self.descriptor.manifest.apps.iter() {
            let base_window = app.window_kinds.iter().next().map(|kind| kind.id.clone());
            for window_kind in app.window_kinds.iter() {
                for action in window_kind.actions.iter() {
                    if capability_id == format!("{plugin_id}.{}.{}", app.id, action.id) {
                        address = Some(semio_framework::manifest::ActionAddress {
                            plugin_id: plugin_id.to_string(),
                            app_id: app.id.clone(),
                            mode_id: app.default_mode_id.clone(),
                            window_kind_id: window_kind.id.clone(),
                            window_instance_id: window_kind.id.clone(),
                            action_id: action.id.clone(),
                        });
                        break 'apps;
                    }
                }
            }
            for action in app.actions.iter() {
                if capability_id == format!("{plugin_id}.{}.{}", app.id, action.id) {
                    let Some(window_instance_id) = base_window.clone() else {
                        return Err(Self::not_wired("PureCommand", format!("app `{}` declares action `{}` but no window kind at all, so there is no pane an agent could address it in", app.id, action.id)));
                    };
                    address = Some(semio_framework::manifest::ActionAddress {
                        plugin_id: plugin_id.to_string(),
                        app_id: app.id.clone(),
                        mode_id: app.default_mode_id.clone(),
                        window_kind_id: "*".to_string(),
                        window_instance_id,
                        action_id: action.id.clone(),
                    });
                    break 'apps;
                }
            }
        }
        let address = address.ok_or_else(|| Fault {
            code: "capability.not-found".to_string(),
            message: format!("plugin `{plugin_id}` declares no app action named by capability `{capability_id}` — its committed manifest publishes {} app(s)", self.descriptor.manifest.apps.len()),
        })?;
        let arguments = match input {
            serde_json::Value::Object(map) => map.iter().map(|(key, value)| (key.clone(), semio_framework_value::DslValue::from(value))).collect(),
            serde_json::Value::Null => std::collections::BTreeMap::new(),
            other => return Err(Self::not_wired("PureCommand", format!("an action's input is a named argument object; this call carried {other}"))),
        };
        Ok(semio_framework::manifest::ActionInvocation { address, arguments })
    }

    pub(crate) fn not_wired(what: &str, detail: impl std::fmt::Display) -> Fault {
        Fault { code: "channel.not-wired".to_string(), message: format!("{what}: {detail}") }
    }

    fn budget_fault(what: &str) -> Fault {
        Fault { code: "budget.exceeded".to_string(), message: format!("{what} reached its wall budget before it settled") }
    }

    /// ☠️ Throws an instance away. A turn cut by an epoch deadline or a trap leaves the component
    /// instance mid-call — wasmtime answers `cannot enter component instance` for every later call —
    /// so there is nothing to resume and nothing to salvage; the next `ensure_instance` builds a
    /// fresh one. `drop_instance` is driven rather than relying on `Drop` because it is what the
    /// `GuestRuntime` contract names for closing an instance, and under wasmtime it is exactly the
    /// `Store` release the pooling allocator needs to reclaim the slab.
    fn discard_instance(&mut self, instance: u32) {
        self.session_documents.remove(&instance);
        self.backbone_bindings.remove(&instance);
        if let Some(guest) = self.instances.remove(&instance) {
            semio_framework_async::block_on(self.runtime.drop_instance(guest));
        }
    }

    /// 🎬️ Opens `instance`'s guest and drives it to settle: `Event::InstanceOpen`, then the
    /// `InstanceLifecycleAck` for whatever receipt that open published, until the guest reports
    /// `Idle` with nothing owed. Bounded by [`INSTANCE_OPEN_WALL_BUDGET`], with each turn bounded by
    /// [`headless_open_budget`].
    ///
    /// 🐎️ The mid-flight pump this replaces (slices, an `undelivered` queue, a `turn_in_flight`
    /// guard) existed because an OWNED-interpreter turn can yield mid-call and refuse new events
    /// until it settles. A compiled turn is atomic — it returns a `TurnResult` or it faults — so the
    /// queue has nothing to hold and the guard nothing to guard. What survives the move is the part
    /// that was never about the runtime: the acknowledgement below.
    ///
    /// 🧾️ The guest's `ActorInstanceLifecycleReceipt` is acknowledged here. An open the host never
    /// acknowledges is retained state the guest re-offers on every later turn — `🖥️host`'s own
    /// `🔬️poll-turn-memory` law measures exactly that — so without this the instance never reaches
    /// `Idle` and every command afterwards pays for an open that never finished.
    ///
    /// ♻️ The guest's own retained lifecycle-deadline verdict is runtime-independent too: it is the
    /// guest saying "I kept my progress, ask me again", so the events it did not consume are
    /// re-offered on the next turn rather than lost.
    pub(crate) fn ensure_instance(&mut self, instance: u32) -> Result<(), Fault> {
        self.ensure_instance_scoped(instance, &ActivationScope::detached())
    }

    /// 🎬️ [`Self::ensure_instance`] under a caller's progress and cancel: a cancel observed between
    /// turns throws the half-open guest away, so the next open starts clean.
    ///
    /// 🎟️ The grants are handed to `instantiate` — the store's capability table is built with the
    /// instance — and travel in `Event::InstanceOpen` as well, which is where the guest reads them.
    ///
    /// ⏱️ A compiled turn cut by its own ceiling is not a yield to resume from: the
    /// instance is mid-call and unusable. Report it as the bounded budget fault it is,
    /// rather than as a wiring fault, and throw the half-open instance away.
    pub(crate) fn ensure_instance_scoped(&mut self, instance: u32, scope: &ActivationScope) -> Result<(), Fault> {
        if self.instances.contains_key(&instance) {
            return Ok(());
        }
        scope.enter(ActivationPhase::OpeningGuest)?;
        let actor = ::semio_framework_async::poll::resolve_ready(semio_framework_actor::ActorId::new((instance as u16).wrapping_add(1), 0, 1, 0));
        let caps: Vec<semio_framework::kernel::BrokerCapabilityGrant> = self
            .descriptor
            .capability_requests
            .iter()
            .map(|request| semio_framework::kernel::BrokerCapabilityGrant { token: semio_framework::CapabilityToken(0), id: request.id.clone(), scope: request.scope.clone(), expires_ms: None })
            .collect();
        let budget = headless_open_budget();
        let mut guest = semio_framework_async::block_on(self.runtime.instantiate(&self.compiled, actor, &caps, &budget)).map_err(|error| match error {
            semio_framework_plugin_guest_contract::PluginHostError::Refused(fault) => Fault { code: fault.code.0.clone(), message: fault.message.clone() },
            other => Self::not_wired("instantiate", other),
        })?;
        let assets = find_repo_root()
            .map_err(|error| Fault { code: "package-assets.root".to_string(), message: error.to_string() })
            .and_then(|root| semio_framework_plugin_host::package_assets::package_file_assets(&root, &self.descriptor.assets).map_err(|message| Fault { code: "package-assets.refused".to_string(), message }))?;
        let mut owed = vec![semio_framework::kernel::Event::InstanceOpen {
            request: semio_framework::kernel::ActorInstanceOpenRequest { activation_generation: 1, instance_id: instance, request_sequence: 1 },
            app_id: semio_framework::kernel::AppInstanceId(self.app_ref.app_id.clone()),
            actor: self.actor_label.clone(),
            config: Vec::new(),
            assets,
            capabilities: caps,
            quotas: self.descriptor.quotas.clone(),
        }];
        let deadline = std::time::Instant::now() + INSTANCE_OPEN_WALL_BUDGET;
        loop {
            let events = std::mem::take(&mut owed);
            match semio_framework_async::block_on(self.runtime.execute_turn(&mut guest, &events, headless_open_budget())) {
                Ok(turn) => {
                    if let Some(receipt) = turn.lifecycle_receipt {
                        owed.push(semio_framework::kernel::Event::InstanceLifecycleAck(semio_framework::kernel::ActorInstanceLifecycleAck { receipt }));
                    }
                    if matches!(turn.status, semio_framework::kernel::TurnStatus::Idle) && owed.is_empty() {
                        self.instances.insert(instance, guest);
                        return Ok(());
                    }
                }
                Err(error) if semio_framework_plugin_host::retryable_lifecycle_turn(&error, &events) => {
                    owed.splice(0..0, events);
                }
                Err(error @ (semio_framework_plugin_guest_contract::TurnFault::DeadlineExceeded | semio_framework_plugin_guest_contract::TurnFault::FuelExhausted)) => {
                    semio_framework_async::block_on(self.runtime.drop_instance(guest));
                    return Err(Fault { code: "budget.exceeded".to_string(), message: format!("InstanceOpen exceeded one turn's ceiling ({error}); the guest never came up") });
                }
                Err(error) => {
                    semio_framework_async::block_on(self.runtime.drop_instance(guest));
                    return Err(Self::not_wired("InstanceOpen", error));
                }
            }
            if let Err(cancelled) = scope.checkpoint() {
                semio_framework_async::block_on(self.runtime.drop_instance(guest));
                return Err(cancelled);
            }
            if std::time::Instant::now() >= deadline {
                semio_framework_async::block_on(self.runtime.drop_instance(guest));
                return Err(Self::budget_fault("InstanceOpen"));
            }
        }
    }

    /// 🔁️ Drives one `AppCommand` to its terminal frame. The command becomes a retained host batch
    /// whose exact current page is handed to one guest turn; the next page stays untouched until the
    /// guest acknowledges the one before it, so a command of N pages is N+1 turns. An interactive
    /// host takes those turns off its own frame loop; a headless gateway has no next frame, so it
    /// takes them here, bounded by [`COMMAND_RESUME_WALL_BUDGET`] and never unbounded.
    ///
    /// 🔢️ ONE sequence for the whole command, stamped into the command itself before it is
    /// encoded. The kernel envelope and the `AppCommand` must carry the same number: the guest
    /// answers with an `AppFrame` whose `in_reply_to` is the COMMAND's `seq`. Every call site in
    /// this file used to pass a literal `seq: 0` while the envelope counted from 1, so even a host
    /// reading the right lane would have correlated nothing.
    pub(crate) fn exchange_one_real(&mut self, instance: u32, mut real_command: store::AppCommand) -> Result<store::AppFrame, Fault> {
        let seq = self.take_seq();
        *app_command_seq_mut(&mut real_command) = seq;
        self.exchange_sequenced(instance, &real_command, seq)
    }

    /// 🔢️ The next command sequence — the one number the kernel envelope and the command itself share.
    fn take_seq(&mut self) -> u64 {
        let seq = self.next_seq;
        self.next_seq += 1;
        seq
    }

    /// 🪆️ The inference dependencies a composed artifact's readers compose on read (design §20.15): one
    /// `child:<slot>/<childId>` → HEAD snapshot pack per owned child of `artifact_id`, read with `ReadChildHeads`
    /// from the session guest that holds it. An artifact no session guest holds, or one without children, has none.
    fn inference_child_dependencies(&mut self, artifact_id: &str) -> Result<Vec<(String, Vec<u8>)>, Fault> {
        let Some(instance) = self.session_documents.iter().find_map(|(instance, loaded)| (loaded == artifact_id).then_some(*instance)) else { return Ok(Vec::new()) };
        let seq = self.take_seq();
        match self.exchange_sequenced(instance, &store::AppCommand::ReadChildHeads { seq }, seq)? {
            store::AppFrame::ChildHeads { entries, .. } => Ok(child_head_dependencies(entries)),
            store::AppFrame::Error { fault, .. } => Err(decode_guest_fault(&fault)),
            other => Err(Self::not_wired("ReadChildHeads", format!("the guest answered seq {seq} with {}", app_frame_tag(&other)))),
        }
    }

    /// 🔁️ [`Self::exchange_one_real`] for a command already stamped with `seq` — the archive load's
    /// `store::DocumentArchiveLoadHost` stamps its own, because its operation IS its admission's sequence.
    fn exchange_sequenced(&mut self, instance: u32, real_command: &store::AppCommand, seq: u64) -> Result<store::AppFrame, Fault> {
        let deadline = std::time::Instant::now() + COMMAND_RESUME_WALL_BUDGET;
        loop {
            match self.exchange_one_turn(instance, real_command, seq)? {
                CommandTurn::Settled(frame) => return Ok(frame),
                CommandTurn::MoreWork if std::time::Instant::now() < deadline => continue,
                CommandTurn::MoreWork => {
                    self.discard_instance(instance);
                    return Err(Fault { code: "budget.exceeded".to_string(), message: format!("AppCommand seq {seq} reached its wall budget before it settled; the instance was discarded and the next command starts on a fresh one") });
                }
            }
        }
    }

    /// 🔁️ ONE guest turn, or one bounded host-side bookkeeping step, of the command above. Returns
    /// [`CommandTurn::MoreWork`] when the command genuinely owes another one — the guest has more
    /// retained pages to read, or a fixed-capacity registry is mid-teardown — and a `Fault` only
    /// when the command is over. The two used to be the same value: every "call me again" was minted
    /// as a `budget.exceeded` fault and the loop above re-entered on the fault's CODE, so a real
    /// budget failure and normal paging were indistinguishable both to the loop and to the agent
    /// reading the message.
    ///
    /// 📄️ One retained page per turn. Under a compiled runtime every turn completes, so the
    /// cursor always advances here; when the driver has no page left the guest is woken instead,
    /// which is how it gets to publish its terminal acknowledgement.
    ///
    /// ⏱️ The suspension window is "while this guest turn runs, nobody touches the driver", so
    /// it is closed on every arm, not just the `Ok` one: leaving the driver `Suspended` (and
    /// linked for close) made the next turn read `!is_active`, tear the whole command down and
    /// re-send it under a BRAND NEW `seq` while the guest still retained the owner of the
    /// first attempt — the whole of `plugin.command-cursor-mismatch: owner is 1, the retained
    /// owner's is 554` (ticket 26/09/18 slice A2 §6.1). The instance itself is gone either
    /// way: a turn cut by its ceiling leaves the component mid-call and unusable.
    ///
    /// 🧾️ The guest's OWN reason for refusing the ingress rides in `command_ingress`, and
    /// the cleanup below takes further turns — so it is captured on the exchange the
    /// moment it is seen, not read again at the terminal where it no longer exists. It
    /// used to be dropped entirely, and "faulted for seq N after bounded exact-owner
    /// cleanup" told the agent (and every earlier slice of this ticket) nothing at all.
    ///
    /// 🧹️ An exchange left behind by an ABANDONED command — its wall budget ran out, or its guest turn faulted and the
    /// instance was discarded — is closed step by step before the next command on the instance is admitted. It used to
    /// stay active, and the next command's turn met it with `assert_eq!(pending.seq, seq)`: the gateway's main thread
    /// panicked and every MCP session on it died (live hub coverage on 7800/p33, ticket 26/09/23 G12 session 14c).
    fn exchange_one_turn(&mut self, instance: u32, real_command: &store::AppCommand, seq: u64) -> Result<CommandTurn, Fault> {
        if !self.rejected_command_builds.terminal_is_empty() {
            self.rejected_command_builds.close_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES);
            return Ok(CommandTurn::MoreWork);
        }
        begin_closing_abandoned_exchange(&self.pending_exchanges, &mut self.pending_command_closes, instance, seq)?;
        if let Some(seq) = self.pending_exchanges.get(instance).map(|pending| pending.seq) {
            if !self.pending_command_closes.is_active(u64::from(instance), seq) {
                let (response_complete, _) = self.pending_exchanges.close_response_step(instance, semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES);
                if !response_complete {
                    return Ok(CommandTurn::MoreWork);
                }
                let (complete, _, _) = self.pending_command_closes.close_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES);
                if complete {
                    let terminal = self.pending_exchanges.remove(instance).expect("closed pending exchange reached terminal empty");
                    assert!(terminal.response.terminal_is_empty(), "closed pending response reached terminal empty");
                }
                return Ok(CommandTurn::MoreWork);
            }
        }
        if self.pending_exchanges.get(instance).is_none() && !persistent_command_completion_port_ready() {
            return Err(Self::not_wired("AppCommand", "persistent command completion submit/poll/cancel authority is not admitted"));
        }
        if self.pending_exchanges.get(instance).is_none() {
            if !self.pending_exchanges.can_insert(instance) || !self.pending_command_closes.can_insert(u64::from(instance)) || !self.rejected_command_builds.can_insert(u64::from(instance)) {
                return Err(Fault { code: "budget.exceeded".to_string(), message: "fixed pending command authority is saturated or collided".to_string() });
            }
            let command = ::semio_framework_async::poll::resolve_ready(store::encode_app_command(real_command)).map_err(|fault| Self::not_wired("encoding AppCommand", format!("{}: {}", fault.code.0, fault.message)))?;
            let envelope = semio_framework::kernel::CommandEnvelope { instance, seq, command };
            let mut owners = semio_framework::kernel::CommandEnvelopeSet::try_new().map_err(|fault| Self::not_wired("reserving command batch", format!("{}: {}", fault.code.0, fault.message)))?;
            if let Err((fault, rejected)) = owners.try_push(envelope) {
                self.rejected_command_builds.insert_admitted(u64::from(instance), semio_framework::kernel::RejectedCommandBuild::new(owners, rejected));
                return Err(Self::not_wired("admitting command owner", format!("{}: {}", fault.code.0, fault.message)));
            }
            let batch = match semio_framework::kernel::CommandBatch::try_new(seq, owners) {
                Ok(batch) => batch,
                Err((fault, owners)) => {
                    self.rejected_command_builds.insert_admitted(u64::from(instance), semio_framework::kernel::RejectedCommandBuild::from_admitted(owners));
                    return Err(Self::not_wired("admitting command batch", format!("{}: {}", fault.code.0, fault.message)));
                }
            };
            self.pending_command_closes.insert_admitted(u64::from(instance), seq, semio_framework::kernel::CommandBatchDriver::new(seq, batch));
            self.pending_exchanges.insert_admitted(PendingExchange { instance, seq, response: PendingResponsePage::Empty, ingress_fault: None });
        }
        assert_eq!(self.pending_exchanges.get(instance).expect("pending exchange was admitted").seq, seq, "the retained exchange answers the command this turn drives");
        let event = self
            .pending_command_closes
            .with_driver_mut(u64::from(instance), seq, |driver| driver.next_page())
            .map_err(|fault| Self::not_wired("retained command driver", format!("{}: {}", fault.code.0, fault.message)))?
            .map_err(|fault| Self::not_wired("command page", format!("{}: {}", fault.code.0, fault.message)))?
            .map(|(cursor, bytes)| semio_framework::kernel::Event::CommandIngressPage { cursor, bytes })
            .unwrap_or(semio_framework::kernel::Event::Wake);
        let runtime = Arc::clone(&self.runtime);
        let guest = self.instances.get_mut(&instance).ok_or_else(|| Self::not_wired("exchange", format!("no open instance {instance}")))?;
        self.pending_command_closes.prepare_suspend(u64::from(instance), seq).map_err(|fault| Self::not_wired("suspending command owner", format!("{}: {}", fault.code.0, fault.message)))?;
        let outcome = semio_framework_async::block_on(runtime.execute_turn(guest, std::slice::from_ref(&event), headless_command_budget()));
        let turn = match outcome {
            Ok(turn) => {
                self.pending_command_closes.resume(u64::from(instance), seq).map_err(|fault| Self::not_wired("resuming command owner", format!("{}: {}", fault.code.0, fault.message)))?;
                turn
            }
            Err(error) => {
                self.pending_command_closes.resume(u64::from(instance), seq).map_err(|fault| Self::not_wired("resuming command owner", format!("{}: {}", fault.code.0, fault.message)))?;
                self.discard_instance(instance);
                return Err(match error {
                    semio_framework_plugin_guest_contract::TurnFault::DeadlineExceeded | semio_framework_plugin_guest_contract::TurnFault::FuelExhausted => {
                        Fault { code: "budget.exceeded".to_string(), message: format!("AppCommand exceeded one turn's ceiling ({error}); the instance was discarded") }
                    }
                    error => Self::not_wired("execute_turn", error),
                });
            }
        };
        let progress = self
            .pending_command_closes
            .with_driver_mut(u64::from(instance), seq, |driver| driver.observe(&turn.command_ingress, semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES))
            .map_err(|fault| Self::not_wired("retained command driver", format!("{}: {}", fault.code.0, fault.message)))?
            .map_err(|fault| Self::not_wired("command acknowledgement", format!("{}: {} — the guest answered {:?} while this gateway drives instance {instance} seq {seq}", fault.code.0, fault.message, turn.command_ingress)))?;
        let mut published: Vec<String> = Vec::new();
        let mut backbone_egress: Vec<Vec<u8>> = Vec::new();
        for effect in &turn.effects {
            published.push(effect_shape(instance, effect));
            if let Some(payload) = document_backbone_payload(&self.actor_label, effect) {
                backbone_egress.push(payload.to_vec());
            }
            Self::admit_reply_frame(self.pending_exchanges.get_mut(instance).expect("pending exchange was admitted"), instance, seq, effect);
        }
        self.backbone_egress.append(&mut backbone_egress);
        match progress {
            semio_framework::kernel::CommandBatchProgress::Complete => {
                self.pending_command_closes.remove_terminal(u64::from(instance), seq).map_err(|fault| Self::not_wired("terminal command owner", format!("{}: {}", fault.code.0, fault.message)))?;
                let mut pending = self.pending_exchanges.remove(instance).expect("terminal pending exchange");
                self.await_response(instance, seq, &mut pending.response).map_err(|fault| Fault { code: fault.code, message: format!("{}; the acknowledging turn published {}", fault.message, named_shapes(&published)) })?;
                pending.response.take(seq).map(CommandTurn::Settled)
            }
            semio_framework::kernel::CommandBatchProgress::Faulted => {
                if let Some(pending) = self.pending_exchanges.get_mut(instance) {
                    if pending.ingress_fault.is_none() {
                        pending.ingress_fault = Some(ingress_fault_text(&turn.command_ingress));
                    }
                }
                self.pending_command_closes.begin_close(u64::from(instance), seq).map_err(|fault| Self::not_wired("faulted command owner", format!("{}: {}", fault.code.0, fault.message)))?;
                let (response_complete, _) = self.pending_exchanges.close_response_step(instance, semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES);
                if !response_complete {
                    return Ok(CommandTurn::MoreWork);
                }
                let (complete, _, _) = self.pending_command_closes.close_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES);
                if complete {
                    let terminal = self.pending_exchanges.remove(instance).expect("faulted exchange reached an empty terminal owner");
                    assert!(terminal.response.terminal_is_empty(), "faulted pending response reached terminal empty");
                    Err(Self::not_wired("command ingress", format!("the guest refused seq {seq}: {}", terminal.ingress_fault.unwrap_or_else(|| "no reason reported".to_string()))))
                } else {
                    Ok(CommandTurn::MoreWork)
                }
            }
            semio_framework::kernel::CommandBatchProgress::PageReady | semio_framework::kernel::CommandBatchProgress::Waiting => Ok(CommandTurn::MoreWork),
        }
    }

    /// 📬️ Admits one effect, if it is THIS command's answer.
    ///
    /// 🧭️ **A command's answer is not an `Effect::Respond`.** The reactor publishes `Respond` for
    /// exactly one inbound event — `Event::Request`, the extension-capability seam
    /// (`⚛️reactor/🔄️turn/🦀️.rs`'s `inbound_request_effects`) — which this gateway never sends. A
    /// command's answer goes through `route_app_frame`, which addresses every non-`UiPatch`
    /// `AppFrame` to `Effect::SendMessage{target: Shell{instance}}` as encoded frame bytes,
    /// correlated by the frame's own `in_reply_to` against the COMMAND's `seq`. The React host reads
    /// exactly this lane (`🔌️PluginRuntime/🟦️.tsx`'s `shellFrameBytes`, "a `Shell{instance}`
    /// send-message IS an `AppFrame` reply"), which is why mutation verbs have always worked there
    /// and never here.
    ///
    /// 🐛️ Reading the respond lane meant the answer was *sitting in the same turn's effects*,
    /// unread: the guest drove ingress to `Complete`, published its frames, and went `Idle` — and the
    /// host reported "the guest acknowledged command seq 1 and then went idle without publishing a
    /// response for it" (ticket 26/09/18, WR1 §5.4 → WR2; measured: the acknowledging turn published
    /// `sendMessage(shell:0, Document(in_reply_to 1), 579 B)`).
    ///
    /// 🏁️ `AppFrame::Done` naming this seq is a reply STAMP, not an answer of its own. Every
    /// transaction route publishes its terminal frame AND a `Done` on the same turn
    /// (`🔌️plugin/🦀️.rs`'s `TransactionPrepare`/`Commit`/`Rollback` arms each push both), so
    /// admitting the second one reported "the guest published more than one frame answering seq
    /// N" for a guest that answered exactly once — measured 2026-09-20 the first time a real
    /// `TransactionPrepare` crossed the headless ingress. A `Done` still IS the answer for the
    /// verbs that publish nothing else (`LoadDocumentArchive`, `TransactionUndo`/`Redo`), which is why
    /// it is dropped only once a real frame is already held. A genuine duplicate — two answering
    /// frames neither of which is a stamp — stays as loud as WR2 made it.
    fn admit_reply_frame(pending: &mut PendingExchange, instance: u32, seq: u64, effect: &semio_framework::kernel::Effect) {
        let Some(payload) = shell_app_frame_payload(instance, effect) else {
            return;
        };
        let Ok(frame) = ::semio_framework_async::poll::resolve_ready(store::decode_app_frame(payload)) else {
            return;
        };
        if matches!(frame, store::AppFrame::Done { .. }) && pending.response.holds_frame() {
            return;
        }
        if app_frame_reply_seq(&frame) == Some(seq) || (app_frame_reply_seq(&frame).is_none() && app_frame_is_transaction_terminal(&frame)) {
            pending.response.admit_frame(payload);
        }
    }

    /// 📮️ Wakes the guest until it publishes the frame answering `seq`, once its command ingress has
    /// gone terminal.
    ///
    /// 🧭️ Acknowledging a command and ANSWERING it are two different events on this wire.
    /// `CommandBatchProgress::Complete` means the guest consumed every page and acknowledged the
    /// batch; the answer is a frame it publishes when the work is done — usually on the very turn
    /// that acknowledges, sometimes on a later one. Bounded by [`COMMAND_RESUME_WALL_BUDGET`].
    ///
    /// 🏁️ A settled command that published no frame naming itself is **not** a fault: it is a
    /// command whose verb has nothing to say, and the answer is a stamped `AppFrame::Done` — the
    /// same rule the React host applies (`commandIngressNeedsReplyStampV1`). Making it a fault here
    /// would refuse every archive admission/`TransactionUndo` the guest completed silently. What stays
    /// a fault is a guest that answers ANOTHER request, because that is a routing defect.
    ///
    /// 🛑️ A guest that is `Idle` with no wake pending has nothing left to do, so waiting the
    /// rest of the wall is pointless. A `Respond` under a request id this gateway never sent
    /// is the one shape that is still a defect worth naming.
    fn await_response(&mut self, instance: u32, seq: u64, response: &mut PendingResponsePage) -> Result<(), Fault> {
        let deadline = std::time::Instant::now() + COMMAND_RESUME_WALL_BUDGET;
        let mut foreign: Vec<u64> = Vec::new();
        while matches!(response, PendingResponsePage::Empty) {
            let runtime = Arc::clone(&self.runtime);
            let guest = self.instances.get_mut(&instance).ok_or_else(|| Self::not_wired("response", format!("no open instance {instance}")))?;
            let turn = match semio_framework_async::block_on(runtime.execute_turn(guest, std::slice::from_ref(&semio_framework::kernel::Event::Wake), headless_command_budget())) {
                Ok(turn) => turn,
                Err(error) => {
                    self.discard_instance(instance);
                    return Err(Self::not_wired("response", error));
                }
            };
            let idle = matches!(turn.status, semio_framework::kernel::TurnStatus::Idle) && turn.next_wake.is_none();
            for effect in &turn.effects {
                if let semio_framework::kernel::Effect::Respond { req, .. } = effect {
                    foreign.push(req.0);
                }
                let Some(payload) = shell_app_frame_payload(instance, effect) else {
                    continue;
                };
                let Ok(frame) = ::semio_framework_async::poll::resolve_ready(store::decode_app_frame(payload)) else {
                    continue;
                };
                if app_frame_reply_seq(&frame) == Some(seq) || (app_frame_reply_seq(&frame).is_none() && app_frame_is_transaction_terminal(&frame)) {
                    response.admit_frame(payload);
                }
            }
            if matches!(response, PendingResponsePage::Empty) && idle {
                if !foreign.is_empty() {
                    return Err(Fault { code: "channel.not-wired".to_string(), message: format!("the guest acknowledged command seq {seq} and answered request(s) {foreign:?} instead") });
                }
                response.stamp_settled();
                return Ok(());
            }
            if std::time::Instant::now() >= deadline {
                return Err(Self::budget_fault(&format!("the guest acknowledged command seq {seq} but published no response in")));
            }
        }
        Ok(())
    }
}

//#endregion 🔖️ArtifactChannel
