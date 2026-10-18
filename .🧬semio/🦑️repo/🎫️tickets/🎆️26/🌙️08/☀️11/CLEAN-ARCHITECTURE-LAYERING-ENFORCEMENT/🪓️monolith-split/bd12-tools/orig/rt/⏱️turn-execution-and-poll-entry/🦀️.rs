//! ⏱️ Turn execution and poll entry of the reactor turn: the guest turn execution witness, the kernel poll entry points, the boxed plugin exchange and the turn patch budget and page. See [`super`].

use super::*;

//#region ⏱️TurnExecution
/// ⏱️ One turn's EXECUTING microseconds — wall time spent inside the guest's own `Future::poll`
/// calls, with every `Pending` gap excluded. The strict lifecycle authority
/// ([`semio_framework_trace::GUEST_LIFECYCLE_TURN_CEILING_US`]) must bound the guest's own work; a
/// plain `now_us() - started_us` also bills the guest for time it is not running at all — a JSPI
/// suspension parked on a host import, a hidden browser tab whose timer/microtask continuation is
/// throttled, or an OS deschedule of a loaded box. Measured 2026-09-10: generation3d's whole
/// `InstanceOpen` turn executes in 13.9 ms natively, yet its browser first step was killed at the
/// 5 s ceiling, because the wall clock kept running across the descheduled continuations.
#[derive(Clone, Copy)]
pub(super) struct TurnExecution {
    pub accumulated_us: u64,
    poll_started_us: Option<u64>,
    clock_lost: bool,
}

pub(super) const TURN_EXECUTION_IDLE: TurnExecution = TurnExecution { accumulated_us: 0, poll_started_us: None, clock_lost: false };

/// ⏱️ Executing microseconds of the turn in flight, including the poll currently on the stack.
/// `None` means the host clock was missing or ran backward, which the lifecycle authority retains
/// its receipt for exactly like an overrun.
#[doc(hidden)]
pub fn guest_turn_executing_us() -> Option<u64> {
    let execution = TURN_EXECUTION.with(Cell::get);
    if execution.clock_lost {
        return None;
    }
    let Some(poll_started_us) = execution.poll_started_us else { return Some(execution.accumulated_us) };
    Some(execution.accumulated_us.saturating_add(semio_framework_job::default_now_us()?.checked_sub(poll_started_us)?))
}

/// ⏱️ Runs one turn under [`TURN_EXECUTION`] accounting; the gaps between polls never accrue.
///
/// 📏️ Takes the turn future ALREADY PINNED, by pointer. Taking it `impl Future` by value cost an
/// exact duplicate of the largest generator in the guest: a parameter owns a coroutine slot from
/// `Unresumed` onward, `pin!` then moved it into a second local that lives across the `.await`, and
/// the moved-from parameter slot is never reused — `with_turn_execution` measured 243 184 B around
/// a 121 584 B `poll_kernel_turn`. `Pin<&mut …>` is 8 bytes and the caller owns the one slot. See
/// `📓️poll-task-leak-2026-09-10.md` §3.1.
#[doc(hidden)]
pub async fn with_turn_execution<T>(mut future: std::pin::Pin<&mut (impl std::future::Future<Output = T> + ?Sized)>) -> T {
    TURN_EXECUTION.with(|execution| execution.set(TURN_EXECUTION_IDLE));
    std::future::poll_fn(move |context| {
        TURN_EXECUTION.with(|execution| {
            let mut current = execution.get();
            current.poll_started_us = semio_framework_job::default_now_us();
            current.clock_lost = current.clock_lost || current.poll_started_us.is_none();
            execution.set(current);
        });
        let polled = future.as_mut().poll(context);
        TURN_EXECUTION.with(|execution| {
            let mut current = execution.get();
            match current.poll_started_us.take().zip(semio_framework_job::default_now_us()).map(|(started_us, now_us)| now_us.checked_sub(started_us)) {
                Some(Some(spent_us)) => current.accumulated_us = current.accumulated_us.saturating_add(spent_us),
                _ => current.clock_lost = true,
            }
            execution.set(current);
        });
        polled
    })
    .await
}
//#endregion ⏱️TurnExecution

/// 🧠️ Repository-owned actor ABI entrypoint. The component-model wrapper above and the native
/// interpreter both call this exact kernel reducer, so WIT lifting is no longer the production
/// host's semantic authority.
pub async fn poll_kernel<PA: semio_framework_plugin_reactor_app::PluginApp + 'static>(
    runtime: &semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>,
    events: &mut Option<Event>,
    command_page: Option<(semio_framework::kernel::CommandPageCursor, semio_framework::kernel::FixedCommandPage)>,
    cold_pair_page: Option<semio_framework::kernel::ColdDocumentPairPage>,
    budget: semio_framework::kernel::Budget,
    identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,
    cx:&mut semio_framework_job::StepContext<'_>,
) -> Result<semio_framework::kernel::TurnResult, semio_framework_job::OriginalTurnFailure> {
    poll_kernel_output(runtime, events, command_page, cold_pair_page, budget, |_| Ok(()), |result, ()| result, identity,cx).await
}

/// 🧠️ One guest turn under [`with_turn_execution`] accounting — the only door into [`poll_kernel_turn`].
#[doc(hidden)]
#[expect(clippy::result_large_err, reason = "Patch reservation and publication callbacks return the original fixed surface or patch owner on refusal; these transfers must not allocate an error wrapper.")]
pub async fn poll_kernel_output<PA: semio_framework_plugin_reactor_app::PluginApp, T, Prepared>(
    runtime: &semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>,
    events: &mut Option<Event>,
    command_page: Option<(semio_framework::kernel::CommandPageCursor, semio_framework::kernel::FixedCommandPage)>,
    cold_pair_page: Option<semio_framework::kernel::ColdDocumentPairPage>,
    budget: semio_framework::kernel::Budget,
    prepare: impl FnOnce(&semio_framework::kernel::TurnResult) -> Result<Prepared, semio_framework::Fault>,
    publish: impl FnOnce(semio_framework::kernel::TurnResult, Prepared) -> T,
    identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,
    cx:&mut semio_framework_job::StepContext<'_>,
) -> Result<T, semio_framework_job::OriginalTurnFailure> {
    with_turn_execution(std::pin::pin!(poll_kernel_turn(runtime, events, command_page, cold_pair_page, budget, prepare, publish, identity,cx))).await
}

/// 📏️ `plugin_exchange`'s future is 76 848 B — awaiting it INLINE made it the single largest local
/// of the turn generator, so every `poll` boxed a task future two thirds of which was one callee
/// that most turns never reach. Boxed here, the turn generator holds an 8-byte pointer and the
/// exchange state is allocated only on the turns that actually run a command. See
/// `📓️poll-task-leak-2026-09-10.md` §3.2.
pub(super) fn plugin_exchange_boxed<'a, 'b, 'context, PA: semio_framework_plugin_reactor_app::PluginApp>(
    runtime: &'a semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>,
    instance_id: u32,
    command: Option<(u64, semio_framework_plugin_runtime_ops::ops_ingress::PluginCommandIngress)>,
    identity: &'a mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'b>,
    cx:&'a mut semio_framework_job::StepContext<'context>,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<semio_framework_plugin_runtime_ops::ops_ingress::PluginExchangeOutput, semio_framework_job::OriginalTurnFailure>> + 'a>> {
    Box::pin(async move{
        let mut output=semio_framework_plugin_runtime_exchange::exch_plugin_exchange::plugin_exchange(runtime,instance_id,command,identity,cx).await?;
        if let Err(refusal)=collect_original_publication_receipts(&mut output,cx){return Err(retain_refused_publication(instance_id,output,refusal,cx));}
        Ok(output)
    })
}

/// 📏️ The per-turn patch BYTE budget this turn admits against — the host's own declaration
/// (`Budget::max_patch_bytes`), clamped into the window the guest's declared linear-memory budget
/// leaves open: never below one contiguous request ceiling (a lane that declared less could not carry
/// one document-scaled publication at all) and never above
/// [`semio_framework::kernel::UI_TURN_PATCH_BUDGET_BYTES`].
#[doc(hidden)]
pub fn turn_patch_budget_bytes(budget: semio_framework::kernel::Budget) -> usize {
    usize::try_from(budget.max_patch_bytes).unwrap_or(semio_framework::kernel::UI_TURN_PATCH_BUDGET_BYTES).clamp(semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES, semio_framework::kernel::UI_TURN_PATCH_BUDGET_BYTES)
}

/// 📤️ Fills this turn's patch page with EVERY publication that is ready and fits, and mints the one
/// receipt that authorizes the whole batch.
///
/// 🐛️ Until 2026-09-15 this took exactly one patch per turn, because `UI_TURN_PATCHES_MAXIMUM` was 1:
/// N published surfaces cost N + 1 host round trips by construction, and the React renderer publishes
/// about six per hop (`📓️reactor-reconcile-spin-2026-09-14.md` §7 named this floor and could not move
/// it). The admission rule is now a BYTE budget, not a count:
///
/// 1. the FIRST ready patch is always admitted, whatever it costs — otherwise a publication larger
///    than the whole budget could never leave the guest and the surface would never converge;
/// 2. every further patch joins only while the page's own [`semio_framework::kernel::ui_patch_turn_bytes`]
///    sum stays inside the budget — the one that does not fit is handed straight back to its reserved
///    slot and travels on the next turn, in order, still exactly once;
/// 3. the page's fixed capacity stops the loop at [`semio_framework::kernel::UI_TURN_PATCHES_MAXIMUM`],
///    which is itself derived so the page never becomes a contiguous request past the guest's ceiling.
#[doc(hidden)]
#[expect(clippy::result_large_err, reason = "A refused publication returns its exact patch owner to the reserved pending slot without allocating an error wrapper.")]
pub fn take_turn_patch_page(budget_bytes: usize) -> Result<semio_framework::kernel::UiTurnPatches, semio_framework::Fault> {
    let mut page = semio_framework::kernel::UiTurnPatches::default();
    let mut spent = 0usize;
    while page.len() < semio_framework::kernel::UI_TURN_PATCHES_MAXIMUM {
        let taken = with_pending_patches(|pending| pending.borrow_mut().take_one(semio_framework_ui_runtime::SURFACE_RECONCILE_PAGE_BYTES))
            .map_err(|reason| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.pending-patch-authority"), reason))?;
        let Some(patch) = taken else { break };
        let cost = semio_framework::kernel::ui_patch_turn_bytes(&patch);
        if !page.is_empty() && spent.saturating_add(cost) > budget_bytes {
            with_pending_patches(|pending| pending.borrow_mut().hand_back_turn(patch)).expect("a patch over the turn byte budget returns to its reserved slot");
            break;
        }
        match page.try_push_ui_patch(patch) {
            Ok(()) => spent = spent.saturating_add(cost),
            Err(patch) => {
                with_pending_patches(|pending| pending.borrow_mut().hand_back_turn(patch)).expect("exact unpublished patch returns to its reserved slot");
                break;
            }
        }
    }
    Ok(page)
}
