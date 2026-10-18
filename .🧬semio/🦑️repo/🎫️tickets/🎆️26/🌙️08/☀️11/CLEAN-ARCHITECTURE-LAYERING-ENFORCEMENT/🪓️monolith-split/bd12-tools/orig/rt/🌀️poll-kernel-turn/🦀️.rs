//! 🌀️ Poll kernel turn of the reactor turn: the one reactor turn that polls the kernel, routes events, drives retirement, reconciliation and publication and fills the turn patch. See [`super`].

use super::*;

/// ↩️ One `Effect::Respond` per inbound `Event::Request` this turn served — kept apart from the
/// document-backbone effects so the answer's ordering against them is explicit rather than
/// incidental (answers trail, so a request that also wrote the document publishes the write first).
///
/// 🚪️ A close-cleanup verdict names ONE already-retired lifetime, so it may not kill the turn that
/// every OTHER instance in this actor shares.
///
/// 🐛️ It did: `?` here turned instance 5's `plugin.internal.zero-progress` into a shard worker fault
/// for the whole turn, the host trapped the actor, and instance 6 — a sibling pane mid-boot in the
/// same shard — went to Plugin Recovery with it (six-pane demonstrator boot, ticket 26/08/28). The
/// instance whose cleanup faulted is already gone and has already left quarantine; what is left is a
/// diagnostic, not a live window's crash.
///
/// 🌡️ One fair live-maintenance step per turn — and a burst while the stepped instance reports
/// displaced-owner pressure, bounded by the turn's retirement deadline. A results-window
/// playback (a coalesced window-config amend per frame, three displaced owners each) outran the
/// single step's one owner per 26-stage rotation and saturated its 1 024-slot queue after
/// ~340 frames (ticket 26/09/16/FEM-3D-INTERACTIVE-FEATURE-COMPLETE).
///
/// 🎯️ M1 (ticket 26/08/17 `design-unified.md`): decodes the pack-encoded
/// `ui_contract::UiIntent`, drops it if it targets a tree the user can no longer see (the
/// revision guard, at the reconciler that owns the revision — `PATCHES.revision`,
/// `ui_runtime::is_stale_intent` imported rather than reimplemented), and otherwise
/// batches it per instance for the dispatch pass below (mirrors `app_commands`'
/// batch-then-dispatch shape). Real dispatch replaces the prior packet's "decode-and-
/// mark-dirty" interim — see `📓️terra-sdk-wire-report.md`'s M1 section for the full route.
///
/// 🔁️ Two delivery shapes share one id space. A `RequestRegistry::request` id parks a
/// future and is woken by `resolve`; a `request_continuation` id parks NOTHING and is
/// answered by redispatching its `response_action` into the owning app instance with the
/// outcome merged onto the original request object. Before this branch existed the second
/// shape did not: a plugin that wanted an extension result had to hand-mint a `RequestId`,
/// which owns no registry slot, so `resolve` silently dropped every extension outcome.
///
/// 📄️ A host answer larger than one `GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES` page arrives
/// as PROLOGUE pages here and its terminal page rides the `Event::Completed` above, so
/// `Event::Completed` stays THE one completion door for a `req` and this arm never needs
/// a second dispatch site in the turn generator. A continuation claims its own pages;
/// everything else falls through to the parked-future accumulator.
///
/// 🐛️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (sdk-async): used to discard every
/// non-final chunk outright (`if done { resolve(req, Ok(bytes)) }` — every earlier
/// `bytes` was simply dropped on the floor, silent data loss for any multi-chunk
/// response). `append_chunk` accumulates instead; `cap` is the owning instance's
/// `QuotaSchema.message_bytes` (default 64 MiB when unset/unknown — matches
/// `instance_task_quota`'s own `unwrap_or` fallback idiom above).
///
/// 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (J1, design-abi.md §4): a job spawned
/// through `host::jobs::spawn` (`🌐host/🦀️.rs`) allocates its `job` id from
/// THE SAME `RequestRegistry` counter as every other awaitable `host::*` call — the
/// `Effect::SpawnJob{job, ..}` this actor emitted carried `job == req.0` — so
/// `Event::JobCompleted{job, result}` resolves the identical parked `RequestFuture`
/// an `Event::Completed{req, result}` would, closing the "no `req`-per-job
/// correlation table yet" gap `📓️terra-M5-report.md` §4 named (no separate table
/// needed: the request id already IS the job id).
///
/// 📥️ The inbound half of the `request`/`respond` seam (`📜️.wit`'s `request-event`,
/// "every 'someone else calls INTO this actor' seam is now one inbound `request`,
/// answered with the `respond` effect within a bounded number of turns"). This actor's
/// installed `ExtensionBundle` is the ONE capability table that answers it, so the
/// request is served inline and answered on the SAME turn — an extension handler is a
/// pure `Fn(&[u8]) -> Result<Vec<u8>, Fault>` with nothing to await. An actor with no
/// bundle answers the bundle's own typed refusal (`extension.inactive`/
/// `extension.missing`), never silence: dropping the event stranded every caller's
/// parked request forever (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
///
/// 📥️ A page that names a different command on THIS instance is the host saying it has
/// moved on: it drives one command per instance at a time, so the owner retained here can
/// no longer have a driver. It is superseded into the retiring lane (see
/// `COMMAND_INGRESS_RETIRING_SLOT`) and the host is asked for this page once more, which
/// the now-free live lane admits on the next turn. A page for ANOTHER instance is genuine
/// backpressure — that owner's host driver is still live — and waits as it always did.
///
/// 📥️ Taken only when it IS the assembly this page belongs to. An `if let` on a bare
/// `retained.take()` dropped every other owner shape on the floor — the page went with it,
/// the turn still answered `Idle`, and the host's drain then spun its whole continuation
/// ceiling asking a reactor that no longer owned anything.
///
/// 📥️ The terminal arm this chain never had: a page that matches no admission shape is a
/// page nobody owns, and it says so by name instead of leaving the turn `Idle` with the
/// page silently gone.
///
/// 🎯️ M1: surviving intents dispatch through the SAME `route_app_frame`/effects/events plumbing
/// as `app_commands` above, immediately after it (so a mutation an app command made this turn is
/// already visible to the intent's own dispatch) — via the NEW `plugin_dispatch_intents`, which
/// routes through the app's EXISTING typed command path (`PluginApp::handle_intent_frame` →
/// `ArtifactApp::command_from_intent` → `dispatch_typed_command_inner`), never a parallel path.
/// Each handled batch's surfaces feed the retained render set so the reply patch — the next `UiPatch`
/// revision bump — is produced in this SAME turn (design decision: no new reply channel).
///
/// 🚫️async: E5 executor bridge — `plugin_dispatch_intents` stays genuinely `async fn`; safe to
/// resolve synchronously here for the same reason as `plugin_exchange` above.
///
/// 🌳️ Deduped per instance — several intents on the same surface this turn must not queue a
/// redundant re-render (the second `diff()` would return `None` anyway, but there is no reason
/// to pay for it).
///
/// 🧹️ The SECOND publication retirement of this turn, and the one that matters for the crossing
/// count: the first ran before the events, so every `Event::PatchAck` this turn carried left its
/// slot `Acknowledged` with nothing to retire it until the NEXT turn — one host round trip per
/// published surface whose only work was freeing a slot (the `<n>:ea---` half of the two-state
/// ping-pong, `📓️reactor-reconcile-spin-2026-09-14.md` §1). Bounded by the same units and the same
/// wall deadline as the first run, so a turn cut short still answers `MoreWork` through
/// `PendingPatchAuthority::has_retiring` and finishes on the next one.
///
/// 👥️ M2 (ticket 26/08/17 `design-unified.md`): `now_ms` is read ONCE for both `record_peer`'s
/// expiry stamping below and `PRESENCE.expire` at the end of this turn — a single wall-clock
/// reading per poll, not one per presence update.
///
/// 🕹️ A refused commit THROWS AWAY the tree this turn just rendered and leaves the
/// surface at its previous revision, which the host then reads back as `unchanged` —
/// indistinguishable from a healthy refresh. It is the one exit on this path that can
/// lose a render the host explicitly asked for, so it is recorded as a shell fault
/// naming the surface rather than a `let _ =` (ticket 26/09/02/PUZZLE-3D-END-TO-END
/// wave B46 §8.1, wave B48 §3). Not a turn-fatal `?`: a commit refused because the
/// instance is closing is an ordinary teardown race, and faulting the whole turn for
/// it would break the close ladder.
///
/// 🕹️ A surface whose previous reconcile is still in flight cannot be re-rendered now, so the
/// dirty render is DEFERRED — and a deferred ring with no room left would drop it, which is a
/// render the host asked for that no later turn ever performs. That is exactly the shape of
/// "the guest never re-publishes its selection lane after a pick" (ticket
/// 26/09/02/PUZZLE-3D-END-TO-END wave B46 §8.1: `interactionSelect` settles, no verdict is
/// recorded, and `data-guest-selection-json` stays `selectedIds:[]` for 150 s), so it is a
/// typed fault naming the surface rather than a `let _ =`.
///
/// 🎟️ A refusal by one of the PROCESS-WIDE reconcile tables is not the ordinary "previous
/// reconcile still in flight" race the defer ring exists for: it says this surface cannot be
/// admitted at all until some other surface hands credit back, and while the reservation was
/// priced at the per-surface CEILING that was structural — three surfaces of a thirteen-surface
/// session, refused/deferred/re-dirtied for ever with `effects=0` (ticket
/// 26/09/02/PUZZLE-3D-END-TO-END wave B56 §1.1, wave B58). Priced by its own body it should
/// never happen, so it is reported by NAME, once per refusal, instead of deferring in silence.
///
/// 🚫️async: E5 executor bridge (× 2) — `LocalExecutor::{run_until_idle,has_ready}` stay
/// genuinely `async fn` (its own doc: "run_until_idle handles Pending without ever yielding
/// its own future" — matches `⚛️reactor/💼️jobs`'s identical use of this exact bridge).
///
/// 🧵️ The COLD executor's own slice of the same hold: every `AsyncTask` `spawn_task` admitted
/// lives on `TASK_EXECUTOR`, and until the task lane became a product lane nothing but a native
/// fixture ever polled it — a spawned task would have parked forever inside a real guest. Its
/// answer is "some task is still alive", ready or parked, so it is folded into `more_work` the
/// same way `REACTOR_EXECUTOR`'s is.
///
/// 🔁️ The process-pool pump stays gated on the REACTOR executor alone. `TASK_EXECUTOR`'s answer
/// is "some task is still alive", ready OR PARKED, so folding it in here stopped pumping the
/// pool for as long as any task was parked on a host round trip — which starved every
/// interactive job step behind it (measured 2026-09-22: a command page set never reached its
/// terminal ingress status in 512 turns).
///
/// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4): resumed `AsyncTask` follow-
/// ups (and any replayed `task_restarts` from a `restore` before this turn) — AFTER
/// `run_until_idle` so a task that resolved just now is redispatched the SAME turn, not the
/// next one. A resume can itself spawn more tasks (`dispatch_emit` runs for real), so the
/// executor may have fresh ready work by the time this returns — folded into `more_work` below
/// rather than requiring a second `run_until_idle` pass this turn (the next `poll` picks it up).
///
/// 🔒️ `typed_operation_scan.contended` is deliberately NOT folded in. A busy instance lock is not
/// an answer of "there is runnable work", it is "another owner is mid-step and I could not look" —
/// and the guest is single-threaded, so on wasm it can only ever be a reader inside this very turn.
/// Answering `MoreWork` for it turned every concurrent read into a host turn round trip that
/// produced nothing: 12 of 512 idle generation3d turns, measured 2026-09-10. Every owner that can
/// hold that lock is itself either a reactor executor task (`executor_pending`), a task resume
/// (`resumes`), an ingress command (`command_ingress`) or a lifecycle step (`lifecycle`), each of
/// which already arms this turn on its own account. See `📓️idle-turns-2026-09-10.md`.
///
/// 👥️ M2: once per poll — expire ages-out peer marks, then flush drains every key touched since
/// the last flush into one coalesced `PresenceUpdate` each (free burst coalescing: a hover storm
/// between polls still costs exactly one update per `(surface, node_key)`).
#[expect(clippy::result_large_err, reason = "Patch reservation and publication callbacks return the original fixed surface or patch owner on refusal; these transfers must not allocate an error wrapper.")]
pub(super) async fn poll_kernel_turn<PA: semio_framework_plugin_reactor_app::PluginApp, T, Prepared>(
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
    budget.retained.validate()?;
    if budget.retained.operation!=cx.operation().0||budget.retained.generation!=cx.generation().0||budget.retained.grant!=cx.retained_grant(){return Err(reactor_close_fault("original reactor retained input does not match its borrowed context").into());}
    let original_progress=cx.retained_progress();let original_work=cx.original_work_units_spent();
    if REFUSED_PUBLICATION.with(|owner|owner.borrow().is_some()){return Err(reactor_close_fault("original refused publication still owns its output and receipts").into());}
    trace_turn_phase_retention("enter");
    let retryable_lifecycle = command_page.is_none() && cold_pair_page.is_none() && events.iter().all(|event| matches!(event, Event::InstanceOpen { .. } | Event::InstanceClose(_) | Event::InstanceLifecycleAck(_)));
    let mut dirty = DirtyPollOwners::new();
    let mut focus = None;
    for event in events.iter() {
        let instance = match event {
            Event::InstanceOpen { request, .. } => Some(request.instance_id),
            Event::InstanceClose(request) => Some(request.lifetime.instance_id),
            Event::InstanceLifecycleAck(ack) => Some(match ack.receipt {
                ActorInstanceLifecycleReceipt::Captured { lifetime, .. } | ActorInstanceLifecycleReceipt::Accepted { lifetime, .. } | ActorInstanceLifecycleReceipt::Retired { lifetime, .. } => lifetime.instance_id,
            }),
            _ => None,
        };
        if let Some(instance) = instance {
            if focus.replace(instance).is_some() {
                return Err(reactor_close_fault("one lifecycle command is admitted per turn").into());
            }
        }
    }
    for event in events.iter() {
        match event {
            Event::InstanceClose(request) => {
                let mut lifetimes = runtime.guest_lifetimes.try_borrow_mut().map_err(|_| reactor_close_fault("lifecycle authority busy"))?;
                let slot = lifetimes.get_mut(request.lifetime.instance_id).ok_or_else(|| reactor_close_fault("close lifetime absent"))?;
                slot.cell.validate_close(*request).map_err(reactor_close_fault)?;
                slot.cell.owner_mut().ok_or_else(|| reactor_close_fault("close native owner absent"))?.request_close(*request, runtime, &ReactorClosePeers)?;
            }
            Event::InstanceLifecycleAck(ack) => {
                let mut lifetimes = runtime.guest_lifetimes.try_borrow_mut().map_err(|_| reactor_close_fault("lifecycle authority busy"))?;
                lifetimes.get_mut(focus.expect("ACK focus")).ok_or_else(|| reactor_close_fault("ACK lifetime absent"))?.cell.stage_ack(*ack).map_err(reactor_close_fault)?;
            }
            _ => {}
        }
    }
    if focus.is_none() {
        focus = runtime.guest_lifetimes.borrow_mut().next_work();
    }
    if let Some(instance) = focus {
        let mut lifetimes = runtime.guest_lifetimes.borrow_mut();
        if let Some(slot) = lifetimes.get_mut(instance) {
            if let Some(admission) = slot.cell.owner_mut().map(|owner| owner.advance_admission(runtime, &ReactorClosePeers)).transpose()?.flatten() {
                slot.cell.record_close_admission(admission.0, admission.1).map_err(reactor_close_fault)?;
            }
        }
    }
    let mut close_instances: Vec<u32> = events.iter().filter_map(|event| if let Event::InstanceClose(request) = event { Some(request.lifetime.instance_id) } else { None }).collect();
    let mut document_backbone_effects = Vec::new();
    let mut inbound_request_effects: Vec<Effect> = Vec::new();
    trace_turn_phase_retention("lifecycle");
    retire_while_progress_fallible(cx, REACTOR_CLOSE_UNITS_PER_TURN, step_reactor_close)?;
    retire_while_progress_bounded(cx, REACTOR_CLOSE_UNITS_PER_TURN, || COLD_PAIR_INGRESS.with(|ingress| ingress.borrow_mut().advance_close_one()));
    PATCHES.with(|patches| -> Result<(),semio_framework_job::OriginalTurnFailure> {
        for _ in 0..PATCH_CLOSE_UNITS_PER_TURN {
            if cx.should_yield() {
                break;
            }
            if patches.close_step(cx)? {
                break;
            }
        }
        Ok(())
    })?;
    for _ in 0..PATCH_CLOSE_UNITS_PER_TURN {
        if cx.should_yield(){break}
        cx.consume_fuel(1);
        if semio_framework_ui_runtime::close_surface_reconcile_handback_one().map_err(reactor_close_fault)? {
            break;
        }
        if cx.should_yield() {
            break;
        }
    }
    retire_until_complete(cx, || ui_contract::close_ui_document_page_with_grant(PATCH_RETIREMENT_ITEMS_PER_UNIT, PATCH_RETIREMENT_BYTES_PER_UNIT).expect("queued document retirement remains valid").complete);
    retire_until_complete(cx, semio_framework_ui_contract_validation::close_ui_patch_owner_one);
    retire_until_complete(cx, || ui_contract::close_ui_value_page_with_grant(PATCH_RETIREMENT_ITEMS_PER_UNIT, PATCH_RETIREMENT_BYTES_PER_UNIT).expect("exact UI value retirement queue remains valid").complete);
    retire_until_complete(cx, ui_contract::close_built_node_page_one);
    retire_while_progress(cx, || semio_framework::kernel::close_ui_turn_patch_owner_with_grant(PATCH_RETIREMENT_ITEMS_PER_UNIT, PATCH_RETIREMENT_BYTES_PER_UNIT));
    retire_while_progress(cx, || {
        matches!(semio_framework::kernel::close_ui_turn_patch_transport_with_grant(PATCH_RETIREMENT_ITEMS_PER_UNIT, PATCH_RETIREMENT_BYTES_PER_UNIT), Ok(semio_framework::kernel::UiTurnPatchTransportProgress::Pending { .. }))
    });
    with_pending_patches(|pending| pending.borrow_mut().advance_rejection(|surface, generation| PATCHES.with(|patches| patches.mark_rejected(surface, generation))));
    for _ in 0..PATCH_CLOSE_UNITS_PER_TURN {
        if cx.should_yield() {
            break;
        }
        if with_pending_patches(|pending| pending.borrow_mut().close_step(PATCH_RETIREMENT_ITEMS_PER_UNIT, PATCH_RETIREMENT_BYTES_PER_UNIT)).map_err(reactor_close_fault)? {
            break;
        }
    }
    let close_cleanup_work=semio_framework_plugin_runtime_instances::inst_cleanup_turns::plugin_step_close_cleanup(runtime,cx)?;
    for _ in 0..semio_framework_plugin_runtime_instances::inst_cleanup_turns::LIVE_CLEANUP_PRESSURE_STEPS_PER_TURN {
        if cx.should_yield(){break}
        cx.consume_fuel(1);
        let _ = semio_framework_plugin_runtime_instances::inst_cleanup_turns::plugin_step_live_cleanup(runtime,cx)?;
        if !semio_framework_plugin_runtime_instances::inst_cleanup_turns::plugin_live_cleanup_under_pressure(runtime) || cx.should_yield() {
            break;
        }
    }
    semio_framework_plugin_runtime_instances::inst_opening::plugin_resume_actor_openings(runtime,identity,cx).await?;
    PATCHES.with(|patches| redirty_acknowledged_deferred_surfaces(patches, &mut dirty,cx))?;
    redirty_arena_starved_surfaces(&mut dirty)?;
    if matches!(events.as_ref(),Some(Event::Completed { .. })) {
        original_receiving::retain_completed(runtime,events,cx)?;
    }
    let selected_event = if matches!(events.as_ref(),Some(Event::Completed { .. })) {
        None
    } else if events.is_some() {
        let grant=cx.retained_grant();
        if cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_depth==0 { None } else {
            cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()})?;
            cx.consume_fuel(1);
            events.take()
        }
    } else { None };
    for event in selected_event {
        match event {
            Event::InstanceOpen { request, app_id, actor, assets, quotas, .. } => {
                let instance = request.instance_id;
                let fresh = runtime.guest_lifetimes.borrow_mut().admit(request).map_err(reactor_close_fault)?;
                if fresh {
                    semio_framework_plugin_reactor_core::admit_package_assets(assets);
                    if let Err(error) = INSTANCE_METADATA.with(|metadata| metadata.borrow_mut().insert(instance, app_id.0.clone(), quotas)) {
                        runtime.guest_lifetimes.borrow_mut().remove_uncreated(instance).map_err(reactor_close_fault)?;
                        return Err(error.into());
                    }
                    semio_framework_plugin_runtime_instances::inst_opening::plugin_open_actor_instance(runtime,request,app_id.0,actor,identity,cx).await?;
                }
            }
            Event::InstanceClose(_) | Event::InstanceLifecycleAck(_) => {}
            Event::CommandIngressPage { .. } => {
                return Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.command-page-event-bypass"), "command page must use poll_kernel's dedicated owner argument").into());
            }
            Event::ColdDocumentPairPage(_) => {
                return Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.cold-pair-page-event-bypass"), "cold document pair page must use poll_kernel's dedicated owner argument").into());
            }
            Event::UiIntent { instance, intent } => {
                let numeric_instance = instance.0.parse::<u32>().map_err(|_| reactor_close_fault("invalid intent instance"))?;
                native_close_key(runtime, numeric_instance)?;
                if let Ok(intent_value) = store::pack_rt::decode_wire_value(&intent) {
                    if let Ok(intent) = serde_json::from_value::<ui_contract::UiIntent>(intent_value.into()) {
                        if parse_surface_instance(intent.surface.as_ref()) != Some(numeric_instance) {
                            return Err(reactor_close_fault("intent surface names another lifetime").into());
                        }
                        let current_revision = PATCHES.with(|patches| patches.revision(&intent.surface.0));
                        if !is_stale_intent(intent.revision, current_revision, DEFAULT_REVISION_TOLERANCE) {
                            dirty
                                .try_intent(numeric_instance, intent)
                                .map_err(|_| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.dirty-intent-capacity"), "fixed dirty intent authority is saturated"))?;
                        }
                    }
                }
            }
            Event::SurfaceVisible { surface, body_key, view_state } => {
                if let Some(instance) = parse_surface_instance(&surface) {
                    native_close_key(runtime, instance)?;
                    semio_framework_plugin_runtime_ops::ops_surface_render::plugin_mount_surface(runtime, instance, surface.clone(), body_key, &view_state).await?;
                    let surface =
                        ui_contract::SurfaceId::try_from(surface).map_err(|_| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.surface-capacity"), "surface id exceeds fixed text capacity"))?;
                    dirty.try_surface(instance, surface).map_err(|_| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.dirty-surface-capacity"), "fixed dirty surface authority is saturated"))?;
                }
            }
            Event::SurfaceHidden { surface } => {
                if let Some(instance) = parse_surface_instance(&surface) {
                    if native_close_key(runtime, instance).is_ok() {
                        semio_framework_plugin_runtime_ops::ops_surface_render::plugin_hide_surface(runtime, instance, &surface).await?;
                    }
                }
            }
            Event::SurfaceResized { .. } => {}
            Event::PatchAck { receipt, surface, revision } => {
                if !live_patch_receipt(runtime, receipt) {
                    continue;
                }
                with_pending_patches(|pending| pending.borrow_mut().apply_issued_ack(receipt, &surface, revision, semio_framework_ui_runtime::SURFACE_RECONCILE_PAGE_BYTES, |ack| PATCHES.with(|patches| patches.mark_published_ack(ack))))
                    .map_err(|reason| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("ui.patch-ack-authority"), reason))?;
            }
            Event::PatchRejected { receipt, surface, revision, .. } => {
                if !live_patch_receipt(runtime, receipt) {
                    continue;
                }
                with_pending_patches(|pending| pending.borrow_mut().apply_issued_rejection(receipt, &surface, revision, |generation| PATCHES.with(|patches| patches.mark_rejected(&surface, generation))));
            }
            Event::Completed { .. } => unreachable!("Completed source donation uses the original borrowed receiver"),
            Event::HttpChunk { req, bytes, done } => {
                if append_extension_response_page(req, &bytes) {
                    continue;
                }
                REGISTRY.with(|registry| {
                    let cap = registry.instance_of(req).and_then(|instance| INSTANCE_METADATA.with(|metadata| metadata.borrow().get(instance).and_then(|entry| entry.quota.message_bytes))).unwrap_or(64 * 1024 * 1024) as usize;
                    registry.append_chunk(req, &bytes, done, cap);
                });
            }
            Event::JobProgress { job, .. } => {
                if let Some(binding) = JOB_RENDER_BINDINGS.with(|bindings| bindings.borrow().accepted(job)) {
                    dirty_background_surfaces(runtime, binding.instance, &mut dirty).await?;
                }
            }
            Event::JobCompleted { job, result } => {
                let instance = JOB_RENDER_BINDINGS.with(|bindings| bindings.borrow().owned(job).map(|binding| binding.instance));
                if let Some(binding) = JOB_RENDER_BINDINGS.with(|bindings| bindings.borrow_mut().complete(job)) {
                    dirty_background_surfaces(runtime, binding.instance, &mut dirty).await?;
                }
                let outcome = semio_framework_plugin_actor_host::outcome_to_result(result);
                if let Some(instance) = instance {
                    if native_close_key(runtime, instance).is_ok() {
                        let reserved_output = match &outcome {
                            Ok(bytes) => Ok(bytes.clone()),
                            Err(fault) => Err(fault.clone()),
                        };
                        let output = semio_framework_plugin_runtime_ops::ops_typed_operations::plugin_complete_reserved_spawned_job(runtime, instance, job, reserved_output, identity,cx).await?;
                        route_exchange_output(instance, output, &mut document_backbone_effects,cx)?;
                    }
                }
                REGISTRY.with(|registry| registry.resolve(semio_framework::kernel::RequestId(job), outcome));
            }
            Event::Message { source: MessageEndpoint::Shell { instance }, payload } => {
                if let Some(token) = semio_framework_plugin_job::TypedOperationResultPage::renderer_ack_token(&payload) {
                    if instance.0.parse::<u32>().ok() == Some(token.receiver) {
                        let _ = semio_framework_plugin_runtime_ops::ops_typed_operations::plugin_acknowledge_typed_operation_result(runtime, token).await?;
                    }
                } else if let Some(effects) = semio_framework_plugin_runtime_ops::ops_document::plugin_handle_document_backbone_binding(runtime, &instance.0, &payload).await? {
                    document_backbone_effects.extend(effects);
                }
            }
            Event::Message { source: MessageEndpoint::Backbone { uri }, payload } => {
                let output = semio_framework_plugin_runtime_ops::ops_document::plugin_receive_document_backbone(runtime, &uri, &payload, identity, cx).await?;
                if output.document_changed {
                    dirty_document_surfaces(runtime, output.instance_id, &mut dirty).await?;
                } else {
                    dirty_background_surfaces(runtime, output.instance_id, &mut dirty).await?;
                }
                for frame in output.frames {
                    route_app_frame(output.instance_id, &frame, &mut document_backbone_effects);
                }
                document_backbone_effects.extend(output.effects);
            }
            Event::Message { .. } => {}
            Event::Timer { id } => {
                ARMED_TIMERS.with(|timers| {
                    timers.borrow_mut().remove(id);
                });
                TASK_EXECUTOR.with(|executor| executor.wake(id));
            }
            Event::Wake => {}
            Event::Request { req, from, capability, payload } => {
                let owner=RetainedExtensionInvocation::new(req,from,capability,payload);
                RETAINED_EXTENSION_INVOCATIONS.with(|owners|{let mut owners=owners.borrow_mut();let slot=owners.iter_mut().find(|slot|slot.is_none()).ok_or_else(||reactor_close_fault("original extension request slots are full"))?;*slot=Some(owner);Ok::<_,semio_framework::Fault>(())})?;
            }
            Event::SuspendRequest => semio_framework_plugin_extension::extension_runtime_lift::extension_deactivate().await,
            Event::Activate { .. } | Event::CapabilityChanged { .. } | Event::QuotaChanged { .. } => {}
        }
    }

    if let Some(effect)=advance_retained_extension_invocation(cx).await?{inbound_request_effects.push(effect);}
    let original_extension_grant = runtime.mounted_owner_policy().close;
    let remaining=cx.retained_grant();
    let extension_grant = semio_framework_value::RetainedCloneGrant { maximum_items: original_extension_grant.maximum_items.min(remaining.maximum_items),maximum_copy_bytes:original_extension_grant.maximum_copy_bytes.min(remaining.maximum_copy_bytes),maximum_capacity_bytes:original_extension_grant.maximum_capacity_bytes.min(remaining.maximum_capacity_bytes),maximum_release_bytes:original_extension_grant.maximum_release_bytes.min(remaining.maximum_release_bytes),maximum_depth:original_extension_grant.maximum_depth.min(remaining.maximum_depth) };
    let (extension_retirement_work, _) = cx.with_retained_loan(extension_grant,|same|semio_framework_plugin_extension::extension_runtime_lift::extension_retirement_turn(same))?;
    let close_cleanup_work = close_cleanup_work || extension_retirement_work;
    let mut effects: Vec<Effect> = document_backbone_effects;
    effects.extend(inbound_request_effects);
    let mut cold_pair_ingress = semio_framework::kernel::ColdPairIngressStatus::Idle;
    let cold_pair_page_lifetime = cold_pair_page.as_ref().map(|page| page.header.lifetime);
    if let Some(page) = cold_pair_page {
        let lifetime = page.header.lifetime;
        let transfer_generation = page.header.transfer_generation;
        supersede_cold_pair_document_load(runtime, lifetime, transfer_generation).await;
        let terminal_cursor = page.header.cursor(page.header.page_count.saturating_sub(1));
        let live = native_close_key(runtime, lifetime.instance_id).ok().filter(|key| key.lifetime() == lifetime).map(|key| key.lifetime());
        let another_loads = COLD_PAIR_DOCUMENT_LOAD.with(|slot| slot.borrow().as_ref().is_some_and(|pending| pending.cursor.lifetime != lifetime));
        cold_pair_ingress = if another_loads && page.page_index.saturating_add(1) == page.header.page_count {
            semio_framework::kernel::ColdPairIngressStatus::Backpressure(page.header.cursor(page.page_index))
        } else {
            COLD_PAIR_INGRESS.with(|ingress| ingress.borrow_mut().accept_page(&page, live))
        };
        if matches!(cold_pair_ingress, semio_framework::kernel::ColdPairIngressStatus::Loading(_)) {
            let live = native_close_key(runtime, lifetime.instance_id).ok().filter(|key| key.lifetime() == lifetime).map(|key| key.lifetime());
            let load = COLD_PAIR_INGRESS.with(|ingress| ingress.borrow_mut().begin_load(lifetime, transfer_generation, live));
            match load.map(|load| cold_pair::ColdPairDocumentLoad::new(load, transfer_generation, terminal_cursor)) {
                Some(pending) => match semio_framework_plugin_runtime_ops::ops_document::plugin_begin_document_archive_load(runtime, lifetime.instance_id, pending.operation, pending.archive()).await {
                    Ok(()) => COLD_PAIR_DOCUMENT_LOAD.with(|slot| *slot.borrow_mut() = Some(pending)),
                    Err(fault) => cold_pair_ingress = COLD_PAIR_INGRESS.with(|ingress| ingress.borrow_mut().finish_load(pending.load, Err(semio_framework_diagnostic::encode_fault_bytes(&fault)), live)),
                },
                None => cold_pair_ingress = semio_framework::kernel::ColdPairIngressStatus::Fault { cursor: terminal_cursor, fault: b"cold-pair.load-admission".to_vec() },
            }
        }
    }
    if let Some(pending) = COLD_PAIR_DOCUMENT_LOAD.with(|slot| slot.borrow_mut().take()) {
        let answers_this_turn = !pending.superseded && cold_pair_page_lifetime.is_none_or(|lifetime| lifetime == pending.cursor.lifetime);
        let status = step_cold_pair_document_load(runtime, pending, identity, cx).await;
        if answers_this_turn {
            cold_pair_ingress = status;
        }
    }
    let cold_pair_loading = matches!(cold_pair_ingress, semio_framework::kernel::ColdPairIngressStatus::Loading(_));
    let document_load_work = step_restored_document_loads(runtime, &mut effects, identity, cx).await || COLD_PAIR_DOCUMENT_LOAD.with(|slot| slot.borrow().is_some());
    let mut command_ingress = semio_framework::kernel::CommandIngressStatus::Idle;
    step_retiring_command_ingress(cx)?;
    let (mut retained, mut retained_key) = COMMAND_INGRESS.with(|ingress| match ingress.borrow_mut()[COMMAND_INGRESS_LIVE_SLOT].take() {
        Some(entry) => (Some(entry.state), Some(entry.key)),
        None => (None, None),
    });
    if let Some(key) = retained_key {
        if native_close_key(runtime, key.instance()).ok() != Some(key) {
            close_instances.push(key.instance());
        }
    }
    retained = match retained.take() {
        Some(CommandIngressOwner::ReservedPresence { cursor, admission, .. }) if close_instances.contains(&cursor.instance) => {
            admission.cancel.cancel_now();
            command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-cancelled-by-close".to_vec() };
            None
        }
        Some(CommandIngressOwner::PendingPresencePage { cursor, .. }) if close_instances.contains(&cursor.instance) => {
            command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-cancelled-by-close".to_vec() };
            None
        }
        Some(CommandIngressOwner::Generic { cursor, command }) if close_instances.contains(&cursor.instance) => {
            command_ingress = semio_framework::kernel::CommandIngressStatus::CommandPending(cursor.clone());
            Some(CommandIngressOwner::Generic {
                cursor,
                command: command.cancel(semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.command-cancelled-by-close"), "command ingress was cancelled by instance close")),
            })
        }
        Some(CommandIngressOwner::GenericAssembly { cursor, pages }) if close_instances.contains(&cursor.instance) => {
            command_ingress = semio_framework::kernel::CommandIngressStatus::CommandPending(cursor.clone());
            Some(CommandIngressOwner::ClosingAssembly { cursor, pages })
        }
        owner => owner,
    };
    retained = match retained.take() {
        Some(CommandIngressOwner::ClosingAssembly { cursor, mut pages }) => {
            let (complete, _) = pages.close_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES);
            if complete {
                command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-cancelled-by-close".to_vec() };
                None
            } else {
                command_ingress = semio_framework::kernel::CommandIngressStatus::CommandPending(cursor.clone());
                Some(CommandIngressOwner::ClosingAssembly { cursor, pages })
            }
        }
        owner => owner,
    };
    retained = match retained.take() {
        Some(CommandIngressOwner::ReservedPresence { cursor, admission, page }) => {
            let now_ms = semio_framework_plugin_actor_host::now_ms().await;
            match semio_framework_plugin_runtime_ops::ops_typed_operations::plugin_admit_reserved_presence(runtime, cursor.instance, admission, if cursor.metadata & 0x100 != 0 { Some((cursor.metadata & 0xff) as u8) } else { None }, cursor.item_count, page, now_ms).await {
                Ok(publication_generation) => {
                    command_ingress = semio_framework::kernel::CommandIngressStatus::PageAccepted(cursor.clone());
                    Some(CommandIngressOwner::Presence { cursor, publication_generation })
                }
                Err((_fault, admission, page)) => {
                    command_ingress = semio_framework::kernel::CommandIngressStatus::CommandPending(cursor.clone());
                    Some(CommandIngressOwner::ReservedPresence { cursor, admission, page })
                }
            }
        }
        owner => owner,
    };
    retained = match retained.take() {
        Some(CommandIngressOwner::PendingPresencePage { cursor, publication_generation, page }) => match semio_framework_plugin_runtime_ops::ops_typed_operations::plugin_push_reserved_presence_page(runtime, cursor.instance, publication_generation, cursor.page_index, page).await {
            Ok(()) => {
                command_ingress = semio_framework::kernel::CommandIngressStatus::PageAccepted(cursor.clone());
                Some(CommandIngressOwner::Presence { cursor, publication_generation })
            }
            Err((_fault, page)) => {
                command_ingress = semio_framework::kernel::CommandIngressStatus::CommandPending(cursor.clone());
                Some(CommandIngressOwner::PendingPresencePage { cursor, publication_generation, page })
            }
        },
        owner => owner,
    };
    if matches!(command_ingress, semio_framework::kernel::CommandIngressStatus::Idle) {
        if let Some(CommandIngressOwner::Presence { cursor, .. }) = retained.as_ref() {
            let cursor = cursor.clone();
            if close_instances.contains(&cursor.instance) {
                retained = None;
                command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-cancelled-by-close".to_vec() };
            } else {
                match plugin_exchange_boxed(runtime, cursor.instance, None, identity,cx).await {
                    Ok(output) => {
                        let instance = cursor.instance;
                        if output.presence_terminal == Some(cursor.seq) {
                            retained = None;
                            command_ingress = match advance_command_cursor(cursor) {
                                Ok(terminal) => match output.presence_terminal_fault.as_ref() {
                                    Some(fault) => semio_framework::kernel::CommandIngressStatus::Fault { cursor: terminal, fault: fault.clone() },
                                    None => semio_framework::kernel::CommandIngressStatus::CommandComplete(terminal),
                                },
                                Err(cursor) => semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-page-index-exhausted".to_vec() },
                            };
                        } else if cursor.page_index.checked_add(1) == Some(cursor.page_count) {
                            command_ingress = match advance_command_cursor(cursor.clone()) {
                                Ok(pending) => semio_framework::kernel::CommandIngressStatus::CommandPending(pending),
                                Err(cursor) => semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-page-index-exhausted".to_vec() },
                            };
                        }
                        route_exchange_output(instance, output, &mut effects,cx)?;
                    }
                    Err(failure) => return Err(failure),
                }
            }
        }
    }
    if let Some(owner) = retained.take() {
        match owner {
            CommandIngressOwner::Generic { cursor, command } => match plugin_exchange_boxed(runtime, cursor.instance, Some((cursor.seq, command)), identity,cx).await {
                Ok(mut output) => {
                    match advance_command_cursor(cursor.clone()) {
                        Ok(terminal) => {
                            if let Some((_, command)) = output.retry_command.take() {
                                retained = Some(CommandIngressOwner::Generic { cursor: cursor.clone(), command });
                                command_ingress = semio_framework::kernel::CommandIngressStatus::CommandPending(terminal);
                            } else if let Some(fault) = output.command_terminal_fault.as_ref() {
                                command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor: terminal, fault: fault.clone() };
                            } else {
                                command_ingress = semio_framework::kernel::CommandIngressStatus::CommandComplete(terminal);
                            }
                        }
                        Err(cursor) => command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-page-index-exhausted".to_vec() },
                    }
                    route_exchange_output(cursor.instance, output, &mut effects,cx)?;
                }
                Err(failure) => return Err(failure),
            },
            owner => retained = Some(owner),
        }
    }
    if let Some((cursor, page)) = command_page {
        if retained.is_none() {
            retained_key = native_close_key(runtime, cursor.instance).ok();
        }
        if !runtime.guest_lifetimes.borrow().get(cursor.instance).is_some_and(|slot| slot.cell.is_live())
            || close_instances.contains(&cursor.instance)
            || page.len() > semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES
            || cursor.page_count == 0
            || cursor.page_count as usize > semio_framework::kernel::COMMAND_MAXIMUM_PAGES
            || cursor.command_count == 0
            || cursor.command_count as usize > semio_framework::kernel::COMMAND_BATCH_MAXIMUM_ITEMS
            || cursor.command_index >= cursor.command_count
            || cursor.page_index >= cursor.page_count
            || cursor.item_count as usize > semio_framework::kernel::COMMAND_BATCH_MAXIMUM_ITEMS
            || (cursor.kind == 28 && cursor.page_count != cursor.item_count.max(1))
            || (cursor.kind == 28 && ((cursor.item_count == 0) != page.is_empty()))
            || (cursor.kind != 28
                && (page.is_empty()
                    || cursor.item_count != 0
                    || cursor.metadata != 0
                    || (cursor.page_index == 0 && cursor.kind != page.as_slice()[0])
                    || (cursor.page_index.checked_add(1).is_some_and(|next| next < cursor.page_count) && page.len() != semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES)))
        {
            command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-page-invalid".to_vec() };
        } else if retained.as_ref().is_some_and(|owner| !same_command_cursor(&command_ingress_owner_cursor(owner), &cursor)) {
            if retained.as_ref().is_some_and(|owner| command_ingress_owner_cursor(owner).instance == cursor.instance) && retiring_command_ingress_is_free() {
                park_retiring_command_ingress(retained_key.expect("a retained owner holds its exact lifetime"), retained.take().expect("the superseded owner was retained"),cx)?;
            }
            command_ingress = semio_framework::kernel::CommandIngressStatus::Backpressure(cursor);
        } else if matches!(retained, Some(CommandIngressOwner::ReservedPresence { .. } | CommandIngressOwner::PendingPresencePage { .. })) {
            command_ingress = semio_framework::kernel::CommandIngressStatus::CommandPending(cursor);
        } else if cursor.kind == 28 {
            let own_color = if cursor.metadata & !0x1ff != 0 {
                command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor: cursor.clone(), fault: b"plugin.command-presence-metadata".to_vec() };
                None
            } else if cursor.metadata & 0x100 != 0 {
                Some((cursor.metadata & 0xff) as u8)
            } else {
                None
            };
            if !matches!(command_ingress, semio_framework::kernel::CommandIngressStatus::Fault { .. }) {
                if cursor.page_index == 0 && retained.is_none() {
                    match semio_framework_plugin_runtime_ops::ops_typed_operations::plugin_reserve_presence_ingress(runtime, cursor.instance, cursor.seq).await {
                        Ok(admission) => {
                            let now_ms = semio_framework_plugin_actor_host::now_ms().await;
                            match semio_framework_plugin_runtime_ops::ops_typed_operations::plugin_admit_reserved_presence(runtime, cursor.instance, admission, own_color, cursor.item_count, page, now_ms).await {
                                Ok(publication_generation) => {
                                    retained = Some(CommandIngressOwner::Presence { cursor: cursor.clone(), publication_generation });
                                    command_ingress = semio_framework::kernel::CommandIngressStatus::PageAccepted(cursor);
                                }
                                Err((_fault, admission, page)) => {
                                    retained = Some(CommandIngressOwner::ReservedPresence { cursor: cursor.clone(), admission, page });
                                    command_ingress = semio_framework::kernel::CommandIngressStatus::CommandPending(cursor);
                                }
                            }
                        }
                        Err(_) => command_ingress = semio_framework::kernel::CommandIngressStatus::Backpressure(cursor),
                    }
                } else if let Some(publication_generation) = retained.as_ref().and_then(|owner| match owner {
                    CommandIngressOwner::Presence { publication_generation, .. } => Some(*publication_generation),
                    _ => None,
                }) {
                    match semio_framework_plugin_runtime_ops::ops_typed_operations::plugin_push_reserved_presence_page(runtime, cursor.instance, publication_generation, cursor.page_index, page).await {
                        Ok(()) => {
                            if let Some(CommandIngressOwner::Presence { cursor: active, .. }) = retained.as_mut() {
                                active.page_index = cursor.page_index;
                            }
                            command_ingress = semio_framework::kernel::CommandIngressStatus::PageAccepted(cursor);
                        }
                        Err((_fault, page)) => {
                            retained = Some(CommandIngressOwner::PendingPresencePage { cursor: cursor.clone(), publication_generation, page });
                            command_ingress = semio_framework::kernel::CommandIngressStatus::CommandPending(cursor);
                        }
                    }
                }
            }
        } else if cursor.page_index == 0 && retained.is_none() {
            match semio_framework::kernel::CommandPageSet::try_new(cursor.page_count as usize) {
                Ok(mut pages) => match pages.try_push(page) {
                    Err((fault, _page)) => command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: semio_framework_diagnostic::encode_fault_bytes(&fault) },
                    Ok(()) if cursor.page_count == 1 => match semio_framework::kernel::PagedCommand::try_from_pages(pages) {
                        Ok(command) => match plugin_exchange_boxed(runtime, cursor.instance, Some((cursor.seq, semio_framework_plugin_runtime_ops::ops_ingress::PluginCommandIngress::Encoded(command))), identity,cx).await {
                            Ok(mut output) => {
                                if let Some((_, command)) = output.retry_command.take() {
                                    retained = Some(CommandIngressOwner::Generic { cursor: cursor.clone(), command });
                                    command_ingress = match advance_command_cursor(cursor.clone()) {
                                        Ok(pending) => semio_framework::kernel::CommandIngressStatus::CommandPending(pending),
                                        Err(cursor) => semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-page-index-exhausted".to_vec() },
                                    };
                                } else {
                                    command_ingress = terminal_command_ingress(cursor.clone(), output.command_terminal_fault.take());
                                }
                                route_exchange_output(cursor.instance, output, &mut effects,cx)?;
                            }
                            Err(failure) => return Err(failure),
                        },
                        Err((fault, _pages)) => command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: semio_framework_diagnostic::encode_fault_bytes(&fault) },
                    },
                    Ok(()) => {
                        retained = Some(CommandIngressOwner::GenericAssembly { cursor: cursor.clone(), pages });
                        command_ingress = semio_framework::kernel::CommandIngressStatus::PageAccepted(cursor);
                    }
                },
                Err(fault) => command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: semio_framework_diagnostic::encode_fault_bytes(&fault) },
            }
        } else if let Some(CommandIngressOwner::GenericAssembly { cursor: active, mut pages }) = {
            match retained {
                Some(CommandIngressOwner::GenericAssembly { .. }) => retained.take(),
                _ => None,
            }
        } {
            if cursor.page_index as usize != pages.len() {
                command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-page-order".to_vec() };
            } else {
                match pages.try_push(page) {
                    Err((fault, _page)) => command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: semio_framework_diagnostic::encode_fault_bytes(&fault) },
                    Ok(()) if cursor.page_index.checked_add(1) == Some(cursor.page_count) => match semio_framework::kernel::PagedCommand::try_from_pages(pages) {
                        Ok(command) => match plugin_exchange_boxed(runtime, cursor.instance, Some((cursor.seq, semio_framework_plugin_runtime_ops::ops_ingress::PluginCommandIngress::Encoded(command))), identity,cx).await {
                            Ok(mut output) => {
                                if let Some((_, command)) = output.retry_command.take() {
                                    retained = Some(CommandIngressOwner::Generic { cursor: cursor.clone(), command });
                                    command_ingress = match advance_command_cursor(cursor.clone()) {
                                        Ok(pending) => semio_framework::kernel::CommandIngressStatus::CommandPending(pending),
                                        Err(cursor) => semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-page-index-exhausted".to_vec() },
                                    };
                                } else {
                                    command_ingress = terminal_command_ingress(cursor.clone(), output.command_terminal_fault.take());
                                }
                                route_exchange_output(cursor.instance, output, &mut effects,cx)?;
                            }
                            Err(failure) => return Err(failure),
                        },
                        Err((fault, _pages)) => command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: semio_framework_diagnostic::encode_fault_bytes(&fault) },
                    },
                    Ok(()) => {
                        retained = Some(CommandIngressOwner::GenericAssembly { cursor: active, pages });
                        if let Some(CommandIngressOwner::GenericAssembly { cursor: active, .. }) = retained.as_mut() {
                            active.page_index = cursor.page_index;
                        }
                        command_ingress = semio_framework::kernel::CommandIngressStatus::PageAccepted(cursor);
                    }
                }
            }
        } else {
            command_ingress = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-page-unowned".to_vec() };
        }
    }
    command_ingress = command_ingress_while_owned(command_ingress, retained.as_ref().map(command_ingress_owner_cursor));
    if let Some(retained) = retained {
        COMMAND_INGRESS.with(|ingress| {
            ingress.borrow_mut()[COMMAND_INGRESS_LIVE_SLOT] = Some(RetainedCommandIngress { key: retained_key.expect("admitted command retains its exact lifetime"), state: retained });
        });
    }
    if !original_receiving::retain_intents(&mut dirty.intents,cx)? {
        return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original intent receiving slot retains an earlier batch").into());
    }
    original_receiving::advance_responses(runtime,identity,&mut effects,cx).await?;
    original_receiving::advance_intents(runtime,identity,&mut effects,cx).await?;

    for _ in 0..PATCH_CLOSE_UNITS_PER_TURN {
        if cx.should_yield() {
            break;
        }
        if with_pending_patches(|pending| pending.borrow_mut().close_step(PATCH_RETIREMENT_ITEMS_PER_UNIT, PATCH_RETIREMENT_BYTES_PER_UNIT)).map_err(reactor_close_fault)? {
            break;
        }
    }

    trace_turn_phase_retention("ingress");
    let (continuation, typed_operation_scan) = semio_framework_plugin_runtime_ops::ops_typed_operations::plugin_continue_typed_operations(runtime, semio_framework_plugin_runtime_ops::ops_typed_operations::TypedOperationGrant::turn(budget,cx), identity,cx).await?;
    let typed_operation_contended = typed_operation_scan.contended;
    if let Some((instance, output)) = continuation {
        route_exchange_output(instance, output, &mut effects,cx)?;
    }

    trace_turn_phase_retention("continuation");
    let now_ms = u64::try_from(semio_framework_plugin_actor_host::now_ms().await).unwrap_or(0);

    trace_turn_phase_retention("presence-clock");
    for (instance, surface) in dirty.surfaces {
        if native_close_key(runtime, instance).is_err() {
            continue;
        }
        let surface_key = surface.as_ref().to_owned();
        let _ = semio_framework_plugin_window_kits_rich::tree_kit::take_arena_unbuilt_rows();
        let mounted = match native_close_key(runtime, instance) {
            Ok(key) => PATCHES.with(|patches| patches.reserve_mounted(surface, key)),
            Err(_) => Err(surface),
        };
        match mounted {
            Ok(mut grant) => match semio_framework_plugin_runtime_ops::ops_surface_render::plugin_render_surface(runtime, instance, &surface_key,&mut |source,cx|grant.commit_source(source,cx),cx).await {
                Ok(Some(presence)) => {
                    let unbuilt = semio_framework_plugin_window_kits_rich::tree_kit::take_arena_unbuilt_rows();
                    ARENA_STARVED.with(|starved| starved.borrow_mut().note(instance, surface_key.as_str(), unbuilt));
                    for update in presence {
                        PRESENCE.with(|hub| {
                            let mut hub = hub.borrow_mut();
                            hub.record_own(update.surface.clone(), update.node_key.clone(), update.own, update.ttl_ms);
                            for peer in update.peers {
                                hub.record_peer(update.surface.clone(), update.node_key.clone(), peer, update.ttl_ms, now_ms);
                            }
                        });
                    }
                }
                Ok(None)=>{
                    grant.cancel_and_defer();
                }
                Err(failure)=>{
                    grant.cancel();
                    return Err(failure);
                }
            },
            Err(surface) => {
                if let Some(reason) = PATCHES.with(|patches| patches.take_unreported_reserve_refusal()).filter(|reason| reason.starts_with("registry-")) {
                    effects.push(shell_fault_effect(
                        instance,
                        &semio_framework::Fault::new(
                            semio_framework::FaultOrigin::Os,
                            semio_framework::FaultCode::new("ui.surface-reconcile-unadmitted"),
                            format!("dirty surface {surface_key} was refused its reconcile reservation by {reason}, so its render is deferred until another surface returns credit"),
                        ),
                    ));
                }
                PATCHES.with(|patches| patches.defer(surface)).map_err(|surface| {
                    semio_framework::Fault::new(
                        semio_framework::FaultOrigin::Os,
                        semio_framework::FaultCode::new("ui.dirty-surface-deferred-capacity"),
                        format!("deferred render ring is full, so dirty surface {} would never re-render", surface.as_ref()),
                    )
                })?;
            }
        }
    }
    let reconcile_work = PATCHES
        .with(|patches| -> Result<bool,semio_framework_job::OriginalTurnFailure> {
            drive_reconcile_within(patches, reconcile_step_opportunities(cx.fuel_remaining()), cx)?;
            Ok(reconcile_arms_turn(patches))
        })
        ?;
    if let Some((instance, message)) = PATCHES.with(patches::PatchTracker::take_render_fault) {
        effects.push(shell_fault_effect(instance, &semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new(SURFACE_RENDER_FAULT_CODE), message)));
    }

    let executor_deadline_work = REACTOR_EXECUTOR.with(|executor| executor.run_until_deadline(64, 256 * 1_024, std::time::Instant::now() + std::time::Duration::from_millis(REACTOR_TURN_EXECUTOR_HOLD_MS)));
    let task_executor_work = TASK_EXECUTOR.with(|executor| executor.run_until_deadline(64, std::time::Instant::now() + std::time::Duration::from_millis(REACTOR_TURN_EXECUTOR_HOLD_MS)));
    let process_pool_work = !executor_deadline_work && pump_process_worker_pool();
    let more_work = executor_deadline_work || task_executor_work || process_pool_work;
    for effect in REGISTRY.with(|registry| registry.drain()) {
        push_admitted_effect(&mut effects, 0, effect);
    }

    let resumes_remain = drain_task_resumes(runtime, &mut effects, 64, identity, cx);
    effects.extend(semio_framework_plugin_runtime_ops::ops_document::plugin_drain_document_backbones(runtime)?);
    let executor_pending = REACTOR_EXECUTOR.with(|executor| executor.has_pending()) || TASK_EXECUTOR.with(|executor| executor.has_pending());
    let command_ingress_pending = COMMAND_INGRESS.with(|ingress| ingress.borrow().iter().any(Option::is_some));
    let lifecycle_work = runtime.guest_lifetimes.borrow().has_work();
    let ui_retirement_work = close_late_ui_retirement()?;
    let extension_invocation_pending=RETAINED_EXTENSION_INVOCATIONS.with(|owners|owners.borrow().iter().any(Option::is_some))||original_receiving::response_pending()||original_receiving::intent_pending();
    let more_work = events.is_some() || more_work || extension_invocation_pending || close_cleanup_work || typed_operation_scan.runnable || reconcile_work || resumes_remain || executor_pending || command_ingress_pending || lifecycle_work || ui_retirement_work || document_load_work;
    LAST_MORE_WORK_SOURCES.set(TurnMoreWorkSources {
        executor_deadline: executor_deadline_work,
        process_pool: process_pool_work,
        close_cleanup: close_cleanup_work,
        typed_operation: typed_operation_scan.runnable,
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

    let lifecycle_receipt = focus.map(|instance| runtime.guest_lifetimes.borrow_mut().prepare_turn(instance, &ReactorClosePeers)).transpose()?.flatten();
    let (ui_patches, ui_patch_receipt) = if cold_pair_loading { (semio_framework::kernel::UiTurnPatches::default(), None) } else { fill_turn_patch_page(runtime, turn_patch_budget_bytes(budget))? };
    let presence = PRESENCE.with(|hub| {
        let mut hub = hub.borrow_mut();
        hub.expire(now_ms);
        hub.flush()
    });
    let status = if more_work { TurnStatus::MoreWork } else { TurnStatus::Idle };

    let retained_receipt=budget.retained.return_original(original_turn_progress(original_progress,cx.retained_progress())?,u64::try_from(cx.original_work_units_spent()-original_work).map_err(|_|reactor_close_fault("original turn work exceeds fixed receipt extent"))?)?;
    let mut result = semio_framework::kernel::TurnResult { retained_receipt, ui_patches, effects, presence, next_wake: ARMED_TIMERS.with(|timers| timers.borrow().first()), status, fuel_used: 0, command_ingress, cold_pair_ingress, lifecycle_receipt, ui_patch_receipt };
    with_pending_patches(|pending| {
        let mut pending = pending.borrow_mut();
        let prepared = (|| {
            result.validate_ui_patch_receipt().map_err(reactor_close_fault)?;
            if let Some(receipt) = result.ui_patch_receipt {
                pending.stage_emission(receipt, result.ui_patches.iter()).map_err(reactor_close_fault)?;
            }
            let prepared = prepare(&result)?;
            if let Some(instance) = focus {
                runtime.guest_lifetimes.borrow_mut().finish_turn(instance, guest_turn_executing_us()).map_err(|reason| {
                    if reason == instance_lifetime::GUEST_LIFECYCLE_TURN_DEADLINE {
                        semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.reactor-turn-deadline"), reason).with_retryable(retryable_lifecycle)
                    } else {
                        reactor_close_fault(reason)
                    }
                })?;
            }
            Ok(prepared)
        })();
        match prepared {
            Err(fault) => {
                loop {
                    let returned = result.ui_patches.try_transfer_one(|patch| pending.hand_back_turn(patch));
                    assert!(!matches!(returned, semio_framework::kernel::UiTurnPatchTransfer::Refused), "failed output retains its exact pending patch slot");
                    if matches!(returned, semio_framework::kernel::UiTurnPatchTransfer::Empty) {
                        break;
                    }
                }
                Err(fault)
            }
            Ok(prepared) => {
                if result.ui_patch_receipt.is_some() {
                    pending.commit_emission();
                }
                let published = publish(result, prepared);
                trace_turn_phase_retention("publish");
                Ok(published)
            }
        }
    })
}
