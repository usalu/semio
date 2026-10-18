//! 🔦️ More-work sources and tracing of the reactor turn: the settled and busy more-work sources, the late UI retirement, the turn event note and the guest memory and phase retention traces. See [`super`].

use super::*;

/// 🔦️ The named sources a turn folds into `TurnStatus::MoreWork`, recorded once per turn.
///
/// An idle actor must answer `Idle`: every `MoreWork` costs the host another turn round trip, and at
/// the browser's cadence that is the difference between a settled app and a permanently hot reactor.
/// A law that asserts "idle" needs to say WHICH source stayed armed, so the reading is structural
/// rather than a boolean. Costs one `Cell` store per turn. See `📓️idle-turns-2026-09-10.md`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TurnMoreWorkSources {
    /// ⏱️ The reactor executor still had ready work when its 8 ms slice expired.
    pub executor_deadline: bool,
    /// 🧵️ The cooperative process-worker pool advanced a worker this turn.
    pub process_pool: bool,
    /// 🚪️ An instance close ladder has un-run cleanup steps.
    pub close_cleanup: bool,
    /// 🧬️ A typed operation continuation has not reached its terminal state.
    pub typed_operation: bool,
    /// 🔒️ OBSERVATION, never work: another owner held an instance's operation authority when the
    /// scan ran, so the scan could not answer. Deliberately outside [`Self::any`] — see the fold in
    /// `poll_kernel_turn`.
    pub typed_operation_contended: bool,
    /// 🩹️ A surface reconcile has publishable or unpublished patch work.
    pub reconcile: bool,
    /// 🔁️ Resumed async tasks remained undrained at the resume budget.
    pub resumes: bool,
    /// 🧵️ The reactor executor holds tasks that are parked, not ready.
    pub executor_pending: bool,
    /// 📥️ A command ingress slot still owns an in-assembly command.
    pub command_ingress: bool,
    /// 📨️ An original extension request still owes invocation, publication or paid retirement.
    pub extension_invocation: bool,
    /// 🚪️ The guest lifecycle registry owes a receipt or a transition.
    pub lifecycle: bool,
    /// ♻️ UI owners released during this turn still owe bounded retirement work.
    pub ui_retirement: bool,
    /// 🛬️ A cold-pair or checkpoint-restored document still loads through the stepped archive load.
    pub document_load: bool,
}

impl TurnMoreWorkSources {
    /// 😴️ The reading of a turn that answered `Idle`.
    pub const SETTLED: Self = Self {
        executor_deadline: false,
        process_pool: false,
        close_cleanup: false,
        typed_operation: false,
        typed_operation_contended: false,
        reconcile: false,
        resumes: false,
        executor_pending: false,
        command_ingress: false,
        extension_invocation: false,
        lifecycle: false,
        ui_retirement: false,
        document_load: false,
    };

    /// 🔦️ Whether any source is armed — equal to the turn's `MoreWork` verdict.
    pub fn any(self) -> bool {
        !self.names().is_empty()
    }

    /// 🏷️ The armed source names, for a law's failure message.
    pub fn names(self) -> Vec<&'static str> {
        [
            (self.executor_deadline, "executor_deadline"),
            (self.process_pool, "process_pool"),
            (self.close_cleanup, "close_cleanup"),
            (self.typed_operation, "typed_operation"),
            (self.reconcile, "reconcile"),
            (self.resumes, "resumes"),
            (self.executor_pending, "executor_pending"),
            (self.command_ingress, "command_ingress"),
            (self.extension_invocation, "extension_invocation"),
            (self.lifecycle, "lifecycle"),
            (self.ui_retirement, "ui_retirement"),
            (self.document_load, "document_load"),
        ]
        .into_iter()
        .filter_map(|(armed, name)| armed.then_some(name))
        .collect()
    }
}

/// 🧹️ Retires late-built children and documents before their released arguments and keeps queued ownership runnable.
#[doc(hidden)]
pub fn close_late_ui_retirement() -> Result<bool, semio_framework::Fault> {
    let built_complete = ui_contract::close_built_node_page_one();
    let documents = ui_contract::close_ui_document_page_with_grant(PATCH_RETIREMENT_ITEMS_PER_UNIT, PATCH_RETIREMENT_BYTES_PER_UNIT).map_err(reactor_close_fault)?;
    let values = ui_contract::close_ui_value_page_with_grant(PATCH_RETIREMENT_ITEMS_PER_UNIT, PATCH_RETIREMENT_BYTES_PER_UNIT).map_err(reactor_close_fault)?;
    Ok(!built_complete || !documents.complete || !values.complete)
}

/// 🩺️ Records what this turn was handed so the per-turn memory line can put growth beside input
/// size — the one comparison that separates "the events retain" from "the call itself leaks". Only
/// the component boundary can price the events, and only wasm has a linear memory to price.
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
pub fn note_turn_events(count: usize, payload_bytes: usize) {
    TURN_EVENTS.set((count, payload_bytes));
}

/// 🔦️ The [`TurnMoreWorkSources`] of the most recent turn on this actor's thread.
pub fn last_turn_more_work_sources() -> TurnMoreWorkSources {
    LAST_MORE_WORK_SOURCES.get()
}

/// 🧮️ Attributes a turn's retained bytes to the PHASE that kept them. A settled turn must retain
/// nothing; the wasm harness measured 4 437 B per turn against an instance-less guest answering
/// `Idle`, so the owner is a framework phase, not app work. Only weighs anything in a binary that
/// installed `semio_framework_trace::HeapWitness`, and only under `SEMIO_RUNTIME_DIAGNOSTICS` — which
/// a guest arms for itself with `set_runtime_diagnostics` when it ships the witness at all.
/// See `📓️idle-turns-2026-09-10.md`.
pub(super) fn trace_turn_phase_retention(phase: &str) {
    if !semio_framework_trace::runtime_diagnostics_enabled() {
        return;
    }
    let retained = semio_framework_trace::retained_heap_bytes();
    let previous = TURN_PHASE_RETENTION.replace(retained);
    trace_guest_line(&format!("[TRACE] turn phase {phase}: retained {retained} B, delta {}", retained - previous));
}

/// 🗣️ One diagnostic line, out where it can actually be read. A `wasm32-wasip2` guest's `stderr` is
/// whatever `WasiCtx` the host built — nothing, in the Wasmtime harness and in the browser — while
/// the actor world's own `log` import always lands somewhere the host prints. Off wasm the two are
/// the same stream, so this stays one call site.
pub(super) fn trace_guest_line(line: &str) {
    #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
    ::semio_framework_async::poll::resolve_ready(semio_framework_plugin_abi::wasip2::log("debug", line));
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    eprintln!("{line}");
}

/// 🧮️ Weighs the guest's own linear memory once per turn — one `memory.size` on wasm, nothing at all
/// off it. Reaching the wasm `maximum` is a bare `rust_oom` → `unreachable` with no Rust panic and no
/// diagnosis (boot #9b trapped at `189328 bytes failed` inside the exported `poll`'s own task box),
/// so the guest says how full it is BEFORE it traps: once, on first crossing the schema's install-peak
/// ceiling, unconditionally — and every 16 MiB of further growth under
/// [`semio_framework_trace::runtime_diagnostics_enabled`]
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub(super) fn trace_guest_memory_pressure() {
    let Some(bytes) = semio_framework_trace::guest_linear_memory_bytes() else { return };
    let witnessed = GUEST_MEMORY_WITNESS.get();
    let turn = GUEST_MEMORY_TURN.get().saturating_add(1);
    GUEST_MEMORY_TURN.set(turn);
    let delta = bytes as i64 - witnessed as i64;
    if bytes > witnessed {
        GUEST_MEMORY_WITNESS.set(bytes);
        if bytes >= semio_framework_trace::guest_linear_memory_install_peak_ceiling_bytes() && !GUEST_MEMORY_REPORTED.replace(true) {
            eprintln!(
                "guest linear memory at {bytes} B — {}% of the {} B budget, past the {}% install-peak ceiling",
                semio_framework_trace::guest_linear_memory_percent(bytes),
                semio_framework_trace::GUEST_LINEAR_MEMORY_MAXIMUM_BYTES,
                semio_framework_trace::GUEST_LINEAR_MEMORY_INSTALL_PEAK_PERCENT
            );
        }
    }
    if semio_framework_trace::runtime_diagnostics_enabled() {
        let (events, events_bytes) = TURN_EVENTS.get();
        trace_guest_line(&format!(
            "[TRACE] guest linear memory turn={turn} bytes={bytes} delta={delta} percent={} events={events} eventsBytes={events_bytes} elapsed_us={}",
            semio_framework_trace::guest_linear_memory_percent(bytes),
            guest_turn_executing_us().map(|us| us.to_string()).unwrap_or_else(|| "unmeasured".into())
        ));
    }
}

pub(super) fn native_close_key<PA: semio_framework_plugin_reactor_app::PluginApp>(runtime: &semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>, instance: u32) -> Result<instance_lifetime::NativeCloseKey, semio_framework::Fault> {
    let lifetimes = runtime.guest_lifetimes.try_borrow().map_err(|_| reactor_close_fault("lifecycle authority busy"))?;
    lifetimes.get(instance).filter(|slot| slot.cell.is_live()).and_then(|slot| slot.cell.owner()).map(|owner| owner.key()).ok_or_else(|| reactor_close_fault("instance has no acknowledged live lifetime"))
}
