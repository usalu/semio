
/// 🔦️ The work the executors reported on one turn.
#[derive(Clone, Copy)]
pub(super) struct TurnExecutorWork {
    deadline: bool,
    task: bool,
    pool: bool,
}

/// 🧮️ Every source the turn read before deciding whether another turn is needed.
pub(super) struct TurnWorkInputs {
    pub(super) events_pending: bool,
    pub(super) executors: TurnExecutorWork,
    pub(super) close_cleanup_work: bool,
    pub(super) typed_operation_runnable: bool,
    pub(super) typed_operation_contended: bool,
    pub(super) reconcile_work: bool,
    pub(super) resumes_remain: bool,
    pub(super) document_load_work: bool,
}

/// 🏃️ Runs the reactor and task executors to their deadline, pumps the process worker pool and admits the registry's effects.
pub(super) fn drive_turn_executors(effects: &mut Vec<Effect>) -> TurnExecutorWork {
    let executor_deadline_work = REACTOR_EXECUTOR.with(|executor| executor.run_until_deadline(64, 256 * 1_024, std::time::Instant::now() + std::time::Duration::from_millis(REACTOR_TURN_EXECUTOR_HOLD_MS)));
    let task_executor_work = TASK_EXECUTOR.with(|executor| executor.run_until_deadline(64, std::time::Instant::now() + std::time::Duration::from_millis(REACTOR_TURN_EXECUTOR_HOLD_MS)));
    let process_pool_work = !executor_deadline_work && pump_process_worker_pool();
    for effect in REGISTRY.with(|registry| registry.drain()) {
        push_admitted_effect(&mut *effects, 0, effect);
    }
    TurnExecutorWork { deadline: executor_deadline_work, task: task_executor_work, pool: process_pool_work }
}

/// 🔦️ Decides whether the turn answers `MoreWork`, records which sources armed it and traces the more-work streak.
pub(super) fn settle_turn_work<PA: semio_framework_plugin_reactor_app::PluginApp>(runtime: &semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>, inputs: TurnWorkInputs, effects: &[Effect]) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
    let TurnWorkInputs { events_pending, executors, close_cleanup_work, typed_operation_runnable, typed_operation_contended, reconcile_work, resumes_remain, document_load_work } = inputs;
    let (executor_deadline_work, task_executor_work, process_pool_work) = (executors.deadline, executors.task, executors.pool);
    let more_work = executor_deadline_work || task_executor_work || process_pool_work;
    let executor_pending = REACTOR_EXECUTOR.with(|executor| executor.has_pending()) || TASK_EXECUTOR.with(|executor| executor.has_pending());
    let command_ingress_pending = COMMAND_INGRESS.with(|ingress| ingress.borrow().iter().any(Option::is_some));
    let lifecycle_work = runtime.guest_lifetimes.borrow().has_work();
    let ui_retirement_work = close_late_ui_retirement()?;
    let extension_invocation_pending=RETAINED_EXTENSION_INVOCATIONS.with(|owners|owners.borrow().iter().any(Option::is_some))||original_receiving::response_pending()||original_receiving::intent_pending();
    let more_work = events_pending || more_work || extension_invocation_pending || close_cleanup_work || typed_operation_runnable || reconcile_work || resumes_remain || executor_pending || command_ingress_pending || lifecycle_work || ui_retirement_work || document_load_work;
    LAST_MORE_WORK_SOURCES.set(TurnMoreWorkSources {
        executor_deadline: executor_deadline_work,
        process_pool: process_pool_work,
        close_cleanup: close_cleanup_work,
        typed_operation: typed_operation_runnable,
        typed_operation_contended,
        reconcile: reconcile_work,
        resumes: resumes_remain,
        executor_pending,
        command_ingress: command_ingress_pending,
        extension_invocation: extension_invocation_pending,
        lifecycle: lifecycle_work,
        ui_retirement: ui_retirement_work,
        document_load: document_load_work,
    });
    trace_turn_phase_retention("render");
    trace_guest_memory_pressure();
    MORE_WORK_TRACE.with(|trace| {
        let mut trace = trace.borrow_mut();
        let (streak, seen) = if more_work { (trace.0 + 1, trace.1 + 1) } else { (0, trace.1) };
        if !semio_framework_trace::runtime_diagnostics_enabled() {
            *trace = (streak, seen);
            return;
        }
        if more_work {
            trace_guest_line(&format!(
                "[TRACE] reactor more-work streak={streak} seen={seen} sources={:?} contended={typed_operation_contended} effects={} patches=[{}] pending=[{}]",
                LAST_MORE_WORK_SOURCES.get().names(),
                effects.len(),
                PATCHES.with(patches::PatchTracker::debug_state),
                with_pending_patches(|pending| pending.borrow().debug_state())
            ));
        } else if trace.0 > 0 {
            trace_guest_line(&format!("[TRACE] reactor more-work streak ended after {} turns (seen={seen})", trace.0));
        }
        *trace = (streak, seen);
    });
    Ok(more_work)
}
