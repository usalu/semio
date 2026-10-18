extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
use semio_framework_plugin_reactor_core::{ARMED_TIMERS, PRESENCE, ReactorClosePeers, SURFACE_RENDER_FAULT_CODE, admit_package_assets, advance_command_cursor, reconcile_step_opportunities, shell_fault_effect, terminal_command_ingress};
use semio_framework::kernel::{ActorInstanceLifecycleReceipt, ActorUiPatchReceipt, Effect, Event, MessageEndpoint, TurnStatus, UiPatch};
use semio_framework_plugin_reactor_surfaces::{COLD_PAIR_DOCUMENT_LOAD, COLD_PAIR_INGRESS, RESTORED_DOCUMENT_LOADS, checkpoint};
use std::cell::{Cell, RefCell};
use semio_framework_ui_runtime::{DEFAULT_REVISION_TOLERANCE, is_stale_intent};
use semio_framework_plugin_reactor_tasks::{INSTANCE_METADATA, JOB_RENDER_BINDINGS, REACTOR_EXECUTOR, REGISTRY, TASK_EXECUTOR, TASK_RESUMES};
use semio_framework_plugin_reactor_patches::PATCHES;
use semio_framework_plugin_reactor_tasks::tasks_fixed_slots::PLUGIN_REACTOR_INSTANCE_SLOTS;
use semio_framework_plugin_reactor_lifecycle::{REACTOR_CLOSES, REACTOR_CLOSE_CURSOR, instance_lifetime};
#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
use semio_framework::kernel::RequestOutcome;
use semio_framework_plugin_reactor_tasks::tasks_task_lane::{TaskResumeOutcome, cancel_instance_tasks_step};
use semio_framework_plugin_reactor_tasks::tasks_extension_continuation::{append_extension_response_page, host};
use semio_framework_plugin_reactor_surfaces as cold_pair;
use semio_framework_plugin_reactor_patches as patches;
use semio_framework_plugin_reactor_lifecycle as pending;
use crate as turn;
use semio_framework_ui_contract as ui_contract;
use semio_framework_plugin_actor_host::{executor, requests};
use semio_framework_plugin_reactor_lifecycle::life_reactor_close::reactor_close_fault;
use crate::turn_close_step::step_reactor_close;
#[doc(hidden)]
#[path="📥️receiving/🦀️.rs"]
pub mod original_receiving;
pub(crate) fn original_receiving_sources_empty(key:instance_lifetime::NativeCloseKey)->bool{original_receiving::response_key_empty(key)&&original_receiving::intent_key_empty(key)}

use semio_framework_plugin_extension::extension_invocation_failure::{ExtensionInvocationCause, RetainedExtensionFaultReply};

#[path = "🔌️extension-invocation-and-command-ingress/🦀️.rs"]
mod extension_invocation_and_command_ingress;
pub use extension_invocation_and_command_ingress::*;

thread_local!{
    static RETAINED_EXTENSION_INVOCATIONS:RefCell<[Option<RetainedExtensionInvocation>;requests::REQUEST_SLOTS]>=const{RefCell::new([const{None};requests::REQUEST_SLOTS])};
    static RETAINED_EXTENSION_CURSOR:Cell<usize>=const{Cell::new(0)};
}

semio_framework_plugin_abi::component_persistent_local! {
    static COMMAND_INGRESS: RefCell<[Option<RetainedCommandIngress>; 2]> = RefCell::new([None, None]);
}

thread_local! {
    /// 🐞️ `[TRACE]` more-work streak trace: (current streak, total more-work turns) — temporary, ticket 26/09/02/PUZZLE-3D-END-TO-END.
    static MORE_WORK_TRACE: RefCell<(u64, u64)> = const { RefCell::new((0, 0)) };
    /// 🧮️ Largest guest linear-memory reading this actor has witnessed, in bytes.
    static GUEST_MEMORY_WITNESS: Cell<usize> = const { Cell::new(0) };
    /// 🧮️ Whether the install-peak ceiling has already been reported for this actor.
    static GUEST_MEMORY_REPORTED: Cell<bool> = const { Cell::new(false) };
    /// 🎯️ Turns sampled by [`trace_guest_memory_pressure`] on this actor.
    static GUEST_MEMORY_TURN: Cell<u64> = const { Cell::new(0) };
    /// 🩺️ `(event count, bulk payload bytes)` the current turn was handed, for the memory trace.
    static TURN_EVENTS: Cell<(usize, usize)> = const { Cell::new((0, 0)) };
    /// 🧮️ Retained-heap reading at the previous [`trace_turn_phase_retention`] probe.
    static TURN_PHASE_RETENTION: Cell<isize> = const { Cell::new(0) };
    /// 🔦️ Which sources answered `MoreWork` on the most recent turn.
    static LAST_MORE_WORK_SOURCES: Cell<TurnMoreWorkSources> = const { Cell::new(TurnMoreWorkSources::SETTLED) };
}

#[path = "🔦️more-work-sources-and-tracing/🦀️.rs"]
mod more_work_sources_and_tracing;
pub use more_work_sources_and_tracing::*;

#[path = "🎯️dirty-poll-surfaces/🦀️.rs"]
mod dirty_poll_surfaces;
pub use dirty_poll_surfaces::*;

thread_local! {
    /// 🎟️ This actor's [`ArenaStarvedSurfaces`].
    static ARENA_STARVED: RefCell<ArenaStarvedSurfaces> = RefCell::new(ArenaStarvedSurfaces { slots: [const { None }; DIRTY_RENDER_CAPACITY] });
}

//#region ⏱️TurnExecution

#[path = "⏱️turn-execution-and-poll-entry/🦀️.rs"]
mod turn_execution_and_poll_entry;
pub use turn_execution_and_poll_entry::*;

thread_local! {
    static TURN_EXECUTION: Cell<TurnExecution> = const { Cell::new(TURN_EXECUTION_IDLE) };
}

//#endregion ⏱️TurnExecution

#[path = "🌀️poll-kernel-turn/🦀️.rs"]
mod poll_turn;
use poll_turn::*;

#[path = "🧊️cold-pair-loads-and-process-pool/🦀️.rs"]
mod cold_pair_loads_and_process_pool;
use cold_pair_loads_and_process_pool::*;

#[cfg(target_arch = "wasm32")]
semio_framework_plugin_abi::component_persistent_local! {
    /// 🧵️ The one process worker pool, resolved once instead of once per turn.
    ///
    /// `process_worker_pool` re-reads `available_parallelism`, rebuilds a `WorkerPoolConfig` and
    /// re-asserts it against the established one on every call; `pump_process_worker_pool` runs on
    /// EVERY reactor turn, so that was a per-turn cost on the guest's hottest path for a handle that
    /// can never change.
    static PROCESS_POOL: semio_framework_async::WorkerPool = {
        let cores = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
        semio_framework_async::process_worker_pool(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, cores))
    };
}

#[path = "🧯️retirement-drivers-and-turn-limits/🦀️.rs"]
mod retirement_drivers_and_turn_limits;
pub use retirement_drivers_and_turn_limits::*;

#[path = "🩹️reconcile-drive-and-publication-receipts/🦀️.rs"]
mod reconcile_drive_and_publication_receipts;
pub use reconcile_drive_and_publication_receipts::*;

semio_framework_plugin_abi::component_persistent_local!{static REFUSED_PUBLICATION:RefCell<Option<RefusedPublicationOwner>>=RefCell::new(None);}

use semio_framework_plugin_reactor_lifecycle::{parse_surface_instance, with_state as with_pending_patches};

#[path = "📬️task-resumes-and-wire-effects/🦀️.rs"]
mod task_resumes_and_wire_effects;
pub use task_resumes_and_wire_effects::*;

#[path = "🏁️close-step/🦀️.rs"]
pub mod turn_close_step;
